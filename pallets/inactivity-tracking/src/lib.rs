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
//! # Inactivity Tracking Pallet
//!
//! This pallet tracks and stores the activity of container chain and orchestrator chain collators
//! for configurable number of sessions. It is used to determine if a collator is inactive
//! for that period of time.
//!
//! The tracking functionality can be enabled or disabled with root privileges.
//! By default, the tracking is enabled.
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
        AuthorNotingHook, AuthorNotingInfo, GetContainerChainsWithCollators, GetSessionIndex,
        MaybeSelfChainBlockAuthor, NodeActivityTrackingHelper,
    },
};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use weights::WeightInfo;

#[cfg(feature = "runtime-benchmarks")]
use tp_traits::{BlockNumber, ParaId};

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use tp_traits::ForSession;
    use {
        super::*,
        crate::weights::WeightInfo,
        core::marker::PhantomData,
        frame_support::{pallet_prelude::*, storage::types::StorageMap},
        frame_system::pallet_prelude::*,
        sp_std::collections::btree_set::BTreeSet,
    };

    /// The status of collator activity tracking
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
        Enabled {
            /// The session in which we will start recording the collator activity after enabling it
            start: SessionIndex,
            /// The session after which the activity tracking can be disabled
            end: SessionIndex,
        },
        Disabled {
            /// The session after which the activity tracking can be enabled
            end: SessionIndex,
        },
    }
    impl Default for ActivityTrackingStatus {
        fn default() -> Self {
            ActivityTrackingStatus::Enabled { start: 0, end: 0 }
        }
    }

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

        /// Helper that fetches a list of collators eligible to produce blocks for the current session
        type CurrentCollatorsFetcher: GetContainerChainsWithCollators<Self::CollatorId>;

        /// Helper that returns the block author for the orchestrator chain (if it exists)
        type GetSelfChainBlockAuthor: MaybeSelfChainBlockAuthor<Self::CollatorId>;

        /// The weight information of this pallet.
        type WeightInfo: weights::WeightInfo;
    }

    /// Switch to enable/disable activity tracking
    #[pallet::storage]
    pub type CurrentActivityTrackingStatus<T: Config> =
        StorageValue<_, ActivityTrackingStatus, ValueQuery>;

    /// A storage map of inactive collators for a session
    #[pallet::storage]
    pub type InactiveCollators<T: Config> = StorageMap<
        _,
        Twox64Concat,
        SessionIndex,
        BoundedBTreeSet<T::CollatorId, T::MaxCollatorsPerSession>,
        ValueQuery,
    >;

    /// A list of active collators for a session. Repopulated at the start of every session
    #[pallet::storage]
    pub type ActiveCollatorsForCurrentSession<T: Config> =
        StorageValue<_, BoundedBTreeSet<T::CollatorId, T::MaxCollatorsPerSession>, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event emitted when the activity tracking status is updated
        ActivityTrackingStatusSet { status: ActivityTrackingStatus },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The size of a collator set for a session has already reached MaxCollatorsPerSession value
        MaxCollatorsPerSessionReached,
        /// Error returned when the activity tracking status is attempted to be updated before the end session
        ActivityTrackingStatusUpdateSuspended,
        /// Error returned when the activity tracking status is attempted to be enabled when it is already enabled
        ActivityTrackingStatusAlreadyEnabled,
        /// Error returned when the activity tracking status is attempted to be disabled when it is already disabled
        ActivityTrackingStatusAlreadyDisabled,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::set_inactivity_tracking_status())]
        pub fn set_inactivity_tracking_status(
            origin: OriginFor<T>,
            enable_inactivity_tracking: bool,
        ) -> DispatchResult {
            ensure_root(origin)?;
            let current_status_end_session_index = match <CurrentActivityTrackingStatus<T>>::get() {
                ActivityTrackingStatus::Enabled { start: _, end } => {
                    ensure!(
                        !enable_inactivity_tracking,
                        Error::<T>::ActivityTrackingStatusAlreadyEnabled
                    );
                    end
                }
                ActivityTrackingStatus::Disabled { end } => {
                    ensure!(
                        enable_inactivity_tracking,
                        Error::<T>::ActivityTrackingStatusAlreadyDisabled
                    );
                    end
                }
            };
            let current_session_index = T::CurrentSessionIndex::session_index();
            ensure!(
                current_session_index > current_status_end_session_index,
                Error::<T>::ActivityTrackingStatusUpdateSuspended
            );
            Self::set_inactivity_tracking_status_inner(
                current_session_index,
                enable_inactivity_tracking,
            );
            Ok(())
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
            let mut total_weight = T::DbWeight::get().reads_writes(1, 0);
            // Process the orchestrator chain block author (if it exists) and activity tracking is enabled
            if let Some(orchestrator_chain_author) = T::GetSelfChainBlockAuthor::get_block_author()
            {
                total_weight.saturating_accrue(T::DbWeight::get().reads(1));
                if let ActivityTrackingStatus::Enabled { start, end: _ } =
                    <CurrentActivityTrackingStatus<T>>::get()
                {
                    total_weight.saturating_accrue(T::DbWeight::get().reads(1));
                    if start <= T::CurrentSessionIndex::session_index() {
                        total_weight
                            .saturating_accrue(Self::on_author_noted(orchestrator_chain_author));
                    }
                }
            }
            total_weight
        }
    }

    impl<T: Config> Pallet<T> {
        /// Internal function to set the activity tracking status and
        /// clear ActiveCollatorsForCurrentSession if disabled
        fn set_inactivity_tracking_status_inner(
            current_session_index: SessionIndex,
            enable_inactivity_tracking: bool,
        ) {
            let new_status_end_session_index =
                current_session_index.saturating_add(T::MaxInactiveSessions::get());
            let new_status = if enable_inactivity_tracking {
                ActivityTrackingStatus::Enabled {
                    start: current_session_index.saturating_add(1),
                    end: new_status_end_session_index,
                }
            } else {
                <ActiveCollatorsForCurrentSession<T>>::put(BoundedBTreeSet::new());
                ActivityTrackingStatus::Disabled {
                    end: new_status_end_session_index,
                }
            };
            <CurrentActivityTrackingStatus<T>>::put(new_status.clone());
            Self::deposit_event(Event::<T>::ActivityTrackingStatusSet { status: new_status })
        }

        /// Internal function to clear the active collators for the current session
        /// and remove the collators records that are outside the activity period.
        /// Triggered at the beginning of each session.
        pub fn process_ended_session() {
            if let Some(current_session_index) =
                T::CurrentSessionIndex::session_index().checked_sub(1)
            {
                match <CurrentActivityTrackingStatus<T>>::get() {
                    ActivityTrackingStatus::Disabled { .. } => return,
                    ActivityTrackingStatus::Enabled { start, end: _ } => {
                        if start > current_session_index {
                            return;
                        }
                    }
                }
                if let Ok(inactive_collators) =
                    BoundedBTreeSet::<T::CollatorId, T::MaxCollatorsPerSession>::try_from(
                        T::CurrentCollatorsFetcher::get_all_collators_assigned_to_chains(
                            ForSession::Current,
                        )
                        .difference(&<ActiveCollatorsForCurrentSession<T>>::get())
                        .cloned()
                        .collect::<BTreeSet<T::CollatorId>>(),
                    )
                {
                    InactiveCollators::<T>::insert(current_session_index, inactive_collators);
                } else {
                    // If we reach MaxCollatorsPerSession limit there must be a bug in the pallet
                    // so we disable the activity tracking
                    Self::set_inactivity_tracking_status_inner(current_session_index, false);
                }
            }

            let current_session_index = T::CurrentSessionIndex::session_index();
            <ActiveCollatorsForCurrentSession<T>>::put(BoundedBTreeSet::new());

            // Cleanup active collator info for sessions that are older than the maximum allowed
            if current_session_index > T::MaxInactiveSessions::get() {
                <crate::pallet::InactiveCollators<T>>::remove(
                    current_session_index
                        .saturating_sub(T::MaxInactiveSessions::get())
                        .saturating_sub(1),
                );
            }
        }

        /// Internal update the current session active collator records.
        /// This function is called when a container chain or orchestrator chain collator is noted.
        pub fn on_author_noted(author: T::CollatorId) -> Weight {
            let mut total_weight = T::DbWeight::get().reads_writes(1, 0);
            let _ = <ActiveCollatorsForCurrentSession<T>>::try_mutate(
                |active_collators| -> DispatchResult {
                    if let Err(_) = active_collators.try_insert(author.clone()) {
                        // If we reach MaxCollatorsPerSession limit there must be a bug in the pallet
                        // so we disable the activity tracking
                        Self::set_inactivity_tracking_status_inner(
                            T::CurrentSessionIndex::session_index(),
                            false,
                        );
                        total_weight.saturating_accrue(T::DbWeight::get().reads_writes(1, 2));
                        return Err(Error::<T>::MaxCollatorsPerSessionReached.into());
                    } else {
                        total_weight.saturating_accrue(T::DbWeight::get().writes(1));
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
        let current_session_index = T::CurrentSessionIndex::session_index();
        let minimum_sessions_required = T::MaxInactiveSessions::get();
        match <CurrentActivityTrackingStatus<T>>::get() {
            ActivityTrackingStatus::Disabled { .. } => return false,
            ActivityTrackingStatus::Enabled { start, end: _ } => {
                if start.saturating_add(minimum_sessions_required) > current_session_index {
                    return false;
                }
            }
        }

        let start_session_index = current_session_index.saturating_sub(minimum_sessions_required);
        for session_index in start_session_index..current_session_index {
            if !<InactiveCollators<T>>::get(session_index).contains(node) {
                return false;
            }
        }
        true
    }
}

impl<T: Config> AuthorNotingHook<T::CollatorId> for Pallet<T> {
    fn on_container_authors_noted(info: &[AuthorNotingInfo<T::CollatorId>]) -> Weight {
        let mut total_weight = T::DbWeight::get().reads_writes(1, 0);
        if let ActivityTrackingStatus::Enabled { start, end: _ } =
            <CurrentActivityTrackingStatus<T>>::get()
        {
            if start <= T::CurrentSessionIndex::session_index() {
                for author_info in info {
                    total_weight
                        .saturating_accrue(Self::on_author_noted(author_info.author.clone()));
                }
            }
        }
        total_weight
    }
    #[cfg(feature = "runtime-benchmarks")]
    fn prepare_worst_case_for_bench(_a: &T::CollatorId, _b: BlockNumber, _para_id: ParaId) {}
}
