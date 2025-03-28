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
    crate::{mock::*, Error, Event, HoldReason, ParaInfo, REGISTRAR_PARAS_INDEX},
    cumulus_test_relay_sproof_builder::RelayStateSproofBuilder,
    dp_container_chain_genesis_data::ContainerChainGenesisData,
    frame_support::{
        assert_noop, assert_ok, dispatch::GetDispatchInfo, traits::fungible::InspectHold,
        BoundedVec, Hashable,
    },
    parity_scale_codec::Encode,
    sp_core::Pair,
    sp_runtime::DispatchError,
    tp_traits::{ParaId, SlotFrequency},
};

#[test]
fn register_para_id_42() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data(),
            None
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

        // Check that InnerRegistrar methods were called properly.
        assert!(Mock::mock()
            .called_hooks
            .contains(&HookCall::InnerRegister(42u32.into())));
        assert!(Mock::mock()
            .called_hooks
            .contains(&HookCall::InnerScheduleParaUpgrade(42u32.into())));
    });
}

#[test]
fn register_para_id_42_twice() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data(),
            None
        ));
        assert_noop!(
            ParaRegistrar::register(
                RuntimeOrigin::signed(ALICE),
                42.into(),
                empty_genesis_data(),
                None
            ),
            Error::<Test>::ParaIdAlreadyRegistered
        );
    });
}

#[test]
fn register_para_id_42_genesis_data_size_too_big() {
    new_test_ext_with_balance(1_000_000_000).execute_with(|| {
        run_to_block(1);
        let genesis_data = ContainerChainGenesisData {
            storage: BoundedVec::try_from(vec![(vec![], vec![0; 5_000_000]).into()]).unwrap(),
            name: Default::default(),
            id: Default::default(),
            fork_id: Default::default(),
            extensions: Default::default(),
            properties: Default::default(),
        };
        assert_noop!(
            ParaRegistrar::register(RuntimeOrigin::signed(ALICE), 42.into(), genesis_data, None),
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
            empty_genesis_data(),
            None
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
            empty_genesis_data(),
            None
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

        // Run two more sessions for the paraId to get deregistered
        // in the relay context (if any) via InnerRegistrar.
        run_to_session(5);
        // Run end_block after run_to_session to mock the reality and
        // kill BufferedParasToDeregister storage after a session change.
        end_block();

        // Check that InnerRegistrar methods were called properly.
        assert!(Mock::mock()
            .called_hooks
            .contains(&HookCall::InnerRegister(42u32.into())));
        assert!(Mock::mock()
            .called_hooks
            .contains(&HookCall::InnerScheduleParaUpgrade(42u32.into())));
        assert!(Mock::mock()
            .called_hooks
            .contains(&HookCall::InnerScheduleParaDowngrade(42u32.into())));
        assert!(Mock::mock()
            .called_hooks
            .contains(&HookCall::InnerDeregister(42u32.into())));
        assert!(Mock::mock()
            .called_hooks
            .contains(&HookCall::InnerDeregisterWeight));
    });
}

#[test]
fn deregister_para_id_42_after_2_sessions() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data(),
            None
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
            empty_genesis_data(),
            None
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
            storage: BoundedVec::try_from(vec![(b"key".to_vec(), b"value".to_vec()).into()])
                .unwrap(),
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
            None
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
            ParaRegistrar::register(RuntimeOrigin::root(), 42.into(), empty_genesis_data(), None),
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
            empty_genesis_data(),
            None
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
            empty_genesis_data(),
            None
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
            empty_genesis_data(),
            None
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
            empty_genesis_data(),
            None
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
        (1.into(), empty_genesis_data(), None),
        (2.into(), empty_genesis_data(), None),
        (3.into(), empty_genesis_data(), None),
        (4.into(), empty_genesis_data(), None),
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
        (4.into(), empty_genesis_data(), None),
        (2.into(), empty_genesis_data(), None),
        (3.into(), empty_genesis_data(), None),
        (1.into(), empty_genesis_data(), None),
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
        (2.into(), empty_genesis_data(), None),
        (3.into(), empty_genesis_data(), None),
        (4.into(), empty_genesis_data(), None),
        (2.into(), empty_genesis_data(), None),
    ])
    .execute_with(|| {
        run_to_block(1);
    });
}

#[test]
#[should_panic = "genesis data for para_id 2 is too large: 5000024 bytes"]
fn genesis_error_genesis_data_size_too_big() {
    let genesis_data = ContainerChainGenesisData {
        storage: BoundedVec::try_from(vec![(vec![], vec![0; 5_000_000]).into()]).unwrap(),
        name: Default::default(),
        id: Default::default(),
        fork_id: Default::default(),
        extensions: Default::default(),
        properties: Default::default(),
    };
    new_test_ext_with_genesis(vec![(2.into(), genesis_data, None)]).execute_with(|| {
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
            empty_genesis_data(),
            None
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
            empty_genesis_data(),
            None
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
            empty_genesis_data(),
            None
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
            empty_genesis_data(),
            None
        ));
        assert_eq!(
            Mock::mock().called_hooks,
            vec![HookCall::InnerRegister(42.into())]
        );
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
        ));
        assert_eq!(
            Mock::mock().called_hooks,
            vec![
                HookCall::InnerRegister(42.into()),
                HookCall::MarkedValid(42.into()),
                HookCall::InnerScheduleParaUpgrade(42.into())
            ]
        );
    });
}

#[test]
fn deregister_returns_bond_immediately_if_not_marked_as_valid() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let genesis_data = empty_genesis_data();
        let genesis_size_bytes = genesis_data.encoded_size();
        let bond = DataDepositPerByte::get() * genesis_size_bytes as u128;
        let balance_before = Balances::free_balance(ALICE);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            genesis_data,
            None
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
        let genesis_data = empty_genesis_data();
        let genesis_size_bytes = genesis_data.encoded_size();
        let bond = DataDepositPerByte::get() * genesis_size_bytes as u128;
        let balance_before = Balances::free_balance(ALICE);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            genesis_data,
            None
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
            empty_genesis_data(),
            None
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
            empty_genesis_data(),
            None
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
            empty_genesis_data(),
            None
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
            empty_genesis_data(),
            None
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
            empty_genesis_data(),
            None
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
                None
            ),
            Error::<Test>::ParaIdAlreadyRegistered,
        );
        run_to_session(1);
        assert_noop!(
            ParaRegistrar::register(
                RuntimeOrigin::signed(ALICE),
                42.into(),
                empty_genesis_data(),
                None
            ),
            Error::<Test>::ParaIdAlreadyRegistered,
        );
        run_to_session(2);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data(),
            None
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
            empty_genesis_data(),
            None
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
            storage: BoundedVec::try_from(vec![(b"key".to_vec(), b"value".to_vec()).into()])
                .unwrap(),
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
            None
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
            empty_genesis_data(),
            None
        ));
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            43.into(),
            empty_genesis_data(),
            None
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
        // Run end_block after each run_to_session to mock the reality and
        // kill BufferedParasToDeregister storage after a session change.
        end_block();

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
            Mock::mock().called_hooks,
            vec![
                HookCall::InnerRegister(42.into()),
                HookCall::InnerRegister(43.into()),
                HookCall::MarkedValid(42.into()),
                HookCall::InnerScheduleParaUpgrade(42.into()),
                HookCall::MarkedValid(43.into()),
                HookCall::InnerScheduleParaUpgrade(43.into()),
                HookCall::InnerScheduleParaDowngrade(42.into()),
                HookCall::InnerScheduleParaDowngrade(43.into()),
            ]
        );

        run_to_session(4);
        end_block();
        start_block();

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
            Mock::mock().called_hooks,
            vec![
                HookCall::InnerRegister(42.into()),
                HookCall::InnerRegister(43.into()),
                HookCall::MarkedValid(42.into()),
                HookCall::InnerScheduleParaUpgrade(42.into()),
                HookCall::MarkedValid(43.into()),
                HookCall::InnerScheduleParaUpgrade(43.into()),
                HookCall::InnerScheduleParaDowngrade(42.into()),
                HookCall::InnerScheduleParaDowngrade(43.into()),
                HookCall::Deregistered(42.into()),
                HookCall::Deregistered(43.into()),
                HookCall::InnerDeregisterWeight,
                HookCall::InnerDeregister(42.into()),
                HookCall::InnerDeregisterWeight,
                HookCall::InnerDeregister(43.into()),
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
            empty_genesis_data(),
            None
        ));
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            43.into(),
            empty_genesis_data(),
            None
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
        // Run end_block after each run_to_session to mock the reality and
        // kill BufferedParasToDeregister storage after a session change.
        end_block();

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
        end_block();
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
            Mock::mock().called_hooks,
            vec![
                HookCall::InnerRegister(42.into()),
                HookCall::InnerRegister(43.into()),
                HookCall::MarkedValid(42.into()),
                HookCall::InnerScheduleParaUpgrade(42.into()),
                HookCall::MarkedValid(43.into()),
                HookCall::InnerScheduleParaUpgrade(43.into()),
                HookCall::InnerScheduleParaDowngrade(42.into()),
                HookCall::InnerScheduleParaDowngrade(43.into()),
            ]
        );

        run_to_session(4);
        end_block();
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
            Mock::mock().called_hooks,
            vec![
                HookCall::InnerRegister(42.into()),
                HookCall::InnerRegister(43.into()),
                HookCall::MarkedValid(42.into()),
                HookCall::InnerScheduleParaUpgrade(42.into()),
                HookCall::MarkedValid(43.into()),
                HookCall::InnerScheduleParaUpgrade(43.into()),
                HookCall::InnerScheduleParaDowngrade(42.into()),
                HookCall::InnerScheduleParaDowngrade(43.into()),
                HookCall::Deregistered(42.into()),
                HookCall::InnerDeregisterWeight,
                HookCall::InnerDeregister(42.into()),
            ]
        );

        run_to_session(5);
        end_block();
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
            Mock::mock().called_hooks,
            vec![
                HookCall::InnerRegister(42.into()),
                HookCall::InnerRegister(43.into()),
                HookCall::MarkedValid(42.into()),
                HookCall::InnerScheduleParaUpgrade(42.into()),
                HookCall::MarkedValid(43.into()),
                HookCall::InnerScheduleParaUpgrade(43.into()),
                HookCall::InnerScheduleParaDowngrade(42.into()),
                HookCall::InnerScheduleParaDowngrade(43.into()),
                HookCall::Deregistered(42.into()),
                HookCall::InnerDeregisterWeight,
                HookCall::InnerDeregister(42.into()),
                HookCall::Deregistered(43.into()),
                HookCall::InnerDeregisterWeight,
                HookCall::InnerDeregister(43.into()),
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
            empty_genesis_data(),
            None
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
            empty_genesis_data(),
            None
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
            empty_genesis_data(),
            None
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
            empty_genesis_data(),
            None
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
            empty_genesis_data(),
            None
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
            empty_genesis_data(),
            None
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

mod register_with_relay_proof {
    use super::*;

    #[test]
    fn can_register_using_relay_manager_signature() {
        new_test_ext().execute_with(|| {
            run_to_block(1);

            let pairs = get_ed25519_pairs(1);
            let mut sproof = RelayStateSproofBuilder::default();
            let para_id: ParaId = 42.into();
            let bytes = para_id.twox_64_concat();
            let key = [REGISTRAR_PARAS_INDEX, bytes.as_slice()].concat();
            let para_info: ParaInfo<
                cumulus_primitives_core::relay_chain::AccountId,
                cumulus_primitives_core::relay_chain::Balance,
            > = ParaInfo {
                manager: pairs[0].public().into(),
                deposit: Default::default(),
                locked: None,
            };
            sproof.additional_key_values = vec![(key, para_info.encode())];
            let (relay_parent_storage_root, proof) = sproof.into_state_root_and_proof();

            Mock::mutate(|m| {
                m.relay_storage_roots.insert(1, relay_parent_storage_root);
            });

            let account = ALICE;
            let relay_storage_root = relay_parent_storage_root;
            let signature_msg =
                ParaRegistrar::relay_signature_msg(para_id, &account, relay_storage_root);
            let signature: cumulus_primitives_core::relay_chain::Signature =
                pairs[0].sign(&signature_msg).into();

            assert_ok!(ParaRegistrar::register_with_relay_proof(
                RuntimeOrigin::signed(ALICE),
                42.into(),
                None,
                1,
                proof,
                signature,
                empty_genesis_data(),
                None
            ));

            System::assert_last_event(Event::ParaIdRegistered { para_id: 42.into() }.into());
        });
    }

    #[test]
    fn cannot_register_para_if_relay_root_not_stored() {
        // Check that storage proof is invalid when the relay storage root provider returns `None`.
        new_test_ext().execute_with(|| {
            run_to_block(1);

            let pairs = get_ed25519_pairs(1);
            let mut sproof = RelayStateSproofBuilder::default();
            let para_id: ParaId = 42.into();
            let bytes = para_id.twox_64_concat();
            let key = [REGISTRAR_PARAS_INDEX, bytes.as_slice()].concat();
            let para_info: ParaInfo<
                cumulus_primitives_core::relay_chain::AccountId,
                cumulus_primitives_core::relay_chain::Balance,
            > = ParaInfo {
                manager: pairs[0].public().into(),
                deposit: Default::default(),
                locked: None,
            };
            sproof.additional_key_values = vec![(key, para_info.encode())];
            // Intentionally don't store the storage root in Mock
            let (relay_parent_storage_root, proof) = sproof.into_state_root_and_proof();
            let account = ALICE;
            let relay_storage_root = relay_parent_storage_root;
            let signature_msg =
                ParaRegistrar::relay_signature_msg(para_id, &account, relay_storage_root);
            let signature: cumulus_primitives_core::relay_chain::Signature =
                pairs[0].sign(&signature_msg).into();

            assert_noop!(
                ParaRegistrar::register_with_relay_proof(
                    RuntimeOrigin::signed(ALICE),
                    42.into(),
                    None,
                    1,
                    proof,
                    signature,
                    empty_genesis_data(),
                    None
                ),
                Error::<Test>::RelayStorageRootNotFound
            );
        });
    }

    #[test]
    fn cannot_register_invalid_storage_proof() {
        new_test_ext().execute_with(|| {
            run_to_block(1);

            let pairs = get_ed25519_pairs(1);
            let mut sproof = RelayStateSproofBuilder::default();
            let para_id: ParaId = 42.into();
            let bytes = para_id.twox_64_concat();
            let key = [REGISTRAR_PARAS_INDEX, bytes.as_slice()].concat();
            let para_info: ParaInfo<
                cumulus_primitives_core::relay_chain::AccountId,
                cumulus_primitives_core::relay_chain::Balance,
            > = ParaInfo {
                manager: pairs[0].public().into(),
                deposit: Default::default(),
                locked: None,
            };
            sproof.additional_key_values = vec![(key, para_info.encode())];
            let (relay_parent_storage_root, _actual_proof) = sproof.into_state_root_and_proof();

            Mock::mutate(|m| {
                m.relay_storage_roots.insert(1, relay_parent_storage_root);
            });

            let account = ALICE;
            let relay_storage_root = relay_parent_storage_root;
            let signature_msg =
                ParaRegistrar::relay_signature_msg(para_id, &account, relay_storage_root);
            let signature: cumulus_primitives_core::relay_chain::Signature =
                pairs[0].sign(&signature_msg).into();
            // Instead of using the actual proof, just pass some random bytes
            let proof =
                sp_trie::StorageProof::new([b"A".to_vec(), b"AA".to_vec(), b"AAA".to_vec()]);

            assert_noop!(
                ParaRegistrar::register_with_relay_proof(
                    RuntimeOrigin::signed(ALICE),
                    42.into(),
                    None,
                    1,
                    proof,
                    signature,
                    empty_genesis_data(),
                    None
                ),
                Error::<Test>::InvalidRelayStorageProof
            );
        });
    }

    #[test]
    fn cannot_register_if_not_registered_in_relay() {
        new_test_ext().execute_with(|| {
            run_to_block(1);

            let pairs = get_ed25519_pairs(1);
            let (relay_parent_storage_root, proof) =
                RelayStateSproofBuilder::default().into_state_root_and_proof();

            Mock::mutate(|m| {
                m.relay_storage_roots.insert(1, relay_parent_storage_root);
            });

            let para_id: ParaId = 42.into();
            let account = ALICE;
            let relay_storage_root = relay_parent_storage_root;
            let signature_msg =
                ParaRegistrar::relay_signature_msg(para_id, &account, relay_storage_root);
            let signature: cumulus_primitives_core::relay_chain::Signature =
                pairs[0].sign(&signature_msg).into();

            assert_noop!(
                ParaRegistrar::register_with_relay_proof(
                    RuntimeOrigin::signed(ALICE),
                    42.into(),
                    None,
                    1,
                    proof,
                    signature,
                    empty_genesis_data(),
                    None
                ),
                Error::<Test>::InvalidRelayStorageProof
            );
        });
    }

    #[test]
    fn cannot_register_signature_for_a_different_account() {
        new_test_ext().execute_with(|| {
            run_to_block(1);

            let pairs = get_ed25519_pairs(1);
            let mut sproof = RelayStateSproofBuilder::default();
            let para_id: ParaId = 42.into();
            let bytes = para_id.twox_64_concat();
            let key = [REGISTRAR_PARAS_INDEX, bytes.as_slice()].concat();
            let para_info: ParaInfo<
                cumulus_primitives_core::relay_chain::AccountId,
                cumulus_primitives_core::relay_chain::Balance,
            > = ParaInfo {
                manager: pairs[0].public().into(),
                deposit: Default::default(),
                locked: None,
            };
            sproof.additional_key_values = vec![(key, para_info.encode())];
            let (relay_parent_storage_root, proof) = sproof.into_state_root_and_proof();

            Mock::mutate(|m| {
                m.relay_storage_roots.insert(1, relay_parent_storage_root);
            });

            let account = ALICE;
            let relay_storage_root = relay_parent_storage_root;
            let signature_msg =
                ParaRegistrar::relay_signature_msg(para_id, &account, relay_storage_root);
            let signature: cumulus_primitives_core::relay_chain::Signature =
                pairs[0].sign(&signature_msg).into();

            // Signature is for ALICE, we use origin BOB
            assert_noop!(
                ParaRegistrar::register_with_relay_proof(
                    RuntimeOrigin::signed(BOB),
                    42.into(),
                    None,
                    1,
                    proof,
                    signature,
                    empty_genesis_data(),
                    None
                ),
                Error::<Test>::InvalidRelayManagerSignature
            );
        });
    }

    #[test]
    fn cannot_register_signature_for_a_different_para_id() {
        new_test_ext().execute_with(|| {
            run_to_block(1);

            let pairs = get_ed25519_pairs(1);
            let mut sproof = RelayStateSproofBuilder::default();
            let para_id: ParaId = 43.into();
            let bytes = para_id.twox_64_concat();
            let key = [REGISTRAR_PARAS_INDEX, bytes.as_slice()].concat();
            let para_info: ParaInfo<
                cumulus_primitives_core::relay_chain::AccountId,
                cumulus_primitives_core::relay_chain::Balance,
            > = ParaInfo {
                manager: pairs[0].public().into(),
                deposit: Default::default(),
                locked: None,
            };
            sproof.additional_key_values = vec![(key, para_info.encode())];
            let (relay_parent_storage_root, proof) = sproof.into_state_root_and_proof();

            Mock::mutate(|m| {
                m.relay_storage_roots.insert(1, relay_parent_storage_root);
            });

            let account = ALICE;
            let relay_storage_root = relay_parent_storage_root;
            let signature_msg =
                ParaRegistrar::relay_signature_msg(para_id, &account, relay_storage_root);
            let signature: cumulus_primitives_core::relay_chain::Signature =
                pairs[0].sign(&signature_msg).into();

            // The signature and the storage proof are for para id 43, registering 42 should fail
            assert_noop!(
                ParaRegistrar::register_with_relay_proof(
                    RuntimeOrigin::signed(ALICE),
                    42.into(),
                    None,
                    1,
                    proof,
                    signature,
                    empty_genesis_data(),
                    None
                ),
                Error::<Test>::InvalidRelayStorageProof
            );
        });
    }

    #[test]
    fn cannot_register_invalid_signature() {
        new_test_ext().execute_with(|| {
            run_to_block(1);

            let pairs = get_ed25519_pairs(1);
            let mut sproof = RelayStateSproofBuilder::default();
            let para_id: ParaId = 42.into();
            let bytes = para_id.twox_64_concat();
            let key = [REGISTRAR_PARAS_INDEX, bytes.as_slice()].concat();
            let para_info: ParaInfo<
                cumulus_primitives_core::relay_chain::AccountId,
                cumulus_primitives_core::relay_chain::Balance,
            > = ParaInfo {
                manager: pairs[0].public().into(),
                deposit: Default::default(),
                locked: None,
            };
            sproof.additional_key_values = vec![(key, para_info.encode())];
            let (relay_parent_storage_root, proof) = sproof.into_state_root_and_proof();

            Mock::mutate(|m| {
                m.relay_storage_roots.insert(1, relay_parent_storage_root);
            });

            let account = ALICE;
            let relay_storage_root = relay_parent_storage_root;
            let signature_msg =
                ParaRegistrar::relay_signature_msg(para_id, &account, relay_storage_root);
            let mut signature_ed25519 = pairs[0].sign(&signature_msg);
            // Flip one bit in the signature to make it invalid
            signature_ed25519.0[30] ^= 0x01;
            let signature: cumulus_primitives_core::relay_chain::Signature =
                signature_ed25519.into();

            assert_noop!(
                ParaRegistrar::register_with_relay_proof(
                    RuntimeOrigin::signed(ALICE),
                    42.into(),
                    None,
                    1,
                    proof,
                    signature,
                    empty_genesis_data(),
                    None
                ),
                Error::<Test>::InvalidRelayManagerSignature
            );
        });
    }
}

mod deregister_with_relay_proof {
    use super::*;

    #[test]
    fn can_deregister_with_empty_relay_state() {
        // Create a relay state proof for an empty state. Check that any parachain can be deregistered.
        new_test_ext().execute_with(|| {
            run_to_block(1);
            let genesis_data = empty_genesis_data();
            let genesis_size_bytes = genesis_data.encoded_size();
            let hold = DataDepositPerByte::get() * genesis_size_bytes as u128;
            assert_ok!(ParaRegistrar::register(
                RuntimeOrigin::signed(ALICE),
                42.into(),
                genesis_data,
                None
            ));
            assert!(ParaRegistrar::registrar_deposit(ParaId::from(42)).is_some());
            assert_ok!(ParaRegistrar::mark_valid_for_collating(
                RuntimeOrigin::root(),
                42.into(),
            ));
            assert_eq!(
                Balances::balance_on_hold(&HoldReason::RegistrarDeposit.into(), &ALICE),
                hold
            );

            let alice_balance_before = System::account(ALICE).data;
            let bob_balance_before = System::account(BOB).data;

            let (relay_parent_storage_root, proof) =
                RelayStateSproofBuilder::default().into_state_root_and_proof();

            Mock::mutate(|m| {
                m.relay_storage_roots.insert(1, relay_parent_storage_root);
            });

            // Can deregister para because it does not exist in the relay chain
            assert_ok!(ParaRegistrar::deregister_with_relay_proof(
                RuntimeOrigin::signed(BOB),
                42.into(),
                1,
                proof,
            ));
            System::assert_last_event(Event::ParaIdDeregistered { para_id: 42.into() }.into());

            // Check that Bob is given Alice deposit
            let alice_balance_after = System::account(ALICE).data;
            let bob_balance_after = System::account(BOB).data;
            // Alice free balance has not increased
            assert_eq!(alice_balance_after.free, alice_balance_before.free);
            // Deposit is no longer on hold
            assert_eq!(
                Balances::balance_on_hold(&HoldReason::RegistrarDeposit.into(), &ALICE),
                0u128
            );
            // Bob gained exactly Alice reserve
            assert_eq!(
                bob_balance_after.free,
                bob_balance_before.free + alice_balance_before.reserved
            );
            // Alice no longer has any reserved balance
            assert_eq!(alice_balance_after.reserved, 0);
        });
    }

    #[test]
    fn can_deregister_pending_para() {
        // Create a relay state proof for an empty state. Check that any parachain can be deregistered.
        new_test_ext().execute_with(|| {
            run_to_block(1);
            let genesis_data = empty_genesis_data();
            let genesis_size_bytes = genesis_data.encoded_size();
            let hold = DataDepositPerByte::get() * genesis_size_bytes as u128;
            assert_ok!(ParaRegistrar::register(
                RuntimeOrigin::signed(ALICE),
                42.into(),
                genesis_data,
                None
            ));
            assert!(ParaRegistrar::registrar_deposit(ParaId::from(42)).is_some());
            assert_eq!(
                Balances::balance_on_hold(&HoldReason::RegistrarDeposit.into(), &ALICE),
                hold
            );
            // Do not call mark_valid_for_collating

            let alice_balance_before = System::account(ALICE).data;
            let bob_balance_before = System::account(BOB).data;

            let (relay_parent_storage_root, proof) =
                RelayStateSproofBuilder::default().into_state_root_and_proof();

            Mock::mutate(|m| {
                m.relay_storage_roots.insert(1, relay_parent_storage_root);
            });

            // Can deregister para because it does not exist in the relay chain
            assert_ok!(ParaRegistrar::deregister_with_relay_proof(
                RuntimeOrigin::signed(BOB),
                42.into(),
                1,
                proof,
            ));
            System::assert_last_event(Event::ParaIdDeregistered { para_id: 42.into() }.into());

            // Check that Bob is given Alice deposit
            let alice_balance_after = System::account(ALICE).data;
            let bob_balance_after = System::account(BOB).data;
            // Alice free balance has not increased
            assert_eq!(alice_balance_after.free, alice_balance_before.free);
            // Deposit is no longer on hold
            assert_eq!(
                Balances::balance_on_hold(&HoldReason::RegistrarDeposit.into(), &ALICE),
                0u128
            );
            // Bob gained exactly Alice reserve
            assert_eq!(
                bob_balance_after.free,
                bob_balance_before.free + alice_balance_before.reserved
            );
            // Alice no longer has any reserved balance
            assert_eq!(alice_balance_after.reserved, 0);
        });
    }

    #[test]
    fn cannot_deregister_para_if_relay_root_not_stored() {
        // Check that storage proof is invalid when the relay storage root provider returns `None`.
        new_test_ext().execute_with(|| {
            run_to_block(1);
            assert_ok!(ParaRegistrar::register(
                RuntimeOrigin::signed(ALICE),
                42.into(),
                empty_genesis_data(),
                None
            ));
            assert!(ParaRegistrar::registrar_deposit(ParaId::from(42)).is_some());
            assert_ok!(ParaRegistrar::mark_valid_for_collating(
                RuntimeOrigin::root(),
                42.into(),
            ));

            // Intentionally don't store the storage root in Mock
            let (_relay_parent_storage_root, proof) =
                RelayStateSproofBuilder::default().into_state_root_and_proof();

            assert_noop!(
                ParaRegistrar::deregister_with_relay_proof(
                    RuntimeOrigin::signed(BOB),
                    42.into(),
                    1,
                    proof,
                ),
                Error::<Test>::RelayStorageRootNotFound
            );
        });
    }

    #[test]
    fn cannot_deregister_para_still_present_in_relay() {
        // Create a relay state proof where a parachain is still registered there.
        // Check that it cannot be deregistered.
        new_test_ext().execute_with(|| {
            run_to_block(1);
            assert_ok!(ParaRegistrar::register(
                RuntimeOrigin::signed(ALICE),
                42.into(),
                empty_genesis_data(),
                None
            ));
            assert!(ParaRegistrar::registrar_deposit(ParaId::from(42)).is_some());
            assert_ok!(ParaRegistrar::mark_valid_for_collating(
                RuntimeOrigin::root(),
                42.into(),
            ));

            let mut sproof = RelayStateSproofBuilder::default();
            let para_id: ParaId = 42.into();
            let bytes = para_id.twox_64_concat();
            let key = [REGISTRAR_PARAS_INDEX, bytes.as_slice()].concat();
            let para_info: ParaInfo<
                cumulus_primitives_core::relay_chain::AccountId,
                cumulus_primitives_core::relay_chain::Balance,
            > = ParaInfo {
                manager: cumulus_primitives_core::relay_chain::AccountId::from([0; 32]),
                deposit: Default::default(),
                locked: None,
            };
            sproof.additional_key_values = vec![(key, para_info.encode())];
            let (relay_parent_storage_root, proof) = sproof.into_state_root_and_proof();

            Mock::mutate(|m| {
                m.relay_storage_roots.insert(1, relay_parent_storage_root);
            });

            assert_noop!(
                ParaRegistrar::deregister_with_relay_proof(
                    RuntimeOrigin::signed(BOB),
                    42.into(),
                    1,
                    proof,
                ),
                Error::<Test>::ParaStillExistsInRelay
            );
        });
    }

    #[test]
    fn cannot_deregister_invalid_storage_proof() {
        // Passing an invalid storage proof returns an error, even if the para id could be
        // deregistered with a real proof.
        new_test_ext().execute_with(|| {
            run_to_block(1);
            assert_ok!(ParaRegistrar::register(
                RuntimeOrigin::signed(ALICE),
                42.into(),
                empty_genesis_data(),
                None
            ));
            assert!(ParaRegistrar::registrar_deposit(ParaId::from(42)).is_some());
            assert_ok!(ParaRegistrar::mark_valid_for_collating(
                RuntimeOrigin::root(),
                42.into(),
            ));

            let (relay_parent_storage_root, _actual_proof) =
                RelayStateSproofBuilder::default().into_state_root_and_proof();

            Mock::mutate(|m| {
                m.relay_storage_roots.insert(1, relay_parent_storage_root);
            });

            // Instead of using the actual proof, just pass some random bytes
            let proof =
                sp_trie::StorageProof::new([b"A".to_vec(), b"AA".to_vec(), b"AAA".to_vec()]);

            assert_noop!(
                ParaRegistrar::deregister_with_relay_proof(
                    RuntimeOrigin::signed(BOB),
                    42.into(),
                    1,
                    proof,
                ),
                Error::<Test>::InvalidRelayStorageProof
            );
        });
    }
}

#[test]
fn weights_assigned_to_extrinsics_are_correct() {
    new_test_ext().execute_with(|| {
        assert_eq!(
            crate::Call::<Test>::register {
                para_id: 42.into(),
                genesis_data: empty_genesis_data(),
                head_data: None
            }
            .get_dispatch_info()
            .call_weight,
            <() as crate::weights::WeightInfo>::register(
                empty_genesis_data().encoded_size() as u32,
                0
            )
        );

        assert_eq!(
            crate::Call::<Test>::deregister { para_id: 42.into() }
                .get_dispatch_info()
                .call_weight,
            <() as crate::weights::WeightInfo>::deregister_immediate()
                .max(<() as crate::weights::WeightInfo>::deregister_scheduled())
        );

        assert_eq!(
            crate::Call::<Test>::mark_valid_for_collating { para_id: 42.into() }
                .get_dispatch_info()
                .call_weight,
            <() as crate::weights::WeightInfo>::mark_valid_for_collating()
        );

        assert_eq!(
            crate::Call::<Test>::pause_container_chain { para_id: 42.into() }
                .get_dispatch_info()
                .call_weight,
            <() as crate::weights::WeightInfo>::pause_container_chain()
        );

        assert_eq!(
            crate::Call::<Test>::unpause_container_chain { para_id: 42.into() }
                .get_dispatch_info()
                .call_weight,
            <() as crate::weights::WeightInfo>::unpause_container_chain()
        );
    });
}
