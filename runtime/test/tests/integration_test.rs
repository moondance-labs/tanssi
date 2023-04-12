#![cfg(test)]

mod common;
use common::*;
use cumulus_primitives_core::ParaId;
use frame_support::{assert_ok, BoundedVec};
use pallet_collator_assignment_runtime_api::runtime_decl_for_collator_assignment_api::CollatorAssignmentApi;
use pallet_registrar_runtime_api::runtime_decl_for_registrar_api::RegistrarApi;
use parity_scale_codec::Encode;
use sp_consensus_aura::AURA_ENGINE_ID;
use sp_core::Get;
use sp_runtime::{traits::BlakeTwo256, DigestItem};
use sp_std::vec;
use test_runtime::{AuthorNoting, CollatorAssignment, CollatorSelection, Configuration};
use tp_author_noting_inherent::{
    AuthorNotingSproofBuilder, AuthorNotingSproofBuilderItem, HeaderAs,
};

const UNIT: Balance = 1_000_000_000_000_000_000;

#[test]
fn genesis_balances() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
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
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .with_para_ids(vec![1001, 1002])
        .build()
        .execute_with(|| {
            assert_eq!(Registrar::registered_para_ids(), vec![1001, 1002]);
        });
}

#[test]
fn genesis_para_registrar_deregister() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .with_para_ids(vec![1001, 1002])
        .build()
        .execute_with(|| {
            assert_eq!(Registrar::registered_para_ids(), vec![1001, 1002]);

            run_to_block(2, false);
            assert_ok!(Registrar::deregister(root_origin(), 1002), ());

            // Pending
            assert_eq!(
                Registrar::pending_registered_para_ids(),
                vec![(2u32, BoundedVec::try_from(vec![1001u32]).unwrap())]
            );

            run_to_session(1, false);
            assert_eq!(
                Registrar::pending_registered_para_ids(),
                vec![(2u32, BoundedVec::try_from(vec![1001u32]).unwrap())]
            );
            assert_eq!(Registrar::registered_para_ids(), vec![1001, 1002]);

            run_to_session(2, false);
            assert_eq!(Registrar::pending_registered_para_ids(), vec![]);
            assert_eq!(Registrar::registered_para_ids(), vec![1001]);
        });
}

#[test]
fn genesis_para_registrar_runtime_api() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .with_para_ids(vec![1001, 1002])
        .build()
        .execute_with(|| {
            assert_eq!(Registrar::registered_para_ids(), vec![1001, 1002]);
            assert_eq!(Runtime::registered_paras(), vec![1001, 1002]);

            run_to_block(2, false);
            assert_ok!(Registrar::deregister(root_origin(), 1002), ());
            assert_eq!(Runtime::registered_paras(), vec![1001, 1002]);

            run_to_session(1, false);
            assert_eq!(Registrar::registered_para_ids(), vec![1001, 1002]);
            assert_eq!(Runtime::registered_paras(), vec![1001, 1002]);

            run_to_session(2, false);
            assert_eq!(Registrar::registered_para_ids(), vec![1001]);
            assert_eq!(Runtime::registered_paras(), vec![1001]);
        });
}

#[test]
fn test_author_collation_aura() {
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
        .with_para_ids(vec![1001, 1002])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            orchestrator_collators: 2,
            collators_per_container: 2,
        })
        .build()
        .execute_with(|| {
            run_to_block(5, true);
            // Assert current slot gets updated
            assert_eq!(Aura::current_slot(), 4u64);
            // slot 4, alice
            assert!(Authorship::author().unwrap() == AccountId::from(ALICE));

            run_to_block(6, true);

            assert_eq!(Aura::current_slot(), 5u64);
            // slot 5, bob
            assert!(Authorship::author().unwrap() == AccountId::from(BOB));
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
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .with_para_ids(vec![1001, 1002])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            orchestrator_collators: 2,
            collators_per_container: 2,
        })
        .build()
        .execute_with(|| {
            run_to_block(2, true);
            // Assert current slot gets updated
            assert_eq!(Aura::current_slot(), 1u64);
            assert!(Authorship::author().unwrap() == AccountId::from(BOB));

            // We change invulnerables
            // We first need to set the keys
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            let charlie_id = get_aura_id_from_seed(&AccountId::from(CHARLIE).to_string());
            let dave_id = get_aura_id_from_seed(&AccountId::from(DAVE).to_string());

            // Set CHARLIE and DAVE keys
            assert_ok!(Session::set_keys(
                origin_of(CHARLIE.into()),
                test_runtime::SessionKeys {
                    aura: charlie_id.clone(),
                },
                vec![]
            ));
            assert_ok!(Session::set_keys(
                origin_of(DAVE.into()),
                test_runtime::SessionKeys {
                    aura: dave_id.clone(),
                },
                vec![]
            ));

            // Set new invulnerables
            assert_ok!(CollatorSelection::set_invulnerables(
                root_origin(),
                vec![CHARLIE.into(), DAVE.into()]
            ));

            // SESSION CHANGE. First session. it takes 2 sessions to see the change
            run_to_session(1u32, true);
            // Session boundary even slot, ALICE.
            assert!(Authorship::author().unwrap() == AccountId::from(ALICE));
            assert!(Aura::authorities() == vec![alice_id, bob_id]);

            // Invulnerables should have triggered on new session authorities change
            run_to_session(2u32, true);
            assert!(Authorship::author().unwrap() == AccountId::from(CHARLIE));
            // Session boundary even slot, CHARLIE.
            assert!(Aura::authorities() == vec![charlie_id, dave_id]);
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
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .with_para_ids(vec![1001, 1002])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            orchestrator_collators: 2,
            collators_per_container: 2,
        })
        .build()
        .execute_with(|| {
            run_to_block(2, true);
            // Assert current slot gets updated
            assert_eq!(Aura::current_slot(), 1u64);
            assert!(Authorship::author().unwrap() == AccountId::from(BOB));

            // We change invulnerables
            // We first need to set the keys
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            let charlie_id = get_aura_id_from_seed(&AccountId::from(CHARLIE).to_string());
            let dave_id = get_aura_id_from_seed(&AccountId::from(DAVE).to_string());

            // Set CHARLIE and DAVE keys
            assert_ok!(Session::set_keys(
                origin_of(CHARLIE.into()),
                test_runtime::SessionKeys {
                    aura: charlie_id.clone(),
                },
                vec![]
            ));
            assert_ok!(Session::set_keys(
                origin_of(DAVE.into()),
                test_runtime::SessionKeys {
                    aura: dave_id.clone(),
                },
                vec![]
            ));

            // Set new invulnerables
            assert_ok!(CollatorSelection::set_invulnerables(
                root_origin(),
                vec![ALICE.into(), BOB.into(), CHARLIE.into(), DAVE.into()]
            ));

            // SESSION CHANGE. First session. it takes 2 sessions to see the change
            run_to_session(1u32, true);
            // Session boundary even slot, ALICE.
            assert!(Authorship::author().unwrap() == AccountId::from(ALICE));
            assert!(Aura::authorities() == vec![alice_id.clone(), bob_id.clone()]);

            // Invulnerables should have triggered on new session authorities change
            // However charlie and dave shoudl have gone to one para (1001)
            run_to_session(2u32, true);
            assert!(Aura::authorities() == vec![alice_id, bob_id]);
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32],
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
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            orchestrator_collators: 2,
            collators_per_container: 2,
        })
        .build()
        .execute_with(|| {
            run_to_block(2, true);
            // Assert current slot gets updated
            assert_eq!(Aura::current_slot(), 1u64);
            assert!(Authorship::author().unwrap() == AccountId::from(BOB));

            // Only Alice and Bob collate for our chain
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            // It does not matter if we insert more collators, only two will be assigned
            // FIXME(#32): should this work like this?
            assert_eq!(Aura::authorities(), vec![alice_id, bob_id]);
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
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            orchestrator_collators: 2,
            collators_per_container: 2,
        })
        .build()
        .execute_with(|| {
            run_to_block(2, true);
            // Assert current slot gets updated
            assert_eq!(Aura::current_slot(), 1u64);
            assert!(Authorship::author().unwrap() == AccountId::from(BOB));

            // Alice and Bob collate in our chain
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            assert_eq!(Aura::authorities(), vec![alice_id, bob_id]);

            assert_ok!(Registrar::register(root_origin(), 1001), ());
            assert_ok!(Registrar::register(root_origin(), 1002), ());

            // Assignment should happen after 2 sessions
            run_to_session(1u32, true);
            let assignment = CollatorAssignment::collator_container_chain();
            assert!(assignment.container_chains.is_empty());
            run_to_session(2u32, true);

            // Charlie and Dave should be assigne dot para 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32],
                vec![CHARLIE.into(), DAVE.into()]
            );
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
        .with_para_ids(vec![1001, 1002])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            orchestrator_collators: 2,
            collators_per_container: 2,
        })
        .build()
        .execute_with(|| {
            run_to_block(2, true);
            // Assert current slot gets updated
            assert_eq!(Aura::current_slot(), 1u64);
            assert!(Authorship::author().unwrap() == AccountId::from(BOB));

            // Alice and Bob are authorities
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            assert_eq!(Aura::authorities(), vec![alice_id, bob_id]);

            // Charlie and Dave to 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32],
                vec![CHARLIE.into(), DAVE.into()]
            );

            assert_ok!(Registrar::deregister(root_origin(), 1001), ());

            // Assignment should happen after 2 sessions
            run_to_session(1u32, true);

            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32],
                vec![CHARLIE.into(), DAVE.into()]
            );

            run_to_session(2u32, true);

            // Charlie and Dave should be assigne dot para 1002 this time
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1002u32],
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
        .with_para_ids(vec![1001, 1002])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            orchestrator_collators: 2,
            collators_per_container: 2,
        })
        .build()
        .execute_with(|| {
            run_to_block(2, true);
            // Assert current slot gets updated
            assert_eq!(Aura::current_slot(), 1u64);
            assert!(Authorship::author().unwrap() == AccountId::from(BOB));

            // Alice and Bob are authorities
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            assert_eq!(Aura::authorities(), vec![alice_id, bob_id]);

            // Set moondance collators to 1
            assert_ok!(
                Configuration::set_orchestrator_collators(root_origin(), 1),
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
                assignment.container_chains[&1001u32],
                vec![CHARLIE.into(), DAVE.into()]
            );

            // Assignment should happen after 2 sessions
            run_to_session(1u32, true);

            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32],
                vec![CHARLIE.into(), DAVE.into()]
            );

            run_to_session(2u32, true);

            // Charlie, Dave and BOB should be assigne dot para 1001 this time
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32],
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
        .with_para_ids(vec![1001, 1002])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            orchestrator_collators: 2,
            collators_per_container: 2,
        })
        .build()
        .execute_with(|| {
            run_to_block(2, true);
            // Assert current slot gets updated
            assert_eq!(Aura::current_slot(), 1u64);
            assert!(Authorship::author().unwrap() == AccountId::from(ALICE));

            // Alice and Bob are authorities
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());

            assert_eq!(Aura::authorities(), vec![alice_id]);
        });
}

#[test]
fn test_configuration_on_session_change() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 0,
            orchestrator_collators: 0,
            collators_per_container: 0,
        })
        .build()
        .execute_with(|| {
            run_to_block(1, false);
            assert_eq!(Configuration::config().max_collators, 0);
            assert_eq!(Configuration::config().orchestrator_collators, 0);
            assert_eq!(Configuration::config().collators_per_container, 0);
            assert_ok!(Configuration::set_max_collators(root_origin(), 50), ());

            run_to_session(1u32, false);

            assert_ok!(
                Configuration::set_orchestrator_collators(root_origin(), 20),
                ()
            );

            assert_eq!(Configuration::config().max_collators, 0);
            assert_eq!(Configuration::config().orchestrator_collators, 0);
            assert_eq!(Configuration::config().collators_per_container, 0);

            run_to_session(2u32, false);

            assert_ok!(
                Configuration::set_collators_per_container(root_origin(), 10),
                ()
            );
            assert_eq!(Configuration::config().max_collators, 50);
            assert_eq!(Configuration::config().orchestrator_collators, 0);
            assert_eq!(Configuration::config().collators_per_container, 0);

            run_to_session(3u32, false);

            assert_eq!(Configuration::config().max_collators, 50);
            assert_eq!(Configuration::config().orchestrator_collators, 20);
            assert_eq!(Configuration::config().collators_per_container, 0);

            run_to_session(4u32, false);

            assert_eq!(Configuration::config().max_collators, 50);
            assert_eq!(Configuration::config().orchestrator_collators, 20);
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
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .with_para_ids(vec![1001, 1002])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            orchestrator_collators: 2,
            collators_per_container: 2,
        })
        .build()
        .execute_with(|| {
            run_to_block(2, true);
            // Assert current slot gets updated
            assert_eq!(Aura::current_slot(), 1u64);
            assert!(Authorship::author().unwrap() == AccountId::from(BOB));
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
                test_runtime::SessionKeys {
                    aura: charlie_id.clone(),
                },
                vec![]
            ));
            assert_ok!(Session::set_keys(
                origin_of(DAVE.into()),
                test_runtime::SessionKeys {
                    aura: dave_id.clone(),
                },
                vec![]
            ));

            // Set new invulnerables
            assert_ok!(CollatorSelection::set_invulnerables(
                root_origin(),
                vec![ALICE.into(), BOB.into(), CHARLIE.into(), DAVE.into()]
            ));

            // SESSION CHANGE. First session. it takes 2 sessions to see the change
            run_to_session(1u32, true);
            // Session boundary even slot, ALICE.
            assert!(Authorship::author().unwrap() == AccountId::from(ALICE));
            assert!(Aura::authorities() == vec![alice_id.clone(), bob_id.clone()]);
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
            run_to_session(2u32, true);
            assert!(Aura::authorities() == vec![alice_id, bob_id]);
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32],
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
            assert_ok!(CollatorSelection::set_invulnerables(
                root_origin(),
                vec![ALICE.into(), CHARLIE.into(), DAVE.into()]
            ));

            run_to_session(3u32, true);
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

            run_to_session(4u32, true);
            assert_eq!(
                Runtime::parachain_collators(100.into()),
                Some(vec![ALICE.into()])
            );
            assert_eq!(
                Runtime::parachain_collators(1001.into()),
                Some(vec![CHARLIE.into(), DAVE.into()])
            );
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
fn test_author_noting_self_para_id_not_noting() {
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
            let mut sproof = AuthorNotingSproofBuilder::default();
            let slot: u64 = 5;
            let self_para = parachain_info::Pallet::<Runtime>::get();
            let mut s = AuthorNotingSproofBuilderItem::default();
            s.para_id = self_para;
            s.author_id = HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                parent_hash: Default::default(),
                number: Default::default(),
                state_root: Default::default(),
                extrinsics_root: Default::default(),
                digest: sp_runtime::generic::Digest {
                    logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                },
            });
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
        .with_para_ids(vec![1001])
        .build()
        .execute_with(|| {
            let mut sproof = AuthorNotingSproofBuilder::default();
            let slot: u64 = 5;
            let other_para: ParaId = 1001u32.into();

            // Charlie and Dave to 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32],
                vec![CHARLIE.into(), DAVE.into()]
            );

            let mut s = AuthorNotingSproofBuilderItem::default();
            s.para_id = other_para;
            s.author_id = HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                parent_hash: Default::default(),
                number: Default::default(),
                state_root: Default::default(),
                extrinsics_root: Default::default(),
                digest: sp_runtime::generic::Digest {
                    logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                },
            });
            sproof.items.push(s);

            set_author_noting_inherent_data(sproof);

            assert_eq!(
                AuthorNoting::latest_author(other_para),
                Some(AccountId::from(DAVE))
            );
        });
}
