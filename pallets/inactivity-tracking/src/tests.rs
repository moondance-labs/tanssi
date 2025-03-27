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
        mock::*, ActiveCollators, ActiveCollatorsForCurrentSession, ActivityTrackingStatus,
        AuthorNotingHook, Config, CurrentActivityTrackingStatus, Error, NodeActivityTrackingHelper,
        Pallet,
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
        para_id: CONTAINER_CHAIN_ID,
    }
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

fn get_max_inactive_sessions() -> u32 {
    <Test as Config>::MaxInactiveSessions::get()
}

#[test]
fn enabling_and_disabling_inactivity_tracking_works() {
    ExtBuilder.build().execute_with(|| {
        assert_eq!(
            CurrentActivityTrackingStatus::<Test>::get(),
            ActivityTrackingStatus::Enabled { end: 0 }
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
                end: 2 * suspension_period
            }
        );
        roll_to(SESSION_BLOCK_LENGTH * (2 * suspension_period as u64 + 1));

        assert_ok!(Pallet::<Test>::set_inactivity_tracking_status(
            RuntimeOrigin::root(),
            true
        ));

        assert_eq!(
            CurrentActivityTrackingStatus::<Test>::get(),
            ActivityTrackingStatus::Enabled {
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
fn enabling_and_disabling_inactivity_tracking_fails_if_called_before_end_of_suspension_period() {
    ExtBuilder.build().execute_with(|| {
        assert_eq!(
            CurrentActivityTrackingStatus::<Test>::get(),
            ActivityTrackingStatus::Enabled { end: 0 }
        );
        assert_noop!(
            Pallet::<Test>::set_inactivity_tracking_status(RuntimeOrigin::root(), true),
            Error::<Test>::ActivityStatusUpdateSuspended
        );
    });
}

#[test]
fn inactivity_tracking_handler_with_enabled_or_disabled_tracking_works() {
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

        let active_collator = get_collator_set(vec![COLLATOR_1]);
        for session_id in 0..get_max_inactive_sessions() {
            ActiveCollators::<Test>::insert(session_id, active_collator.clone());
            assert_eq!(ActiveCollators::<Test>::get(session_id), active_collator);
        }

        assert_eq!(
            <Pallet::<Test> as NodeActivityTrackingHelper<AccountId>>::is_node_inactive(
                &COLLATOR_2
            ),
            true
        );
        assert_eq!(
            <Pallet::<Test> as NodeActivityTrackingHelper<AccountId>>::is_node_inactive(
                &COLLATOR_1
            ),
            false
        );

        assert_ok!(Pallet::<Test>::set_inactivity_tracking_status(
            RuntimeOrigin::root(),
            false
        ));

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
fn inactivity_tracking_handler_with_one_active_session_works() {
    ExtBuilder.build().execute_with(|| {
        roll_to(get_max_inactive_sessions() as u64 * SESSION_BLOCK_LENGTH + 1);

        let active_collator_1 = get_collator_set(vec![COLLATOR_1]);
        let active_collator_2 = get_collator_set(vec![COLLATOR_2]);
        ActiveCollators::<Test>::insert(0, active_collator_1.clone());
        ActiveCollators::<Test>::insert(1, active_collator_2.clone());
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
fn processing_ended_session_correctly_updates_current_session_collators_and_active_collators_records(
) {
    ExtBuilder.build().execute_with(|| {
        roll_to(SESSION_BLOCK_LENGTH + 1);

        let current_session_active_collator_record: BoundedBTreeSet<AccountId, ConstU32<5>> =
            get_collator_set(vec![COLLATOR_1]);
        let empty_vec: BoundedBTreeSet<AccountId, ConstU32<5>> = BoundedBTreeSet::new();

        ActiveCollatorsForCurrentSession::<Test>::put(
            current_session_active_collator_record.clone(),
        );

        assert_eq!(
            ActiveCollatorsForCurrentSession::<Test>::get(),
            current_session_active_collator_record
        );
        assert_eq!(ActiveCollators::<Test>::get(0), empty_vec);

        Pallet::<Test>::process_ended_session();

        assert_eq!(ActiveCollatorsForCurrentSession::<Test>::get(), empty_vec);
        assert_eq!(
            ActiveCollators::<Test>::get(0),
            current_session_active_collator_record
        );
    });
}

#[test]
fn processing_ended_session_correctly_cleans_outdated_collator_records() {
    ExtBuilder.build().execute_with(|| {
        roll_to(SESSION_BLOCK_LENGTH + 1);
        assert_eq!(<Test as Config>::CurrentSessionIndex::session_index(), 1);

        let current_session_active_collator_record: BoundedBTreeSet<AccountId, ConstU32<5>> =
            get_collator_set(vec![COLLATOR_1]);
        let empty_vec: BoundedBTreeSet<AccountId, ConstU32<5>> = BoundedBTreeSet::new();

        ActiveCollatorsForCurrentSession::<Test>::put(
            current_session_active_collator_record.clone(),
        );

        assert_eq!(
            ActiveCollatorsForCurrentSession::<Test>::get(),
            current_session_active_collator_record
        );
        assert_eq!(ActiveCollators::<Test>::get(0), empty_vec);

        Pallet::<Test>::process_ended_session();

        assert_eq!(ActiveCollatorsForCurrentSession::<Test>::get(), empty_vec);
        assert_eq!(
            ActiveCollators::<Test>::get(0),
            current_session_active_collator_record
        );

        roll_to(get_max_inactive_sessions() as u64 * SESSION_BLOCK_LENGTH + 1);
        assert_eq!(
            <Test as Config>::CurrentSessionIndex::session_index(),
            get_max_inactive_sessions()
        );

        Pallet::<Test>::process_ended_session();

        assert_eq!(ActiveCollatorsForCurrentSession::<Test>::get(), empty_vec);
        assert_eq!(
            ActiveCollators::<Test>::get(0),
            current_session_active_collator_record
        );
        assert_eq!(ActiveCollators::<Test>::get(1), empty_vec);

        roll_to((get_max_inactive_sessions() as u64 + 1) * SESSION_BLOCK_LENGTH + 1);
        assert_eq!(
            <Test as Config>::CurrentSessionIndex::session_index(),
            get_max_inactive_sessions() + 1
        );

        Pallet::<Test>::process_ended_session();

        assert_eq!(ActiveCollatorsForCurrentSession::<Test>::get(), empty_vec);
        assert_eq!(ActiveCollators::<Test>::get(0), empty_vec);
    });
}

#[test]
fn active_collators_noting_for_current_session_works() {
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
