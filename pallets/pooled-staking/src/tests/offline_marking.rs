use {
    super::*,
    crate::Pallet,
    sp_runtime::DispatchError::BadOrigin;
    frame_support::{assert_noop, assert_ok},
};

#[test]
fn enabling_and_disabling_offline_marking_works() {
    ExtBuilder::default().build().execute_with(|| {
        assert_eq!(Pallet::<Runtime>::is_marking_offline_enabled(), false);
        
        assert_ok!(Pallet::<Runtime>::enable_offline_marking(RuntimeOrigin::root(), true));
        assert_eq!(Pallet::<Runtime>::is_marking_offline_enabled(), true);

        assert_ok!(Pallet::<Runtime>::enable_offline_marking(RuntimeOrigin::root(), false));
        assert_eq!(Pallet::<Runtime>::is_marking_offline_enabled(), false);
    });
}

#[test]
fn enabling_and_disabling_offline_marking_fails_for_non_root() {
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(
            Pallet::<Runtime>::enable_offline_marking(
                RuntimeOrigin::signed(ACCOUNT_CANDIDATE_1),
                true
            ),
            BadOrigin
        );
    });
}




