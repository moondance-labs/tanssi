use crate::{mock::*, Error, Event};
use frame_support::sp_runtime::Percent;
use frame_support::{
	assert_noop, assert_ok,
	traits::{OnFinalize, OnInitialize},
};

use pallet_community_loan_pool::{BoundedProposedMilestones, Config, ProposedMilestone};

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

#[test]
fn stake_works() {
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
		run_to_block(21);
		assert_eq!(CommunityLoanPool::ongoing_loans().len(), 1);
		assert_ok!(XcavateStaking::stake(RuntimeOrigin::signed([0; 32].into()), 100));
		System::assert_last_event(Event::Locked { staker: [0; 32].into(), amount: 100 }.into());
		let total_stake = XcavateStaking::total_stake();
		assert_eq!(total_stake, 100);
		let stakers = XcavateStaking::active_stakings();
		assert_eq!(stakers.len(), 1);
	});
}

#[test]
fn queue_works() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateStaking::stake(RuntimeOrigin::signed([0; 32].into()), 100));
		assert_eq!(XcavateStaking::queue_ledger(1).unwrap().locked, 100);
		assert_eq!(XcavateStaking::ledger(1), None);
	})
}

#[test]
fn staking_and_queuing_works() {
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
		run_to_block(21);
		assert_eq!(CommunityLoanPool::ongoing_loans().len(), 1);
		assert_ok!(XcavateStaking::stake(RuntimeOrigin::signed([0; 32].into()), 200));
		assert_eq!(XcavateStaking::queue_ledger(1).unwrap().locked, 100);
		assert_eq!(XcavateStaking::ledger(1).unwrap().locked, 100);
	})
}

#[test]
fn withdraw_from_queue() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateStaking::stake(RuntimeOrigin::signed([0; 32].into()), 100));
		assert_eq!(XcavateStaking::queue_ledger(1).unwrap().locked, 100);
		assert_eq!(XcavateStaking::ledger(1), None);
		assert_ok!(XcavateStaking::withdraw_from_queue(
			RuntimeOrigin::signed([0; 32].into()),
			1,
			200
		));
		assert_eq!(XcavateStaking::queue_ledger(1), None);
	})
}

#[test]
fn withdraw_fails_if_caller_not_staker() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [3; 32].into()));
		assert_ok!(XcavateStaking::stake(RuntimeOrigin::signed([0; 32].into()), 100));
		assert_eq!(XcavateStaking::queue_ledger(1).unwrap().locked, 100);
		assert_eq!(XcavateStaking::ledger(1), None);
		assert_noop!(
			XcavateStaking::withdraw_from_queue(RuntimeOrigin::signed([3; 32].into()), 1, 100),
			Error::<Test>::CallerNotStaker
		);
	})
}

#[test]
fn unstake_if_loan_payed_back() {
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
		run_to_block(21);
		assert_eq!(CommunityLoanPool::ongoing_loans().len(), 1);
		assert_ok!(XcavateStaking::stake(RuntimeOrigin::signed([0; 32].into()), 200));
		assert_eq!(XcavateStaking::queue_ledger(1).unwrap().locked, 100);
		assert_eq!(XcavateStaking::ledger(1).unwrap().locked, 100);
		assert_ok!(CommunityLoanPool::withdraw(RuntimeOrigin::signed([1; 32].into()), 1, 10));
		assert_ok!(CommunityLoanPool::repay(RuntimeOrigin::signed([1; 32].into()), 1, 10));
		System::reset_events();
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		XcavateStaking::on_initialize(System::block_number());
		assert_eq!(XcavateStaking::queue_ledger(2).unwrap().locked, 10);
		assert_eq!(XcavateStaking::ledger(1).unwrap().locked, 90);
		assert_eq!(XcavateStaking::queue_staking().len(), 2);
	})
}

#[test]
fn stakes_if_loan_increases() {
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
		run_to_block(21);
		assert_eq!(CommunityLoanPool::ongoing_loans().len(), 1);
		assert_ok!(XcavateStaking::stake(RuntimeOrigin::signed([0; 32].into()), 200));
		assert_eq!(XcavateStaking::queue_ledger(1).unwrap().locked, 100);
		assert_eq!(XcavateStaking::ledger(1).unwrap().locked, 100);
		assert_ok!(CommunityLoanPool::propose(
			RuntimeOrigin::signed([1; 32].into()),
			50,
			sp_runtime::MultiAddress::Id([3; 32].into()),
			13,
			20
		));
		assert_ok!(CommunityLoanPool::set_milestones(
			RuntimeOrigin::signed([0; 32].into()),
			2,
			get_milestones(10),
		));
		run_to_block(41);
		System::reset_events();
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		XcavateStaking::on_initialize(System::block_number());
		assert_eq!(XcavateStaking::queue_ledger(1).unwrap().locked, 50);
		assert_eq!(XcavateStaking::ledger(2).unwrap().locked, 50);
		assert_eq!(XcavateStaking::active_stakings().len(), 2);
	})
}

#[test]
fn stake_with_several_people_works() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [2; 32].into()));
		assert_ok!(CommunityLoanPool::add_committee_member(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(CommunityLoanPool::propose(
			RuntimeOrigin::signed([1; 32].into()),
			10000,
			sp_runtime::MultiAddress::Id([1; 32].into()),
			13,
			20
		));
		assert_ok!(CommunityLoanPool::set_milestones(
			RuntimeOrigin::signed([0; 32].into()),
			1,
			get_milestones(10),
		));
		run_to_block(21);
		assert_eq!(CommunityLoanPool::ongoing_loans().len(), 1);
		assert_ok!(XcavateStaking::stake(RuntimeOrigin::signed([0; 32].into()), 100));
		assert_ok!(XcavateStaking::stake(RuntimeOrigin::signed([1; 32].into()), 400));
		assert_ok!(XcavateStaking::stake(RuntimeOrigin::signed([2; 32].into()), 500));
		let total_stake = XcavateStaking::total_stake();
		assert_eq!(total_stake, 1000);
		let stakers = XcavateStaking::active_stakings();
		assert_eq!(stakers.len(), 3);
	})
}

#[test]
fn person_cant_stake_0_token() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_noop!(
			XcavateStaking::stake(RuntimeOrigin::signed([0; 32].into()), 0),
			Error::<Test>::StakingWithNoValue
		);
	})
}

#[test]
fn unstake_works() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(CommunityLoanPool::add_committee_member(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(CommunityLoanPool::propose(
			RuntimeOrigin::signed([1; 32].into()),
			200,
			sp_runtime::MultiAddress::Id([1; 32].into()),
			13,
			20
		));
		assert_ok!(CommunityLoanPool::set_milestones(
			RuntimeOrigin::signed([0; 32].into()),
			1,
			get_milestones(10),
		));
		run_to_block(21);
		assert_eq!(CommunityLoanPool::ongoing_loans().len(), 1);
		assert_ok!(XcavateStaking::stake(RuntimeOrigin::signed([0; 32].into()), 100));
		assert_ok!(XcavateStaking::unstake(RuntimeOrigin::signed([0; 32].into()), 1, 100));
		let total_stake = XcavateStaking::total_stake();
		assert_eq!(total_stake, 0);
		let stakers = XcavateStaking::active_stakings();
		assert_eq!(stakers.len(), 0);
	})
}

#[test]
fn unstake_fails_if_caller_not_staker() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [3; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(CommunityLoanPool::add_committee_member(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(CommunityLoanPool::propose(
			RuntimeOrigin::signed([1; 32].into()),
			200,
			sp_runtime::MultiAddress::Id([1; 32].into()),
			13,
			20
		));
		assert_ok!(CommunityLoanPool::set_milestones(
			RuntimeOrigin::signed([0; 32].into()),
			1,
			get_milestones(10),
		));
		run_to_block(21);
		assert_eq!(CommunityLoanPool::ongoing_loans().len(), 1);
		assert_ok!(XcavateStaking::stake(RuntimeOrigin::signed([0; 32].into()), 100));
		assert_noop!(
			XcavateStaking::unstake(RuntimeOrigin::signed([3; 32].into()), 1, 100),
			Error::<Test>::CallerNotStaker
		);
	})
}

#[test]
fn unstake_doesnt_work_for_nonstaker() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateStaking::stake(RuntimeOrigin::signed([0; 32].into()), 100));
		assert_noop!(
			XcavateStaking::unstake(RuntimeOrigin::signed([1; 32].into()), 1, 100),
			Error::<Test>::NoStaker
		);
	})
}

#[test]
fn claiming_of_rewards_works() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(CommunityLoanPool::add_committee_member(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(CommunityLoanPool::propose(
			RuntimeOrigin::signed([1; 32].into()),
			10003000,
			sp_runtime::MultiAddress::Id([1; 32].into()),
			13,
			20
		));
		assert_ok!(CommunityLoanPool::set_milestones(
			RuntimeOrigin::signed([0; 32].into()),
			1,
			get_milestones(10),
		));
		run_to_block(21);
		assert_eq!(CommunityLoanPool::ongoing_loans().len(), 1);
		assert_eq!(CommunityLoanPool::loans(1).unwrap().available_amount, 1000300);
		assert_eq!(CommunityLoanPool::total_loan_amount(), 10003000);
		assert_ok!(XcavateStaking::stake(RuntimeOrigin::signed([0; 32].into()), 10000000));
		assert_eq!(XcavateStaking::ledger(1).unwrap().locked, 10000000);
		Timestamp::set_timestamp(10000);
		System::reset_events();
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		XcavateStaking::on_initialize(System::block_number());
		assert_eq!(XcavateStaking::ledger(1).unwrap().locked, 10000000);
		System::assert_last_event(Event::RewardsClaimed { amount: 0, apy: 823 }.into());
	})
}

#[test]
fn unstake_and_adding_staking_from_queue_works() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateStaking::stake(RuntimeOrigin::signed([0; 32].into()), 400));
		assert_eq!(XcavateStaking::queue_ledger(1).unwrap().locked, 400);
		assert_ok!(CommunityLoanPool::add_committee_member(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(CommunityLoanPool::propose(
			RuntimeOrigin::signed([1; 32].into()),
			200,
			sp_runtime::MultiAddress::Id([1; 32].into()),
			13,
			20
		));
		assert_ok!(CommunityLoanPool::set_milestones(
			RuntimeOrigin::signed([0; 32].into()),
			1,
			get_milestones(10),
		));
		run_to_block(21);
		assert_eq!(CommunityLoanPool::ongoing_loans().len(), 1);
		assert_eq!(CommunityLoanPool::loans(1).unwrap().available_amount, 20);
		assert_eq!(CommunityLoanPool::total_loan_amount(), 200);
		//assert_eq!(XcavateStaking::ledger(1).unwrap().locked, 10000000);
		System::reset_events();
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		XcavateStaking::on_initialize(System::block_number());
		assert_eq!(XcavateStaking::ledger(1).unwrap().locked, 200);
		assert_eq!(XcavateStaking::queue_ledger(1).unwrap().locked, 200);
		assert_ok!(XcavateStaking::unstake(RuntimeOrigin::signed([0; 32].into()), 1, 100));
		System::reset_events();
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		XcavateStaking::on_initialize(System::block_number());
		assert_eq!(XcavateStaking::ledger(1).unwrap().locked, 100);
		assert_eq!(XcavateStaking::ledger(2).unwrap().locked, 100);
		assert_eq!(XcavateStaking::queue_ledger(1).unwrap().locked, 100);
	})
}

#[test]
fn repayment_and_adding_to_queue() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateStaking::stake(RuntimeOrigin::signed([0; 32].into()), 400));
		assert_eq!(XcavateStaking::queue_ledger(1).unwrap().locked, 400);
		assert_ok!(CommunityLoanPool::add_committee_member(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(CommunityLoanPool::propose(
			RuntimeOrigin::signed([1; 32].into()),
			200,
			sp_runtime::MultiAddress::Id([1; 32].into()),
			13,
			20
		));
		assert_ok!(CommunityLoanPool::set_milestones(
			RuntimeOrigin::signed([0; 32].into()),
			1,
			get_milestones(1),
		));
		run_to_block(21);
		assert_eq!(CommunityLoanPool::ongoing_loans().len(), 1);
		assert_eq!(CommunityLoanPool::loans(1).unwrap().available_amount, 200);
		assert_eq!(CommunityLoanPool::total_loan_amount(), 200);
		//assert_eq!(XcavateStaking::ledger(1).unwrap().locked, 10000000);
		System::reset_events();
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		XcavateStaking::on_initialize(System::block_number());
		assert_eq!(XcavateStaking::ledger(1).unwrap().locked, 200);
		assert_eq!(XcavateStaking::queue_ledger(1).unwrap().locked, 200);
		assert_ok!(CommunityLoanPool::withdraw(RuntimeOrigin::signed([1; 32].into()), 1, 100));
		assert_ok!(CommunityLoanPool::repay(RuntimeOrigin::signed([1; 32].into()), 1, 100));
		System::reset_events();
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		XcavateStaking::on_initialize(System::block_number());
		assert_eq!(XcavateStaking::ledger(1).unwrap().locked, 100);
		assert_eq!(XcavateStaking::queue_ledger(1).unwrap().locked, 200);
		assert_eq!(XcavateStaking::queue_ledger(2).unwrap().locked, 100);
	})
}

#[test]
fn testing_issue() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateStaking::stake(RuntimeOrigin::signed([0; 32].into()), 10000000));
		assert_eq!(XcavateStaking::queue_ledger(1).unwrap().locked, 10000000);
		assert_ok!(CommunityLoanPool::add_committee_member(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(CommunityLoanPool::propose(
			RuntimeOrigin::signed([1; 32].into()),
			5003000,
			sp_runtime::MultiAddress::Id([1; 32].into()),
			13,
			20
		));
		assert_ok!(CommunityLoanPool::set_milestones(
			RuntimeOrigin::signed([0; 32].into()),
			1,
			get_milestones(10),
		));
		run_to_block(21);
		assert_eq!(CommunityLoanPool::ongoing_loans().len(), 1);
		assert_eq!(CommunityLoanPool::loans(1).unwrap().available_amount, 500300);
		assert_eq!(CommunityLoanPool::total_loan_amount(), 5003000);
		//assert_eq!(XcavateStaking::ledger(1).unwrap().locked, 10000000);
		Timestamp::set_timestamp(10000);
		System::reset_events();
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		XcavateStaking::on_initialize(System::block_number());
		assert_eq!(XcavateStaking::ledger(1).unwrap().locked, 5003000);
		assert_eq!(XcavateStaking::queue_ledger(1).unwrap().locked, 4997000);
	})
}