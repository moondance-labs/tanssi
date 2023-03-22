#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::*;
use frame_support::traits::OneSessionHandler;
use scale_info::prelude::collections::HashMap;
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

pub trait GetParachains {
    // TODO: import ParaId type
    fn parachains() -> Vec<u32>;
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
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type SessionIndex: codec::FullCodec + TypeInfo + Copy + AtLeast32BitUnsigned;

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
        type Parachains: GetParachains;
        type CurrentSessionIndex: GetSessionIndex<Self::SessionIndex>;
    }

    #[pallet::error]
    pub enum Error<T> {}

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {}

    #[pallet::storage]
    #[pallet::getter(fn collator_parachain)]
    pub(crate) type CollatorParachain<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, u32>;

    #[pallet::storage]
    #[pallet::getter(fn moondance_collators)]
    pub(crate) type MoondanceCollators<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    #[derive(Debug, Clone, Default)]
    struct AssignedCollators<AccountId> {
        moondance: Vec<AccountId>,
        parachains: HashMap<u32, Vec<AccountId>>,
    }

    impl<AccountId> AssignedCollators<AccountId>
    where
        AccountId: PartialEq,
    {
        fn find_collator(&self, x: &AccountId) -> bool {
            self.moondance.iter().any(|a| a == x)
                || self
                    .parachains
                    .iter()
                    .any(|(_id, cs)| cs.iter().any(|a| a == x))
        }

        fn remove_parachains_not_in_list(&mut self, parachains: &[u32]) {
            self.parachains.retain(|id, _cs| parachains.contains(id));
        }

        fn remove_collators_not_in_list(&mut self, collators: &[AccountId]) {
            self.moondance.retain(|c| collators.contains(c));
            for (_id, cs) in self.parachains.iter_mut() {
                cs.retain(|c| collators.contains(c))
            }
        }

        fn remove_moondance_excess_collators(&mut self, num_moondance: usize) {
            self.moondance.truncate(num_moondance);
        }

        fn remove_parachain_excess_collators(&mut self, num_each_parachain: usize) {
            for (_id, cs) in self.parachains.iter_mut() {
                cs.truncate(num_each_parachain);
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

        fn fill_parachain_collators<I>(&mut self, num_each_parachain: usize, next_collator: &mut I)
        where
            I: Iterator<Item = AccountId>,
        {
            // TODO: the iteration order of a HashMap is not deterministic, so testing this is a bit annoying
            // Could be fixed using a BTreeMap instead, or sorting here before iterating
            for (_id, cs) in self.parachains.iter_mut() {
                while cs.len() < num_each_parachain {
                    if let Some(nc) = next_collator.next() {
                        cs.push(nc);
                    } else {
                        return;
                    }
                }
            }
        }

        fn add_new_parachains(&mut self, parachains: &[u32]) {
            for para_id in parachains {
                self.parachains.entry(*para_id).or_default();
            }
        }
    }

    impl<T: Config> Pallet<T> {
        /// Assign new collators
        pub fn assign_collators() {
            let collators = T::Collators::collators();
            let parachain_ids = T::Parachains::parachains();

            let (old_assigned, old_num_collators) = Self::read_assigned_collators();

            let AssignedCollators {
                moondance,
                parachains,
            } = Self::assign_collators_always_keep_old(
                collators,
                &parachain_ids,
                T::HostConfiguration::moondance_collators() as usize,
                T::HostConfiguration::collators_per_container() as usize,
                old_assigned,
            );

            // Write changes to storage
            // TODO: maybe it will be more efficient to store it everything under a single key?
            // TODO: but a limit of 0 also works?
            let _multi_removal_result =
                CollatorParachain::<T>::clear(old_num_collators as u32, None);

            // Write new collators to storage
            // TODO: this can be optimized:
            // Do not clear.
            // Iterate over old_collators:
            // If the new para_id has changed, write to storage.
            // If the collator no longer exists, remove from storage
            // Iterate over new_collators:
            // If the collator does not exist in old_collators, write to storage.
            for (para_id, collators) in parachains {
                for collator in collators {
                    CollatorParachain::<T>::insert(collator, para_id);
                }
            }
            let moondance_para_id = T::MoondanceParaId::get();
            MoondanceCollators::<T>::put(moondance.clone());
            for collator in moondance {
                CollatorParachain::<T>::insert(collator, moondance_para_id);
            }
            // TODO: we may want to wait a few sessions before making the change, to give
            // new collators enough time to sync the respective parachain
            // In that case, instead of writing to CollatorParachain we must write to PendingCollatorParachain or similar
            // And the optimization mentioned above does not make sense anymore because we will be writing a new map
        }

        /// Assign new collators to missing parachains.
        /// Old collators always have preference to remain on the same chain.
        /// If there are no missing collators, nothing is changed.
        fn assign_collators_always_keep_old(
            collators: Vec<T::AccountId>,
            parachain_ids: &[u32],
            num_moondance: usize,
            num_each_parachain: usize,
            old_assigned: AssignedCollators<T::AccountId>,
        ) -> AssignedCollators<T::AccountId> {
            // TODO: the performance of this function is sad, could be improved by having sets of
            // old_collators and new_collators instead of doing array.contains() every time.
            let mut new_assigned = old_assigned;
            new_assigned.remove_collators_not_in_list(&collators);
            new_assigned.remove_parachains_not_in_list(parachain_ids);
            // Only need to do these two if the config params change
            new_assigned.remove_moondance_excess_collators(num_moondance);
            new_assigned.remove_parachain_excess_collators(num_each_parachain);

            // Collators that are not present in old_assigned
            // TODO: unless we save all the old_collators somewhere, it is still possible for a
            // collator to change from parachain 1001 to None to 1002
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
            new_assigned.add_new_parachains(parachain_ids);
            new_assigned.fill_parachain_collators(num_each_parachain, &mut new_collators);

            new_assigned
        }

        // Returns the current assigned collators as read from storage, and the number of collators.
        fn read_assigned_collators() -> (AssignedCollators<T::AccountId>, usize) {
            let mut parachains: HashMap<u32, Vec<T::AccountId>> = HashMap::new();
            let mut num_collators = 0;

            for (c, para_id) in CollatorParachain::<T>::iter() {
                parachains.entry(para_id).or_default().push(c);
                num_collators += 1;
            }

            let moondance = parachains
                .remove(&T::MoondanceParaId::get())
                .unwrap_or_default();

            (
                AssignedCollators {
                    moondance,
                    parachains,
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
