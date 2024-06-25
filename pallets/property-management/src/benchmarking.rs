//! Benchmarking setup for pallet-property-management
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as PropertyManagement;
use frame_benchmarking::__private::vec;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use frame_support::sp_runtime::traits::Bounded;
use pallet_nft_marketplace::Pallet as NftMarketplace;
 type DepositBalanceOf<T> = <<T as pallet_nfts::Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::Balance;  
type DepositBalanceOf1<T> =
	<<T as pallet::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
use pallet_xcavate_whitelist::Pallet as Whitelist;
use frame_support::{traits::Get, assert_ok};
use frame_support::BoundedVec;
use frame_support::sp_runtime::traits::StaticLookup;

type BalanceOf1<T> = <<T as pallet_nft_marketplace::Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::Balance; 
use pallet_assets::Pallet as Assets;

fn setup_real_estate_object<T: Config>() -> BoundedVec<u8, <T as pallet_nft_marketplace::Config>::PostcodeLimit> {
	let value: BalanceOf1<T> = 100_000u32.into();
	let caller: T::AccountId = whitelisted_caller();
	<T as pallet_nfts::Config>::Currency::make_free_balance_be(
		&caller,
		DepositBalanceOf::<T>::max_value(),
	);
	assert_ok!(NftMarketplace::<T>::create_new_region(RawOrigin::Root.into()));
	let location: BoundedVec<u8, <T as pallet_nft_marketplace::Config>::PostcodeLimit> = vec![0; <T as pallet_nft_marketplace::Config>::PostcodeLimit::get() as usize]
		.try_into()
		.unwrap();
	assert_ok!(NftMarketplace::<T>::create_new_location(RawOrigin::Root.into(), 0, location.clone()));
	assert_ok!(Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), caller.clone()));
	let user_lookup = <T::Lookup as StaticLookup>::unlookup(caller.clone());
	let asset_id = <T as pallet::Config>::Helper::to_asset(1);
	assert_ok!(Assets::<T, Instance1>::create(
		RawOrigin::Signed(caller.clone()).into(),
		asset_id.clone().into(),
		user_lookup.clone(),
		1u32.into(),
	));
	assert_ok!(Assets::<T, Instance1>::mint(
		RawOrigin::Signed(caller.clone()).into(),
		asset_id.clone().into(),
		user_lookup,
		1_000_000u32.into(),
	));
	assert_ok!(NftMarketplace::<T>::list_object(
		RawOrigin::Signed(caller.clone()).into(),
		0,
		location.clone(),
		1u32.into(),
		100,
		vec![0; <T as pallet_nfts::Config>::StringLimit::get() as usize]
			.try_into()
			.unwrap(),
	));
	assert_ok!(NftMarketplace::<T>::buy_token(RawOrigin::Signed(caller.clone()).into(), 0, 100));
	location
}  

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn add_letting_agent() {
		assert_ok!(NftMarketplace::<T>::create_new_region(RawOrigin::Root.into()));
		let location: BoundedVec<u8, <T as pallet_nft_marketplace::Config>::PostcodeLimit> = vec![0; <T as pallet_nft_marketplace::Config>::PostcodeLimit::get() as usize]
			.try_into()
			.unwrap();
		assert_ok!(NftMarketplace::<T>::create_new_location(RawOrigin::Root.into(), 0, location.clone()));
		let letting_agent: T::AccountId = whitelisted_caller();
/* 		<T as pallet_nfts::Config>::Currency::make_free_balance_be(
			&letting_agent,
			DepositBalanceOf::<T>::max_value(),
		); */
		#[extrinsic_call]
		add_letting_agent(RawOrigin::Root, 0, location, letting_agent.clone());

		assert_eq!(PropertyManagement::<T>::letting_info(letting_agent).is_some(), true);
	} 
 
 	#[benchmark]
	fn letting_agent_deposit() {
		assert_ok!(NftMarketplace::<T>::create_new_region(RawOrigin::Root.into()));
		let location: BoundedVec<u8, <T as pallet_nft_marketplace::Config>::PostcodeLimit> = vec![0; <T as pallet_nft_marketplace::Config>::PostcodeLimit::get() as usize]
			.try_into()
			.unwrap();
		assert_ok!(NftMarketplace::<T>::create_new_location(RawOrigin::Root.into(), 0, location.clone()));
		let letting_agent: T::AccountId = whitelisted_caller();
		<T as pallet_nfts::Config>::Currency::make_free_balance_be(
			&letting_agent,
			DepositBalanceOf::<T>::max_value(),
		);
		assert_ok!(PropertyManagement::<T>::add_letting_agent(RawOrigin::Root.into(), 0, location.clone(), letting_agent.clone()));
		#[extrinsic_call]
		letting_agent_deposit(RawOrigin::Signed(letting_agent.clone()));

		assert_eq!(PropertyManagement::<T>::letting_info(letting_agent).unwrap().deposited, true);
		assert_eq!(PropertyManagement::<T>::letting_agent_locations(0, location).len(), 1);
	}

 	#[benchmark]
	fn add_letting_agent_to_location() {
		assert_ok!(NftMarketplace::<T>::create_new_region(RawOrigin::Root.into()));
		let location1: BoundedVec<u8, <T as pallet_nft_marketplace::Config>::PostcodeLimit> = vec![0; <T as pallet_nft_marketplace::Config>::PostcodeLimit::get() as usize]
			.try_into()
			.unwrap();
		let location2: BoundedVec<u8, <T as pallet_nft_marketplace::Config>::PostcodeLimit> = vec![1; <T as pallet_nft_marketplace::Config>::PostcodeLimit::get() as usize]
			.try_into()
			.unwrap();
		assert_ok!(NftMarketplace::<T>::create_new_location(RawOrigin::Root.into(), 0, location1.clone()));
		assert_ok!(NftMarketplace::<T>::create_new_location(RawOrigin::Root.into(), 0, location2.clone()));
		let letting_agent: T::AccountId = whitelisted_caller();
		<T as pallet_nfts::Config>::Currency::make_free_balance_be(
			&letting_agent,
			DepositBalanceOf::<T>::max_value(),
		);
		assert_ok!(PropertyManagement::<T>::add_letting_agent(RawOrigin::Root.into(), 0, location1.clone(), letting_agent.clone()));
		assert_ok!(PropertyManagement::<T>::letting_agent_deposit(RawOrigin::Signed(letting_agent.clone()).into()));
		#[extrinsic_call]
		add_letting_agent_to_location(RawOrigin::Root, location2.clone(), letting_agent.clone());

 		assert_eq!(PropertyManagement::<T>::letting_agent_locations(0, location1).len(), 1);
		assert_eq!(PropertyManagement::<T>::letting_agent_locations(0, location2).len(), 1); 
	} 

	#[benchmark]
	fn set_letting_agent() {
		let loation = setup_real_estate_object::<T>();
		let letting_agent: T::AccountId = whitelisted_caller();
		<T as pallet_nfts::Config>::Currency::make_free_balance_be(
			&letting_agent,
			DepositBalanceOf::<T>::max_value(),
		);
		assert_ok!(PropertyManagement::<T>::add_letting_agent(RawOrigin::Root.into(), 0, loation, letting_agent.clone()));
		assert_ok!(PropertyManagement::<T>::letting_agent_deposit(RawOrigin::Signed(letting_agent.clone()).into()));
		#[extrinsic_call]
		set_letting_agent(RawOrigin::Signed(letting_agent.clone()), 0);

		assert_eq!(PropertyManagement::<T>::letting_storage(0).is_some(), true);
	}

	#[benchmark]
	fn distribute_income() {
		let loation = setup_real_estate_object::<T>();
		let letting_agent: T::AccountId = whitelisted_caller();
		<T as pallet_nfts::Config>::Currency::make_free_balance_be(
			&letting_agent,
			DepositBalanceOf::<T>::max_value(),
		);
		assert_ok!(PropertyManagement::<T>::add_letting_agent(RawOrigin::Root.into(), 0, loation, letting_agent.clone()));
		assert_ok!(PropertyManagement::<T>::letting_agent_deposit(RawOrigin::Signed(letting_agent.clone()).into()));
		assert_ok!(PropertyManagement::<T>::set_letting_agent(RawOrigin::Signed(letting_agent.clone()).into(), 0));
		let amount: BalanceOf<T> = 100_000u32.into();
		let caller: T::AccountId = whitelisted_caller();
		#[extrinsic_call]
		distribute_income(RawOrigin::Signed(letting_agent), 0, amount);

		assert_eq!(PropertyManagement::<T>::stored_funds(caller), amount);
	}

/* 	#[benchmark]
	fn withdraw_funds() {
		let loation = setup_real_estate_object::<T>();
		let letting_agent: T::AccountId = whitelisted_caller();
		<T as pallet_nfts::Config>::Currency::make_free_balance_be(
			&letting_agent,
			DepositBalanceOf::<T>::max_value(),
		);
		assert_ok!(PropertyManagement::<T>::add_letting_agent(RawOrigin::Root.into(), 0, loation, letting_agent.clone()));
		assert_ok!(PropertyManagement::<T>::letting_agent_deposit(RawOrigin::Signed(letting_agent.clone()).into()));
		assert_ok!(PropertyManagement::<T>::set_letting_agent(RawOrigin::Signed(letting_agent.clone()).into(), 0));
		let amount: BalanceOf<T> = 1u32.into();
		assert_ok!(PropertyManagement::<T>::distribute_income(RawOrigin::Signed(letting_agent).into(), 0, amount));
		let caller: T::AccountId = whitelisted_caller();
		let pallet_account = PropertyManagement::<T>::account_id();
 		<T as pallet_nfts::Config>::Currency::make_free_balance_be(
			&pallet_account,
			DepositBalanceOf::<T>::max_value(),
		); 
		assert_eq!(PropertyManagement::<T>::stored_funds(caller.clone()), amount);
		#[extrinsic_call]
		withdraw_funds(RawOrigin::Signed(caller.clone()));

		assert_eq!(PropertyManagement::<T>::stored_funds(caller), 0u32.into());
	}   */

	impl_benchmark_test_suite!(PropertyManagement, crate::mock::new_test_ext(), crate::mock::Test);
}