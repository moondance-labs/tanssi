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
    crate::tests::common::*,
    crate::{
        Balances, CollatorConfiguration, ContainerRegistrar, ServicesPayment,
        TanssiAuthorityMapping, TanssiInvulnerables,
    },
    cumulus_primitives_core::{relay_chain::HeadData, ParaId},
    frame_support::assert_ok,
    parity_scale_codec::Encode,
    sp_consensus_aura::AURA_ENGINE_ID,
    sp_runtime::{traits::BlakeTwo256, DigestItem},
    sp_std::vec,
    test_relay_sproof_builder::{HeaderAs, ParaHeaderSproofBuilder, ParaHeaderSproofBuilderItem},
};

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
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 2,
            min_orchestrator_collators: 0,
            max_orchestrator_collators: 0,
            collators_per_container: 2,
            ..Default::default()
        })
        .with_empty_parachains(vec![1000])
        .build()
        .execute_with(|| {
            run_to_block(2);
            // We change invulnerables
            // We first need to set the keys
            let alice_keys =
                get_authority_keys_from_seed(&AccountId::from(ALICE).to_string(), None);
            let bob_keys = get_authority_keys_from_seed(&AccountId::from(BOB).to_string(), None);

            // Set CHARLIE and DAVE keys
            let charlie_keys =
                get_authority_keys_from_seed(&AccountId::from(CHARLIE).to_string(), None);
            let dave_keys = get_authority_keys_from_seed(&AccountId::from(DAVE).to_string(), None);

            assert_ok!(Session::set_keys(
                origin_of(CHARLIE.into()),
                crate::SessionKeys {
                    babe: charlie_keys.babe.clone(),
                    grandpa: charlie_keys.grandpa.clone(),
                    para_validator: charlie_keys.para_validator.clone(),
                    para_assignment: charlie_keys.para_assignment.clone(),
                    authority_discovery: charlie_keys.authority_discovery.clone(),
                    beefy: charlie_keys.beefy.clone(),
                    nimbus: charlie_keys.nimbus.clone(),
                },
                vec![]
            ));

            assert_ok!(Session::set_keys(
                origin_of(DAVE.into()),
                crate::SessionKeys {
                    babe: dave_keys.babe.clone(),
                    grandpa: dave_keys.grandpa.clone(),
                    para_validator: dave_keys.para_validator.clone(),
                    para_assignment: dave_keys.para_assignment.clone(),
                    authority_discovery: dave_keys.authority_discovery.clone(),
                    beefy: dave_keys.beefy.clone(),
                    nimbus: dave_keys.nimbus.clone(),
                },
                vec![]
            ));

            // Change invulnerables
            assert_ok!(TanssiInvulnerables::remove_invulnerable(
                root_origin(),
                ALICE.into()
            ));
            assert_ok!(TanssiInvulnerables::remove_invulnerable(
                root_origin(),
                BOB.into()
            ));
            assert_ok!(TanssiInvulnerables::add_invulnerable(
                root_origin(),
                CHARLIE.into()
            ));
            assert_ok!(TanssiInvulnerables::add_invulnerable(
                root_origin(),
                DAVE.into()
            ));

            assert!(
                authorities_for_container(1000u32.into())
                    == Some(vec![alice_keys.nimbus.clone(), bob_keys.nimbus.clone()])
            );

            // SESSION CHANGE. First session. it takes 2 sessions to see the change
            run_to_session(1u32);

            assert!(babe_authorities() == vec![alice_keys.babe.clone(), bob_keys.babe.clone()]);
            assert!(
                authorities_for_container(1000u32.into())
                    == Some(vec![alice_keys.nimbus.clone(), bob_keys.nimbus.clone()])
            );

            // Invulnerables should have triggered on new session authorities change
            run_to_session(2u32);

            assert!(babe_authorities() == vec![alice_keys.babe.clone(), bob_keys.babe.clone()]);
            assert!(
                authorities_for_container(1000u32.into())
                    == Some(vec![charlie_keys.nimbus.clone(), dave_keys.nimbus.clone()])
            );
        });
}

#[test]
fn test_collators_per_container() {
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
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 2,
            min_orchestrator_collators: 0,
            max_orchestrator_collators: 0,
            collators_per_container: 2,
            ..Default::default()
        })
        .with_empty_parachains(vec![1000])
        .build()
        .execute_with(|| {
            run_to_block(2);
            let alice_keys =
                get_authority_keys_from_seed(&AccountId::from(ALICE).to_string(), None);
            let bob_keys = get_authority_keys_from_seed(&AccountId::from(BOB).to_string(), None);
            let charlie_keys =
                get_authority_keys_from_seed(&AccountId::from(CHARLIE).to_string(), None);

            assert_ok!(Session::set_keys(
                origin_of(CHARLIE.into()),
                crate::SessionKeys {
                    babe: charlie_keys.babe.clone(),
                    grandpa: charlie_keys.grandpa.clone(),
                    para_validator: charlie_keys.para_validator.clone(),
                    para_assignment: charlie_keys.para_assignment.clone(),
                    authority_discovery: charlie_keys.authority_discovery.clone(),
                    beefy: charlie_keys.beefy.clone(),
                    nimbus: charlie_keys.nimbus.clone(),
                },
                vec![]
            ));

            assert_ok!(TanssiInvulnerables::add_invulnerable(
                root_origin(),
                CHARLIE.into()
            ));

            // Initial assignment: Alice & Bob collating for container 1000
            assert!(
                authorities_for_container(1000u32.into())
                    == Some(vec![alice_keys.nimbus.clone(), bob_keys.nimbus.clone()])
            );

            // Change the collators_per_container param to 3.
            // This will imply that Charlie will join as a collator for container 1000.
            assert_ok!(CollatorConfiguration::set_collators_per_container(
                root_origin(),
                3
            ));

            // SESSION CHANGE. First session. it takes 2 sessions to see the change
            run_to_session(1u32);
            assert!(
                authorities_for_container(1000u32.into())
                    == Some(vec![alice_keys.nimbus.clone(), bob_keys.nimbus.clone()])
            );

            // We should see Charlie included in the authorities now
            run_to_session(2u32);
            assert!(
                authorities_for_container(1000u32.into())
                    == Some(vec![
                        alice_keys.nimbus.clone(),
                        bob_keys.nimbus.clone(),
                        charlie_keys.nimbus.clone()
                    ])
            );
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
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 2,
            min_orchestrator_collators: 0,
            max_orchestrator_collators: 0,
            collators_per_container: 2,
            ..Default::default()
        })
        .with_empty_parachains(vec![1000])
        .build()
        .execute_with(|| {
            run_to_block(2);
            let alice_keys =
                get_authority_keys_from_seed(&AccountId::from(ALICE).to_string(), None);
            let bob_keys = get_authority_keys_from_seed(&AccountId::from(BOB).to_string(), None);

            let alice_keys_2 = get_authority_keys_from_seed("ALICE2", None);
            let bob_keys_2 = get_authority_keys_from_seed("BOB2", None);

            let key_mapping_session_0 =
                TanssiAuthorityAssignment::collator_container_chain(0).unwrap();
            assert_eq!(
                key_mapping_session_0.container_chains[&1000u32.into()],
                vec![alice_keys.nimbus.clone(), bob_keys.nimbus.clone()],
            );

            let assignment = TanssiCollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1000u32.into()],
                vec![ALICE.into(), BOB.into()],
            );

            let key_mapping_session_1 =
                TanssiAuthorityAssignment::collator_container_chain(1).unwrap();
            assert_eq!(key_mapping_session_1, key_mapping_session_0,);
            let old_assignment_session_1 =
                TanssiCollatorAssignment::pending_collator_container_chain().unwrap();
            assert_eq!(
                old_assignment_session_1,
                TanssiCollatorAssignment::collator_container_chain(),
            );

            let key_mapping_session_2 = TanssiAuthorityAssignment::collator_container_chain(2);
            assert!(key_mapping_session_2.is_none());

            // Let's check Babe authorities to ensure nothing breaks
            assert_eq!(
                babe_authorities(),
                vec![alice_keys.babe.clone(), bob_keys.babe.clone()]
            );

            // Change Alice and Bob keys to something different
            // for now lets change it to alice_keys_2 and bob_keys_2
            assert_ok!(Session::set_keys(
                origin_of(ALICE.into()),
                crate::SessionKeys {
                    babe: alice_keys_2.babe.clone(),
                    grandpa: alice_keys_2.grandpa.clone(),
                    para_validator: alice_keys_2.para_validator.clone(),
                    para_assignment: alice_keys_2.para_assignment.clone(),
                    authority_discovery: alice_keys_2.authority_discovery.clone(),
                    beefy: alice_keys_2.beefy.clone(),
                    nimbus: alice_keys_2.nimbus.clone(),
                },
                vec![]
            ));

            assert_ok!(Session::set_keys(
                origin_of(BOB.into()),
                crate::SessionKeys {
                    babe: bob_keys_2.babe.clone(),
                    grandpa: bob_keys_2.grandpa.clone(),
                    para_validator: bob_keys_2.para_validator.clone(),
                    para_assignment: bob_keys_2.para_assignment.clone(),
                    authority_discovery: bob_keys_2.authority_discovery.clone(),
                    beefy: bob_keys_2.beefy.clone(),
                    nimbus: bob_keys_2.nimbus.clone(),
                },
                vec![]
            ));

            run_to_session(1u32);
            let old_key_mapping_session_1 = key_mapping_session_1;

            // Session 0 got removed
            let key_mapping_session_0 = TanssiAuthorityAssignment::collator_container_chain(0);
            assert!(key_mapping_session_0.is_none());

            // The values at session 1 did not change
            let key_mapping_session_1 =
                TanssiAuthorityAssignment::collator_container_chain(1).unwrap();
            assert_eq!(key_mapping_session_1, old_key_mapping_session_1,);
            assert_eq!(
                TanssiCollatorAssignment::collator_container_chain(),
                old_assignment_session_1,
            );

            // Session 2 uses the new keys
            let key_mapping_session_2 =
                TanssiAuthorityAssignment::collator_container_chain(2).unwrap();
            assert_eq!(
                key_mapping_session_2.container_chains[&1000u32.into()],
                vec![alice_keys_2.nimbus.clone(), bob_keys_2.nimbus.clone()],
            );
            assert_eq!(
                TanssiCollatorAssignment::pending_collator_container_chain(),
                None
            );

            let key_mapping_session_3 = TanssiAuthorityAssignment::collator_container_chain(3);
            assert!(key_mapping_session_3.is_none());

            // Check Babe authorities again
            assert_eq!(
                babe_authorities(),
                vec![alice_keys.babe.clone(), bob_keys.babe.clone()]
            );

            run_to_session(2u32);

            // Session 1 got removed
            let key_mapping_session_1 = TanssiAuthorityAssignment::collator_container_chain(1);
            assert!(key_mapping_session_1.is_none());

            // Session 2 uses the new keys
            let key_mapping_session_2 =
                TanssiAuthorityAssignment::collator_container_chain(2).unwrap();
            assert_eq!(
                key_mapping_session_2.container_chains[&1000u32.into()],
                vec![alice_keys_2.nimbus.clone(), bob_keys_2.nimbus.clone()],
            );
            assert_eq!(
                old_assignment_session_1,
                TanssiCollatorAssignment::collator_container_chain(),
            );

            // Session 3 uses the new keys
            let key_mapping_session_3 =
                TanssiAuthorityAssignment::collator_container_chain(3).unwrap();
            assert_eq!(
                key_mapping_session_3.container_chains[&1000u32.into()],
                vec![alice_keys_2.nimbus.clone(), bob_keys_2.nimbus.clone()],
            );
            assert_eq!(
                TanssiCollatorAssignment::pending_collator_container_chain(),
                None
            );

            let key_mapping_session_4 = TanssiAuthorityAssignment::collator_container_chain(4);
            assert!(key_mapping_session_4.is_none());

            // Check Babe authorities for the last time
            assert_eq!(
                babe_authorities(),
                vec![alice_keys_2.babe.clone(), bob_keys_2.babe.clone()]
            );
        });
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
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 2,
            min_orchestrator_collators: 0,
            max_orchestrator_collators: 0,
            collators_per_container: 2,
            ..Default::default()
        })
        .build()
        .execute_with(|| {
            run_to_block(2);
            let key_mapping_session_0 = TanssiAuthorityMapping::authority_id_mapping(0).unwrap();
            let key_mapping_session_1 = TanssiAuthorityMapping::authority_id_mapping(1).unwrap();

            let alice_keys =
                get_authority_keys_from_seed(&AccountId::from(ALICE).to_string(), None);
            let bob_keys = get_authority_keys_from_seed(&AccountId::from(BOB).to_string(), None);

            let alice_keys_2 = get_authority_keys_from_seed("ALICE2", None);
            let bob_keys_2 = get_authority_keys_from_seed("BOB2", None);

            assert_eq!(key_mapping_session_0.len(), 2);
            assert_eq!(
                key_mapping_session_0.get(&alice_keys.nimbus),
                Some(&ALICE.into())
            );
            assert_eq!(
                key_mapping_session_0.get(&bob_keys.nimbus),
                Some(&BOB.into())
            );

            // keys for session 1 should be identical
            assert_eq!(key_mapping_session_1.len(), 2);
            assert_eq!(
                key_mapping_session_1.get(&alice_keys.nimbus),
                Some(&ALICE.into())
            );
            assert_eq!(
                key_mapping_session_1.get(&bob_keys.nimbus),
                Some(&BOB.into())
            );

            // Check Babe authorities
            assert_eq!(
                babe_authorities(),
                vec![alice_keys.babe.clone(), bob_keys.babe.clone()]
            );

            // Change Alice and Bob keys to something different
            // for now lets change it to alice_keys_2 and bob_keys_2
            assert_ok!(Session::set_keys(
                origin_of(ALICE.into()),
                crate::SessionKeys {
                    babe: alice_keys_2.babe.clone(),
                    grandpa: alice_keys_2.grandpa.clone(),
                    para_validator: alice_keys_2.para_validator.clone(),
                    para_assignment: alice_keys_2.para_assignment.clone(),
                    authority_discovery: alice_keys_2.authority_discovery.clone(),
                    beefy: alice_keys_2.beefy.clone(),
                    nimbus: alice_keys_2.nimbus.clone(),
                },
                vec![]
            ));

            assert_ok!(Session::set_keys(
                origin_of(BOB.into()),
                crate::SessionKeys {
                    babe: bob_keys_2.babe.clone(),
                    grandpa: bob_keys_2.grandpa.clone(),
                    para_validator: bob_keys_2.para_validator.clone(),
                    para_assignment: bob_keys_2.para_assignment.clone(),
                    authority_discovery: bob_keys_2.authority_discovery.clone(),
                    beefy: bob_keys_2.beefy.clone(),
                    nimbus: bob_keys_2.nimbus.clone(),
                },
                vec![]
            ));

            run_to_session(1u32);
            let key_mapping_session_0 = TanssiAuthorityMapping::authority_id_mapping(0).unwrap();
            assert_eq!(key_mapping_session_0.len(), 2);
            assert_eq!(
                key_mapping_session_0.get(&alice_keys.nimbus),
                Some(&ALICE.into())
            );
            assert_eq!(
                key_mapping_session_0.get(&bob_keys.nimbus),
                Some(&BOB.into())
            );

            let key_mapping_session_1 = TanssiAuthorityMapping::authority_id_mapping(1).unwrap();
            assert_eq!(key_mapping_session_1.len(), 2);
            assert_eq!(
                key_mapping_session_1.get(&alice_keys.nimbus),
                Some(&ALICE.into())
            );
            assert_eq!(
                key_mapping_session_1.get(&bob_keys.nimbus),
                Some(&BOB.into())
            );

            // Keys have been scheduled for session 2
            let key_mapping_session_2 = TanssiAuthorityMapping::authority_id_mapping(2).unwrap();

            assert_eq!(key_mapping_session_2.len(), 2);
            assert_eq!(
                key_mapping_session_2.get(&alice_keys_2.nimbus),
                Some(&ALICE.into())
            );
            assert_eq!(
                key_mapping_session_2.get(&bob_keys_2.nimbus),
                Some(&BOB.into())
            );

            // Let's check Babe again
            assert_eq!(
                babe_authorities(),
                vec![alice_keys.babe.clone(), bob_keys.babe.clone()]
            );

            run_to_session(2u32);
            assert!(TanssiAuthorityMapping::authority_id_mapping(0).is_none());

            let key_mapping_session_1 = TanssiAuthorityMapping::authority_id_mapping(1).unwrap();
            assert_eq!(key_mapping_session_1.len(), 2);
            assert_eq!(
                key_mapping_session_1.get(&alice_keys.nimbus),
                Some(&ALICE.into())
            );
            assert_eq!(
                key_mapping_session_1.get(&bob_keys.nimbus),
                Some(&BOB.into())
            );

            let key_mapping_session_2 = TanssiAuthorityMapping::authority_id_mapping(2).unwrap();
            assert_eq!(key_mapping_session_2.len(), 2);
            assert_eq!(
                key_mapping_session_2.get(&alice_keys_2.nimbus),
                Some(&ALICE.into())
            );
            assert_eq!(
                key_mapping_session_2.get(&bob_keys_2.nimbus),
                Some(&BOB.into())
            );

            // Babe should be using the new keys
            assert_eq!(
                babe_authorities(),
                vec![alice_keys_2.babe.clone(), bob_keys_2.babe.clone()]
            );
        });
}

#[test]
fn test_authors_paras_inserted_a_posteriori() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 2,
            min_orchestrator_collators: 0,
            max_orchestrator_collators: 0,
            collators_per_container: 2,
            ..Default::default()
        })
        .build()
        .execute_with(|| {
            run_to_block(2);

            let alice_keys =
                get_authority_keys_from_seed(&AccountId::from(ALICE).to_string(), None);
            let bob_keys = get_authority_keys_from_seed(&AccountId::from(BOB).to_string(), None);

            assert_eq!(
                babe_authorities(),
                vec![alice_keys.babe.clone(), bob_keys.babe.clone()]
            );

            assert_ok!(ContainerRegistrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data(),
                Some(HeadData(vec![1u8, 1u8, 1u8]))
            ));

            // TODO: uncomment when we add DataPreservers
            // set_dummy_boot_node(origin_of(ALICE.into()), 1001.into());
            assert_ok!(ContainerRegistrar::mark_valid_for_collating(
                root_origin(),
                1001.into()
            ));

            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                1001.into(),
                block_credits_to_required_balance(1000, 1001.into())
            ));

            assert_ok!(ContainerRegistrar::register(
                origin_of(ALICE.into()),
                1002.into(),
                empty_genesis_data(),
                Some(HeadData(vec![1u8, 1u8, 1u8]))
            ));

            // TODO: uncomment when we add DataPreservers
            // set_dummy_boot_node(origin_of(ALICE.into()), 1002.into());
            assert_ok!(ContainerRegistrar::mark_valid_for_collating(
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
            let assignment = TanssiCollatorAssignment::collator_container_chain();
            assert!(assignment.container_chains.is_empty());
            run_to_session(2u32);

            // Alice and Bob should be assigned to para 1001
            let assignment = TanssiCollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![ALICE.into(), BOB.into()]
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
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
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
            assert_ok!(ContainerRegistrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data(),
                Some(HeadData(vec![1u8, 1u8, 1u8]))
            ));

            // TODO: uncomment when we add DataPreservers
            // set_dummy_boot_node(origin_of(ALICE.into()), 1001.into());

            assert_ok!(ContainerRegistrar::mark_valid_for_collating(
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
            let assignment = TanssiCollatorAssignment::collator_container_chain();
            assert!(assignment.container_chains.is_empty());
            run_to_session(2u32);

            // Alice and Bob are now assigned to para 1001
            let assignment = TanssiCollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
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
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .build()
        .execute_with(|| {
            assert_ok!(ContainerRegistrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data(),
                Some(HeadData(vec![1u8, 1u8, 1u8]))
            ));

            // TODO: uncomment when we add DataPreservers
            // set_dummy_boot_node(origin_of(ALICE.into()), 1001.into());

            assert_ok!(ContainerRegistrar::mark_valid_for_collating(
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
            let assignment = TanssiCollatorAssignment::collator_container_chain();
            assert!(assignment.container_chains.is_empty());
            run_to_session(2u32);

            // Nobody should be assigned to para 1001
            let assignment = TanssiCollatorAssignment::collator_container_chain();
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
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .build()
        .execute_with(|| {
            assert_ok!(ContainerRegistrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data(),
                Some(HeadData(vec![1u8, 1u8, 1u8]))
            ));

            // TODO: uncomment when we add DataPreservers
            // set_dummy_boot_node(origin_of(ALICE.into()), 1001.into());

            assert_ok!(ContainerRegistrar::mark_valid_for_collating(
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
            let credits_1001 = crate::EpochDurationInBlocks::get() - 1;
            assert_ok!(ServicesPayment::set_block_production_credits(
                root_origin(),
                1001.into(),
                credits_1001
            ));

            // Assignment should happen after 2 sessions
            run_to_session(1u32);
            let assignment = TanssiCollatorAssignment::collator_container_chain();
            assert!(assignment.container_chains.is_empty());
            run_to_session(2u32);
            // Nobody should be assigned to para 1001
            let assignment = TanssiCollatorAssignment::collator_container_chain();
            assert_eq!(assignment.container_chains.get(&1001u32.into()), None);

            // Now purchase the missing block credit
            assert_ok!(ServicesPayment::set_block_production_credits(
                root_origin(),
                1001.into(),
                credits_1001 + 1
            ));

            run_to_session(4u32);
            // Alice and Bob should be assigned to para 1001
            let assignment = TanssiCollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![ALICE.into(), BOB.into()]
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
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .build()
        .execute_with(|| {
            assert_ok!(ContainerRegistrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data(),
                Some(HeadData(vec![1u8, 1u8, 1u8]))
            ));

            // TODO: uncomment when we add DataPreservers
            // set_dummy_boot_node(origin_of(ALICE.into()), 1001.into());

            assert_ok!(ContainerRegistrar::mark_valid_for_collating(
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
            let credits_1001 = crate::EpochDurationInBlocks::get();
            assert_ok!(ServicesPayment::set_block_production_credits(
                root_origin(),
                1001.into(),
                credits_1001
            ));

            // Assignment should happen after 2 sessions
            run_to_session(1u32);
            let assignment = TanssiCollatorAssignment::collator_container_chain();
            assert!(assignment.container_chains.is_empty());
            run_to_session(2u32);
            // Alice and Bob should be assigned to para 1001
            let assignment = TanssiCollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![ALICE.into(), BOB.into()]
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
            let assignment = TanssiCollatorAssignment::collator_container_chain();
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
fn test_can_buy_credits_before_registering_para() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .build()
        .execute_with(|| {
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
fn test_can_buy_credits_before_registering_para_and_receive_free_credits() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
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
                    crate::FreeBlockProductionCredits::get() - 1,
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
                    crate::FreeBlockProductionCredits::get() - 1,
                    1001.into()
                )
            );

            let expected_cost = block_credits_to_required_balance(
                crate::FreeBlockProductionCredits::get() - 1,
                1001.into(),
            );
            assert_eq!(balance_before - balance_after, expected_cost);

            // Now register para
            assert_ok!(ContainerRegistrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data(),
                Some(HeadData(vec![1u8, 1u8, 1u8]))
            ));

            // TODO: uncomment when we add DataPreservers
            // set_dummy_boot_node(origin_of(ALICE.into()), 1001.into());

            assert_ok!(ContainerRegistrar::mark_valid_for_collating(
                root_origin(),
                1001.into()
            ));

            // We received free credits, because we cannot have more than MaxCreditsStored
            let credits =
                pallet_services_payment::BlockProductionCredits::<Runtime>::get(ParaId::from(1001))
                    .unwrap_or_default();
            assert_eq!(credits, crate::FreeBlockProductionCredits::get());
        });
}

#[test]
fn test_ed_plus_block_credit_session_purchase_works() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .build()
        .execute_with(|| {
            assert_ok!(ContainerRegistrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data(),
                Some(HeadData(vec![1u8, 1u8, 1u8]))
            ));

            // TODO: uncomment when we add DataPreservers
            // set_dummy_boot_node(origin_of(ALICE.into()), 1001.into());

            assert_ok!(ContainerRegistrar::mark_valid_for_collating(
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
                block_credits_to_required_balance(crate::EpochDurationInBlocks::get(), 1001.into())
                    + crate::EXISTENTIAL_DEPOSIT;

            // Fill the tank
            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                1001.into(),
                credits_1001
            ));

            // Assignment should happen after 2 sessions
            run_to_session(1u32);
            let assignment = TanssiCollatorAssignment::collator_container_chain();
            assert!(assignment.container_chains.is_empty());
            run_to_session(2u32);
            // Alice and Bob should be assigned to para 1001
            let assignment = TanssiCollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![ALICE.into(), BOB.into()]
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
            let assignment = TanssiCollatorAssignment::collator_container_chain();
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
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .build()
        .execute_with(|| {
            assert_ok!(ContainerRegistrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data(),
                Some(HeadData(vec![1u8, 1u8, 1u8]))
            ));

            // TODO: uncomment when we add DataPreservers
            // set_dummy_boot_node(origin_of(ALICE.into()), 1001.into());

            assert_ok!(ContainerRegistrar::mark_valid_for_collating(
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
                block_credits_to_required_balance(crate::EpochDurationInBlocks::get(), 1001.into())
                    + crate::EXISTENTIAL_DEPOSIT
                    - 1;

            // Fill the tank
            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                1001.into(),
                credits_1001
            ));

            // Assignment should happen after 2 sessions
            run_to_session(1u32);
            let assignment = TanssiCollatorAssignment::collator_container_chain();
            assert!(assignment.container_chains.is_empty());
            run_to_session(2u32);
            // Alice and Bob should not be assigned to para 1001
            let assignment = TanssiCollatorAssignment::collator_container_chain();
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
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .build()
        .execute_with(|| {
            assert_ok!(ContainerRegistrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data(),
                Some(HeadData(vec![1u8, 1u8, 1u8]))
            ));

            // TODO: uncomment when we add DataPreservers
            // set_dummy_boot_node(origin_of(ALICE.into()), 1001.into());

            assert_ok!(ContainerRegistrar::mark_valid_for_collating(
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
            let credits_1001 = block_credits_to_required_balance(
                crate::EpochDurationInBlocks::get() * 2,
                1001.into(),
            ) + crate::EXISTENTIAL_DEPOSIT;

            // Fill the tank
            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                1001.into(),
                credits_1001
            ));

            // Assignment should happen after 2 sessions
            run_to_session(1u32);
            let assignment = TanssiCollatorAssignment::collator_container_chain();
            assert!(assignment.container_chains.is_empty());

            run_to_session(2u32);
            // Alice and Bob should be assigned to para 1001
            let assignment = TanssiCollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![ALICE.into(), BOB.into()]
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
            // Alice and Bob should be assigned to para 1001
            let assignment = TanssiCollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![ALICE.into(), BOB.into()]
            );

            // After this it should not be assigned anymore, since credits are not payable
            run_to_session(4u32);

            // Nobody should be assigned to para 1001
            let assignment = TanssiCollatorAssignment::collator_container_chain();
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
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .build()
        .execute_with(|| {
            assert_ok!(ContainerRegistrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data(),
                Some(HeadData(vec![1u8, 1u8, 1u8]))
            ));

            // TODO: uncomment when we add DataPreservers
            // set_dummy_boot_node(origin_of(ALICE.into()), 1001.into());

            assert_ok!(ContainerRegistrar::mark_valid_for_collating(
                root_origin(),
                1001.into()
            ));
            // Need to reset credits to 0 because now parachains are given free credits on register
            assert_ok!(ServicesPayment::set_block_production_credits(
                root_origin(),
                1001.into(),
                0
            ));
            let credits_1001 = block_credits_to_required_balance(
                crate::EpochDurationInBlocks::get() * 2,
                1001.into(),
            ) + crate::EXISTENTIAL_DEPOSIT
                - 1;

            // Fill the tank
            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                1001.into(),
                credits_1001
            ));

            // Assignment should happen after 2 sessions
            run_to_session(1u32);
            let assignment = TanssiCollatorAssignment::collator_container_chain();
            assert!(assignment.container_chains.is_empty());

            run_to_session(2u32);
            // Alice and Bob should be assigned to para 1001
            let assignment = TanssiCollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![ALICE.into(), BOB.into()]
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
            let assignment = TanssiCollatorAssignment::collator_container_chain();
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
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .build()
        .execute_with(|| {
            assert_ok!(ContainerRegistrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data(),
                Some(HeadData(vec![1u8, 1u8, 1u8]))
            ));

            // TODO: uncomment when we add DataPreservers
            // set_dummy_boot_node(origin_of(ALICE.into()), 1001.into());

            assert_ok!(ContainerRegistrar::mark_valid_for_collating(
                root_origin(),
                1001.into()
            ));
            // Set 1 session of free credits and purchase 1 session of credits
            assert_ok!(ServicesPayment::set_block_production_credits(
                root_origin(),
                1001.into(),
                crate::EpochDurationInBlocks::get()
            ));
            let credits_1001 =
                block_credits_to_required_balance(crate::EpochDurationInBlocks::get(), 1001.into())
                    + crate::EXISTENTIAL_DEPOSIT;

            // Fill the tank
            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                1001.into(),
                credits_1001
            ));

            // Assignment should happen after 2 sessions
            run_to_session(1u32);
            let assignment = TanssiCollatorAssignment::collator_container_chain();
            assert!(assignment.container_chains.is_empty());
            run_to_session(2u32);
            // Alice and Bob should be assigned to para 1001
            let assignment = TanssiCollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![ALICE.into(), BOB.into()]
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
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .build()
        .execute_with(|| {
            assert_ok!(ContainerRegistrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data(),
                Some(HeadData(vec![1u8, 1u8, 1u8]))
            ));

            // TODO: uncomment when we add DataPreservers
            // set_dummy_boot_node(origin_of(ALICE.into()), 1001.into());

            assert_ok!(ContainerRegistrar::mark_valid_for_collating(
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
                + crate::EXISTENTIAL_DEPOSIT;

            // Fill the tank
            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                1001.into(),
                credits_1001
            ));

            // Assignment should happen after 2 sessions
            run_to_session(1u32);
            let assignment = TanssiCollatorAssignment::collator_container_chain();
            assert!(assignment.container_chains.is_empty());
            run_to_session(2u32);
            // Alice and Bob should be assigned to para 1001
            let assignment = TanssiCollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![ALICE.into(), BOB.into()]
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
            let assignment = TanssiCollatorAssignment::collator_container_chain();
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
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .build()
        .execute_with(|| {
            assert_ok!(ContainerRegistrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data(),
                Some(HeadData(vec![1u8, 1u8, 1u8]))
            ));

            // TODO: uncomment when we add DataPreservers
            // set_dummy_boot_node(origin_of(ALICE.into()), 1001.into());

            assert_ok!(ContainerRegistrar::mark_valid_for_collating(
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
                + crate::EXISTENTIAL_DEPOSIT
                - 1;

            // Fill the tank
            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                1001.into(),
                credits_1001
            ));

            // Assignment should happen after 2 sessions
            run_to_session(1u32);
            let assignment = TanssiCollatorAssignment::collator_container_chain();
            assert!(assignment.container_chains.is_empty());
            run_to_session(2u32);
            // Alice and Bob should not be assigned to para 1001
            let assignment = TanssiCollatorAssignment::collator_container_chain();
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
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .build()
        .execute_with(|| {
            assert_ok!(ContainerRegistrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data(),
                Some(HeadData(vec![1u8, 1u8, 1u8]))
            ));

            // TODO: uncomment when we add DataPreservers
            // set_dummy_boot_node(origin_of(ALICE.into()), 1001.into());

            assert_ok!(ContainerRegistrar::mark_valid_for_collating(
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
                + crate::EXISTENTIAL_DEPOSIT;

            // Fill the tank
            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                1001.into(),
                credits_1001
            ));

            // Assignment should happen after 2 sessions
            run_to_session(1u32);
            let assignment = TanssiCollatorAssignment::collator_container_chain();
            assert!(assignment.container_chains.is_empty());
            run_to_session(2u32);
            // Alice and Bob should be assigned to para 1001
            let assignment = TanssiCollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![ALICE.into(), BOB.into()]
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
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .build()
        .execute_with(|| {
            assert_ok!(ContainerRegistrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data(),
                Some(HeadData(vec![1u8, 1u8, 1u8]))
            ));

            // TODO: uncomment when we add DataPreservers
            // set_dummy_boot_node(origin_of(ALICE.into()), 1001.into());
            assert_ok!(ContainerRegistrar::mark_valid_for_collating(
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
            let block_production_credits = block_credits_to_required_balance(
                crate::EpochDurationInBlocks::get() * 2,
                1001.into(),
            );

            // Fill the tank
            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                1001.into(),
                collator_assignation_credits
                    + block_production_credits
                    + crate::EXISTENTIAL_DEPOSIT
            ));

            // Assignment should happen after 2 sessions
            run_to_session(1u32);
            let assignment = TanssiCollatorAssignment::collator_container_chain();
            assert!(assignment.container_chains.is_empty());
            run_to_session(2u32);
            // Alice and Bob should be assigned to para 1001
            let assignment = TanssiCollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![ALICE.into(), BOB.into()]
            );

            // After this it should not be assigned anymore, since credits are not payable
            run_to_session(4u32);
            // Nobody should be assigned to para 1001
            let assignment = TanssiCollatorAssignment::collator_container_chain();
            assert_eq!(assignment.container_chains.get(&1001u32.into()), None,);
        });
}

#[test]
fn test_collator_assignment_tip_priority_on_congestion() {
    ExtBuilder::default()
        .with_balances(vec![
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .with_empty_parachains(vec![1001, 1002, 1003])
        .build()
        .execute_with(|| {
            let para_id = 1003u32;
            let tank_funds = 100 * UNIT;
            let max_tip = 1 * UNIT;

            assert_eq!(
                TanssiCollatorAssignment::collator_container_chain().container_chains
                    [&1003u32.into()]
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
                TanssiCollatorAssignment::collator_container_chain().container_chains
                    [&para_id.into()]
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
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .with_empty_parachains(vec![1001, 1002, 1003])
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
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .with_empty_parachains(vec![1001, 1002, 1003])
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
                TanssiCollatorAssignment::collator_container_chain().container_chains
                    [&para_id.into()]
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
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .with_empty_parachains(vec![1001, 1002, 1003])
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

            assert_eq!(
                Balances::usable_balance(ServicesPayment::parachain_tank(
                    para_id_without_tip.into()
                )),
                tank_funds,
            );

            // Only set tip for 1003
            assert_ok!(ServicesPayment::set_max_tip(
                root_origin(),
                para_id_with_tip.into(),
                Some(max_tip),
            ));

            run_to_session(2);

            let assignment = TanssiCollatorAssignment::collator_container_chain().container_chains;

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
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .with_empty_parachains(vec![1001, 1002, 1003])
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
                TanssiCollatorAssignment::collator_container_chain().container_chains
                    [&para_id_1003.into()]
                    .len(),
                2
            );
            assert_eq!(
                TanssiCollatorAssignment::collator_container_chain().container_chains
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
