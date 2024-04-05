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

//! # Authority Mapping Pallet
//!
//! This pallet stores the AuthorityId -> AccountID mapping for two sessions
//! In particular it holds the mapping for the current and the past session
//! Both are necessary to verify block-authorship with respect to current
//! block proposals or blocks that have been proposed in the past-session

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use {
    frame_support::pallet_prelude::*,
    sp_runtime::{
        traits::{AtLeast32BitUnsigned, CheckedSub},
        RuntimeAppPublic,
    },
    sp_std::{collections::btree_map::BTreeMap, vec},
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

        // Sessions after which keys should be removed
        #[pallet::constant]
        type SessionRemovalBoundary: Get<Self::SessionIndex>;

        type AuthorityId: Member
            + Parameter
            + Ord
            + RuntimeAppPublic
            + MaybeSerializeDeserialize
            + MaxEncodedLen;
    }

    #[pallet::storage]
    #[pallet::getter(fn authority_id_mapping)]
    pub(super) type AuthorityIdMapping<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::SessionIndex,
        BTreeMap<T::AuthorityId, T::AccountId>,
        OptionQuery,
    >;

    impl<T: Config> Pallet<T> {
        pub fn initializer_on_new_session(
            session_index: &T::SessionIndex,
            authorities: &[(T::AccountId, T::AuthorityId)],
        ) {
            // Remove only if the checked sub does not saturate
            if let Some(session_index_to_remove) =
                session_index.checked_sub(&T::SessionRemovalBoundary::get())
            {
                AuthorityIdMapping::<T>::remove(session_index_to_remove)
            }

            let assignation: BTreeMap<T::AuthorityId, T::AccountId> =
                authorities.iter().cloned().map(|(a, b)| (b, a)).collect();
            AuthorityIdMapping::<T>::insert(session_index, assignation);
        }
    }
}
