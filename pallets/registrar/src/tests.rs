use tp_traits::ParaId;

use {
    crate::{mock::*, ContainerChainGenesisData, Error, Event},
    frame_support::{assert_noop, assert_ok, BoundedVec},
    sp_runtime::DispatchError,
};

#[test]
fn register_para_id_42() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::root(),
            42.into(),
            empty_genesis_data()
        ));
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![42u32.into()]).unwrap())]
        );
        // Assert that the correct event was deposited
        System::assert_last_event(Event::ParaIdRegistered { para_id: 42.into() }.into());

        // Assert after two sessions it goes to the non-pending
        ParaRegistrar::initializer_on_new_session(&2);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![42.into()]);
        assert_eq!(ParaRegistrar::pending_registered_para_ids(), vec![]);
    });
}

#[test]
fn register_para_id_42_twice() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::root(),
            42.into(),
            empty_genesis_data()
        ));
        assert_noop!(
            ParaRegistrar::register(RuntimeOrigin::root(), 42.into(), empty_genesis_data()),
            Error::<Test>::ParaIdAlreadyRegistered
        );
    });
}

#[test]
fn register_para_id_42_genesis_data_size_too_big() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let genesis_data = ContainerChainGenesisData {
            storage: vec![(vec![], vec![0; 5_000_000]).into()],
            name: Default::default(),
            id: Default::default(),
            fork_id: Default::default(),
            extensions: Default::default(),
            properties: Default::default(),
        };
        assert_noop!(
            ParaRegistrar::register(RuntimeOrigin::root(), 42.into(), genesis_data,),
            Error::<Test>::GenesisDataTooBig,
        );
    });
}

#[test]
fn deregister_para_id_from_empty_list() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_noop!(
            ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into()),
            Error::<Test>::ParaIdNotRegistered
        );
    });
}

#[test]
fn deregister_para_id_42() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::root(),
            42.into(),
            empty_genesis_data()
        ));
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![42u32.into()]).unwrap())]
        );

        // Assert after two sessions it goes to the non-pending
        ParaRegistrar::initializer_on_new_session(&2);

        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into()));
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![]).unwrap())]
        );

        // Assert that the correct event was deposited
        System::assert_last_event(Event::ParaIdDeregistered { para_id: 42.into() }.into());
    });
}

#[test]
fn deregister_para_id_42_after_session_changes() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::root(),
            42.into(),
            empty_genesis_data()
        ));
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![42u32.into()]).unwrap())]
        );

        ParaRegistrar::initializer_on_new_session(&2);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![42.into()]);

        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into()));
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![]).unwrap())]
        );

        // Assert that the correct event was deposited
        System::assert_last_event(Event::ParaIdDeregistered { para_id: 42.into() }.into());
    });
}

#[test]
fn deregister_para_id_42_twice() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::root(),
            42.into(),
            empty_genesis_data()
        ));
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![42u32.into()]).unwrap())]
        );
        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into()));
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![]).unwrap())]
        );
        assert_noop!(
            ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into()),
            Error::<Test>::ParaIdNotRegistered
        );
    });
}

#[test]
fn deregister_para_id_removes_genesis_data() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let genesis_data = ContainerChainGenesisData {
            storage: vec![(b"key".to_vec(), b"value".to_vec()).into()],
            name: Default::default(),
            id: Default::default(),
            fork_id: Default::default(),
            extensions: Default::default(),
            properties: Default::default(),
        };
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::root(),
            42.into(),
            genesis_data.clone(),
        ));
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![42u32.into()]).unwrap())]
        );
        assert_eq!(
            ParaRegistrar::para_genesis_data(ParaId::from(42)),
            Some(genesis_data),
        );

        // Assert after two sessions it goes to the non-pending
        ParaRegistrar::initializer_on_new_session(&2);

        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into()));
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![]).unwrap())]
        );

        // Assert that the correct event was deposited
        System::assert_last_event(Event::ParaIdDeregistered { para_id: 42.into() }.into());

        // Genesis data has been deleted
        // TODO: it should probably not be deleted until the next session change when the
        // para id is actually deregistered
        assert_eq!(ParaRegistrar::para_genesis_data(ParaId::from(42)), None,);
    });
}

#[test]
fn register_para_id_bad_origin() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_noop!(
            ParaRegistrar::register(RuntimeOrigin::signed(1), 42.into(), empty_genesis_data()),
            DispatchError::BadOrigin
        );
    });
}

#[test]
fn deregister_para_id_bad_origin() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_noop!(
            ParaRegistrar::deregister(RuntimeOrigin::signed(1), 42.into()),
            DispatchError::BadOrigin
        );
    });
}

#[test]
fn genesis_loads_para_ids() {
    new_test_ext_with_genesis(vec![
        (1.into(), empty_genesis_data()),
        (2.into(), empty_genesis_data()),
        (3.into(), empty_genesis_data()),
        (4.into(), empty_genesis_data()),
    ])
    .execute_with(|| {
        System::set_block_number(1);
        assert_eq!(
            ParaRegistrar::registered_para_ids(),
            vec![1.into(), 2.into(), 3.into(), 4.into()]
        );
    });
}

#[test]
fn genesis_sorts_para_ids() {
    new_test_ext_with_genesis(vec![
        (4.into(), empty_genesis_data()),
        (2.into(), empty_genesis_data()),
        (3.into(), empty_genesis_data()),
        (1.into(), empty_genesis_data()),
    ])
    .execute_with(|| {
        System::set_block_number(1);
        assert_eq!(
            ParaRegistrar::registered_para_ids(),
            vec![1.into(), 2.into(), 3.into(), 4.into()]
        );
    });
}

#[test]
#[should_panic = "Duplicate para_id: 2"]
fn genesis_error_on_duplicate() {
    new_test_ext_with_genesis(vec![
        (2.into(), empty_genesis_data()),
        (3.into(), empty_genesis_data()),
        (4.into(), empty_genesis_data()),
        (2.into(), empty_genesis_data()),
    ])
    .execute_with(|| {
        System::set_block_number(1);
    });
}

#[test]
#[should_panic = "genesis data for para_id 2 is too large: 5000023 bytes"]
fn genesis_error_genesis_data_size_too_big() {
    let genesis_data = ContainerChainGenesisData {
        storage: vec![(vec![], vec![0; 5_000_000]).into()],
        name: Default::default(),
        id: Default::default(),
        fork_id: Default::default(),
        extensions: Default::default(),
        properties: Default::default(),
    };
    new_test_ext_with_genesis(vec![(2.into(), genesis_data)]).execute_with(|| {
        System::set_block_number(1);
    });
}
