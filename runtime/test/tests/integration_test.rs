#![cfg(test)]

mod common;
use common::*;
use frame_support::{assert_ok, BoundedVec};
use test_runtime::{CollatorSelection, Configuration};

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
        .build()
        .execute_with(|| {
            run_to_block(2, true);
            // Assert current slot gets updated
            assert_eq!(Aura::current_slot(), 1u64);
            // slot 4, alice
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
fn test_configuration_on_session_change() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            run_to_block(1, false);
            assert_eq!(Configuration::config().max_collators, 0);
            assert_eq!(Configuration::config().moondance_collators, 0);
            assert_eq!(Configuration::config().collators_per_container, 0);
            assert_ok!(Configuration::set_max_collators(root_origin(), 50), ());

            run_to_session(1u32, false);

            assert_ok!(
                Configuration::set_moondance_collators(root_origin(), 20),
                ()
            );

            assert_eq!(Configuration::config().max_collators, 0);
            assert_eq!(Configuration::config().moondance_collators, 0);
            assert_eq!(Configuration::config().collators_per_container, 0);

            run_to_session(2u32, false);

            assert_ok!(
                Configuration::set_collators_per_container(root_origin(), 10),
                ()
            );
            assert_eq!(Configuration::config().max_collators, 50);
            assert_eq!(Configuration::config().moondance_collators, 0);
            assert_eq!(Configuration::config().collators_per_container, 0);

            run_to_session(3u32, false);

            assert_eq!(Configuration::config().max_collators, 50);
            assert_eq!(Configuration::config().moondance_collators, 20);
            assert_eq!(Configuration::config().collators_per_container, 0);

            run_to_session(4u32, false);

            assert_eq!(Configuration::config().max_collators, 50);
            assert_eq!(Configuration::config().moondance_collators, 20);
            assert_eq!(Configuration::config().collators_per_container, 10);
        });
}
