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
    frame_support::pallet_prelude::*,
    sp_runtime::{
        traits::{AtLeast32BitUnsigned, One, Zero},
        Saturating,
    },
    sp_std::{prelude::*, vec},
    tp_collator_assignment::AssignedCollators,
    tp_traits::{
        GetContainerChainAuthor, GetHostConfiguration, GetSessionContainerChains, ParaId, Slot,
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
        type SessionIndex: parity_scale_codec::FullCodec + TypeInfo + Copy + AtLeast32BitUnsigned;
        // `SESSION_DELAY` is used to delay any changes to Paras registration or configurations.
        // Wait until the session index is 2 larger then the current index to apply any changes,
        // which guarantees that at least one full session has passed before any changes are applied.
        type HostConfiguration: GetHostConfiguration<Self::SessionIndex>;
        type ContainerChains: GetSessionContainerChains<Self::SessionIndex>;
        /// The weight information of this pallet.
        type WeightInfo: WeightInfo;
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

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    /// A struct that holds the assignment that is active after the session change and optionally
    /// the assignment that becomes active after the next session change.
    pub struct SessionChangeOutcome<T: Config> {
        /// New active assignment.
        pub active_assignment: AssignedCollators<T::AccountId>,
        /// Next session active assignment.
        pub next_assignment: AssignedCollators<T::AccountId>,
    }

    impl<T: Config> Pallet<T> {
        /// Assign new collators
        /// collators should be queued collators
        pub fn assign_collators(
            current_session_index: &T::SessionIndex,
            collators: Vec<T::AccountId>,
        ) -> SessionChangeOutcome<T> {
            // We work with one session delay to calculate assignments
            let session_delay = T::SessionIndex::one();
            let target_session_index = current_session_index.saturating_add(session_delay);
            // We get the containerChains that we will have at the target session
            let container_chain_ids =
                T::ContainerChains::session_container_chains(target_session_index);
            // We read current assigned collators
            let old_assigned = Self::read_assigned_collators();
            // We assign new collators
            // we use the config scheduled at the target_session_index
            let new_assigned = Self::assign_collators_always_keep_old(
                collators,
                &container_chain_ids,
                T::HostConfiguration::min_collators_for_orchestrator(target_session_index) as usize,
                T::HostConfiguration::max_collators_for_orchestrator(target_session_index) as usize,
                T::HostConfiguration::collators_per_container(target_session_index) as usize,
                old_assigned.clone(),
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
                };
            }

            SessionChangeOutcome {
                active_assignment: old_assigned,
                next_assignment: new_assigned,
            }
        }

        /// Assign new collators to missing container_chains.
        /// Old collators always have preference to remain on the same chain.
        /// If there are no missing collators, nothing is changed.
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
            // TODO: unless we save all the old_collators somewhere, it is still possible for a
            // collator to change from container_chain 1001 to None to 1002
            // And ideally that should not happen until the automatic chain rotation is implemented
            // But the current implementation allows changes, even without passing through None
            let mut new_collators = vec![];
            for c in collators {
                if !new_assigned.find_collator(&c) && !extra_orchestrator_collators.contains(&c) {
                    new_collators.push(c);
                }
            }

            // Fill orchestrator chain collators up to min_num_orchestrator_chain
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
            new_assigned.add_new_container_chains(container_chain_ids);
            new_assigned.fill_container_chain_collators(
                num_each_container_chain,
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
                .reorganize_incomplete_container_chains_collators(num_each_container_chain);

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
            let num_collators = collators.len();
            let assigned_collators = Self::assign_collators(session_index, collators);
            let num_parachains = assigned_collators.next_assignment.container_chains.len();

            frame_system::Pallet::<T>::register_extra_weight_unchecked(
                T::WeightInfo::new_session(num_collators as u32, num_parachains as u32),
                DispatchClass::Mandatory,
            );

            assigned_collators
        }
    }

    impl<T: Config> GetContainerChainAuthor<T::AccountId> for Pallet<T> {
        fn author_for_slot(slot: Slot, para_id: ParaId) -> Option<T::AccountId> {
            let assigned_collators = Pallet::<T>::collator_container_chain();
            let collators = assigned_collators.container_chains.get(&para_id)?;
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
}
