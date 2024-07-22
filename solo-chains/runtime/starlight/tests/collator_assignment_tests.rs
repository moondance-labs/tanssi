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
    crate::common::*,
    frame_support::assert_ok,
    sp_std::vec,
    starlight_runtime::{
        CollatorConfiguration, ContainerRegistrar, TanssiAuthorityMapping, TanssiInvulnerables,
    },
};

mod common;

const UNIT: Balance = 1_000_000_000_000_000_000;

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
            let alice_keys = get_authority_keys_from_seed(&AccountId::from(ALICE).to_string());
            let bob_keys = get_authority_keys_from_seed(&AccountId::from(BOB).to_string());

            // Set CHARLIE and DAVE keys
            let charlie_keys = get_authority_keys_from_seed(&AccountId::from(CHARLIE).to_string());
            let dave_keys = get_authority_keys_from_seed(&AccountId::from(DAVE).to_string());

            assert_ok!(Session::set_keys(
                origin_of(CHARLIE.into()),
                starlight_runtime::SessionKeys {
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
                starlight_runtime::SessionKeys {
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
            let alice_keys = get_authority_keys_from_seed(&AccountId::from(ALICE).to_string());
            let bob_keys = get_authority_keys_from_seed(&AccountId::from(BOB).to_string());
            let charlie_keys = get_authority_keys_from_seed(&AccountId::from(CHARLIE).to_string());

            assert_ok!(Session::set_keys(
                origin_of(CHARLIE.into()),
                starlight_runtime::SessionKeys {
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
            let alice_keys = get_authority_keys_from_seed(&AccountId::from(ALICE).to_string());
            let bob_keys = get_authority_keys_from_seed(&AccountId::from(BOB).to_string());

            let alice_keys_2 = get_authority_keys_from_seed("ALICE2");
            let bob_keys_2 = get_authority_keys_from_seed("BOB2");

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
                starlight_runtime::SessionKeys {
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
                starlight_runtime::SessionKeys {
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

            let alice_keys = get_authority_keys_from_seed(&AccountId::from(ALICE).to_string());
            let bob_keys = get_authority_keys_from_seed(&AccountId::from(BOB).to_string());

            let alice_keys_2 = get_authority_keys_from_seed("ALICE2");
            let bob_keys_2 = get_authority_keys_from_seed("BOB2");

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
                starlight_runtime::SessionKeys {
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
                starlight_runtime::SessionKeys {
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

            let alice_keys = get_authority_keys_from_seed(&AccountId::from(ALICE).to_string());
            let bob_keys = get_authority_keys_from_seed(&AccountId::from(BOB).to_string());

            assert_eq!(
                babe_authorities(),
                vec![alice_keys.babe.clone(), bob_keys.babe.clone()]
            );

            assert_ok!(ContainerRegistrar::register(
                origin_of(ALICE.into()),
                1001.into(),
                empty_genesis_data()
            ));

            // TODO: uncomment when we add DataPreservers
            // set_dummy_boot_node(origin_of(ALICE.into()), 1001.into());
            assert_ok!(ContainerRegistrar::mark_valid_for_collating(
                root_origin(),
                1001.into()
            ));

            // TODO: uncomment when we add ServicesPayment
            /*  assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                1001.into(),
                block_credits_to_required_balance(1000, 1001.into())
            )); */

            assert_ok!(ContainerRegistrar::register(
                origin_of(ALICE.into()),
                1002.into(),
                empty_genesis_data()
            ));

            // TODO: uncomment when we add DataPreservers
            // set_dummy_boot_node(origin_of(ALICE.into()), 1002.into());
            assert_ok!(ContainerRegistrar::mark_valid_for_collating(
                root_origin(),
                1002.into()
            ));

            // TODO: uncomment when we add ServicesPayment
            /*  assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                1002.into(),
                block_credits_to_required_balance(1000, 1002.into())
            )); */

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
