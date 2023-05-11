//! # Nimbus Collator Assignment Pallet

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use {
    frame_support::pallet_prelude::*,
    sp_runtime::{
        traits::{AtLeast32BitUnsigned, One, Zero},
        Saturating,
    },
    sp_std::{collections::btree_map::BTreeMap, prelude::*, vec},
    tp_collator_assignment::AssignedCollators,
};

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
        type AuthorityId: parity_scale_codec::FullCodec + TypeInfo + Clone;
    }

    #[pallet::storage]
    #[pallet::getter(fn collator_container_chain)]
    pub type CollatorContainerChain<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::SessionIndex,
        AssignedCollators<T::AuthorityId>,
        OptionQuery,
    >;

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    impl<T: Config> Pallet<T> {
        /// Assign new collators
        /// collators should be queued collators
        pub fn assign_collators(
            current_session_index: &T::SessionIndex,
            id_map: &BTreeMap<T::AccountId, T::AuthorityId>,
            next_collator_assignment: &AssignedCollators<T::AccountId>,
        ) {
            let next_nimbus_assignment =
                next_collator_assignment.map(|account_id| id_map[account_id].clone());

            // Only applies to session index 0
            if current_session_index == &T::SessionIndex::zero() {
                CollatorContainerChain::<T>::insert(
                    current_session_index,
                    next_nimbus_assignment.clone(),
                );
                CollatorContainerChain::<T>::insert(
                    current_session_index.saturating_add(T::SessionIndex::one()),
                    next_nimbus_assignment.clone(),
                );

                return;
            }

            // Remove value at session - 1, insert new value at session + 1
            CollatorContainerChain::<T>::remove(
                current_session_index.saturating_sub(T::SessionIndex::one()),
            );
            CollatorContainerChain::<T>::insert(
                current_session_index.saturating_add(T::SessionIndex::one()),
                next_nimbus_assignment.clone(),
            );
        }

        pub fn initializer_on_new_session(
            current_session_index: &T::SessionIndex,
            id_map: &BTreeMap<T::AccountId, T::AuthorityId>,
            next_collator_assignment: &AssignedCollators<T::AccountId>,
        ) {
            Self::assign_collators(current_session_index, id_map, next_collator_assignment)
        }
    }
}
