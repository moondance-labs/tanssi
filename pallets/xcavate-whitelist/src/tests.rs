use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::BadOrigin;

#[test]
fn add_to_whitelist_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Whitelist::add_to_whitelist(RuntimeOrigin::root(), 1));
		assert_eq!(Whitelist::whitelisted_accounts(1), true);
	});
}

#[test]
fn add_to_whitelist_fails_when_user_already_added() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Whitelist::add_to_whitelist(RuntimeOrigin::root(), 1));
		assert_noop!(
			Whitelist::add_to_whitelist(RuntimeOrigin::root(), 1),
			Error::<Test>::AccountAlreadyWhitelisted
		);
	});
}

#[test]
fn add_to_whitelist_fails_with_no_permission() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_noop!(Whitelist::add_to_whitelist(RuntimeOrigin::signed(2), 1), BadOrigin);
	});
}

#[test]
fn remove_from_whitelist_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Whitelist::add_to_whitelist(RuntimeOrigin::root(), 1));
		assert_ok!(Whitelist::remove_from_whitelist(RuntimeOrigin::root(), 1));
		assert_eq!(Whitelist::whitelisted_accounts(1), false);
	});
}

#[test]
fn remove_from_whitelist_fails_with_no_permission() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Whitelist::add_to_whitelist(RuntimeOrigin::root(), 1));
		assert_noop!(Whitelist::remove_from_whitelist(RuntimeOrigin::signed(2), 1), BadOrigin);
	});
}

#[test]
fn remove_from_whitelist_fails() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_noop!(
			Whitelist::remove_from_whitelist(RuntimeOrigin::root(), 1),
			Error::<Test>::UserNotInWhitelist
		);
	});
}
