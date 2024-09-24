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

#![cfg(test)]

use {
    crate::{
        tests::common::*, ContainerRegistrar, Paras, Registrar, RuntimeCall, SlotFrequency, System,
    },
    cumulus_primitives_core::relay_chain::HeadData,
    frame_support::{assert_noop, assert_ok},
    pallet_registrar::Event as ContainerRegistrarEvent,
    pallet_registrar_runtime_api::{
        runtime_decl_for_registrar_api::RegistrarApi, ContainerChainGenesisData,
    },
    runtime_common::paras_registrar,
    runtime_parachains::configuration as parachains_configuration,
    sp_runtime::traits::Dispatchable,
    sp_std::vec,
};

#[test]
fn registrar_needs_a_reserved_para_id() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_next_free_para_id(2000u32.into())
        .build()
        .execute_with(|| {
            //run_to_block(3);
            assert_noop!(
                Registrar::register(
                    origin_of(ALICE.into()),
                    100u32.into(),
                    vec![].into(),
                    vec![].into()
                ),
                paras_registrar::Error::<Runtime>::NotReserved
            );

            // After a reservation, we can register
            let next_para_id = paras_registrar::NextFreeParaId::<Runtime>::get();

            assert_ok!(Registrar::reserve(origin_of(ALICE.into())));

            assert_noop!(
                Registrar::register(
                    origin_of(ALICE.into()),
                    next_para_id,
                    vec![].into(),
                    vec![].into()
                ),
                paras_registrar::Error::<Runtime>::InvalidCode
            );

            let validation_code: cumulus_primitives_core::relay_chain::ValidationCode =
                vec![1u8; cumulus_primitives_core::relay_chain::MIN_CODE_SIZE as usize].into();
            assert_ok!(Registrar::register(
                origin_of(ALICE.into()),
                next_para_id,
                vec![].into(),
                validation_code.clone()
            ));

            assert!(Paras::lifecycle(next_para_id)
                .expect("para should be onboarding")
                .is_onboarding());
            // Two sessions later the para should be a parathread
            // But only if the pvf is accepted! which we havent done
            run_to_session(2);
            assert!(Paras::lifecycle(next_para_id)
                .expect("para should be onboarding")
                .is_onboarding());

            // Now let's accept the pvf, so that after 2 sesssions we have the chain onboarded
            assert_ok!(Paras::add_trusted_validation_code(
                root_origin(),
                validation_code
            ));
            run_to_session(4);

            // PVF accepted and the para should be a parathread
            assert!(Paras::lifecycle(next_para_id)
                .expect("para should be parathread")
                .is_parathread());
        });
}

#[test]
fn register_para_via_container_registrar() {
    ExtBuilder::default()
        .with_para_ids(vec![
            (1001, empty_genesis_data(), u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            // In this test we're gonna register a para via ContainerRegistrar,
            // which will internally use the InnerRegistrar type to also register the para
            // in the relay Registrar pallet.

            assert_eq!(Runtime::registered_paras(), vec![1001.into(), 1002.into()]);
            assert_eq!(Runtime::genesis_data(1003.into()).as_ref(), None);
            run_to_session(2u32);
            assert_eq!(
                parachains_configuration::ActiveConfig::<Runtime>::get().max_head_data_size,
                20500
            );

            let validation_code =
                vec![1u8; cumulus_primitives_core::relay_chain::MIN_CODE_SIZE as usize];
            let genesis_data_1003 = ContainerChainGenesisData {
                storage: vec![(b":code".to_vec(), validation_code.clone()).into()],
                name: Default::default(),
                id: Default::default(),
                fork_id: Default::default(),
                extensions: vec![],
                properties: Default::default(),
            };

            assert_ok!(
                ContainerRegistrar::register(
                    origin_of(ALICE.into()),
                    1003.into(),
                    genesis_data_1003.clone(),
                    Some(HeadData(vec![1u8, 1u8, 1u8]))
                ),
                ()
            );

            // Now let's check if the para was preoperly registered in the relay.
            // Run to next session.
            run_to_session(3);
            assert!(Paras::lifecycle(1003.into())
                .expect("para should be onboarding")
                .is_onboarding());

            // We need to accept the validation code, so that the para is onboarded after 2 sessions.
            assert_ok!(Paras::add_trusted_validation_code(
                root_origin(),
                validation_code.into()
            ));
            run_to_session(5);

            // Now the para should be a parathread.
            assert!(Paras::lifecycle(1003.into())
                .expect("para should be parathread")
                .is_parathread());
        });
}

#[test]
fn cannot_register_para_twice_in_relay() {
    ExtBuilder::default()
        .with_para_ids(vec![
            (1001, empty_genesis_data(), u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            // In this test we're gonna confirm that a para cannot be registered in the relay
            // after being already registered through ContainerRegistrar.

            assert_eq!(Runtime::registered_paras(), vec![1001.into(), 1002.into()]);
            assert_eq!(Runtime::genesis_data(1003.into()).as_ref(), None);
            run_to_session(2u32);
            assert_eq!(
                parachains_configuration::ActiveConfig::<Runtime>::get().max_head_data_size,
                20500
            );

            let validation_code =
                vec![1u8; cumulus_primitives_core::relay_chain::MIN_CODE_SIZE as usize];
            let genesis_data_1003 = ContainerChainGenesisData {
                storage: vec![(b":code".to_vec(), validation_code.clone()).into()],
                name: Default::default(),
                id: Default::default(),
                fork_id: Default::default(),
                extensions: vec![],
                properties: Default::default(),
            };

            assert_ok!(
                ContainerRegistrar::register(
                    origin_of(ALICE.into()),
                    1003.into(),
                    genesis_data_1003.clone(),
                    Some(HeadData(vec![1u8, 1u8, 1u8]))
                ),
                ()
            );

            // Now let's check if the para was preoperly registered in the relay.
            // Run to next session.
            run_to_session(3);
            assert!(Paras::lifecycle(1003.into())
                .expect("para should be onboarding")
                .is_onboarding());

            // We need to accept the validation code, so that the para is onboarded after 2 sessions.
            assert_ok!(Paras::add_trusted_validation_code(
                root_origin(),
                validation_code.clone().into()
            ));
            run_to_session(5);

            // Now the para should be a parathread.
            assert!(Paras::lifecycle(1003.into())
                .expect("para should be parathread")
                .is_parathread());

            // We shouldn't be able to register the para again
            assert_noop!(
                Registrar::register(
                    origin_of(ALICE.into()),
                    1003.into(),
                    vec![].into(),
                    validation_code.into()
                ),
                paras_registrar::Error::<Runtime>::AlreadyRegistered
            );
        });
}

#[test]
fn mark_valid_for_collating_converts_to_parachain() {
    ExtBuilder::default()
        .with_para_ids(vec![
            (1001, empty_genesis_data(), u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            // In this test we're gonna check that inside mark_valid_for_collating(),
            // if we are passing a parathread, this one will be upgraded to a parachain
            // at the end of the execution.

            assert_eq!(Runtime::registered_paras(), vec![1001.into(), 1002.into()]);
            assert_eq!(Runtime::genesis_data(1003.into()).as_ref(), None);
            run_to_session(2u32);
            assert_eq!(
                parachains_configuration::ActiveConfig::<Runtime>::get().max_head_data_size,
                20500
            );

            let validation_code =
                vec![1u8; cumulus_primitives_core::relay_chain::MIN_CODE_SIZE as usize];
            let genesis_data_1003 = ContainerChainGenesisData {
                storage: vec![(b":code".to_vec(), validation_code.clone()).into()],
                name: Default::default(),
                id: Default::default(),
                fork_id: Default::default(),
                extensions: vec![],
                properties: Default::default(),
            };

            assert_ok!(
                ContainerRegistrar::register(
                    origin_of(ALICE.into()),
                    1003.into(),
                    genesis_data_1003.clone(),
                    Some(HeadData(vec![1u8, 1u8, 1u8]))
                ),
                ()
            );

            // Now let's check if the para was preoperly registered in the relay.
            // Run to next session.
            run_to_session(3);
            assert!(Paras::lifecycle(1003.into())
                .expect("para should be onboarding")
                .is_onboarding());

            // We need to accept the validation code, so that the para is onboarded after 2 sessions.
            assert_ok!(Paras::add_trusted_validation_code(
                root_origin(),
                validation_code.into()
            ));
            run_to_session(5);

            // Now the para should be a parathread.
            assert!(Paras::lifecycle(1003.into())
                .expect("para should be parathread")
                .is_parathread());

            set_dummy_boot_node(origin_of(ALICE.into()), 1003.into());

            // Call mark_valid_for_collating.
            assert_ok!(
                ContainerRegistrar::mark_valid_for_collating(root_origin(), 1003.into()),
                ()
            );

            // The change should be applied after 2 sessions.
            run_to_session(7);
            assert!(Paras::lifecycle(1003.into())
                .expect("para should be parachain")
                .is_parachain());
        });
}

#[test]
fn deregister_calls_schedule_para_cleanup() {
    ExtBuilder::default()
        .with_para_ids(vec![
            (1001, empty_genesis_data(), u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            // In this test we're gonna check that when calling ContainerRegistrar::deregister(),
            // the para is also offboarded from the relay.

            assert_eq!(Runtime::registered_paras(), vec![1001.into(), 1002.into()]);
            assert_eq!(Runtime::genesis_data(1003.into()).as_ref(), None);
            run_to_session(2u32);
            assert_eq!(
                parachains_configuration::ActiveConfig::<Runtime>::get().max_head_data_size,
                20500
            );

            let validation_code =
                vec![1u8; cumulus_primitives_core::relay_chain::MIN_CODE_SIZE as usize];
            let genesis_data_1003 = ContainerChainGenesisData {
                storage: vec![(b":code".to_vec(), validation_code.clone()).into()],
                name: Default::default(),
                id: Default::default(),
                fork_id: Default::default(),
                extensions: vec![],
                properties: Default::default(),
            };

            assert_ok!(
                ContainerRegistrar::register(
                    origin_of(ALICE.into()),
                    1003.into(),
                    genesis_data_1003.clone(),
                    Some(HeadData(vec![1u8, 1u8, 1u8]))
                ),
                ()
            );

            // Now let's check if the para was preoperly registered in the relay.
            // Run to next session.
            run_to_session(3);
            assert!(Paras::lifecycle(1003.into())
                .expect("para should be onboarding")
                .is_onboarding());

            // We need to accept the validation code, so that the para is onboarded after 2 sessions.
            assert_ok!(Paras::add_trusted_validation_code(
                root_origin(),
                validation_code.into()
            ));

            run_to_session(5);

            // Now the para should be a parathread.
            assert!(Paras::lifecycle(1003.into())
                .expect("para should be parathread")
                .is_parathread());

            set_dummy_boot_node(origin_of(ALICE.into()), 1003.into());

            // Call mark_valid_for_collating.
            assert_ok!(
                ContainerRegistrar::mark_valid_for_collating(root_origin(), 1003.into()),
                ()
            );

            // The change should be applied after 2 sessions.
            run_to_session(7);
            assert!(Paras::lifecycle(1003.into())
                .expect("para should be parachain")
                .is_parachain());

            assert_eq!(
                Runtime::genesis_data(1003.into()).as_ref(),
                Some(&genesis_data_1003)
            );

            assert_ok!(ContainerRegistrar::deregister(root_origin(), 1003.into()));

            // Assert that the ParaIdDeregistered event was properly deposited
            System::assert_last_event(
                ContainerRegistrarEvent::ParaIdDeregistered {
                    para_id: 1003.into(),
                }
                .into(),
            );

            run_to_session(9);

            assert_eq!(Runtime::genesis_data(1003.into()).as_ref(), None);

            // Para should be offboarding after 2 sessions.
            assert!(Paras::lifecycle(1003.into())
                .expect("para should be offboarding")
                .is_offboarding());
        });
}

#[test]
fn deregister_two_paras_in_the_same_block() {
    ExtBuilder::default()
        .with_para_ids(vec![
            (1001, empty_genesis_data(), u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            // In this test we're gonna check that when calling ContainerRegistrar::deregister(),
            // two paraIds are properly offboarded from the relay.

            assert_eq!(Runtime::registered_paras(), vec![1001.into(), 1002.into()]);
            assert_eq!(Runtime::genesis_data(1003.into()).as_ref(), None);
            assert_eq!(Runtime::genesis_data(1004.into()).as_ref(), None);
            run_to_session(2u32);
            assert_eq!(
                parachains_configuration::ActiveConfig::<Runtime>::get().max_head_data_size,
                20500
            );

            let validation_code =
                vec![1u8; cumulus_primitives_core::relay_chain::MIN_CODE_SIZE as usize];
            let genesis_data_1003_and_1004 = ContainerChainGenesisData {
                storage: vec![(b":code".to_vec(), validation_code.clone()).into()],
                name: Default::default(),
                id: Default::default(),
                fork_id: Default::default(),
                extensions: vec![],
                properties: Default::default(),
            };

            // Register paraId 1003
            assert_ok!(
                ContainerRegistrar::register(
                    origin_of(ALICE.into()),
                    1003.into(),
                    genesis_data_1003_and_1004.clone(),
                    Some(HeadData(vec![1u8, 1u8, 1u8]))
                ),
                ()
            );

            // Register paraId 1004
            assert_ok!(
                ContainerRegistrar::register(
                    origin_of(ALICE.into()),
                    1004.into(),
                    genesis_data_1003_and_1004.clone(),
                    Some(HeadData(vec![1u8, 1u8, 1u8]))
                ),
                ()
            );

            // Now let's check if the paras were preoperly registered in the relay.
            // Run to next session.
            run_to_session(3);
            assert!(Paras::lifecycle(1003.into())
                .expect("para should be onboarding")
                .is_onboarding());

            assert!(Paras::lifecycle(1004.into())
                .expect("para should be onboarding")
                .is_onboarding());

            // We need to accept the validation code, so that paras are onboarded after 2 sessions.
            assert_ok!(Paras::add_trusted_validation_code(
                root_origin(),
                validation_code.into()
            ));

            run_to_session(5);

            // Now paras should be parathreads.
            assert!(Paras::lifecycle(1003.into())
                .expect("para should be parathread")
                .is_parathread());
            assert!(Paras::lifecycle(1004.into())
                .expect("para should be parathread")
                .is_parathread());

            set_dummy_boot_node(origin_of(ALICE.into()), 1003.into());
            set_dummy_boot_node(origin_of(ALICE.into()), 1004.into());

            // Call mark_valid_for_collating.
            assert_ok!(
                ContainerRegistrar::mark_valid_for_collating(root_origin(), 1003.into()),
                ()
            );

            assert_ok!(
                ContainerRegistrar::mark_valid_for_collating(root_origin(), 1004.into()),
                ()
            );

            // The change should be applied after 2 sessions.
            run_to_session(7);
            assert!(Paras::lifecycle(1003.into())
                .expect("para should be parachain")
                .is_parachain());

            assert!(Paras::lifecycle(1004.into())
                .expect("para should be parachain")
                .is_parachain());

            assert_eq!(
                Runtime::genesis_data(1003.into()).as_ref(),
                Some(&genesis_data_1003_and_1004)
            );

            assert_eq!(
                Runtime::genesis_data(1004.into()).as_ref(),
                Some(&genesis_data_1003_and_1004)
            );

            assert_ok!(ContainerRegistrar::deregister(root_origin(), 1003.into()));

            // Assert that the ParaIdDeregistered event was properly deposited
            System::assert_last_event(
                ContainerRegistrarEvent::ParaIdDeregistered {
                    para_id: 1003.into(),
                }
                .into(),
            );

            assert_ok!(ContainerRegistrar::deregister(root_origin(), 1004.into()));
            System::assert_last_event(
                ContainerRegistrarEvent::ParaIdDeregistered {
                    para_id: 1004.into(),
                }
                .into(),
            );

            run_to_session(9);

            assert_eq!(Runtime::genesis_data(1003.into()).as_ref(), None);
            assert_eq!(Runtime::genesis_data(1004.into()).as_ref(), None);

            // Paras should be offboarding after 2 sessions.
            assert!(Paras::lifecycle(1003.into())
                .expect("para should be offboarding")
                .is_offboarding());

            assert!(Paras::lifecycle(1004.into())
                .expect("para should be offboarding")
                .is_offboarding());
        });
}

#[test]
fn test_register_parathread_not_allowed() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);

            assert_noop!(
                RuntimeCall::ContainerRegistrar(
                    pallet_registrar::Call::<Runtime>::register_parathread {
                        para_id: 3001.into(),
                        slot_frequency: SlotFrequency { min: 1, max: 1 },
                        genesis_data: empty_genesis_data(),
                        head_data: None
                    }
                )
                .dispatch(
                    <Runtime as frame_system::Config>::RuntimeOrigin::signed(AccountId::from(
                        ALICE
                    ))
                ),
                frame_system::Error::<Runtime>::CallFiltered
            );
        });
}

#[test]
fn test_relay_registrar_through_extrinsic_not_allowed() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);

            let validation_code =
                vec![1u8; cumulus_primitives_core::relay_chain::MIN_CODE_SIZE as usize];

            assert_noop!(
                RuntimeCall::Registrar(paras_registrar::Call::<Runtime>::register {
                    id: 3001.into(),
                    validation_code: cumulus_primitives_core::relay_chain::ValidationCode(
                        validation_code
                    ),
                    genesis_head: HeadData(vec![1u8, 1u8, 1u8]),
                })
                .dispatch(
                    <Runtime as frame_system::Config>::RuntimeOrigin::signed(AccountId::from(
                        ALICE
                    ))
                ),
                frame_system::Error::<Runtime>::CallFiltered
            );
        });
}

#[test]
fn test_relay_registrar_deregister_through_extrinsic_not_allowed() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);

            assert_noop!(
                RuntimeCall::Registrar(paras_registrar::Call::<Runtime>::deregister {
                    id: 3001.into()
                })
                .dispatch(
                    <Runtime as frame_system::Config>::RuntimeOrigin::signed(AccountId::from(
                        ALICE
                    ))
                ),
                frame_system::Error::<Runtime>::CallFiltered
            );
        });
}
