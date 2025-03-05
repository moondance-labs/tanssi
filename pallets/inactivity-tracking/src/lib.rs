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
use {
    sp_runtime::{traits::Get, BoundedVec},
    sp_staking::SessionIndex,
    tp_traits::{
        GetCurrentContainerChains, GetSessionIndex, LatestAuthorInfoFetcher,
        NodeInactivityTrackingHelper,
    },
};

pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
    use {
        super::*,
        core::marker::PhantomData,
        frame_support::{
            pallet_prelude::*, storage::types::StorageDoubleMap,
            StorageDoubleMap as StorageDoubleMapTrait,
        },
        frame_system::pallet_prelude::*,
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

        /// The maximum amount of collators that can stored for a session
        #[pallet::constant]
        type MaxCollatorsPerSession: Get<u32>;

        /// Helper that returns the current session index.
        type CurrentSessionIndex: GetSessionIndex<SessionIndex>;

        /// Helper that fetches the latest set of container chains valid for collation
        type RegisteredContainerChainsFetcher: GetCurrentContainerChains;

        /// Helper that fetches the latest block author info for a container chain
        type ContainerChainBlockAuthorInfoFetcher: LatestAuthorInfoFetcher<Self::CollatorId>;
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

    /// A list of inactive collators for a session. Repopulated at the start of every session
    #[pallet::storage]
    pub type CurrentSessionInactiveCollators<T: Config> =
        StorageValue<_, BoundedVec<T::CollatorId, T::MaxCollatorsPerSession>, ValueQuery>;

    ///
    #[pallet::storage]
    pub type LastUnprocessedSession<T: Config> = StorageValue<_, SessionIndex, ValueQuery>;

    #[pallet::event]
    pub enum Event<T: Config> {}

    #[pallet::error]
    pub enum Error<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
            let current_session = T::CurrentSessionIndex::session_index();
            let current_unprocessed_session = <LastUnprocessedSession<T>>::get();

            // Update inactive collator records only after a session has ended
            if current_unprocessed_session < current_session {
                // Update the inactive collator records for the previous session
                <CurrentSessionInactiveCollators<T>>::get()
                    .into_iter()
                    .for_each(|collator_id| {
                        <InactiveCollators<T>>::insert(
                            current_unprocessed_session,
                            collator_id,
                            (),
                        );
                    });

                <LastUnprocessedSession<T>>::put(current_session);
            } else {
                Self::update_collators_activity();
            }
            // Self::update_inactive_validator_info();
            Weight::zero()
        }
        fn on_finalize(_n: BlockNumberFor<T>) {
            Self::cleanup_inactive_collator_info();
            // Self::cleanup_inactive_validator_info();
        }
    }

    impl<T: Config> Pallet<T> {
        fn update_collators_activity() {
            // Collator can be marked as inactive only if:
            // 1. It has not produced a block in the previous session
            // 2. Chain has not advanced at all in the previous session

            T::RegisteredContainerChainsFetcher::current_container_chains()
                .into_iter()
                .for_each(|chain_id| {
                    let latest_container_chain_author =
                        T::ContainerChainBlockAuthorInfoFetcher::get_latest_author_info(chain_id)
                            .unwrap()
                            .author;

                    if <CurrentSessionInactiveCollators<T>>::get()
                        .contains(&latest_container_chain_author)
                    {
                        let _ = <CurrentSessionInactiveCollators<T>>::try_mutate(
                            |current_seesion_collators| -> DispatchResult {
                                current_seesion_collators
                                    .retain(|c| c != &latest_container_chain_author);
                                Ok(())
                            },
                        );
                    }
                });
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

impl<T: Config> NodeInactivityTrackingHelper<T::CollatorId> for Pallet<T> {
    fn is_node_inactive(node: &T::CollatorId) -> bool {
        let current_session = T::CurrentSessionIndex::session_index();

        let minimum_sessions_required = T::MaxInactiveSessions::get() + 1;
        if current_session < minimum_sessions_required {
            return false;
        }

        for session_index in current_session.saturating_sub(T::MaxInactiveSessions::get().into())
            ..current_session.saturating_sub(1u32.into())
        {
            if !<InactiveCollators<T>>::contains_key(session_index, node.clone()) {
                return false;
            }
        }
        true
    }
}
