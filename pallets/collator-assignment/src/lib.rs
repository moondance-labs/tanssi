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

pub use pallet::*;
use {
    crate::weights::WeightInfo,
    dp_collator_assignment::AssignedCollators,
    frame_support::pallet_prelude::*,
    frame_system::pallet_prelude::BlockNumberFor,
    rand::{seq::SliceRandom, SeedableRng},
    rand_chacha::ChaCha20Rng,
    sp_runtime::{
        traits::{AtLeast32BitUnsigned, One, Zero},
        Saturating,
    },
    sp_std::{fmt::Debug, prelude::*, vec},
    tp_traits::{
        GetContainerChainAuthor, GetHostConfiguration, GetSessionContainerChains, ParaId,
        RemoveInvulnerables, RemoveParaIdsWithNoCredits, ShouldRotateAllCollators, Slot,
    },
};

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;

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
            mut collators: Vec<T::AccountId>,
        ) -> SessionChangeOutcome<T> {
            // We work with one session delay to calculate assignments
            let session_delay = T::SessionIndex::one();
            let target_session_index = current_session_index.saturating_add(session_delay);
            // We get the containerChains that we will have at the target session
            let mut container_chain_ids =
                T::ContainerChains::session_container_chains(target_session_index);
            let num_total_registered_paras = container_chain_ids.len() as u32;
            // Remove the containerChains that do not have enough credits for block production
            T::RemoveParaIdsWithNoCredits::remove_para_ids_with_no_credits(
                &mut container_chain_ids,
            );

            // If the random_seed is all zeros, we don't shuffle the list of collators nor the list
            // of container chains.
            // This should only happen in tests, and in the genesis block.
            if random_seed != [0; 32] {
                let mut rng: ChaCha20Rng = SeedableRng::from_seed(random_seed);
                collators.shuffle(&mut rng);
                container_chain_ids.shuffle(&mut rng);
            }

            // We read current assigned collators
            let old_assigned = Self::read_assigned_collators();
            // We assign new collators
            // we use the config scheduled at the target_session_index
            let new_assigned =
                if T::ShouldRotateAllCollators::should_rotate_all_collators(target_session_index) {
                    log::info!(
                        "Collator assignment: rotating collators. Session {:?}, Seed: {:?}",
                        current_session_index.encode(),
                        random_seed
                    );

                    Self::deposit_event(Event::NewPendingAssignment {
                        random_seed,
                        full_rotation: true,
                        target_session: target_session_index,
                    });

                    Self::assign_collators_rotate_all(
                        collators,
                        &container_chain_ids,
                        T::HostConfiguration::min_collators_for_orchestrator(target_session_index)
                            as usize,
                        T::HostConfiguration::max_collators_for_orchestrator(target_session_index)
                            as usize,
                        T::HostConfiguration::collators_per_container(target_session_index)
                            as usize,
                    )
                } else {
                    log::info!(
                        "Collator assignment: keep old assigned. Session {:?}, Seed: {:?}",
                        current_session_index.encode(),
                        random_seed
                    );

                    Self::deposit_event(Event::NewPendingAssignment {
                        random_seed,
                        full_rotation: false,
                        target_session: target_session_index,
                    });

                    Self::assign_collators_always_keep_old(
                        collators,
                        &container_chain_ids,
                        T::HostConfiguration::min_collators_for_orchestrator(target_session_index)
                            as usize,
                        T::HostConfiguration::max_collators_for_orchestrator(target_session_index)
                            as usize,
                        T::HostConfiguration::collators_per_container(target_session_index)
                            as usize,
                        old_assigned.clone(),
                    )
                };

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

        /// Recompute collator assignment from scratch. If the list of collators and the list of
        /// container chains are shuffled, this returns a random assignment.
        fn assign_collators_rotate_all(
            collators: Vec<T::AccountId>,
            container_chain_ids: &[ParaId],
            min_num_orchestrator_chain: usize,
            max_num_orchestrator_chain: usize,
            num_each_container_chain: usize,
        ) -> AssignedCollators<T::AccountId> {
            // This is just the "always_keep_old" algorithm but with an empty "old"
            let old_assigned = Default::default();

            Self::assign_collators_always_keep_old(
                collators,
                container_chain_ids,
                min_num_orchestrator_chain,
                max_num_orchestrator_chain,
                num_each_container_chain,
                old_assigned,
            )
        }

        /// Assign new collators to missing container_chains.
        /// Old collators always have preference to remain on the same chain.
        /// If there are no missing collators, nothing is changed.
        ///
        /// `container_chain_ids` should be shuffled or at least rotated on every session to ensure
        /// a fair distribution, because the order of that list affects container chain priority:
        /// the first container chain on that list will be the first one to get new collators.
        fn assign_collators_always_keep_old(
            collators: Vec<T::AccountId>,
            container_chain_ids: &[ParaId],
            min_num_orchestrator_chain: usize,
            max_num_orchestrator_chain: usize,
            num_each_container_chain: usize,
            old_assigned: AssignedCollators<T::AccountId>,
        ) -> AssignedCollators<T::AccountId> {
            // TODO: the performance of this function is sad, could be improved by having sets of
            // old_collators and new_collators instead of doing array.contains() every time.
            let mut new_assigned = old_assigned;
            new_assigned.remove_collators_not_in_list(&collators);
            new_assigned.remove_container_chains_not_in_list(container_chain_ids);
            let extra_orchestrator_collators =
                new_assigned.remove_orchestrator_chain_excess_collators(min_num_orchestrator_chain);
            // Only need to do this if the config params change
            new_assigned.remove_container_chain_excess_collators(num_each_container_chain);

            // Collators that are not present in old_assigned
            // This is used to keep track of which collators are old and which ones are new, to keep
            // the old collators on the same chain if possible.
            let mut new_collators = vec![];
            for c in collators {
                if !new_assigned.find_collator(&c) && !extra_orchestrator_collators.contains(&c) {
                    new_collators.push(c);
                }
            }

            // Fill orchestrator chain collators up to min_num_orchestrator_chain
            // Give priority to invulnerables
            let num_missing_orchestrator_collators =
                min_num_orchestrator_chain.saturating_sub(new_assigned.orchestrator_chain.len());
            let invulnerables_for_orchestrator = T::RemoveInvulnerables::remove_invulnerables(
                &mut new_collators,
                num_missing_orchestrator_collators,
            );
            new_assigned.fill_orchestrator_chain_collators(
                min_num_orchestrator_chain,
                &mut invulnerables_for_orchestrator.into_iter(),
            );
            // If there are no enough invulnerables, or if the invulnerables are currently assigned to other chains,
            // fill orchestrator chain with regular collators
            let mut new_collators = new_collators.into_iter();
            new_assigned
                .fill_orchestrator_chain_collators(min_num_orchestrator_chain, &mut new_collators);

            // Fill container chain collators using new collators and also the extra
            // collators that were previously assigned to the orchestrator chain,
            // but give preference to new collators
            let mut extra_orchestrator_collators = extra_orchestrator_collators.into_iter();
            let mut new_plus_extra_collators = new_collators
                .by_ref()
                .chain(&mut extra_orchestrator_collators);

            new_assigned.add_and_fill_new_container_chains_in_order(
                num_each_container_chain,
                container_chain_ids,
                &mut new_plus_extra_collators,
            );

            // Fill orchestrator chain collators back up to max_num_orchestrator_chain,
            // but give preference to collators that were already there
            let mut extra_collators_plus_new = extra_orchestrator_collators
                .by_ref()
                .chain(&mut new_collators);
            new_assigned.fill_orchestrator_chain_collators(
                max_num_orchestrator_chain,
                &mut extra_collators_plus_new,
            );

            // Reorganize container chain collators to fill the maximum number of container
            // chains. For example, if num_each_container_chain == 2 and the number of collators
            // in each container chain is
            // [1, 1, 1, 1, 1]
            // Then we can convert that into
            // [2, 2, 0, 0, 0]
            // and assign 1 extra collator to the orchestrator chain, if needed.
            let incomplete_container_chains_collators = new_assigned
                .reorganize_incomplete_container_chains_collators(
                    container_chain_ids,
                    num_each_container_chain,
                );

            // Assign collators from container chains that do not reach
            // "num_each_container_chain" to orchestrator chain
            new_assigned.fill_orchestrator_chain_collators(
                max_num_orchestrator_chain,
                &mut incomplete_container_chains_collators.into_iter(),
            );

            new_assigned
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
