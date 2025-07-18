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
    core::ops::Mul,
    frame_support::{pallet_prelude::*, traits::Currency},
    frame_system::pallet_prelude::BlockNumberFor,
    rand::{seq::SliceRandom, SeedableRng},
    rand_chacha::ChaCha20Rng,
    sp_runtime::{
        traits::{AtLeast32BitUnsigned, One, Zero},
        Perbill, Saturating,
    },
    sp_std::{collections::btree_set::BTreeSet, fmt::Debug, prelude::*, vec},
    tp_traits::{
        CollatorAssignmentTip, ForSession, FullRotationModes, GetContainerChainAuthor,
        GetContainerChainsWithCollators, GetHostConfiguration, GetSessionContainerChains, ParaId,
        ParaIdAssignmentHooks, RemoveInvulnerables, ShouldRotateAllCollators, Slot,
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

#[derive(Encode, Decode, Debug, TypeInfo)]
pub struct CoreAllocationConfiguration {
    pub core_count: u32,
    pub max_parachain_percentage: Perbill,
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
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
        type Randomness: CollatorAssignmentRandomness<BlockNumberFor<Self>>;
        type RemoveInvulnerables: RemoveInvulnerables<Self::AccountId>;
        type ParaIdAssignmentHooks: ParaIdAssignmentHooks<BalanceOf<Self>, Self::AccountId>;
        type Currency: Currency<Self::AccountId>;
        type CollatorAssignmentTip: CollatorAssignmentTip<BalanceOf<Self>>;
        type ForceEmptyOrchestrator: Get<bool>;
        type CoreAllocationConfiguration: Get<Option<CoreAllocationConfiguration>>;
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
            full_rotation_mode: FullRotationModes,
        },
    }

    #[pallet::storage]
    #[pallet::unbounded]
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
    #[pallet::unbounded]
    pub(crate) type PendingCollatorContainerChain<T: Config> =
        StorageValue<_, Option<AssignedCollators<T::AccountId>>, ValueQuery>;

    /// Randomness from previous block. Used to shuffle collators on session change.
    /// Should only be set on the last block of each session and should be killed on the on_initialize of the next block.
    /// The default value of [0; 32] disables randomness in the pallet.
    #[pallet::storage]
    pub(crate) type Randomness<T: Config> = StorageValue<_, [u8; 32], ValueQuery>;

    /// Ratio of assigned collators to max collators.
    #[pallet::storage]
    pub type CollatorFullnessRatio<T: Config> = StorageValue<_, Perbill, OptionQuery>;

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
        pub(crate) fn enough_collators_for_all_chains(
            bulk_paras: &[ChainNumCollators],
            pool_paras: &[ChainNumCollators],
            target_session_index: T::SessionIndex,
            number_of_collators: u32,
            collators_per_container: u32,
            collators_per_parathread: u32,
        ) -> bool {
            number_of_collators
                >= T::HostConfiguration::min_collators_for_orchestrator(target_session_index)
                    .saturating_add(collators_per_container.saturating_mul(bulk_paras.len() as u32))
                    .saturating_add(
                        collators_per_parathread.saturating_mul(pool_paras.len() as u32),
                    )
        }

        /// Takes the bulk paras (parachains) and pool paras (parathreads)
        /// and checks if we if a) Do we have enough collators? b) Do we have enough cores?
        /// If either of the answer is yes. We  separately sort bulk_paras and pool_paras and
        /// then append the two vectors.
        pub(crate) fn order_paras_with_core_config(
            mut bulk_paras: Vec<ChainNumCollators>,
            mut pool_paras: Vec<ChainNumCollators>,
            core_allocation_configuration: &CoreAllocationConfiguration,
            target_session_index: T::SessionIndex,
            number_of_collators: u32,
            collators_per_container: u32,
            collators_per_parathread: u32,
        ) -> (Vec<ChainNumCollators>, bool) {
            let core_count = core_allocation_configuration.core_count;
            let max_number_of_bulk_paras = core_allocation_configuration
                .max_parachain_percentage
                .mul(core_count);

            let enough_cores_for_bulk_paras = bulk_paras.len() <= max_number_of_bulk_paras as usize;

            let enough_collators = Self::enough_collators_for_all_chains(
                &bulk_paras,
                &pool_paras,
                target_session_index,
                number_of_collators,
                collators_per_container,
                collators_per_parathread,
            );

            // We should charge tip if parachain demand exceeds the `max_number_of_bulk_paras` OR
            // if `num_collators` is not enough to satisfy  collation need of all paras.
            let should_charge_tip = !enough_cores_for_bulk_paras || !enough_collators;

            // Currently, we are sorting both bulk and pool paras by tip, even when for example
            // only number of bulk paras are restricted due to core availability since we deduct tip from
            // all paras.
            // We need to sort both separately as we have fixed space for parachains at the moment
            // which means even when we have some parathread cores empty we cannot schedule parachain there.
            if should_charge_tip {
                bulk_paras.sort_by(|a, b| {
                    T::CollatorAssignmentTip::get_para_tip(b.para_id)
                        .cmp(&T::CollatorAssignmentTip::get_para_tip(a.para_id))
                });

                pool_paras.sort_by(|a, b| {
                    T::CollatorAssignmentTip::get_para_tip(b.para_id)
                        .cmp(&T::CollatorAssignmentTip::get_para_tip(a.para_id))
                });
            }

            bulk_paras.truncate(max_number_of_bulk_paras as usize);
            // We are not truncating pool paras, since their workload is not continuous one core
            // can be shared by many paras during the session.

            let chains: Vec<_> = bulk_paras.into_iter().chain(pool_paras).collect();

            (chains, should_charge_tip)
        }

        pub(crate) fn order_paras(
            bulk_paras: Vec<ChainNumCollators>,
            pool_paras: Vec<ChainNumCollators>,
            target_session_index: T::SessionIndex,
            number_of_collators: u32,
            collators_per_container: u32,
            collators_per_parathread: u32,
        ) -> (Vec<ChainNumCollators>, bool) {
            // Are there enough collators to satisfy the minimum demand?
            let enough_collators_for_all_chain = Self::enough_collators_for_all_chains(
                &bulk_paras,
                &pool_paras,
                target_session_index,
                number_of_collators,
                collators_per_container,
                collators_per_parathread,
            );

            let mut chains: Vec<_> = bulk_paras.into_iter().chain(pool_paras).collect();

            // Prioritize paras by tip on congestion
            // As of now this doesn't distinguish between bulk paras and pool paras
            if !enough_collators_for_all_chain {
                chains.sort_by(|a, b| {
                    T::CollatorAssignmentTip::get_para_tip(b.para_id)
                        .cmp(&T::CollatorAssignmentTip::get_para_tip(a.para_id))
                });
            }

            (chains, !enough_collators_for_all_chain)
        }

        /// Assign new collators
        /// collators should be queued collators
        pub fn assign_collators(
            current_session_index: &T::SessionIndex,
            random_seed: [u8; 32],
            collators: Vec<T::AccountId>,
        ) -> SessionChangeOutcome<T> {
            let maybe_core_allocation_configuration = T::CoreAllocationConfiguration::get();
            // We work with one session delay to calculate assignments
            let session_delay = T::SessionIndex::one();
            let target_session_index = current_session_index.saturating_add(session_delay);

            let collators_per_container =
                T::HostConfiguration::collators_per_container(target_session_index);
            let collators_per_parathread =
                T::HostConfiguration::collators_per_parathread(target_session_index);

            // We get the containerChains that we will have at the target session
            let container_chains =
                T::ContainerChains::session_container_chains(target_session_index);
            let num_total_registered_paras = container_chains
                .parachains
                .len()
                .saturating_add(container_chains.parathreads.len())
                as u32;
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
            T::ParaIdAssignmentHooks::pre_assignment(
                &mut container_chain_ids,
                &old_assigned_para_ids,
            );
            // TODO: parathreads should be treated a bit differently, they don't need to have the same amount of credits
            // as parathreads because they will not be producing blocks on every slot.
            T::ParaIdAssignmentHooks::pre_assignment(&mut parathreads, &old_assigned_para_ids);

            let mut shuffle_collators = None;
            // If the random_seed is all zeros, we don't shuffle the list of collators nor the list
            // of container chains.
            // This should only happen in tests_without_core_config, and in the genesis block.
            if random_seed != [0; 32] {
                let mut rng: ChaCha20Rng = SeedableRng::from_seed(random_seed);
                container_chain_ids.shuffle(&mut rng);
                parathreads.shuffle(&mut rng);
                shuffle_collators = Some(move |collators: &mut Vec<T::AccountId>| {
                    collators.shuffle(&mut rng);
                })
            }

            let orchestrator_chain: ChainNumCollators = if T::ForceEmptyOrchestrator::get() {
                ChainNumCollators {
                    para_id: T::SelfParaId::get(),
                    min_collators: 0u32,
                    max_collators: 0u32,
                    parathread: false,
                }
            } else {
                ChainNumCollators {
                    para_id: T::SelfParaId::get(),
                    min_collators: T::HostConfiguration::min_collators_for_orchestrator(
                        target_session_index,
                    ),
                    max_collators: T::HostConfiguration::max_collators_for_orchestrator(
                        target_session_index,
                    ),
                    parathread: false,
                }
            };

            // Initialize list of chains as `[container1, container2, parathread1, parathread2]`.
            // The order means priority: the first chain in the list will be the first one to get assigned collators.
            // Chains will not be assigned less than `min_collators`, except the orchestrator chain.
            // First all chains will be assigned `min_collators`, and then the first one will be assigned up to `max`,
            // then the second one, and so on.
            let mut bulk_paras = vec![];
            let mut pool_paras = vec![];

            for para_id in &container_chain_ids {
                bulk_paras.push(ChainNumCollators {
                    para_id: *para_id,
                    min_collators: collators_per_container,
                    max_collators: collators_per_container,
                    parathread: false,
                });
            }
            for para_id in &parathreads {
                pool_paras.push(ChainNumCollators {
                    para_id: *para_id,
                    min_collators: collators_per_parathread,
                    max_collators: collators_per_parathread,
                    parathread: true,
                });
            }

            let (chains, need_to_charge_tip) =
                if let Some(core_allocation_configuration) = maybe_core_allocation_configuration {
                    Self::order_paras_with_core_config(
                        bulk_paras,
                        pool_paras,
                        &core_allocation_configuration,
                        target_session_index,
                        collators.len() as u32,
                        collators_per_container,
                        collators_per_parathread,
                    )
                } else {
                    Self::order_paras(
                        bulk_paras,
                        pool_paras,
                        target_session_index,
                        collators.len() as u32,
                        collators_per_container,
                        collators_per_parathread,
                    )
                };

            // We assign new collators
            // we use the config scheduled at the target_session_index
            let full_rotation =
                T::ShouldRotateAllCollators::should_rotate_all_collators(target_session_index);
            if full_rotation {
                log::info!(
                    "Collator assignment: rotating collators. Session {:?}, Seed: {:?}",
                    current_session_index.encode(),
                    random_seed
                );
            } else {
                log::info!(
                    "Collator assignment: keep old assigned. Session {:?}, Seed: {:?}",
                    current_session_index.encode(),
                    random_seed
                );
            }

            let full_rotation_mode = if full_rotation {
                T::HostConfiguration::full_rotation_mode(target_session_index)
            } else {
                // On sessions where there is no rotation, we try to keep all collators assigned to the same chains
                FullRotationModes::keep_all()
            };

            Self::deposit_event(Event::NewPendingAssignment {
                random_seed,
                full_rotation,
                target_session: target_session_index,
                full_rotation_mode: full_rotation_mode.clone(),
            });

            let new_assigned = Assignment::<T>::assign_collators_always_keep_old(
                collators,
                orchestrator_chain,
                chains,
                old_assigned.clone(),
                shuffle_collators,
                full_rotation_mode,
            );

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
            let maybe_tip: Option<BalanceOf<T>> = if !need_to_charge_tip {
                None
            } else {
                assigned_containers
                    .into_keys()
                    .filter_map(T::CollatorAssignmentTip::get_para_tip)
                    .min()
            };

            // TODO: this probably is asking for a refactor
            // only apply the onCollatorAssignedHook if sufficient collators
            T::ParaIdAssignmentHooks::post_assignment(
                &old_assigned_para_ids,
                &mut new_assigned.container_chains,
                &maybe_tip,
            );

            Self::store_collator_fullness(
                &new_assigned,
                T::HostConfiguration::max_collators(target_session_index),
            );

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

        /// Count number of collators assigned to any chain, divide that by `max_collators` and store
        /// in pallet storage.
        fn store_collator_fullness(
            new_assigned: &AssignedCollators<T::AccountId>,
            max_collators: u32,
        ) {
            // Count number of assigned collators
            let mut num_collators = 0;
            num_collators.saturating_accrue(new_assigned.orchestrator_chain.len());
            for collators in new_assigned.container_chains.values() {
                num_collators.saturating_accrue(collators.len());
            }

            let mut num_collators = num_collators as u32;
            if num_collators > max_collators {
                // Shouldn't happen but just in case
                num_collators = max_collators;
            }

            let ratio = Perbill::from_rational(num_collators, max_collators);

            CollatorFullnessRatio::<T>::put(ratio);
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
            let random_seed = T::Randomness::take_randomness();
            let num_collators = collators.len();
            let assigned_collators = Self::assign_collators(session_index, random_seed, collators);
            let num_total_registered_paras = assigned_collators.num_total_registered_paras;

            frame_system::Pallet::<T>::register_extra_weight_unchecked(
                T::WeightInfo::new_session(num_collators as u32, num_total_registered_paras),
                DispatchClass::Mandatory,
            );

            assigned_collators
        }

        pub fn collator_container_chain() -> AssignedCollators<T::AccountId> {
            CollatorContainerChain::<T>::get()
        }

        pub fn pending_collator_container_chain() -> Option<AssignedCollators<T::AccountId>> {
            PendingCollatorContainerChain::<T>::get()
        }
    }

    impl<T: Config> GetContainerChainAuthor<T::AccountId> for Pallet<T> {
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
            weight.saturating_accrue(T::Randomness::prepare_randomness_weight(n));

            weight
        }

        fn on_finalize(n: BlockNumberFor<T>) {
            // If the next block is a session change, read randomness and store in pallet storage
            T::Randomness::prepare_randomness(n);
        }
    }

    impl<T: Config> GetContainerChainsWithCollators<T::AccountId> for Pallet<T> {
        fn container_chains_with_collators(
            for_session: ForSession,
        ) -> Vec<(ParaId, Vec<T::AccountId>)> {
            // If next session has None then current session data will stay.
            let chains = (for_session == ForSession::Next)
                .then(PendingCollatorContainerChain::<T>::get)
                .flatten()
                .unwrap_or_else(CollatorContainerChain::<T>::get);

            chains.container_chains.into_iter().collect()
        }

        fn get_all_collators_assigned_to_chains(for_session: ForSession) -> BTreeSet<T::AccountId> {
            let mut all_chains: Vec<T::AccountId> =
                Self::container_chains_with_collators(for_session)
                    .iter()
                    .flat_map(|(_para_id, collators)| collators.iter())
                    .cloned()
                    .collect();
            all_chains.extend(
                Self::collator_container_chain()
                    .orchestrator_chain
                    .iter()
                    .cloned(),
            );
            all_chains.into_iter().collect()
        }

        #[cfg(feature = "runtime-benchmarks")]
        fn set_container_chains_with_collators(
            for_session: ForSession,
            container_chains: &[(ParaId, Vec<T::AccountId>)],
        ) {
            match for_session {
                ForSession::Current => {
                    let mut collators = CollatorContainerChain::<T>::get();
                    collators.container_chains = container_chains.iter().cloned().collect();
                    CollatorContainerChain::<T>::put(collators);
                }
                ForSession::Next => {
                    let mut collators =
                        PendingCollatorContainerChain::<T>::get().unwrap_or_default();
                    collators.container_chains = container_chains.iter().cloned().collect();
                    PendingCollatorContainerChain::<T>::put(Some(collators));
                }
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

/// Only works on parachains because in relaychains it is not possible to know for sure if the next
/// block will be in the same session as the current one, as it depends on slots and validators can
/// skip slots.
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

pub trait CollatorAssignmentRandomness<BlockNumber> {
    /// Called in on_initialize, returns weight needed by prepare_randomness call.
    fn prepare_randomness_weight(n: BlockNumber) -> Weight;
    /// Called in on_finalize.
    /// Prepares randomness for the next block if the next block is a new session start.
    fn prepare_randomness(n: BlockNumber);
    /// Called once at the start of each session in on_initialize of pallet_initializer
    fn take_randomness() -> [u8; 32];
}

impl<BlockNumber> CollatorAssignmentRandomness<BlockNumber> for () {
    fn prepare_randomness_weight(_n: BlockNumber) -> Weight {
        Weight::zero()
    }
    fn prepare_randomness(_n: BlockNumber) {}
    fn take_randomness() -> [u8; 32] {
        [0; 32]
    }
}

/// Parachain randomness impl.
///
/// Reads relay chain randomness in the last block of the session and stores it in pallet storage.
/// When new session starts, takes that value from storage removing it.
/// Relay randomness cannot be accessed in `on_initialize`, so `prepare_randomness` is executed in
/// `on_finalize`, with `prepare_randomness_weight` reserving the weight needed.
pub struct ParachainRandomness<T, Runtime>(PhantomData<(T, Runtime)>);

impl<BlockNumber, T, Runtime> CollatorAssignmentRandomness<BlockNumber>
    for ParachainRandomness<T, Runtime>
where
    BlockNumber: Saturating + One,
    T: GetRandomnessForNextBlock<BlockNumber>,
    Runtime: frame_system::Config + crate::Config,
{
    fn prepare_randomness_weight(n: BlockNumber) -> Weight {
        let mut weight = Weight::zero();

        if T::should_end_session(n.saturating_add(One::one())) {
            weight.saturating_accrue(Runtime::DbWeight::get().reads_writes(1, 1));
        }

        weight
    }

    fn prepare_randomness(n: BlockNumber) {
        if T::should_end_session(n.saturating_add(One::one())) {
            let random_seed = T::get_randomness();
            Randomness::<Runtime>::put(random_seed);
        }
    }

    fn take_randomness() -> [u8; 32] {
        Randomness::<Runtime>::take()
    }
}

/// Solochain randomness.
///
/// Uses current block randomness. This randomness exists in `on_initialize` so we don't need to
/// `prepare_randomness` in the previous block.
pub struct SolochainRandomness<T>(PhantomData<T>);

impl<BlockNumber, T> CollatorAssignmentRandomness<BlockNumber> for SolochainRandomness<T>
where
    T: Get<[u8; 32]>,
{
    fn prepare_randomness_weight(_n: BlockNumber) -> Weight {
        Weight::zero()
    }

    fn prepare_randomness(_n: BlockNumber) {}

    fn take_randomness() -> [u8; 32] {
        #[cfg(feature = "runtime-benchmarks")]
        if let Some(x) =
            frame_support::storage::unhashed::take(b"__bench_collator_assignment_randomness")
        {
            return x;
        }

        T::get()
    }
}
