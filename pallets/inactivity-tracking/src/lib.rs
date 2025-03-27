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
    parity_scale_codec::{Decode, Encode},
    scale_info::TypeInfo,
    serde::{Deserialize, Serialize},
    sp_core::{MaxEncodedLen, RuntimeDebug},
    sp_runtime::{traits::Get, BoundedBTreeSet},
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

/// The status of the activity tracking
#[derive(
    Clone,
    PartialEq,
    Eq,
    Encode,
    Decode,
    TypeInfo,
    Serialize,
    Deserialize,
    RuntimeDebug,
    MaxEncodedLen,
)]
pub enum ActivityTrackingStatus {
    Enabled { end: SessionIndex },
    Disabled { end: SessionIndex },
}
impl Default for ActivityTrackingStatus {
    fn default() -> Self {
        ActivityTrackingStatus::Enabled { end: 0 }
    }
}

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use {
        super::*,
        crate::weights::WeightInfo,
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

        /// The weight information of this pallet.
        type WeightInfo: WeightInfo;
    }

    /// Switch to enable/disable inactivity tracking
    #[pallet::storage]
    pub type CurrentActivityTrackingStatus<T: Config> =
        StorageValue<_, ActivityTrackingStatus, ValueQuery>;

    /// A list of double map of inactive collators for a session
    #[pallet::storage]
    pub type ActiveCollators<T: Config> = StorageMap<
        _,
        Twox64Concat,
        SessionIndex,
        BoundedBTreeSet<T::CollatorId, T::MaxCollatorsPerSession>,
        ValueQuery,
    >;

    /// A list of inactive collators for a session. Repopulated at the start of every session
    #[pallet::storage]
    pub type ActiveCollatorsForCurrentSession<T: Config> =
        StorageValue<_, BoundedBTreeSet<T::CollatorId, T::MaxCollatorsPerSession>, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ActivityTrackingStatusSet { status: ActivityTrackingStatus },
    }

    #[pallet::error]
    pub enum Error<T> {
        MaxCollatorsPerSessionReached,
        ActivityStatusUpdateSuspended,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::set_inactivity_tracking_status())]
        pub fn set_inactivity_tracking_status(
            origin: OriginFor<T>,
            is_enabled: bool,
        ) -> DispatchResult {
            ensure_root(origin)?;
            let current_status_end_session_index = match <CurrentActivityTrackingStatus<T>>::get() {
                ActivityTrackingStatus::Enabled { end } => end,
                ActivityTrackingStatus::Disabled { end } => end,
            };
            let current_session_index = T::CurrentSessionIndex::session_index();
            ensure!(
                current_session_index > current_status_end_session_index,
                Error::<T>::ActivityStatusUpdateSuspended
            );
            let new_status_end_session_index =
                current_session_index + T::MaxInactiveSessions::get();
            let new_status = if is_enabled {
                ActivityTrackingStatus::Enabled {
                    end: new_status_end_session_index,
                }
            } else {
                ActivityTrackingStatus::Disabled {
                    end: new_status_end_session_index,
                }
            };
            <CurrentActivityTrackingStatus<T>>::put(new_status.clone());
            Self::deposit_event(Event::<T>::ActivityTrackingStatusSet { status: new_status });
            Ok(())
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
            let mut total_weight = T::DbWeight::get().reads_writes(0, 0);

            // Process the orchestrator chain block author (if it exists)
            if let Some(orchestrator_chain_author) = T::GetSelfChainBlockAuthor::get_block_author()
            {
                total_weight += Self::on_author_noted(orchestrator_chain_author);
            }

            total_weight
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn process_ended_session() {
            let current_session_index = T::CurrentSessionIndex::session_index();
            // Since this function will be executed in the beginning of a session
            // we can populate the active collators from the session that just ended
            // before resetting the collators storage for the current session
            // without affecting the current session's active collators records
            ActiveCollators::<T>::insert(
                current_session_index.saturating_sub(1),
                <ActiveCollatorsForCurrentSession<T>>::get(),
            );

            <ActiveCollatorsForCurrentSession<T>>::put(BoundedBTreeSet::new());

            // Cleanup active collator info for sessions that are older than the maximum allowed
            if current_session_index > T::MaxInactiveSessions::get() {
                <crate::pallet::ActiveCollators<T>>::remove(
                    current_session_index - T::MaxInactiveSessions::get() - 1,
                );
            }
        }
        pub fn on_author_noted(author: T::CollatorId) -> Weight {
            let mut total_weight = T::DbWeight::get().reads_writes(1, 0);
            let _ = <ActiveCollatorsForCurrentSession<T>>::try_mutate(
                |active_collators| -> DispatchResult {
                    if active_collators
                        .try_insert(author)
                        .map_err(|_| Error::<T>::MaxCollatorsPerSessionReached)?
                    {
                        total_weight += T::DbWeight::get().writes(1);
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
        // If inactivity tracking is not enabled all nodes are considered active.
        // We don't need to check the inactivity records and can return false
        if let ActivityTrackingStatus::Disabled { .. } = <CurrentActivityTrackingStatus<T>>::get() {
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
