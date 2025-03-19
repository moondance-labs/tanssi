use {
    crate::{mock::*, EnableInactivityTracking, Pallet},
    frame_support::{assert_noop, assert_ok},
    sp_runtime::DispatchError::BadOrigin,
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
            Pallet::<Test>::set_inactivity_tracking_status(RuntimeOrigin::signed(ACCOUNT_1), true),
            BadOrigin
        );
    });
}
