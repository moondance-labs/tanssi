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

//! # Initializer Pallet
//!
//! This pallet is in charge of organizing what happens on session changes.
//! In particular this pallet has implemented the OneSessionHandler trait
//! which will be called upon a session change. There it will call the
//! SessionHandler config trait

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;
use {
    frame_support::{pallet_prelude::*, traits::OneSessionHandler},
    scale_info::TypeInfo,
    sp_runtime::{traits::AtLeast32BitUnsigned, RuntimeAppPublic},
    sp_std::prelude::*,
};

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    // The apply_new_session trait. We need to comply with this
    pub trait ApplyNewSession<T: Config> {
        fn apply_new_session(
            changed: bool,
            session_index: T::SessionIndex,
            all_validators: Vec<(T::AccountId, T::AuthorityId)>,
            queued: Vec<(T::AccountId, T::AuthorityId)>,
        );
    }

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type SessionIndex: parity_scale_codec::FullCodec + TypeInfo + Copy + AtLeast32BitUnsigned;

        /// The identifier type for an authority.
        type AuthorityId: Member
            + Parameter
            + RuntimeAppPublic
            + MaybeSerializeDeserialize
            + MaxEncodedLen;

        type SessionHandler: ApplyNewSession<Self>;
    }
}

impl<T: Config> Pallet<T> {
    /// Should be called when a new session occurs. If `queued` is `None`,
    /// the `validators` are considered queued.
    fn on_new_session<'a, I>(
        changed: bool,
        session_index: T::SessionIndex,
        validators: I,
        queued: Option<I>,
    ) where
        I: Iterator<Item = (&'a T::AccountId, T::AuthorityId)> + 'a,
    {
        let validators: Vec<_> = validators.map(|(k, v)| (k.clone(), v)).collect();
        let queued: Vec<_> = if let Some(queued) = queued {
            queued.map(|(k, v)| (k.clone(), v)).collect()
        } else {
            validators.clone()
        };

        T::SessionHandler::apply_new_session(changed, session_index, validators, queued);
    }

    /// Should be called when a new session occurs. Buffers the session notification to be applied
    /// at the end of the block. If `queued` is `None`, the `validators` are considered queued.
    fn on_genesis_session<'a, I>(validators: I)
    where
        I: Iterator<Item = (&'a T::AccountId, T::AuthorityId)> + 'a,
    {
        <Pallet<T>>::on_new_session(false, 0u32.into(), validators, None);
    }

    // Allow to trigger `on_new_session` in tests, this is needed as long as `pallet_session` is not
    // implemented in mock.
    #[cfg(any(test, feature = "runtime-benchmarks"))]
    pub(crate) fn test_trigger_on_new_session<'a, I>(
        changed: bool,
        session_index: T::SessionIndex,
        validators: I,
        queued: Option<I>,
    ) where
        I: Iterator<Item = (&'a T::AccountId, T::AuthorityId)> + 'a,
    {
        Self::on_new_session(changed, session_index, validators, queued)
    }
}

impl<T: Config> sp_runtime::BoundToRuntimeAppPublic for Pallet<T> {
    type Public = T::AuthorityId;
}

impl<T: pallet_session::Config + Config> OneSessionHandler<T::AccountId> for Pallet<T> {
    type Key = T::AuthorityId;

    fn on_genesis_session<'a, I>(validators: I)
    where
        I: Iterator<Item = (&'a T::AccountId, Self::Key)> + 'a,
    {
        <Pallet<T>>::on_genesis_session(validators);
    }

    fn on_new_session<'a, I>(changed: bool, validators: I, queued: I)
    where
        I: Iterator<Item = (&'a T::AccountId, Self::Key)> + 'a,
    {
        let session_index = <pallet_session::Pallet<T>>::current_index();
        <Pallet<T>>::on_new_session(changed, session_index.into(), validators, Some(queued));
    }

    fn on_disabled(_i: u32) {}
}
