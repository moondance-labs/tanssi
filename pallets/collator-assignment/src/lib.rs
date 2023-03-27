#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::*;
use scale_info::prelude::collections::BTreeMap;
use sp_runtime::traits::AtLeast32BitUnsigned;
use sp_runtime::traits::One;
use sp_runtime::Saturating;
use sp_std::prelude::*;

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait GetHostConfiguration<SessionIndex> {
    fn orchestrator_chain_collators(session_index: SessionIndex) -> u32;
    fn collators_per_container(session_index: SessionIndex) -> u32;
}

pub trait GetCollators<AccountId, SessionIndex> {
    fn collators(session_index: SessionIndex) -> Vec<AccountId>;
}

pub trait GetContainerChains<SessionIndex> {
    // TODO: import ParaId type
    fn container_chains(session_index: SessionIndex) -> Vec<u32>;
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
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
        type Collators: GetCollators<Self::AccountId, Self::SessionIndex>;
        type ContainerChains: GetContainerChains<Self::SessionIndex>;
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
    pub(crate) type PendingCollatorContainerChain<T: Config> =
        StorageValue<_, Option<AssignedCollators<T::AccountId>>, ValueQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    #[derive(Clone, Encode, Decode, PartialEq, sp_core::RuntimeDebug, scale_info::TypeInfo)]
    #[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
    pub struct AssignedCollators<AccountId> {
        pub orchestrator_chain: Vec<AccountId>,
        pub container_chains: BTreeMap<u32, Vec<AccountId>>,
    }

    // Manual default impl that does not require AccountId: Default
    impl<AccountId> Default for AssignedCollators<AccountId> {
        fn default() -> Self {
            Self {
                orchestrator_chain: Default::default(),
                container_chains: Default::default(),
            }
        }
    }

    impl<AccountId> AssignedCollators<AccountId>
    where
        AccountId: PartialEq,
    {
        fn find_collator(&self, x: &AccountId) -> bool {
            self.orchestrator_chain.iter().any(|a| a == x)
                || self
                    .container_chains
                    .iter()
                    .any(|(_id, cs)| cs.iter().any(|a| a == x))
        }

        fn remove_container_chains_not_in_list(&mut self, container_chains: &[u32]) {
            self.container_chains
                .retain(|id, _cs| container_chains.contains(id));
        }

        fn remove_collators_not_in_list(&mut self, collators: &[AccountId]) {
            self.orchestrator_chain.retain(|c| collators.contains(c));
            for (_id, cs) in self.container_chains.iter_mut() {
                cs.retain(|c| collators.contains(c))
            }
        }

        fn remove_orchestrator_chain_excess_collators(&mut self, num_orchestrator_chain: usize) {
            self.orchestrator_chain.truncate(num_orchestrator_chain);
        }

        fn remove_container_chain_excess_collators(&mut self, num_each_container_chain: usize) {
            for (_id, cs) in self.container_chains.iter_mut() {
                cs.truncate(num_each_container_chain);
            }
        }

        fn fill_orchestrator_chain_collators<I>(
            &mut self,
            num_orchestrator_chain: usize,
            next_collator: &mut I,
        ) where
            I: Iterator<Item = AccountId>,
        {
            while self.orchestrator_chain.len() < num_orchestrator_chain {
                if let Some(nc) = next_collator.next() {
                    self.orchestrator_chain.push(nc);
                } else {
                    return;
                }
            }
        }

        fn fill_container_chain_collators<I>(
            &mut self,
            num_each_container_chain: usize,
            next_collator: &mut I,
        ) where
            I: Iterator<Item = AccountId>,
        {
            for (_id, cs) in self.container_chains.iter_mut() {
                while cs.len() < num_each_container_chain {
                    if let Some(nc) = next_collator.next() {
                        cs.push(nc);
                    } else {
                        return;
                    }
                }
            }
        }

        fn add_new_container_chains(&mut self, container_chains: &[u32]) {
            for para_id in container_chains {
                self.container_chains.entry(*para_id).or_default();
            }
        }
    }

    impl<T: Config> Pallet<T> {
        /// Assign new collators
        pub fn assign_collators(current_session_index: &T::SessionIndex) {
            let session_delay = T::SessionIndex::one();
            let target_session_index = current_session_index.saturating_add(session_delay);
            let collators = T::Collators::collators(target_session_index);
            let container_chain_ids = T::ContainerChains::container_chains(target_session_index);
            let old_assigned = Self::read_assigned_collators();
            let new_assigned = Self::assign_collators_always_keep_old(
                collators,
                &container_chain_ids,
                T::HostConfiguration::orchestrator_chain_collators(target_session_index) as usize,
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
                pending = Some(new_assigned);
                pending_changed = true;
            }
            // Update PendingCollatorContainerChain, if it changed
            if pending_changed {
                PendingCollatorContainerChain::<T>::put(pending);
            }
        }

        /// Assign new collators to missing container_chains.
        /// Old collators always have preference to remain on the same chain.
        /// If there are no missing collators, nothing is changed.
        fn assign_collators_always_keep_old(
            collators: Vec<T::AccountId>,
            container_chain_ids: &[u32],
            num_orchestrator_chain: usize,
            num_each_container_chain: usize,
            old_assigned: AssignedCollators<T::AccountId>,
        ) -> AssignedCollators<T::AccountId> {
            // TODO: the performance of this function is sad, could be improved by having sets of
            // old_collators and new_collators instead of doing array.contains() every time.
            let mut new_assigned = old_assigned;
            new_assigned.remove_collators_not_in_list(&collators);
            new_assigned.remove_container_chains_not_in_list(container_chain_ids);
            // Only need to do these two if the config params change
            new_assigned.remove_orchestrator_chain_excess_collators(num_orchestrator_chain);
            new_assigned.remove_container_chain_excess_collators(num_each_container_chain);

            // Collators that are not present in old_assigned
            // TODO: unless we save all the old_collators somewhere, it is still possible for a
            // collator to change from container_chain 1001 to None to 1002
            // And ideally that should not happen until the automatic chain rotation is implemented
            // But the current implementation allows changes, even without passing through None
            let mut new_collators = vec![];
            for c in collators {
                if !new_assigned.find_collator(&c) {
                    new_collators.push(c);
                }
            }

            let mut new_collators = new_collators.into_iter();
            new_assigned
                .fill_orchestrator_chain_collators(num_orchestrator_chain, &mut new_collators);
            new_assigned.add_new_container_chains(container_chain_ids);
            new_assigned
                .fill_container_chain_collators(num_each_container_chain, &mut new_collators);

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

        pub fn initializer_on_new_session(session_index: &T::SessionIndex) {
            Self::assign_collators(session_index);
        }
    }
}
