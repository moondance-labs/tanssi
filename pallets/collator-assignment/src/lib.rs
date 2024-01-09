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
    sp_std::{
        collections::btree_map::BTreeMap, collections::btree_set::BTreeSet, fmt::Debug, prelude::*,
        vec,
    },
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
            let mut parathreads = T::ContainerChains::session_parathreads(target_session_index);
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
                parathreads.shuffle(&mut rng);
            }

            // We read current assigned collators
            let old_assigned = Self::read_assigned_collators();
            // Initialize list of container chains as `[orchestrator, container1, container2, parathread1, parathread2]`.
            // The order means priority: the first chain in the list will be the first one to get assigned collators.
            // Chains will not be assigned less than `min_collators`, except the orchestrator chain.
            // First all chains will be assigned `min_collators`, and then the first one will be assigned up to `max`,
            // then the second one, and so on.
            let mut container_chains = vec![ContainerChain {
                para_id: T::SelfParaId::get(),
                min_collators: T::HostConfiguration::min_collators_for_orchestrator(
                    target_session_index,
                ) as usize,
                max_collators: T::HostConfiguration::max_collators_for_orchestrator(
                    target_session_index,
                ) as usize,
            }];
            let collators_per_container =
                T::HostConfiguration::collators_per_container(target_session_index) as usize;
            for para_id in &container_chain_ids {
                container_chains.push(ContainerChain {
                    para_id: *para_id,
                    min_collators: collators_per_container,
                    max_collators: collators_per_container,
                });
            }
            let collators_per_parathread =
                T::HostConfiguration::collators_per_parathread(target_session_index) as usize;
            for para_id in &parathreads {
                container_chains.push(ContainerChain {
                    para_id: *para_id,
                    min_collators: collators_per_parathread,
                    max_collators: collators_per_parathread,
                });
            }
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

                    Self::assign_collators_rotate_all(collators, container_chains)
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
                        container_chains,
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
            container_chains: Vec<ContainerChain>,
        ) -> AssignedCollators<T::AccountId> {
            // This is just the "always_keep_old" algorithm but with an empty "old"
            let old_assigned = Default::default();

            Self::assign_collators_always_keep_old(collators, container_chains, old_assigned)
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
            container_chains: Vec<ContainerChain>,
            mut old_assigned: AssignedCollators<T::AccountId>,
        ) -> AssignedCollators<T::AccountId> {
            let all_para_ids: Vec<ParaId> = container_chains.iter().map(|cc| cc.para_id).collect();
            /*
            // Segregate collators into old and new. Collators from chains that were just deregistered will be
            // considered new.
            // Use total number of collators to estimate which chains will be assigned collators. The rest of chains
            // will have their collators removed. Note, take into account variable `min_collators`: if the chains
            // require [5, 2, 2, 1, 1] and we have 5+2+1 collators, we want to assign [5, 2, 0, 1, 0].
            if num_collators < required_num_collators {
                // Delete excess container chains (will be filled after the algorithm is done for UI purposes)
            }
            if num_collators > required_num_collators {
                // Mutate the min_collators of each chain in order, increasing it up to max_collators for that chain
            }
            // For each chain, first reassign the old from this chain, then pick new collators up to the limit.
            for cc in &container_chains {
                new_assigned.fill(cc, old_assigned.get(cc.para_id).chain(new_collators));
            }
             */
            let collators_set = BTreeSet::from_iter(&collators);
            // The rest of this function mostly treats orchestrator chain as another container chain, so move it into
            // `old_assigned.container_chains`
            let old_orchestrator_assigned = std::mem::take(&mut old_assigned.orchestrator_chain);
            old_assigned
                .container_chains
                .insert(T::SelfParaId::get(), old_orchestrator_assigned);
            old_assigned.remove_container_chains_not_in_list(&all_para_ids);
            let mut old_assigned = old_assigned.container_chains;

            // old_assigned.remove_collators_not_in_set
            for (_id, cs) in old_assigned.iter_mut() {
                cs.retain(|c| collators_set.contains(c));
            }

            // The collators min and max may have changed, but we will keep collators in old_assigned for now

            // At this point we have:
            // * collators, collators_set: list of all collators assigned or not
            // * old_assigned: subset of the collators that were assigned to some para id in the previous session
            // * container_chains: para ids that need collators for this session, and may or may not have them in the
            // previous session

            // Let's count how many container chains we can support with the current number of collators
            let mut available_collators = collators.len();
            // Edge case: num collators less than min orchestrator collators: fill as much as we can
            assert_eq!(container_chains[0].para_id, T::SelfParaId::get());
            let min_orchestrator_collators = container_chains[0].min_collators;
            if available_collators < min_orchestrator_collators {
                available_collators = 0;
            } else {
                available_collators -= min_orchestrator_collators;
            }

            let mut container_chains_without_collators = BTreeSet::new();
            // Skipping orchestrator chain because it was handled above
            for cc in container_chains.iter().skip(1) {
                if available_collators >= cc.min_collators {
                    available_collators -= cc.min_collators;
                } else {
                    container_chains_without_collators.insert(cc.para_id);
                    // Do not break here because we want to push all the remaining para_ids
                }
            }
            // We remove container_chains_without_collators from old_assigned
            for para_id in container_chains_without_collators.iter() {
                old_assigned.remove(para_id);
            }
            let container_chains_with_collators: Vec<_> = container_chains
                .iter()
                .filter(|cc| !container_chains_without_collators.contains(&cc.para_id))
                .collect();

            // At this point we have:
            // * collators, collators_set: list of all collators assigned or not
            // * old_assigned: subset of the collators that were assigned to some para id in the previous session, and
            // will likely be assigned to that same para id in this session (except if the min/max changed)
            // * container_chains_with_collators: para ids that need collators for this session, and may or may not have
            // had them in the previous session
            // * container_chains_without_collators: para ids that will have 0 collators this session
            let mut required_collators_min = 0;
            for cc in &container_chains_with_collators {
                required_collators_min += cc.min_collators;
            }

            // Ensure the first `min_orchestrator_collators` of orchestrator chain are invulnerables
            Self::prioritize_invulnerables(&collators, &container_chains, &mut old_assigned);

            let new_assigned_chains = {
                // Edge case: num collators less than min orchestrator collators: fill as much as we can
                assert_eq!(container_chains[0].para_id, T::SelfParaId::get());
                let min_orchestrator_collators = container_chains[0].min_collators;

                if collators.len() < min_orchestrator_collators {
                    let orch_min = vec![(container_chains[0].para_id, collators.len() as u32)];

                    Self::assign_fill(collators, orch_min, old_assigned)
                } else {
                    // Set a variable number of collators, some chains at max and some chains at min
                    let mut required_collators_remainder = collators.len() - required_collators_min;
                    let mut container_chains_variable = vec![];
                    for cc in &container_chains_with_collators {
                        let mut num = cc.min_collators;
                        while required_collators_remainder > 0 && num < cc.max_collators {
                            // Move 1 collator from remainder to this chain
                            required_collators_remainder -= 1;
                            num += 1;
                        }
                        container_chains_variable.push((cc.para_id, num as u32))
                    }

                    Self::assign_fill(collators, container_chains_variable, old_assigned)
                }
            };

            let mut new_assigned = AssignedCollators::default();
            new_assigned.container_chains = new_assigned_chains;

            // Add container chains with 0 collators so that they are shown in UI
            for para_id in all_para_ids {
                new_assigned.container_chains.entry(para_id).or_default();
            }

            // The rest of this function mostly treats orchestrator chain as another container chain, remove it from
            // container chains before returning the final assignment.
            let orchestrator_assigned = new_assigned
                .container_chains
                .remove(&T::SelfParaId::get())
                .unwrap();
            new_assigned.orchestrator_chain = orchestrator_assigned;

            new_assigned
        }

        /// Ensure orchestrator chain has min_orchestrator invulnerables
        /// Get invulnerables from:
        /// * old_assigned in orchestrator
        /// * new collators
        /// * old_assigned elsewhere
        fn prioritize_invulnerables(
            collators: &[T::AccountId],
            container_chains: &[ContainerChain],
            old_assigned: &mut BTreeMap<ParaId, Vec<T::AccountId>>,
        ) {
            // TODO: clean this up, maybe change remove_invulnerables trait into something more ergonomic
            let min_orchestrator_collators = container_chains[0].min_collators;
            let invulnerables = T::RemoveInvulnerables::remove_invulnerables(
                &mut old_assigned.get(&T::SelfParaId::get()).unwrap().clone(),
                min_orchestrator_collators,
            );
            insert_invulnerables(
                old_assigned.get_mut(&T::SelfParaId::get()).unwrap(),
                &invulnerables,
            );
            if invulnerables.len() < min_orchestrator_collators {
                // Not enough invulnerables currently assigned, get rest from new_collators
                let mut new_collators = collators.to_vec();
                for (_id, cs) in old_assigned.iter() {
                    new_collators.retain(|c| !cs.contains(c));
                }
                let num_missing_invulnerables = min_orchestrator_collators - invulnerables.len();
                let missing_invulnerables = T::RemoveInvulnerables::remove_invulnerables(
                    &mut new_collators,
                    num_missing_invulnerables,
                );
                let missing_invulnerables_len = missing_invulnerables.len();
                for invulnerable in missing_invulnerables.into_iter().rev() {
                    let orch_collators = old_assigned.get_mut(&T::SelfParaId::get()).unwrap();
                    orch_collators.insert(invulnerables.len(), invulnerable);
                }

                if missing_invulnerables_len < num_missing_invulnerables {
                    // Still not enough invulnerables, try to get an invulnerable that is currently assigned somewhere else

                    let num_missing_invulnerables =
                        num_missing_invulnerables - missing_invulnerables_len;
                    let mut collators = collators.to_vec();
                    let orch_collators_set = BTreeSet::from_iter(
                        old_assigned
                            .get(&T::SelfParaId::get())
                            .unwrap()
                            .iter()
                            .cloned(),
                    );
                    collators.retain(|c| {
                        // Remove collators already assigned to this chain
                        !orch_collators_set.contains(c)
                    });
                    let reassigned_invulnerables = T::RemoveInvulnerables::remove_invulnerables(
                        &mut collators,
                        num_missing_invulnerables,
                    );

                    if reassigned_invulnerables.is_empty() {
                        // If at this point we still do not have enough invulnerables, it means that there are no invulnerables, so no problem
                    } else {
                        let reassigned_invulnerables_set =
                            BTreeSet::from_iter(reassigned_invulnerables.iter().cloned());

                        // In this case we must delete the old assignment of the invulnerables
                        // old_assigned.remove_collators_in_set
                        for (_id, cs) in old_assigned.iter_mut() {
                            cs.retain(|c| !reassigned_invulnerables_set.contains(c));
                        }

                        for invulnerable in reassigned_invulnerables.into_iter().rev() {
                            let orch_collators =
                                old_assigned.get_mut(&T::SelfParaId::get()).unwrap();
                            orch_collators.insert(
                                invulnerables.len() + missing_invulnerables_len,
                                invulnerable,
                            );
                        }
                    }
                }
            }
        }

        /// Assign collators assuming that the number of collators is greater than or equal to the required.
        /// The order of both container chains and collators is important to ensure randomness when `old_assigned` is
        /// empty.
        ///
        /// # Params
        ///
        /// * `old_assigned` does not need to be a subset of `collators`: collators are checked and removed.
        /// * `old_assigned` does not need to be a subset of `container_chains`, unused para ids are removed. Collators
        /// assigned to a para_id not present in `container_chains` may be reassigned to another para_id.
        /// * `container_chains` `num_collators` can be 0. In that case an empty vec is returned for that para id.
        /// * `old_assigned` must not have duplicate collators.
        ///
        /// # Panics
        ///
        /// This function panics if the number of collators is not enough to fill all the container chains.
        fn assign_fill(
            collators: Vec<T::AccountId>,
            container_chains: Vec<(ParaId, u32)>,
            mut old_assigned: BTreeMap<ParaId, Vec<T::AccountId>>,
        ) -> BTreeMap<ParaId, Vec<T::AccountId>> {
            let mut required_collators = 0;
            for (_para_id, num_collators) in container_chains.iter() {
                required_collators += num_collators;
            }

            // This check is required to ensure priority: if the number of collators was less than the required, it is
            // possible that the chain with least priority would be assigned collators (because they are in
            // old_assigned), while some chains with more priority would have no collators.
            assert!(
                collators.len() >= required_collators as usize,
                "assign_fill: not enough collators: {}, required {}, chains: {:?}",
                collators.len(),
                required_collators,
                container_chains
            );

            // Remove para_ids not in list
            let para_ids_set = BTreeSet::from_iter(
                container_chains
                    .iter()
                    .map(|(para_id, _num_collators)| *para_id),
            );
            let mut para_ids_to_remove = vec![];
            for (para_id, _para_id_collators) in old_assigned.iter() {
                if !para_ids_set.contains(para_id) {
                    para_ids_to_remove.push(*para_id);
                }
            }
            for para_id in para_ids_to_remove {
                old_assigned.remove(&para_id);
            }

            // Remove old_assigned collators not in list
            let collators_set = BTreeSet::from_iter(&collators);
            for (_para_id, para_id_collators) in old_assigned.iter_mut() {
                para_id_collators.retain(|x| collators_set.contains(x));
            }

            // Truncate num collators to required
            for (para_id, num_collators) in container_chains.iter() {
                let entry = old_assigned.entry(*para_id).or_default();
                entry.truncate(*num_collators as usize);
            }

            // Remove already assigned from `collators`
            let mut new_collators = collators;
            for (_para_id, para_id_collators) in old_assigned.iter() {
                for collator in para_id_collators {
                    let idx = new_collators
                        .iter()
                        .position(|x| x == collator)
                        .expect("duplicate collator in old_assigned");
                    new_collators.remove(idx);
                }
            }

            let mut next_collator = new_collators.into_iter();

            // Fill missing collators
            for (para_id, num_collators) in container_chains.iter() {
                let cs = old_assigned.entry(*para_id).or_default();

                while cs.len() < *num_collators as usize {
                    // unwrap is safe because we checked that `collators.len() >= required_collators`
                    let nc = next_collator.next().unwrap();
                    cs.push(nc);
                }
            }

            old_assigned
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

// Ensure that invulnerables are always the first in the list
fn insert_invulnerables<T>(oo: &mut Vec<T>, invulnerables: &[T])
where
    T: PartialEq + Clone,
{
    oo.retain(|item| !invulnerables.contains(item));

    let mut new_oo = invulnerables.to_vec();
    new_oo.extend(oo.iter().cloned());

    *oo = new_oo;
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

impl<BlockNumber> GetRandomnessForNextBlock<BlockNumber> for () {
    fn should_end_session(_block_number: BlockNumber) -> bool {
        false
    }

    fn get_randomness() -> [u8; 32] {
        [0; 32]
    }
}

pub struct ContainerChain {
    pub para_id: ParaId,
    pub min_collators: usize,
    // This will only be filled if all the other min have been reached
    pub max_collators: usize,
}
