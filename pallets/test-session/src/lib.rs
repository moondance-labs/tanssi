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
        traits::{AtLeast32BitUnsigned, One, Saturating, Zero},
        RuntimeAppPublic,
    },
    sp_std::prelude::*,
};

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
    //#[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        SessionEndChanged {
            first_block_of_next_session: T::BlockNumber,
        },
    }

    #[pallet::storage]
    pub type CurrentSessionEnd<T: Config> = StorageValue<_, T::BlockNumber, OptionQuery>;

    #[pallet::storage]
    pub type CurrentSessionStart<T: Config> = StorageValue<_, T::BlockNumber, ValueQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn set_current_session_end(
            origin: OriginFor<T>,
            first_block_of_next_session: T::BlockNumber,
        ) -> DispatchResult {
            ensure_root(origin)?;
            CurrentSessionEnd::<T>::put(first_block_of_next_session);
            Self::deposit_event(Event::SessionEndChanged {
                first_block_of_next_session,
            });
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn initializer_on_new_session(block_number: T::BlockNumber) {
            CurrentSessionEnd::<T>::kill();
            CurrentSessionStart::<T>::put(block_number);
        }
    }
}

pub struct OverridablePeriodicSessions<T, Period>(PhantomData<(T, Period)>);

mod session_impls {
    use core::ops::{Rem, Sub};
    use frame_support::{traits::EstimateNextSessionRotation, traits::Get, weights::Weight};
    use pallet_session::ShouldEndSession;
    use sp_runtime::{
        traits::{AtLeast32BitUnsigned, One, Zero},
        Permill, Saturating,
    };

    use crate::{Config, CurrentSessionEnd, CurrentSessionStart, OverridablePeriodicSessions};

    impl<T: Config, Period: Get<T::BlockNumber>> ShouldEndSession<T::BlockNumber>
        for OverridablePeriodicSessions<T, Period>
    {
        fn should_end_session(now: T::BlockNumber) -> bool {
            let offset = CurrentSessionStart::<T>::get();

            match CurrentSessionEnd::<T>::get() {
                Some(end) => now >= end,
                None => now >= offset && ((now - offset) % Period::get()).is_zero(),
            }
        }
    }

    impl<T: Config, Period: Get<T::BlockNumber>> EstimateNextSessionRotation<T::BlockNumber>
        for OverridablePeriodicSessions<T, Period>
    {
        fn average_session_length() -> T::BlockNumber {
            Period::get()
        }

        fn estimate_current_session_progress(now: T::BlockNumber) -> (Option<Permill>, Weight) {
            let offset = CurrentSessionStart::<T>::get();
            let period = CurrentSessionEnd::<T>::get()
                .map(|end| end.saturating_sub(offset))
                .map(|period| if period.is_zero() { One::one() } else { period })
                .unwrap_or(Period::get());

            // NOTE: we add one since we assume that the current block has already elapsed,
            // i.e. when evaluating the last block in the session the progress should be 100%
            // (0% is never returned).
            let progress = if now >= offset {
                let current = (now - offset) % period.clone() + One::one();
                Some(Permill::from_rational(current, period))
            } else {
                Some(Permill::from_rational(now + One::one(), offset))
            };

            // Weight note: `estimate_current_session_progress` has no storage reads and trivial
            // computational overhead. There should be no risk to the chain having this weight value be
            // zero for now. However, this value of zero was not properly calculated, and so it would be
            // reasonable to come back here and properly calculate the weight of this function.
            (progress, Zero::zero())
        }

        fn estimate_next_session_rotation(now: T::BlockNumber) -> (Option<T::BlockNumber>, Weight) {
            let offset = CurrentSessionStart::<T>::get();
            let period = CurrentSessionEnd::<T>::get()
                .map(|end| end.saturating_sub(offset))
                .map(|period| if period.is_zero() { One::one() } else { period })
                .unwrap_or(Period::get());

            let next_session = if now > offset {
                let block_after_last_session = (now.clone() - offset) % period.clone();
                if block_after_last_session > Zero::zero() {
                    now.saturating_add(period.saturating_sub(block_after_last_session))
                } else {
                    // this branch happens when the session is already rotated or will rotate in this
                    // block (depending on being called before or after `session::on_initialize`). Here,
                    // we assume the latter, namely that this is called after `session::on_initialize`,
                    // and thus we add period to it as well.
                    now + period
                }
            } else {
                offset
            };

            // Weight note: `estimate_next_session_rotation` has no storage reads and trivial
            // computational overhead. There should be no risk to the chain having this weight value be
            // zero for now. However, this value of zero was not properly calculated, and so it would be
            // reasonable to come back here and properly calculate the weight of this function.
            (Some(next_session), Zero::zero())
        }
    }
}
