use crate::{mock::*, Error, Event};
use frame_support::sp_runtime::Percent;
use frame_support::{
	assert_noop, assert_ok,
	traits::{OnFinalize, OnInitialize},
};

use crate::Config;
use crate::{BoundedProposedMilestones, ProposedMilestone};

fn get_milestones(mut n: u32) -> BoundedProposedMilestones<Test> {
	let max = <Test as Config>::MaxMilestonesPerProject::get();
	if n > max {
		n = max
	}
	(0..n)
		.map(|_| ProposedMilestone { percentage_to_unlock: Percent::from_percent((100 / n) as u8) })
		.collect::<Vec<ProposedMilestone>>()
		.try_into()
		.expect("bound is ensured; qed")
}

fn run_to_block(n: u64) {
	while System::block_number() < n {
		if System::block_number() > 0 {
			CommunityLoanPool::on_finalize(System::block_number());
			System::on_finalize(System::block_number());
		}
		System::reset_events();
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		CommunityLoanPool::on_initialize(System::block_number());
	}
}

#[test]
fn propose_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(CommunityLoanPool::propose(
			RuntimeOrigin::signed([0; 32].into()),
			100,
			sp_runtime::MultiAddress::Id([1; 32].into()),
			13,
			20
		));
		System::assert_last_event(Event::Proposed { proposal_index: 1 }.into());
	})
}

#[test]
fn propose_doesnt_work_not_enough_userbalance() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [6; 32].into()));
		assert_noop!(
			CommunityLoanPool::propose(
				RuntimeOrigin::signed([6; 32].into()),
				100,
				sp_runtime::MultiAddress::Id([1; 32].into()),
				13,
				20
			),
			Error::<Test>::InsufficientProposersBalance
		);
	})
}

#[test]
fn propose_doesnt_work_too_much_reserved() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(CommunityLoanPool::propose(
			RuntimeOrigin::signed([0; 32].into()),
			60_000_000,
			sp_runtime::MultiAddress::Id([1; 32].into()),
			13,
			20
		));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_noop!(
			CommunityLoanPool::propose(
				RuntimeOrigin::signed([1; 32].into()),
				60_000_000,
				sp_runtime::MultiAddress::Id([1; 32].into()),
				13,
				20
			),
			Error::<Test>::NotEnoughLoanFundsAvailable
		);
	})
}

#[test]
fn free_reserved_funds_after_rejection() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(CommunityLoanPool::add_committee_member(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(CommunityLoanPool::propose(
			RuntimeOrigin::signed([0; 32].into()),
			60_000_000,
			sp_runtime::MultiAddress::Id([1; 32].into()),
			13,
			20
		));
		run_to_block(22);
		assert_ok!(CommunityLoanPool::propose(
			RuntimeOrigin::signed([0; 32].into()),
			60_000_000,
			sp_runtime::MultiAddress::Id([1; 32].into()),
			13,
			20
		));
	})
}

#[test]
fn add_committee_member_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(CommunityLoanPool::add_committee_member(RuntimeOrigin::root(), [0; 32].into()));
		assert_eq!(CommunityLoanPool::voting_committee()[0], [0; 32].into());
	})
}

#[test]
fn add_committee_member_fails_when_member_is_two_times_added() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(CommunityLoanPool::add_committee_member(RuntimeOrigin::root(), [0; 32].into()));
		assert_eq!(CommunityLoanPool::voting_committee()[0], [0; 32].into());
		assert_noop!(
			CommunityLoanPool::add_committee_member(RuntimeOrigin::root(), [0; 32].into()),
			Error::<Test>::AlreadyMember
		);
	})
}

#[test]
fn voting_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		Timestamp::set_timestamp(1);
		assert_ok!(CommunityLoanPool::add_committee_member(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(CommunityLoanPool::add_committee_member(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(CommunityLoanPool::propose(
			RuntimeOrigin::signed([1; 32].into()),
			100,
			sp_runtime::MultiAddress::Id([1; 32].into()),
			13,
			20
		));
		assert_ok!(CommunityLoanPool::set_milestones(
			RuntimeOrigin::signed([0; 32].into()),
			1,
			get_milestones(10),
		));
		assert_ok!(CommunityLoanPool::vote_on_proposal(
			RuntimeOrigin::signed([1; 32].into()),
			1,
			crate::Vote::Yes
		));
		assert_eq!(CommunityLoanPool::ongoing_votes(1).unwrap().yes_votes, 2);
	})
}

#[test]
fn vote_rejected_with_no_votes() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		Timestamp::set_timestamp(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(CommunityLoanPool::add_committee_member(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(CommunityLoanPool::add_committee_member(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(CommunityLoanPool::propose(
			RuntimeOrigin::signed([1; 32].into()),
			100,
			sp_runtime::MultiAddress::Id([1; 32].into()),
			13,
			20
		));
		assert_ok!(CommunityLoanPool::set_milestones(
			RuntimeOrigin::signed([0; 32].into()),
			1,
			get_milestones(10),
		));
		run_to_block(22);
		assert_noop!(
			CommunityLoanPool::vote_on_proposal(
				RuntimeOrigin::signed([1; 32].into()),
				1,
				crate::Vote::No
			),
			Error::<Test>::InvalidIndex
		);
	})
}

#[test]
fn voting_works_only_for_members() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [2; 32].into()));
		assert_ok!(CommunityLoanPool::add_committee_member(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(CommunityLoanPool::propose(
			RuntimeOrigin::signed([1; 32].into()),
			100,
			sp_runtime::MultiAddress::Id([1; 32].into()),
			13,
			20
		));
		assert_ok!(CommunityLoanPool::set_milestones(
			RuntimeOrigin::signed([0; 32].into()),
			1,
			get_milestones(10),
		));
		assert_noop!(
			CommunityLoanPool::vote_on_proposal(
				RuntimeOrigin::signed([2; 32].into()),
				1,
				crate::Vote::Yes
			),
			Error::<Test>::InsufficientPermission
		);
	})
}

#[test]
fn vote_evaluated_after_yes_votes() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(1);
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(CommunityLoanPool::add_committee_member(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(CommunityLoanPool::propose(
			RuntimeOrigin::signed([1; 32].into()),
			100,
			sp_runtime::MultiAddress::Id([1; 32].into()),
			13,
			20
		));
		assert_ok!(CommunityLoanPool::set_milestones(
			RuntimeOrigin::signed([0; 32].into()),
			1,
			get_milestones(10),
		));
		run_to_block(22);
		assert_eq!(CommunityLoanPool::loan_count(), 1);
	})
}

#[test]
fn milestone_works() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(1);
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(CommunityLoanPool::add_committee_member(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(CommunityLoanPool::propose(
			RuntimeOrigin::signed([1; 32].into()),
			100,
			sp_runtime::MultiAddress::Id([1; 32].into()),
			13,
			20
		));
		assert_ok!(CommunityLoanPool::set_milestones(
			RuntimeOrigin::signed([0; 32].into()),
			1,
			get_milestones(10),
		));
		run_to_block(22);
		assert_eq!(CommunityLoanPool::loans(1).unwrap().available_amount, 10);
		assert_ok!(CommunityLoanPool::propose_milestone(RuntimeOrigin::signed([1; 32].into()), 1));
		assert_ok!(CommunityLoanPool::vote_on_milestone_proposal(
			RuntimeOrigin::signed([0; 32].into()),
			1,
			crate::Vote::Yes
		));
		run_to_block(43);
		assert_eq!(CommunityLoanPool::loans(1).unwrap().available_amount, 20);
		assert_eq!(CommunityLoanPool::loans(1).unwrap().loan_apy, 1023);
	})
}

#[test]
fn withdraw_works() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(CommunityLoanPool::add_committee_member(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(CommunityLoanPool::propose(
			RuntimeOrigin::signed([1; 32].into()),
			100,
			sp_runtime::MultiAddress::Id([1; 32].into()),
			13,
			20
		));
		assert_ok!(CommunityLoanPool::set_milestones(
			RuntimeOrigin::signed([0; 32].into()),
			1,
			get_milestones(10),
		));
		run_to_block(22);
		assert_ok!(CommunityLoanPool::withdraw(RuntimeOrigin::signed([1; 32].into()), 1, 10));
		assert_eq!(Balances::free_balance(&([1; 32].into())), 15_010);
		assert_eq!(CommunityLoanPool::loans(1).unwrap().borrowed_amount, 10);
	})
}

#[test]
fn withdraw_fails_by_wrong_caller() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [2; 32].into()));
		assert_ok!(CommunityLoanPool::add_committee_member(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(CommunityLoanPool::propose(
			RuntimeOrigin::signed([1; 32].into()),
			100,
			sp_runtime::MultiAddress::Id([1; 32].into()),
			13,
			20
		));
		assert_ok!(CommunityLoanPool::set_milestones(
			RuntimeOrigin::signed([0; 32].into()),
			1,
			get_milestones(10),
		));
		run_to_block(22);
		assert_noop!(
			CommunityLoanPool::withdraw(RuntimeOrigin::signed([2; 32].into()), 1, 10),
			Error::<Test>::InsufficientPermission
		);
	})
}

#[test]
fn repay_works() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(CommunityLoanPool::add_committee_member(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(CommunityLoanPool::propose(
			RuntimeOrigin::signed([1; 32].into()),
			100,
			sp_runtime::MultiAddress::Id([1; 32].into()),
			13,
			20
		));
		assert_ok!(CommunityLoanPool::set_milestones(
			RuntimeOrigin::signed([0; 32].into()),
			1,
			get_milestones(10),
		));
		run_to_block(22);
		assert_ok!(CommunityLoanPool::withdraw(RuntimeOrigin::signed([1; 32].into()), 1, 10));
		assert_ok!(CommunityLoanPool::repay(RuntimeOrigin::signed([1; 32].into()), 1, 10));
		assert_eq!(Balances::free_balance(&([1; 32].into())), 15_000);
		assert_eq!(CommunityLoanPool::loans(1).unwrap().borrowed_amount, 0);
	})
}

#[test]
fn repay_if_its_too_much() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(CommunityLoanPool::add_committee_member(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(CommunityLoanPool::propose(
			RuntimeOrigin::signed([1; 32].into()),
			100,
			sp_runtime::MultiAddress::Id([1; 32].into()),
			13,
			20
		));
		assert_ok!(CommunityLoanPool::set_milestones(
			RuntimeOrigin::signed([0; 32].into()),
			1,
			get_milestones(10),
		));
		run_to_block(22);
		assert_ok!(CommunityLoanPool::withdraw(RuntimeOrigin::signed([1; 32].into()), 1, 10));
		assert_noop!(
			CommunityLoanPool::repay(RuntimeOrigin::signed([1; 32].into()), 1, 15),
			Error::<Test>::WantsToRepayTooMuch
		);
	})
}

#[test]
fn deletion_works() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(CommunityLoanPool::add_committee_member(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(CommunityLoanPool::propose(
			RuntimeOrigin::signed([1; 32].into()),
			100,
			sp_runtime::MultiAddress::Id([1; 32].into()),
			13,
			20
		));
		assert_ok!(CommunityLoanPool::set_milestones(
			RuntimeOrigin::signed([0; 32].into()),
			1,
			get_milestones(10),
		));
		run_to_block(22);
		assert_ok!(CommunityLoanPool::withdraw(RuntimeOrigin::signed([1; 32].into()), 1, 10));
		assert_ok!(CommunityLoanPool::repay(RuntimeOrigin::signed([1; 32].into()), 1, 10));
		assert_eq!(Balances::free_balance(&([1; 32].into())), 15_000);
		assert_eq!(CommunityLoanPool::loans(1).unwrap().borrowed_amount, 0);
		assert_ok!(CommunityLoanPool::propose_deletion(RuntimeOrigin::signed([1; 32].into()), 1));
		assert_ok!(CommunityLoanPool::vote_on_deletion_proposal(
			RuntimeOrigin::signed([0; 32].into()),
			1,
			crate::Vote::Yes
		));
		run_to_block(32);
		assert_eq!(CommunityLoanPool::loans(1), None);
	})
}

#[test]
fn charge_apy_works() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(CommunityLoanPool::add_committee_member(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(CommunityLoanPool::propose(
			RuntimeOrigin::signed([0; 32].into()),
			100000000,
			sp_runtime::MultiAddress::Id([0; 32].into()),
			1,
			1
		));
		assert_ok!(CommunityLoanPool::set_milestones(
			RuntimeOrigin::signed([0; 32].into()),
			1,
			get_milestones(10),
		));
		run_to_block(11);
		Timestamp::set_timestamp(10000);
		System::reset_events();
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		CommunityLoanPool::on_initialize(System::block_number());
		assert_eq!(CommunityLoanPool::loans(1).unwrap().borrowed_amount, 3);
		System::assert_last_event(Event::ApyCharged { loan_index: 1, interest_balance: 3 }.into());
	})
}

#[test]
fn charge_apy_and_repaying_the_interests_works() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(CommunityLoanPool::add_committee_member(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(CommunityLoanPool::propose(
			RuntimeOrigin::signed([0; 32].into()),
			100000000,
			sp_runtime::MultiAddress::Id([0; 32].into()),
			1,
			1
		));
		assert_ok!(CommunityLoanPool::set_milestones(
			RuntimeOrigin::signed([0; 32].into()),
			1,
			get_milestones(10),
		));
		run_to_block(11);
		Timestamp::set_timestamp(10000);
		System::reset_events();
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		CommunityLoanPool::on_initialize(System::block_number());
		assert_eq!(CommunityLoanPool::loans(1).unwrap().borrowed_amount, 3);
		System::assert_last_event(Event::ApyCharged { loan_index: 1, interest_balance: 3 }.into());
		assert_eq!(CommunityLoanPool::total_loan_amount(), 100000000);
		assert_eq!(CommunityLoanPool::total_loan_interests(), 3);
		assert_ok!(CommunityLoanPool::withdraw(RuntimeOrigin::signed([0; 32].into()), 1, 10000000));
		assert_ok!(CommunityLoanPool::repay(RuntimeOrigin::signed([0; 32].into()), 1, 10000000));
		assert_eq!(CommunityLoanPool::total_loan_amount(), 90000000);
		assert_eq!(CommunityLoanPool::total_loan_interests(), 3);
	})
}