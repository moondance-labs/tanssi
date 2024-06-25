//! Benchmarking setup for pallet-property-governance
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as PropertyGovernance;
use frame_benchmarking::__private::vec;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use frame_support::sp_runtime::traits::Bounded;
use pallet_nft_marketplace::Pallet as NftMarketplace;
type DepositBalanceOf<T> = <<T as pallet_nfts::Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::Balance;
use pallet_xcavate_whitelist::Pallet as Whitelist;
use pallet_property_management::Pallet as PropertyManagement;
use frame_support::{traits::Get, assert_ok};
use frame_support::BoundedVec;
use frame_support::sp_runtime::traits::StaticLookup;
use pallet_assets::Pallet as Assets;

type BalanceOf1<T> = <<T as pallet_nft_marketplace::Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::Balance;
type BalanceOf2<T> = <T as pallet_assets::Config<pallet_assets::Instance1>>::Balance;

fn setup_real_estate_object<T: Config>() {
	let value: BalanceOf2<T> = 1u32.into();
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
		value.into(),
		100,
		vec![0; <T as pallet_nfts::Config>::StringLimit::get() as usize]
			.try_into()
			.unwrap(),
	));
	assert_ok!(NftMarketplace::<T>::buy_token(RawOrigin::Signed(caller.clone()).into(), 0, 100));
	let letting_agent: T::AccountId = whitelisted_caller();
	<T as pallet_nfts::Config>::Currency::make_free_balance_be(
		&letting_agent,
		DepositBalanceOf::<T>::max_value(),
	);
	assert_ok!(PropertyManagement::<T>::add_letting_agent(RawOrigin::Root.into(), 0, location, letting_agent.clone()));
	assert_ok!(PropertyManagement::<T>::letting_agent_deposit(RawOrigin::Signed(letting_agent.clone()).into()));
	assert_ok!(PropertyManagement::<T>::set_letting_agent(RawOrigin::Signed(letting_agent.clone()).into(), 0));	
}

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn propose() {
		setup_real_estate_object::<T>();
		let caller: T::AccountId = whitelisted_caller();
		<T as pallet_nfts::Config>::Currency::make_free_balance_be(
			&caller,
			DepositBalanceOf::<T>::max_value(),
		);
		#[extrinsic_call]
		propose(RawOrigin::Signed(caller.clone()), 0);

		assert_eq!(PropertyGovernance::<T>::proposals(1).is_some(), true);
	}

 	#[benchmark]
	fn inquery_against_letting_agent() {
		setup_real_estate_object::<T>();
		let caller: T::AccountId = whitelisted_caller();
		<T as pallet_nfts::Config>::Currency::make_free_balance_be(
			&caller,
			DepositBalanceOf::<T>::max_value(),
		);
		#[extrinsic_call]
		inquery_against_letting_agent(RawOrigin::Signed(caller.clone()), 0);

		assert_eq!(PropertyGovernance::<T>::inqueries(1).is_some(), true);
	}

	#[benchmark]
	fn vote_on_proposal() {
		setup_real_estate_object::<T>();
		let caller: T::AccountId = whitelisted_caller();
		<T as pallet_nfts::Config>::Currency::make_free_balance_be(
			&caller,
			DepositBalanceOf::<T>::max_value(),
		);
		assert_ok!{PropertyGovernance::<T>::propose(RawOrigin::Signed(caller.clone()).into(), 0)};
		#[extrinsic_call]
		vote_on_proposal(RawOrigin::Signed(caller.clone()), 1, crate::Vote::Yes);

		assert_eq!(PropertyGovernance::<T>::ongoing_votes(1).unwrap().yes_votes, 100);
		assert_eq!(PropertyGovernance::<T>::proposal_voter(1).len(), 1);
	}

	#[benchmark]
	fn vote_on_letting_agent_inquery() {
		setup_real_estate_object::<T>();
		let caller: T::AccountId = whitelisted_caller();
		<T as pallet_nfts::Config>::Currency::make_free_balance_be(
			&caller,
			DepositBalanceOf::<T>::max_value(),
		);
		assert_ok!{PropertyGovernance::<T>::inquery_against_letting_agent(RawOrigin::Signed(caller.clone()).into(), 0)};
		#[extrinsic_call]
		vote_on_letting_agent_inquery(RawOrigin::Signed(caller.clone()), 1, crate::Vote::Yes);

		assert_eq!(PropertyGovernance::<T>::ongoing_inquery_votes(1).unwrap().yes_votes, 100);
		assert_eq!(PropertyGovernance::<T>::inquery_voter(1).len(), 1);
	}


	impl_benchmark_test_suite!(PropertyGovernance, crate::mock::new_test_ext(), crate::mock::Test);
}
