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
    crate::{mock::*, Error, Event},
    frame_support::{assert_noop, assert_ok, BoundedVec},
    sp_runtime::DispatchError,
    tp_container_chain_genesis_data::ContainerChainGenesisData,
    tp_traits::ParaId,
};

const ALICE: u64 = 1;

#[test]
fn register_para_id_42() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));
        // Assert that the correct event was deposited
        System::assert_last_event(Event::ParaIdRegistered { para_id: 42.into() }.into());
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
        ));
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![42u32.into()]).unwrap())]
        );
        // Assert that the correct event was deposited
        System::assert_last_event(Event::ParaIdValidForCollating { para_id: 42.into() }.into());

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
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));
        assert_noop!(
            ParaRegistrar::register(
                RuntimeOrigin::signed(ALICE),
                42.into(),
                empty_genesis_data()
            ),
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
            ParaRegistrar::register(RuntimeOrigin::signed(ALICE), 42.into(), genesis_data,),
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
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
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
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
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
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
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
            RuntimeOrigin::signed(ALICE),
            42.into(),
            genesis_data.clone(),
        ));
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
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
            ParaRegistrar::register(RuntimeOrigin::root(), 42.into(), empty_genesis_data()),
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
fn mark_valid_for_collating_bad_origin() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_noop!(
            ParaRegistrar::mark_valid_for_collating(RuntimeOrigin::signed(1), 42.into()),
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
#[should_panic = "genesis data for para_id 2 is too large: 5000024 bytes"]
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

#[test]
fn register_without_mark_valid_for_collating() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));
        // Assert that the correct event was deposited
        System::assert_last_event(Event::ParaIdRegistered { para_id: 42.into() }.into());
        assert_eq!(ParaRegistrar::pending_registered_para_ids(), vec![]);

        // Assert after two sessions registered para ids are still empty
        ParaRegistrar::initializer_on_new_session(&2);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![]);
        assert_eq!(ParaRegistrar::pending_registered_para_ids(), vec![]);
    });
}

#[test]
fn mark_valid_for_collating_twice() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
        ));
        assert_noop!(
            ParaRegistrar::mark_valid_for_collating(RuntimeOrigin::root(), 42.into(),),
            Error::<Test>::ParaIdNotInPendingVerification
        );
    });
}

#[test]
fn mark_valid_for_collating_invalid_para_id() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_noop!(
            ParaRegistrar::mark_valid_for_collating(RuntimeOrigin::root(), 1.into(),),
            Error::<Test>::ParaIdNotInPendingVerification
        );
    });
}

#[test]
fn mark_valid_for_collating_already_valid_para_id() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));
        // Assert that the correct event was deposited
        System::assert_last_event(Event::ParaIdRegistered { para_id: 42.into() }.into());
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
        ));

        ParaRegistrar::initializer_on_new_session(&2);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![42.into()]);
        assert_eq!(ParaRegistrar::pending_registered_para_ids(), vec![]);
        assert_noop!(
            ParaRegistrar::mark_valid_for_collating(RuntimeOrigin::root(), 42.into(),),
            Error::<Test>::ParaIdNotInPendingVerification
        );
    });
}

#[test]
fn deregister_returns_bond() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let bond = DepositAmount::get();
        let balance_before = Balances::free_balance(ALICE);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
        ));
        assert_eq!(Balances::free_balance(ALICE), balance_before - bond);

        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into(),));

        assert_eq!(Balances::free_balance(ALICE), balance_before);
    });
}

#[test]
fn can_deregister_before_valid_for_collating() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));

        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into(),));
    });
}
