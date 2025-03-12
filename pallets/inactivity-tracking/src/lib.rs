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
    frame_support::{dispatch::DispatchResult, pallet_prelude::Weight},
    sp_runtime::{traits::Get, BoundedVec},
    sp_staking::SessionIndex,
    tp_traits::{
        AuthorNotingHook, AuthorNotingInfo, ForSession, GetContainerChainsWithCollators,
        GetRandomnessForNextBlock, GetSessionIndex, MaybeSelfChainBlockAuthor,
        NodeActivityTrackingHelper, ParaId,
    },
};

#[cfg(feature = "runtime-benchmarks")]
use tp_traits::BlockNumber;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use {
        super::*,
        core::marker::PhantomData,
        frame_support::{pallet_prelude::*, storage::types::StorageMap},
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

        /// The maximum amount of collators that can be stored for a session
        #[pallet::constant]
        type MaxCollatorsPerSession: Get<u32>;

        /// The maximum amount of container chains that can be stored
        #[pallet::constant]
        type MaxContainerChains: Get<u32>;

        /// Helper that returns the current session index.
        type CurrentSessionIndex: GetSessionIndex<SessionIndex>;

        /// Helper that returns the block author for the orchestrator chain (if it exists)
        type GetSelfChainBlockAuthor: MaybeSelfChainBlockAuthor<Self::CollatorId>;

        /// Helper that fetches the latest set of container chains and their collators
        type ContainerChainsFetcher: GetContainerChainsWithCollators<Self::CollatorId>;

        /// Helper that allows to check if a new session will start in the next block
        type SessionEndChecker: GetRandomnessForNextBlock<BlockNumberFor<Self>>;
    }

    /// A list of double map of inactive collators for a session
    #[pallet::storage]
    pub type ActiveCollators<T: Config> = StorageMap<
        _,
        Twox64Concat,
        SessionIndex,
        BoundedVec<T::CollatorId, T::MaxCollatorsPerSession>,
        ValueQuery,
    >;

    /// A list of inactive collators for a session. Repopulated at the start of every session
    #[pallet::storage]
    pub type ActiveCollatorsForCurrentSession<T: Config> =
        StorageValue<_, BoundedVec<T::CollatorId, T::MaxCollatorsPerSession>, ValueQuery>;

    /// A list of inactive container chains for a session. Repopulated at the start of every session
    #[pallet::storage]
    pub type ActiveContainerChainsForCurrentSession<T: Config> =
        StorageValue<_, BoundedVec<ParaId, T::MaxInactiveSessions>, ValueQuery>;

    /// The last session index for which the inactive collators have not been processed
    #[pallet::storage]
    pub type LastUnprocessedSession<T: Config> = StorageValue<_, SessionIndex, ValueQuery>;

    #[pallet::event]
    pub enum Event<T: Config> {}

    #[pallet::error]
    pub enum Error<T> {
        MaxCollatorsPerSessionReached,
        MaxContainerChainsReached,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
            let mut total_weight = T::DbWeight::get().reads_writes(1, 0);

            // Process the orchestrator chain block author (if it exists)
            if let Some(orchestrator_chain_author) = T::GetSelfChainBlockAuthor::get_block_author()
            {
                total_weight += Self::on_author_noted(orchestrator_chain_author);
            }

            // Update inactive collator records only after a session has ended
            let current_session = T::CurrentSessionIndex::session_index();
            let current_unprocessed_session = <LastUnprocessedSession<T>>::get();
            if current_unprocessed_session < current_session {
                total_weight +=
                    Self::process_ended_session(current_unprocessed_session, current_session);
            }
            total_weight
        }

        fn on_finalize(n: BlockNumberFor<T>) {
            if T::SessionEndChecker::should_end_session(n) {
                Self::process_inactive_chains_for_session();
            }
        }
    }

    impl<T: Config> Pallet<T> {
        fn process_ended_session(
            unprocessed_session_id: SessionIndex,
            current_session_id: SessionIndex,
        ) -> Weight {
            let mut total_weight = T::DbWeight::get().reads_writes(0, 4);
            ActiveCollators::<T>::insert(
                unprocessed_session_id,
                <ActiveCollatorsForCurrentSession<T>>::get(),
            );

            <ActiveCollatorsForCurrentSession<T>>::put(BoundedVec::new());
            <ActiveContainerChainsForCurrentSession<T>>::put(BoundedVec::new());

            // Cleanup active collator info for sessions that are older than the maximum allowed
            let minimum_sessions_required = T::MaxInactiveSessions::get() + 1;
            if current_session_id >= minimum_sessions_required {
                total_weight += T::DbWeight::get().writes(1);
                let _ = <crate::pallet::ActiveCollators<T>>::remove(
                    current_session_id - minimum_sessions_required,
                );
            }
            <LastUnprocessedSession<T>>::put(current_session_id);
            total_weight
        }
        fn process_inactive_chains_for_session() {
            let active_chains = <ActiveContainerChainsForCurrentSession<T>>::get();
            let _ = <ActiveCollatorsForCurrentSession<T>>::try_mutate(
                |active_collators| -> DispatchResult {
                    T::ContainerChainsFetcher::container_chains_with_collators(ForSession::Current)
                        .iter()
                        .for_each(|(para_id, collator_ids)| {
                            if !active_chains.contains(para_id) {
                                collator_ids.iter().for_each(|collator_id| -> () {
                                    if !active_collators.contains(collator_id) {
                                        let _ = active_collators
                                            .try_push(collator_id.clone())
                                            .map_err(|_| Error::<T>::MaxCollatorsPerSessionReached);
                                    }
                                });
                            }
                        });
                    Ok(())
                },
            );
        }
        pub fn on_author_noted(author: T::CollatorId) -> Weight {
            let mut total_weight = T::DbWeight::get().reads_writes(1, 0);
            let _ = <ActiveCollatorsForCurrentSession<T>>::try_mutate(
                |active_collators| -> DispatchResult {
                    if !active_collators.contains(&author) {
                        total_weight += T::DbWeight::get().writes(1);
                        active_collators
                            .try_push(author)
                            .map_err(|_| Error::<T>::MaxCollatorsPerSessionReached)?;
                    }
                    Ok(())
                },
            );
            total_weight
        }

        pub fn on_chain_noted(chain_id: ParaId) -> Weight {
            let mut total_weight = T::DbWeight::get().reads_writes(1, 0);
            let _ = <ActiveContainerChainsForCurrentSession<T>>::try_mutate(
                |active_chains| -> DispatchResult {
                    if active_chains.contains(&chain_id) {
                        total_weight += T::DbWeight::get().writes(1);
                        active_chains
                            .try_push(chain_id)
                            .map_err(|_| Error::<T>::MaxContainerChainsReached)?;
                    }
                    Ok(())
                },
            );
            total_weight
        }
    }
}

impl<T: Config> NodeActivityTrackingHelper<T::CollatorId> for Pallet<T> {
    fn is_node_inactive(node: &T::CollatorId) -> bool {
        let current_session = T::CurrentSessionIndex::session_index();

        let minimum_sessions_required = T::MaxInactiveSessions::get() + 1;
        if current_session < minimum_sessions_required {
            return false;
        }

        for session_index in current_session.saturating_sub(T::MaxInactiveSessions::get().into())
            ..current_session.saturating_sub(1u32.into())
        {
            if <ActiveCollators<T>>::get(session_index).contains(node) {
                return false;
            }
        }
        true
    }
}

impl<T: Config> AuthorNotingHook<T::CollatorId> for Pallet<T> {
    fn on_container_authors_noted(info: &[AuthorNotingInfo<T::CollatorId>]) -> Weight {
        let mut total_weight = T::DbWeight::get().reads_writes(0, 0);
        for author_info in info {
            total_weight += Self::on_author_noted(author_info.author.clone());
            total_weight += Self::on_chain_noted(author_info.para_id.clone());
        }
        total_weight
    }
    #[cfg(feature = "runtime-benchmarks")]
    fn prepare_worst_case_for_bench(_a: &T::CollatorId, _b: BlockNumber, para_id: ParaId) {}
}
