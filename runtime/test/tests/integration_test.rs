#![cfg(test)]

mod common;
use common::*;

const GLMR: Balance = 1_000_000_000_000_000_000;

#[test]
fn genesis_balances() {
	ExtBuilder::default()
		.with_balances(vec![
			// Alice gets 10k extra tokens for her mapping deposit
			(AccountId::from(ALICE), 210_000 * GLMR),
			(AccountId::from(BOB), 100_000 * GLMR),
		])
		.build()
		.execute_with(|| {
			set_parachain_inherent_data();
			assert_eq!(
				Balances::usable_balance(AccountId::from(ALICE)),
				210_000 * GLMR,
			);
			assert_eq!(
				Balances::usable_balance(AccountId::from(BOB)),
				100_000 * GLMR,
			);
		});
}

#[test]
fn genesis_para_registrar() {
	ExtBuilder::default()
		.with_balances(vec![
			// Alice gets 10k extra tokens for her mapping deposit
			(AccountId::from(ALICE), 210_000 * GLMR),
			(AccountId::from(BOB), 100_000 * GLMR),
		])
		.with_para_ids(vec![
			1001, 1002
		])
		.build()
		.execute_with(|| {
			set_parachain_inherent_data();
			assert_eq!(
				Registrar::registered_para_ids(),
				vec![1001, 1002],
			);
		});
}