//! Benchmarking setup for pallet-xcavate-staking

use super::*;

#[allow(unused)]
use crate::Pallet as XcavateStaking;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
const SEED: u32 = 0;
use frame_support::sp_runtime::traits::Bounded;
use pallet_community_loan_pool::Pallet as CommunityLoanPool;
use pallet_community_loan_pool::{BoundedProposedMilestones, ProposedMilestone};
use pallet_xcavate_whitelist::Pallet as Whitelist;

use frame_support::sp_runtime::traits::StaticLookup;

use frame_system::pallet_prelude::BlockNumberFor;

type AccountIdLookupOf<T> = <<T as frame_system::Config>::Lookup as StaticLookup>::Source;

pub type BalanceOf1<T> = <<T as pallet_community_loan_pool::Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::Balance;

type DepositBalanceOf<T> =
	<<T as pallet::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

use frame_support::sp_runtime::Percent;

use frame_support::assert_ok;

fn setup_proposal<T: Config>(
	u: u32,
) -> (T::AccountId, BalanceOf1<T>, AccountIdLookupOf<T>, u64, u64) {
	let caller = account("caller", u, SEED);
	let value = <T as pallet_community_loan_pool::Config>::ProposalBondMinimum::get()
		.saturating_mul(100u32.into());
	let _ =
		<T as pallet_community_loan_pool::Config>::Currency::make_free_balance_be(&caller, value);
	let beneficiary = account("beneficiary", u, SEED);
	let beneficiary_lookup = T::Lookup::unlookup(beneficiary);
	let developer_experience = 13;
	let loan_term = 20;
	(caller, value, beneficiary_lookup, developer_experience, loan_term)
}

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn stake() {
		/* 		let alice: T::AccountId = account("alice", SEED, SEED);
		CommunityLoanPool::<T>::add_committee_member(RawOrigin::Root.into(), alice);
		 let bob: T::AccountId = account("bob", SEED, SEED);
		CommunityLoanPool::<T>::add_committee_member(RawOrigin::Root.into(), bob.clone());
		assert_ok!(Whitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		let (caller, value, beneficiary_lookup, developer_experience, loan_term) =
			setup_proposal::<T>(SEED);
		CommunityLoanPool::<T>::propose(
			RawOrigin::Signed(caller.clone()).into(),
			value,
			beneficiary_lookup,
			developer_experience,
			loan_term,
		);
		let alice = account("alice", SEED, SEED);
		let proposal_id = CommunityLoanPool::<T>::proposal_count();
		let milestones = get_max_milestones::<T>();
		CommunityLoanPool::<T>::set_milestones(
			RawOrigin::Signed(alice).into(),
			proposal_id,
			milestones,
		);
		run_to_block::<T>(30u32.into()); */
		let caller: T::AccountId = account("Alice", SEED, SEED);
		let value: BalanceOf<T> = 100u32.into();
		<T as pallet::Config>::Currency::make_free_balance_be(
			&caller,
			DepositBalanceOf::<T>::max_value(),
		);
		assert_ok!(Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), caller.clone()));

		#[extrinsic_call]
		stake(RawOrigin::Signed(caller.clone()), value);
		let index = XcavateStaking::<T>::queue_count();
		assert_last_event::<T>(Event::Locked { staker: caller.clone(), amount: value }.into());
		assert_eq!(XcavateStaking::<T>::queue_staking().len(), 1);
		assert_eq!(XcavateStaking::<T>::queue_ledger(index).is_none(), false);
	}

	#[benchmark]
	fn unstake() {
		let alice: T::AccountId = account("alice", SEED, SEED);
		assert_ok!(CommunityLoanPool::<T>::add_committee_member(RawOrigin::Root.into(), alice));
		let bob: T::AccountId = account("bob", SEED, SEED);
		assert_ok!(CommunityLoanPool::<T>::add_committee_member(RawOrigin::Root.into(), bob.clone()));
		let (caller, value, beneficiary_lookup, developer_experience, loan_term) =
			setup_proposal::<T>(SEED);
		assert_ok!(Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), caller.clone()));
		assert_ok!(CommunityLoanPool::<T>::propose(
			RawOrigin::Signed(caller.clone()).into(),
			value,
			beneficiary_lookup,
			developer_experience,
			loan_term,
		));
		let alice: T::AccountId = account("alice", SEED, SEED);
		assert_ok!(Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), alice.clone()));
		let proposal_id = CommunityLoanPool::<T>::proposal_count();
		let milestones = get_max_milestones::<T>();
		assert_ok!(CommunityLoanPool::<T>::set_milestones(
			RawOrigin::Signed(alice).into(),
			proposal_id,
			milestones,
		));
		run_to_block::<T>(30u32.into());
		let caller: T::AccountId = account("alice", SEED, SEED);
		let value: BalanceOf<T> = 1_000u32.into();
		let _ = <T as pallet::Config>::Currency::make_free_balance_be(
			&caller,
			DepositBalanceOf::<T>::max_value(),
		);
		assert_ok!(XcavateStaking::<T>::stake(RawOrigin::Signed(caller.clone()).into(), value));
		assert_eq!(XcavateStaking::<T>::active_stakings().len(), 1);
		let unstake_value: BalanceOf<T> = 1u32.into();
		let index = XcavateStaking::<T>::active_stakings()[0];
		#[extrinsic_call]
		unstake(RawOrigin::Signed(caller.clone()), index, unstake_value);

		assert_last_event::<T>(Event::Unlocked { staker: caller, amount: unstake_value }.into());
		let staked_value: BalanceOf<T> = 999u32.into();
		assert_eq!(XcavateStaking::<T>::ledger(index).unwrap().locked, staked_value);
	}

	#[benchmark]
	fn withdraw_from_queue() {
		let caller: T::AccountId = account("Alice", SEED, SEED);
		let value: BalanceOf<T> = 1000u32.into();
		let _ = <T as pallet::Config>::Currency::make_free_balance_be(
			&caller,
			DepositBalanceOf::<T>::max_value(),
		);
		assert_ok!(Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), caller.clone()));
		assert_ok!(XcavateStaking::<T>::stake(RawOrigin::Signed(caller.clone()).into(), value));
		assert_eq!(XcavateStaking::<T>::queue_staking().len(), 1);
		let unstake_value: BalanceOf<T> = 100u32.into();
		let x = XcavateStaking::<T>::queue_staking()[0];
		assert_eq!(XcavateStaking::<T>::queue_ledger(x).unwrap().locked, value);
		#[extrinsic_call]
		withdraw_from_queue(RawOrigin::Signed(caller.clone()), 1, unstake_value);
		assert_eq!(XcavateStaking::<T>::queue_ledger(x).unwrap().locked, 900_u32.into());
		assert_eq!(XcavateStaking::<T>::queue_staking().len(), 1);
	}

	impl_benchmark_test_suite!(XcavateStaking, crate::mock::new_test_ext(), crate::mock::Test);
}

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

fn run_to_block<T: Config>(new_block: frame_system::pallet_prelude::BlockNumberFor<T>) {
	while frame_system::Pallet::<T>::block_number() < new_block {
		if frame_system::Pallet::<T>::block_number() > 0u32.into() {
			<pallet_community_loan_pool::Pallet<T> as frame_support::traits::Hooks<
				BlockNumberFor<T>,
			>>::on_initialize(frame_system::Pallet::<T>::block_number());
			frame_system::Pallet::<T>::on_finalize(frame_system::Pallet::<T>::block_number());
		}
		frame_system::Pallet::<T>::reset_events();
		frame_system::Pallet::<T>::set_block_number(
			frame_system::Pallet::<T>::block_number() + 1u32.into(),
		);
		<frame_system::Pallet<T> as frame_support::traits::Hooks<BlockNumberFor<T>>>::on_initialize(
			frame_system::Pallet::<T>::block_number(),
		);
		<pallet_community_loan_pool::Pallet<T> as frame_support::traits::Hooks<
			BlockNumberFor<T>,
		>>::on_initialize(frame_system::Pallet::<T>::block_number());
	}
}

fn get_max_milestones<T: Config>() -> BoundedProposedMilestones<T> {
	let max_milestones: u32 =
		<T as pallet_community_loan_pool::Config>::MaxMilestonesPerProject::get();
	get_milestones::<T>(max_milestones)
}

fn get_milestones<T: Config>(mut n: u32) -> BoundedProposedMilestones<T> {
	let max = <T as pallet_community_loan_pool::Config>::MaxMilestonesPerProject::get();
	if n > max {
		n = max;
	}

	(0..n)
		.map(|_| ProposedMilestone { percentage_to_unlock: Percent::from_percent((100 / n) as u8) })
		.collect::<Vec<ProposedMilestone>>()
		.try_into()
		.expect("qed")
}
