use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use frame_support::traits::Currency;
use frame_support::BoundedVec;

use pallet_balances::Error as BalancesError;

macro_rules! bvec {
	($( $x:tt )*) => {
		vec![$( $x )*].try_into().unwrap()
	}
}

#[test]
fn add_letting_agent_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(PropertyManagement::add_letting_agent(
			RuntimeOrigin::root(),
			0,
			bvec![10, 10],
			[0; 32].into(),
		));
		assert_eq!(PropertyManagement::letting_info::<AccountId>([0; 32].into()).is_some(), true);
		let location: BoundedVec<u8, Postcode> = bvec![10, 10];
		assert_eq!(PropertyManagement::letting_info::<AccountId>([0; 32].into()).unwrap().locations[0], location);
	});
}

#[test]
fn add_letting_agent_fails() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_noop!(PropertyManagement::add_letting_agent(
			RuntimeOrigin::root(),
			0,
			bvec![10, 10],
			[0; 32].into(),
		), Error::<Test>::RegionUnknown);
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_noop!(PropertyManagement::add_letting_agent(
			RuntimeOrigin::root(),
			0,
			bvec![10, 10],
			[0; 32].into(),
		), Error::<Test>::LocationUnknown);
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(PropertyManagement::add_letting_agent(
			RuntimeOrigin::root(),
			0,
			bvec![10, 10],
			[0; 32].into(),
		));
		assert_eq!(PropertyManagement::letting_info::<AccountId>([0; 32].into()).is_some(), true);
		assert_noop!(PropertyManagement::add_letting_agent(
			RuntimeOrigin::root(),
			0,
			bvec![10, 10],
			[0; 32].into(),
		), Error::<Test>::LettingAgentExists);
	});
}

 #[test]
fn let_letting_agent_deposit() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(PropertyManagement::add_letting_agent(
			RuntimeOrigin::root(),
			0,
			bvec![10, 10],
			[0; 32].into(),
		));
		assert_ok!(PropertyManagement::letting_agent_deposit(RuntimeOrigin::signed([0; 32].into())));
		assert_eq!(PropertyManagement::letting_agent_locations::<u32, BoundedVec<u8, Postcode>>(0, bvec![10, 10]).contains(&[0; 32].into()), true);
		assert_eq!(Balances::free_balance(&([0; 32].into())), 19_999_900);
	});
}

#[test]
fn let_letting_agent_deposit_fails() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(PropertyManagement::add_letting_agent(
			RuntimeOrigin::root(),
			0,
			bvec![10, 10],
			[0; 32].into(),
		));
		assert_ok!(PropertyManagement::letting_agent_deposit(RuntimeOrigin::signed([0; 32].into())));
		assert_noop!(PropertyManagement::letting_agent_deposit(RuntimeOrigin::signed([0; 32].into())), Error::<Test>::LettingAgentInLocation);
		assert_noop!(PropertyManagement::letting_agent_deposit(RuntimeOrigin::signed([1; 32].into())), Error::<Test>::NoPermission);
		assert_eq!(Balances::free_balance(&([0; 32].into())), 19_999_900);
		for x in 1..100 {
			assert_ok!(PropertyManagement::add_letting_agent(
				RuntimeOrigin::root(),
				0,
				bvec![10, 10],
				[x; 32].into(),
			));
			Balances::make_free_balance_be(&[x; 32].into(), 200);
			assert_ok!(PropertyManagement::letting_agent_deposit(RuntimeOrigin::signed([x; 32].into())));
		}
		assert_ok!(PropertyManagement::add_letting_agent(
			RuntimeOrigin::root(),
			0,
			bvec![10, 10],
			[100; 32].into(),
		));
		Balances::make_free_balance_be(&[100; 32].into(), 200);
		assert_noop!(PropertyManagement::letting_agent_deposit(RuntimeOrigin::signed([100; 32].into())), Error::<Test>::TooManyLettingAgents);
	});
}

#[test]
fn let_letting_agent_deposit_not_enough_funds() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(PropertyManagement::add_letting_agent(
			RuntimeOrigin::root(),
			0,
			bvec![10, 10],
			[5; 32].into(),
		));
		assert_noop!(PropertyManagement::letting_agent_deposit(RuntimeOrigin::signed([5; 32].into())), BalancesError::<Test, _>::InsufficientBalance);
	});
}

#[test]
fn add_letting_agent_to_location_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![9, 10]));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(PropertyManagement::add_letting_agent(
			RuntimeOrigin::root(),
			0,
			bvec![9, 10],
			[0; 32].into(),
		));
		assert_eq!(PropertyManagement::letting_info::<AccountId>([0; 32].into()).is_some(), true);
		assert_ok!(PropertyManagement::letting_agent_deposit(RuntimeOrigin::signed([0; 32].into())));
		assert_ok!(PropertyManagement::add_letting_agent_to_location(RuntimeOrigin::root(), bvec![10, 10], [0; 32].into()));
		assert_eq!(PropertyManagement::letting_agent_locations::<u32, BoundedVec<u8, Postcode>>(0, bvec![9, 10]).contains(&[0; 32].into()), true);
		assert_eq!(PropertyManagement::letting_agent_locations::<u32, BoundedVec<u8, Postcode>>(0, bvec![10, 10]).contains(&[0; 32].into()), true);
		assert_eq!(PropertyManagement::letting_info::<AccountId>([0; 32].into()).unwrap().locations.len(), 2);
	});
}

#[test]
fn add_letting_agent_to_location_fails() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_noop!(PropertyManagement::add_letting_agent_to_location(RuntimeOrigin::root(), bvec![10, 10], [0; 32].into()), Error::<Test>::NoLettingAgentFound);
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![9, 10]));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(PropertyManagement::add_letting_agent(
			RuntimeOrigin::root(),
			0,
			bvec![10, 10],
			[0; 32].into(),
		));
		assert_eq!(PropertyManagement::letting_info::<AccountId>([0; 32].into()).is_some(), true);
		assert_ok!(PropertyManagement::letting_agent_deposit(RuntimeOrigin::signed([0; 32].into())));
		assert_noop!(PropertyManagement::add_letting_agent_to_location(RuntimeOrigin::root(), bvec![5, 10], [0; 32].into()), Error::<Test>::LocationUnknown);
		assert_noop!(PropertyManagement::add_letting_agent_to_location(RuntimeOrigin::root(), bvec![10, 10], [0; 32].into()), Error::<Test>::LettingAgentInLocation);
	});
}

#[test]
fn set_letting_agent_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			bvec![10, 10],
			10_000,
			100,
			bvec![22, 22]
		));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([1; 32].into()), 0, 100));
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			bvec![10, 10],
			1_000,
			100,
			bvec![22, 22]
		));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([1; 32].into()), 1, 100));
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			bvec![10, 10],
			1_000,
			100,
			bvec![22, 22]
		));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([1; 32].into()), 2, 100));
		assert_ok!(PropertyManagement::add_letting_agent(
			RuntimeOrigin::root(),
			0,
			bvec![10, 10],
			[2; 32].into(),
		));
		assert_ok!(PropertyManagement::add_letting_agent(
			RuntimeOrigin::root(),
			0,
			bvec![10, 10],
			[3; 32].into(),
		));
		assert_ok!(PropertyManagement::add_letting_agent(
			RuntimeOrigin::root(),
			0,
			bvec![10, 10],
			[4; 32].into(),
		));
		assert_ok!(PropertyManagement::letting_agent_deposit(RuntimeOrigin::signed([2; 32].into())));
		assert_ok!(PropertyManagement::letting_agent_deposit(RuntimeOrigin::signed([3; 32].into())));
		assert_ok!(PropertyManagement::letting_agent_deposit(RuntimeOrigin::signed([4; 32].into())));
		assert_ok!(PropertyManagement::set_letting_agent(RuntimeOrigin::signed([0; 32].into()), 0));
		assert_ok!(PropertyManagement::set_letting_agent(RuntimeOrigin::signed([0; 32].into()), 2));
		assert_ok!(PropertyManagement::set_letting_agent(RuntimeOrigin::signed([0; 32].into()), 3));
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			bvec![10, 10],
			1_000,
			100,
			bvec![22, 22]
		));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([1; 32].into()), 3, 100));
		assert_ok!(PropertyManagement::set_letting_agent(RuntimeOrigin::signed([0; 32].into()), 4));
		assert_eq!(PropertyManagement::letting_storage(0).unwrap(), [2; 32].into());
		assert_eq!(PropertyManagement::letting_storage(2).unwrap(), [3; 32].into());
		assert_eq!(PropertyManagement::letting_storage(3).unwrap(), [4; 32].into());
		assert_eq!(PropertyManagement::letting_agent_locations::<u32, BoundedVec<u8, Postcode>>(0, bvec![10, 10]).len(), 3);
		assert_eq!(PropertyManagement::letting_info::<AccountId>([2; 32].into()).unwrap().assigned_properties.len(), 2);
		assert_eq!(PropertyManagement::letting_info::<AccountId>([3; 32].into()).unwrap().assigned_properties.len(), 1);
		assert_eq!(PropertyManagement::letting_info::<AccountId>([4; 32].into()).unwrap().assigned_properties.len(), 1);
	});
}

#[test]
fn set_letting_agent_fails() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(PropertyManagement::add_letting_agent(
			RuntimeOrigin::root(),
			0,
			bvec![10, 10],
			[0; 32].into(),
		));
		assert_ok!(PropertyManagement::letting_agent_deposit(RuntimeOrigin::signed([0; 32].into())));
		assert_eq!(Balances::free_balance(&([0; 32].into())), 19_999_900);
		assert_noop!(PropertyManagement::set_letting_agent(RuntimeOrigin::signed([0; 32].into()), 0), Error::<Test>::NoObjectFound);
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			bvec![10, 10],
			100,
			100,
			bvec![22, 22]
		));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([1; 32].into()), 0, 100));
		assert_ok!(PropertyManagement::set_letting_agent(RuntimeOrigin::signed([0; 32].into()), 0));
		assert_noop!(PropertyManagement::set_letting_agent(RuntimeOrigin::signed([0; 32].into()), 0), Error::<Test>::LettingAgentAlreadySet);
		for x in 2..101 {
			assert_ok!(NftMarketplace::list_object(
				RuntimeOrigin::signed([0; 32].into()),
				0,
				bvec![10, 10],
				1_000,
				100,
				bvec![22, 22]
			));
			assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [(x); 32].into()));
			Balances::make_free_balance_be(&[x; 32].into(), 100_000);
			assert_ok!(Assets::mint(
				RuntimeOrigin::signed([0; 32].into()),
				1.into(),
				sp_runtime::MultiAddress::Id([x; 32].into()),
				1_000_000,
			));
			assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([x; 32].into()), (x as u32 - 1).into(), 100));
			assert_ok!(PropertyManagement::set_letting_agent(RuntimeOrigin::signed([0; 32].into()), x.into()));
		}
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			bvec![10, 10],
			10_000,
			100,
			bvec![22, 22]
		));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([1; 32].into()), 100, 100));
		assert_noop!(PropertyManagement::set_letting_agent(RuntimeOrigin::signed([0; 32].into()), 101), Error::<Test>::TooManyAssignedProperties);
	});
}

#[test]
fn set_letting_agent_no_letting_agent() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			bvec![10, 10],
			10_000,
			100,
			bvec![22, 22]
		));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([1; 32].into()), 0, 20));
		assert_noop!(PropertyManagement::set_letting_agent(RuntimeOrigin::signed([0; 32].into()), 0), Error::<Test>::NoLettingAgentFound);
	});
}

#[test]
fn distribute_income_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [2; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [3; 32].into()));
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			bvec![10, 10],
			10_000,
			100,
			bvec![22, 22]
		));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([1; 32].into()), 0, 20));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([2; 32].into()), 0, 30));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([3; 32].into()), 0, 50));
		assert_ok!(PropertyManagement::add_letting_agent(
			RuntimeOrigin::root(),
			0,
			bvec![10, 10],
			[4; 32].into(),
		));
		assert_ok!(PropertyManagement::letting_agent_deposit(RuntimeOrigin::signed([4; 32].into())));
		assert_ok!(PropertyManagement::set_letting_agent(RuntimeOrigin::signed([0; 32].into()), 0));
		assert_ok!(PropertyManagement::distribute_income(RuntimeOrigin::signed([4; 32].into()), 0, 200));
		assert_eq!(PropertyManagement::stored_funds::<AccountId>([1; 32].into()), 40);
		assert_eq!(PropertyManagement::stored_funds::<AccountId>([2; 32].into()), 60);
		assert_eq!(PropertyManagement::stored_funds::<AccountId>([3; 32].into()), 100);
		assert_eq!(Balances::free_balance(&([4; 32].into())), 4700);
	});
}

#[test]
fn distribute_income_fails() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			bvec![10, 10],
			10_000,
			100,
			bvec![22, 22]
		));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([1; 32].into()), 0, 100));
		assert_noop!(PropertyManagement::distribute_income(RuntimeOrigin::signed([5; 32].into()), 0, 200), Error::<Test>::NoLettingAgentFound);
		assert_eq!(PropertyManagement::stored_funds::<AccountId>([1; 32].into()), 0);
		assert_ok!(PropertyManagement::add_letting_agent(
			RuntimeOrigin::root(),
			0,
			bvec![10, 10],
			[4; 32].into(),
		));
		assert_ok!(PropertyManagement::letting_agent_deposit(RuntimeOrigin::signed([4; 32].into())));
		assert_ok!(PropertyManagement::set_letting_agent(RuntimeOrigin::signed([0; 32].into()), 0));
		assert_noop!(PropertyManagement::distribute_income(RuntimeOrigin::signed([5; 32].into()), 0, 200), Error::<Test>::NoPermission);
	});
}

#[test]
fn withdraw_funds_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			bvec![10, 10],
			10_000,
			100,
			bvec![22, 22]
		));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([1; 32].into()), 0, 100));
		assert_ok!(PropertyManagement::add_letting_agent(
			RuntimeOrigin::root(),
			0,
			bvec![10, 10],
			[4; 32].into(),
		));
		assert_ok!(PropertyManagement::letting_agent_deposit(RuntimeOrigin::signed([4; 32].into())));
		assert_ok!(PropertyManagement::set_letting_agent(RuntimeOrigin::signed([0; 32].into()), 0));
		assert_ok!(PropertyManagement::distribute_income(RuntimeOrigin::signed([4; 32].into()), 0, 200));
		assert_eq!(PropertyManagement::stored_funds::<AccountId>([1; 32].into()), 200);
		assert_eq!(Balances::free_balance(&([4; 32].into())), 4700);
		assert_eq!(Balances::free_balance(&PropertyManagement::account_id()), 5200);
		assert_ok!(PropertyManagement::withdraw_funds(RuntimeOrigin::signed([1; 32].into())));
		assert_eq!(Balances::free_balance(&PropertyManagement::account_id()), 5000);
		assert_eq!(Balances::free_balance(&([1; 32].into())), 15_000_200);
	});
}

#[test]
fn withdraw_funds_fails() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			bvec![10, 10],
			10_000,
			100,
			bvec![22, 22]
		));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([1; 32].into()), 0, 100));
		assert_ok!(PropertyManagement::add_letting_agent(
			RuntimeOrigin::root(),
			0,
			bvec![10, 10],
			[4; 32].into(),
		));
		assert_ok!(PropertyManagement::letting_agent_deposit(RuntimeOrigin::signed([4; 32].into())));
		assert_ok!(PropertyManagement::set_letting_agent(RuntimeOrigin::signed([0; 32].into()), 0));
		assert_ok!(PropertyManagement::distribute_income(RuntimeOrigin::signed([4; 32].into()), 0, 200));
		assert_eq!(PropertyManagement::stored_funds::<AccountId>([1; 32].into()), 200);
		assert_noop!(PropertyManagement::withdraw_funds(RuntimeOrigin::signed([2; 32].into())), Error::<Test>::UserHasNoFundsStored);
	});
}
