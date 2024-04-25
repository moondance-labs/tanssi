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
    cumulus_primitives_core::{ParaId, Weight},
    dp_consensus::runtime_decl_for_tanssi_authority_assignment_api::TanssiAuthorityAssignmentApiV1,
    dp_core::well_known_keys,
    flashbox_runtime::{StreamPaymentAssetId, TimeUnit},
    frame_support::{assert_noop, assert_ok, BoundedVec},
    frame_system::ConsumedWeight,
    nimbus_primitives::NIMBUS_KEY_ID,
    pallet_author_noting::ContainerChainBlockInfo,
    pallet_author_noting_runtime_api::runtime_decl_for_author_noting_api::AuthorNotingApi,
    pallet_collator_assignment_runtime_api::runtime_decl_for_collator_assignment_api::CollatorAssignmentApi,
    pallet_migrations::Migration,
    pallet_registrar_runtime_api::{
        runtime_decl_for_registrar_api::RegistrarApi, ContainerChainGenesisData,
    },
    parity_scale_codec::Encode,
    runtime_common::migrations::MigrateServicesPaymentAddCollatorAssignmentCredits,
    sp_consensus_aura::AURA_ENGINE_ID,
    sp_core::Get,
    sp_runtime::{
        traits::{BadOrigin, BlakeTwo256, OpaqueKeys},
        DigestItem,
    },
    sp_std::vec,
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
                flashbox_runtime::SessionKeys {
                    nimbus: charlie_id.clone(),
                },
                vec![]
            ));
            assert_ok!(Session::set_keys(
                origin_of(DAVE.into()),
                flashbox_runtime::SessionKeys {
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
                flashbox_runtime::SessionKeys { nimbus: charlie_id },
                vec![]
            ));
            assert_ok!(Session::set_keys(
                origin_of(DAVE.into()),
                flashbox_runtime::SessionKeys { nimbus: dave_id },
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
            // However charlie and dave should have gone to one para (1001)
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
            full_rotation_period: 0,
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
            assert_eq!(assignment.container_chains.get(&1001u32.into()), None);
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
            let credits_1001 = flashbox_runtime::Period::get() - 1;
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
            let credits_1001 = flashbox_runtime::Period::get();
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
                flashbox_runtime::SessionKeys { nimbus: charlie_id },
                vec![]
            ));
            assert_ok!(Session::set_keys(
                origin_of(DAVE.into()),
                flashbox_runtime::SessionKeys { nimbus: dave_id },
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
                flashbox_runtime::SessionKeys {
                    nimbus: charlie_id.clone(),
                },
                vec![]
            ));
            assert_ok!(Session::set_keys(
                origin_of(DAVE.into()),
                flashbox_runtime::SessionKeys {
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
                flashbox_runtime::SessionKeys {
                    nimbus: charlie_id.clone(),
                },
                vec![]
            ));
            assert_ok!(Session::set_keys(
                origin_of(DAVE.into()),
                flashbox_runtime::SessionKeys {
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

            let session_two_edge = flashbox_runtime::Period::get() * 2;
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
                flashbox_runtime::SessionKeys {
                    nimbus: charlie_id.clone(),
                },
                vec![]
            ));
            assert_ok!(Session::set_keys(
                origin_of(DAVE.into()),
                flashbox_runtime::SessionKeys {
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

            let session_two_edge = flashbox_runtime::Period::get() * 2;
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
fn session_keys_key_type_id() {
    assert_eq!(
        flashbox_runtime::SessionKeys::key_ids(),
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
                flashbox_runtime::SessionKeys {
                    nimbus: alice_id_2.clone(),
                },
                vec![]
            ));
            assert_ok!(Session::set_keys(
                origin_of(BOB.into()),
                flashbox_runtime::SessionKeys {
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
                flashbox_runtime::SessionKeys {
                    nimbus: alice_id_2.clone(),
                },
                vec![]
            ));
            assert_ok!(Session::set_keys(
                origin_of(BOB.into()),
                flashbox_runtime::SessionKeys {
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
        | ProxyType::SudoRegistrar => (),
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
        flashbox_runtime::PalletInfo::name::<AuthorityAssignment>(),
        Some("AuthorityAssignment")
    );
    assert_eq!(
        well_known_keys::AUTHORITY_ASSIGNMENT_PREFIX,
        frame_support::storage::storage_prefix(b"AuthorityAssignment", b"CollatorContainerChain")
    );

    assert_eq!(
        flashbox_runtime::PalletInfo::name::<Session>(),
        Some("Session")
    );
    assert_eq!(
        well_known_keys::SESSION_INDEX,
        frame_support::storage::storage_prefix(b"Session", b"CurrentIndex")
    );
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
                flashbox_runtime::SessionKeys {
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

            // Try to buy (FreeBlockProductionCredits - 1) credits
            let balance_before = System::account(AccountId::from(ALICE)).data.free;
            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                1001.into(),
                block_credits_to_required_balance(
                    flashbox_runtime::FreeBlockProductionCredits::get() - 1,
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
                    flashbox_runtime::FreeBlockProductionCredits::get() - 1,
                    1001.into()
                )
            );

            let expected_cost = block_credits_to_required_balance(
                flashbox_runtime::FreeBlockProductionCredits::get() - 1,
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

            // We received aññ free credits, because we cannot have more than FreeBlockProductionCredits
            let credits =
                pallet_services_payment::BlockProductionCredits::<Runtime>::get(ParaId::from(1001))
                    .unwrap_or_default();
            assert_eq!(credits, flashbox_runtime::FreeBlockProductionCredits::get());
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
            assert_eq!(credits, flashbox_runtime::FreeBlockProductionCredits::get());
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
                flashbox_runtime::FreeBlockProductionCredits::get()
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
                block_credits_to_required_balance(flashbox_runtime::Period::get(), 1001.into())
                    + flashbox_runtime::EXISTENTIAL_DEPOSIT;

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
                block_credits_to_required_balance(flashbox_runtime::Period::get(), 1001.into())
                    + flashbox_runtime::EXISTENTIAL_DEPOSIT
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
                block_credits_to_required_balance(flashbox_runtime::Period::get() * 2, 1001.into())
                    + flashbox_runtime::EXISTENTIAL_DEPOSIT;

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
                block_credits_to_required_balance(flashbox_runtime::Period::get() * 2, 1001.into())
                    + flashbox_runtime::EXISTENTIAL_DEPOSIT
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
fn test_credits_with_purchase_can_be_combined() {
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
            // Set 1 session of free credits and purchase 1 session of credits
            assert_ok!(ServicesPayment::set_block_production_credits(
                root_origin(),
                1001.into(),
                flashbox_runtime::Period::get()
            ));
            let credits_1001 =
                block_credits_to_required_balance(flashbox_runtime::Period::get(), 1001.into())
                    + flashbox_runtime::EXISTENTIAL_DEPOSIT;

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
                + flashbox_runtime::EXISTENTIAL_DEPOSIT;

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
                + flashbox_runtime::EXISTENTIAL_DEPOSIT
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
                + flashbox_runtime::EXISTENTIAL_DEPOSIT;

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
                block_credits_to_required_balance(flashbox_runtime::Period::get() * 2, 1001.into());

            // Fill the tank
            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                1001.into(),
                collator_assignation_credits
                    + block_production_credits
                    + flashbox_runtime::EXISTENTIAL_DEPOSIT
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
            flashbox_runtime::FreeCollatorAssignmentCredits::get()
        );
        let credits_1002 =
            pallet_services_payment::CollatorAssignmentCredits::<Runtime>::get(ParaId::from(1002))
                .unwrap_or_default();
        assert_eq!(
            credits_1002,
            flashbox_runtime::FreeCollatorAssignmentCredits::get()
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
            let max_block_weights = flashbox_runtime::RuntimeBlockWeights::get();
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
