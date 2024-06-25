//! Benchmarking setup for pallet-community-loan-pool
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as CommunityLoanPool;
use frame_benchmarking::v2::*;
use frame_support::sp_runtime::Saturating;
use frame_system::RawOrigin;
const SEED: u32 = 0;
use frame_support::assert_ok;
use frame_support::sp_runtime::traits::Bounded;
use pallet_xcavate_whitelist::Pallet as Whitelist;

type DepositBalanceOf<T> =
	<<T as pallet::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

fn setup_proposal<T: Config>(
	u: u32,
) -> (T::AccountId, BalanceOf<T>, AccountIdLookupOf<T>, u64, u64) {
	let caller = account("caller", u, SEED);
	let value: BalanceOf<T> = T::ProposalBondMinimum::get().saturating_mul(100u32.into());
	<T as pallet::Config>::Currency::make_free_balance_be(
		&caller,
		DepositBalanceOf::<T>::max_value(),
	);
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
	fn propose() {
		let (caller, value, beneficiary_lookup, developer_experience, loan_term) =
			setup_proposal::<T>(SEED);
		assert_ok!(Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), caller.clone()));
		#[extrinsic_call]
		propose(
			RawOrigin::Signed(caller),
			value,
			beneficiary_lookup,
			developer_experience,
			loan_term,
		);

		assert_last_event::<T>(Event::Proposed { proposal_index: 1 }.into());
		assert_eq!(CommunityLoanPool::<T>::proposals(1).is_some(), true);
	}

 	#[benchmark]
	fn add_committee_member() {
		let alice = account("alice", SEED, SEED);
		#[extrinsic_call]
		add_committee_member(RawOrigin::Root, alice);
		assert_eq!(CommunityLoanPool::<T>::voting_committee()[0], account("alice", SEED, SEED));
	}

	#[benchmark]
	fn set_milestones() {
		let alice: T::AccountId = account("alice", SEED, SEED);
		assert_ok!(Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), alice.clone()));
		assert_ok!(CommunityLoanPool::<T>::add_committee_member(RawOrigin::Root.into(), alice));
		let (caller, value, beneficiary_lookup, developer_experience, loan_term) =
			setup_proposal::<T>(SEED);
		assert_ok!(Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), caller.clone()));
		assert_ok!(CommunityLoanPool::<T>::propose(
			RawOrigin::Signed(caller).into(),
			value,
			beneficiary_lookup,
			developer_experience,
			loan_term,
		));
		let alice = account("alice", SEED, SEED);
		let proposal_id = CommunityLoanPool::<T>::proposal_count();
		let milestones = get_max_milestones::<T>();
		let sum: u64 = milestones.iter().map(|i| i.percentage_to_unlock.deconstruct() as u64).sum();
		assert_eq!(sum, 100);
		#[extrinsic_call]
		set_milestones(RawOrigin::Signed(alice), proposal_id, milestones);

		assert_eq!(CommunityLoanPool::<T>::ongoing_votes(proposal_id).unwrap().yes_votes, 1);
	}

	#[benchmark]
	fn vote_on_proposal() {
		let alice: T::AccountId = account("alice", SEED, SEED);
		assert_ok!(Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), alice.clone()));
		assert_ok!(CommunityLoanPool::<T>::add_committee_member(RawOrigin::Root.into(), alice));
		let bob: T::AccountId = account("bob", SEED, SEED);
		assert_ok!(Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), bob.clone()));
		assert_ok!(CommunityLoanPool::<T>::add_committee_member(RawOrigin::Root.into(), bob));
		let (caller, value, beneficiary_lookup, developer_experience, loan_term) =
			setup_proposal::<T>(SEED);
		assert_ok!(Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), caller.clone()));
		assert_ok!(CommunityLoanPool::<T>::propose(
			RawOrigin::Signed(caller).into(),
			value,
			beneficiary_lookup,
			developer_experience,
			loan_term,
		));
		let alice: T::AccountId = account("alice", SEED, SEED);
		let proposal_id = CommunityLoanPool::<T>::proposal_count();
		let milestones = get_max_milestones::<T>();
		assert_ok!(CommunityLoanPool::<T>::set_milestones(
			RawOrigin::Signed(alice).into(),
			proposal_id,
			milestones,
		));
		let bob: T::AccountId = account("bob", SEED, SEED);
		//assert_ok!(Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), bob.clone()));
		#[extrinsic_call]
		vote_on_proposal(RawOrigin::Signed(bob), proposal_id, crate::Vote::Yes);

		assert_eq!(CommunityLoanPool::<T>::ongoing_votes(proposal_id).unwrap().yes_votes, 2);
	}

	#[benchmark]
	fn withdraw() {
		let alice = account("alice", SEED, SEED);
		assert_ok!(CommunityLoanPool::<T>::add_committee_member(RawOrigin::Root.into(), alice));
		let bob = account("bob", SEED, SEED);
		assert_ok!(CommunityLoanPool::<T>::add_committee_member(RawOrigin::Root.into(), bob));
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
		let bob: T::AccountId = account("bob", SEED, SEED);
		assert_ok!(Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), bob.clone()));
		assert_ok!(CommunityLoanPool::<T>::vote_on_proposal(
			RawOrigin::Signed(bob).into(),
			proposal_id,
			crate::Vote::Yes,
		));
		run_to_block::<T>(30u32.into());
		//assert_eq!(CommunityLoanPool::<T>::ongoing_votes(proposal_id).unwrap().yes_votes, 2);
		assert_eq!(CommunityLoanPool::<T>::ongoing_loans().len(), 1);
		let withdraw_value: BalanceOf<T> = 100u32.into();
		let beneficiary: T::AccountId = account("beneficiary", SEED, SEED);
		assert_ok!(Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), beneficiary.clone()));
		<T as pallet::Config>::Currency::make_free_balance_be(
			&beneficiary,
			DepositBalanceOf::<T>::max_value() - withdraw_value - withdraw_value,
		);
		let loan_account = CommunityLoanPool::<T>::account_id();
		<T as pallet::Config>::Currency::make_free_balance_be(
			&loan_account,
			DepositBalanceOf::<T>::max_value(),
		);

		#[extrinsic_call]
		withdraw(RawOrigin::Signed(beneficiary), 1, withdraw_value);
		assert_eq!(CommunityLoanPool::<T>::loans(1).unwrap().borrowed_amount, withdraw_value);
	}
/*
	#[benchmark]
	fn repay() {
		let alice = account("alice", SEED, SEED);
		assert_ok!(CommunityLoanPool::<T>::add_committee_member(RawOrigin::Root.into(), alice));
		let bob = account("bob", SEED, SEED);
		assert_ok!(CommunityLoanPool::<T>::add_committee_member(RawOrigin::Root.into(), bob));
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
		let bob: T::AccountId = account("bob", SEED, SEED);
		assert_ok!(Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), bob.clone()));
		assert_ok!(CommunityLoanPool::<T>::vote_on_proposal(
			RawOrigin::Signed(bob).into(),
			proposal_id,
			crate::Vote::Yes,
		));
		run_to_block::<T>(30u32.into());
		//assert_eq!(CommunityLoanPool::<T>::ongoing_votes(proposal_id).unwrap().yes_votes, 2);
		assert_eq!(CommunityLoanPool::<T>::ongoing_loans().len(), 1);
		let withdraw_value: BalanceOf<T> = 100u32.into();
		let beneficiary: T::AccountId = account("beneficiary", SEED, SEED);
		assert_ok!(Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), beneficiary.clone()));
		<T as pallet::Config>::Currency::make_free_balance_be(
			&beneficiary,
			DepositBalanceOf::<T>::max_value() - withdraw_value,
		);
		let loan_account = CommunityLoanPool::<T>::account_id();
		<T as pallet::Config>::Currency::make_free_balance_be(
			&loan_account,
			DepositBalanceOf::<T>::max_value() - withdraw_value - withdraw_value,
		);
		assert_ok!(CommunityLoanPool::<T>::withdraw(
			RawOrigin::Signed(beneficiary.clone()).into(),
			1,
			withdraw_value,
		));
		<T as pallet::Config>::Currency::make_free_balance_be(
			&loan_account,
			<T as pallet::Config>::Currency::minimum_balance(),
		);
		#[extrinsic_call]
		repay(RawOrigin::Signed(beneficiary), 1, withdraw_value);
		assert_eq!(CommunityLoanPool::<T>::loans(1).unwrap().borrowed_amount, 0_u32.into());
	}

	#[benchmark]
	fn propose_milestone() {
		let alice: T::AccountId = account("alice", SEED, SEED);
		assert_ok!(Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), alice.clone()));
		assert_ok!(CommunityLoanPool::<T>::add_committee_member(RawOrigin::Root.into(), alice));
		let bob: T::AccountId = account("bob", SEED, SEED);
		Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), bob.clone());
		assert_ok!(CommunityLoanPool::<T>::add_committee_member(RawOrigin::Root.into(), bob));
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
		let alice = account("alice", SEED, SEED);
		let proposal_id = CommunityLoanPool::<T>::proposal_count();
		let milestones = get_max_milestones::<T>();
		assert_ok!(CommunityLoanPool::<T>::set_milestones(
			RawOrigin::Signed(alice).into(),
			proposal_id,
			milestones,
		));
		let bob = account("bob", SEED, SEED);
		assert_ok!(CommunityLoanPool::<T>::vote_on_proposal(
			RawOrigin::Signed(bob).into(),
			proposal_id,
			crate::Vote::Yes,
		));
		run_to_block::<T>(30u32.into());
		#[extrinsic_call]
		propose_milestone(RawOrigin::Signed(caller), 1);
		assert_last_event::<T>(Event::MilestoneProposed { proposal_index: 1 }.into());
	}

	#[benchmark]
	fn vote_on_milestone_proposal() {
		let alice: T::AccountId = account("alice", SEED, SEED);
		Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), alice.clone());
		assert_ok!(CommunityLoanPool::<T>::add_committee_member(RawOrigin::Root.into(), alice));
		let bob: T::AccountId = account("bob", SEED, SEED);
		Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), bob.clone());
		assert_ok!(CommunityLoanPool::<T>::add_committee_member(RawOrigin::Root.into(), bob.clone()));
		let (caller, value, beneficiary_lookup, developer_experience, loan_term) =
			setup_proposal::<T>(SEED);
		Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), caller.clone());
		assert_ok!(CommunityLoanPool::<T>::propose(
			RawOrigin::Signed(caller.clone()).into(),
			value,
			beneficiary_lookup,
			developer_experience,
			loan_term,
		));
		let alice: T::AccountId = account("alice", SEED, SEED);
		let proposal_id = CommunityLoanPool::<T>::proposal_count();
		let milestones = get_max_milestones::<T>();
		assert_ok!(CommunityLoanPool::<T>::set_milestones(
			RawOrigin::Signed(alice).into(),
			proposal_id,
			milestones,
		));
		assert_ok!(CommunityLoanPool::<T>::vote_on_proposal(
			RawOrigin::Signed(bob.clone()).into(),
			proposal_id,
			crate::Vote::Yes,
		));
		run_to_block::<T>(30u32.into());
		assert_ok!(CommunityLoanPool::<T>::propose_milestone(RawOrigin::Signed(caller).into(), 1));
		#[extrinsic_call]
		vote_on_milestone_proposal(RawOrigin::Signed(bob), proposal_id, crate::Vote::Yes);

		assert_eq!(
			CommunityLoanPool::<T>::ongoing_milestone_votes(proposal_id).unwrap().yes_votes,
			1
		);
	}

	#[benchmark]
	fn propose_deletion() {
		let alice: T::AccountId = account("alice", SEED, SEED);
		Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), alice.clone());
		assert_ok!(CommunityLoanPool::<T>::add_committee_member(RawOrigin::Root.into(), alice));
		let bob: T::AccountId = account("bob", SEED, SEED);
		assert_ok!(CommunityLoanPool::<T>::add_committee_member(RawOrigin::Root.into(), bob.clone()));
		let (caller, value, beneficiary_lookup, developer_experience, loan_term) =
			setup_proposal::<T>(SEED);
		Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), caller.clone());
		assert_ok!(CommunityLoanPool::<T>::propose(
			RawOrigin::Signed(caller.clone()).into(),
			value,
			beneficiary_lookup,
			developer_experience,
			loan_term,
		));
		let alice: T::AccountId = account("alice", SEED, SEED);
		let proposal_id = CommunityLoanPool::<T>::proposal_count();
		let milestones = get_max_milestones::<T>();
		assert_ok!(CommunityLoanPool::<T>::set_milestones(
			RawOrigin::Signed(alice).into(),
			proposal_id,
			milestones,
		));
		assert_ok!(Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), bob.clone()));
		assert_ok!(CommunityLoanPool::<T>::vote_on_proposal(
			RawOrigin::Signed(bob.clone()).into(),
			proposal_id,
			crate::Vote::Yes,
		)); 
		run_to_block::<T>(30u32.into());
		let beneficiary: T::AccountId = account("beneficiary", SEED, SEED);
		Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), beneficiary.clone());
		#[extrinsic_call]
		propose_deletion(RawOrigin::Signed(beneficiary), 1);
		assert_last_event::<T>(Event::DeletionProposed { proposal_index: 1, loan_index: 1 }.into());
	}

	#[benchmark]
	fn vote_on_deletion_proposal() {
		let alice: T::AccountId = account("alice", SEED, SEED);
		Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), alice.clone());
		assert_ok!(CommunityLoanPool::<T>::add_committee_member(RawOrigin::Root.into(), alice));
		let bob: T::AccountId = account("bob", SEED, SEED);
		Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), bob.clone());
		assert_ok!(CommunityLoanPool::<T>::add_committee_member(RawOrigin::Root.into(), bob.clone()));
		let (caller, value, beneficiary_lookup, developer_experience, loan_term) =
			setup_proposal::<T>(SEED);
		Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), caller.clone());
		assert_ok!(CommunityLoanPool::<T>::propose(
			RawOrigin::Signed(caller.clone()).into(),
			value,
			beneficiary_lookup,
			developer_experience,
			loan_term,
		));
		let alice = account("alice", SEED, SEED);
		let proposal_id = CommunityLoanPool::<T>::proposal_count();
		let milestones = get_max_milestones::<T>();
		assert_ok!(CommunityLoanPool::<T>::set_milestones(
			RawOrigin::Signed(alice).into(),
			proposal_id,
			milestones,
		));
		assert_ok!(CommunityLoanPool::<T>::vote_on_proposal(
			RawOrigin::Signed(bob.clone()).into(),
			proposal_id,
			crate::Vote::Yes,
		));
		run_to_block::<T>(30u32.into());
		let beneficiary: T::AccountId = account("beneficiary", SEED, SEED);
		assert_ok!(Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), beneficiary.clone()));
		assert_ok!(CommunityLoanPool::<T>::propose_deletion(RawOrigin::Signed(beneficiary).into(), 1));
		#[extrinsic_call]
		vote_on_deletion_proposal(RawOrigin::Signed(bob), 1, crate::Vote::Yes);

		assert_eq!(
			CommunityLoanPool::<T>::ongoing_deletion_votes(proposal_id).unwrap().yes_votes,
			1
		);
	} */

	impl_benchmark_test_suite!(CommunityLoanPool, crate::mock::new_test_ext(), crate::mock::Test);
}

fn get_max_milestones<T: Config>() -> BoundedProposedMilestones<T> {
	let max_milestones: u32 = <T as Config>::MaxMilestonesPerProject::get();
	get_milestones::<T>(max_milestones)
}

fn get_milestones<T: Config>(mut n: u32) -> BoundedProposedMilestones<T> {
	let max = <T as Config>::MaxMilestonesPerProject::get();
	if n > max {
		n = max;
	}

	(0..n)
		.map(|_| ProposedMilestone { percentage_to_unlock: Percent::from_percent((100 / n) as u8) })
		.collect::<Vec<ProposedMilestone>>()
		.try_into()
		.expect("qed")
}

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

fn run_to_block<T: Config>(new_block: frame_system::pallet_prelude::BlockNumberFor<T>) {
	while frame_system::Pallet::<T>::block_number() < new_block {
		if frame_system::Pallet::<T>::block_number() > 0u32.into() {
			CommunityLoanPool::<T>::on_initialize(frame_system::Pallet::<T>::block_number());
			frame_system::Pallet::<T>::on_finalize(frame_system::Pallet::<T>::block_number());
		}
		frame_system::Pallet::<T>::reset_events();
		frame_system::Pallet::<T>::set_block_number(
			frame_system::Pallet::<T>::block_number() + 1u32.into(),
		);
		frame_system::Pallet::<T>::on_initialize(frame_system::Pallet::<T>::block_number());
		CommunityLoanPool::<T>::on_initialize(frame_system::Pallet::<T>::block_number());
	}
}
