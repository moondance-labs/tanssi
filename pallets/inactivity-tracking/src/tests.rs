use crate::ActiveCollatorsForCurrentSession;
use sp_core::ConstU32;
use {
    crate::{
        mock::*, ActiveCollators, AuthorNotingHook, Config, EnableInactivityTracking,
        NodeActivityTrackingHelper, Pallet,
    },
    frame_support::{assert_noop, assert_ok, pallet_prelude::Get},
    sp_runtime::{BoundedVec, DispatchError::BadOrigin},
    tp_traits::{AuthorNotingInfo, GetSessionIndex},
};

fn get_active_collators(block: u32) -> AuthorNotingInfo<AccountId> {
    AuthorNotingInfo {
        block_number: block,
        author: COLLATOR_1,
        para_id: CONTAINER_CHAIN_ID,
    }
}

#[test]
fn enabling_and_disabling_inactivity_tracking_works() {
    ExtBuilder.build().execute_with(|| {
        assert_eq!(EnableInactivityTracking::<Test>::get(), false);

        assert_ok!(Pallet::<Test>::set_inactivity_tracking_status(
            RuntimeOrigin::root(),
            true
        ));
        assert_eq!(EnableInactivityTracking::<Test>::get(), true);

        assert_ok!(Pallet::<Test>::set_inactivity_tracking_status(
            RuntimeOrigin::root(),
            false
        ));
        assert_eq!(EnableInactivityTracking::<Test>::get(), false);
    });
}

#[test]
fn enabling_and_disabling_inactivity_tracking_fails_for_non_root() {
    ExtBuilder.build().execute_with(|| {
        assert_noop!(
            Pallet::<Test>::set_inactivity_tracking_status(RuntimeOrigin::signed(COLLATOR_1), true),
            BadOrigin
        );
    });
}

#[test]
fn inactivity_tracking_handler_with_enabled_or_disabled_tracking_works() {
    ExtBuilder.build().execute_with(|| {
        assert_ok!(Pallet::<Test>::set_inactivity_tracking_status(
            RuntimeOrigin::root(),
            true
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

        let max_inactive_sessions: u32 = <Test as Config>::MaxInactiveSessions::get();

        roll_to(max_inactive_sessions as u64 * SESSION_BLOCK_LENGTH + 1);
        assert_eq!(
            <Test as Config>::CurrentSessionIndex::session_index(),
            max_inactive_sessions
        );

        let active_collator = BoundedVec::truncate_from(vec![COLLATOR_1]);
        for session_id in 0..max_inactive_sessions {
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
        assert_ok!(Pallet::<Test>::set_inactivity_tracking_status(
            RuntimeOrigin::root(),
            true
        ));
        let max_inactive_sessions: u32 = <Test as Config>::MaxInactiveSessions::get();

        roll_to(max_inactive_sessions as u64 * SESSION_BLOCK_LENGTH + 1);

        let active_collator_1 = BoundedVec::truncate_from(vec![COLLATOR_1]);
        let active_collator_2 = BoundedVec::truncate_from(vec![COLLATOR_2]);
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
        assert_ok!(Pallet::<Test>::set_inactivity_tracking_status(
            RuntimeOrigin::root(),
            true
        ));
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
        assert_ok!(Pallet::<Test>::set_inactivity_tracking_status(
            RuntimeOrigin::root(),
            true
        ));

        roll_to(SESSION_BLOCK_LENGTH + 1);

        let current_session_active_collator_record: BoundedVec<AccountId, ConstU32<5>> =
            BoundedVec::truncate_from(vec![COLLATOR_1]);
        let empty_vec: BoundedVec<AccountId, ConstU32<5>> = BoundedVec::new();

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
        assert_ok!(Pallet::<Test>::set_inactivity_tracking_status(
            RuntimeOrigin::root(),
            true
        ));

        roll_to(SESSION_BLOCK_LENGTH + 1);
        assert_eq!(<Test as Config>::CurrentSessionIndex::session_index(), 1);

        let current_session_active_collator_record: BoundedVec<AccountId, ConstU32<5>> =
            BoundedVec::truncate_from(vec![COLLATOR_1]);
        let empty_vec: BoundedVec<AccountId, ConstU32<5>> = BoundedVec::new();

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

        let max_inactive_sessions: u32 = <Test as Config>::MaxInactiveSessions::get();
        roll_to(max_inactive_sessions as u64 * SESSION_BLOCK_LENGTH + 1);
        assert_eq!(
            <Test as Config>::CurrentSessionIndex::session_index(),
            max_inactive_sessions
        );

        Pallet::<Test>::process_ended_session();

        assert_eq!(ActiveCollatorsForCurrentSession::<Test>::get(), empty_vec);
        assert_eq!(
            ActiveCollators::<Test>::get(0),
            current_session_active_collator_record
        );
        assert_eq!(ActiveCollators::<Test>::get(1), empty_vec);

        roll_to((max_inactive_sessions as u64 + 1) * SESSION_BLOCK_LENGTH + 1);
        assert_eq!(
            <Test as Config>::CurrentSessionIndex::session_index(),
            max_inactive_sessions + 1
        );

        Pallet::<Test>::process_ended_session();

        assert_eq!(ActiveCollatorsForCurrentSession::<Test>::get(), empty_vec);
        assert_eq!(ActiveCollators::<Test>::get(0), empty_vec);
    });
}

#[test]
fn active_collators_noting_for_current_session_works() {
    ExtBuilder.build().execute_with(|| {
        let current_session_active_collator_record: BoundedVec<AccountId, ConstU32<5>> =
            BoundedVec::truncate_from(vec![COLLATOR_1]);
        assert_ok!(Pallet::<Test>::set_inactivity_tracking_status(
            RuntimeOrigin::root(),
            true
        ));
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
