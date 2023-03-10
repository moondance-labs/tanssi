use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::DispatchError;

#[test]
fn register_para_id_42() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(ParaRegistrar::register(RuntimeOrigin::root(), 42));
        assert_eq!(
            ParaRegistrar::registered_para_ids().unwrap_or_default(),
            vec![42]
        );
        // Assert that the correct event was deposited
        System::assert_last_event(Event::ParaIdRegistered { para_id: 42 }.into());
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
            ParaRegistrar::registered_para_ids().unwrap_or_default(),
            vec![42]
        );
        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42));
        assert_eq!(
            ParaRegistrar::registered_para_ids().unwrap_or_default(),
            vec![]
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
            ParaRegistrar::registered_para_ids().unwrap_or_default(),
            vec![42]
        );
        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42));
        assert_eq!(
            ParaRegistrar::registered_para_ids().unwrap_or_default(),
            vec![]
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
        assert_eq!(
            ParaRegistrar::registered_para_ids().unwrap_or_default(),
            vec![1, 2, 3, 4]
        );
    });
}

#[test]
fn genesis_sorts_para_ids() {
    new_test_ext_with_genesis(vec![4, 2, 3, 1]).execute_with(|| {
        System::set_block_number(1);
        assert_eq!(
            ParaRegistrar::registered_para_ids().unwrap_or_default(),
            vec![1, 2, 3, 4]
        );
    });
}

#[test]
#[should_panic = "Duplicate para_id: 2"]
fn genesis_error_on_duplicate() {
    new_test_ext_with_genesis(vec![2, 3, 4, 2]).execute_with(|| {
        System::set_block_number(1);
    });
}
