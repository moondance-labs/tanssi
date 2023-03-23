#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::*;
use frame_support::traits::OneSessionHandler;
use scale_info::prelude::collections::BTreeMap;
use sp_runtime::traits::AtLeast32BitUnsigned;
use sp_runtime::RuntimeAppPublic;
use sp_std::prelude::*;

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait GetHostConfiguration {
    fn moondance_collators() -> u32;
    fn collators_per_container() -> u32;
}

pub trait GetCollators<AccountId> {
    fn collators() -> Vec<AccountId>;
}

pub trait GetContainerChains {
    // TODO: import ParaId type
    fn container_chains() -> Vec<u32>;
}

pub trait GetSessionIndex<SessionIndex> {
    /// Returns current session index.
    fn session_index() -> SessionIndex;
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

        /// The identifier type for an authority.
        type AuthorityId: Member
            + Parameter
            + RuntimeAppPublic
            + MaybeSerializeDeserialize
            + MaxEncodedLen;

        #[pallet::constant]
        type MoondanceParaId: Get<u32>;

        type HostConfiguration: GetHostConfiguration;
        type Collators: GetCollators<Self::AccountId>;
        type ContainerChains: GetContainerChains;
        type CurrentSessionIndex: GetSessionIndex<Self::SessionIndex>;
    }

    #[pallet::storage]
    #[pallet::getter(fn collator_container_chain)]
    pub(crate) type CollatorContainerChain<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, u32>;

    #[pallet::storage]
    #[pallet::getter(fn moondance_collators)]
    pub(crate) type MoondanceCollators<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    #[derive(Debug, Clone, Default)]
    struct AssignedCollators<AccountId> {
        moondance: Vec<AccountId>,
        container_chains: BTreeMap<u32, Vec<AccountId>>,
    }

    impl<AccountId> AssignedCollators<AccountId>
    where
        AccountId: PartialEq,
    {
        fn find_collator(&self, x: &AccountId) -> bool {
            self.moondance.iter().any(|a| a == x)
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
            self.moondance.retain(|c| collators.contains(c));
            for (_id, cs) in self.container_chains.iter_mut() {
                cs.retain(|c| collators.contains(c))
            }
        }

        fn remove_moondance_excess_collators(&mut self, num_moondance: usize) {
            self.moondance.truncate(num_moondance);
        }

        fn remove_container_chain_excess_collators(&mut self, num_each_container_chain: usize) {
            for (_id, cs) in self.container_chains.iter_mut() {
                cs.truncate(num_each_container_chain);
            }
        }

        fn fill_moondance_collators<I>(&mut self, num_moondance: usize, next_collator: &mut I)
        where
            I: Iterator<Item = AccountId>,
        {
            while self.moondance.len() < num_moondance {
                if let Some(nc) = next_collator.next() {
                    self.moondance.push(nc);
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
        pub fn assign_collators() {
            let collators = T::Collators::collators();
            let container_chain_ids = T::ContainerChains::container_chains();

            let (old_assigned, old_num_collators) = Self::read_assigned_collators();

            let AssignedCollators {
                moondance,
                container_chains,
            } = Self::assign_collators_always_keep_old(
                collators,
                &container_chain_ids,
                T::HostConfiguration::moondance_collators() as usize,
                T::HostConfiguration::collators_per_container() as usize,
                old_assigned,
            );

            // Write changes to storage
            // TODO: maybe it will be more efficient to store it everything under a single key?
            // TODO: but a limit of 0 also works?
            let _multi_removal_result =
                CollatorContainerChain::<T>::clear(old_num_collators as u32, None);

            // Write new collators to storage
            // TODO: this can be optimized:
            // Do not clear.
            // Iterate over old_collators:
            // If the new para_id has changed, write to storage.
            // If the collator no longer exists, remove from storage
            // Iterate over new_collators:
            // If the collator does not exist in old_collators, write to storage.
            for (para_id, collators) in container_chains {
                for collator in collators {
                    CollatorContainerChain::<T>::insert(collator, para_id);
                }
            }
            let moondance_para_id = T::MoondanceParaId::get();
            MoondanceCollators::<T>::put(moondance.clone());
            for collator in moondance {
                CollatorContainerChain::<T>::insert(collator, moondance_para_id);
            }
            // TODO: we may want to wait a few sessions before making the change, to give
            // new collators enough time to sync the respective container_chain
            // In that case, instead of writing to CollatorContainerChain we must write to PendingCollatorContainerChain or similar
            // And the optimization mentioned above does not make sense anymore because we will be writing a new map
        }

        /// Assign new collators to missing container_chains.
        /// Old collators always have preference to remain on the same chain.
        /// If there are no missing collators, nothing is changed.
        fn assign_collators_always_keep_old(
            collators: Vec<T::AccountId>,
            container_chain_ids: &[u32],
            num_moondance: usize,
            num_each_container_chain: usize,
            old_assigned: AssignedCollators<T::AccountId>,
        ) -> AssignedCollators<T::AccountId> {
            // TODO: the performance of this function is sad, could be improved by having sets of
            // old_collators and new_collators instead of doing array.contains() every time.
            let mut new_assigned = old_assigned;
            new_assigned.remove_collators_not_in_list(&collators);
            new_assigned.remove_container_chains_not_in_list(container_chain_ids);
            // Only need to do these two if the config params change
            new_assigned.remove_moondance_excess_collators(num_moondance);
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
            new_assigned.fill_moondance_collators(num_moondance, &mut new_collators);
            new_assigned.add_new_container_chains(container_chain_ids);
            new_assigned
                .fill_container_chain_collators(num_each_container_chain, &mut new_collators);

            new_assigned
        }

        // Returns the current assigned collators as read from storage, and the number of collators.
        fn read_assigned_collators() -> (AssignedCollators<T::AccountId>, usize) {
            let mut container_chains: BTreeMap<u32, Vec<T::AccountId>> = BTreeMap::new();
            let mut num_collators = 0;

            for (c, para_id) in CollatorContainerChain::<T>::iter() {
                container_chains.entry(para_id).or_default().push(c);
                num_collators += 1;
            }

            let moondance = container_chains
                .remove(&T::MoondanceParaId::get())
                .unwrap_or_default();

            (
                AssignedCollators {
                    moondance,
                    container_chains,
                },
                num_collators,
            )
        }

        pub fn initializer_on_new_session(_session_index: &T::SessionIndex) {
            Self::assign_collators();
        }
    }

    // These traits are to automatically call initializer_on_new_session when needed.
    // Can be removed after we implement the initializer pallet
    impl<T: Config> sp_runtime::BoundToRuntimeAppPublic for Pallet<T> {
        type Public = T::AuthorityId;
    }

    impl<T: Config> OneSessionHandler<T::AccountId> for Pallet<T> {
        type Key = T::AuthorityId;

        fn on_genesis_session<'a, I: 'a>(_validators: I)
        where
            I: Iterator<Item = (&'a T::AccountId, T::AuthorityId)>,
        {
        }

        fn on_new_session<'a, I: 'a>(_changed: bool, _validators: I, _queued_validators: I)
        where
            I: Iterator<Item = (&'a T::AccountId, T::AuthorityId)>,
        {
            Self::initializer_on_new_session(&T::CurrentSessionIndex::session_index());
        }

        fn on_disabled(_i: u32) {}
    }
}
