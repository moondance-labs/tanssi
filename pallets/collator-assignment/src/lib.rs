// Copyright (C) Moondance Labs Ltd.
// This file is part of Tanssi.

// Tanssi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tanssi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>

//! # Collator Assignment Pallet
//!
//! This pallet assigns a list of collators to:
//!    - the orchestrator chain
//!    - a set of container chains
//!
//! The set of container chains is retrieved thanks to the GetContainerChains trait
//! The number of collators to assign to the orchestrator chain and the number
//! of collators to assign to each container chain is retrieved through the GetHostConfiguration
//! trait.
//!  
//! The pallet uses the following approach:
//!
//! - First, it aims at filling the necessary collators to serve the orchestrator chain
//! - Second, it aims at filling in-order (FIFO) the existing containerChains
//!
//! Upon new session, this pallet takes whatever assignation was in the PendingCollatorContainerChain
//! storage, and assigns it as the current CollatorContainerChain. In addition, it takes the next
//! queued set of parachains and collators and calculates the assignment for the next session, storing
//! it in the PendingCollatorContainerChain storage item.
//!
//! The reason for the collator-assignment pallet to work with a one-session delay assignment is because
//! we want collators to know at least one session in advance the container chain/orchestrator that they
//! are assigned to.

#![cfg_attr(not(feature = "std"), no_std)]

use {
    crate::assignment::{Assignment, ChainNumCollators},
    frame_support::{pallet_prelude::*, traits::Currency},
    frame_system::pallet_prelude::BlockNumberFor,
    rand::{seq::SliceRandom, SeedableRng},
    rand_chacha::ChaCha20Rng,
    sp_runtime::{
        traits::{AtLeast32BitUnsigned, One, Zero},
        Saturating,
    },
    sp_std::{collections::btree_set::BTreeSet, fmt::Debug, prelude::*, vec},
    tp_traits::{
        CollatorAssignmentHook, CollatorAssignmentTip, GetContainerChainAuthor,
        GetHostConfiguration, GetSessionContainerChains, ParaId, RemoveInvulnerables,
        RemoveParaIdsWithNoCredits, ShouldRotateAllCollators, Slot,
    },
};
pub use {dp_collator_assignment::AssignedCollators, pallet::*};

mod assignment;
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::WeightInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type SessionIndex: parity_scale_codec::FullCodec
            + TypeInfo
            + Copy
            + AtLeast32BitUnsigned
            + Debug;
        // `SESSION_DELAY` is used to delay any changes to Paras registration or configurations.
        // Wait until the session index is 2 larger then the current index to apply any changes,
        // which guarantees that at least one full session has passed before any changes are applied.
        type HostConfiguration: GetHostConfiguration<Self::SessionIndex>;
        type ContainerChains: GetSessionContainerChains<Self::SessionIndex>;
        type SelfParaId: Get<ParaId>;
        type ShouldRotateAllCollators: ShouldRotateAllCollators<Self::SessionIndex>;
        type GetRandomnessForNextBlock: GetRandomnessForNextBlock<BlockNumberFor<Self>>;
        type RemoveInvulnerables: RemoveInvulnerables<Self::AccountId>;
        type RemoveParaIdsWithNoCredits: RemoveParaIdsWithNoCredits;
        type CollatorAssignmentHook: CollatorAssignmentHook<BalanceOf<Self>>;
        type Currency: Currency<Self::AccountId>;
        type CollatorAssignmentTip: CollatorAssignmentTip<BalanceOf<Self>>;
        /// The weight information of this pallet.
        type WeightInfo: WeightInfo;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        NewPendingAssignment {
            random_seed: [u8; 32],
            full_rotation: bool,
            target_session: T::SessionIndex,
        },
    }

    #[pallet::storage]
    #[pallet::getter(fn collator_container_chain)]
    pub(crate) type CollatorContainerChain<T: Config> =
        StorageValue<_, AssignedCollators<T::AccountId>, ValueQuery>;

    /// Pending configuration changes.
    ///
    /// This is a list of configuration changes, each with a session index at which it should
    /// be applied.
    ///
    /// The list is sorted ascending by session index. Also, this list can only contain at most
    /// 2 items: for the next session and for the `scheduled_session`.
    #[pallet::storage]
    #[pallet::getter(fn pending_collator_container_chain)]
    pub(crate) type PendingCollatorContainerChain<T: Config> =
        StorageValue<_, Option<AssignedCollators<T::AccountId>>, ValueQuery>;

    /// Randomness from previous block. Used to shuffle collators on session change.
    /// Should only be set on the last block of each session and should be killed on the on_initialize of the next block.
    /// The default value of [0; 32] disables randomness in the pallet.
    #[pallet::storage]
    #[pallet::getter(fn randomness)]
    pub(crate) type Randomness<T: Config> = StorageValue<_, [u8; 32], ValueQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    /// A struct that holds the assignment that is active after the session change and optionally
    /// the assignment that becomes active after the next session change.
    pub struct SessionChangeOutcome<T: Config> {
        /// New active assignment.
        pub active_assignment: AssignedCollators<T::AccountId>,
        /// Next session active assignment.
        pub next_assignment: AssignedCollators<T::AccountId>,
        /// Total number of registered parachains before filtering them out, used as a weight hint
        pub num_total_registered_paras: u32,
    }

    impl<T: Config> Pallet<T> {
        /// Assign new collators
        /// collators should be queued collators
        pub fn assign_collators(
            current_session_index: &T::SessionIndex,
            random_seed: [u8; 32],
            collators: Vec<T::AccountId>,
        ) -> SessionChangeOutcome<T> {
            // We work with one session delay to calculate assignments
            let session_delay = T::SessionIndex::one();
            let target_session_index = current_session_index.saturating_add(session_delay);
            // We get the containerChains that we will have at the target session
            let container_chains =
                T::ContainerChains::session_container_chains(target_session_index);
            let num_total_registered_paras =
                (container_chains.parachains.len() + container_chains.parathreads.len()) as u32;
            let mut container_chain_ids = container_chains.parachains;
            let mut parathreads: Vec<_> = container_chains
                .parathreads
                .into_iter()
                .map(|(para_id, _)| para_id)
                .collect();

            // We read current assigned collators
            let old_assigned = Self::read_assigned_collators();
            let old_assigned_para_ids: BTreeSet<ParaId> =
                old_assigned.container_chains.keys().cloned().collect();

            // Remove the containerChains that do not have enough credits for block production
            T::RemoveParaIdsWithNoCredits::remove_para_ids_with_no_credits(
                &mut container_chain_ids,
                &old_assigned_para_ids,
            );
            // TODO: parathreads should be treated a bit differently, they don't need to have the same amount of credits
            // as parathreads because they will not be producing blocks on every slot.
            T::RemoveParaIdsWithNoCredits::remove_para_ids_with_no_credits(
                &mut parathreads,
                &old_assigned_para_ids,
            );

            let mut shuffle_collators = None;
            // If the random_seed is all zeros, we don't shuffle the list of collators nor the list
            // of container chains.
            // This should only happen in tests, and in the genesis block.
            if random_seed != [0; 32] {
                let mut rng: ChaCha20Rng = SeedableRng::from_seed(random_seed);
                container_chain_ids.shuffle(&mut rng);
                parathreads.shuffle(&mut rng);
                shuffle_collators = Some(move |collators: &mut Vec<T::AccountId>| {
                    collators.shuffle(&mut rng);
                })
            }

            let orchestrator_chain = ChainNumCollators {
                para_id: T::SelfParaId::get(),
                min_collators: T::HostConfiguration::min_collators_for_orchestrator(
                    target_session_index,
                ),
                max_collators: T::HostConfiguration::max_collators_for_orchestrator(
                    target_session_index,
                ),
            };
            // Initialize list of chains as `[container1, container2, parathread1, parathread2]`.
            // The order means priority: the first chain in the list will be the first one to get assigned collators.
            // Chains will not be assigned less than `min_collators`, except the orchestrator chain.
            // First all chains will be assigned `min_collators`, and then the first one will be assigned up to `max`,
            // then the second one, and so on.
            let mut chains = vec![];
            let collators_per_container =
                T::HostConfiguration::collators_per_container(target_session_index);
            for para_id in &container_chain_ids {
                chains.push(ChainNumCollators {
                    para_id: *para_id,
                    min_collators: collators_per_container,
                    max_collators: collators_per_container,
                });
            }
            let collators_per_parathread =
                T::HostConfiguration::collators_per_parathread(target_session_index);
            for para_id in &parathreads {
                chains.push(ChainNumCollators {
                    para_id: *para_id,
                    min_collators: collators_per_parathread,
                    max_collators: collators_per_parathread,
                });
            }

            // Are there enough collators to satisfy the minimum demand?
            let enough_collators_for_all_chain = collators.len() as u32
                >= T::HostConfiguration::min_collators_for_orchestrator(target_session_index)
                    .saturating_add(
                        collators_per_container.saturating_mul(container_chain_ids.len() as u32),
                    )
                    .saturating_add(
                        collators_per_parathread.saturating_mul(parathreads.len() as u32),
                    );

            // Prioritize paras by tip on congestion
            // As of now this doesn't distinguish between parachains and parathreads
            // TODO apply different logic to parathreads
            if !enough_collators_for_all_chain {
                chains.sort_by(|a, b| {
                    T::CollatorAssignmentTip::get_para_tip(b.para_id)
                        .cmp(&T::CollatorAssignmentTip::get_para_tip(a.para_id))
                });
            }

            // We assign new collators
            // we use the config scheduled at the target_session_index
            let new_assigned =
                if T::ShouldRotateAllCollators::should_rotate_all_collators(target_session_index) {
                    log::debug!(
                        "Collator assignment: rotating collators. Session {:?}, Seed: {:?}",
                        current_session_index.encode(),
                        random_seed
                    );

                    Self::deposit_event(Event::NewPendingAssignment {
                        random_seed,
                        full_rotation: true,
                        target_session: target_session_index,
                    });

                    Assignment::<T>::assign_collators_rotate_all(
                        collators,
                        orchestrator_chain,
                        chains,
                        shuffle_collators,
                    )
                } else {
                    log::debug!(
                        "Collator assignment: keep old assigned. Session {:?}, Seed: {:?}",
                        current_session_index.encode(),
                        random_seed
                    );

                    Self::deposit_event(Event::NewPendingAssignment {
                        random_seed,
                        full_rotation: false,
                        target_session: target_session_index,
                    });

                    Assignment::<T>::assign_collators_always_keep_old(
                        collators,
                        orchestrator_chain,
                        chains,
                        old_assigned.clone(),
                        shuffle_collators,
                    )
                };

            let mut new_assigned = match new_assigned {
                Ok(x) => x,
                Err(e) => {
                    log::error!(
                        "Error in collator assignment, will keep previous assignment. {:?}",
                        e
                    );

                    old_assigned.clone()
                }
            };

            let mut assigned_containers = new_assigned.container_chains.clone();
            assigned_containers.retain(|_, v| !v.is_empty());

            // On congestion, prioritized chains need to pay the minimum tip of the prioritized chains
            let maybe_tip: Option<BalanceOf<T>> = if enough_collators_for_all_chain {
                None
            } else {
                assigned_containers
                    .into_keys()
                    .filter_map(T::CollatorAssignmentTip::get_para_tip)
                    .min()
            };

            // TODO: this probably is asking for a refactor
            // only apply the onCollatorAssignedHook if sufficient collators
            for para_id in &container_chain_ids {
                if !new_assigned
                    .container_chains
                    .get(para_id)
                    .unwrap_or(&vec![])
                    .is_empty()
                {
                    if let Err(e) = T::CollatorAssignmentHook::on_collators_assigned(
                        *para_id,
                        maybe_tip.as_ref(),
                        false,
                    ) {
                        // On error remove para from assignment
                        log::warn!(
                            "CollatorAssignmentHook error! Removing para {} from assignment: {:?}",
                            u32::from(*para_id),
                            e
                        );
                        new_assigned.container_chains.remove(para_id);
                    }
                }
            }

            for para_id in &parathreads {
                if !new_assigned
                    .container_chains
                    .get(para_id)
                    .unwrap_or(&vec![])
                    .is_empty()
                {
                    if let Err(e) = T::CollatorAssignmentHook::on_collators_assigned(
                        *para_id,
                        maybe_tip.as_ref(),
                        true,
                    ) {
                        // On error remove para from assignment
                        log::warn!(
                            "CollatorAssignmentHook error! Removing para {} from assignment: {:?}",
                            u32::from(*para_id),
                            e
                        );
                        new_assigned.container_chains.remove(para_id);
                    }
                }
            }

            let mut pending = PendingCollatorContainerChain::<T>::get();

            let old_assigned_changed = old_assigned != new_assigned;
            let mut pending_changed = false;
            // Update CollatorContainerChain using last entry of pending, if needed
            if let Some(current) = pending.take() {
                pending_changed = true;
                CollatorContainerChain::<T>::put(current);
            }
            if old_assigned_changed {
                pending = Some(new_assigned.clone());
                pending_changed = true;
            }
            // Update PendingCollatorContainerChain, if it changed
            if pending_changed {
                PendingCollatorContainerChain::<T>::put(pending);
            }

            // Only applies to session index 0
            if current_session_index == &T::SessionIndex::zero() {
                CollatorContainerChain::<T>::put(new_assigned.clone());
                return SessionChangeOutcome {
                    active_assignment: new_assigned.clone(),
                    next_assignment: new_assigned,
                    num_total_registered_paras,
                };
            }

            SessionChangeOutcome {
                active_assignment: old_assigned,
                next_assignment: new_assigned,
                num_total_registered_paras,
            }
        }

        // Returns the assigned collators as read from storage.
        // If there is any item in PendingCollatorContainerChain, returns that element.
        // Otherwise, reads and returns the current CollatorContainerChain
        fn read_assigned_collators() -> AssignedCollators<T::AccountId> {
            let mut pending_collator_list = PendingCollatorContainerChain::<T>::get();

            if let Some(assigned_collators) = pending_collator_list.take() {
                assigned_collators
            } else {
                // Read current
                CollatorContainerChain::<T>::get()
            }
        }

        pub fn initializer_on_new_session(
            session_index: &T::SessionIndex,
            collators: Vec<T::AccountId>,
        ) -> SessionChangeOutcome<T> {
            let random_seed = Randomness::<T>::take();
            let num_collators = collators.len();
            let assigned_collators = Self::assign_collators(session_index, random_seed, collators);
            let num_total_registered_paras = assigned_collators.num_total_registered_paras;

            frame_system::Pallet::<T>::register_extra_weight_unchecked(
                T::WeightInfo::new_session(num_collators as u32, num_total_registered_paras),
                DispatchClass::Mandatory,
            );

            assigned_collators
        }
    }

    impl<T: Config> GetContainerChainAuthor<T::AccountId> for Pallet<T> {
        // TODO: pending collator container chain if the block is a session change!
        fn author_for_slot(slot: Slot, para_id: ParaId) -> Option<T::AccountId> {
            let assigned_collators = Pallet::<T>::collator_container_chain();
            let collators = if para_id == T::SelfParaId::get() {
                Some(&assigned_collators.orchestrator_chain)
            } else {
                assigned_collators.container_chains.get(&para_id)
            }?;

            if collators.is_empty() {
                // Avoid division by zero below
                return None;
            }
            let author_index = u64::from(slot) % collators.len() as u64;
            collators.get(author_index as usize).cloned()
        }

        #[cfg(feature = "runtime-benchmarks")]
        fn set_authors_for_para_id(para_id: ParaId, authors: Vec<T::AccountId>) {
            let mut assigned_collators = Pallet::<T>::collator_container_chain();
            assigned_collators.container_chains.insert(para_id, authors);
            CollatorContainerChain::<T>::put(assigned_collators);
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(n: BlockNumberFor<T>) -> Weight {
            let mut weight = Weight::zero();

            // Account reads and writes for on_finalize
            if T::GetRandomnessForNextBlock::should_end_session(n.saturating_add(One::one())) {
                weight += T::DbWeight::get().reads_writes(1, 1);
            }

            weight
        }

        fn on_finalize(n: BlockNumberFor<T>) {
            // If the next block is a session change, read randomness and store in pallet storage
            if T::GetRandomnessForNextBlock::should_end_session(n.saturating_add(One::one())) {
                let random_seed = T::GetRandomnessForNextBlock::get_randomness();
                Randomness::<T>::put(random_seed);
            }
        }
    }
}

/// Balance used by this pallet
pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub struct RotateCollatorsEveryNSessions<Period>(PhantomData<Period>);

impl<Period> ShouldRotateAllCollators<u32> for RotateCollatorsEveryNSessions<Period>
where
    Period: Get<u32>,
{
    fn should_rotate_all_collators(session_index: u32) -> bool {
        let period = Period::get();

        if period == 0 {
            // A period of 0 disables rotation
            false
        } else {
            session_index % Period::get() == 0
        }
    }
}

pub trait GetRandomnessForNextBlock<BlockNumber> {
    fn should_end_session(block_number: BlockNumber) -> bool;
    fn get_randomness() -> [u8; 32];
}

impl<BlockNumber> GetRandomnessForNextBlock<BlockNumber> for () {
    fn should_end_session(_block_number: BlockNumber) -> bool {
        false
    }

    fn get_randomness() -> [u8; 32] {
        [0; 32]
    }
}
