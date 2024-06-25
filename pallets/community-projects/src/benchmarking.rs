//! Benchmarking setup for pallet-community-projects
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as CommunityProjects;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use sp_std::prelude::*;
const SEED: u32 = 0;
use frame_support::sp_runtime::traits::Bounded;
use frame_support::traits::Get;
type DepositBalanceOf<T> = <<T as pallet_nfts::Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::Balance;
use frame_support::assert_ok;
use frame_support::traits::Hooks;
use pallet_assets::Pallet as Assets;
use pallet_xcavate_whitelist::Pallet as Whitelist;

fn setup_listing<T: Config>(
	u: u32,
) -> (
	T::AccountId,
	BoundedNftDonationTypes<T>,
	BoundedVec<BoundedVec<u8, <T as pallet_nfts::Config>::StringLimit>, <T as Config>::MaxNftTypes>,
	u32,
	BalanceOf<T>,
	BoundedVec<u8, <T as pallet_nfts::Config>::StringLimit>,
) {
	let caller: T::AccountId = account("caller", u, SEED);
	let project_types = get_project_nfts::<T>(4);
	<T as pallet_nfts::Config>::Currency::make_free_balance_be(
		&caller,
		DepositBalanceOf::<T>::max_value(),
	);
	let metadatas = get_nft_metadata::<T>(4);
	let duration = 12;
	let value: BalanceOf<T> = 100u32.into();
	let single_metadata = vec![0; <T as pallet_nfts::Config>::StringLimit::get() as usize]
		.try_into()
		.unwrap();
	(caller, project_types, metadatas, duration, value, single_metadata)
}

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn list_project() {
		let (caller, project_types, metadatas, duration, value, single_metadata) =
			setup_listing::<T>(SEED);
		assert_ok!(Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), caller.clone()));
		#[extrinsic_call]
		list_project(
			RawOrigin::Signed(caller),
			project_types,
			metadatas,
			duration,
			value,
			single_metadata,
		);
		assert_eq!(
			CommunityProjects::<T>::listed_nft_types::<<T as pallet::Config>::CollectionId, u8>(
				0.into(),
				4
			)
			.unwrap()
			.len(),
			4
		);
	}

	#[benchmark]
	fn buy_nft() {
		let (caller, _, metadatas, duration, value, single_metadata) =
			setup_listing::<T>(SEED);
		assert_ok!(Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), caller.clone()));
		let many_project_types = get_project_nfts_many::<T>(4);
		assert_ok!(CommunityProjects::<T>::list_project(
			RawOrigin::Signed(caller).into(),
			many_project_types,
			metadatas,
			duration,
			value,
			single_metadata,
		));
		let buyer: T::AccountId = account("buyer", SEED, SEED);
		assert_ok!(Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), buyer.clone()));
		<T as pallet_nfts::Config>::Currency::make_free_balance_be(
			&buyer,
			DepositBalanceOf::<T>::max_value(),
		);
		let amount: BalanceOf<T> = 1u32.into();
		//let origin = ensure_signed(buyer.clone());
		let user_lookup = <T::Lookup as StaticLookup>::unlookup(buyer.clone());
		let asset_id = <T as pallet::Config>::Helper::to_asset(1);
		let amount2: BalanceOf<T> = 4294967295u32.into();
		assert_ok!(Assets::<T, Instance1>::create(
			RawOrigin::Signed(buyer.clone()).into(),
			asset_id.clone().into(),
			user_lookup.clone(),
			amount,
		));
		assert_ok!(Assets::<T, Instance1>::mint(
			RawOrigin::Signed(buyer.clone()).into(),
			asset_id.clone().into(),
			user_lookup,
			amount2,
		));
		assert_eq!(Assets::<T, Instance1>::balance(asset_id, buyer.clone()), amount2);
		#[extrinsic_call]
		buy_nft(RawOrigin::Signed(buyer), 0.into(), 1, 100);

		//assert_eq!(Something::<T>::get(), Some(101u32));
	}

	#[benchmark]
	fn bond_token() {
		let (caller, project_types, metadatas, duration, value, single_metadata) =
			setup_listing::<T>(SEED);
		assert_ok!(Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), caller.clone()));
		assert_ok!(CommunityProjects::<T>::list_project(
			RawOrigin::Signed(caller).into(),
			project_types,
			metadatas,
			duration,
			value,
			single_metadata,
		));
		let user: T::AccountId = account("user", SEED, SEED);
		assert_ok!(Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), user.clone()));
		<T as pallet_nfts::Config>::Currency::make_free_balance_be(
			&user,
			DepositBalanceOf::<T>::max_value(),
		);
		let amount: BalanceOf2<T> = 10u32.into();
		#[extrinsic_call]
		bond_token(RawOrigin::Signed(user), 0.into(), amount);

		assert_eq!(CommunityProjects::<T>::total_bonded(), amount);
	}

	#[benchmark]
	fn vote_on_milestone() {
		let (caller, project_types, metadatas, duration, value, single_metadata) =
			setup_listing::<T>(SEED);
		assert_ok!(Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), caller.clone()));
		assert_ok!(CommunityProjects::<T>::list_project(
			RawOrigin::Signed(caller).into(),
			project_types,
			metadatas,
			duration,
			value,
			single_metadata,
		));
		let buyer: T::AccountId = account("buyer", SEED, SEED);
		assert_ok!(Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), buyer.clone()));
		<T as pallet_nfts::Config>::Currency::make_free_balance_be(
			&buyer,
			DepositBalanceOf::<T>::max_value(),
		);
		let amount: BalanceOf<T> = 1u32.into();
		//let origin = ensure_signed(buyer.clone());
		let user_lookup = <T::Lookup as StaticLookup>::unlookup(buyer.clone());
		let asset_id = <T as pallet::Config>::Helper::to_asset(1);
		let amount2: BalanceOf<T> = 4294967295u32.into();
		assert_ok!(Assets::<T, Instance1>::create(
			RawOrigin::Signed(buyer.clone()).into(),
			asset_id.clone().into(),
			user_lookup.clone(),
			amount,
		));
		assert_ok!(Assets::<T, Instance1>::mint(
			RawOrigin::Signed(buyer.clone()).into(),
			asset_id.clone().into(),
			user_lookup,
			amount2,
		));
		assert_eq!(Assets::<T, Instance1>::balance(asset_id, buyer.clone()), amount2);
		assert_ok!(CommunityProjects::<T>::buy_nft(RawOrigin::Signed(buyer.clone()).into(), 0.into(), 1, 1));
		run_to_block::<T>(11u32.into());
		#[extrinsic_call]
		vote_on_milestone(RawOrigin::Signed(buyer), 0.into(), crate::Vote::Yes);

		//assert_eq!(Something::<T>::get(), Some(101u32));
	}

	#[benchmark]
	fn claim_refunded_token() {
		let (caller, project_types, metadatas, duration, value, single_metadata) =
			setup_listing::<T>(SEED);
		assert_ok!(Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), caller.clone()));
		assert_ok!(CommunityProjects::<T>::list_project(
			RawOrigin::Signed(caller).into(),
			project_types,
			metadatas,
			duration,
			value,
			single_metadata,
		));
		let buyer: T::AccountId = account("buyer", SEED, SEED);
		assert_ok!(Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), buyer.clone()));
		<T as pallet_nfts::Config>::Currency::make_free_balance_be(
			&buyer,
			DepositBalanceOf::<T>::max_value(),
		);
		let amount: BalanceOf<T> = 1u32.into();
		//let origin = ensure_signed(buyer.clone());
		let user_lookup = <T::Lookup as StaticLookup>::unlookup(buyer.clone());
		let asset_id = <T as pallet::Config>::Helper::to_asset(1);
		let amount2: BalanceOf<T> = 4294967295u32.into();
		assert_ok!(Assets::<T, Instance1>::create(
			RawOrigin::Signed(buyer.clone()).into(),
			asset_id.clone().into(),
			user_lookup.clone(),
			amount,
		));
		assert_ok!(Assets::<T, Instance1>::mint(
			RawOrigin::Signed(buyer.clone()).into(),
			asset_id.clone().into(),
			user_lookup,
			amount2,
		));
		assert_eq!(Assets::<T, Instance1>::balance(asset_id, buyer.clone()), amount2);
		assert_ok!(CommunityProjects::<T>::buy_nft(RawOrigin::Signed(buyer.clone()).into(), 0.into(), 1, 1));
		run_to_block::<T>(100u32.into());
		assert_eq!(
			CommunityProjects::<T>::ended_projects::<<T as pallet::Config>::CollectionId>(
				0_u32.into()
			)
			.unwrap()
			.project_success,
			false
		);
		#[extrinsic_call]
		claim_refunded_token(RawOrigin::Signed(buyer), 0.into());

		assert_eq!(
			CommunityProjects::<T>::ended_projects::<<T as pallet::Config>::CollectionId>(
				0_u32.into()
			),
			None
		);
	}

	#[benchmark]
	fn claim_bonding() {
		let (caller, project_types, metadatas, duration, value, single_metadata) =
			setup_listing::<T>(SEED);
		assert_ok!(Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), caller.clone()));
		assert_ok!(CommunityProjects::<T>::list_project(
			RawOrigin::Signed(caller).into(),
			project_types,
			metadatas,
			duration,
			value,
			single_metadata,
		));
		let buyer: T::AccountId = account("buyer", SEED, SEED);
		assert_ok!(Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), buyer.clone()));
		<T as pallet_nfts::Config>::Currency::make_free_balance_be(
			&buyer,
			DepositBalanceOf::<T>::max_value(),
		);
		let amount: BalanceOf<T> = 1u32.into();
		//let origin = ensure_signed(buyer.clone());
		let user_lookup = <T::Lookup as StaticLookup>::unlookup(buyer.clone());
		let asset_id = <T as pallet::Config>::Helper::to_asset(1);
		let amount2: BalanceOf<T> = 4294967295u32.into();
		assert_ok!(Assets::<T, Instance1>::create(
			RawOrigin::Signed(buyer.clone()).into(),
			asset_id.clone().into(),
			user_lookup.clone(),
			amount,
		));
		assert_ok!(Assets::<T, Instance1>::mint(
			RawOrigin::Signed(buyer.clone()).into(),
			asset_id.clone().into(),
			user_lookup,
			amount2,
		));
		assert_eq!(Assets::<T, Instance1>::balance(asset_id, buyer.clone()), amount2);
		let user: T::AccountId = account("user", SEED, SEED);
		assert_ok!(Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), user.clone()));
		<T as pallet_nfts::Config>::Currency::make_free_balance_be(
			&user,
			DepositBalanceOf::<T>::max_value(),
		);
		let amount: BalanceOf2<T> = 10u32.into();
		assert_ok!(CommunityProjects::<T>::bond_token(
			RawOrigin::Signed(user.clone()).into(),
			0.into(),
			amount,
		));
		assert_ok!(CommunityProjects::<T>::buy_nft(RawOrigin::Signed(buyer.clone()).into(), 0.into(), 1, 1));
		run_to_block::<T>(100u32.into());
		assert_eq!(
			CommunityProjects::<T>::ended_projects::<<T as pallet::Config>::CollectionId>(
				0_u32.into()
			)
			.unwrap()
			.project_success,
			false
		);
		run_to_block::<T>(100u32.into());
		assert_eq!(
			CommunityProjects::<T>::ended_projects::<<T as pallet::Config>::CollectionId>(
				0_u32.into()
			)
			.unwrap()
			.project_success,
			false
		);
		assert_eq!(
			CommunityProjects::<T>::project_bonding::<
				<T as pallet::Config>::CollectionId,
				AccountIdOf<T>,
			>(0_u32.into(), user.clone())
			.unwrap(),
			10_u32.try_into().map_err(|_| Error::<T>::ConversionError).unwrap()
		);
		assert_eq!(
			CommunityProjects::<T>::ended_projects::<<T as pallet::Config>::CollectionId>(
				0_u32.into()
			)
			.unwrap()
			.bonding_balance,
			10_u32.try_into().map_err(|_| Error::<T>::ConversionError).unwrap()
		);
		#[extrinsic_call]
		claim_bonding(RawOrigin::Signed(user.clone()), 0.into());

		assert_eq!(
			CommunityProjects::<T>::ended_projects::<<T as pallet::Config>::CollectionId>(
				0_u32.into()
			)
			.unwrap()
			.bonding_balance,
			0_u32.try_into().map_err(|_| Error::<T>::ConversionError).unwrap()
		);
	}

	impl_benchmark_test_suite!(CommunityProjects, crate::mock::new_test_ext(), crate::mock::Test);
}

fn get_project_nfts<T: Config>(mut n: u32) -> BoundedNftDonationTypes<T> {
	let max = <T as Config>::MaxNftTypes::get();
	if n > max {
		n = max
	}
	(1..=n)
		.map(|x| NftDonationTypes::<BalanceOf<T>> { price: (100 * x).into(), amount: x })
		.collect::<Vec<NftDonationTypes<BalanceOf<T>>>>()
		.try_into()
		.expect("bound is ensured; qed")
}

fn get_project_nfts_many<T: Config>(mut n: u32) -> BoundedNftDonationTypes<T> {
	let max = <T as Config>::MaxNftTypes::get();
	if n > max {
		n = max
	}
	(1..=n)
		.map(|x| NftDonationTypes::<BalanceOf<T>> { price: (100 * x).into(), amount: 100 })
		.collect::<Vec<NftDonationTypes<BalanceOf<T>>>>()
		.try_into()
		.expect("bound is ensured; qed")
}

fn get_nft_metadata<T: Config>(
	mut n: u32,
) -> BoundedVec<BoundedVec<u8, <T as pallet_nfts::Config>::StringLimit>, <T as Config>::MaxNftTypes>
{
	let max = <T as Config>::MaxNftTypes::get();
	if n > max {
		n = max
	}
	(1..=n)
		.map(|_| {
			vec![0; <T as pallet_nfts::Config>::StringLimit::get() as usize]
				.try_into()
				.unwrap()
		})
		.collect::<Vec<BoundedVec<u8, <T as pallet_nfts::Config>::StringLimit>>>()
		.try_into()
		.expect("bound is ensured; qed")
}

fn run_to_block<T: Config>(new_block: frame_system::pallet_prelude::BlockNumberFor<T>) {
	while frame_system::Pallet::<T>::block_number() < new_block {
		if frame_system::Pallet::<T>::block_number() > 0u32.into() {
			CommunityProjects::<T>::on_initialize(frame_system::Pallet::<T>::block_number());
			frame_system::Pallet::<T>::on_finalize(frame_system::Pallet::<T>::block_number());
		}
		frame_system::Pallet::<T>::reset_events();
		frame_system::Pallet::<T>::set_block_number(
			frame_system::Pallet::<T>::block_number() + 1u32.into(),
		);
		frame_system::Pallet::<T>::on_initialize(frame_system::Pallet::<T>::block_number());
		CommunityProjects::<T>::on_initialize(frame_system::Pallet::<T>::block_number());
	}
}
