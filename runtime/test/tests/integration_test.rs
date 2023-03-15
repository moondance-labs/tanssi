#![cfg(test)]

mod common;
use common::*;
use frame_support::assert_ok;

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
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .build()
        .execute_with(|| {
            set_parachain_inherent_data();

            run_to_block(100, None);
        });
}
