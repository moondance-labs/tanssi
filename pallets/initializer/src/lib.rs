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
//! which will be called upon a session change. This pallet will then store
//! the bufferedSessionChanges (collators, new session index, etc) in the
//! BufferedSessionChanges storage item. This storage item gets read on_finalize
//! and calls the  SessionHandler config trait

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;
use {
    frame_support::{pallet_prelude::*, traits::OneSessionHandler},
    frame_system::pallet_prelude::*,
    parity_scale_codec::{Decode, Encode},
    scale_info::TypeInfo,
    sp_runtime::{
        traits::{AtLeast32BitUnsigned, Zero},
        RuntimeAppPublic,
    },
    sp_std::prelude::*,
};

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[derive(Encode, Decode, TypeInfo)]
    #[scale_info(skip_type_params(T))]
    pub struct BufferedSessionChange<T: Config> {
        pub changed: bool,
        pub validators: Vec<(T::AccountId, T::AuthorityId)>,
        pub queued: Vec<(T::AccountId, T::AuthorityId)>,
        pub session_index: T::SessionIndex,
    }

    // The apply_new_sseion trait. We need to comply with this
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

    /// Buffered session changes along with the block number at which they should be applied.
    ///
    /// Typically this will be empty or one element long. Apart from that this item never hits
    /// the storage.
    ///
    /// However this is a `Vec` regardless to handle various edge cases that may occur at runtime
    /// upgrade boundaries or if governance intervenes.
    #[pallet::storage]
    pub(super) type BufferedSessionChanges<T: Config> =
        StorageValue<_, BufferedSessionChange<T>, OptionQuery>;

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_finalize(_now: BlockNumberFor<T>) {
            // Apply buffered session changes as the last thing. This way the runtime APIs and the
            // next block will observe the next session.
            //
            // Note that we only apply the last session as all others lasted less than a block (weirdly).
            if let Some(BufferedSessionChange {
                changed,
                session_index,
                validators,
                queued,
            }) = BufferedSessionChanges::<T>::take()
            {
                // Changes to be applied on new session
                T::SessionHandler::apply_new_session(changed, session_index, validators, queued);
            }
        }
    }
}

impl<T: Config> Pallet<T> {
    /// Should be called when a new session occurs. Buffers the session notification to be applied
    /// at the end of the block. If `queued` is `None`, the `validators` are considered queued.
    fn on_new_session<'a, I: 'a>(
        changed: bool,
        session_index: T::SessionIndex,
        validators: I,
        queued: Option<I>,
    ) where
        I: Iterator<Item = (&'a T::AccountId, T::AuthorityId)>,
    {
        let validators: Vec<_> = validators.map(|(k, v)| (k.clone(), v)).collect();
        let queued: Vec<_> = if let Some(queued) = queued {
            queued.map(|(k, v)| (k.clone(), v)).collect()
        } else {
            validators.clone()
        };

        if session_index == T::SessionIndex::zero() {
            // Genesis session should be immediately enacted.
            T::SessionHandler::apply_new_session(false, 0u32.into(), validators, queued);
        } else {
            BufferedSessionChanges::<T>::mutate(|v| {
                *v = Some(BufferedSessionChange {
                    changed,
                    validators,
                    queued,
                    session_index,
                })
            });
        }
    }

    /// Should be called when a new session occurs. Buffers the session notification to be applied
    /// at the end of the block. If `queued` is `None`, the `validators` are considered queued.
    fn on_genesis_session<'a, I: 'a>(validators: I)
    where
        I: Iterator<Item = (&'a T::AccountId, T::AuthorityId)>,
    {
        <Pallet<T>>::on_new_session(false, 0u32.into(), validators, None);
    }

    // Allow to trigger `on_new_session` in tests, this is needed as long as `pallet_session` is not
    // implemented in mock.
    #[cfg(any(test, feature = "runtime-benchmarks"))]
    pub(crate) fn test_trigger_on_new_session<'a, I: 'a>(
        changed: bool,
        session_index: T::SessionIndex,
        validators: I,
        queued: Option<I>,
    ) where
        I: Iterator<Item = (&'a T::AccountId, T::AuthorityId)>,
    {
        Self::on_new_session(changed, session_index, validators, queued)
    }
}

impl<T: Config> sp_runtime::BoundToRuntimeAppPublic for Pallet<T> {
    type Public = T::AuthorityId;
}

impl<T: pallet_session::Config + Config> OneSessionHandler<T::AccountId> for Pallet<T> {
    type Key = T::AuthorityId;

    fn on_genesis_session<'a, I: 'a>(validators: I)
    where
        I: Iterator<Item = (&'a T::AccountId, Self::Key)>,
    {
        <Pallet<T>>::on_genesis_session(validators);
    }

    fn on_new_session<'a, I: 'a>(changed: bool, validators: I, queued: I)
    where
        I: Iterator<Item = (&'a T::AccountId, Self::Key)>,
    {
        let session_index = <pallet_session::Pallet<T>>::current_index();
        <Pallet<T>>::on_new_session(changed, session_index.into(), validators, Some(queued));
    }

    fn on_disabled(_i: u32) {}
}
