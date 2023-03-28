use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, BoundedVec};
use sp_runtime::DispatchError;

#[test]
fn register_para_id_42() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(ParaRegistrar::register(RuntimeOrigin::root(), 42));
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![42u32]).unwrap())]
        );
        // Assert that the correct event was deposited
        System::assert_last_event(Event::ParaIdRegistered { para_id: 42 }.into());

        // Assert after two sessions it goes to the non-pending
        ParaRegistrar::initializer_on_new_session(&2);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![42]);
        assert_eq!(ParaRegistrar::pending_registered_para_ids(), vec![]);
    });
}

#[test]
fn register_para_id_42_twice() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(ParaRegistrar::register(RuntimeOrigin::root(), 42));
        assert_noop!(
            ParaRegistrar::register(RuntimeOrigin::root(), 42),
            Error::<Test>::ParaIdAlreadyRegistered
        );
    });
}

#[test]
fn deregister_para_id_from_empty_list() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_noop!(
            ParaRegistrar::deregister(RuntimeOrigin::root(), 42),
            Error::<Test>::ParaIdNotRegistered
        );
    });
}

#[test]
fn deregister_para_id_42() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(ParaRegistrar::register(RuntimeOrigin::root(), 42));
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![42u32]).unwrap())]
        );

        // Assert after two sessions it goes to the non-pending
        ParaRegistrar::initializer_on_new_session(&2);

        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42));
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![]).unwrap())]
        );

        // Assert that the correct event was deposited
        System::assert_last_event(Event::ParaIdDeregistered { para_id: 42 }.into());
    });
}

#[test]
fn deregister_para_id_42_after_session_changes() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(ParaRegistrar::register(RuntimeOrigin::root(), 42));
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![42u32]).unwrap())]
        );

        ParaRegistrar::initializer_on_new_session(&2);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![42]);

        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42));
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![]).unwrap())]
        );

        // Assert that the correct event was deposited
        System::assert_last_event(Event::ParaIdDeregistered { para_id: 42 }.into());
    });
}

#[test]
fn deregister_para_id_42_twice() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(ParaRegistrar::register(RuntimeOrigin::root(), 42));
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![42u32]).unwrap())]
        );
        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42));
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![]).unwrap())]
        );
        assert_noop!(
            ParaRegistrar::deregister(RuntimeOrigin::root(), 42),
            Error::<Test>::ParaIdNotRegistered
        );
    });
}

#[test]
fn register_para_id_bad_origin() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_noop!(
            ParaRegistrar::register(RuntimeOrigin::signed(1), 42),
            DispatchError::BadOrigin
        );
    });
}

#[test]
fn deregister_para_id_bad_origin() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_noop!(
            ParaRegistrar::deregister(RuntimeOrigin::signed(1), 42),
            DispatchError::BadOrigin
        );
    });
}

#[test]
fn genesis_loads_para_ids() {
    new_test_ext_with_genesis(vec![1, 2, 3, 4]).execute_with(|| {
        System::set_block_number(1);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![1, 2, 3, 4]);
    });
}

#[test]
fn genesis_sorts_para_ids() {
    new_test_ext_with_genesis(vec![4, 2, 3, 1]).execute_with(|| {
        System::set_block_number(1);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![1, 2, 3, 4]);
    });
}

#[test]
#[should_panic = "Duplicate para_id: 2"]
fn genesis_error_on_duplicate() {
    new_test_ext_with_genesis(vec![2, 3, 4, 2]).execute_with(|| {
        System::set_block_number(1);
    });
}
