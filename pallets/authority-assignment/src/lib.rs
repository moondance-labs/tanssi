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

//! # Nimbus Collator Assignment Pallet

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use {
    dp_collator_assignment::AssignedCollators,
    frame_support::pallet_prelude::*,
    sp_runtime::{
        traits::{AtLeast32BitUnsigned, One, Zero},
        Saturating,
    },
    sp_std::{collections::btree_map::BTreeMap, prelude::*, vec},
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
        Twox64Concat,
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
            queued_id_to_nimbus_map: &BTreeMap<T::AccountId, T::AuthorityId>,
            next_collator_assignment: &AssignedCollators<T::AccountId>,
        ) {
            let next_nimbus_assignment = next_collator_assignment
                .map(|account_id| queued_id_to_nimbus_map[account_id].clone());

            // Only applies to session index 0
            if current_session_index == &T::SessionIndex::zero() {
                CollatorContainerChain::<T>::insert(
                    current_session_index,
                    next_nimbus_assignment.clone(),
                );
                CollatorContainerChain::<T>::insert(
                    current_session_index.saturating_add(T::SessionIndex::one()),
                    next_nimbus_assignment,
                );

                return;
            }

            // Remove value at session - 1, insert new value at session + 1
            CollatorContainerChain::<T>::remove(
                current_session_index.saturating_sub(T::SessionIndex::one()),
            );
            CollatorContainerChain::<T>::insert(
                current_session_index.saturating_add(T::SessionIndex::one()),
                next_nimbus_assignment,
            );
        }

        pub fn initializer_on_new_session(
            current_session_index: &T::SessionIndex,
            queued_id_to_nimbus_map: &BTreeMap<T::AccountId, T::AuthorityId>,
            next_collator_assignment: &AssignedCollators<T::AccountId>,
        ) {
            Self::assign_collators(
                current_session_index,
                queued_id_to_nimbus_map,
                next_collator_assignment,
            )
        }
    }
}
