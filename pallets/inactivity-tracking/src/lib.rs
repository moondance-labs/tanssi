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
        CurrentEligibleCollatorsHelper, GetCurrentContainerChains, GetSessionIndex,
        LatestAuthorInfoFetcher, NodeInactivityTrackingHelper,
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

        /// Helper that fetches a list of collators eligible for to produce blocks for the current session
        type CurrentCollatorsListFetcher: CurrentEligibleCollatorsHelper<Self::CollatorId>;
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
            Self::update_collators_activity();

            // Update inactive collator records only after a session has ended
            let current_session = T::CurrentSessionIndex::session_index();
            let current_unprocessed_session = <LastUnprocessedSession<T>>::get();
            if current_unprocessed_session < current_session {
                // Update the inactive collator records for the previous session
                // Collator can be marked as inactive only if:
                // 1. It has not produced a block in the previous session
                // 2. Chain has advanced in the previous session
                Self::process_ended_session(current_unprocessed_session);

                <LastUnprocessedSession<T>>::put(current_session);
            }
            Weight::zero()
        }
        fn on_finalize(_n: BlockNumberFor<T>) {
            Self::cleanup_inactive_collator_info();
        }
    }

    impl<T: Config> Pallet<T> {
        fn process_ended_session(session_id: SessionIndex) {
            <CurrentSessionInactiveCollators<T>>::get()
                .into_iter()
                .for_each(|collator_id| {
                    <InactiveCollators<T>>::insert(session_id, collator_id, ());
                });
            let eligible_collators = T::CurrentCollatorsListFetcher::get_eligible_collators();
            // TO DO: Remove from the list collators assigned to container chains that have not advanced
            <CurrentSessionInactiveCollators<T>>::put(BoundedVec::truncate_from(
                eligible_collators,
            ));
        }

        fn update_collators_activity() {
            T::RegisteredContainerChainsFetcher::current_container_chains()
                .into_iter()
                .for_each(|chain_id| {
                    let container_chain_block_info =
                        T::ContainerChainBlockAuthorInfoFetcher::get_latest_author_info(chain_id);

                    if container_chain_block_info.is_some() {
                        let container_chain_block_author =
                            container_chain_block_info.unwrap().author;

                        if <CurrentSessionInactiveCollators<T>>::get()
                            .contains(&container_chain_block_author)
                        {
                            let _ = <CurrentSessionInactiveCollators<T>>::try_mutate(
                                |current_seesion_collators| -> DispatchResult {
                                    current_seesion_collators
                                        .retain(|c| c != &container_chain_block_author);
                                    Ok(())
                                },
                            );
                        }
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
