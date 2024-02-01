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
    frame_support::{assert_noop, assert_ok, dispatch::GetDispatchInfo, BoundedVec},
    parity_scale_codec::Encode,
    sp_core::Get,
    sp_runtime::DispatchError,
    tp_container_chain_genesis_data::ContainerChainGenesisData,
    tp_traits::{ParaId, SlotFrequency},
};

const ALICE: u64 = 1;
//const BOB: u64 = 2;

#[test]
fn register_para_id_42() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
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
        run_to_session(2);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![42.into()]);
        assert_eq!(ParaRegistrar::pending_registered_para_ids(), vec![]);
    });
}

#[test]
fn register_para_id_42_twice() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
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
        run_to_block(1);
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
        run_to_block(1);
        assert_noop!(
            ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into()),
            Error::<Test>::ParaIdNotRegistered
        );
    });
}

#[test]
fn deregister_para_id_42_after_0_sessions() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
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
        // Assert that the correct event was deposited
        System::assert_last_event(Event::ParaIdDeregistered { para_id: 42.into() }.into());
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![]).unwrap())]
        );
        // This para id will never be in registered so we do not need to keep the genesis data,
        // but we do anyway, and the genesis data is deleted after 2 sessions
        assert!(ParaRegistrar::para_genesis_data(ParaId::from(42)).is_some());

        run_to_session(1);
        assert!(ParaRegistrar::para_genesis_data(ParaId::from(42)).is_some());
        // Assert after two sessions genesis data gets deleted
        run_to_session(2);
        assert!(ParaRegistrar::para_genesis_data(ParaId::from(42)).is_none());
    });
}

#[test]
fn deregister_para_id_42_after_1_sessions() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
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
        assert!(ParaRegistrar::para_genesis_data(ParaId::from(42)).is_some());

        run_to_session(1);
        // Deregister while its pending
        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into()));
        // Assert that the correct event was deposited
        System::assert_last_event(Event::ParaIdDeregistered { para_id: 42.into() }.into());
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![
                (2u32, BoundedVec::try_from(vec![42u32.into()]).unwrap()),
                (3u32, BoundedVec::try_from(vec![]).unwrap())
            ]
        );
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![]);
        assert!(ParaRegistrar::para_genesis_data(ParaId::from(42)).is_some());

        run_to_session(2);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![42.into()]);
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(3u32, BoundedVec::try_from(vec![]).unwrap())]
        );
        assert!(ParaRegistrar::para_genesis_data(ParaId::from(42)).is_some());

        run_to_session(3);
        assert_eq!(ParaRegistrar::pending_registered_para_ids(), vec![]);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![]);
        assert!(ParaRegistrar::para_genesis_data(ParaId::from(42)).is_none());
    });
}

#[test]
fn deregister_para_id_42_after_2_sessions() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
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
        run_to_session(2);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![42.into()]);
        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into()));
        // Assert that the correct event was deposited
        System::assert_last_event(Event::ParaIdDeregistered { para_id: 42.into() }.into());
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(4u32, BoundedVec::try_from(vec![]).unwrap())]
        );

        // Assert that the correct event was deposited
        System::assert_last_event(Event::ParaIdDeregistered { para_id: 42.into() }.into());
        assert!(ParaRegistrar::para_genesis_data(ParaId::from(42)).is_some());

        run_to_session(3);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![42.into()]);
        assert!(ParaRegistrar::para_genesis_data(ParaId::from(42)).is_some());

        run_to_session(4);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![]);
        assert!(ParaRegistrar::para_genesis_data(ParaId::from(42)).is_none());
    });
}

#[test]
fn deregister_para_id_42_twice() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
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
        run_to_block(1);
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
            ParaRegistrar::para_genesis_data(ParaId::from(42)).as_ref(),
            Some(&genesis_data),
        );

        // Assert after two sessions it goes to the non-pending
        run_to_session(2);

        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into()));
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(4u32, BoundedVec::try_from(vec![]).unwrap())]
        );

        // Assert that the correct event was deposited
        System::assert_last_event(Event::ParaIdDeregistered { para_id: 42.into() }.into());

        // Genesis data has not been deleted yet, it will be deleted after 2 sessions
        assert_eq!(
            ParaRegistrar::para_genesis_data(ParaId::from(42)),
            Some(genesis_data),
        );
        run_to_session(4);
        assert_eq!(ParaRegistrar::para_genesis_data(ParaId::from(42)), None);
    });
}

#[test]
fn register_para_id_bad_origin() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_noop!(
            ParaRegistrar::register(RuntimeOrigin::root(), 42.into(), empty_genesis_data()),
            DispatchError::BadOrigin
        );
    });
}

#[test]
fn deregister_para_id_bad_origin() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_noop!(
            ParaRegistrar::deregister(RuntimeOrigin::signed(1), 42.into()),
            DispatchError::BadOrigin
        );
    });
}

#[test]
fn mark_valid_for_collating_bad_origin() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_noop!(
            ParaRegistrar::mark_valid_for_collating(RuntimeOrigin::signed(1), 42.into()),
            DispatchError::BadOrigin
        );
    });
}

#[test]
fn pause_para_id_42_works() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));

        // Enable the container-chain for the first time
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
        ));
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![42u32.into()]).unwrap())]
        );

        run_to_session(2);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![42.into()]);

        // Pause the container-chain
        assert_ok!(ParaRegistrar::pause_container_chain(
            RuntimeOrigin::root(),
            42.into(),
        ));

        // Assert that the ParaIdPaused event was emitted
        System::assert_last_event(Event::ParaIdPaused { para_id: 42.into() }.into());

        // Check genesis data was not removed
        assert!(ParaRegistrar::para_genesis_data(ParaId::from(42)).is_some());

        // Check the container chain was not selected for the next period
        run_to_session(4);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![]);
    });
}

#[test]
fn pause_para_id_42_twice_fails() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));

        // Enable the container-chain for collating
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
        ));
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![42u32.into()]).unwrap())]
        );

        run_to_session(2);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![42.into()]);

        // Pause the container-chain
        assert_ok!(ParaRegistrar::pause_container_chain(
            RuntimeOrigin::root(),
            42.into(),
        ));

        // Try to pause again
        assert_noop!(
            ParaRegistrar::pause_container_chain(RuntimeOrigin::root(), 42.into()),
            Error::<Test>::ParaIdAlreadyPaused
        );
    });
}

#[test]
fn pause_para_id_42_fails_not_registered() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        // Try to pause
        assert_noop!(
            ParaRegistrar::pause_container_chain(RuntimeOrigin::root(), 42.into()),
            Error::<Test>::ParaIdNotRegistered
        );
    });
}

#[test]
fn pause_container_chain_bad_origin() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_noop!(
            ParaRegistrar::pause_container_chain(RuntimeOrigin::signed(1), 42.into()),
            DispatchError::BadOrigin
        );
    });
}

#[test]
fn unpause_para_id_that_is_not_paused_fails() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));

        // Enable the container-chain for collating
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
        ));
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![42u32.into()]).unwrap())]
        );

        run_to_session(2);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![42.into()]);

        // Try to unpause
        assert_noop!(
            ParaRegistrar::unpause_container_chain(RuntimeOrigin::root(), 42.into()),
            Error::<Test>::ParaIdNotPaused
        );
    });
}

#[test]
fn unpause_para_id_42_twice_fails() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));

        // Enable the container-chain for collating
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
        ));
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![42u32.into()]).unwrap())]
        );

        run_to_session(2);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![42.into()]);

        // Pause the container-chain
        assert_ok!(ParaRegistrar::pause_container_chain(
            RuntimeOrigin::root(),
            42.into(),
        ));

        // Unpause
        assert_ok!(ParaRegistrar::unpause_container_chain(
            RuntimeOrigin::root(),
            42.into(),
        ),);

        // Unpause again fails
        assert_noop!(
            ParaRegistrar::unpause_container_chain(RuntimeOrigin::root(), 42.into()),
            Error::<Test>::ParaIdNotPaused
        );
    });
}

#[test]
fn unpause_para_id_42_fails_not_registered() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        // Try to pause
        assert_noop!(
            ParaRegistrar::unpause_container_chain(RuntimeOrigin::root(), 42.into()),
            Error::<Test>::ParaIdNotPaused
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
        run_to_block(1);
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
        run_to_block(1);
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
        run_to_block(1);
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
        run_to_block(1);
    });
}

#[test]
fn register_without_mark_valid_for_collating() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));
        // Assert that the correct event was deposited
        System::assert_last_event(Event::ParaIdRegistered { para_id: 42.into() }.into());
        assert_eq!(ParaRegistrar::pending_registered_para_ids(), vec![]);

        // Assert after two sessions registered para ids are still empty
        run_to_session(2);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![]);
        assert_eq!(ParaRegistrar::pending_registered_para_ids(), vec![]);
    });
}

#[test]
fn mark_valid_for_collating_twice() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
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
        run_to_block(1);
        assert_noop!(
            ParaRegistrar::mark_valid_for_collating(RuntimeOrigin::root(), 1.into(),),
            Error::<Test>::ParaIdNotInPendingVerification
        );
    });
}

#[test]
fn mark_valid_for_collating_already_valid_para_id() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
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

        run_to_session(2);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![42.into()]);
        assert_eq!(ParaRegistrar::pending_registered_para_ids(), vec![]);
        assert_noop!(
            ParaRegistrar::mark_valid_for_collating(RuntimeOrigin::root(), 42.into(),),
            Error::<Test>::ParaIdNotInPendingVerification
        );
    });
}

#[test]
fn mark_valid_for_collating_calls_registered_hook() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));
        assert_eq!(Mock::get().called_hooks, vec![]);
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
        ));
        assert_eq!(
            Mock::get().called_hooks,
            vec![HookCall::MarkedValid(42.into())]
        );
    });
}

#[test]
fn deregister_returns_bond_immediately_if_not_marked_as_valid() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let bond = DepositAmount::get();
        let balance_before = Balances::free_balance(ALICE);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));
        assert_eq!(Balances::free_balance(ALICE), balance_before - bond);

        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into(),));

        // Bond is returned immediately
        assert_eq!(Balances::free_balance(ALICE), balance_before);
    });
}

#[test]
fn deregister_returns_bond_after_2_sessions_if_marked_as_valid() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
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

        // Bond is returned after 2 sessions
        assert_eq!(Balances::free_balance(ALICE), balance_before - bond);
        run_to_session(2);
        assert_eq!(Balances::free_balance(ALICE), balance_before);
    });
}

#[test]
fn can_deregister_before_valid_for_collating() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));

        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into(),));
        System::assert_has_event(Event::ParaIdDeregistered { para_id: 42.into() }.into());
    });
}

#[test]
fn can_deregister_paused_para_id_after_0_sessions() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
        ));
        // Pause and deregister in the same block
        assert_ok!(ParaRegistrar::pause_container_chain(
            RuntimeOrigin::root(),
            42.into(),
        ));
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![]);
        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into()));
        // Assert that the correct event was deposited
        System::assert_last_event(Event::ParaIdDeregistered { para_id: 42.into() }.into());
        assert!(ParaRegistrar::para_genesis_data(ParaId::from(42)).is_some());

        run_to_session(1);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![]);
        assert!(ParaRegistrar::para_genesis_data(ParaId::from(42)).is_some());

        run_to_session(2);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![]);
        assert!(ParaRegistrar::para_genesis_data(ParaId::from(42)).is_none());
    });
}

#[test]
fn can_deregister_paused_para_id_after_1_sessions() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
        ));
        // Pause, wait 1 session, and deregister
        assert_ok!(ParaRegistrar::pause_container_chain(
            RuntimeOrigin::root(),
            42.into(),
        ));
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![]);

        run_to_session(1);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![]);
        assert_eq!(ParaRegistrar::paused(), vec![]);
        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into()));
        // Assert that the correct event was deposited
        System::assert_last_event(Event::ParaIdDeregistered { para_id: 42.into() }.into());
        assert!(ParaRegistrar::para_genesis_data(ParaId::from(42)).is_some());

        run_to_session(2);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![]);
        assert_eq!(ParaRegistrar::paused(), vec![42.into()]);
        assert!(ParaRegistrar::para_genesis_data(ParaId::from(42)).is_some());

        run_to_session(3);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![]);
        assert_eq!(ParaRegistrar::paused(), vec![]);
        assert!(ParaRegistrar::para_genesis_data(ParaId::from(42)).is_none());
    });
}

#[test]
fn can_deregister_paused_para_id_after_2_sessions() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
        ));
        assert_ok!(ParaRegistrar::pause_container_chain(
            RuntimeOrigin::root(),
            42.into(),
        ));
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![]);

        run_to_session(2);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![]);
        // Pause, wait 2 sessions, and deregister
        assert_eq!(ParaRegistrar::paused(), vec![42.into()]);
        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into()));
        // Assert that the correct event was deposited
        System::assert_last_event(Event::ParaIdDeregistered { para_id: 42.into() }.into());
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![]);
        assert_eq!(ParaRegistrar::paused(), vec![42.into()]);
        assert!(ParaRegistrar::para_genesis_data(ParaId::from(42)).is_some());

        run_to_session(3);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![]);
        assert_eq!(ParaRegistrar::paused(), vec![42.into()]);
        assert!(ParaRegistrar::para_genesis_data(ParaId::from(42)).is_some());

        run_to_session(4);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![]);
        assert_eq!(ParaRegistrar::paused(), vec![]);
        assert!(ParaRegistrar::para_genesis_data(ParaId::from(42)).is_none());
    });
}

#[test]
fn cannot_register_same_para_id_while_deregister_is_pending() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
        ));
        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into(),));
        assert_noop!(
            ParaRegistrar::register(
                RuntimeOrigin::signed(ALICE),
                42.into(),
                empty_genesis_data(),
            ),
            Error::<Test>::ParaIdAlreadyRegistered,
        );
        run_to_session(1);
        assert_noop!(
            ParaRegistrar::register(
                RuntimeOrigin::signed(ALICE),
                42.into(),
                empty_genesis_data(),
            ),
            Error::<Test>::ParaIdAlreadyRegistered,
        );
        run_to_session(2);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));
    });
}

#[test]
fn register_deregister_register_in_same_block() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));
        assert_eq!(
            ParaRegistrar::para_genesis_data(ParaId::from(42)).as_ref(),
            Some(&empty_genesis_data())
        );
        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into(),));
        assert_eq!(
            ParaRegistrar::para_genesis_data(ParaId::from(42)).as_ref(),
            None
        );
        let new_genesis_data = ContainerChainGenesisData {
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
            new_genesis_data.clone(),
        ));
        assert_eq!(
            ParaRegistrar::para_genesis_data(ParaId::from(42)).as_ref(),
            Some(&new_genesis_data)
        );
        run_to_session(2);
        assert_eq!(
            ParaRegistrar::para_genesis_data(ParaId::from(42)).as_ref(),
            Some(&new_genesis_data)
        );
    });
}

#[test]
fn deregister_2_container_chains_in_same_block() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            43.into(),
            empty_genesis_data()
        ));
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
        ));
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            43.into(),
        ));

        run_to_session(2);
        assert_eq!(
            ParaRegistrar::registered_para_ids(),
            vec![42.into(), 43.into()]
        );
        assert_eq!(
            ParaRegistrar::para_genesis_data(ParaId::from(42)).as_ref(),
            Some(&empty_genesis_data())
        );
        assert_eq!(
            ParaRegistrar::para_genesis_data(ParaId::from(43)).as_ref(),
            Some(&empty_genesis_data())
        );
        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into(),));
        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 43.into(),));
        assert_eq!(
            Mock::get().called_hooks,
            vec![
                HookCall::MarkedValid(42.into()),
                HookCall::MarkedValid(43.into()),
            ]
        );

        run_to_session(4);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![]);
        assert_eq!(
            ParaRegistrar::para_genesis_data(ParaId::from(42)).as_ref(),
            None
        );
        assert_eq!(
            ParaRegistrar::para_genesis_data(ParaId::from(43)).as_ref(),
            None
        );
        assert_eq!(
            Mock::get().called_hooks,
            vec![
                HookCall::MarkedValid(42.into()),
                HookCall::MarkedValid(43.into()),
                HookCall::Deregistered(42.into()),
                HookCall::Deregistered(43.into()),
            ]
        );
    });
}

#[test]
fn deregister_2_container_chains_in_consecutive_sessions() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            43.into(),
            empty_genesis_data()
        ));
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
        ));
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            43.into(),
        ));

        run_to_session(2);
        assert_eq!(
            ParaRegistrar::registered_para_ids(),
            vec![42.into(), 43.into()]
        );
        assert_eq!(
            ParaRegistrar::para_genesis_data(ParaId::from(42)).as_ref(),
            Some(&empty_genesis_data())
        );
        assert_eq!(
            ParaRegistrar::para_genesis_data(ParaId::from(43)).as_ref(),
            Some(&empty_genesis_data())
        );
        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into(),));

        run_to_session(3);
        assert_eq!(
            ParaRegistrar::registered_para_ids(),
            vec![42.into(), 43.into()]
        );
        assert_eq!(
            ParaRegistrar::para_genesis_data(ParaId::from(42)).as_ref(),
            Some(&empty_genesis_data())
        );
        assert_eq!(
            ParaRegistrar::para_genesis_data(ParaId::from(43)).as_ref(),
            Some(&empty_genesis_data())
        );
        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 43.into(),));
        assert_eq!(
            Mock::get().called_hooks,
            vec![
                HookCall::MarkedValid(42.into()),
                HookCall::MarkedValid(43.into()),
            ]
        );

        run_to_session(4);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![43.into()]);
        assert_eq!(
            ParaRegistrar::para_genesis_data(ParaId::from(42)).as_ref(),
            None
        );
        assert_eq!(
            ParaRegistrar::para_genesis_data(ParaId::from(43)).as_ref(),
            Some(&empty_genesis_data())
        );
        assert_eq!(
            Mock::get().called_hooks,
            vec![
                HookCall::MarkedValid(42.into()),
                HookCall::MarkedValid(43.into()),
                HookCall::Deregistered(42.into()),
            ]
        );

        run_to_session(5);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![]);
        assert_eq!(
            ParaRegistrar::para_genesis_data(ParaId::from(42)).as_ref(),
            None
        );
        assert_eq!(
            ParaRegistrar::para_genesis_data(ParaId::from(43)).as_ref(),
            None
        );
        assert_eq!(
            Mock::get().called_hooks,
            vec![
                HookCall::MarkedValid(42.into()),
                HookCall::MarkedValid(43.into()),
                HookCall::Deregistered(42.into()),
                HookCall::Deregistered(43.into()),
            ]
        );
    });
}

#[test]
fn deposit_removed_on_deregister_if_not_marked_as_valid() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));
        assert!(ParaRegistrar::registrar_deposit(ParaId::from(42)).is_some());
        assert!(ParaRegistrar::para_genesis_data(ParaId::from(42)).is_some());

        // Deregister a para id that was not marked as valid_for_collating, deposit and genesis data are
        // removed immediately because no collators are assigned to this chain.
        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into()));
        assert!(ParaRegistrar::registrar_deposit(ParaId::from(42)).is_none());
        assert!(ParaRegistrar::para_genesis_data(ParaId::from(42)).is_none());
    });
}

#[test]
fn deposit_removed_after_2_sessions_if_marked_as_valid() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));
        assert!(ParaRegistrar::registrar_deposit(ParaId::from(42)).is_some());
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
        ));

        // Deregister a para id that has been marked as valid_for_collating, deposit and genesis data
        // will be stored until all collators are unassigned, after 2 sessions.
        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into()));
        assert!(ParaRegistrar::registrar_deposit(ParaId::from(42)).is_some());
        assert!(ParaRegistrar::para_genesis_data(ParaId::from(42)).is_some());

        // Deposit removed after 2 sessions
        run_to_session(2);
        assert!(ParaRegistrar::registrar_deposit(ParaId::from(42)).is_none());
        assert!(ParaRegistrar::para_genesis_data(ParaId::from(42)).is_none());
    });
}

#[test]
fn parathread_change_params_after_two_sessions() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(ParaRegistrar::register_parathread(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            SlotFrequency { min: 1, max: 1 },
            empty_genesis_data()
        ));
        assert!(ParaRegistrar::registrar_deposit(ParaId::from(42)).is_some());
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
        ));
        assert_ok!(ParaRegistrar::set_parathread_params(
            RuntimeOrigin::root(),
            ParaId::from(42),
            SlotFrequency { min: 2, max: 2 }
        ));
        // Params are not updated immediately
        assert_eq!(
            ParaRegistrar::parathread_params(ParaId::from(42)).map(|x| x.slot_frequency),
            Some(SlotFrequency { min: 1, max: 1 })
        );

        // Params are updated after 2 sessions
        run_to_session(2);
        assert_eq!(
            ParaRegistrar::parathread_params(ParaId::from(42)).map(|x| x.slot_frequency),
            Some(SlotFrequency { min: 2, max: 2 })
        );
    });
}

#[test]
fn parathread_params_cannot_be_set_for_parachains() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));
        assert!(ParaRegistrar::registrar_deposit(ParaId::from(42)).is_some());
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
        ));
        assert_noop!(
            ParaRegistrar::set_parathread_params(
                RuntimeOrigin::root(),
                ParaId::from(42),
                SlotFrequency { min: 2, max: 2 }
            ),
            Error::<Test>::NotAParathread
        );
    });
}

#[test]
fn parathread_register_change_params_deregister() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(ParaRegistrar::register_parathread(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            SlotFrequency { min: 1, max: 1 },
            empty_genesis_data()
        ));
        assert!(ParaRegistrar::registrar_deposit(ParaId::from(42)).is_some());
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
        ));
        assert_ok!(ParaRegistrar::set_parathread_params(
            RuntimeOrigin::root(),
            ParaId::from(42),
            SlotFrequency { min: 2, max: 2 }
        ));

        // Deregister parathread while parathread params are pending
        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into()));
        assert!(ParaRegistrar::para_genesis_data(ParaId::from(42)).is_some());
        assert_eq!(
            ParaRegistrar::parathread_params(ParaId::from(42)).map(|x| x.slot_frequency),
            Some(SlotFrequency { min: 1, max: 1 })
        );

        // Params removed after 2 sessions
        run_to_session(2);
        assert!(ParaRegistrar::para_genesis_data(ParaId::from(42)).is_none());
        assert!(ParaRegistrar::parathread_params(ParaId::from(42)).is_none());
    });
}

#[test]
fn parathread_register_deregister_change_params() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(ParaRegistrar::register_parathread(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            SlotFrequency { min: 1, max: 1 },
            empty_genesis_data()
        ));
        assert!(ParaRegistrar::registrar_deposit(ParaId::from(42)).is_some());
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
        ));

        // Deregister parathread while parathread params are pending
        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into()));
        assert!(ParaRegistrar::para_genesis_data(ParaId::from(42)).is_some());
        assert!(ParaRegistrar::parathread_params(ParaId::from(42)).is_some());

        run_to_session(1);
        assert_ok!(ParaRegistrar::set_parathread_params(
            RuntimeOrigin::root(),
            ParaId::from(42),
            SlotFrequency { min: 2, max: 2 }
        ));

        // Params removed after 2 sessions
        run_to_session(2);
        assert!(ParaRegistrar::para_genesis_data(ParaId::from(42)).is_none());
        assert!(ParaRegistrar::parathread_params(ParaId::from(42)).is_none());

        // Params not updated after 3 sessions
        run_to_session(3);
        assert!(ParaRegistrar::parathread_params(ParaId::from(42)).is_none());
    });
}

#[test]
fn weights_assigned_to_extrinsics_are_correct() {
    new_test_ext().execute_with(|| {
        assert_eq!(
            crate::Call::<Test>::register {
                para_id: 42.into(),
                genesis_data: empty_genesis_data()
            }
            .get_dispatch_info()
            .weight,
            <() as crate::weights::WeightInfo>::register(
                empty_genesis_data().encoded_size() as u32,
                <Test as crate::Config>::MaxLengthParaIds::get(),
                0
            )
        );

        assert_eq!(
            crate::Call::<Test>::deregister { para_id: 42.into() }
                .get_dispatch_info()
                .weight,
            <() as crate::weights::WeightInfo>::deregister_immediate(
                <Test as crate::Config>::MaxGenesisDataSize::get(),
                <Test as crate::Config>::MaxLengthParaIds::get()
            )
            .max(<() as crate::weights::WeightInfo>::deregister_scheduled(
                <Test as crate::Config>::MaxGenesisDataSize::get(),
                <Test as crate::Config>::MaxLengthParaIds::get()
            ))
        );

        assert_eq!(
            crate::Call::<Test>::mark_valid_for_collating { para_id: 42.into() }
                .get_dispatch_info()
                .weight,
            <() as crate::weights::WeightInfo>::mark_valid_for_collating(
                <Test as crate::Config>::MaxLengthParaIds::get()
            )
        );

        assert_eq!(
            crate::Call::<Test>::pause_container_chain { para_id: 42.into() }
                .get_dispatch_info()
                .weight,
            <() as crate::weights::WeightInfo>::pause_container_chain(
                <Test as crate::Config>::MaxLengthParaIds::get()
            )
        );

        assert_eq!(
            crate::Call::<Test>::unpause_container_chain { para_id: 42.into() }
                .get_dispatch_info()
                .weight,
            <() as crate::weights::WeightInfo>::unpause_container_chain(
                <Test as crate::Config>::MaxLengthParaIds::get()
            )
        );
    });
}
