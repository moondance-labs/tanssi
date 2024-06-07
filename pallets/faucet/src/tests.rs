use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, traits::StoredMap};

#[test]
fn single_claim_succeeds() {
	new_test_ext().execute_with(|| {
		let account_1 = Origin::signed(1);
		let account_2 = Origin::signed(2);
		let account_balance_1: u64 = AccountStore::get(&1).free;
		let account_balance_2: u64 = AccountStore::get(&2).free;
		let drip_amount: u64 = FaucetDripAmount::get();
		let total_issuance: u64 = Balances::total_issuance();

		assert_ok!(Faucet::claim_tokens(account_1));
		assert_eq!(AccountStore::get(&1).free, account_balance_1 + drip_amount);
		assert_eq!(Balances::total_issuance(), total_issuance + drip_amount);

		assert_ok!(Faucet::claim_tokens(account_2));
		assert_eq!(AccountStore::get(&2).free, account_balance_2 + drip_amount);
		assert_eq!(Balances::total_issuance(), total_issuance + (2 * drip_amount));

		assert_eq!(Faucet::last_claim_of(1), Some((1, 1)));
		assert_eq!(Faucet::last_claim_of(2), Some((1, 1)));
		assert_eq!(Faucet::total_amount_dripped(), (2 * drip_amount));

		// FaucetDripped events are raised
		assert!(System::events()
			.iter()
			.any(|er| er.event == TestEvent::Faucet(Event::FaucetDripped(drip_amount, 1))));
		assert!(System::events()
			.iter()
			.any(|er| er.event == TestEvent::Faucet(Event::FaucetDripped(drip_amount, 2))));
	});
}

#[test]
fn multiple_claims_fail_when_too_recent() {
	new_test_ext().execute_with(|| {
		let account = Origin::signed(1);

		// Note: mock runtime configured with
		// Faucet param MinBlocksBetweenClaims = 2

		// Block 1: Success
		System::set_block_number(1);
		assert_ok!(Faucet::claim_tokens(account.clone()));

		// Block 2: Fail
		System::set_block_number(2);
		assert_noop!(Faucet::claim_tokens(account.clone()), Error::<Test>::LastClaimTooRecent);

		// Block 3: Fail
		System::set_block_number(3);
		assert_noop!(Faucet::claim_tokens(account.clone()), Error::<Test>::LastClaimTooRecent);

		// Block 5: Success
		System::set_block_number(4);
		assert_ok!(Faucet::claim_tokens(account));
	});
}

#[test]
fn multiple_claims_succeed_when_delayed() {
	new_test_ext().execute_with(|| {
		let account = Origin::signed(1);
		let account_balance: u64 = AccountStore::get(&1).free;
		let drip_amount: u64 = FaucetDripAmount::get();
		let total_issuance: u64 = Balances::total_issuance();

		// Note: mock runtime configured with
		// Faucet param MinBlocksBetweenClaims = 2

		let mut times = 1;
		for b in (1..=7).step_by(MinBlocksBetweenClaims::get() as usize + 1) {
			// Claims at blocks 1,4,7 succeed; account balance & total issuance are updated accordingly
			System::set_block_number(b);
			assert_ok!(Faucet::claim_tokens(account.clone()));
			assert_eq!(AccountStore::get(&1).free, account_balance + (times * drip_amount));
			assert_eq!(Balances::total_issuance(), total_issuance + (times * drip_amount));
			times += 1;
		}

		assert_eq!(Faucet::last_claim_of(1), Some((3, 7)));
		assert_eq!(Faucet::total_amount_dripped(), (3 * drip_amount));
	});
}

#[test]
fn multiple_claims_fail_when_max_exceeded() {
	new_test_ext().execute_with(|| {
		let account = Origin::signed(1);

		// Note: mock runtime configured with
		// Faucet param MinBlocksBetweenClaims = 2
		// and MaxClaimsPerAccount = 3

		System::set_block_number(1);
		assert_ok!(Faucet::claim_tokens(account.clone()));
		System::set_block_number(4);
		assert_ok!(Faucet::claim_tokens(account.clone()));
		System::set_block_number(7);
		assert_ok!(Faucet::claim_tokens(account.clone()));
		System::set_block_number(10);
		assert_noop!(Faucet::claim_tokens(account.clone()), Error::<Test>::MaxClaimsExceeded);
		System::set_block_number(13);
		assert_noop!(Faucet::claim_tokens(account), Error::<Test>::MaxClaimsExceeded);
	});
}
