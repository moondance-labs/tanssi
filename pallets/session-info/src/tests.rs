use {crate::mock::*, frame_support::assert_ok};

#[test]
fn get_intiial_session_duration() {
    new_test_ext().execute_with(|| assert_eq!(SessionInfo::session_duration(), 5u64))
}

#[test]
fn change_initial_session_duration() {
    new_test_ext().execute_with(|| {
        assert_ok!(SessionInfo::set_new_session_duration(
            RuntimeOrigin::root(),
            10u64
        ));
        assert_eq!(SessionInfo::session_duration(), 10u64)
    })
}
