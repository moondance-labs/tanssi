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
#![cfg_attr(not(feature = "std"), no_std)]
use {sp_runtime::traits::Get, sp_staking::SessionIndex};

#[frame_support::pallet]
pub mod pallet {
    use {
        super::*, core::marker::PhantomData, frame_support::pallet_prelude::*,
        frame_support::StorageDoubleMap, frame_system::pallet_prelude::*,
        tp_traits::GetSessionIndex,
    };

    #[pallet::pallet]
    pub struct Pallet<T>(PhantomData<T>);
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// A stable ID for a collator.
        type CollatorId: Member
            + Parameter
            + Ord
            + MaybeSerializeDeserialize
            + MaxEncodedLen
            + TryFrom<Self::AccountId>;

        /// The maximum number of sessions for which a collator can be inactive
        /// before being moved to the offline queue
        #[pallet::constant]
        type MaxInactiveSessions: Get<u32>;

        /// Helper that returns the current session index.
        type CurrentSessionIndex: GetSessionIndex<SessionIndex>;
    }

    /// A list of double map of inactive collators for a session
    #[pallet::storage]
    pub type InactiveCollators<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        SessionIndex,
        Twox64Concat,
        T::CollatorId,
        (),
        OptionQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {}

    #[pallet::error]
    pub enum Error<T> {}

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
            Self::update_inactive_collator_info();
            // Self::update_inactive_validator_info();
            Weight::zero()
        }
        fn on_finalize(_n: BlockNumberFor<T>) {
            Self::cleanup_inactive_collator_info();
            // Self::cleanup_inactive_validator_info();
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    impl<T: Config> Pallet<T> {
        fn update_inactive_collator_info() {
            let _current_session = T::CurrentSessionIndex::session_index();
            // TO DO: implement inactivity tracking
            if false {
                //<InactiveCollators<T>>::insert(current_session, collator_id, ());
            }
        }
        fn cleanup_inactive_collator_info() {
            let current_session = T::CurrentSessionIndex::session_index();
            let minimum_sessions_required = T::MaxInactiveSessions::get() + 1;

            if current_session < minimum_sessions_required
                || !<InactiveCollators<T>>::contains_prefix(current_session)
            {
                return;
            }

            let _ =
                <InactiveCollators<T>>::iter_prefix(current_session - minimum_sessions_required)
                    .drain()
                    .next();
        }
    }
}
