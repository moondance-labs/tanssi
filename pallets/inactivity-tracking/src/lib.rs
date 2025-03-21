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
    frame_support::{dispatch::DispatchResult, pallet_prelude::Weight, traits::OneSessionHandler},
    sp_runtime::{traits::Get, BoundedVec},
    sp_staking::SessionIndex,
    tp_traits::{
        AuthorNotingHook, AuthorNotingInfo, GetSessionIndex, MaybeSelfChainBlockAuthor,
        NodeActivityTrackingHelper,
    },
};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;

#[cfg(feature = "runtime-benchmarks")]
use tp_traits::{BlockNumber, ParaId};

pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
    use sp_runtime::RuntimeAppPublic;
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

        /// A stable identifier for a collator.
        type CollatorId: Member
            + Parameter
            + Ord
            + MaybeSerializeDeserialize
            + MaxEncodedLen
            + TryFrom<Self::AccountId>;

        /// The identifier type for an authority.
        type AuthorityId: Member
            + Parameter
            + RuntimeAppPublic
            + MaybeSerializeDeserialize
            + MaxEncodedLen;

        /// The maximum number of sessions for which a collator can be inactive
        /// before being moved to the offline queue
        #[pallet::constant]
        type MaxInactiveSessions: Get<u32>;

        /// The maximum amount of collators that can stored for a session
        #[pallet::constant]
        type MaxCollatorsPerSession: Get<u32>;

        /// Helper that returns the current session index.
        type CurrentSessionIndex: GetSessionIndex<SessionIndex>;

        /// Helper that returns the block author for the orchestrator chain (if it exists)
        type GetSelfChainBlockAuthor: MaybeSelfChainBlockAuthor<Self::CollatorId>;
    }

    /// Switch to enable/disable inactivity tracking
    #[pallet::storage]
    pub type EnableInactivityTracking<T: Config> = StorageValue<_, bool, ValueQuery>;

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

    /// The last session index for which the inactive collators have not been processed
    #[pallet::storage]
    pub type LastUnprocessedSession<T: Config> = StorageValue<_, SessionIndex, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        InactivityTrackingEnabled { is_enabled: bool },
    }

    #[pallet::error]
    pub enum Error<T> {
        MaxCollatorsPerSessionReached,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::DbWeight::get().reads_writes(0, 1))]
        pub fn set_inactivity_tracking_status(
            origin: OriginFor<T>,
            is_enabled: bool,
        ) -> DispatchResult {
            ensure_root(origin)?;
            <EnableInactivityTracking<T>>::put(is_enabled);
            Self::deposit_event(Event::<T>::InactivityTrackingEnabled { is_enabled });
            Ok(())
        }
    }

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
    }

    impl<T: Config> Pallet<T> {
        fn process_ended_session(
            unprocessed_session_id: SessionIndex,
            active_session_id: SessionIndex,
        ) -> Weight {
            let mut total_weight = T::DbWeight::get().reads_writes(1, 3);
            // Since this function will be executed in the beginning of a session
            // we can populate the active collators from the session that just ended (unprocessed_session_id)
            // before resetting the collators storage for the current session (active_session_id)
            // without affecting the current session's active collators records
            ActiveCollators::<T>::insert(
                unprocessed_session_id,
                <ActiveCollatorsForCurrentSession<T>>::get(),
            );

            <ActiveCollatorsForCurrentSession<T>>::put(BoundedVec::new());

            // Cleanup active collator info for sessions that are older than the maximum allowed
            let minimum_sessions_required = T::MaxInactiveSessions::get() + 1;
            if active_session_id >= minimum_sessions_required {
                total_weight += T::DbWeight::get().writes(1);
                let _ = <crate::pallet::ActiveCollators<T>>::remove(
                    active_session_id - minimum_sessions_required,
                );
            }
            <LastUnprocessedSession<T>>::put(active_session_id);
            total_weight
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
    }
}

impl<T: Config> NodeActivityTrackingHelper<T::CollatorId> for Pallet<T> {
    fn is_node_inactive(node: &T::CollatorId) -> bool {
        // If EnableInactivityTracking is false all nodes are considered active
        // and we don't need to check the inactivity records
        if !<EnableInactivityTracking<T>>::get() {
            return false;
        }

        let current_session_index = T::CurrentSessionIndex::session_index();

        let minimum_sessions_required = T::MaxInactiveSessions::get();
        if current_session_index < minimum_sessions_required {
            return false;
        }

        let start_session_index = current_session_index.saturating_sub(minimum_sessions_required);
        for session_index in start_session_index..current_session_index {
            if <ActiveCollators<T>>::get(session_index).contains(node) {
                return false;
            }
        }
        true
    }
}

impl<T: Config> AuthorNotingHook<T::CollatorId> for Pallet<T> {
    fn on_container_authors_noted(info: &[AuthorNotingInfo<T::CollatorId>]) -> Weight {
        let mut total_weight = T::DbWeight::get().reads_writes(1, 0);
        for author_info in info {
            total_weight += Self::on_author_noted(author_info.author.clone());
        }
        total_weight
    }
    #[cfg(feature = "runtime-benchmarks")]
    fn prepare_worst_case_for_bench(_a: &T::CollatorId, _b: BlockNumber, _para_id: ParaId) {}
}
impl<T: Config> sp_runtime::BoundToRuntimeAppPublic for Pallet<T> {
    type Public = T::AuthorityId;
}
impl<T: pallet_session::Config + Config> OneSessionHandler<T::AccountId> for Pallet<T> {
    type Key = T::AuthorityId;
    fn on_genesis_session<'a, I>(_validators: I)
    where
        I: Iterator<Item = (&'a T::AccountId, Self::Key)> + 'a,
    {
    }
    fn on_new_session<'a, I>(_changed: bool, _validators: I, _queued: I)
    where
        I: Iterator<Item = (&'a T::AccountId, Self::Key)> + 'a,
    {
    }
    fn on_before_session_ending() {
        // TODO: Move relevant logic from `on_initialize` and `on_finalize` to here
    }
    fn on_disabled(_validator_index: u32) {}
}
