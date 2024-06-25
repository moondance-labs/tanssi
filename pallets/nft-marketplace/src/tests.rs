use crate::{mock::*, Error};
use crate::ItemId;
use frame_support::{assert_noop, assert_ok};
use frame_support::BoundedVec;

macro_rules! bvec {
	($( $x:tt )*) => {
		vec![$( $x )*].try_into().unwrap()
	}
}

// create_new_region function
#[test]
fn create_new_region_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_eq!(NftMarketplace::region_collections(0).unwrap(), 0);
		assert_eq!(NftMarketplace::region_collections(1).unwrap(), 1);
	})
}

// create_new_location function
#[test]
fn create_new_location_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![9, 10]));
 		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 1, bvec![9, 10])); 
 		assert_eq!(NftMarketplace::location_registration::<u32, BoundedVec<u8, Postcode>>(0, bvec![10, 10]), true);
 		assert_eq!(NftMarketplace::location_registration::<u32, BoundedVec<u8, Postcode>>(0, bvec![9, 10]), true);
		assert_eq!(NftMarketplace::location_registration::<u32, BoundedVec<u8, Postcode>>(1, bvec![9, 10]), true);
		assert_eq!(NftMarketplace::location_registration::<u32, BoundedVec<u8, Postcode>>(1, bvec![10, 10]), false);
		assert_eq!(NftMarketplace::location_registration::<u32, BoundedVec<u8, Postcode>>(1, bvec![8, 10]), false);  
	})
}

#[test]
fn create_new_location_fails() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_noop!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 1, bvec![10, 10]), Error::<Test>::RegionUnknown);
	})
}

// list_object function
#[test]
fn list_object_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			bvec![10, 10],
			10_000,
			100,
			bvec![22, 22]
		));
		assert_eq!(NftMarketplace::listed_token(0).unwrap(), 100);
		assert_eq!(NftMarketplace::next_nft_id(0), 1);
		assert_eq!(NftMarketplace::next_nft_id(1), 0);
		assert_eq!(NftMarketplace::next_asset_id(), 1);
		assert_eq!(NftMarketplace::ongoing_object_listing(0).is_some(), true);
		assert_eq!(NftMarketplace::registered_nft_details(0, 0).is_some(), true);
		assert_eq!(Uniques::owner(0, 0).unwrap(), NftMarketplace::account_id());
	})
}

#[test]
fn list_object_fails() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_noop!(
			NftMarketplace::list_object(
				RuntimeOrigin::signed([0; 32].into()),
				0,
				bvec![10, 10],
				10_000,
				100,
				bvec![22, 22]
			),
			Error::<Test>::RegionUnknown
		);
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_noop!(
			NftMarketplace::list_object(
				RuntimeOrigin::signed([0; 32].into()),
				0,
				bvec![10, 10],
				10_000,
				100,
				bvec![22, 22]
			),
			Error::<Test>::LocationUnknown
		);
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_noop!(
			NftMarketplace::list_object(
				RuntimeOrigin::signed([0; 32].into()),
				0,
				bvec![10, 10],
				10_000,
				251,
				bvec![22, 22]
			),
			Error::<Test>::TooManyToken
		);
	})
}
 
// buy_token function
#[test]
fn buy_token_works() {
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
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([1; 32].into()), 0, 30));
		assert_eq!(NftMarketplace::listed_token(0).unwrap(), 70);
		assert_eq!(NftMarketplace::token_owner::<AccountId, ItemId<Test>>([1; 32].into(), 0), 30);
		assert_eq!(NftMarketplace::token_buyer(0).len(), 1);
		assert_eq!(Balances::free_balance(&([1; 32].into())), 15_000_000);
		assert_eq!(Assets::balance(1, &[1; 32].into()), 1_200_000);
	})
}

#[test]
fn distributes_nfts_and_funds() {
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
		assert_eq!(Assets::balance(1, &[0; 32].into()), 20_990_000);
		assert_eq!(Assets::balance(1, &NftMarketplace::treasury_account_id()), 9000);
		assert_eq!(Assets::balance(1, &NftMarketplace::community_account_id()), 1000);
		assert_eq!(Assets::balance(1, &[1; 32].into()), 500_000);
		assert_eq!(NftMarketplace::registered_nft_details(0, 0).unwrap().spv_created, true);
		assert_eq!(NftMarketplace::listed_token(0), None);
		assert_eq!(NftMarketplace::token_owner::<AccountId, ItemId<Test>>([1; 32].into(), 0), 0);
		assert_eq!(NftMarketplace::token_buyer(0).len(), 0);
		assert_eq!(Assets::balance(0, &[1; 32].into()), 100);
	})
}

#[test]
fn buy_token_doesnt_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_noop!(
			NftMarketplace::buy_token(RuntimeOrigin::signed([0; 32].into()), 1, 1),
			Error::<Test>::TokenNotForSale
		);
	})
}

#[test]
fn buy_token_doesnt_work_2() {
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
		assert_noop!(
			NftMarketplace::buy_token(RuntimeOrigin::signed([1; 32].into()), 0, 101),
			Error::<Test>::NotEnoughTokenAvailable
		);
	})
}

#[test]
fn listing_and_selling_multiple_objects() {
	new_test_ext().execute_with(|| {
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [3; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [2; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([3; 32].into()),
			0,
			bvec![10, 10],
			10_000,
			100,
			bvec![22, 22]
		));
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([2; 32].into()),
			0,
			bvec![10, 10],
			10_000,
			100,
			bvec![22, 22]
		));
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			bvec![10, 10],
			10_000,
			100,
			bvec![22, 22]
		));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([1; 32].into()), 1, 80));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([1; 32].into()), 1, 20));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([1; 32].into()), 2, 10));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([1; 32].into()), 2, 10));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([2; 32].into()), 2, 30));
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([3; 32].into()),
			0,
			bvec![10, 10],
			10_000,
			100,
			bvec![22, 22]
		));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([2; 32].into()), 0, 33));
		assert_eq!(NftMarketplace::listed_token(0).unwrap(), 67);
		assert_eq!(NftMarketplace::listed_token(2).unwrap(), 50);
		assert_eq!(NftMarketplace::listed_token(3).unwrap(), 100);
		assert_eq!(NftMarketplace::token_owner::<AccountId, ItemId<Test>>([2; 32].into(), 2), 30);
		assert_eq!(NftMarketplace::token_buyer(2).len(), 2);
		assert_eq!(NftMarketplace::token_owner::<AccountId, ItemId<Test>>([1; 32].into(), 1), 0);
		assert_eq!(NftMarketplace::token_buyer(1).len(), 0);
		assert_eq!(NftMarketplace::property_owner_token::<u32, AccountId>(2, [1; 32].into()), 100);
	});
}

// list_token function
#[test]
fn relist_a_nft() {
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
		assert_eq!(NftMarketplace::registered_nft_details(0, 0).unwrap().spv_created, true);
		assert_ok!(NftMarketplace::relist_token(
			RuntimeOrigin::signed([1; 32].into()),
			0,
			0,
			1000,
			1
		));
		assert_eq!(NftMarketplace::token_listings(1).is_some(), true);
		assert_eq!(NftMarketplace::token_listings(1).unwrap().item_id, 0);
		assert_eq!(Assets::balance(0, NftMarketplace::account_id()), 1);
		assert_eq!(Assets::balance(0, &[1; 32].into()), 99);
	})
}

#[test]
fn relist_nfts_not_created_with_marketplace_fails() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(Uniques::create(
			RuntimeOrigin::signed([0; 32].into()),
			sp_runtime::MultiAddress::Id([0; 32].into()),
			Default::default()
		));
		assert_ok!(Uniques::mint(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			0,
			sp_runtime::MultiAddress::Id([0; 32].into()),
			None
		));
		assert_noop!(
			NftMarketplace::relist_token(RuntimeOrigin::signed([0; 32].into()), 0, 0, 1000, 1),
			Error::<Test>::RegionUnknown
		);
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_noop!(
			NftMarketplace::relist_token(RuntimeOrigin::signed([0; 32].into()), 0, 0, 1000, 1),
			Error::<Test>::NftNotFound
		);
	})
}

#[test]
fn relist_a_nft_fails() {
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
		assert_eq!(NftMarketplace::registered_nft_details(0, 0).unwrap().spv_created, true);
		assert_noop!(NftMarketplace::relist_token(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			0,
			1000,
			1
		), Error::<Test>::NotEnoughFunds);
	})
}

// buy_relisted_token function
#[test]
fn buy_relisted_token_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [3; 32].into()));
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			bvec![10, 10],
			10_000,
			100,
			bvec![22, 22]
		));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([1; 32].into()), 0, 100));
		assert_eq!(Assets::balance(1, &([0; 32].into())), 20990000);
		assert_eq!(Assets::balance(1, &NftMarketplace::treasury_account_id()), 9000);
		assert_eq!(Assets::balance(1, &NftMarketplace::community_account_id()), 1000);
		assert_eq!(Assets::balance(1, &([1; 32].into())), 500_000);
		assert_eq!(NftMarketplace::registered_nft_details(0, 0).unwrap().spv_created, true);
		assert_ok!(NftMarketplace::relist_token(
			RuntimeOrigin::signed([1; 32].into()),
			0,
			0,
			1000,
			3
		));
		assert_ok!(NftMarketplace::buy_relisted_token(RuntimeOrigin::signed([3; 32].into()), 1, 2));
		assert_eq!(NftMarketplace::token_listings(1).is_some(), true);
		assert_ok!(NftMarketplace::buy_relisted_token(RuntimeOrigin::signed([3; 32].into()), 1, 1));
		assert_eq!(NftMarketplace::token_listings(1).is_some(), false);
		assert_ok!(NftMarketplace::relist_token(
			RuntimeOrigin::signed([1; 32].into()),
			0,
			0,
			500,
			1
		));
		assert_ok!(NftMarketplace::buy_relisted_token(RuntimeOrigin::signed([3; 32].into()), 2, 1));
		assert_eq!(NftMarketplace::token_listings(0).is_some(), false);
		assert_eq!(NftMarketplace::property_owner(0).len(), 2);
		assert_eq!(NftMarketplace::property_owner_token::<u32, AccountId>(0, [1; 32].into()), 96);
		assert_eq!(NftMarketplace::property_owner_token::<u32, AccountId>(0, [3; 32].into()), 4);
		assert_eq!(Assets::balance(1, &([1; 32].into())), 503_465);
		assert_eq!(Assets::balance(1, &([3; 32].into())), 1_501);
		assert_eq!(Assets::balance(0, &[1; 32].into()), 96);
		assert_eq!(Assets::balance(0, &[3; 32].into()), 4);
	})
}

#[test]
fn buy_relisted_token_fails() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [3; 32].into()));
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			bvec![10, 10],
			10_000,
			100,
			bvec![22, 22]
		));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([1; 32].into()), 0, 100));
		assert_eq!(Assets::balance(1, &([0; 32].into())), 20990000);
		assert_eq!(Assets::balance(1, &NftMarketplace::treasury_account_id()), 9000);
		assert_eq!(Assets::balance(1, &NftMarketplace::community_account_id()), 1000);
		assert_eq!(Assets::balance(1, &([1; 32].into())), 500_000);
		assert_eq!(NftMarketplace::registered_nft_details(0, 0).unwrap().spv_created, true);
		assert_noop!(NftMarketplace::buy_relisted_token(RuntimeOrigin::signed([3; 32].into()), 1, 1), Error::<Test>::TokenNotForSale);
	})
}

// make_offer function
#[test]
fn make_offer_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [2; 32].into()));
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			bvec![10, 10],
			10_000,
			100,
			bvec![22, 22]
		));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([1; 32].into()), 0, 100));
		assert_ok!(NftMarketplace::relist_token(
			RuntimeOrigin::signed([1; 32].into()),
			0,
			0,
			500,
			1
		));
		assert_ok!(NftMarketplace::make_offer(
			RuntimeOrigin::signed([2; 32].into()),
			1,
			2000,
			1
		));
		assert_eq!(NftMarketplace::token_listings(1).is_some(), true);
		assert_eq!(NftMarketplace::ongoing_offers(1, 0).is_some(), true);
		assert_eq!(Assets::balance(1, &([2; 32].into())), 1_148_000);
		assert_eq!(Assets::balance(1, &NftMarketplace::account_id()), 2000);
	})
}

#[test]
fn make_offer_fails() {
	new_test_ext().execute_with(|| {
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [2; 32].into()));
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			bvec![10, 10],
			10_000,
			100,
			bvec![22, 22]
		));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([1; 32].into()), 0, 100));
		assert_noop!(NftMarketplace::make_offer(
			RuntimeOrigin::signed([2; 32].into()),
			1,
			200,
			1
		), Error::<Test>::TokenNotForSale);
		assert_ok!(NftMarketplace::relist_token(
			RuntimeOrigin::signed([1; 32].into()),
			0,
			0,
			500,
			1
		));
		assert_noop!(NftMarketplace::make_offer(
			RuntimeOrigin::signed([2; 32].into()),
			1,
			200,
			2
		), Error::<Test>::NotEnoughTokenAvailable);
	})
}

// handle_offer function
#[test]
fn handle_offer_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [2; 32].into()));
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			bvec![10, 10],
			10_000,
			100,
			bvec![22, 22]
		));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([1; 32].into()), 0, 100));
		assert_ok!(NftMarketplace::relist_token(
			RuntimeOrigin::signed([1; 32].into()),
			0,
			0,
			5000,
			20
		));
		assert_ok!(NftMarketplace::make_offer(
			RuntimeOrigin::signed([2; 32].into()),
			1,
			200,
			1
		));
		assert_ok!(NftMarketplace::handle_offer(
			RuntimeOrigin::signed([1; 32].into()),
			1,
			0,
			crate::Offer::Reject
		));
		assert_eq!(Assets::balance(1, &([2; 32].into())), 1_150_000);
		assert_eq!(Assets::balance(1, &NftMarketplace::account_id()), 0);
		assert_eq!(NftMarketplace::token_listings(1).is_some(), true);
		assert_eq!(NftMarketplace::ongoing_offers(1, 0).is_none(), true);
 		assert_ok!(NftMarketplace::make_offer(
			RuntimeOrigin::signed([2; 32].into()),
			1,
			2000,
			10
		));
		assert_eq!(Assets::balance(1, &([2; 32].into())), 1_130_000);
		assert_eq!(Assets::balance(1, &NftMarketplace::account_id()), 20000);
		assert_ok!(NftMarketplace::handle_offer(
			RuntimeOrigin::signed([1; 32].into()),
			1,
			1,
			crate::Offer::Accept
		)); 
		assert_eq!(NftMarketplace::token_listings(1).unwrap().amount, 10);
		assert_eq!(NftMarketplace::ongoing_offers(1, 1).is_none(), true);
		assert_eq!(Assets::balance(1, &NftMarketplace::account_id()), 0);
		assert_eq!(Assets::balance(0, &([1; 32].into())), 80);
		assert_eq!(Assets::balance(0, &([2; 32].into())), 10);
		assert_eq!(Assets::balance(0, &NftMarketplace::account_id()), 10);
		assert_eq!(Assets::balance(1, &([1; 32].into())), 519_800);
		assert_eq!(Assets::balance(1, &([2; 32].into())), 1_130_000);
	})
}

#[test]
fn handle_offer_fails() {
	new_test_ext().execute_with(|| {
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [2; 32].into()));
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			bvec![10, 10],
			10_000,
			100,
			bvec![22, 22]
		));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([1; 32].into()), 0, 100));
		assert_noop!(NftMarketplace::handle_offer(
			RuntimeOrigin::signed([1; 32].into()),
			1,
			0,
			crate::Offer::Reject
		), Error::<Test>::TokenNotForSale);
		assert_ok!(NftMarketplace::relist_token(
			RuntimeOrigin::signed([1; 32].into()),
			0,
			0,
			5000,
			2
		));
		assert_noop!(NftMarketplace::handle_offer(
			RuntimeOrigin::signed([1; 32].into()),
			1,
			0,
			crate::Offer::Reject
		), Error::<Test>::InvalidIndex);
		assert_ok!(NftMarketplace::make_offer(
			RuntimeOrigin::signed([2; 32].into()),
			1,
			200,
			1
		));
		assert_noop!(NftMarketplace::handle_offer(
			RuntimeOrigin::signed([2; 32].into()),
			1,
			0,
			crate::Offer::Accept
		), Error::<Test>::NoPermission);
	})
}

// cancel_offer function

#[test]
fn cancel_offer_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [2; 32].into()));
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			bvec![10, 10],
			10_000,
			100,
			bvec![22, 22]
		));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([1; 32].into()), 0, 100));
		assert_ok!(NftMarketplace::relist_token(
			RuntimeOrigin::signed([1; 32].into()),
			0,
			0,
			500,
			1
		));
		assert_ok!(NftMarketplace::make_offer(
			RuntimeOrigin::signed([2; 32].into()),
			1,
			2000,
			1
		));
		assert_eq!(NftMarketplace::token_listings(1).is_some(), true);
		assert_eq!(NftMarketplace::ongoing_offers(1, 0).is_some(), true);
		assert_eq!(Assets::balance(1, &([2; 32].into())), 1_148_000);
		assert_eq!(Assets::balance(1, &NftMarketplace::account_id()), 2000);
		assert_ok!(NftMarketplace::cancel_offer(
			RuntimeOrigin::signed([2; 32].into()),
			1,
			0
		));
		assert_eq!(NftMarketplace::token_listings(1).is_some(), true);
		assert_eq!(NftMarketplace::ongoing_offers(1, 0).is_some(), false);
		assert_eq!(Assets::balance(1, &([2; 32].into())), 1_150_000);
		assert_eq!(Assets::balance(1, &NftMarketplace::account_id()), 0);
	})
}


#[test]
fn cancel_offer_fails() {
	new_test_ext().execute_with(|| {
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [2; 32].into()));
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			bvec![10, 10],
			10_000,
			100,
			bvec![22, 22]
		));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([1; 32].into()), 0, 100));
		assert_ok!(NftMarketplace::relist_token(
			RuntimeOrigin::signed([1; 32].into()),
			0,
			0,
			500,
			1
		));
		assert_noop!(NftMarketplace::cancel_offer(
			RuntimeOrigin::signed([2; 32].into()),
			1,
			0
		), Error::<Test>::InvalidIndex);
		assert_ok!(NftMarketplace::make_offer(
			RuntimeOrigin::signed([2; 32].into()),
			1,
			2000,
			1
		));
		assert_eq!(NftMarketplace::token_listings(1).is_some(), true);
		assert_eq!(NftMarketplace::ongoing_offers(1, 0).is_some(), true);
		assert_eq!(Assets::balance(1, &([2; 32].into())), 1_148_000);
		assert_eq!(Assets::balance(1, &NftMarketplace::account_id()), 2000);
		assert_noop!(NftMarketplace::cancel_offer(
			RuntimeOrigin::signed([1; 32].into()),
			1,
			0
		), Error::<Test>::NoPermission);
	})
}

// upgrade_listing function
#[test]
fn upgrade_price_works() {
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
		assert_ok!(NftMarketplace::relist_token(
			RuntimeOrigin::signed([1; 32].into()),
			0,
			0,
			1000,
			1
		));
		assert_ok!(NftMarketplace::upgrade_listing(RuntimeOrigin::signed([1; 32].into()), 1, 300));
		assert_eq!(NftMarketplace::token_listings(1).unwrap().token_price, 300);
	})
}

#[test]
fn upgrade_price_fails_if_not_owner() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [4; 32].into()));
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			bvec![10, 10],
			10_000,
			100,
			bvec![22, 22]
		));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([1; 32].into()), 0, 100));
		assert_ok!(NftMarketplace::relist_token(
			RuntimeOrigin::signed([1; 32].into()),
			0,
			0,
			1000,
			1
		));
		assert_noop!(
			NftMarketplace::upgrade_listing(RuntimeOrigin::signed([4; 32].into()), 1, 300),
			Error::<Test>::NoPermission
		);
	})
}

// upgrade_object function
#[test]
fn upgrade_object_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			bvec![10, 10],
			10_000,
			100,
			bvec![22, 22]
		));
		assert_ok!(NftMarketplace::upgrade_object(RuntimeOrigin::signed([0; 32].into()), 0, 30000));
		assert_eq!(NftMarketplace::ongoing_object_listing(0).unwrap().token_price, 30000);
	})
}

#[test]
fn upgrade_object_and_distribute_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [2; 32].into()));
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			bvec![10, 10],
			10_000,
			100,
			bvec![22, 22]
		));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([1; 32].into()), 0, 50));
		assert_ok!(NftMarketplace::upgrade_object(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			20_000
		));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([2; 32].into()), 0, 50));
		assert_eq!(Assets::balance(1, &([0; 32].into())), 21485000);
		assert_eq!(Assets::balance(1, &NftMarketplace::treasury_account_id()), 13500);
		assert_eq!(Assets::balance(1, &NftMarketplace::community_account_id()), 1500);
		assert_eq!(Assets::balance(1, &([1; 32].into())), 1_000_000);
		assert_eq!(Assets::balance(1, &([2; 32].into())), 150_000);

		assert_eq!(NftMarketplace::registered_nft_details(0, 0).unwrap().spv_created, true);
		assert_eq!(NftMarketplace::listed_token(0), None);
	})
}


#[test]
fn upgrade_single_nft_from_listed_object_fails() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			bvec![10, 10],
			10_000,
			100,
			bvec![22, 22]
		));
		assert_noop!(
			NftMarketplace::upgrade_listing(RuntimeOrigin::signed([0; 32].into()), 0, 300),
			Error::<Test>::TokenNotForSale
		);
	})
}

#[test]
fn upgrade_object_for_relisted_nft_fails() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			bvec![10, 10],
			10_000,
			100,
			bvec![22, 22]
		));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([0; 32].into()), 0, 100));
		assert_ok!(NftMarketplace::relist_token(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			0,
			1000,
			1
		));
		assert_noop!(
			NftMarketplace::upgrade_object(RuntimeOrigin::signed([0; 32].into()), 1, 300),
			Error::<Test>::InvalidIndex
		);
	})
}

#[test]
fn upgrade_unknown_collection_fails() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_noop!(
			NftMarketplace::upgrade_object(RuntimeOrigin::signed([0; 32].into()), 0, 300),
			Error::<Test>::InvalidIndex
		);
	})
}

// delist_token function
#[test]
fn delist_single_token_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [2; 32].into()));
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			bvec![10, 10],
			10_000,
			100,
			bvec![22, 22]
		));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([1; 32].into()), 0, 100));
		assert_ok!(NftMarketplace::relist_token(
			RuntimeOrigin::signed([1; 32].into()),
			0,
			0,
			1000,
			1
		));
		assert_ok!(NftMarketplace::delist_token(RuntimeOrigin::signed([1; 32].into()), 1));
		assert_eq!(NftMarketplace::token_listings(0), None);
		assert_ok!(NftMarketplace::relist_token(
			RuntimeOrigin::signed([1; 32].into()),
			0,
			0,
			1000,
			3
		));
		assert_ok!(NftMarketplace::buy_relisted_token(RuntimeOrigin::signed([2; 32].into()), 2, 2));
		assert_ok!(NftMarketplace::delist_token(RuntimeOrigin::signed([1; 32].into()), 2));
		assert_eq!(Assets::balance(0, &[2; 32].into()), 2);
		assert_eq!(Assets::balance(0, &[1; 32].into()), 98);
	})
}

#[test]
fn delist_fails() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [4; 32].into()));
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			bvec![10, 10],
			10_000,
			100,
			bvec![22, 22]
		));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([1; 32].into()), 0, 100));
		assert_ok!(NftMarketplace::relist_token(
			RuntimeOrigin::signed([1; 32].into()),
			0,
			0,
			1000,
			1
		));
		assert_noop!(
			NftMarketplace::delist_token(RuntimeOrigin::signed([4; 32].into()), 1),
			Error::<Test>::NoPermission
		);
		assert_noop!(
			NftMarketplace::delist_token(RuntimeOrigin::signed([1; 32].into()), 2),
			Error::<Test>::TokenNotForSale
		);
	})
}



#[test]
fn listing_objects_in_different_regions() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 1, bvec![10, 10]));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 2, bvec![10, 10]));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [2; 32].into()));
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			bvec![10, 10],
			10_000,
			100,
			bvec![22, 22]
		));
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([0; 32].into()),
			1,
			bvec![10, 10],
			10_000,
			100,
			bvec![22, 22]
		));
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([0; 32].into()),
			2,
			bvec![10, 10],
			10_000,
			100,
			bvec![22, 22]
		));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([1; 32].into()), 1, 100));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([2; 32].into()), 2, 100));
		assert_eq!(NftMarketplace::registered_nft_details(1, 0).unwrap().spv_created, true);
		assert_eq!(NftMarketplace::registered_nft_details(2, 0).unwrap().spv_created, true);
		assert_ok!(NftMarketplace::relist_token(
			RuntimeOrigin::signed([1; 32].into()),
			1,
			0,
			1000,
			100
		));
		assert_ok!(NftMarketplace::buy_relisted_token(RuntimeOrigin::signed([2; 32].into()), 3, 100));
		assert_eq!(Assets::balance(2, &[2; 32].into()), 100);
		assert_eq!(Assets::balance(3, &[2; 32].into()), 100);
	}) 
}  