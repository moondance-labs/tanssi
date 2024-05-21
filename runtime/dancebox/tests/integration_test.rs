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
    common::*,
    cumulus_primitives_core::ParaId,
    dancebox_runtime::{
        RewardsCollatorCommission, StreamPayment, StreamPaymentAssetId, TimeUnit,
        TransactionPayment,
    },
    dp_consensus::runtime_decl_for_tanssi_authority_assignment_api::TanssiAuthorityAssignmentApiV1,
    dp_core::well_known_keys,
    frame_support::{assert_noop, assert_ok, BoundedVec},
    frame_system::ConsumedWeight,
    nimbus_primitives::NIMBUS_KEY_ID,
    pallet_author_noting::ContainerChainBlockInfo,
    pallet_author_noting_runtime_api::runtime_decl_for_author_noting_api::AuthorNotingApi,
    pallet_balances::Instance1,
    pallet_collator_assignment_runtime_api::runtime_decl_for_collator_assignment_api::CollatorAssignmentApi,
    pallet_migrations::Migration,
    pallet_pooled_staking::{
        traits::IsCandidateEligible, AllTargetPool, EligibleCandidate, PendingOperationKey,
        PendingOperationQuery, PoolsKey, SharesOrStake, TargetPool,
    },
    pallet_registrar_runtime_api::{
        runtime_decl_for_registrar_api::RegistrarApi, ContainerChainGenesisData,
    },
    parity_scale_codec::Encode,
    runtime_common::migrations::{
        MigrateConfigurationParathreads, MigrateServicesPaymentAddCollatorAssignmentCredits,
        RegistrarPendingVerificationValueToMap,
    },
    sp_consensus_aura::AURA_ENGINE_ID,
    sp_core::Get,
    sp_runtime::{
        traits::{BadOrigin, BlakeTwo256, OpaqueKeys},
        DigestItem, FixedU128,
    },
    sp_std::vec,
    staging_xcm::latest::prelude::*,
    test_relay_sproof_builder::{HeaderAs, ParaHeaderSproofBuilder, ParaHeaderSproofBuilderItem},
    tp_traits::SlotFrequency,
};

mod common;

const UNIT: Balance = 1_000_000_000_000_000_000;

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
        .with_collators(vec![
            // Remove ALICE and BOB from collators
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .build()
        .execute_with(|| {
            assert_eq!(
                Balances::usable_balance(AccountId::from(ALICE)),
                210_000 * UNIT,
            );
            assert_eq!(
                Balances::usable_balance(AccountId::from(BOB)),
                100_000 * UNIT,
            );
        });
}

#[test]
fn genesis_para_registrar() {
    ExtBuilder::default()
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            assert_eq!(
                Registrar::registered_para_ids(),
                vec![1001.into(), 1002.into()]
            );
        });
}

#[test]
fn genesis_para_registrar_deregister() {
    ExtBuilder::default()
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            assert_eq!(
                Registrar::registered_para_ids(),
                vec![1001.into(), 1002.into()]
            );

            run_to_block(2);
            assert_ok!(Registrar::deregister(root_origin(), 1002.into()), ());

            // Pending
            assert_eq!(
                Registrar::pending_registered_para_ids(),
                vec![(2u32, BoundedVec::try_from(vec![1001u32.into()]).unwrap())]
            );

            run_to_session(1);
            assert_eq!(
                Registrar::pending_registered_para_ids(),
                vec![(2u32, BoundedVec::try_from(vec![1001u32.into()]).unwrap())]
            );
            assert_eq!(
                Registrar::registered_para_ids(),
                vec![1001.into(), 1002.into()]
            );

            run_to_session(2);
            assert_eq!(Registrar::pending_registered_para_ids(), vec![]);
            assert_eq!(Registrar::registered_para_ids(), vec![1001.into()]);
        });
}

#[test]
fn genesis_para_registrar_runtime_api() {
    ExtBuilder::default()
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            assert_eq!(
                Registrar::registered_para_ids(),
                vec![1001.into(), 1002.into()]
            );
            assert_eq!(Runtime::registered_paras(), vec![1001.into(), 1002.into()]);

            run_to_block(2);
            assert_ok!(Registrar::deregister(root_origin(), 1002.into()), ());
            assert_eq!(Runtime::registered_paras(), vec![1001.into(), 1002.into()]);

            run_to_session(1);
            assert_eq!(
                Registrar::registered_para_ids(),
                vec![1001.into(), 1002.into()]
            );
            assert_eq!(Runtime::registered_paras(), vec![1001.into(), 1002.into()]);

            run_to_session(2);
            assert_eq!(Registrar::registered_para_ids(), vec![1001.into()]);
            assert_eq!(Runtime::registered_paras(), vec![1001.into()]);
        });
}

#[test]
fn genesis_para_registrar_container_chain_genesis_data_runtime_api() {
    let genesis_data_1001 = empty_genesis_data();
    let genesis_data_1002 = ContainerChainGenesisData {
        storage: vec![(b"key".to_vec(), b"value".to_vec()).into()],
        name: Default::default(),
        id: Default::default(),
        fork_id: Default::default(),
        extensions: vec![],
        properties: Default::default(),
    };
    ExtBuilder::default()
        .with_para_ids(vec![
            (1001, genesis_data_1001.clone(), vec![], u32::MAX, u32::MAX).into(),
            (1002, genesis_data_1002.clone(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            assert_eq!(
                Registrar::registered_para_ids(),
                vec![1001.into(), 1002.into()]
            );
            assert_eq!(Runtime::registered_paras(), vec![1001.into(), 1002.into()]);

            assert_eq!(
                Runtime::genesis_data(1001.into()).as_ref(),
                Some(&genesis_data_1001)
            );
            assert_eq!(
                Runtime::genesis_data(1002.into()).as_ref(),
                Some(&genesis_data_1002)
            );
            assert_eq!(Runtime::genesis_data(1003.into()).as_ref(), None);

            // This API cannot be used to get the genesis data of the orchestrator chain,
            // with id 100
            // TODO: where is that 100 defined?
            assert_eq!(Runtime::genesis_data(100.into()).as_ref(), None);

            run_to_block(2);
            assert_ok!(Registrar::deregister(root_origin(), 1002.into()), ());

            assert_eq!(Runtime::genesis_data(1002.into()).as_ref(), Some(&genesis_data_1002), "Deregistered container chain genesis data should not be removed until after 2 sessions");

            let genesis_data_1003 = ContainerChainGenesisData {
                storage: vec![(b"key3".to_vec(), b"value3".to_vec()).into()],
                name: Default::default(),
                id: Default::default(),
                fork_id: Default::default(),
                extensions: vec![],
                properties: Default::default(),
            };
            assert_ok!(
                Registrar::register(
                    origin_of(ALICE.into()),
                    1003.into(),
                    genesis_data_1003.clone()
                ),
                ()
            );

            // Registered container chains are inserted immediately
            assert_eq!(
                Runtime::genesis_data(1003.into()).as_ref(),
                Some(&genesis_data_1003)
            );

            // Deregistered container chain genesis data is removed after 2 sessions
            run_to_session(2u32);
            assert_eq!(Runtime::genesis_data(1002.into()).as_ref(), None);
        });
}

#[test]
fn test_author_collation_aura() {
    ExtBuilder::default()
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            run_to_block(5);
            // Assert current slot gets updated
            assert_eq!(current_slot(), 4u64);
            // slot 4, alice
            assert!(current_author() == AccountId::from(ALICE));

            run_to_block(6);

            assert_eq!(current_slot(), 5u64);
            // slot 5, bob
            assert!(current_author() == AccountId::from(BOB));
        });
}

#[test]
fn test_author_collation_aura_change_of_authorities_on_session() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);
            // Assert current slot gets updated
            assert_eq!(current_slot(), 1u64);
            assert!(current_author() == AccountId::from(BOB));

            // We change invulnerables
            // We first need to set the keys
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            let charlie_id = get_aura_id_from_seed(&AccountId::from(CHARLIE).to_string());
            let dave_id = get_aura_id_from_seed(&AccountId::from(DAVE).to_string());

            // Set CHARLIE and DAVE keys
            assert_ok!(Session::set_keys(
                origin_of(CHARLIE.into()),
                dancebox_runtime::SessionKeys {
                    nimbus: charlie_id.clone(),
                },
                vec![]
            ));
            assert_ok!(Session::set_keys(
                origin_of(DAVE.into()),
                dancebox_runtime::SessionKeys {
                    nimbus: dave_id.clone(),
                },
                vec![]
            ));

            // Change invulnerables
            assert_ok!(Invulnerables::remove_invulnerable(
                root_origin(),
                ALICE.into()
            ));
            assert_ok!(Invulnerables::remove_invulnerable(
                root_origin(),
                BOB.into()
            ));
            assert_ok!(Invulnerables::add_invulnerable(
                root_origin(),
                CHARLIE.into()
            ));
            assert_ok!(Invulnerables::add_invulnerable(root_origin(), DAVE.into()));

            // SESSION CHANGE. First session. it takes 2 sessions to see the change
            run_to_session(1u32);
            let author = get_orchestrator_current_author().unwrap();

            assert_eq!(current_author(), author);
            assert!(authorities() == vec![alice_id.clone(), bob_id.clone()]);

            // Invulnerables should have triggered on new session authorities change
            run_to_session(2u32);
            let author_after_changes = get_orchestrator_current_author().unwrap();

            assert_eq!(current_author(), author_after_changes);
            assert_eq!(authorities(), vec![charlie_id, dave_id]);
        });
}

#[test]
fn test_author_collation_aura_add_assigned_to_paras() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);
            // Assert current slot gets updated
            assert_eq!(current_slot(), 1u64);
            assert!(current_author() == AccountId::from(BOB));

            // We change invulnerables
            // We first need to set the keys
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            let charlie_id = get_aura_id_from_seed(&AccountId::from(CHARLIE).to_string());
            let dave_id = get_aura_id_from_seed(&AccountId::from(DAVE).to_string());

            // Set CHARLIE and DAVE keys
            assert_ok!(Session::set_keys(
                origin_of(CHARLIE.into()),
                dancebox_runtime::SessionKeys { nimbus: charlie_id },
                vec![]
            ));
            assert_ok!(Session::set_keys(
                origin_of(DAVE.into()),
                dancebox_runtime::SessionKeys { nimbus: dave_id },
                vec![]
            ));

            // Add new invulnerables
            assert_ok!(Invulnerables::add_invulnerable(
                root_origin(),
                CHARLIE.into()
            ));
            assert_ok!(Invulnerables::add_invulnerable(root_origin(), DAVE.into()));

            // SESSION CHANGE. First session. it takes 2 sessions to see the change
            run_to_session(1u32);
            let author = get_orchestrator_current_author().unwrap();

            assert_eq!(current_author(), author);
            assert_eq!(authorities(), vec![alice_id.clone(), bob_id.clone()]);

            // Invulnerables should have triggered on new session authorities change
            // However charlie and dave shoudl have gone to one para (1001)
            run_to_session(2u32);
            assert_eq!(authorities(), vec![alice_id, bob_id]);
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );
        });
}

#[test]
fn test_authors_without_paras() {
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
            // Assert current slot gets updated
            assert_eq!(current_slot(), 1u64);
            assert!(current_author() == AccountId::from(BOB));

            // Only Alice and Bob collate for our chain
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());
            let charlie_id = get_aura_id_from_seed(&AccountId::from(CHARLIE).to_string());
            let dave_id = get_aura_id_from_seed(&AccountId::from(DAVE).to_string());

            // It does not matter if we insert more collators, only two will be assigned
            assert_eq!(authorities(), vec![alice_id.clone(), bob_id.clone()]);

            // Set moondance collators to min 2 max 5
            assert_ok!(
                Configuration::set_min_orchestrator_collators(root_origin(), 2),
                ()
            );
            assert_ok!(
                Configuration::set_max_orchestrator_collators(root_origin(), 5),
                ()
            );

            run_to_session(2);
            assert_eq!(authorities(), vec![alice_id, bob_id, charlie_id, dave_id]);
        });
}

#[test]
fn test_authors_paras_inserted_a_posteriori() {
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
            // Assert current slot gets updated
            assert_eq!(current_slot(), 1u64);
            assert!(current_author() == AccountId::from(BOB));

            // Alice and Bob collate in our chain
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            assert_eq!(authorities(), vec![alice_id, bob_id]);

            assert_ok!(Registrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data()
            ));
            assert_ok!(DataPreservers::set_boot_nodes(
                origin_of(ALICE.into()),
                1001.into(),
                dummy_boot_nodes()
            ));
            assert_ok!(Registrar::mark_valid_for_collating(
                root_origin(),
                1001.into()
            ));
            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                1001.into(),
                block_credits_to_required_balance(1000, 1001.into())
            ));
            assert_ok!(Registrar::register(
                origin_of(ALICE.into()),
                1002.into(),
                empty_genesis_data()
            ));
            assert_ok!(DataPreservers::set_boot_nodes(
                origin_of(ALICE.into()),
                1002.into(),
                dummy_boot_nodes()
            ));
            assert_ok!(Registrar::mark_valid_for_collating(
                root_origin(),
                1002.into()
            ));
            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                1002.into(),
                block_credits_to_required_balance(1000, 1002.into())
            ));

            // Assignment should happen after 2 sessions
            run_to_session(1u32);
            let assignment = CollatorAssignment::collator_container_chain();
            assert!(assignment.container_chains.is_empty());
            run_to_session(2u32);

            // Charlie and Dave should be assigned to para 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );
        });
}

#[test]
fn test_authors_paras_inserted_a_posteriori_with_collators_already_assigned() {
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
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            min_orchestrator_collators: 2,
            max_orchestrator_collators: 5,
            collators_per_container: 2,
            full_rotation_period: 24,
            ..Default::default()
        })
        .build()
        .execute_with(|| {
            run_to_block(2);
            // Assert current slot gets updated
            assert_eq!(current_slot(), 1u64);
            assert!(current_author() == AccountId::from(BOB));

            // Alice and Bob collate in our chain
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());
            let charlie_id = get_aura_id_from_seed(&AccountId::from(CHARLIE).to_string());
            let dave_id = get_aura_id_from_seed(&AccountId::from(DAVE).to_string());

            assert_eq!(authorities(), vec![alice_id, bob_id, charlie_id, dave_id]);

            assert_ok!(Registrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data()
            ));
            assert_ok!(DataPreservers::set_boot_nodes(
                origin_of(ALICE.into()),
                1001.into(),
                dummy_boot_nodes()
            ));
            assert_ok!(Registrar::mark_valid_for_collating(
                root_origin(),
                1001.into()
            ));
            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                1001.into(),
                block_credits_to_required_balance(1000, 1001.into())
            ));

            // Assignment should happen after 2 sessions
            run_to_session(1u32);
            let assignment = CollatorAssignment::collator_container_chain();
            assert!(assignment.container_chains.is_empty());
            run_to_session(2u32);

            // Charlie and Dave are now assigned to para 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );
            assert_eq!(
                assignment.orchestrator_chain,
                vec![ALICE.into(), BOB.into()]
            );
        });
}

#[test]
fn test_paras_registered_but_zero_credits() {
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
            // Assert current slot gets updated
            assert_eq!(current_slot(), 1u64);
            assert!(current_author() == AccountId::from(BOB));

            // Alice and Bob collate in our chain
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            assert_eq!(authorities(), vec![alice_id, bob_id]);

            assert_ok!(Registrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data()
            ));
            assert_ok!(DataPreservers::set_boot_nodes(
                origin_of(ALICE.into()),
                1001.into(),
                dummy_boot_nodes()
            ));
            assert_ok!(Registrar::mark_valid_for_collating(
                root_origin(),
                1001.into()
            ));
            // Need to reset credits to 0 because now parachains are given free credits on register
            assert_ok!(ServicesPayment::set_block_production_credits(
                root_origin(),
                1001.into(),
                0
            ));

            // Assignment should happen after 2 sessions
            run_to_session(1u32);
            let assignment = CollatorAssignment::collator_container_chain();
            assert!(assignment.container_chains.is_empty());
            run_to_session(2u32);

            // Nobody should be assigned to para 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(assignment.container_chains.get(&1001u32.into()), None,);
        });
}

#[test]
fn test_paras_registered_but_not_enough_credits() {
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
            // Assert current slot gets updated
            assert_eq!(current_slot(), 1u64);
            assert!(current_author() == AccountId::from(BOB));

            // Alice and Bob collate in our chain
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            assert_eq!(authorities(), vec![alice_id, bob_id]);

            assert_ok!(Registrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data()
            ));
            assert_ok!(DataPreservers::set_boot_nodes(
                origin_of(ALICE.into()),
                1001.into(),
                dummy_boot_nodes()
            ));
            assert_ok!(Registrar::mark_valid_for_collating(
                root_origin(),
                1001.into()
            ));
            // Need to reset credits to 0 because now parachains are given free credits on register
            assert_ok!(ServicesPayment::set_block_production_credits(
                root_origin(),
                1001.into(),
                0
            ));
            // Purchase 1 credit less that what is needed
            let credits_1001 = dancebox_runtime::Period::get() - 1;
            assert_ok!(ServicesPayment::set_block_production_credits(
                root_origin(),
                1001.into(),
                credits_1001
            ));

            // Assignment should happen after 2 sessions
            run_to_session(1u32);
            let assignment = CollatorAssignment::collator_container_chain();
            assert!(assignment.container_chains.is_empty());
            run_to_session(2u32);
            // Nobody should be assigned to para 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(assignment.container_chains.get(&1001u32.into()), None);

            // Now purchase the missing block credit
            assert_ok!(ServicesPayment::set_block_production_credits(
                root_origin(),
                1001.into(),
                credits_1001 + 1
            ));

            run_to_session(4u32);
            // Charlie and Dave should be assigned to para 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );
        });
}

#[test]
fn test_paras_registered_but_only_credits_for_1_session() {
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
            // Assert current slot gets updated
            assert_eq!(current_slot(), 1u64);
            assert!(current_author() == AccountId::from(BOB));

            // Alice and Bob collate in our chain
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            assert_eq!(authorities(), vec![alice_id, bob_id]);

            assert_ok!(Registrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data()
            ));
            assert_ok!(DataPreservers::set_boot_nodes(
                origin_of(ALICE.into()),
                1001.into(),
                dummy_boot_nodes()
            ));
            assert_ok!(Registrar::mark_valid_for_collating(
                root_origin(),
                1001.into()
            ));
            // Need to reset credits to 0 because now parachains are given free credits on register
            assert_ok!(ServicesPayment::set_block_production_credits(
                root_origin(),
                1001.into(),
                0
            ));
            // Purchase only enough credits for 1 session
            let credits_1001 = dancebox_runtime::Period::get();
            assert_ok!(ServicesPayment::set_block_production_credits(
                root_origin(),
                1001.into(),
                credits_1001
            ));

            // Assignment should happen after 2 sessions
            run_to_session(1u32);
            let assignment = CollatorAssignment::collator_container_chain();
            assert!(assignment.container_chains.is_empty());
            run_to_session(2u32);
            // Charlie and Dave should be assigned to para 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );

            // No credits are consumed if the container chain is not producing blocks
            run_block();
            let credits =
                pallet_services_payment::BlockProductionCredits::<Runtime>::get(ParaId::from(1001))
                    .unwrap_or_default();
            assert_eq!(credits, credits_1001);

            // Simulate block inclusion from container chain 1001
            let mut sproof = ParaHeaderSproofBuilder::default();
            let slot: u64 = 5;
            let s = ParaHeaderSproofBuilderItem {
                para_id: 1001.into(),
                author_id: HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                    parent_hash: Default::default(),
                    number: 1,
                    state_root: Default::default(),
                    extrinsics_root: Default::default(),
                    digest: sp_runtime::generic::Digest {
                        logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                    },
                }),
            };
            sproof.items.push(s);
            set_author_noting_inherent_data(sproof);

            run_block();
            let credits =
                pallet_services_payment::BlockProductionCredits::<Runtime>::get(ParaId::from(1001))
                    .unwrap_or_default();
            assert_eq!(credits, credits_1001 - 1);

            run_to_session(4u32);
            // Nobody should be assigned to para 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(assignment.container_chains.get(&1001u32.into()), None,);

            // The container chain only produced one block, so it only consumed one block credit.
            // (it could have produced more blocks, but at most it would have consumed `Period::get()` credits)
            let credits =
                pallet_services_payment::BlockProductionCredits::<Runtime>::get(ParaId::from(1001))
                    .unwrap_or_default();
            assert_eq!(credits, credits_1001 - 1);
        });
}

#[test]
fn test_parachains_deregister_collators_re_assigned() {
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
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);
            // Assert current slot gets updated
            assert_eq!(current_slot(), 1u64);
            assert!(current_author() == AccountId::from(BOB));

            // Alice and Bob are authorities
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            assert_eq!(authorities(), vec![alice_id, bob_id]);

            // Charlie and Dave to 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );

            assert_ok!(Registrar::deregister(root_origin(), 1001.into()), ());

            // Assignment should happen after 2 sessions
            run_to_session(1u32);

            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );

            run_to_session(2u32);

            // Charlie and Dave should be assigne dot para 1002 this time
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1002u32.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );
        });
}

#[test]
fn test_parachains_deregister_collators_config_change_reassigned() {
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
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);
            // Assert current slot gets updated
            assert_eq!(current_slot(), 1u64);
            assert!(current_author() == AccountId::from(BOB));

            // Alice and Bob are authorities
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            assert_eq!(authorities(), vec![alice_id, bob_id]);

            // Set orchestrator collators to 1
            assert_ok!(
                Configuration::set_max_orchestrator_collators(root_origin(), 1),
                ()
            );

            // Set container chain collators to 3
            assert_ok!(
                Configuration::set_collators_per_container(root_origin(), 3),
                ()
            );

            // Charlie and Dave to 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );

            // Assignment should happen after 2 sessions
            run_to_session(1u32);

            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );

            run_to_session(2u32);

            // Charlie, Dave and BOB should be assigne dot para 1001 this time
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![CHARLIE.into(), DAVE.into(), BOB.into()]
            );

            assert_eq!(assignment.orchestrator_chain, vec![ALICE.into()]);
        });
}

#[test]
fn test_orchestrator_collators_with_non_sufficient_collators() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
        ])
        .with_collators(vec![(AccountId::from(ALICE), 210 * UNIT)])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);
            // Assert current slot gets updated
            assert_eq!(current_slot(), 1u64);
            assert!(current_author() == AccountId::from(ALICE));

            // Alice and Bob are authorities
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());

            assert_eq!(authorities(), vec![alice_id]);
        });
}

#[test]
fn test_configuration_on_session_change() {
    ExtBuilder::default().build().execute_with(|| {
        assert_eq!(Configuration::config().max_collators, 100);
        assert_eq!(Configuration::config().min_orchestrator_collators, 2);
        assert_eq!(Configuration::config().collators_per_container, 2);

        assert_ok!(Configuration::set_max_collators(root_origin(), 50), ());
        run_to_session(1u32);

        assert_ok!(
            Configuration::set_min_orchestrator_collators(root_origin(), 20),
            ()
        );
        assert_eq!(Configuration::config().max_collators, 100);
        assert_eq!(Configuration::config().min_orchestrator_collators, 2);
        assert_eq!(Configuration::config().collators_per_container, 2);

        run_to_session(2u32);
        assert_ok!(
            Configuration::set_collators_per_container(root_origin(), 10),
            ()
        );
        assert_eq!(Configuration::config().max_collators, 50);
        assert_eq!(Configuration::config().min_orchestrator_collators, 2);
        assert_eq!(Configuration::config().collators_per_container, 2);

        run_to_session(3u32);

        assert_eq!(Configuration::config().max_collators, 50);
        assert_eq!(Configuration::config().min_orchestrator_collators, 20);
        assert_eq!(Configuration::config().collators_per_container, 2);

        run_to_session(4u32);

        assert_eq!(Configuration::config().max_collators, 50);
        assert_eq!(Configuration::config().min_orchestrator_collators, 20);
        assert_eq!(Configuration::config().collators_per_container, 10);
    });
}

#[test]
fn test_author_collation_aura_add_assigned_to_paras_runtime_api() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);
            // Assert current slot gets updated
            assert_eq!(current_slot(), 1u64);
            assert!(current_author() == AccountId::from(BOB));
            assert_eq!(
                Runtime::parachain_collators(100.into()),
                Some(vec![ALICE.into(), BOB.into()])
            );
            assert_eq!(Runtime::parachain_collators(1001.into()), Some(vec![]));
            assert_eq!(
                Runtime::current_collator_parachain_assignment(ALICE.into()),
                Some(100.into())
            );
            assert_eq!(
                Runtime::future_collator_parachain_assignment(ALICE.into()),
                Some(100.into())
            );
            assert_eq!(
                Runtime::current_collator_parachain_assignment(CHARLIE.into()),
                None
            );
            assert_eq!(
                Runtime::future_collator_parachain_assignment(CHARLIE.into()),
                None
            );

            // We change invulnerables
            // We first need to set the keys
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            let charlie_id = get_aura_id_from_seed(&AccountId::from(CHARLIE).to_string());
            let dave_id = get_aura_id_from_seed(&AccountId::from(DAVE).to_string());

            // Set CHARLIE and DAVE keys
            assert_ok!(Session::set_keys(
                origin_of(CHARLIE.into()),
                dancebox_runtime::SessionKeys { nimbus: charlie_id },
                vec![]
            ));
            assert_ok!(Session::set_keys(
                origin_of(DAVE.into()),
                dancebox_runtime::SessionKeys { nimbus: dave_id },
                vec![]
            ));

            // Set new invulnerables
            assert_ok!(Invulnerables::add_invulnerable(
                root_origin(),
                CHARLIE.into()
            ));
            assert_ok!(Invulnerables::add_invulnerable(root_origin(), DAVE.into()));

            // SESSION CHANGE. First session. it takes 2 sessions to see the change
            run_to_session(1u32);
            let author = get_orchestrator_current_author().unwrap();

            assert_eq!(current_author(), author);
            assert_eq!(authorities(), vec![alice_id.clone(), bob_id.clone()]);
            assert_eq!(
                Runtime::parachain_collators(100.into()),
                Some(vec![ALICE.into(), BOB.into()])
            );
            assert_eq!(Runtime::parachain_collators(1001.into()), Some(vec![]));
            assert_eq!(
                Runtime::current_collator_parachain_assignment(CHARLIE.into()),
                None
            );
            assert_eq!(
                Runtime::future_collator_parachain_assignment(CHARLIE.into()),
                Some(1001.into())
            );

            // Invulnerables should have triggered on new session authorities change
            // However charlie and dave shoudl have gone to one para (1001)
            run_to_session(2u32);
            assert_eq!(authorities(), vec![alice_id, bob_id]);
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );

            assert_eq!(
                Runtime::parachain_collators(100.into()),
                Some(vec![ALICE.into(), BOB.into()])
            );
            assert_eq!(
                Runtime::parachain_collators(1001.into()),
                Some(vec![CHARLIE.into(), DAVE.into()])
            );
            assert_eq!(
                Runtime::current_collator_parachain_assignment(CHARLIE.into()),
                Some(1001.into())
            );
            assert_eq!(
                Runtime::future_collator_parachain_assignment(CHARLIE.into()),
                Some(1001.into())
            );

            // Remove BOB
            assert_ok!(Invulnerables::remove_invulnerable(
                root_origin(),
                BOB.into()
            ));

            run_to_session(3u32);
            assert_eq!(
                Runtime::parachain_collators(100.into()),
                Some(vec![ALICE.into(), BOB.into()])
            );
            assert_eq!(
                Runtime::parachain_collators(1001.into()),
                Some(vec![CHARLIE.into(), DAVE.into()])
            );
            assert_eq!(
                Runtime::current_collator_parachain_assignment(BOB.into()),
                Some(100.into())
            );
            assert_eq!(
                Runtime::future_collator_parachain_assignment(BOB.into()),
                None
            );

            run_to_session(4u32);
            assert_eq!(
                Runtime::parachain_collators(100.into()),
                Some(vec![ALICE.into(), CHARLIE.into()])
            );
            assert_eq!(Runtime::parachain_collators(1001.into()), Some(vec![]));
            assert_eq!(
                Runtime::current_collator_parachain_assignment(BOB.into()),
                None
            );
            assert_eq!(
                Runtime::future_collator_parachain_assignment(BOB.into()),
                None
            );
        });
}

#[test]
fn test_consensus_runtime_api() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);

            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            let charlie_id = get_aura_id_from_seed(&AccountId::from(CHARLIE).to_string());
            let dave_id = get_aura_id_from_seed(&AccountId::from(DAVE).to_string());

            assert_eq!(
                Runtime::para_id_authorities(100.into()),
                Some(vec![alice_id.clone(), bob_id.clone()])
            );
            assert_eq!(Runtime::para_id_authorities(1001.into()), Some(vec![]));
            assert_eq!(
                Runtime::check_para_id_assignment(alice_id.clone()),
                Some(100.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment(bob_id.clone()),
                Some(100.into())
            );
            assert_eq!(Runtime::check_para_id_assignment(charlie_id.clone()), None);
            assert_eq!(Runtime::check_para_id_assignment(dave_id.clone()), None);

            // Set CHARLIE and DAVE keys
            assert_ok!(Session::set_keys(
                origin_of(CHARLIE.into()),
                dancebox_runtime::SessionKeys {
                    nimbus: charlie_id.clone(),
                },
                vec![]
            ));
            assert_ok!(Session::set_keys(
                origin_of(DAVE.into()),
                dancebox_runtime::SessionKeys {
                    nimbus: dave_id.clone(),
                },
                vec![]
            ));

            // Set new invulnerables
            assert_ok!(Invulnerables::add_invulnerable(
                root_origin(),
                CHARLIE.into()
            ));
            assert_ok!(Invulnerables::add_invulnerable(root_origin(), DAVE.into()));

            run_to_session(2u32);
            assert_eq!(
                Runtime::para_id_authorities(100.into()),
                Some(vec![alice_id.clone(), bob_id.clone()])
            );
            assert_eq!(
                Runtime::para_id_authorities(1001.into()),
                Some(vec![charlie_id.clone(), dave_id.clone()])
            );
            assert_eq!(
                Runtime::check_para_id_assignment(alice_id),
                Some(100.into())
            );
            assert_eq!(Runtime::check_para_id_assignment(bob_id), Some(100.into()));
            assert_eq!(
                Runtime::check_para_id_assignment(charlie_id),
                Some(1001.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment(dave_id),
                Some(1001.into())
            );
        });
}

#[test]
fn test_consensus_runtime_api_session_changes() {
    // The test shoul return always the assiignment on the next epoch
    // Meaning that we need to see before the session change block
    // if we can predict correctly
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);

            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            let charlie_id = get_aura_id_from_seed(&AccountId::from(CHARLIE).to_string());
            let dave_id = get_aura_id_from_seed(&AccountId::from(DAVE).to_string());

            assert_eq!(
                Runtime::para_id_authorities(100.into()),
                Some(vec![alice_id.clone(), bob_id.clone()])
            );
            assert_eq!(Runtime::para_id_authorities(1001.into()), Some(vec![]));
            assert_eq!(
                Runtime::check_para_id_assignment(alice_id.clone()),
                Some(100.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment(bob_id.clone()),
                Some(100.into())
            );
            assert_eq!(Runtime::check_para_id_assignment(charlie_id.clone()), None);
            assert_eq!(Runtime::check_para_id_assignment(dave_id.clone()), None);

            // Set CHARLIE and DAVE keys
            assert_ok!(Session::set_keys(
                origin_of(CHARLIE.into()),
                dancebox_runtime::SessionKeys {
                    nimbus: charlie_id.clone(),
                },
                vec![]
            ));
            assert_ok!(Session::set_keys(
                origin_of(DAVE.into()),
                dancebox_runtime::SessionKeys {
                    nimbus: dave_id.clone(),
                },
                vec![]
            ));

            // Set new invulnerables
            assert_ok!(Invulnerables::add_invulnerable(
                root_origin(),
                CHARLIE.into()
            ));
            assert_ok!(Invulnerables::add_invulnerable(root_origin(), DAVE.into()));

            let session_two_edge = dancebox_runtime::Period::get() * 2;
            // Let's run just 2 blocks before the session 2 change first
            // Prediction should still be identical, as we are not in the
            // edge of a session change
            run_to_block(session_two_edge - 2);

            assert_eq!(
                Runtime::para_id_authorities(100.into()),
                Some(vec![alice_id.clone(), bob_id.clone()])
            );
            assert_eq!(Runtime::para_id_authorities(1001.into()), Some(vec![]));
            assert_eq!(
                Runtime::check_para_id_assignment(alice_id.clone()),
                Some(100.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment(bob_id.clone()),
                Some(100.into())
            );
            assert_eq!(Runtime::check_para_id_assignment(charlie_id.clone()), None);
            assert_eq!(Runtime::check_para_id_assignment(dave_id.clone()), None);

            // Now we run to session edge -1. Here we should predict already with
            // authorities of the next block!
            run_to_block(session_two_edge - 1);
            assert_eq!(
                Runtime::para_id_authorities(100.into()),
                Some(vec![alice_id.clone(), bob_id.clone()])
            );
            assert_eq!(
                Runtime::para_id_authorities(1001.into()),
                Some(vec![charlie_id.clone(), dave_id.clone()])
            );
            assert_eq!(
                Runtime::check_para_id_assignment(alice_id),
                Some(100.into())
            );
            assert_eq!(Runtime::check_para_id_assignment(bob_id), Some(100.into()));
            assert_eq!(
                Runtime::check_para_id_assignment(charlie_id),
                Some(1001.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment(dave_id),
                Some(1001.into())
            );
        });
}

#[test]
fn test_consensus_runtime_api_next_session() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);

            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            let charlie_id = get_aura_id_from_seed(&AccountId::from(CHARLIE).to_string());
            let dave_id = get_aura_id_from_seed(&AccountId::from(DAVE).to_string());

            assert_eq!(
                Runtime::para_id_authorities(100.into()),
                Some(vec![alice_id.clone(), bob_id.clone()])
            );
            assert_eq!(Runtime::para_id_authorities(1001.into()), Some(vec![]));
            assert_eq!(
                Runtime::check_para_id_assignment(alice_id.clone()),
                Some(100.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment(bob_id.clone()),
                Some(100.into())
            );
            assert_eq!(Runtime::check_para_id_assignment(charlie_id.clone()), None);
            assert_eq!(Runtime::check_para_id_assignment(dave_id.clone()), None);

            // In the next session the assignment will not change
            assert_eq!(
                Runtime::check_para_id_assignment_next_session(alice_id.clone()),
                Some(100.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment_next_session(bob_id.clone()),
                Some(100.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment_next_session(charlie_id.clone()),
                None,
            );
            assert_eq!(
                Runtime::check_para_id_assignment_next_session(dave_id.clone()),
                None,
            );

            // Set CHARLIE and DAVE keys
            assert_ok!(Session::set_keys(
                origin_of(CHARLIE.into()),
                dancebox_runtime::SessionKeys {
                    nimbus: charlie_id.clone(),
                },
                vec![]
            ));
            assert_ok!(Session::set_keys(
                origin_of(DAVE.into()),
                dancebox_runtime::SessionKeys {
                    nimbus: dave_id.clone(),
                },
                vec![]
            ));

            // Set new invulnerables
            assert_ok!(Invulnerables::add_invulnerable(
                root_origin(),
                CHARLIE.into()
            ));
            assert_ok!(Invulnerables::add_invulnerable(root_origin(), DAVE.into()));

            let session_two_edge = dancebox_runtime::Period::get() * 2;
            // Let's run just 2 blocks before the session 2 change first
            // Prediction should still be identical, as we are not in the
            // edge of a session change
            run_to_block(session_two_edge - 2);

            assert_eq!(
                Runtime::para_id_authorities(100.into()),
                Some(vec![alice_id.clone(), bob_id.clone()])
            );
            assert_eq!(Runtime::para_id_authorities(1001.into()), Some(vec![]));
            assert_eq!(
                Runtime::check_para_id_assignment(alice_id.clone()),
                Some(100.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment(bob_id.clone()),
                Some(100.into())
            );
            assert_eq!(Runtime::check_para_id_assignment(charlie_id.clone()), None);
            assert_eq!(Runtime::check_para_id_assignment(dave_id.clone()), None);

            // But in the next session the assignment will change, so future api returns different value
            assert_eq!(
                Runtime::check_para_id_assignment_next_session(alice_id.clone()),
                Some(100.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment_next_session(bob_id.clone()),
                Some(100.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment_next_session(charlie_id.clone()),
                Some(1001.into()),
            );
            assert_eq!(
                Runtime::check_para_id_assignment_next_session(dave_id.clone()),
                Some(1001.into()),
            );

            // Now we run to session edge -1. Here we should predict already with
            // authorities of the next block!
            run_to_block(session_two_edge - 1);
            assert_eq!(
                Runtime::para_id_authorities(100.into()),
                Some(vec![alice_id.clone(), bob_id.clone()])
            );
            assert_eq!(
                Runtime::para_id_authorities(1001.into()),
                Some(vec![charlie_id.clone(), dave_id.clone()])
            );
            assert_eq!(
                Runtime::check_para_id_assignment(alice_id.clone()),
                Some(100.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment(bob_id.clone()),
                Some(100.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment(charlie_id.clone()),
                Some(1001.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment(dave_id.clone()),
                Some(1001.into())
            );

            // check_para_id_assignment_next_session returns the same value as check_para_id_assignment
            // because we are on a session boundary
            assert_eq!(
                Runtime::check_para_id_assignment_next_session(alice_id),
                Some(100.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment_next_session(bob_id),
                Some(100.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment_next_session(charlie_id),
                Some(1001.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment_next_session(dave_id),
                Some(1001.into())
            );
        });
}

#[test]
fn test_author_noting_self_para_id_not_noting() {
    ExtBuilder::default().build().execute_with(|| {
        let mut sproof = ParaHeaderSproofBuilder::default();
        let slot: u64 = 5;
        let self_para = parachain_info::Pallet::<Runtime>::get();
        let s = ParaHeaderSproofBuilderItem {
            para_id: self_para,
            author_id: HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                parent_hash: Default::default(),
                number: Default::default(),
                state_root: Default::default(),
                extrinsics_root: Default::default(),
                digest: sp_runtime::generic::Digest {
                    logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                },
            }),
        };
        sproof.items.push(s);

        set_author_noting_inherent_data(sproof);

        assert_eq!(AuthorNoting::latest_author(self_para), None);
    });
}

#[test]
fn test_author_noting_not_self_para() {
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
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            let mut sproof = ParaHeaderSproofBuilder::default();
            let slot: u64 = 5;
            let other_para: ParaId = 1001u32.into();

            // Charlie and Dave to 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );

            let s = ParaHeaderSproofBuilderItem {
                para_id: other_para,
                author_id: HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                    parent_hash: Default::default(),
                    number: 1,
                    state_root: Default::default(),
                    extrinsics_root: Default::default(),
                    digest: sp_runtime::generic::Digest {
                        logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                    },
                }),
            };
            sproof.items.push(s);

            set_author_noting_inherent_data(sproof);

            assert_eq!(
                AuthorNoting::latest_author(other_para),
                Some(ContainerChainBlockInfo {
                    block_number: 1,
                    author: AccountId::from(DAVE),
                    latest_slot_number: 0.into(),
                })
            );
        });
}

#[test]
fn test_author_noting_set_author_and_kill_author_data() {
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
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            let other_para: ParaId = 1001u32.into();

            assert_ok!(AuthorNoting::set_author(
                root_origin(),
                other_para,
                1,
                AccountId::from(DAVE),
                1.into()
            ));

            assert_eq!(
                AuthorNoting::latest_author(other_para),
                Some(ContainerChainBlockInfo {
                    block_number: 1,
                    author: AccountId::from(DAVE),
                    latest_slot_number: 1.into(),
                })
            );

            assert_ok!(AuthorNoting::kill_author_data(root_origin(), other_para));

            assert_eq!(AuthorNoting::latest_author(other_para), None);
        });
}

#[test]
fn test_author_noting_set_author_and_kill_author_data_bad_origin() {
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
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            let other_para: ParaId = 1001u32.into();

            assert_noop!(
                AuthorNoting::set_author(
                    origin_of(ALICE.into()),
                    other_para,
                    1,
                    AccountId::from(DAVE),
                    1.into()
                ),
                BadOrigin
            );

            assert_noop!(
                AuthorNoting::kill_author_data(origin_of(ALICE.into()), other_para),
                BadOrigin
            );
        });
}

#[test]
fn test_author_noting_runtime_api() {
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
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            let mut sproof = ParaHeaderSproofBuilder::default();
            let slot: u64 = 5;
            let other_para: ParaId = 1001u32.into();

            // Charlie and Dave to 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );

            let s = ParaHeaderSproofBuilderItem {
                para_id: other_para,
                author_id: HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                    parent_hash: Default::default(),
                    number: 1,
                    state_root: Default::default(),
                    extrinsics_root: Default::default(),
                    digest: sp_runtime::generic::Digest {
                        logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                    },
                }),
            };
            sproof.items.push(s);

            set_author_noting_inherent_data(sproof);

            assert_eq!(
                AuthorNoting::latest_author(other_para),
                Some(ContainerChainBlockInfo {
                    block_number: 1,
                    author: AccountId::from(DAVE),
                    latest_slot_number: 0.into(),
                })
            );

            assert_eq!(
                Runtime::latest_author(other_para),
                Some(AccountId::from(DAVE))
            );
            assert_eq!(Runtime::latest_block_number(other_para), Some(1));
        });
}

#[test]
fn test_collator_assignment_rotation() {
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
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            // Charlie and Dave to 1001
            let assignment = CollatorAssignment::collator_container_chain();
            let initial_assignment = assignment.clone();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );

            let rotation_period = Configuration::config().full_rotation_period;
            run_to_session(rotation_period - 2);
            set_parachain_inherent_data_random_seed([1; 32]);
            run_block();

            assert!(CollatorAssignment::pending_collator_container_chain().is_none());

            run_to_session(rotation_period - 1);
            assert_eq!(
                CollatorAssignment::collator_container_chain(),
                initial_assignment,
            );
            assert!(CollatorAssignment::pending_collator_container_chain().is_some());

            run_to_session(rotation_period);
            // Assignment changed
            assert_ne!(
                CollatorAssignment::collator_container_chain(),
                initial_assignment,
            );
        });
}

#[test]
fn session_keys_key_type_id() {
    assert_eq!(
        dancebox_runtime::SessionKeys::key_ids(),
        vec![NIMBUS_KEY_ID]
    );
}

#[test]
fn test_session_keys_with_authority_mapping() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);
            let key_mapping_session_0 = AuthorityMapping::authority_id_mapping(0).unwrap();
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());
            let alice_id_2 = get_aura_id_from_seed("ALICE2");
            let bob_id_2 = get_aura_id_from_seed("BOB2");

            assert_eq!(key_mapping_session_0.len(), 2);
            assert_eq!(key_mapping_session_0.get(&alice_id), Some(&ALICE.into()));
            assert_eq!(key_mapping_session_0.get(&bob_id), Some(&BOB.into()));

            // Everything should match to aura
            assert_eq!(authorities(), vec![alice_id.clone(), bob_id.clone()]);

            // Change Alice and Bob keys to something different
            // for now lets change it to alice_2 and bob_2
            assert_ok!(Session::set_keys(
                origin_of(ALICE.into()),
                dancebox_runtime::SessionKeys {
                    nimbus: alice_id_2.clone(),
                },
                vec![]
            ));
            assert_ok!(Session::set_keys(
                origin_of(BOB.into()),
                dancebox_runtime::SessionKeys {
                    nimbus: bob_id_2.clone(),
                },
                vec![]
            ));

            run_to_session(1u32);
            let key_mapping_session_0 = AuthorityMapping::authority_id_mapping(0).unwrap();
            assert_eq!(key_mapping_session_0.len(), 2);
            assert_eq!(key_mapping_session_0.get(&alice_id), Some(&ALICE.into()));
            assert_eq!(key_mapping_session_0.get(&bob_id), Some(&BOB.into()));

            let key_mapping_session_1 = AuthorityMapping::authority_id_mapping(1).unwrap();
            assert_eq!(key_mapping_session_1.len(), 2);
            assert_eq!(key_mapping_session_1.get(&alice_id), Some(&ALICE.into()));
            assert_eq!(key_mapping_session_1.get(&bob_id), Some(&BOB.into()));

            // Everything should match to aura
            assert_eq!(authorities(), vec![alice_id.clone(), bob_id.clone()]);
            //

            run_to_session(2u32);
            assert!(AuthorityMapping::authority_id_mapping(0).is_none());

            let key_mapping_session_1 = AuthorityMapping::authority_id_mapping(1).unwrap();
            assert_eq!(key_mapping_session_1.len(), 2);
            assert_eq!(key_mapping_session_1.get(&alice_id), Some(&ALICE.into()));
            assert_eq!(key_mapping_session_1.get(&bob_id), Some(&BOB.into()));

            let key_mapping_session_2 = AuthorityMapping::authority_id_mapping(2).unwrap();
            assert_eq!(key_mapping_session_2.len(), 2);
            assert_eq!(key_mapping_session_2.get(&alice_id_2), Some(&ALICE.into()));
            assert_eq!(key_mapping_session_2.get(&bob_id_2), Some(&BOB.into()));

            // Everything should match to aura
            assert_eq!(authorities(), vec![alice_id_2, bob_id_2]);
        });
}

#[test]
fn test_session_keys_with_authority_assignment() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());
            let alice_id_2 = get_aura_id_from_seed("ALICE2");
            let bob_id_2 = get_aura_id_from_seed("BOB2");

            let key_mapping_session_0 = AuthorityAssignment::collator_container_chain(0).unwrap();
            assert_eq!(
                key_mapping_session_0.orchestrator_chain,
                vec![alice_id.clone(), bob_id.clone()],
            );
            assert_eq!(
                CollatorAssignment::collator_container_chain().orchestrator_chain,
                vec![AccountId::from(ALICE), AccountId::from(BOB)],
            );

            let key_mapping_session_1 = AuthorityAssignment::collator_container_chain(1).unwrap();
            assert_eq!(key_mapping_session_1, key_mapping_session_0,);
            let old_assignment_session_1 =
                CollatorAssignment::pending_collator_container_chain().unwrap();
            assert_eq!(
                old_assignment_session_1,
                CollatorAssignment::collator_container_chain(),
            );

            let key_mapping_session_2 = AuthorityAssignment::collator_container_chain(2);
            assert!(key_mapping_session_2.is_none());

            // Everything should match to aura
            assert_eq!(authorities(), vec![alice_id.clone(), bob_id.clone()]);

            // Change Alice and Bob keys to something different
            // for now lets change it to alice_2 and bob_2
            assert_ok!(Session::set_keys(
                origin_of(ALICE.into()),
                dancebox_runtime::SessionKeys {
                    nimbus: alice_id_2.clone(),
                },
                vec![]
            ));
            assert_ok!(Session::set_keys(
                origin_of(BOB.into()),
                dancebox_runtime::SessionKeys {
                    nimbus: bob_id_2.clone(),
                },
                vec![]
            ));

            run_to_session(1u32);
            let old_key_mapping_session_1 = key_mapping_session_1;

            // Session 0 got removed
            let key_mapping_session_0 = AuthorityAssignment::collator_container_chain(0);
            assert!(key_mapping_session_0.is_none());

            // The values at session 1 did not change
            let key_mapping_session_1 = AuthorityAssignment::collator_container_chain(1).unwrap();
            assert_eq!(key_mapping_session_1, old_key_mapping_session_1,);
            assert_eq!(
                CollatorAssignment::collator_container_chain(),
                old_assignment_session_1,
            );

            // Session 2 uses the new keys
            let key_mapping_session_2 = AuthorityAssignment::collator_container_chain(2).unwrap();
            assert_eq!(
                key_mapping_session_2.orchestrator_chain,
                vec![alice_id_2.clone(), bob_id_2.clone()],
            );
            assert_eq!(CollatorAssignment::pending_collator_container_chain(), None);

            let key_mapping_session_3 = AuthorityAssignment::collator_container_chain(3);
            assert!(key_mapping_session_3.is_none());

            // Everything should match to aura
            assert_eq!(authorities(), vec![alice_id, bob_id]);

            run_to_session(2u32);

            // Session 1 got removed
            let key_mapping_session_1 = AuthorityAssignment::collator_container_chain(1);
            assert!(key_mapping_session_1.is_none());

            // Session 2 uses the new keys
            let key_mapping_session_2 = AuthorityAssignment::collator_container_chain(2).unwrap();
            assert_eq!(
                key_mapping_session_2.orchestrator_chain,
                vec![alice_id_2.clone(), bob_id_2.clone()],
            );
            assert_eq!(
                old_assignment_session_1,
                CollatorAssignment::collator_container_chain(),
            );

            // Session 3 uses the new keys
            let key_mapping_session_3 = AuthorityAssignment::collator_container_chain(3).unwrap();
            assert_eq!(
                key_mapping_session_3.orchestrator_chain,
                vec![alice_id_2.clone(), bob_id_2.clone()],
            );
            assert_eq!(CollatorAssignment::pending_collator_container_chain(), None);

            let key_mapping_session_4 = AuthorityAssignment::collator_container_chain(4);
            assert!(key_mapping_session_4.is_none());

            // Everything should match to aura
            assert_eq!(authorities(), vec![alice_id_2, bob_id_2]);
        });
}

fn call_transfer(
    dest: sp_runtime::MultiAddress<sp_runtime::AccountId32, ()>,
    value: u128,
) -> RuntimeCall {
    RuntimeCall::Balances(pallet_balances::Call::transfer_allow_death { dest, value })
}

#[test]
fn test_proxy_any() {
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
            run_to_block(2);

            let delay = 0;
            assert_ok!(Proxy::add_proxy(
                origin_of(ALICE.into()),
                AccountId::from(BOB).into(),
                ProxyType::Any,
                delay
            ));

            let balance_before = System::account(AccountId::from(BOB)).data.free;
            let call = Box::new(call_transfer(AccountId::from(BOB).into(), 200_000));
            assert_ok!(Proxy::proxy(
                origin_of(BOB.into()),
                AccountId::from(ALICE).into(),
                None,
                call
            ));
            let balance_after = System::account(AccountId::from(BOB)).data.free;

            assert_eq!(balance_after, balance_before + 200_000);
        });
}

#[test]
fn test_proxy_non_transfer() {
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
            run_to_block(2);

            let delay = 0;
            assert_ok!(Proxy::add_proxy(
                origin_of(ALICE.into()),
                AccountId::from(BOB).into(),
                ProxyType::NonTransfer,
                delay
            ));

            let balance_before = System::account(AccountId::from(BOB)).data.free;
            let call = Box::new(call_transfer(AccountId::from(BOB).into(), 200_000));
            // The extrinsic succeeds but the call is filtered, so no transfer is actually done
            assert_ok!(Proxy::proxy(
                origin_of(BOB.into()),
                AccountId::from(ALICE).into(),
                None,
                call
            ));
            let balance_after = System::account(AccountId::from(BOB)).data.free;

            assert_eq!(balance_after, balance_before);
        });
}

#[test]
fn test_proxy_utility() {
    // All proxy types should be able to use Utility pallet, but we ensure
    // subcalls don't allow to circumvent filters.

    // Dummy match to ensure we update this test when adding new proxy types.
    match ProxyType::Any {
        ProxyType::Any
        | ProxyType::NonTransfer
        | ProxyType::Governance
        | ProxyType::Staking
        | ProxyType::CancelProxy
        | ProxyType::Balances
        | ProxyType::Registrar
        | ProxyType::SudoRegistrar
        | ProxyType::SessionKeyManagement => (),
    };

    // All except for any
    let proxy_types = &[
        ProxyType::NonTransfer,
        ProxyType::Governance,
        ProxyType::Staking,
        ProxyType::CancelProxy,
        ProxyType::Balances,
        ProxyType::Registrar,
        ProxyType::SudoRegistrar,
        ProxyType::SessionKeyManagement,
    ];

    for &proxy_type in proxy_types {
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
            ])
            .with_sudo(AccountId::from(ALICE))
            .build()
            .execute_with(|| {
                assert_ok!(Proxy::add_proxy(
                    origin_of(ALICE.into()),
                    AccountId::from(BOB).into(),
                    proxy_type,
                    0
                ));

                let free_balance = Balances::free_balance(AccountId::from(BOB));

                assert_ok!(Proxy::proxy(
                    origin_of(BOB.into()),
                    AccountId::from(ALICE).into(),
                    None,
                    Box::new(
                        pallet_sudo::Call::sudo {
                            call: Box::new(
                                pallet_utility::Call::batch {
                                    calls: vec![pallet_balances::Call::force_set_balance {
                                        who: AccountId::from(BOB).into(),
                                        new_free: 42424242424242
                                    }
                                    .into()]
                                }
                                .into()
                            )
                        }
                        .into()
                    )
                ));

                assert_eq!(Balances::free_balance(AccountId::from(BOB)), free_balance);
            });
    }
}

#[test]
fn check_well_known_keys() {
    use frame_support::traits::PalletInfo;

    // Pallet is named "Paras" in Polkadot.
    assert_eq!(
        well_known_keys::PARAS_HEADS_INDEX,
        frame_support::storage::storage_prefix(b"Paras", b"Heads")
    );

    // Tanssi storage. Since we cannot access the storages themselves,
    // we test the pallet prefix matches and then compute manually the full prefix.
    assert_eq!(
        dancebox_runtime::PalletInfo::name::<AuthorityAssignment>(),
        Some("AuthorityAssignment")
    );
    assert_eq!(
        well_known_keys::AUTHORITY_ASSIGNMENT_PREFIX,
        frame_support::storage::storage_prefix(b"AuthorityAssignment", b"CollatorContainerChain")
    );

    assert_eq!(
        dancebox_runtime::PalletInfo::name::<Session>(),
        Some("Session")
    );
    assert_eq!(
        well_known_keys::SESSION_INDEX,
        frame_support::storage::storage_prefix(b"Session", b"CurrentIndex")
    );
}

#[test]
fn test_staking_no_candidates_in_genesis() {
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
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);

            let initial_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();

            assert_eq!(initial_candidates, vec![]);
        });
}

#[test]
fn test_staking_join() {
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
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);

            let balance_before = System::account(AccountId::from(ALICE)).data.free;
            assert_eq!(System::account(AccountId::from(ALICE)).data.reserved, 0);
            let stake = MinimumSelfDelegation::get() * 10;
            assert_ok!(PooledStaking::request_delegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                stake
            ));

            // Immediately after joining, Alice is the top candidate
            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(
                eligible_candidates,
                vec![EligibleCandidate {
                    candidate: ALICE.into(),
                    stake
                }]
            );

            // And staked amount is immediately marked as "reserved"
            let balance_after = System::account(AccountId::from(ALICE)).data.free;
            assert_eq!(balance_before - balance_after, stake);
            assert_eq!(System::account(AccountId::from(ALICE)).data.reserved, stake);
        });
}

#[test]
fn test_staking_join_no_keys_registered() {
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
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])

        .build()
        .execute_with(|| {
            run_to_block(2);

            let stake = MinimumSelfDelegation::get() * 10;
            let new_account = AccountId::from([42u8; 32]);
            assert_ok!(Balances::transfer_allow_death(
                origin_of(ALICE.into()),
                new_account.clone().into(),
                stake * 2
            ));
            let balance_before = System::account(new_account.clone()).data.free;
            assert_eq!(System::account(new_account.clone()).data.reserved, 0);
            assert_ok!(PooledStaking::request_delegate(
                origin_of(new_account.clone()),
                new_account.clone(),
                TargetPool::AutoCompounding,
                stake
            ));

            // The new account should be the top candidate but it has no keys registered in
            // pallet_session, so it is not eligible
            assert!(!<Runtime as pallet_pooled_staking::Config>::EligibleCandidatesFilter::is_candidate_eligible(&new_account));
            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();

            assert_eq!(eligible_candidates, vec![]);

            // And staked amount is immediately marked as "reserved"
            let balance_after = System::account(new_account.clone()).data.free;
            assert_eq!(balance_before - balance_after, stake);
            assert_eq!(System::account(new_account.clone()).data.reserved, stake);
        });
}

#[test]
fn test_staking_register_keys_after_joining() {
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
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])

        .build()
        .execute_with(|| {
            run_to_block(2);

            let stake = MinimumSelfDelegation::get() * 10;
            let new_account = AccountId::from([42u8; 32]);
            assert_ok!(Balances::transfer_allow_death(
                origin_of(ALICE.into()),
                new_account.clone().into(),
                stake * 2
            ));
            let balance_before = System::account(new_account.clone()).data.free;
            assert_eq!(System::account(new_account.clone()).data.reserved, 0);
            assert_ok!(PooledStaking::request_delegate(
                origin_of(new_account.clone()),
                new_account.clone(),
                TargetPool::AutoCompounding,
                stake
            ));

            // The new account should be the top candidate but it has no keys registered in
            // pallet_session, so it is not eligible
            assert!(!<Runtime as pallet_pooled_staking::Config>::EligibleCandidatesFilter::is_candidate_eligible(&new_account));
            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(eligible_candidates, vec![]);

            // And staked amount is immediately marked as "reserved"
            let balance_after = System::account(new_account.clone()).data.free;
            assert_eq!(balance_before - balance_after, stake);
            assert_eq!(System::account(new_account.clone()).data.reserved, stake);

            // Now register the keys
            let new_account_id = get_aura_id_from_seed(&new_account.to_string());
            assert_ok!(Session::set_keys(
                origin_of(new_account.clone()),
                dancebox_runtime::SessionKeys {
                    nimbus: new_account_id,
                },
                vec![]
            ));

            // Now eligible according to filter
            assert!(<Runtime as pallet_pooled_staking::Config>::EligibleCandidatesFilter::is_candidate_eligible(&new_account));
            // But not eligible according to pallet_pooled_staking, need to manually update candidate list
            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(eligible_candidates, vec![]);

            // Update candidate list
            assert_ok!(PooledStaking::update_candidate_position(
                origin_of(BOB.into()),
                vec![new_account.clone()]
            ));

            // Now it is eligible
            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(
                eligible_candidates,
                vec![EligibleCandidate {
                    candidate: new_account.clone(),
                    stake
                }]
            );
        });
}

#[test]
fn test_staking_join_bad_origin() {
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
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);

            let stake = 10 * MinimumSelfDelegation::get();
            assert_noop!(
                PooledStaking::request_delegate(
                    root_origin(),
                    ALICE.into(),
                    TargetPool::AutoCompounding,
                    stake
                ),
                BadOrigin,
            );
        });
}

#[test]
fn test_staking_join_below_self_delegation_min() {
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
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);

            let stake1 = MinimumSelfDelegation::get() / 3;
            assert_ok!(PooledStaking::request_delegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                stake1
            ));

            // Since stake is below MinimumSelfDelegation, the join operation succeeds
            // but the candidate is not eligible
            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(eligible_candidates, vec![],);

            let stake2 = MinimumSelfDelegation::get() - stake1 - 1;
            assert_ok!(PooledStaking::request_delegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                stake2,
            ));

            // Still below, missing 1 unit
            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(eligible_candidates, vec![],);

            let stake3 = 1;
            assert_ok!(PooledStaking::request_delegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                stake3,
            ));

            // Increasing the stake to above MinimumSelfDelegation makes the candidate eligible
            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(
                eligible_candidates,
                vec![EligibleCandidate {
                    candidate: ALICE.into(),
                    stake: stake1 + stake2 + stake3
                }],
            );
        });
}

#[test]
fn test_staking_join_no_self_delegation() {
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
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);

            // Bob delegates to Alice, but Alice is not a valid candidate (not enough self-delegation)
            let stake = 10 * MinimumSelfDelegation::get();
            assert_ok!(PooledStaking::request_delegate(
                origin_of(BOB.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                stake,
            ));

            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(eligible_candidates, vec![],);
        });
}

#[test]
fn test_staking_join_before_self_delegation() {
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
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);

            // Bob delegates to Alice, but Alice is not a valid candidate (not enough self-delegation)
            let stake = 10 * MinimumSelfDelegation::get();
            assert_ok!(PooledStaking::request_delegate(
                origin_of(BOB.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                stake
            ));

            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(eligible_candidates, vec![],);

            run_to_session(2);
            assert_ok!(PooledStaking::execute_pending_operations(
                origin_of(ALICE.into()),
                vec![PendingOperationQuery {
                    delegator: BOB.into(),
                    operation: PendingOperationKey::JoiningAutoCompounding {
                        candidate: ALICE.into(),
                        at: 0,
                    }
                }]
            ),);

            // Now Alice joins with enough self-delegation
            assert_ok!(PooledStaking::request_delegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                stake
            ));

            // Alice is a valid candidate, and Bob's stake is also counted
            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(
                eligible_candidates,
                vec![EligibleCandidate {
                    candidate: ALICE.into(),
                    stake: stake * 2,
                }],
            );
        });
}

#[test]
fn test_staking_join_twice_in_same_block() {
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
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);

            let stake1 = 10 * MinimumSelfDelegation::get();
            assert_ok!(PooledStaking::request_delegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                stake1
            ));

            let stake2 = 9 * MinimumSelfDelegation::get();
            assert_ok!(PooledStaking::request_delegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                stake2
            ));

            // Both operations succeed and the total stake is the sum of the individual stakes
            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();

            assert_eq!(
                eligible_candidates,
                vec![EligibleCandidate {
                    candidate: ALICE.into(),
                    stake: stake1 + stake2,
                }]
            );

            run_to_session(2);

            assert_ok!(PooledStaking::execute_pending_operations(
                origin_of(ALICE.into()),
                vec![PendingOperationQuery {
                    delegator: ALICE.into(),
                    operation: PendingOperationKey::JoiningAutoCompounding {
                        candidate: ALICE.into(),
                        at: 0,
                    }
                }]
            ),);

            // TODO: ensure the total stake has been moved to auto compounding pool
        });
}

#[test]
fn test_staking_join_execute_before_time() {
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
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);

            let stake = 10 * MinimumSelfDelegation::get();
            assert_ok!(PooledStaking::request_delegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                stake
            ));

            // Immediately after joining, Alice is the top candidate
            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(
                eligible_candidates,
                vec![EligibleCandidate {
                    candidate: ALICE.into(),
                    stake
                }]
            );

            // We called request_delegate in session 0, we will be able to execute it starting from session 2
            let start_of_session_2 = session_to_block(2);
            // Session 2 starts at block 600, but run_to_session runs to block 601, so subtract 2 here to go to 599
            run_to_block(start_of_session_2 - 2);
            assert_noop!(
                PooledStaking::execute_pending_operations(
                    origin_of(ALICE.into()),
                    vec![PendingOperationQuery {
                        delegator: ALICE.into(),
                        operation: PendingOperationKey::JoiningAutoCompounding {
                            candidate: ALICE.into(),
                            at: 0,
                        }
                    }]
                ),
                pallet_pooled_staking::Error::<Runtime>::RequestCannotBeExecuted(0),
            );

            run_to_block(start_of_session_2);
            assert_ok!(PooledStaking::execute_pending_operations(
                origin_of(ALICE.into()),
                vec![PendingOperationQuery {
                    delegator: ALICE.into(),
                    operation: PendingOperationKey::JoiningAutoCompounding {
                        candidate: ALICE.into(),
                        at: 0,
                    }
                }]
            ),);
        });
}

#[test]
fn test_staking_join_execute_any_origin() {
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
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);

            let stake = 10 * MinimumSelfDelegation::get();
            assert_ok!(PooledStaking::request_delegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                stake
            ));

            // Immediately after joining, Alice is the top candidate
            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(
                eligible_candidates,
                vec![EligibleCandidate {
                    candidate: ALICE.into(),
                    stake
                }]
            );

            // We called request_delegate in session 0, we will be able to execute it starting from session 2
            run_to_session(2);
            // Anyone can execute pending operations for anyone else
            assert_ok!(PooledStaking::execute_pending_operations(
                origin_of(BOB.into()),
                vec![PendingOperationQuery {
                    delegator: ALICE.into(),
                    operation: PendingOperationKey::JoiningAutoCompounding {
                        candidate: ALICE.into(),
                        at: 0,
                    }
                }]
            ),);
        });
}

#[test]
fn test_staking_join_execute_bad_origin() {
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
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);

            let stake = 10 * MinimumSelfDelegation::get();
            assert_ok!(PooledStaking::request_delegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                stake
            ));

            // Immediately after joining, Alice is the top candidate
            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(
                eligible_candidates,
                vec![EligibleCandidate {
                    candidate: ALICE.into(),
                    stake
                }]
            );

            // We called request_delegate in session 0, we will be able to execute it starting from session 2
            run_to_session(2);
            assert_noop!(
                PooledStaking::execute_pending_operations(
                    root_origin(),
                    vec![PendingOperationQuery {
                        delegator: ALICE.into(),
                        operation: PendingOperationKey::JoiningAutoCompounding {
                            candidate: ALICE.into(),
                            at: 0,
                        }
                    }]
                ),
                BadOrigin,
            );
        });
}

struct A {
    delegator: AccountId,
    candidate: AccountId,
    target_pool: TargetPool,
    stake: u128,
}

// Setup test environment with provided delegations already being executed. Input function f gets executed at start session 2
fn setup_staking_join_and_execute<R>(ops: Vec<A>, f: impl FnOnce() -> R) {
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
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);

            for op in ops.iter() {
                assert_ok!(PooledStaking::request_delegate(
                    origin_of(op.delegator.clone()),
                    op.candidate.clone(),
                    op.target_pool,
                    op.stake,
                ));
            }

            // We called request_delegate in session 0, we will be able to execute it starting from session 2
            run_to_session(2);

            for op in ops.iter() {
                let operation = match op.target_pool {
                    TargetPool::AutoCompounding => PendingOperationKey::JoiningAutoCompounding {
                        candidate: op.candidate.clone(),
                        at: 0,
                    },
                    TargetPool::ManualRewards => PendingOperationKey::JoiningManualRewards {
                        candidate: op.candidate.clone(),
                        at: 0,
                    },
                };

                assert_ok!(PooledStaking::execute_pending_operations(
                    origin_of(op.delegator.clone()),
                    vec![PendingOperationQuery {
                        delegator: op.delegator.clone(),
                        operation,
                    }]
                ));
            }

            f()
        });
}

#[test]
fn test_staking_leave_exact_amount() {
    setup_staking_join_and_execute(
        vec![A {
            delegator: ALICE.into(),
            candidate: ALICE.into(),
            target_pool: TargetPool::AutoCompounding,
            stake: 10 * MinimumSelfDelegation::get(),
        }],
        || {
            let stake = 10 * MinimumSelfDelegation::get();
            assert_ok!(PooledStaking::request_undelegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                SharesOrStake::Stake(stake),
            ));

            // Immediately after calling request_undelegate, Alice is no longer a candidate
            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(eligible_candidates, vec![]);
        },
    )
}

#[test]
fn test_staking_leave_bad_origin() {
    setup_staking_join_and_execute(
        vec![A {
            delegator: ALICE.into(),
            candidate: ALICE.into(),
            target_pool: TargetPool::AutoCompounding,
            stake: 10 * MinimumSelfDelegation::get(),
        }],
        || {
            let stake = 10 * MinimumSelfDelegation::get();
            assert_noop!(
                PooledStaking::request_undelegate(
                    root_origin(),
                    ALICE.into(),
                    TargetPool::AutoCompounding,
                    SharesOrStake::Stake(stake),
                ),
                BadOrigin
            );
        },
    )
}

#[test]
fn test_staking_leave_more_than_allowed() {
    setup_staking_join_and_execute(
        vec![A {
            delegator: ALICE.into(),
            candidate: ALICE.into(),
            target_pool: TargetPool::AutoCompounding,
            stake: 10 * MinimumSelfDelegation::get(),
        }],
        || {
            let stake = 10 * MinimumSelfDelegation::get();
            assert_noop!(
                PooledStaking::request_undelegate(
                    origin_of(ALICE.into()),
                    ALICE.into(),
                    TargetPool::AutoCompounding,
                    SharesOrStake::Stake(stake + 1 * MinimumSelfDelegation::get()),
                ),
                pallet_pooled_staking::Error::<Runtime>::MathUnderflow,
            );
        },
    );
}

#[test]
fn test_staking_leave_in_separate_transactions() {
    let stake = 10 * MinimumSelfDelegation::get();

    setup_staking_join_and_execute(
        vec![A {
            delegator: ALICE.into(),
            candidate: ALICE.into(),
            target_pool: TargetPool::AutoCompounding,
            stake,
        }],
        || {
            let half_stake = stake / 2;
            assert_ok!(PooledStaking::request_undelegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                SharesOrStake::Stake(half_stake),
            ));

            // Alice is still a valid candidate, now with less stake
            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            let remaining_stake = stake - half_stake;
            assert_eq!(
                eligible_candidates,
                vec![EligibleCandidate {
                    candidate: ALICE.into(),
                    stake: remaining_stake,
                }],
            );

            assert_ok!(PooledStaking::request_undelegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                SharesOrStake::Stake(remaining_stake),
            ));

            // Unstaked remaining stake, so no longer a valid candidate
            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(eligible_candidates, vec![],);
        },
    );
}

#[test]
fn test_staking_leave_all_except_some_dust() {
    let stake = 10 * MinimumSelfDelegation::get();

    setup_staking_join_and_execute(
        vec![A {
            delegator: ALICE.into(),
            candidate: ALICE.into(),
            target_pool: TargetPool::AutoCompounding,
            stake,
        }],
        || {
            let dust = MinimumSelfDelegation::get() / 2;
            assert_ok!(PooledStaking::request_undelegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                SharesOrStake::Stake(stake - dust),
            ));

            // Alice still has some stake left, but not enough to reach MinimumSelfDelegation
            assert_eq!(
                pallet_pooled_staking::Pools::<Runtime>::get(
                    AccountId::from(ALICE),
                    PoolsKey::CandidateTotalStake
                ),
                dust,
            );

            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(eligible_candidates, vec![],);

            // Leave with remaining stake
            assert_ok!(PooledStaking::request_undelegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                SharesOrStake::Stake(dust),
            ));

            // Alice has no more stake left
            assert_eq!(
                pallet_pooled_staking::Pools::<Runtime>::get(
                    AccountId::from(ALICE),
                    PoolsKey::CandidateTotalStake
                ),
                0,
            );
        },
    );
}

#[test]
fn test_staking_leave_execute_before_time() {
    let stake = 10 * MinimumSelfDelegation::get();

    setup_staking_join_and_execute(
        vec![A {
            delegator: ALICE.into(),
            candidate: ALICE.into(),
            target_pool: TargetPool::AutoCompounding,
            stake,
        }],
        || {
            let balance_before = System::account(AccountId::from(ALICE)).data.free;
            let at = Session::current_index();
            assert_ok!(PooledStaking::request_undelegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                SharesOrStake::Stake(stake),
            ));

            // Request undelegate does not change account balance
            assert_eq!(
                balance_before,
                System::account(AccountId::from(ALICE)).data.free
            );

            // We called request_delegate in session 0, we will be able to execute it starting from session 2
            let start_of_session_4 = session_to_block(4);
            // Session 4 starts at block 1200, but run_to_session runs to block 1201, so subtract 2 here to go to 1999
            run_to_block(start_of_session_4 - 2);

            assert_noop!(
                PooledStaking::execute_pending_operations(
                    origin_of(ALICE.into()),
                    vec![PendingOperationQuery {
                        delegator: ALICE.into(),
                        operation: PendingOperationKey::Leaving {
                            candidate: ALICE.into(),
                            at,
                        }
                    }]
                ),
                pallet_pooled_staking::Error::<Runtime>::RequestCannotBeExecuted(0)
            );
        },
    );
}

#[test]
fn test_staking_leave_execute_any_origin() {
    let stake = 10 * MinimumSelfDelegation::get();

    setup_staking_join_and_execute(
        vec![A {
            delegator: ALICE.into(),
            candidate: ALICE.into(),
            target_pool: TargetPool::AutoCompounding,
            stake,
        }],
        || {
            let balance_before = System::account(AccountId::from(ALICE)).data.free;
            let at = Session::current_index();
            assert_ok!(PooledStaking::request_undelegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                SharesOrStake::Stake(stake),
            ));

            // Request undelegate does not change account balance
            assert_eq!(
                balance_before,
                System::account(AccountId::from(ALICE)).data.free
            );

            run_to_session(4);

            let balance_before = System::account(AccountId::from(ALICE)).data.free;

            assert_ok!(PooledStaking::execute_pending_operations(
                // Any signed origin can execute this, the stake will go to Alice account
                origin_of(BOB.into()),
                vec![PendingOperationQuery {
                    delegator: ALICE.into(),
                    operation: PendingOperationKey::Leaving {
                        candidate: ALICE.into(),
                        at,
                    }
                }]
            ),);

            let balance_after = System::account(AccountId::from(ALICE)).data.free;
            assert_eq!(balance_after - balance_before, stake);
        },
    );
}

#[test]
fn test_staking_leave_execute_bad_origin() {
    let stake = 10 * MinimumSelfDelegation::get();

    setup_staking_join_and_execute(
        vec![A {
            delegator: ALICE.into(),
            candidate: ALICE.into(),
            target_pool: TargetPool::AutoCompounding,
            stake,
        }],
        || {
            let at = Session::current_index();
            assert_ok!(PooledStaking::request_undelegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                SharesOrStake::Stake(stake),
            ));

            run_to_session(4);

            assert_noop!(
                PooledStaking::execute_pending_operations(
                    root_origin(),
                    vec![PendingOperationQuery {
                        delegator: ALICE.into(),
                        operation: PendingOperationKey::Leaving {
                            candidate: ALICE.into(),
                            at,
                        }
                    }]
                ),
                BadOrigin
            );
        },
    );
}

#[test]
fn test_staking_swap() {
    setup_staking_join_and_execute(
        vec![A {
            delegator: ALICE.into(),
            candidate: ALICE.into(),
            target_pool: TargetPool::AutoCompounding,
            stake: 10 * MinimumSelfDelegation::get(),
        }],
        || {
            let stake = 10 * MinimumSelfDelegation::get();
            assert_ok!(PooledStaking::swap_pool(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                SharesOrStake::Stake(stake),
            ));

            assert_eq!(
                PooledStaking::computed_stake(
                    ALICE.into(),
                    ALICE.into(),
                    AllTargetPool::AutoCompounding
                ),
                Some(0u32.into())
            );
            assert_eq!(
                PooledStaking::computed_stake(
                    ALICE.into(),
                    ALICE.into(),
                    AllTargetPool::ManualRewards
                ),
                Some(stake)
            );

            assert_ok!(PooledStaking::swap_pool(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::ManualRewards,
                SharesOrStake::Stake(stake),
            ));

            assert_eq!(
                PooledStaking::computed_stake(
                    ALICE.into(),
                    ALICE.into(),
                    AllTargetPool::AutoCompounding
                ),
                Some(stake)
            );
            assert_eq!(
                PooledStaking::computed_stake(
                    ALICE.into(),
                    ALICE.into(),
                    AllTargetPool::ManualRewards
                ),
                Some(0u32.into())
            );
        },
    )
}

#[test]
fn test_pallet_session_takes_validators_from_invulnerables_and_staking() {
    // Alice, Bob, Charlie are invulnerables
    // Alice, Dave are in pallet_staking
    // Expected collators are Alice, Bob, Charlie, Dave
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
        ])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);

            let stake = 10 * MinimumSelfDelegation::get();

            assert_ok!(PooledStaking::request_delegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                stake,
            ));

            // Register Dave in pallet_session (invulnerables are automatically registered)
            let dave_account_id = get_aura_id_from_seed(&AccountId::from(DAVE).to_string());
            assert_ok!(Session::set_keys(
                origin_of(DAVE.into()),
                dancebox_runtime::SessionKeys {
                    nimbus: dave_account_id,
                },
                vec![]
            ));

            assert_ok!(PooledStaking::request_delegate(
                origin_of(DAVE.into()),
                DAVE.into(),
                TargetPool::AutoCompounding,
                stake,
            ));

            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(
                eligible_candidates,
                vec![
                    EligibleCandidate {
                        candidate: ALICE.into(),
                        stake
                    },
                    EligibleCandidate {
                        candidate: DAVE.into(),
                        stake
                    },
                ]
            );

            assert_eq!(
                pallet_invulnerables::Invulnerables::<Runtime>::get().to_vec(),
                vec![
                    AccountId::from(ALICE),
                    AccountId::from(BOB),
                    AccountId::from(CHARLIE),
                ]
            );

            // Need to trigger new session to update pallet_session
            run_to_session(2);

            assert_eq!(
                Session::validators(),
                vec![
                    AccountId::from(ALICE),
                    AccountId::from(BOB),
                    AccountId::from(CHARLIE),
                    AccountId::from(DAVE),
                ]
            );
        });
}

#[test]
fn test_pallet_session_limits_num_validators() {
    // Set max_collators = 2, now only the first 2 invulnerables are valid collators
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
        ])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 2,
            min_orchestrator_collators: 2,
            max_orchestrator_collators: 2,
            collators_per_container: 2,
            full_rotation_period: 24,
            ..Default::default()
        })
        .build()
        .execute_with(|| {
            run_to_block(2);

            let stake = 10 * MinimumSelfDelegation::get();

            assert_ok!(PooledStaking::request_delegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                stake,
            ));

            // Register Dave in pallet_session (invulnerables are automatically registered)
            let dave_account_id = get_aura_id_from_seed(&AccountId::from(DAVE).to_string());
            assert_ok!(Session::set_keys(
                origin_of(DAVE.into()),
                dancebox_runtime::SessionKeys {
                    nimbus: dave_account_id,
                },
                vec![]
            ));

            assert_ok!(PooledStaking::request_delegate(
                origin_of(DAVE.into()),
                DAVE.into(),
                TargetPool::AutoCompounding,
                stake,
            ));

            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(
                eligible_candidates,
                vec![
                    EligibleCandidate {
                        candidate: ALICE.into(),
                        stake
                    },
                    EligibleCandidate {
                        candidate: DAVE.into(),
                        stake
                    },
                ]
            );

            assert_eq!(
                pallet_invulnerables::Invulnerables::<Runtime>::get().to_vec(),
                vec![
                    AccountId::from(ALICE),
                    AccountId::from(BOB),
                    AccountId::from(CHARLIE),
                ]
            );

            // Need to trigger new session to update pallet_session
            run_to_session(2);

            assert_eq!(
                Session::validators(),
                vec![AccountId::from(ALICE), AccountId::from(BOB),]
            );
        });
}

#[test]
fn test_pallet_session_limits_num_validators_from_staking() {
    // Set max_collators = 2, take 1 invulnerable and the rest from staking
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![(AccountId::from(ALICE), 210 * UNIT)])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 2,
            min_orchestrator_collators: 2,
            max_orchestrator_collators: 2,
            collators_per_container: 2,
            full_rotation_period: 24,
            ..Default::default()
        })
        .build()
        .execute_with(|| {
            run_to_block(2);

            let stake = 10 * MinimumSelfDelegation::get();

            // Register accounts in pallet_session (invulnerables are automatically registered)
            let bob_account_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());
            assert_ok!(Session::set_keys(
                origin_of(BOB.into()),
                dancebox_runtime::SessionKeys {
                    nimbus: bob_account_id,
                },
                vec![]
            ));
            let charlie_account_id = get_aura_id_from_seed(&AccountId::from(CHARLIE).to_string());
            assert_ok!(Session::set_keys(
                origin_of(CHARLIE.into()),
                dancebox_runtime::SessionKeys {
                    nimbus: charlie_account_id,
                },
                vec![]
            ));
            let dave_account_id = get_aura_id_from_seed(&AccountId::from(DAVE).to_string());
            assert_ok!(Session::set_keys(
                origin_of(DAVE.into()),
                dancebox_runtime::SessionKeys {
                    nimbus: dave_account_id,
                },
                vec![]
            ));

            assert_ok!(PooledStaking::request_delegate(
                origin_of(BOB.into()),
                BOB.into(),
                TargetPool::AutoCompounding,
                stake,
            ));

            assert_ok!(PooledStaking::request_delegate(
                origin_of(CHARLIE.into()),
                CHARLIE.into(),
                TargetPool::AutoCompounding,
                stake,
            ));

            assert_ok!(PooledStaking::request_delegate(
                origin_of(DAVE.into()),
                DAVE.into(),
                TargetPool::AutoCompounding,
                stake,
            ));

            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(
                eligible_candidates,
                vec![
                    EligibleCandidate {
                        candidate: BOB.into(),
                        stake
                    },
                    EligibleCandidate {
                        candidate: CHARLIE.into(),
                        stake
                    },
                    EligibleCandidate {
                        candidate: DAVE.into(),
                        stake
                    },
                ]
            );

            assert_eq!(
                pallet_invulnerables::Invulnerables::<Runtime>::get().to_vec(),
                vec![AccountId::from(ALICE),]
            );

            // Need to trigger new session to update pallet_session
            run_to_session(2);

            assert_eq!(
                Session::validators(),
                vec![AccountId::from(ALICE), AccountId::from(BOB),]
            );
        });
}

#[test]
fn test_reward_to_staking_candidate() {
    // Alice, Bob, Charlie are invulnerables
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![(AccountId::from(ALICE), 210 * UNIT)])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);

            let dave_account_id = get_aura_id_from_seed(&AccountId::from(DAVE).to_string());
            assert_ok!(Session::set_keys(
                origin_of(DAVE.into()),
                dancebox_runtime::SessionKeys {
                    nimbus: dave_account_id,
                },
                vec![]
            ));

            // We make delegations to DAVE so that she is an elligible candidate.

            let stake = 10 * MinimumSelfDelegation::get();

            assert_ok!(PooledStaking::request_delegate(
                origin_of(DAVE.into()),
                DAVE.into(),
                TargetPool::ManualRewards,
                stake,
            ));
            assert_ok!(PooledStaking::request_delegate(
                origin_of(BOB.into()),
                DAVE.into(),
                TargetPool::AutoCompounding,
                stake,
            ));

            // wait few sessions for the request to be executable
            run_to_session(3u32);
            assert_ok!(PooledStaking::execute_pending_operations(
                origin_of(ALICE.into()),
                vec![
                    PendingOperationQuery {
                        delegator: DAVE.into(),
                        operation: PendingOperationKey::JoiningManualRewards {
                            candidate: DAVE.into(),
                            at: 0
                        }
                    },
                    PendingOperationQuery {
                        delegator: BOB.into(),
                        operation: PendingOperationKey::JoiningAutoCompounding {
                            candidate: DAVE.into(),
                            at: 0
                        }
                    }
                ]
            ));

            // wait for next session so that DAVE is elected
            run_to_session(4u32);

            assert_eq!(
                Session::validators(),
                vec![AccountId::from(ALICE), AccountId::from(DAVE)]
            );

            let account: AccountId = DAVE.into();
            let balance_before = System::account(account.clone()).data.free;
            let summary = (0..100)
                .find_map(|_| {
                    let summary = run_block();
                    if summary.author_id == DAVE.into() {
                        Some(summary)
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| panic!("DAVE doesn't seem to author any blocks"));
            let balance_after = System::account(account).data.free;

            let all_rewards = RewardsPortion::get() * summary.inflation;
            // rewards are shared between orchestrator and registered paras
            let orchestrator_rewards = all_rewards / 3;
            let candidate_rewards = RewardsCollatorCommission::get() * orchestrator_rewards;

            assert_eq!(
                candidate_rewards,
                balance_after - balance_before,
                "dave should get the correct reward portion"
            );
        });
}

#[test]
fn test_reward_to_invulnerable() {
    // Alice, Bob, Charlie are invulnerables
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
        ])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);

            // We make delegations to ALICE so that she is an elligible candidate.
            // However since she is an invulnerable she should get all the
            // rewards.

            let stake = 10 * MinimumSelfDelegation::get();

            assert_ok!(PooledStaking::request_delegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::ManualRewards,
                stake,
            ));
            assert_ok!(PooledStaking::request_delegate(
                origin_of(BOB.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                stake,
            ));

            // wait few sessions for the request to be executable
            run_to_session(3u32);
            assert_ok!(PooledStaking::execute_pending_operations(
                origin_of(ALICE.into()),
                vec![
                    PendingOperationQuery {
                        delegator: ALICE.into(),
                        operation: PendingOperationKey::JoiningAutoCompounding {
                            candidate: ALICE.into(),
                            at: 0
                        }
                    },
                    PendingOperationQuery {
                        delegator: BOB.into(),
                        operation: PendingOperationKey::JoiningAutoCompounding {
                            candidate: ALICE.into(),
                            at: 0
                        }
                    }
                ]
            ));

            // wait for next session so that ALICE is elected
            run_to_session(4u32);

            let account: AccountId = ALICE.into();
            let balance_before = System::account(account.clone()).data.free;

            let summary = (0..100)
                .find_map(|_| {
                    let summary = run_block();
                    if summary.author_id == ALICE.into() {
                        Some(summary)
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| panic!("ALICE doesn't seem to author any blocks"));

            let balance_after = System::account(account).data.free;

            let all_rewards = RewardsPortion::get() * summary.inflation;
            // rewards are shared between orchestrator and registered paras
            let orchestrator_rewards = all_rewards / 3;
            assert_eq!(
                orchestrator_rewards,
                balance_after - balance_before,
                "alice should get the correct reward portion"
            );
        });
}

#[test]
fn test_reward_to_invulnerable_with_key_change() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![(AccountId::from(ALICE), 210 * UNIT)])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);

            run_to_session(2u32);

            // change key, this should be reflected 2 sessions afterward
            let alice_new_key = get_aura_id_from_seed(&AccountId::from(DAVE).to_string());
            assert_ok!(Session::set_keys(
                origin_of(ALICE.into()),
                dancebox_runtime::SessionKeys {
                    nimbus: alice_new_key,
                },
                vec![]
            ));

            run_to_session(4u32);

            let account: AccountId = ALICE.into();
            let balance_before = System::account(account.clone()).data.free;

            let summary = run_block();
            assert_eq!(summary.author_id, ALICE.into());

            let balance_after = System::account(account).data.free;

            let all_rewards = RewardsPortion::get() * summary.inflation;
            // rewards are shared between orchestrator and registered paras
            let orchestrator_rewards = all_rewards / 3;
            assert_eq!(
                orchestrator_rewards,
                balance_after - balance_before,
                "alice should get the correct reward portion"
            );
        });
}

#[test]
fn test_migration_config_full_rotation_period() {
    ExtBuilder::default()
        .build()
        .execute_with(|| {
            const CONFIGURATION_ACTIVE_CONFIG_KEY: &[u8] =
                &hex_literal::hex!("06de3d8a54d27e44a9d5ce189618f22db4b49d95320d9021994c850f25b8e385");
            const CONFIGURATION_PENDING_CONFIGS_KEY: &[u8] =
                &hex_literal::hex!("06de3d8a54d27e44a9d5ce189618f22d53b4123b2e186e07fb7bad5dda5f55c0");

            // Modify active config
            frame_support::storage::unhashed::put_raw(CONFIGURATION_ACTIVE_CONFIG_KEY, &hex_literal::hex!("6300000002000000050000000200000000000000"));
            // Modify pending configs
            frame_support::storage::unhashed::put_raw(CONFIGURATION_PENDING_CONFIGS_KEY, &hex_literal::hex!("08b10800006300000002000000050000000200000000000000b20800006400000002000000050000000200000000000000"));

            let migration = MigrateConfigurationParathreads::<Runtime>(Default::default());
            migration.migrate(Default::default());

            let expected_active = pallet_configuration::HostConfiguration {
                max_collators: 99,
                min_orchestrator_collators: 2,
                max_orchestrator_collators: 5,
                collators_per_container: 2,
                full_rotation_period: 0,
                ..Default::default()
            };
            assert_eq!(Configuration::config(), expected_active);

            let expected_pending = vec![
                (
                    2225,
                    pallet_configuration::HostConfiguration {
                        max_collators: 99,
                        min_orchestrator_collators: 2,
                        max_orchestrator_collators: 5,
                        collators_per_container: 2,
                        full_rotation_period: 0,
                        ..Default::default()
                    },
                ),
                (
                    2226,
                    pallet_configuration::HostConfiguration {
                        max_collators: 100,
                        min_orchestrator_collators: 2,
                        max_orchestrator_collators: 5,
                        collators_per_container: 2,
                        full_rotation_period: 0,
                        ..Default::default()
                    },
                ),
            ];
            assert_eq!(Configuration::pending_configs(), expected_pending);
        });
}

#[test]
fn test_migration_registrar_pending_verification() {
    ExtBuilder::default().build().execute_with(|| {
        const REGISTRAR_PENDING_VERIFICATION_KEY: &[u8] =
            &hex_literal::hex!("3fba98689ebed1138735e0e7a5a790ab57a35de516113188134ad8e43c6d55ec");

        // Modify active config
        let para_ids: Vec<ParaId> = vec![2000.into(), 2001.into(), 2002.into(), 3000.into()];
        frame_support::storage::unhashed::put(REGISTRAR_PENDING_VERIFICATION_KEY, &para_ids);

        let migration = RegistrarPendingVerificationValueToMap::<Runtime>(Default::default());
        migration.migrate(Default::default());

        let empty_key =
            frame_support::storage::unhashed::get_raw(REGISTRAR_PENDING_VERIFICATION_KEY);
        assert_eq!(empty_key, None);

        for para_id in para_ids {
            let exists_in_map =
                pallet_registrar::PendingVerification::<Runtime>::get(para_id).is_some();
            assert!(
                exists_in_map,
                "After migration, para id {:?} does not exist in storage map",
                para_id
            );
        }
    });
}

#[test]
fn test_collator_assignment_gives_priority_to_invulnerables() {
    // Set max_collators = 2, take 1 invulnerable and the rest from staking
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
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);

            let stake = 10 * MinimumSelfDelegation::get();

            // Register accounts in pallet_session (invulnerables are automatically registered)
            let bob_account_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());
            assert_ok!(Session::set_keys(
                origin_of(BOB.into()),
                dancebox_runtime::SessionKeys {
                    nimbus: bob_account_id,
                },
                vec![]
            ));
            let charlie_account_id = get_aura_id_from_seed(&AccountId::from(CHARLIE).to_string());
            assert_ok!(Session::set_keys(
                origin_of(CHARLIE.into()),
                dancebox_runtime::SessionKeys {
                    nimbus: charlie_account_id,
                },
                vec![]
            ));

            assert_ok!(PooledStaking::request_delegate(
                origin_of(BOB.into()),
                BOB.into(),
                TargetPool::AutoCompounding,
                stake,
            ));
            assert_ok!(PooledStaking::request_delegate(
                origin_of(CHARLIE.into()),
                CHARLIE.into(),
                TargetPool::AutoCompounding,
                stake,
            ));

            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(
                eligible_candidates,
                vec![
                    EligibleCandidate {
                        candidate: BOB.into(),
                        stake
                    },
                    EligibleCandidate {
                        candidate: CHARLIE.into(),
                        stake
                    },
                ]
            );

            assert_eq!(
                pallet_invulnerables::Invulnerables::<Runtime>::get().to_vec(),
                vec![AccountId::from(ALICE), AccountId::from(DAVE)]
            );

            set_parachain_inherent_data_random_seed([1; 32]);
            run_block();

            // Need to trigger new session to update pallet_session
            run_to_session(2);

            assert_eq!(
                Session::validators(),
                vec![
                    AccountId::from(ALICE),
                    AccountId::from(DAVE),
                    AccountId::from(BOB),
                    AccountId::from(CHARLIE)
                ]
            );

            // Need to trigger full rotation to ensure invulnerables are assigned
            let rotation_period = Configuration::config().full_rotation_period;
            run_to_session(rotation_period);

            assert!(
                CollatorAssignment::collator_container_chain()
                    .orchestrator_chain
                    .contains(&AccountId::from(ALICE)),
                "CollatorAssignment did not give priority to invulnerable ALICE: {:?}",
                CollatorAssignment::collator_container_chain()
            );

            assert!(
                CollatorAssignment::collator_container_chain()
                    .orchestrator_chain
                    .contains(&AccountId::from(DAVE)),
                "CollatorAssignment did not give priority to invulnerable DAVE: {:?}",
                CollatorAssignment::collator_container_chain()
            );
        });
}

#[test]
fn test_can_buy_credits_before_registering_para() {
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

            // Try to buy the maximum amount of credits
            let balance_before = System::account(AccountId::from(ALICE)).data.free;

            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                1001.into(),
                block_credits_to_required_balance(u32::MAX, 1001.into())
            ));
            let balance_after = System::account(AccountId::from(ALICE)).data.free;

            // Now parachain tank should have this amount
            let balance_tank = System::account(ServicesPayment::parachain_tank(1001.into()))
                .data
                .free;

            assert_eq!(
                balance_tank,
                block_credits_to_required_balance(u32::MAX, 1001.into())
            );

            let expected_cost = block_credits_to_required_balance(u32::MAX, 1001.into());
            assert_eq!(balance_before - balance_after, expected_cost);
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
            assert_ok!(Registrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data()
            ));
            assert_noop!(
                Registrar::mark_valid_for_collating(root_origin(), 1001.into()),
                pallet_data_preservers::Error::<Runtime>::NoBootNodes,
            );
        });
}

#[test]
fn test_can_buy_credits_before_registering_para_and_receive_free_credits() {
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

            // Try to buy (MaxCreditsStored - 1) credits
            let balance_before = System::account(AccountId::from(ALICE)).data.free;
            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                1001.into(),
                block_credits_to_required_balance(
                    dancebox_runtime::FreeBlockProductionCredits::get() - 1,
                    1001.into()
                )
            ));
            let balance_after = System::account(AccountId::from(ALICE)).data.free;

            // Now parachain tank should have this amount
            let balance_tank = System::account(ServicesPayment::parachain_tank(1001.into()))
                .data
                .free;

            assert_eq!(
                balance_tank,
                block_credits_to_required_balance(
                    dancebox_runtime::FreeBlockProductionCredits::get() - 1,
                    1001.into()
                )
            );

            let expected_cost = block_credits_to_required_balance(
                dancebox_runtime::FreeBlockProductionCredits::get() - 1,
                1001.into(),
            );
            assert_eq!(balance_before - balance_after, expected_cost);

            // Now register para
            assert_ok!(Registrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data()
            ));
            assert_ok!(DataPreservers::set_boot_nodes(
                origin_of(ALICE.into()),
                1001.into(),
                dummy_boot_nodes()
            ));
            assert_ok!(Registrar::mark_valid_for_collating(
                root_origin(),
                1001.into()
            ));

            // We received aññ free credits, because we cannot have more than MaxCreditsStored
            let credits =
                pallet_services_payment::BlockProductionCredits::<Runtime>::get(ParaId::from(1001))
                    .unwrap_or_default();
            assert_eq!(credits, dancebox_runtime::FreeBlockProductionCredits::get());
        });
}

#[test]
fn test_deregister_and_register_again_does_not_give_free_credits() {
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

            // Register
            assert_ok!(Registrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data()
            ),);
            assert_ok!(DataPreservers::set_boot_nodes(
                origin_of(ALICE.into()),
                1001.into(),
                dummy_boot_nodes()
            ));
            assert_ok!(Registrar::mark_valid_for_collating(
                root_origin(),
                1001.into()
            ),);
            // We received free credits
            let credits =
                pallet_services_payment::BlockProductionCredits::<Runtime>::get(ParaId::from(1001))
                    .unwrap_or_default();
            assert_eq!(credits, dancebox_runtime::FreeBlockProductionCredits::get());
            // Deregister after 1 session
            run_to_session(1);
            assert_ok!(Registrar::deregister(root_origin(), 1001.into()), ());

            run_to_session(3);
            let credits_before_2nd_register =
                pallet_services_payment::BlockProductionCredits::<Runtime>::get(ParaId::from(1001))
                    .unwrap_or_default();
            // We spent some credits because this container chain had collators for 1 session
            assert_ne!(
                credits_before_2nd_register,
                dancebox_runtime::FreeBlockProductionCredits::get()
            );
            // Register again
            assert_ok!(Registrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data()
            ),);
            assert_ok!(DataPreservers::set_boot_nodes(
                origin_of(ALICE.into()),
                1001.into(),
                dummy_boot_nodes()
            ));
            assert_ok!(Registrar::mark_valid_for_collating(
                root_origin(),
                1001.into()
            ),);
            // No more free credits
            let credits =
                pallet_services_payment::BlockProductionCredits::<Runtime>::get(ParaId::from(1001))
                    .unwrap_or_default();
            assert_eq!(credits, credits_before_2nd_register);
        });
}

#[test]
fn test_sudo_can_register_foreign_assets_and_manager_change_paremeters() {
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

            // We register the asset with Alice as manager
            assert_ok!(ForeignAssetsCreator::create_foreign_asset(root_origin(), MultiLocation::parent(), 1, AccountId::from(ALICE), true, 1), ());
            assert_eq!(ForeignAssetsCreator::foreign_asset_for_id(1), Some(MultiLocation::parent()));
            assert_eq!(ForeignAssetsCreator::asset_id_for_foreign(MultiLocation::parent()), Some(1));

            // Alice now can change parameters like metadata from the asset
            assert_ok!(ForeignAssets::set_metadata(origin_of(ALICE.into()), 1, b"xcDot".to_vec(), b"xcDot".to_vec(), 12));
            assert_eq!(<ForeignAssets as frame_support::traits::fungibles::metadata::Inspect<AccountId>>::name(1),  b"xcDot".to_vec());
            assert_eq!(<ForeignAssets as frame_support::traits::fungibles::metadata::Inspect<AccountId>>::symbol(1),  b"xcDot".to_vec());
            assert_eq!(<ForeignAssets as frame_support::traits::fungibles::metadata::Inspect<AccountId>>::decimals(1),  12);

            // Any other person cannot do this
            assert_noop!(
                ForeignAssets::set_metadata(origin_of(BOB.into()), 1, b"dummy".to_vec(), b"dummy".to_vec(), 12),
                pallet_assets::Error::<Runtime, Instance1>::NoPermission
            );

            // Alice now can mint
            assert_ok!(ForeignAssets::mint(origin_of(ALICE.into()), 1, AccountId::from(BOB).into(), 1000));
            assert_eq!(<ForeignAssets as frame_support::traits::fungibles::Inspect<AccountId>>::total_issuance(1),  1000);
            assert_eq!(<ForeignAssets as frame_support::traits::fungibles::Inspect<AccountId>>::balance(1, &AccountId::from(BOB)),  1000);
        });
}

#[test]
fn test_assets_cannot_be_created_from_signed_origins() {
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
            // We try to register the asset with Alice as origin
            // Any other person cannot do this
            assert_noop!(
                ForeignAssetsCreator::create_foreign_asset(
                    origin_of(ALICE.into()),
                    MultiLocation::parent(),
                    1,
                    AccountId::from(ALICE),
                    true,
                    1
                ),
                BadOrigin
            );

            assert_noop!(
                ForeignAssets::create(origin_of(ALICE.into()), 1, AccountId::from(ALICE).into(), 1),
                BadOrigin
            );
        });
}

#[test]
fn test_asset_rate_can_be_set_from_sudo_but_not_from_signed() {
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
            // We try to set the rate from non-sudo
            assert_noop!(
                AssetRate::create(origin_of(ALICE.into()), Box::new(1), FixedU128::from_u32(1)),
                BadOrigin
            );

            // We try to set the rate from sudo
            assert_ok!(AssetRate::create(
                root_origin(),
                Box::new(1),
                FixedU128::from_u32(1)
            ));

            assert_eq!(
                pallet_asset_rate::ConversionRateToNative::<Runtime>::get(1),
                Some(FixedU128::from_u32(1))
            );
        });
}

#[test]
fn test_division_by_0() {
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
            // We try to set 0 rate to make sure we dont overflow
            assert_ok!(AssetRate::create(
                root_origin(),
                Box::new(1),
                FixedU128::from_u32(0)
            ));

            use frame_support::traits::tokens::ConversionToAssetBalance;
            let balance = AssetRate::to_asset_balance(1, 1);
            assert!(balance.is_err());
        });
}

#[test]
fn test_register_parathread() {
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

            // Register
            assert_ok!(Registrar::register_parathread(
                origin_of(ALICE.into()),
                3001.into(),
                SlotFrequency { min: 1, max: 1 },
                empty_genesis_data()
            ));
            assert_ok!(DataPreservers::set_boot_nodes(
                origin_of(ALICE.into()),
                3001.into(),
                dummy_boot_nodes()
            ));
            assert_ok!(Registrar::mark_valid_for_collating(
                root_origin(),
                3001.into()
            ));

            run_to_session(2);
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&ParaId::from(3001)],
                vec![CHARLIE.into()]
            );
        });
}

#[test]
fn test_ed_plus_block_credit_session_purchase_works() {
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
            // Assert current slot gets updated
            assert_eq!(current_slot(), 1u64);
            assert!(current_author() == AccountId::from(BOB));

            // Alice and Bob collate in our chain
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            assert_eq!(authorities(), vec![alice_id, bob_id]);

            assert_ok!(Registrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data()
            ));
            assert_ok!(DataPreservers::set_boot_nodes(
                origin_of(ALICE.into()),
                1001.into(),
                dummy_boot_nodes()
            ));
            assert_ok!(Registrar::mark_valid_for_collating(
                root_origin(),
                1001.into()
            ));
            // Need to reset credits to 0 because now parachains are given free credits on register
            assert_ok!(ServicesPayment::set_block_production_credits(
                root_origin(),
                1001.into(),
                0
            ));
            let credits_1001 =
                block_credits_to_required_balance(dancebox_runtime::Period::get(), 1001.into())
                    + dancebox_runtime::EXISTENTIAL_DEPOSIT;

            // Fill the tank
            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                1001.into(),
                credits_1001
            ));

            // Assignment should happen after 2 sessions
            run_to_session(1u32);
            let assignment = CollatorAssignment::collator_container_chain();
            assert!(assignment.container_chains.is_empty());
            run_to_session(2u32);
            // Charlie and Dave should be assigned to para 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );

            // Simulate block inclusion from container chain 1001
            let mut sproof: ParaHeaderSproofBuilder = ParaHeaderSproofBuilder::default();
            let slot: u64 = 5;
            let s = ParaHeaderSproofBuilderItem {
                para_id: 1001.into(),
                author_id: HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                    parent_hash: Default::default(),
                    number: 1,
                    state_root: Default::default(),
                    extrinsics_root: Default::default(),
                    digest: sp_runtime::generic::Digest {
                        logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                    },
                }),
            };

            sproof.items.push(s);
            set_author_noting_inherent_data(sproof);

            run_block();

            // After this it should not be assigned anymore, since credits are not payable
            run_to_session(3u32);
            // Nobody should be assigned to para 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(assignment.container_chains.get(&1001u32.into()), None,);
        });
}

#[test]
fn test_ed_plus_block_credit_session_minus_1_purchase_fails() {
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
            // Assert current slot gets updated
            assert_eq!(current_slot(), 1u64);
            assert!(current_author() == AccountId::from(BOB));

            // Alice and Bob collate in our chain
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            assert_eq!(authorities(), vec![alice_id, bob_id]);

            assert_ok!(Registrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data()
            ));
            assert_ok!(DataPreservers::set_boot_nodes(
                origin_of(ALICE.into()),
                1001.into(),
                dummy_boot_nodes()
            ));
            assert_ok!(Registrar::mark_valid_for_collating(
                root_origin(),
                1001.into()
            ));
            // Need to reset credits to 0 because now parachains are given free credits on register
            assert_ok!(ServicesPayment::set_block_production_credits(
                root_origin(),
                1001.into(),
                0
            ));
            let credits_1001 =
                block_credits_to_required_balance(dancebox_runtime::Period::get(), 1001.into())
                    + dancebox_runtime::EXISTENTIAL_DEPOSIT
                    - 1;

            // Fill the tank
            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                1001.into(),
                credits_1001
            ));

            // Assignment should happen after 2 sessions
            run_to_session(1u32);
            let assignment = CollatorAssignment::collator_container_chain();
            assert!(assignment.container_chains.is_empty());
            run_to_session(2u32);
            // Charlie and Dave should not be assigned to para 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(assignment.container_chains.get(&1001u32.into()), None,);
        });
}

#[test]
fn test_reassignment_ed_plus_two_block_credit_session_purchase_works() {
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
            // Assert current slot gets updated
            assert_eq!(current_slot(), 1u64);
            assert!(current_author() == AccountId::from(BOB));

            // Alice and Bob collate in our chain
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            assert_eq!(authorities(), vec![alice_id, bob_id]);

            assert_ok!(Registrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data()
            ));
            assert_ok!(DataPreservers::set_boot_nodes(
                origin_of(ALICE.into()),
                1001.into(),
                dummy_boot_nodes()
            ));
            assert_ok!(Registrar::mark_valid_for_collating(
                root_origin(),
                1001.into()
            ));
            // Need to reset credits to 0 because now parachains are given free credits on register
            assert_ok!(ServicesPayment::set_block_production_credits(
                root_origin(),
                1001.into(),
                0
            ));
            // On reassignment the blocks credits needed should be enough for the current session and the next one
            let credits_1001 =
                block_credits_to_required_balance(dancebox_runtime::Period::get() * 2, 1001.into())
                    + dancebox_runtime::EXISTENTIAL_DEPOSIT;

            // Fill the tank
            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                1001.into(),
                credits_1001
            ));

            // Assignment should happen after 2 sessions
            run_to_session(1u32);
            let assignment = CollatorAssignment::collator_container_chain();
            assert!(assignment.container_chains.is_empty());

            run_to_session(2u32);
            // Charlie and Dave should be assigned to para 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );

            // Simulate block inclusion from container chain 1001
            let mut sproof: ParaHeaderSproofBuilder = ParaHeaderSproofBuilder::default();
            let slot: u64 = 5;
            let s = ParaHeaderSproofBuilderItem {
                para_id: 1001.into(),
                author_id: HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                    parent_hash: Default::default(),
                    number: 1,
                    state_root: Default::default(),
                    extrinsics_root: Default::default(),
                    digest: sp_runtime::generic::Digest {
                        logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                    },
                }),
            };

            sproof.items.push(s);
            set_author_noting_inherent_data(sproof);

            run_block();

            // Session 3 should still be assigned
            run_to_session(3u32);
            // Charlie and Dave should be assigned to para 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );

            // After this it should not be assigned anymore, since credits are not payable
            run_to_session(4u32);
            // Nobody should be assigned to para 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(assignment.container_chains.get(&1001u32.into()), None,);
        });
}

#[test]
fn test_reassignment_ed_plus_two_block_credit_session_minus_1_purchase_fails() {
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
            // Assert current slot gets updated
            assert_eq!(current_slot(), 1u64);
            assert!(current_author() == AccountId::from(BOB));

            // Alice and Bob collate in our chain
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            assert_eq!(authorities(), vec![alice_id, bob_id]);

            assert_ok!(Registrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data()
            ));
            assert_ok!(DataPreservers::set_boot_nodes(
                origin_of(ALICE.into()),
                1001.into(),
                dummy_boot_nodes()
            ));
            assert_ok!(Registrar::mark_valid_for_collating(
                root_origin(),
                1001.into()
            ));
            // Need to reset credits to 0 because now parachains are given free credits on register
            assert_ok!(ServicesPayment::set_block_production_credits(
                root_origin(),
                1001.into(),
                0
            ));
            let credits_1001 =
                block_credits_to_required_balance(dancebox_runtime::Period::get() * 2, 1001.into())
                    + dancebox_runtime::EXISTENTIAL_DEPOSIT
                    - 1;

            // Fill the tank
            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                1001.into(),
                credits_1001
            ));

            // Assignment should happen after 2 sessions
            run_to_session(1u32);
            let assignment = CollatorAssignment::collator_container_chain();
            assert!(assignment.container_chains.is_empty());

            run_to_session(2u32);
            // Charlie and Dave should be assigned to para 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );

            // Simulate block inclusion from container chain 1001
            let mut sproof: ParaHeaderSproofBuilder = ParaHeaderSproofBuilder::default();
            let slot: u64 = 5;
            let s = ParaHeaderSproofBuilderItem {
                para_id: 1001.into(),
                author_id: HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                    parent_hash: Default::default(),
                    number: 1,
                    state_root: Default::default(),
                    extrinsics_root: Default::default(),
                    digest: sp_runtime::generic::Digest {
                        logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                    },
                }),
            };

            sproof.items.push(s);
            set_author_noting_inherent_data(sproof);

            run_block();

            // After this it should not be assigned anymore, since credits are not payable
            run_to_session(3u32);
            // Nobody should be assigned to para 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(assignment.container_chains.get(&1001u32.into()), None,);
        });
}

#[test]
fn test_block_credits_with_purchase_can_be_combined() {
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
            // Assert current slot gets updated
            assert_eq!(current_slot(), 1u64);
            assert!(current_author() == AccountId::from(BOB));

            // Alice and Bob collate in our chain
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            assert_eq!(authorities(), vec![alice_id, bob_id]);

            assert_ok!(Registrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data()
            ));
            assert_ok!(DataPreservers::set_boot_nodes(
                origin_of(ALICE.into()),
                1001.into(),
                dummy_boot_nodes()
            ));
            assert_ok!(Registrar::mark_valid_for_collating(
                root_origin(),
                1001.into()
            ));
            // Need to reset credits to 0 because now parachains are given free credits on register
            assert_ok!(ServicesPayment::set_block_production_credits(
                root_origin(),
                1001.into(),
                dancebox_runtime::Period::get()
            ));
            let credits_1001 =
                block_credits_to_required_balance(dancebox_runtime::Period::get(), 1001.into())
                    + dancebox_runtime::EXISTENTIAL_DEPOSIT;

            // Fill the tank
            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                1001.into(),
                credits_1001
            ));

            // Assignment should happen after 2 sessions
            run_to_session(1u32);
            let assignment = CollatorAssignment::collator_container_chain();
            assert!(assignment.container_chains.is_empty());
            run_to_session(2u32);
            // Charlie and Dave should be assigned to para 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );
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
fn test_ed_plus_collator_assignment_session_purchase_works() {
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
            // Assert current slot gets updated
            assert_eq!(current_slot(), 1u64);
            assert!(current_author() == AccountId::from(BOB));

            // Alice and Bob collate in our chain
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            assert_eq!(authorities(), vec![alice_id, bob_id]);

            assert_ok!(Registrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data()
            ));
            assert_ok!(DataPreservers::set_boot_nodes(
                origin_of(ALICE.into()),
                1001.into(),
                dummy_boot_nodes()
            ));
            assert_ok!(Registrar::mark_valid_for_collating(
                root_origin(),
                1001.into()
            ));
            // Need to reset credits to 0 because now parachains are given free credits on register
            assert_ok!(ServicesPayment::set_collator_assignment_credits(
                root_origin(),
                1001.into(),
                0
            ));
            let credits_1001 = collator_assignment_credits_to_required_balance(1, 1001.into())
                + dancebox_runtime::EXISTENTIAL_DEPOSIT;

            // Fill the tank
            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                1001.into(),
                credits_1001
            ));

            // Assignment should happen after 2 sessions
            run_to_session(1u32);
            let assignment = CollatorAssignment::collator_container_chain();
            assert!(assignment.container_chains.is_empty());
            run_to_session(2u32);
            // Charlie and Dave should be assigned to para 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );

            // Simulate block inclusion from container chain 1001
            let mut sproof: ParaHeaderSproofBuilder = ParaHeaderSproofBuilder::default();
            let slot: u64 = 5;
            let s = ParaHeaderSproofBuilderItem {
                para_id: 1001.into(),
                author_id: HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                    parent_hash: Default::default(),
                    number: 1,
                    state_root: Default::default(),
                    extrinsics_root: Default::default(),
                    digest: sp_runtime::generic::Digest {
                        logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                    },
                }),
            };
            sproof.items.push(s);
            set_author_noting_inherent_data(sproof);

            run_block();

            // After this it should not be assigned anymore, since credits are not payable
            run_to_session(4u32);
            // Nobody should be assigned to para 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(assignment.container_chains.get(&1001u32.into()), None,);
        });
}

#[test]
fn test_ed_plus_collator_assignment_credit_session_minus_1_purchase_fails() {
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
            // Assert current slot gets updated
            assert_eq!(current_slot(), 1u64);
            assert!(current_author() == AccountId::from(BOB));

            // Alice and Bob collate in our chain
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            assert_eq!(authorities(), vec![alice_id, bob_id]);

            assert_ok!(Registrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data()
            ));
            assert_ok!(DataPreservers::set_boot_nodes(
                origin_of(ALICE.into()),
                1001.into(),
                dummy_boot_nodes()
            ));
            assert_ok!(Registrar::mark_valid_for_collating(
                root_origin(),
                1001.into()
            ));
            // Need to reset credits to 0 because now parachains are given free credits on register
            assert_ok!(ServicesPayment::set_collator_assignment_credits(
                root_origin(),
                1001.into(),
                0
            ));
            let credits_1001 = collator_assignment_credits_to_required_balance(1, 1001.into())
                + dancebox_runtime::EXISTENTIAL_DEPOSIT
                - 1;

            // Fill the tank
            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                1001.into(),
                credits_1001
            ));

            // Assignment should happen after 2 sessions
            run_to_session(1u32);
            let assignment = CollatorAssignment::collator_container_chain();
            assert!(assignment.container_chains.is_empty());
            run_to_session(2u32);
            // Charlie and Dave should not be assigned to para 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(assignment.container_chains.get(&1001u32.into()), None,);
        });
}

#[test]
fn test_collator_assignment_credits_with_purchase_can_be_combined() {
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
            // Assert current slot gets updated
            assert_eq!(current_slot(), 1u64);
            assert!(current_author() == AccountId::from(BOB));

            // Alice and Bob collate in our chain
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            assert_eq!(authorities(), vec![alice_id, bob_id]);

            assert_ok!(Registrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data()
            ));
            assert_ok!(DataPreservers::set_boot_nodes(
                origin_of(ALICE.into()),
                1001.into(),
                dummy_boot_nodes()
            ));
            assert_ok!(Registrar::mark_valid_for_collating(
                root_origin(),
                1001.into()
            ));

            // We assign one session to free credits
            assert_ok!(ServicesPayment::set_collator_assignment_credits(
                root_origin(),
                1001.into(),
                1
            ));
            // We buy another session through the tank
            let credits_1001 = collator_assignment_credits_to_required_balance(1, 1001.into())
                + dancebox_runtime::EXISTENTIAL_DEPOSIT;

            // Fill the tank
            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                1001.into(),
                credits_1001
            ));

            // Assignment should happen after 2 sessions
            run_to_session(1u32);
            let assignment = CollatorAssignment::collator_container_chain();
            assert!(assignment.container_chains.is_empty());
            run_to_session(2u32);
            // Charlie and Dave should be assigned to para 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );
        });
}

#[test]
fn test_block_credits_and_collator_assignation_credits_through_tank() {
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
            // Assert current slot gets updated
            assert_eq!(current_slot(), 1u64);
            assert!(current_author() == AccountId::from(BOB));

            // Alice and Bob collate in our chain
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            assert_eq!(authorities(), vec![alice_id, bob_id]);

            assert_ok!(Registrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data()
            ));
            assert_ok!(DataPreservers::set_boot_nodes(
                origin_of(ALICE.into()),
                1001.into(),
                dummy_boot_nodes()
            ));
            assert_ok!(Registrar::mark_valid_for_collating(
                root_origin(),
                1001.into()
            ));

            // We make all free credits 0
            assert_ok!(ServicesPayment::set_collator_assignment_credits(
                root_origin(),
                1001.into(),
                0
            ));
            assert_ok!(ServicesPayment::set_block_production_credits(
                root_origin(),
                1001.into(),
                0
            ));

            // We buy 2 sessions through tank
            let collator_assignation_credits =
                collator_assignment_credits_to_required_balance(2, 1001.into());
            let block_production_credits =
                block_credits_to_required_balance(dancebox_runtime::Period::get() * 2, 1001.into());

            // Fill the tank
            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                1001.into(),
                collator_assignation_credits
                    + block_production_credits
                    + dancebox_runtime::EXISTENTIAL_DEPOSIT
            ));

            // Assignment should happen after 2 sessions
            run_to_session(1u32);
            let assignment = CollatorAssignment::collator_container_chain();
            assert!(assignment.container_chains.is_empty());
            run_to_session(2u32);
            // Charlie and Dave should be assigned to para 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );

            // After this it should not be assigned anymore, since credits are not payable
            run_to_session(4u32);
            // Nobody should be assigned to para 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(assignment.container_chains.get(&1001u32.into()), None,);
        });
}

#[test]
fn test_migration_services_collator_assignment_payment() {
    ExtBuilder::default().build().execute_with(|| {
        // Register a new parachain with no credits
        assert_ok!(Registrar::register(
            origin_of(ALICE.into()),
            1001.into(),
            empty_genesis_data()
        ));
        assert_ok!(DataPreservers::set_boot_nodes(
            origin_of(ALICE.into()),
            1001.into(),
            dummy_boot_nodes()
        ));
        assert_ok!(Registrar::mark_valid_for_collating(
            root_origin(),
            1001.into()
        ));
        // Register another parachain with no credits, do not mark this as valid for collation
        assert_ok!(Registrar::register(
            origin_of(ALICE.into()),
            1002.into(),
            empty_genesis_data()
        ));
        assert_ok!(DataPreservers::set_boot_nodes(
            origin_of(ALICE.into()),
            1002.into(),
            dummy_boot_nodes()
        ));
        assert_ok!(Registrar::mark_valid_for_collating(
            root_origin(),
            1002.into()
        ));

        // Need to reset credits to 0 because now parachains are given free credits on register
        assert_ok!(ServicesPayment::set_collator_assignment_credits(
            root_origin(),
            1001.into(),
            0
        ));
        assert_ok!(ServicesPayment::set_collator_assignment_credits(
            root_origin(),
            1002.into(),
            0
        ));

        let credits_1001 =
            pallet_services_payment::CollatorAssignmentCredits::<Runtime>::get(ParaId::from(1001))
                .unwrap_or_default();
        assert_eq!(credits_1001, 0);
        let credits_1002 =
            pallet_services_payment::CollatorAssignmentCredits::<Runtime>::get(ParaId::from(1002))
                .unwrap_or_default();
        assert_eq!(credits_1002, 0);

        // Apply migration
        let migration =
            MigrateServicesPaymentAddCollatorAssignmentCredits::<Runtime>(Default::default());
        migration.migrate(Default::default());

        // Both parachains have been given credits
        let credits_1001 =
            pallet_services_payment::CollatorAssignmentCredits::<Runtime>::get(ParaId::from(1001))
                .unwrap_or_default();
        assert_eq!(
            credits_1001,
            dancebox_runtime::FreeCollatorAssignmentCredits::get()
        );
        let credits_1002 =
            pallet_services_payment::CollatorAssignmentCredits::<Runtime>::get(ParaId::from(1002))
                .unwrap_or_default();
        assert_eq!(
            credits_1002,
            dancebox_runtime::FreeCollatorAssignmentCredits::get()
        );
    });
}

#[test]
fn test_max_collators_uses_pending_value() {
    // Start with max_collators = 100, and collators_per_container = 2
    // Set max_collators = 2, and collators_per_container = 3
    // It should be impossible to have more than 2 collators per container at any point in time
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
        .with_para_ids(vec![(
            1001,
            empty_genesis_data(),
            vec![],
            u32::MAX,
            u32::MAX,
        )
            .into()])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            min_orchestrator_collators: 1,
            max_orchestrator_collators: 1,
            collators_per_container: 2,
            full_rotation_period: 24,
            ..Default::default()
        })
        .build()
        .execute_with(|| {
            run_to_block(2);

            // Initial assignment: 1 collator in orchestrator chain and 2 collators in container 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(assignment.container_chains[&1001u32.into()].len(), 2);
            assert_eq!(assignment.orchestrator_chain.len(), 1);

            assert_ok!(Configuration::set_max_collators(root_origin(), 2));
            assert_ok!(Configuration::set_collators_per_container(root_origin(), 3));

            // Check invariant for all intermediate assignments. We set collators_per_container = 3
            // but we also set max_collators = 2, so no collators will be assigned to container
            // chains after the change is applied.
            for session in 1..=4 {
                run_to_session(session);

                let assignment = CollatorAssignment::collator_container_chain();
                assert!(
                    assignment.container_chains[&1001u32.into()].len() <= 2,
                    "session {}: {} collators assigned to container chain 1001",
                    session,
                    assignment.container_chains[&1001u32.into()].len()
                );
            }

            // Final assignment: because max_collators = 2, there are only 2 collators, one in
            // orchestrator chain, and the other one idle
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(assignment.container_chains[&1001u32.into()].len(), 0);
            assert_eq!(assignment.orchestrator_chain.len(), 1);
        });
}

#[test]
fn test_slow_adjusting_multiplier_changes_in_response_to_consumed_weight() {
    ExtBuilder::default()
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .build()
        .execute_with(|| {
            end_block();
            // If the block is full, the multiplier increases
            let before_multiplier = TransactionPayment::next_fee_multiplier();
            start_block();
            let max_block_weights = dancebox_runtime::RuntimeBlockWeights::get();
            frame_support::storage::unhashed::put(
                &frame_support::storage::storage_prefix(b"System", b"BlockWeight"),
                &ConsumedWeight::new(|class| {
                    max_block_weights
                        .get(class)
                        .max_total
                        .unwrap_or(Weight::MAX)
                }),
            );
            end_block();
            let current_multiplier = TransactionPayment::next_fee_multiplier();
            assert!(current_multiplier > before_multiplier);

            // If the block is empty, the multiplier decreases
            let before_multiplier = TransactionPayment::next_fee_multiplier();
            start_block();
            frame_support::storage::unhashed::put(
                &frame_support::storage::storage_prefix(b"System", b"BlockWeight"),
                &ConsumedWeight::new(|_class| Weight::zero()),
            );
            end_block();
            let current_multiplier = TransactionPayment::next_fee_multiplier();
            assert!(current_multiplier < before_multiplier);
        });
}

#[test]
fn test_collator_assignment_tip_priority_on_congestion() {
    ExtBuilder::default()
        .with_balances(vec![
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
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1003, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            let para_id = 1003u32;
            let tank_funds = 100 * UNIT;
            let max_tip = 1 * UNIT;

            assert_eq!(
                CollatorAssignment::collator_container_chain().container_chains[&1003u32.into()]
                    .len(),
                0
            );

            // Send funds to tank
            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                para_id.into(),
                tank_funds,
            ));

            // Set tip for 1003
            assert_ok!(ServicesPayment::set_max_tip(
                root_origin(),
                para_id.into(),
                Some(max_tip),
            ));

            run_to_session(2);
            assert_eq!(
                CollatorAssignment::collator_container_chain().container_chains[&para_id.into()]
                    .len(),
                2,
            );
        });
}

#[test]
fn test_collator_assignment_tip_charged_on_congestion() {
    ExtBuilder::default()
        .with_balances(vec![
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
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1003, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            let tank_funds = 100 * UNIT;
            let max_tip = 1 * UNIT;
            let para_id = 1003u32;

            // Send funds to tank
            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                para_id.into(),
                tank_funds,
            ));

            // Set tip for para_id
            assert_ok!(ServicesPayment::set_max_tip(
                root_origin(),
                para_id.into(),
                Some(max_tip),
            ));

            run_to_session(1);
            assert_eq!(
                Balances::usable_balance(ServicesPayment::parachain_tank(para_id.into())),
                tank_funds - max_tip,
            );
        });
}

#[test]
fn test_collator_assignment_tip_not_assigned_on_insufficient_balance() {
    ExtBuilder::default()
        .with_balances(vec![
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
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1003, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            let tank_funds = 1 * UNIT;
            let max_tip = 1 * UNIT;
            let para_id = 1003u32;

            // Send insufficient funds to tank for tip for 2 sessions
            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                para_id.into(),
                tank_funds,
            ));

            // Set tip for para_id
            assert_ok!(ServicesPayment::set_max_tip(
                root_origin(),
                para_id.into(),
                Some(max_tip),
            ));

            run_to_session(1);
            assert_eq!(
                CollatorAssignment::collator_container_chain().container_chains[&para_id.into()]
                    .len(),
                0
            );
        });
}

#[test]
fn test_collator_assignment_tip_only_charge_willing_paras() {
    ExtBuilder::default()
        .with_balances(vec![
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
            (AccountId::from(EVE), 100_000 * UNIT),
            (AccountId::from(FERDIE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
            (AccountId::from(EVE), 100 * UNIT),
            (AccountId::from(FERDIE), 100 * UNIT),
        ])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1003, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            let tank_funds = 100 * UNIT;
            let max_tip = 1 * UNIT;
            let para_id_with_tip = 1003u32;
            let para_id_without_tip = 1001u32;

            // Send funds to tank to both paras
            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                para_id_with_tip.into(),
                tank_funds,
            ));
            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                para_id_without_tip.into(),
                tank_funds,
            ));

            // Only set tip for 1003
            assert_ok!(ServicesPayment::set_max_tip(
                root_origin(),
                para_id_with_tip.into(),
                Some(max_tip),
            ));

            run_to_session(2);

            let assignment = CollatorAssignment::collator_container_chain().container_chains;

            // 2 out of the 3 paras should have collators assigned, with one paying tip to get
            // prioritized, and the other selected at random that should not be charged any tips
            assert_eq!(assignment[&para_id_with_tip.into()].len(), 2);
            assert_eq!(
                Balances::usable_balance(ServicesPayment::parachain_tank(para_id_with_tip.into())),
                tank_funds - max_tip * 2,
            );

            assert_eq!(assignment[&para_id_without_tip.into()].len(), 2);
            assert_eq!(
                Balances::usable_balance(ServicesPayment::parachain_tank(
                    para_id_without_tip.into()
                )),
                tank_funds,
            );
        });
}

#[test]
fn test_collator_assignment_tip_withdraw_min_tip() {
    ExtBuilder::default()
        .with_balances(vec![
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
            (AccountId::from(EVE), 100_000 * UNIT),
            (AccountId::from(FERDIE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
            (AccountId::from(EVE), 100 * UNIT),
            (AccountId::from(FERDIE), 100 * UNIT),
        ])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1002, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
            (1003, empty_genesis_data(), vec![], u32::MAX, u32::MAX).into(),
        ])
        .build()
        .execute_with(|| {
            let tank_funds = 100 * UNIT;
            let max_tip_1003 = 3 * UNIT;
            let max_tip_1002 = 2 * UNIT;
            let para_id_1003 = 1003u32;
            let para_id_1002 = 1002u32;

            // Send funds to tank to both paras
            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                para_id_1003.into(),
                tank_funds,
            ));
            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                para_id_1002.into(),
                tank_funds,
            ));

            // Set tips
            assert_ok!(ServicesPayment::set_max_tip(
                root_origin(),
                para_id_1003.into(),
                Some(max_tip_1003),
            ));
            assert_ok!(ServicesPayment::set_max_tip(
                root_origin(),
                para_id_1002.into(),
                Some(max_tip_1002),
            ));

            run_to_session(2);

            assert_eq!(
                CollatorAssignment::collator_container_chain().container_chains
                    [&para_id_1003.into()]
                    .len(),
                2
            );
            assert_eq!(
                CollatorAssignment::collator_container_chain().container_chains
                    [&para_id_1002.into()]
                    .len(),
                2
            );

            // Should have withdrawn the lowest tip from both paras
            assert_eq!(
                Balances::usable_balance(ServicesPayment::parachain_tank(para_id_1003.into())),
                tank_funds - max_tip_1002 * 2,
            );

            assert_eq!(
                Balances::usable_balance(ServicesPayment::parachain_tank(para_id_1002.into())),
                tank_funds - max_tip_1002 * 2,
            );
        });
}
