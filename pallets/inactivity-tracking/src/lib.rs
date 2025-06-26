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
    frame_support::{
        dispatch::DispatchResult, dispatch::DispatchResultWithPostInfo, ensure,
        pallet_prelude::Weight,
    },
    parity_scale_codec::{Decode, Encode},
    scale_info::TypeInfo,
    serde::{Deserialize, Serialize},
    sp_core::{MaxEncodedLen, RuntimeDebug},
    sp_runtime::{traits::Get, BoundedBTreeSet},
    sp_staking::SessionIndex,
    tp_traits::{
        AuthorNotingHook, AuthorNotingInfo, ForSession, GetContainerChainsWithCollators,
        GetSessionIndex, InvulnerablesHelper, MaybeSelfChainBlockAuthor,
        NodeActivityTrackingHelper, NotifyCollatorOnlineStatusChange, ParaId, ParathreadHelper,
        PendingCollatorAssignmentHelper,
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
use tp_traits::BlockNumber;

pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
    use {
        super::*,
        crate::weights::WeightInfo,
        core::marker::PhantomData,
        frame_support::{pallet_prelude::*, storage::types::StorageMap},
        frame_system::pallet_prelude::*,
        sp_std::collections::btree_set::BTreeSet,
    };

    pub type Collator<T> = <T as frame_system::Config>::AccountId;

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

        /// Helper that fetches a list of collators eligible to produce blocks for the current session
        type CurrentCollatorsFetcher: GetContainerChainsWithCollators<Collator<Self>>
            + PendingCollatorAssignmentHelper<Collator<Self>>;

        /// Helper that returns the block author for the orchestrator chain (if it exists)
        type GetSelfChainBlockAuthor: MaybeSelfChainBlockAuthor<Collator<Self>>;

        /// Helper that checks if a ParaId is a parathread
        type ParaFilter: ParathreadHelper;

        /// Helper for dealing with invulnerables.
        type InvulnerablesFilter: InvulnerablesHelper<Collator<Self>>;

        /// Helper for dealing with collator's stake
        type CollatorStakeHelper: NotifyCollatorOnlineStatusChange<Collator<Self>>;

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
        BoundedBTreeSet<Collator<T>, T::MaxCollatorsPerSession>,
        ValueQuery,
    >;

    /// A list of active collators for a session. Repopulated at the start of every session
    #[pallet::storage]
    pub type ActiveCollatorsForCurrentSession<T: Config> =
        StorageValue<_, BoundedBTreeSet<Collator<T>, T::MaxCollatorsPerSession>, ValueQuery>;

    /// A list of active container chains for a session. Repopulated at the start of every session
    #[pallet::storage]
    pub type ActiveContainerChainsForCurrentSession<T: Config> =
        StorageValue<_, BoundedBTreeSet<ParaId, T::MaxContainerChains>, ValueQuery>;

    /// Switch to enable/disable offline marking.
    #[pallet::storage]
    pub type EnableMarkingOffline<T: Config> = StorageValue<_, bool, ValueQuery>;

    /// Storage map indicating the offline status of a collator
    #[pallet::storage]
    pub type OfflineCollators<T: Config> =
        StorageMap<_, Blake2_128Concat, Collator<T>, bool, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event emitted when the activity tracking status is updated
        ActivityTrackingStatusSet { status: ActivityTrackingStatus },
        /// Collator online status updated
        CollatorStatusUpdated {
            collator: Collator<T>,
            is_offline: bool,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The size of a collator set for a session has already reached MaxCollatorsPerSession value
        MaxCollatorsPerSessionReached,
        /// The size of a chains set for a session has already reached MaxContainerChains value
        MaxContainerChainsReached,
        /// Error returned when the activity tracking status is attempted to be updated before the end session
        ActivityTrackingStatusUpdateSuspended,
        /// Error returned when the activity tracking status is attempted to be enabled when it is already enabled
        ActivityTrackingStatusAlreadyEnabled,
        /// Error returned when the activity tracking status is attempted to be disabled when it is already disabled
        ActivityTrackingStatusAlreadyDisabled,
        /// Error returned when the collator status is attempted to be set to offline when offline marking is disabled
        MarkingOfflineNotEnabled,
        /// Error returned when the collator is not part of the sorted eligible candidates list
        CollatorNotInSortedEligibleCandidates,
        /// Error returned when the collator status is attempted to be set to offline when it is already offline
        CollatorNotOnline,
        /// Error returned when the collator status is attempted to be set to online when it is already online
        CollatorNotOffline,
        /// Error returned when the collator attempted to be set offline is invulnerable
        MarkingInvulnerableOfflineInvalid,
        /// Error returned when the collator attempted to be set offline is not inactive
        CollatorCannotBeNotifiedAsInactive,
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

        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::enable_offline_marking())]
        pub fn enable_offline_marking(origin: OriginFor<T>, value: bool) -> DispatchResult {
            ensure_root(origin)?;
            <EnableMarkingOffline<T>>::set(value);
            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::set_offline())]
        pub fn set_offline(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let collator = ensure_signed(origin)?;
            Self::mark_collator_offline(&collator)
        }

        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::set_online())]
        pub fn set_online(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let collator = ensure_signed(origin)?;
            ensure!(
                <OfflineCollators<T>>::get(&collator),
                Error::<T>::CollatorNotOffline
            );
            <OfflineCollators<T>>::insert(collator.clone(), false);
            T::CollatorStakeHelper::update_staking_on_online_status_change(&collator)?;
            Self::deposit_event(Event::<T>::CollatorStatusUpdated {
                collator,
                is_offline: false,
            });
            Ok(().into())
        }

        #[pallet::call_index(4)]
        #[pallet::weight(T::WeightInfo::notify_inactive_collator())]
        pub fn notify_inactive_collator(
            origin: OriginFor<T>,
            collator: Collator<T>,
        ) -> DispatchResultWithPostInfo {
            ensure_signed(origin)?;
            ensure!(
                Self::is_node_inactive(&collator),
                Error::<T>::CollatorCannotBeNotifiedAsInactive
            );
            Self::mark_collator_offline(&collator)
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
            let current_session_index = T::CurrentSessionIndex::session_index();
            <ActiveCollatorsForCurrentSession<T>>::put(BoundedBTreeSet::new());
            <ActiveContainerChainsForCurrentSession<T>>::put(BoundedBTreeSet::new());

            // Cleanup active collator info for sessions that are older than the maximum allowed
            if current_session_index > T::MaxInactiveSessions::get() {
                <crate::pallet::InactiveCollators<T>>::remove(
                    current_session_index
                        .saturating_sub(T::MaxInactiveSessions::get())
                        .saturating_sub(1),
                );
            }
        }

        /// Internal function to populate the inactivity tracking storage used for marking collator
        /// as inactive. Triggered at the end of a session.
        pub fn on_before_session_ending() {
            let current_session_index = T::CurrentSessionIndex::session_index();
            Self::process_inactive_chains_for_session();
            match <CurrentActivityTrackingStatus<T>>::get() {
                ActivityTrackingStatus::Disabled { .. } => return,
                ActivityTrackingStatus::Enabled { start, end: _ } => {
                    if start > current_session_index {
                        return;
                    }
                }
            }
            if let Ok(inactive_collators) =
                BoundedBTreeSet::<Collator<T>, T::MaxCollatorsPerSession>::try_from(
                    T::CurrentCollatorsFetcher::get_all_collators_assigned_to_chains(
                        ForSession::Current,
                    )
                    .difference(&<ActiveCollatorsForCurrentSession<T>>::get())
                    .cloned()
                    .collect::<BTreeSet<Collator<T>>>(),
                )
            {
                InactiveCollators::<T>::insert(current_session_index, inactive_collators);
            } else {
                // If we reach MaxCollatorsPerSession limit there must be a bug in the pallet
                // so we disable the activity tracking
                Self::set_inactivity_tracking_status_inner(current_session_index, false);
            }
        }

        /// Internal function to populate the current session active collator records with collators
        /// part of inactive chains.
        pub fn process_inactive_chains_for_session() {
            match <CurrentActivityTrackingStatus<T>>::get() {
                ActivityTrackingStatus::Disabled { .. } => return,
                ActivityTrackingStatus::Enabled { start, end: _ } => {
                    if start > T::CurrentSessionIndex::session_index() {
                        return;
                    }
                }
            }
            let mut active_chains = <ActiveContainerChainsForCurrentSession<T>>::get().into_inner();
            // Removing the parathreads for the current session from the active chains array.
            // In this way we handle all parathreads as inactive chains.
            // This solution would only work if a collator either:
            // - is assigned to one chain only
            // - is assigned to multiple chains but all of them are parathreads
            active_chains = active_chains
                .difference(&T::ParaFilter::get_parathreads_for_session())
                .cloned()
                .collect::<BTreeSet<ParaId>>();

            let _ = <ActiveCollatorsForCurrentSession<T>>::try_mutate(
                |active_collators| -> DispatchResult {
                    let container_chains_with_collators =
                        T::CurrentCollatorsFetcher::container_chains_with_collators(
                            ForSession::Current,
                        );

                    for (para_id, collator_ids) in container_chains_with_collators.iter() {
                        if !active_chains.contains(para_id) {
                            // Collators assigned to inactive chain are added
                            // to the current active collators storage
                            for collator_id in collator_ids {
                                if let Err(_) = active_collators.try_insert(collator_id.clone()) {
                                    // If we reach MaxCollatorsPerSession limit there must be a bug in the pallet
                                    // so we disable the activity tracking
                                    Self::set_inactivity_tracking_status_inner(
                                        T::CurrentSessionIndex::session_index(),
                                        false,
                                    );
                                    return Err(Error::<T>::MaxCollatorsPerSessionReached.into());
                                }
                            }
                        }
                    }
                    Ok(())
                },
            );
        }

        /// Internal update the current session active collator records.
        /// This function is called when a container chain or orchestrator chain collator is noted.
        pub fn on_author_noted(author: Collator<T>) -> Weight {
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

        /// Internal update the current session active chains records.
        /// This function is called when a container chain is noted.
        pub fn on_chain_noted(chain_id: ParaId) -> Weight {
            let mut total_weight = T::DbWeight::get().reads_writes(1, 0);
            let _ = <ActiveContainerChainsForCurrentSession<T>>::try_mutate(
                |active_chains| -> DispatchResult {
                    if let Err(_) = active_chains.try_insert(chain_id) {
                        // If we reach MaxContainerChains limit there must be a bug in the pallet
                        // so we disable the activity tracking
                        Self::set_inactivity_tracking_status_inner(
                            T::CurrentSessionIndex::session_index(),
                            false,
                        );
                        total_weight.saturating_accrue(T::DbWeight::get().reads_writes(1, 2));
                        return Err(Error::<T>::MaxContainerChainsReached.into());
                    } else {
                        total_weight += T::DbWeight::get().writes(1);
                    }
                    Ok(())
                },
            );
            total_weight
        }

        /// Internal function to mark a collator as offline.
        pub fn mark_collator_offline(collator: &Collator<T>) -> DispatchResultWithPostInfo {
            ensure!(
                <EnableMarkingOffline<T>>::get(),
                Error::<T>::MarkingOfflineNotEnabled
            );
            ensure!(
                T::CollatorStakeHelper::is_collator_in_sorted_eligible_candidates(collator),
                Error::<T>::CollatorNotInSortedEligibleCandidates
            );
            ensure!(
                !<OfflineCollators<T>>::get(collator.clone()),
                Error::<T>::CollatorNotOnline
            );
            ensure!(
                !T::InvulnerablesFilter::is_invulnerable(collator),
                Error::<T>::MarkingInvulnerableOfflineInvalid
            );
            <OfflineCollators<T>>::insert(collator.clone(), true);
            T::CollatorStakeHelper::update_staking_on_online_status_change(collator)?;
            // To prevent the collator from being assigned to any container chain in the next session
            // we need to remove it from the pending collator assignment
            T::CurrentCollatorsFetcher::remove_offline_collator_from_pending_assignment(collator);
            Self::deposit_event(Event::<T>::CollatorStatusUpdated {
                collator: collator.clone(),
                is_offline: true,
            });
            Ok(().into())
        }
    }
}

impl<T: Config> NodeActivityTrackingHelper<Collator<T>> for Pallet<T> {
    fn is_node_inactive(node: &Collator<T>) -> bool {
        // If inactivity tracking is not enabled all nodes are considered active.
        // We don't need to check the activity records and can return false
        // Inactivity tracking is not enabled if
        // - the status is disabled
        // - the CurrentSessionIndex < start session + MaxInactiveSessions index since there won't be
        // sufficient activity records to determine inactivity
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
    fn is_node_offline(node: &Collator<T>) -> bool {
        <OfflineCollators<T>>::get(node)
    }

    fn set_offline(node: &Collator<T>) -> DispatchResultWithPostInfo {
        Self::mark_collator_offline(node)
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn make_node_inactive(node: &Collator<T>) {
        // First we need to make sure that there are enough session
        // so the node can be marked
        let max_inactive_sessions = T::MaxInactiveSessions::get();
        if T::CurrentSessionIndex::session_index() < max_inactive_sessions {
            T::CurrentSessionIndex::skip_to_session(max_inactive_sessions)
        }

        // Now we can insert the node as inactive for all sessions in the current inactivity window
        let mut inactive_nodes_set: BoundedBTreeSet<
            Collator<T>,
            <T as Config>::MaxCollatorsPerSession,
        > = BoundedBTreeSet::new();
        inactive_nodes_set.try_insert(node.clone());
        for session_index in 0..max_inactive_sessions {
            <InactiveCollators<T>>::insert(session_index, inactive_nodes_set.clone());
        }
    }
}

impl<T: Config> AuthorNotingHook<Collator<T>> for Pallet<T> {
    fn on_container_authors_noted(info: &[AuthorNotingInfo<Collator<T>>]) -> Weight {
        let mut total_weight = T::DbWeight::get().reads_writes(1, 0);
        if let ActivityTrackingStatus::Enabled { start, end: _ } =
            <CurrentActivityTrackingStatus<T>>::get()
        {
            if start <= T::CurrentSessionIndex::session_index() {
                for author_info in info {
                    total_weight
                        .saturating_accrue(Self::on_author_noted(author_info.author.clone()));
                    total_weight.saturating_accrue(Self::on_chain_noted(author_info.para_id));
                }
            }
        }
        total_weight
    }
    #[cfg(feature = "runtime-benchmarks")]
    fn prepare_worst_case_for_bench(_a: &Collator<T>, _b: BlockNumber, _para_id: ParaId) {}
}
