use {
    crate::{
        mock::*, ActiveCollators, Config, EnableInactivityTracking, NodeActivityTrackingHelper,
        Pallet,
    },
    frame_support::{assert_noop, assert_ok, pallet_prelude::Get},
    sp_runtime::{BoundedVec, DispatchError::BadOrigin},
    tp_traits::GetSessionIndex,
};

#[test]
fn enabling_and_disabling_inactivty_tracking_works() {
    ExtBuilder::default().build().execute_with(|| {
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
fn enabling_and_disabling_inactivty_tracking_fails_for_non_root() {
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(
            Pallet::<Test>::set_inactivity_tracking_status(RuntimeOrigin::signed(COLLATOR_1), true),
            BadOrigin
        );
    });
}

#[test]
fn inactivity_tracking_handler_works() {
    ExtBuilder::default().build().execute_with(|| {
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

        roll_to(max_inactive_sessions as u64 * SESSION_BLOCK_LENGTH);
        assert_eq!(
            <Test as Config>::CurrentSessionIndex::session_index(),
            max_inactive_sessions
        );

        for session_id in 0..max_inactive_sessions {
            let active_collators = BoundedVec::truncate_from(vec![COLLATOR_1]);
            ActiveCollators::<Test>::insert(session_id, active_collators.clone());
            assert_eq!(ActiveCollators::<Test>::get(session_id), active_collators);
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
