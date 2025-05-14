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
use {
    crate::{
        mock::*, ActiveCollatorsForCurrentSession, ActiveContainerChainsForCurrentSession,
        ActivityTrackingStatus, AuthorNotingHook, Config, CurrentActivityTrackingStatus, Error,
        InactiveCollators, NodeActivityTrackingHelper, Pallet,
    },
    frame_support::{assert_noop, assert_ok, pallet_prelude::Get},
    sp_core::ConstU32,
    sp_runtime::{BoundedBTreeSet, DispatchError::BadOrigin},
    tp_traits::{AuthorNotingInfo, GetSessionIndex},
};

fn get_active_collators(block: u32) -> AuthorNotingInfo<AccountId> {
    AuthorNotingInfo {
        block_number: block,
        author: COLLATOR_1,
        para_id: CONTAINER_CHAIN_ID_1,
    }
}

fn get_max_active_collators_vec(block: u32) -> Vec<AuthorNotingInfo<AccountId>> {
    let total_collators: u32 = <Test as Config>::MaxCollatorsPerSession::get();
    let mut active_collators: Vec<AuthorNotingInfo<AccountId>> = Vec::new();
    for i in 0u32..total_collators {
        active_collators.push(AuthorNotingInfo {
            block_number: block + i,
            author: (i + 3).into(),
            para_id: 3002.into(),
        });
    }
    active_collators
}

fn get_overflowing_active_collators_vec(block: u32) -> Vec<AuthorNotingInfo<AccountId>> {
    let total_collators = <Test as Config>::MaxCollatorsPerSession::get();
    let mut overflowing_active_collators: Vec<AuthorNotingInfo<AccountId>> = Vec::new();
    for i in 0u32..=total_collators {
        overflowing_active_collators.push(AuthorNotingInfo {
            block_number: block + i,
            author: (i + 1).into(),
            para_id: 2000.into(),
        });
    }
    overflowing_active_collators
}

fn get_overflowing_active_chains_vec(block: u32) -> Vec<AuthorNotingInfo<AccountId>> {
    let total_chains = <Test as Config>::MaxContainerChains::get();
    let mut overflowing_active_chains: Vec<AuthorNotingInfo<AccountId>> = Vec::new();
    for i in 0u32..=total_chains {
        overflowing_active_chains.push(AuthorNotingInfo {
            block_number: block,
            author: COLLATOR_1,
            para_id: (i + 2000).into(),
        });
    }
    overflowing_active_chains
}

fn get_collator_set(
    collators: Vec<AccountId>,
) -> BoundedBTreeSet<AccountId, <Test as Config>::MaxCollatorsPerSession> {
    let mut collator_set = BoundedBTreeSet::new();
    for collator in collators {
        let _ = collator_set.try_insert(collator);
    }
    collator_set
}

fn get_active_chains_set(
    chains: Vec<tp_traits::ParaId>,
) -> BoundedBTreeSet<tp_traits::ParaId, <Test as Config>::MaxContainerChains> {
    let mut chain_set = BoundedBTreeSet::new();
    for chain in chains {
        let _ = chain_set.try_insert(chain);
    }
    chain_set
}

fn get_max_inactive_sessions() -> u32 {
    <Test as Config>::MaxInactiveSessions::get()
}

#[test]
fn enabling_and_disabling_inactivity_tracking_works() {
    ExtBuilder.build().execute_with(|| {
        assert_eq!(
            CurrentActivityTrackingStatus::<Test>::get(),
            ActivityTrackingStatus::Enabled { start: 0, end: 0 }
        );

        roll_to(SESSION_BLOCK_LENGTH);
        assert_ok!(Pallet::<Test>::set_inactivity_tracking_status(
            RuntimeOrigin::root(),
            false
        ));

        let suspension_period: u32 = get_max_inactive_sessions() + 1u32;
        assert_eq!(
            CurrentActivityTrackingStatus::<Test>::get(),
            ActivityTrackingStatus::Disabled {
                end: suspension_period
            }
        );

        roll_to(SESSION_BLOCK_LENGTH * (suspension_period as u64 + 1));

        assert_ok!(Pallet::<Test>::set_inactivity_tracking_status(
            RuntimeOrigin::root(),
            true
        ));

        assert_eq!(
            CurrentActivityTrackingStatus::<Test>::get(),
            ActivityTrackingStatus::Enabled {
                start: suspension_period + 2,
                end: 2 * suspension_period
            }
        );
        roll_to(SESSION_BLOCK_LENGTH * (2 * suspension_period as u64 + 1));

        assert_ok!(Pallet::<Test>::set_inactivity_tracking_status(
            RuntimeOrigin::root(),
            false
        ));

        assert_eq!(
            CurrentActivityTrackingStatus::<Test>::get(),
            ActivityTrackingStatus::Disabled {
                end: 3 * suspension_period
            }
        );
    });
}

#[test]
fn enabling_and_disabling_inactivity_tracking_fails_for_non_root() {
    ExtBuilder.build().execute_with(|| {
        assert_noop!(
            Pallet::<Test>::set_inactivity_tracking_status(
                RuntimeOrigin::signed(COLLATOR_1),
                false
            ),
            BadOrigin
        );
    });
}

#[test]
fn setting_the_same_inactivity_tracking_status_fails() {
    ExtBuilder.build().execute_with(|| {
        assert_noop!(
            Pallet::<Test>::set_inactivity_tracking_status(RuntimeOrigin::root(), true),
            Error::<Test>::ActivityTrackingStatusAlreadyEnabled
        );
        roll_to(SESSION_BLOCK_LENGTH);
        assert_ok!(Pallet::<Test>::set_inactivity_tracking_status(
            RuntimeOrigin::root(),
            false
        ));
        assert_noop!(
            Pallet::<Test>::set_inactivity_tracking_status(RuntimeOrigin::root(), false),
            Error::<Test>::ActivityTrackingStatusAlreadyDisabled
        );
    });
}

#[test]
fn enabling_and_disabling_inactivity_tracking_fails_if_called_before_end_of_suspension_period() {
    ExtBuilder.build().execute_with(|| {
        assert_eq!(
            CurrentActivityTrackingStatus::<Test>::get(),
            ActivityTrackingStatus::Enabled { start: 0, end: 0 }
        );
        assert_noop!(
            Pallet::<Test>::set_inactivity_tracking_status(RuntimeOrigin::root(), false),
            Error::<Test>::ActivityTrackingStatusUpdateSuspended
        );
    });
}

#[test]
fn inactivity_tracking_handler_with_enabled_and_disabled_tracking_works() {
    ExtBuilder.build().execute_with(|| {
        assert_eq!(
            <Pallet::<Test> as NodeActivityTrackingHelper<AccountId>>::is_node_inactive(
                &COLLATOR_1
            ),
            false
        );
        assert_eq!(
            <Pallet::<Test> as NodeActivityTrackingHelper<AccountId>>::is_node_inactive(
                &COLLATOR_2
            ),
            false
        );

        roll_to(get_max_inactive_sessions() as u64 * SESSION_BLOCK_LENGTH + 1);
        assert_eq!(
            <Test as Config>::CurrentSessionIndex::session_index(),
            get_max_inactive_sessions()
        );

        let inactive_collator = get_collator_set(vec![COLLATOR_1]);
        for session_id in 0..get_max_inactive_sessions() {
            InactiveCollators::<Test>::insert(session_id, inactive_collator.clone());
            assert_eq!(
                InactiveCollators::<Test>::get(session_id),
                inactive_collator
            );
        }

        assert_eq!(
            <Pallet::<Test> as NodeActivityTrackingHelper<AccountId>>::is_node_inactive(
                &COLLATOR_1
            ),
            true
        );
        assert_eq!(
            <Pallet::<Test> as NodeActivityTrackingHelper<AccountId>>::is_node_inactive(
                &COLLATOR_2
            ),
            false
        );

        assert_ok!(Pallet::<Test>::set_inactivity_tracking_status(
            RuntimeOrigin::root(),
            false
        ));

        assert_eq!(
            <Pallet::<Test> as NodeActivityTrackingHelper<AccountId>>::is_node_inactive(
                &COLLATOR_1
            ),
            false
        );
        assert_eq!(
            <Pallet::<Test> as NodeActivityTrackingHelper<AccountId>>::is_node_inactive(
                &COLLATOR_2
            ),
            false
        );
    });
}

#[test]
fn inactivity_tracking_handler_with_one_active_session_works() {
    ExtBuilder.build().execute_with(|| {
        roll_to(get_max_inactive_sessions() as u64 * SESSION_BLOCK_LENGTH + 1);

        let inactive_collator_1 = get_collator_set(vec![COLLATOR_1]);
        let inactive_collator_2 = get_collator_set(vec![COLLATOR_2]);
        InactiveCollators::<Test>::insert(0, inactive_collator_1.clone());
        InactiveCollators::<Test>::insert(1, inactive_collator_2.clone());
        assert_eq!(
            <Pallet::<Test> as NodeActivityTrackingHelper<AccountId>>::is_node_inactive(
                &COLLATOR_2
            ),
            false
        );
        assert_eq!(
            <Pallet::<Test> as NodeActivityTrackingHelper<AccountId>>::is_node_inactive(
                &COLLATOR_1
            ),
            false
        );
    });
}

#[test]
fn inactivity_tracking_handler_works_as_expected_with_no_activity_during_initial_sessions() {
    ExtBuilder.build().execute_with(|| {
        roll_to(SESSION_BLOCK_LENGTH + 1);

        assert_eq!(
            <Pallet::<Test> as NodeActivityTrackingHelper<AccountId>>::is_node_inactive(
                &COLLATOR_1
            ),
            false
        );
        assert_eq!(
            <Pallet::<Test> as NodeActivityTrackingHelper<AccountId>>::is_node_inactive(
                &COLLATOR_2
            ),
            false
        );
    });
}

#[test]
fn inactivity_tracking_handler_with_enabled_tracking_after_disabling_it_works() {
    ExtBuilder.build().execute_with(|| {
        roll_to(SESSION_BLOCK_LENGTH);
        assert_ok!(Pallet::<Test>::set_inactivity_tracking_status(
            RuntimeOrigin::root(),
            false
        ));

        let suspension_period: u32 = get_max_inactive_sessions() + 1u32;
        assert_eq!(
            CurrentActivityTrackingStatus::<Test>::get(),
            ActivityTrackingStatus::Disabled {
                end: suspension_period
            }
        );

        roll_to(SESSION_BLOCK_LENGTH * (suspension_period as u64 + 1));

        assert_ok!(Pallet::<Test>::set_inactivity_tracking_status(
            RuntimeOrigin::root(),
            true
        ));
        assert_eq!(
            CurrentActivityTrackingStatus::<Test>::get(),
            ActivityTrackingStatus::Enabled {
                start: suspension_period + 2,
                end: 2 * suspension_period
            }
        );
        assert_eq!(
            <Test as Config>::CurrentSessionIndex::session_index(),
            suspension_period + 1
        );
        let inactive_collators = get_collator_set(vec![COLLATOR_1, COLLATOR_2]);

        // Since we do not introduce any activity record, but the enabled tracking status
        // start = suspension_period + 2 < CurrentSessionIndex + MaxInactiveStatus = suspension_period + 1 + 2
        // so the collators should be considered active
        assert_eq!(
            <Pallet::<Test> as NodeActivityTrackingHelper<AccountId>>::is_node_inactive(
                &COLLATOR_1
            ),
            false
        );
        assert_eq!(
            <Pallet::<Test> as NodeActivityTrackingHelper<AccountId>>::is_node_inactive(
                &COLLATOR_2
            ),
            false
        );
        // Now start = CurrentSessionIndex so the collators should be considered active
        roll_to(SESSION_BLOCK_LENGTH * (suspension_period as u64 + 2));
        // Manually re-insert the collators to inactive collators storage after a session is processed
        // to simulate the case when the collators are inactive
        InactiveCollators::<Test>::insert(suspension_period + 1, inactive_collators.clone());
        assert_eq!(
            <Test as Config>::CurrentSessionIndex::session_index(),
            suspension_period + 2
        );
        assert_eq!(
            <Pallet::<Test> as NodeActivityTrackingHelper<AccountId>>::is_node_inactive(
                &COLLATOR_1
            ),
            false
        );
        assert_eq!(
            <Pallet::<Test> as NodeActivityTrackingHelper<AccountId>>::is_node_inactive(
                &COLLATOR_2
            ),
            false
        );
        roll_to(SESSION_BLOCK_LENGTH * (suspension_period as u64 + 3));
        InactiveCollators::<Test>::insert(suspension_period + 2, inactive_collators.clone());
        // Once CurrentSessionIndex >= start + MaxInactiveSessions collators will be considered inactive
        // since there are inactivity records for it
        roll_to(
            SESSION_BLOCK_LENGTH
                * (suspension_period as u64 + 2 + get_max_inactive_sessions() as u64),
        );
        InactiveCollators::<Test>::insert(suspension_period + 3, inactive_collators);
        assert_eq!(
            <Test as Config>::CurrentSessionIndex::session_index(),
            suspension_period + 2 + get_max_inactive_sessions()
        );
        assert_eq!(
            <Pallet::<Test> as NodeActivityTrackingHelper<AccountId>>::is_node_inactive(
                &COLLATOR_1
            ),
            true
        );
        assert_eq!(
            <Pallet::<Test> as NodeActivityTrackingHelper<AccountId>>::is_node_inactive(
                &COLLATOR_2
            ),
            true
        );
    });
}

#[test]
fn processing_ended_session_correctly_updates_current_session_collators_and_active_collators_records(
) {
    ExtBuilder.build().execute_with(|| {
        let current_session_active_collator_record: BoundedBTreeSet<AccountId, ConstU32<5>> =
            get_collator_set(vec![COLLATOR_1]);
        let inactive_collator_record: BoundedBTreeSet<AccountId, ConstU32<5>> =
            get_collator_set(vec![COLLATOR_2]);
        let current_session_active_chain_record = get_active_chains_set(vec![CONTAINER_CHAIN_ID_1]);
        let empty_set: BoundedBTreeSet<AccountId, ConstU32<5>> = BoundedBTreeSet::new();

        ActiveCollatorsForCurrentSession::<Test>::put(
            current_session_active_collator_record.clone(),
        );
        ActiveContainerChainsForCurrentSession::<Test>::put(
            current_session_active_chain_record.clone(),
        );

        assert_eq!(
            ActiveCollatorsForCurrentSession::<Test>::get(),
            current_session_active_collator_record
        );
        assert_eq!(
            ActiveContainerChainsForCurrentSession::<Test>::get(),
            current_session_active_chain_record
        );

        assert_eq!(InactiveCollators::<Test>::get(0), empty_set);

        roll_to(SESSION_BLOCK_LENGTH + 1);

        assert_eq!(ActiveCollatorsForCurrentSession::<Test>::get(), empty_set);
        assert_eq!(InactiveCollators::<Test>::get(0), inactive_collator_record);
    });
}

#[test]
fn processing_ended_session_correctly_cleans_outdated_collator_records() {
    ExtBuilder.build().execute_with(|| {
        assert_eq!(<Test as Config>::CurrentSessionIndex::session_index(), 0);

        let current_session_active_collator_record: BoundedBTreeSet<AccountId, ConstU32<5>> =
            get_collator_set(vec![COLLATOR_1]);
        let inactive_collator_record: BoundedBTreeSet<AccountId, ConstU32<5>> =
            get_collator_set(vec![COLLATOR_2]);
        let current_session_active_chain_record = get_active_chains_set(vec![CONTAINER_CHAIN_ID_1]);
        let empty_set: BoundedBTreeSet<AccountId, ConstU32<5>> = BoundedBTreeSet::new();

        ActiveCollatorsForCurrentSession::<Test>::put(
            current_session_active_collator_record.clone(),
        );
        ActiveContainerChainsForCurrentSession::<Test>::put(
            current_session_active_chain_record.clone(),
        );

        assert_eq!(
            ActiveCollatorsForCurrentSession::<Test>::get(),
            current_session_active_collator_record
        );
        assert_eq!(
            ActiveContainerChainsForCurrentSession::<Test>::get(),
            current_session_active_chain_record
        );
        assert_eq!(InactiveCollators::<Test>::get(0), empty_set);

        roll_to(SESSION_BLOCK_LENGTH);

        assert_eq!(ActiveCollatorsForCurrentSession::<Test>::get(), empty_set);
        assert_eq!(InactiveCollators::<Test>::get(0), inactive_collator_record);

        roll_to(get_max_inactive_sessions() as u64 * SESSION_BLOCK_LENGTH + 1);
        assert_eq!(
            <Test as Config>::CurrentSessionIndex::session_index(),
            get_max_inactive_sessions()
        );

        assert_eq!(ActiveCollatorsForCurrentSession::<Test>::get(), empty_set);
        assert_eq!(InactiveCollators::<Test>::get(0), inactive_collator_record);

        roll_to((get_max_inactive_sessions() as u64 + 1) * SESSION_BLOCK_LENGTH + 1);
        assert_eq!(
            <Test as Config>::CurrentSessionIndex::session_index(),
            get_max_inactive_sessions() + 1
        );

        assert_eq!(ActiveCollatorsForCurrentSession::<Test>::get(), empty_set);
        assert_eq!(InactiveCollators::<Test>::get(0), empty_set);
    });
}

#[test]
fn active_collators_noting_for_current_session_works_when_activity_tracking_is_enabled() {
    ExtBuilder.build().execute_with(|| {
        let current_session_active_collator_record: BoundedBTreeSet<AccountId, ConstU32<5>> =
            get_collator_set(vec![COLLATOR_1]);
        assert_eq!(ActiveCollatorsForCurrentSession::<Test>::get().len(), 0);
        roll_to(2);
        <Pallet<Test> as AuthorNotingHook<AccountId>>::on_container_authors_noted(&[
            get_active_collators(2),
        ]);
        assert_eq!(
            ActiveCollatorsForCurrentSession::<Test>::get(),
            current_session_active_collator_record
        );
        roll_to(3);
        <Pallet<Test> as AuthorNotingHook<AccountId>>::on_container_authors_noted(&[
            get_active_collators(3),
        ]);
        assert_eq!(
            ActiveCollatorsForCurrentSession::<Test>::get(),
            current_session_active_collator_record
        );
    });
}

#[test]
fn active_collators_noting_for_current_session_works_when_activity_tracking_is_disabled_then_enabled(
) {
    ExtBuilder.build().execute_with(|| {
        assert_eq!(ActiveCollatorsForCurrentSession::<Test>::get().len(), 0);

        roll_to(SESSION_BLOCK_LENGTH);
        assert_ok!(Pallet::<Test>::set_inactivity_tracking_status(
            RuntimeOrigin::root(),
            false
        ));
        // Since the activity tracking is disabled, the active collators noting
        // should not update ActiveCollatorsForCurrentSession
        <Pallet<Test> as AuthorNotingHook<AccountId>>::on_container_authors_noted(&[
            get_active_collators(SESSION_BLOCK_LENGTH as u32),
        ]);
        assert_eq!(ActiveCollatorsForCurrentSession::<Test>::get().len(), 0);
        let suspension_period: u32 = get_max_inactive_sessions() + 1u32;
        roll_to(SESSION_BLOCK_LENGTH * (suspension_period as u64 + 1));
        assert_ok!(Pallet::<Test>::set_inactivity_tracking_status(
            RuntimeOrigin::root(),
            true
        ));
        assert_eq!(
            CurrentActivityTrackingStatus::<Test>::get(),
            ActivityTrackingStatus::Enabled {
                start: suspension_period + 2,
                end: 2 * suspension_period
            }
        );
        // Since the activity tracking is enabled,
        // start = suspension_period + 2 and current session = suspension_period + 1 so
        // the active collators noting should not update ActiveCollatorsForCurrentSession
        <Pallet<Test> as AuthorNotingHook<AccountId>>::on_container_authors_noted(&[
            get_active_collators(SESSION_BLOCK_LENGTH as u32),
        ]);
        assert_eq!(ActiveCollatorsForCurrentSession::<Test>::get().len(), 0);

        roll_to(SESSION_BLOCK_LENGTH * (2 * suspension_period as u64 + 1));
        <Pallet<Test> as AuthorNotingHook<AccountId>>::on_container_authors_noted(&[
            get_active_collators(SESSION_BLOCK_LENGTH as u32),
        ]);
        let current_session_active_collator_record: BoundedBTreeSet<AccountId, ConstU32<5>> =
            get_collator_set(vec![COLLATOR_1]);
        assert_eq!(
            ActiveCollatorsForCurrentSession::<Test>::get(),
            current_session_active_collator_record
        );
    });
}

#[test]
fn disabling_inactivity_tracking_clears_the_current_active_collators_storage() {
    ExtBuilder.build().execute_with(|| {
        assert_eq!(
            CurrentActivityTrackingStatus::<Test>::get(),
            ActivityTrackingStatus::Enabled { start: 0, end: 0 }
        );

        roll_to(SESSION_BLOCK_LENGTH);
        <Pallet<Test> as AuthorNotingHook<AccountId>>::on_container_authors_noted(&[
            get_active_collators(SESSION_BLOCK_LENGTH as u32),
        ]);
        assert_eq!(ActiveCollatorsForCurrentSession::<Test>::get().len(), 1);

        roll_to(SESSION_BLOCK_LENGTH);
        assert_ok!(Pallet::<Test>::set_inactivity_tracking_status(
            RuntimeOrigin::root(),
            false
        ));
        assert_eq!(
            CurrentActivityTrackingStatus::<Test>::get(),
            ActivityTrackingStatus::Disabled {
                end: get_max_inactive_sessions() + 1
            }
        );
        assert_eq!(ActiveCollatorsForCurrentSession::<Test>::get().len(), 0);
    });
}

#[test]
fn inactivity_tracking_is_disabled_if_current_active_collators_storage_overflows() {
    ExtBuilder.build().execute_with(|| {
        assert_eq!(
            CurrentActivityTrackingStatus::<Test>::get(),
            ActivityTrackingStatus::Enabled { start: 0, end: 0 }
        );
        roll_to(1);
        <Pallet<Test> as AuthorNotingHook<AccountId>>::on_container_authors_noted(
            &get_overflowing_active_collators_vec(1).as_slice(),
        );
        assert_eq!(
            CurrentActivityTrackingStatus::<Test>::get(),
            ActivityTrackingStatus::Disabled { end: 2 }
        );
    });
}

#[test]
fn inactivity_tracking_is_disabled_if_ctive_chains_storage_overflows() {
    ExtBuilder.build().execute_with(|| {
        assert_eq!(
            CurrentActivityTrackingStatus::<Test>::get(),
            ActivityTrackingStatus::Enabled { start: 0, end: 0 }
        );
        roll_to(1);
        <Pallet<Test> as AuthorNotingHook<AccountId>>::on_container_authors_noted(
            &get_overflowing_active_chains_vec(1).as_slice(),
        );
        assert_eq!(
            CurrentActivityTrackingStatus::<Test>::get(),
            ActivityTrackingStatus::Disabled { end: 2 }
        );
    });
}

#[test]
fn inactivity_tracking_is_disabled_if_current_active_collators_storage_overflows_while_processing_inactive_chains_in_the_end_of_a_session(
) {
    ExtBuilder.build().execute_with(|| {
        assert_eq!(
            CurrentActivityTrackingStatus::<Test>::get(),
            ActivityTrackingStatus::Enabled { start: 0, end: 0 }
        );
        roll_to(1);
        // We note the max active collators for the session which are not COLLATOR_1 and COLLATOR_2
        // for chains that are not CONTAINER_CHAIN_ID_1 and CONTAINER_CHAIN_ID_2
        <Pallet<Test> as AuthorNotingHook<AccountId>>::on_container_authors_noted(
            &get_max_active_collators_vec(1),
        );
        // Since the active collators storage is not overflowing,
        // we will not disable the activity tracking
        assert_eq!(
            CurrentActivityTrackingStatus::<Test>::get(),
            ActivityTrackingStatus::Enabled { start: 0, end: 0 }
        );
        // Since chain with id CONTAINER_CHAIN_ID_1 is inactive COLLATOR_1 and COLLATOR_2
        // will be added to the active collators storage for current session and it will overflow
        roll_to(SESSION_BLOCK_LENGTH + 1);
        assert_eq!(
            CurrentActivityTrackingStatus::<Test>::get(),
            ActivityTrackingStatus::Disabled { end: 2 }
        );
        assert_eq!(InactiveCollators::<Test>::get(0).len(), 0);
    });
}

#[test]
fn active_chains_noting_for_current_session_works_when_activity_tracking_is_enabled() {
    ExtBuilder.build().execute_with(|| {
        let current_session_active_chain_record = get_active_chains_set(vec![CONTAINER_CHAIN_ID_1]);
        assert_eq!(
            ActiveContainerChainsForCurrentSession::<Test>::get().len(),
            0
        );
        roll_to(2);
        <Pallet<Test> as AuthorNotingHook<AccountId>>::on_container_authors_noted(&[
            get_active_collators(2),
        ]);
        assert_eq!(
            ActiveContainerChainsForCurrentSession::<Test>::get(),
            current_session_active_chain_record
        );
        roll_to(3);
        <Pallet<Test> as AuthorNotingHook<AccountId>>::on_container_authors_noted(&[
            get_active_collators(3),
        ]);
        assert_eq!(
            ActiveContainerChainsForCurrentSession::<Test>::get(),
            current_session_active_chain_record
        );
    });
}
#[test]
fn active_chains_noting_for_current_session_works_when_activity_tracking_is_disabled_than_enabled()
{
    ExtBuilder.build().execute_with(|| {
        let current_session_active_chain_record = get_active_chains_set(vec![CONTAINER_CHAIN_ID_1]);
        assert_eq!(
            ActiveContainerChainsForCurrentSession::<Test>::get().len(),
            0
        );
        roll_to(SESSION_BLOCK_LENGTH);
        assert_ok!(Pallet::<Test>::set_inactivity_tracking_status(
            RuntimeOrigin::root(),
            false
        ));
        <Pallet<Test> as AuthorNotingHook<AccountId>>::on_container_authors_noted(&[
            get_active_collators(SESSION_BLOCK_LENGTH as u32),
        ]);
        assert_eq!(
            ActiveContainerChainsForCurrentSession::<Test>::get().len(),
            0
        );
        let disabled_tracking_status_end_block: u32 =
            (get_max_inactive_sessions() + 2u32) * SESSION_BLOCK_LENGTH as u32;
        roll_to(disabled_tracking_status_end_block as u64);
        assert_ok!(Pallet::<Test>::set_inactivity_tracking_status(
            RuntimeOrigin::root(),
            true
        ));
        // Since the activity tracking is enabled we will need to wait until the start of
        // the next session before we are able to updating ActiveContainerChainsForCurrentSession
        <Pallet<Test> as AuthorNotingHook<AccountId>>::on_container_authors_noted(&[
            get_active_collators(disabled_tracking_status_end_block),
        ]);
        assert_eq!(
            ActiveContainerChainsForCurrentSession::<Test>::get().len(),
            0
        );
        roll_to(disabled_tracking_status_end_block as u64 + SESSION_BLOCK_LENGTH);
        <Pallet<Test> as AuthorNotingHook<AccountId>>::on_container_authors_noted(&[
            get_active_collators(disabled_tracking_status_end_block + SESSION_BLOCK_LENGTH as u32),
        ]);
        assert_eq!(
            ActiveContainerChainsForCurrentSession::<Test>::get(),
            current_session_active_chain_record
        );
    });
}

#[test]
fn inactive_chain_collators_are_correctly_processed_when_activity_tracking_is_enabled() {
    ExtBuilder.build().execute_with(|| {
        assert_eq!(
            ActiveContainerChainsForCurrentSession::<Test>::get().is_empty(),
            true
        );
        assert_eq!(
            ActiveCollatorsForCurrentSession::<Test>::get().is_empty(),
            true
        );
        roll_to(SESSION_BLOCK_LENGTH - 1);
        assert_eq!(
            ActiveContainerChainsForCurrentSession::<Test>::get().is_empty(),
            true
        );
        assert_eq!(
            ActiveCollatorsForCurrentSession::<Test>::get().is_empty(),
            true
        );
        roll_to(SESSION_BLOCK_LENGTH);
        assert_eq!(InactiveCollators::<Test>::get(0).is_empty(), true);
    });
}

#[test]
fn inactive_collator_for_active_chain_is_correctly_processed_when_activity_tracking_is_enabled() {
    ExtBuilder.build().execute_with(|| {
        let current_session_active_collator_record = get_collator_set(vec![COLLATOR_1]);
        let current_session_active_chain_record = get_active_chains_set(vec![CONTAINER_CHAIN_ID_1]);
        assert_eq!(
            ActiveContainerChainsForCurrentSession::<Test>::get().len(),
            0
        );
        assert_eq!(ActiveCollatorsForCurrentSession::<Test>::get().len(), 0);
        roll_to(2);
        <Pallet<Test> as AuthorNotingHook<AccountId>>::on_container_authors_noted(&[
            get_active_collators(2),
        ]);
        roll_to(SESSION_BLOCK_LENGTH - 1);
        assert_eq!(
            ActiveContainerChainsForCurrentSession::<Test>::get(),
            current_session_active_chain_record
        );
        assert_eq!(
            ActiveCollatorsForCurrentSession::<Test>::get(),
            current_session_active_collator_record
        );
        roll_to(SESSION_BLOCK_LENGTH);
        assert_eq!(
            InactiveCollators::<Test>::get(0),
            get_collator_set(vec![COLLATOR_2])
        );
    });
}

#[test]
fn inactive_chain_collators_are_processed_correctly_when_activity_tracking_is_disabled_than_enabled(
) {
    ExtBuilder.build().execute_with(|| {
        let last_disabled_session_id = get_max_inactive_sessions() + 2u32;
        assert_eq!(
            ActiveContainerChainsForCurrentSession::<Test>::get().len(),
            0
        );
        roll_to(SESSION_BLOCK_LENGTH);
        assert_ok!(Pallet::<Test>::set_inactivity_tracking_status(
            RuntimeOrigin::root(),
            false
        ));

        roll_to(2 * SESSION_BLOCK_LENGTH - 1);
        ActiveContainerChainsForCurrentSession::<Test>::put(get_active_chains_set(vec![
            CONTAINER_CHAIN_ID_1,
        ]));
        ActiveCollatorsForCurrentSession::<Test>::put(get_collator_set(vec![COLLATOR_1]));

        roll_to(2 * SESSION_BLOCK_LENGTH);
        assert_eq!(
            ActiveContainerChainsForCurrentSession::<Test>::get().len(),
            0
        );
        assert_eq!(
            ActiveContainerChainsForCurrentSession::<Test>::get().len(),
            0
        );
        assert_eq!(InactiveCollators::<Test>::get(1).len(), 0);

        let disabled_tracking_status_end_block: u32 =
            last_disabled_session_id * SESSION_BLOCK_LENGTH as u32;
        roll_to(disabled_tracking_status_end_block as u64);
        assert_ok!(Pallet::<Test>::set_inactivity_tracking_status(
            RuntimeOrigin::root(),
            true
        ));

        // Since the activity tracking is enabled we will need to wait until the start of
        // the next session before process_inactive_chains_for_session() is enabled
        roll_to(disabled_tracking_status_end_block as u64 + SESSION_BLOCK_LENGTH - 1);
        ActiveContainerChainsForCurrentSession::<Test>::put(get_active_chains_set(vec![
            CONTAINER_CHAIN_ID_1,
        ]));
        ActiveCollatorsForCurrentSession::<Test>::put(get_collator_set(vec![COLLATOR_1]));

        roll_to(disabled_tracking_status_end_block as u64 + SESSION_BLOCK_LENGTH);
        assert_eq!(
            ActiveContainerChainsForCurrentSession::<Test>::get().len(),
            0
        );
        assert_eq!(
            ActiveContainerChainsForCurrentSession::<Test>::get().len(),
            0
        );
        assert_eq!(
            InactiveCollators::<Test>::get(last_disabled_session_id).len(),
            0
        );

        roll_to(disabled_tracking_status_end_block as u64 + 2 * SESSION_BLOCK_LENGTH - 1);
        ActiveContainerChainsForCurrentSession::<Test>::put(get_active_chains_set(vec![
            CONTAINER_CHAIN_ID_1,
        ]));
        ActiveCollatorsForCurrentSession::<Test>::put(get_collator_set(vec![COLLATOR_1]));

        roll_to(disabled_tracking_status_end_block as u64 + 2 * SESSION_BLOCK_LENGTH);
        assert_eq!(
            ActiveContainerChainsForCurrentSession::<Test>::get().len(),
            0
        );
        assert_eq!(
            ActiveContainerChainsForCurrentSession::<Test>::get().len(),
            0
        );
        assert_eq!(
            InactiveCollators::<Test>::get(last_disabled_session_id + 1),
            get_collator_set(vec![COLLATOR_2])
        );
    });
}
