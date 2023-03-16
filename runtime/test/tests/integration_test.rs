#![cfg(test)]

mod common;
use common::*;
use frame_support::assert_ok;
use pallet_configuration::HostConfiguration;
use test_runtime::Configuration;

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
            set_parachain_inherent_data();
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
            set_parachain_inherent_data();
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
            set_parachain_inherent_data();
            assert_eq!(Registrar::registered_para_ids(), vec![1001, 1002]);

            run_to_block(2, None);
            assert_ok!(Registrar::deregister(root_origin(), 1002), ());
            assert_eq!(Registrar::registered_para_ids(), vec![1001]);
        });
}

#[test]
fn genesis_collators() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (
                AccountId::from(ALICE),
                get_aura_id_from_seed("Alice"),
                210 * UNIT,
            ),
            (
                AccountId::from(BOB),
                get_aura_id_from_seed("Bob"),
                100 * UNIT,
            ),
        ])
        .build()
        .execute_with(|| {
            set_parachain_inherent_data();

            run_to_block(100, None);
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
            (
                AccountId::from(ALICE),
                get_aura_id_from_seed("Alice"),
                210 * UNIT,
            ),
            (
                AccountId::from(BOB),
                get_aura_id_from_seed("Bob"),
                100 * UNIT,
            ),
        ])
        .with_para_ids(vec![1001, 1002])
        .build()
        .execute_with(|| {
            run_to_block(5, Some(AccountId::from(ALICE)));
            // Assert current slot gets updated
            assert_eq!(Aura::current_slot(), 4u64);
            // slot 4, alice
            assert!(Authorship::author().unwrap() == AccountId::from(ALICE));

            run_to_block(6, Some(AccountId::from(ALICE)));

            assert_eq!(Aura::current_slot(), 5u64);
            // slot 5, alice
            assert!(Authorship::author().unwrap() == AccountId::from(BOB));
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
            run_to_block(1, None);
            assert_eq!(Configuration::config().max_collators, 0);
            assert_eq!(Configuration::config().moondance_collators, 0);
            assert_eq!(Configuration::config().collators_per_container, 0);
            assert_ok!(Configuration::set_max_collators(root_origin(), 50), ());
            run_to_block(6, None);
            assert_ok!(
                Configuration::set_moondance_collators(root_origin(), 20),
                ()
            );
            assert_eq!(Configuration::config().max_collators, 0);
            assert_eq!(Configuration::config().moondance_collators, 0);
            assert_eq!(Configuration::config().collators_per_container, 0);
            run_to_block(11, None);
            assert_ok!(
                Configuration::set_collators_per_container(root_origin(), 10),
                ()
            );
            /*
            assert_eq!(
                <Configuration as Store>::PendingConfigs::get(),
                vec![
                    (
                        3,
                        HostConfiguration {
                            max_collators: 50,
                            moondance_collators: 20,
                            collators_per_container: 0,
                        }
                    ),
                    (
                        4,
                        HostConfiguration {
                            max_collators: 50,
                            moondance_collators: 20,
                            collators_per_container: 10,
                        }
                    )
                ]
            );
            */
            assert_eq!(Configuration::config().max_collators, 50);
            assert_eq!(Configuration::config().moondance_collators, 0);
            assert_eq!(Configuration::config().collators_per_container, 0);
            run_to_block(16, None);
            assert_eq!(Configuration::config().max_collators, 50);
            assert_eq!(Configuration::config().moondance_collators, 20);
            assert_eq!(Configuration::config().collators_per_container, 0);
            run_to_block(21, None);
            assert_eq!(Configuration::config().max_collators, 50);
            assert_eq!(Configuration::config().moondance_collators, 20);
            assert_eq!(Configuration::config().collators_per_container, 10);
        });
}
