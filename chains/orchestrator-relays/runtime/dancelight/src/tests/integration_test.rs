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
        tests::common::*, Balances, CollatorConfiguration, ContainerRegistrar, DataPreservers,
        PreserversAssignmentPaymentWitness, Registrar, StreamPayment, StreamPaymentAssetId,
        TimeUnit,
    },
    cumulus_primitives_core::{relay_chain::HeadData, ParaId},
    dancelight_runtime_constants::currency::EXISTENTIAL_DEPOSIT,
    frame_support::{assert_err, assert_noop, assert_ok, BoundedVec},
    pallet_registrar_runtime_api::{
        runtime_decl_for_registrar_api::RegistrarApi, ContainerChainGenesisData,
    },
    pallet_stream_payment::StreamConfig,
    sp_std::vec,
};

#[test]
fn genesis_balances() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            assert_eq!(
                Balances::usable_balance(AccountId::from(ALICE)) + EXISTENTIAL_DEPOSIT,
                210_000 * UNIT,
            );
            assert_eq!(
                Balances::usable_balance(AccountId::from(BOB)) + EXISTENTIAL_DEPOSIT,
                100_000 * UNIT,
            );
        });
}

#[test]
fn genesis_para_registrar() {
    ExtBuilder::default()
        .with_empty_parachains(vec![1001, 1002])
        .build()
        .execute_with(|| {
            assert_eq!(
                ContainerRegistrar::registered_para_ids(),
                vec![1001.into(), 1002.into()]
            );
        });
}

#[test]
fn genesis_para_registrar_deregister() {
    ExtBuilder::default()
        .with_empty_parachains(vec![1001, 1002])
        .build()
        .execute_with(|| {
            assert_eq!(
                ContainerRegistrar::registered_para_ids(),
                vec![1001.into(), 1002.into()]
            );

            run_to_block(2);
            assert_ok!(
                ContainerRegistrar::deregister(root_origin(), 1002.into()),
                ()
            );

            // Pending
            assert_eq!(
                ContainerRegistrar::pending_registered_para_ids(),
                vec![(2u32, BoundedVec::try_from(vec![1001u32.into()]).unwrap())]
            );

            run_to_session(1);
            assert_eq!(
                ContainerRegistrar::pending_registered_para_ids(),
                vec![(2u32, BoundedVec::try_from(vec![1001u32.into()]).unwrap())]
            );
            assert_eq!(
                ContainerRegistrar::registered_para_ids(),
                vec![1001.into(), 1002.into()]
            );

            run_to_session(2);
            assert_eq!(ContainerRegistrar::pending_registered_para_ids(), vec![]);
            assert_eq!(ContainerRegistrar::registered_para_ids(), vec![1001.into()]);
        });
}

#[test]
fn genesis_para_registrar_runtime_api() {
    ExtBuilder::default()
        .with_empty_parachains(vec![1001, 1002])
        .build()
        .execute_with(|| {
            assert_eq!(
                ContainerRegistrar::registered_para_ids(),
                vec![1001.into(), 1002.into()]
            );
            assert_eq!(Runtime::registered_paras(), vec![1001.into(), 1002.into()]);

            run_to_block(2);
            assert_ok!(
                ContainerRegistrar::deregister(root_origin(), 1002.into()),
                ()
            );
            assert_eq!(Runtime::registered_paras(), vec![1001.into(), 1002.into()]);

            run_to_session(1);
            assert_eq!(
                ContainerRegistrar::registered_para_ids(),
                vec![1001.into(), 1002.into()]
            );
            assert_eq!(Runtime::registered_paras(), vec![1001.into(), 1002.into()]);

            run_to_session(2);
            assert_eq!(ContainerRegistrar::registered_para_ids(), vec![1001.into()]);
            assert_eq!(Runtime::registered_paras(), vec![1001.into()]);
        });
}

#[test]
fn genesis_para_registrar_container_chain_genesis_data_runtime_api() {
    let genesis_data_2001 = empty_genesis_data();
    let genesis_data_2002 = ContainerChainGenesisData {
        storage: vec![(b"key".to_vec(), b"value".to_vec()).into()],
        name: Default::default(),
        id: Default::default(),
        fork_id: Default::default(),
        extensions: vec![],
        properties: Default::default(),
    };
    ExtBuilder::default()
        .with_para_ids(vec![
            ParaRegistrationParams {
                para_id: 2001,
                genesis_data: genesis_data_2001.clone(),
                block_production_credits: u32::MAX,
                collator_assignment_credits: u32::MAX,
                parathread_params: None,
            },
            ParaRegistrationParams {
                para_id: 2002,
                genesis_data: genesis_data_2002.clone(),
                block_production_credits: u32::MAX,
                collator_assignment_credits: u32::MAX,
                parathread_params: None,
            },
        ])
        .with_next_free_para_id(2003.into())
        .build()
        .execute_with(|| {
            assert_eq!(
                ContainerRegistrar::registered_para_ids(),
                vec![2001.into(), 2002.into()]
            );
            assert_eq!(Runtime::registered_paras(), vec![2001.into(), 2002.into()]);

            assert_eq!(
                Runtime::genesis_data(2001.into()).as_ref(),
                Some(&genesis_data_2001)
            );
            assert_eq!(
                Runtime::genesis_data(2002.into()).as_ref(),
                Some(&genesis_data_2002)
            );
            assert_eq!(Runtime::genesis_data(2003.into()).as_ref(), None);

            run_to_block(2);
            assert_ok!(ContainerRegistrar::deregister(root_origin(), 2002.into()), ());

            assert_eq!(Runtime::genesis_data(2002.into()).as_ref(), Some(&genesis_data_2002), "Deregistered container chain genesis data should not be removed until after 2 sessions");
            assert_ok!(Registrar::reserve(origin_of(ALICE.into())));
            assert_ok!(
                ContainerRegistrar::register(
                    origin_of(ALICE.into()),
                    2003.into(),
                    get_genesis_data_with_validation_code().0,
                    Some(HeadData(vec![1u8, 1u8, 1u8]))
                ),
                ()
            );

            // Registered container chains are inserted immediately
            assert_eq!(
                Runtime::genesis_data(2003.into()).as_ref(),
                Some(&get_genesis_data_with_validation_code().0)
            );

            // Deregistered container chain genesis data is removed after 2 sessions
            run_to_session(2u32);
            assert_eq!(Runtime::genesis_data(2002.into()).as_ref(), None);
        });
}

#[test]
fn test_configuration_on_session_change() {
    ExtBuilder::default().build().execute_with(|| {
        assert_eq!(CollatorConfiguration::config().max_collators, 100);
        assert_eq!(
            CollatorConfiguration::config().min_orchestrator_collators,
            2
        );
        assert_eq!(CollatorConfiguration::config().collators_per_container, 2);

        assert_ok!(
            CollatorConfiguration::set_max_collators(root_origin(), 50),
            ()
        );
        run_to_session(1u32);

        assert_ok!(
            CollatorConfiguration::set_min_orchestrator_collators(root_origin(), 20),
            ()
        );
        assert_eq!(CollatorConfiguration::config().max_collators, 100);
        assert_eq!(
            CollatorConfiguration::config().min_orchestrator_collators,
            2
        );
        assert_eq!(CollatorConfiguration::config().collators_per_container, 2);

        run_to_session(2u32);
        assert_ok!(
            CollatorConfiguration::set_collators_per_container(root_origin(), 10),
            ()
        );
        assert_eq!(CollatorConfiguration::config().max_collators, 50);
        assert_eq!(
            CollatorConfiguration::config().min_orchestrator_collators,
            2
        );
        assert_eq!(CollatorConfiguration::config().collators_per_container, 2);

        run_to_session(3u32);

        assert_eq!(CollatorConfiguration::config().max_collators, 50);
        assert_eq!(
            CollatorConfiguration::config().min_orchestrator_collators,
            20
        );
        assert_eq!(CollatorConfiguration::config().collators_per_container, 2);

        run_to_session(4u32);

        assert_eq!(CollatorConfiguration::config().max_collators, 50);
        assert_eq!(
            CollatorConfiguration::config().min_orchestrator_collators,
            20
        );
        assert_eq!(CollatorConfiguration::config().collators_per_container, 10);
    });
}

#[test]
fn test_cannot_mark_valid_para_with_no_bootnodes() {
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
            assert_ok!(Registrar::reserve(origin_of(ALICE.into())));
            assert_ok!(ContainerRegistrar::register(
                origin_of(ALICE.into()),
                2000.into(),
                get_genesis_data_with_validation_code().0,
                Some(HeadData(vec![1u8, 1u8, 1u8]))
            ));
            assert_noop!(
                ContainerRegistrar::mark_valid_for_collating(root_origin(), 2000.into()),
                pallet_data_preservers::Error::<Runtime>::NoBootNodes,
            );
        });
}

#[test]
fn test_container_deregister_unassign_data_preserver() {
    ExtBuilder::default()
        .with_balances(vec![
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            use pallet_data_preservers::{
                AssignerParameterOf, ParaIdsFilter, Profile, ProfileMode, ProviderRequestOf,
            };

            let profile = Profile {
                url: b"test".to_vec().try_into().unwrap(),
                para_ids: ParaIdsFilter::AnyParaId,
                mode: ProfileMode::Bootnode,
                assignment_request: ProviderRequestOf::<Runtime>::Free,
            };

            let para_id = ParaId::from(2000);
            let profile_id = 0u64;
            assert_ok!(Registrar::reserve(origin_of(ALICE.into())));
            assert_ok!(ContainerRegistrar::register(
                origin_of(ALICE.into()),
                para_id,
                get_genesis_data_with_validation_code().0,
                Some(HeadData(vec![1u8, 1u8, 1u8]))
            ));

            assert_ok!(DataPreservers::create_profile(
                origin_of(BOB.into()),
                profile.clone(),
            ));

            // Start assignment
            assert_ok!(DataPreservers::start_assignment(
                origin_of(ALICE.into()),
                profile_id,
                para_id,
                AssignerParameterOf::<Runtime>::Free
            ));
            assert!(pallet_data_preservers::Assignments::<Runtime>::get(para_id).contains(&0u64));

            // Deregister from Registrar
            assert_ok!(ContainerRegistrar::deregister(root_origin(), para_id), ());

            // Check DataPreserver assignment has been cleared
            assert!(pallet_data_preservers::Assignments::<Runtime>::get(para_id).is_empty());
        });
}

#[test]
fn stream_payment_works() {
    ExtBuilder::default()
        .with_balances(vec![
            (AccountId::from(ALICE), 100_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .build()
        .execute_with(|| {
            use pallet_stream_payment::{ChangeKind, StreamConfig};

            assert_ok!(StreamPayment::open_stream(
                origin_of(ALICE.into()),
                BOB.into(),
                StreamConfig {
                    rate: 2 * UNIT,
                    asset_id: StreamPaymentAssetId::Native,
                    time_unit: TimeUnit::BlockNumber,
                    minimum_request_deadline_delay: 0,
                    soft_minimum_deposit: 0,
                },
                1_000 * UNIT,
            ));

            run_block();

            assert_ok!(StreamPayment::perform_payment(origin_of(CHARLIE.into()), 0));
            assert_eq!(
                Balances::free_balance(AccountId::from(BOB)),
                100_000 * UNIT + 2 * UNIT
            );

            assert_ok!(StreamPayment::request_change(
                origin_of(ALICE.into()),
                0,
                ChangeKind::Suggestion,
                StreamConfig {
                    rate: 1 * UNIT,
                    asset_id: StreamPaymentAssetId::Native,
                    time_unit: TimeUnit::BlockNumber,
                    minimum_request_deadline_delay: 0,
                    soft_minimum_deposit: 0,
                },
                None,
            ));

            assert_ok!(StreamPayment::accept_requested_change(
                origin_of(BOB.into()),
                0,
                1, // nonce
                None,
            ));

            run_block();

            assert_ok!(StreamPayment::close_stream(origin_of(BOB.into()), 0));

            assert_eq!(
                Balances::free_balance(AccountId::from(BOB)),
                100_000 * UNIT + 3 * UNIT
            );
            assert_eq!(
                Balances::free_balance(AccountId::from(ALICE)),
                100_000 * UNIT - 3 * UNIT
            );
        });
}

#[test]
fn test_data_preserver_with_stream_payment() {
    ExtBuilder::default()
        .with_balances(vec![
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            use pallet_data_preservers::{
                AssignerParameterOf, ParaIdsFilter, Profile, ProfileMode, ProviderRequestOf,
            };

            let profile = Profile {
                url: b"test".to_vec().try_into().unwrap(),
                para_ids: ParaIdsFilter::AnyParaId,
                mode: ProfileMode::Bootnode,
                assignment_request: ProviderRequestOf::<Runtime>::StreamPayment {
                    config: StreamConfig {
                        time_unit: TimeUnit::BlockNumber,
                        asset_id: StreamPaymentAssetId::Native,
                        rate: 42,
                        minimum_request_deadline_delay: 0,
                        soft_minimum_deposit: 0,
                    },
                },
            };

            let para_id = ParaId::from(1002);
            let profile_id = 0u64;

            assert_ok!(ContainerRegistrar::register(
                origin_of(ALICE.into()),
                para_id,
                get_genesis_data_with_validation_code().0,
                Some(HeadData(vec![1u8, 1u8, 1u8]))
            ));

            assert_ok!(DataPreservers::create_profile(
                origin_of(BOB.into()),
                profile.clone(),
            ));

            // Start assignment
            assert_ok!(DataPreservers::start_assignment(
                origin_of(ALICE.into()),
                profile_id,
                para_id,
                AssignerParameterOf::<Runtime>::StreamPayment {
                    initial_deposit: 1_000
                }
            ));
            assert!(
                pallet_data_preservers::Assignments::<Runtime>::get(para_id).contains(&profile_id)
            );
            let profile = pallet_data_preservers::Profiles::<Runtime>::get(&profile_id)
                .expect("profile to exists");
            let (assigned_para_id, witness) = profile.assignment.expect("profile to be assigned");
            assert_eq!(assigned_para_id, para_id);
            assert_eq!(
                witness,
                PreserversAssignmentPaymentWitness::StreamPayment { stream_id: 0 }
            );
        });
}

#[test]
fn test_data_preserver_kind_needs_to_match() {
    ExtBuilder::default()
        .with_balances(vec![
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            use pallet_data_preservers::{
                AssignerParameterOf, ParaIdsFilter, Profile, ProfileMode, ProviderRequestOf,
            };

            let profile = Profile {
                url: b"test".to_vec().try_into().unwrap(),
                para_ids: ParaIdsFilter::AnyParaId,
                mode: ProfileMode::Bootnode,
                assignment_request: ProviderRequestOf::<Runtime>::Free,
            };

            let para_id = ParaId::from(1002);
            let profile_id = 0u64;

            assert_ok!(ContainerRegistrar::register(
                origin_of(ALICE.into()),
                para_id,
                get_genesis_data_with_validation_code().0,
                Some(HeadData(vec![1u8, 1u8, 1u8]))
            ));

            assert_ok!(DataPreservers::create_profile(
                origin_of(BOB.into()),
                profile.clone(),
            ));

            // Start assignment
            assert_err!(
                DataPreservers::start_assignment(
                    origin_of(ALICE.into()),
                    profile_id,
                    para_id,
                    AssignerParameterOf::<Runtime>::StreamPayment {
                        initial_deposit: 1_000
                    }
                ),
                pallet_data_preservers::Error::<Runtime>::AssignmentPaymentRequestParameterMismatch
            );
        });
}
