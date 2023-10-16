use crate::{mock::*, Error, Event};
use frame_support::{
	assert_noop, assert_ok,
	traits::{OnFinalize, OnInitialize},
};

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
fn stake_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateStaking::stake(RuntimeOrigin::signed([0; 32].into()), 100));
		System::assert_last_event(Event::Locked { staker: [0; 32].into(), amount: 100 }.into());
		let total_stake = XcavateStaking::total_stake();
		assert_eq!(total_stake, 100);
		let stakers = XcavateStaking::active_stakers();
		assert_eq!(stakers.len(), 1);
	});
}

#[test]
fn stake_with_several_people_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateStaking::stake(RuntimeOrigin::signed([0; 32].into()), 100));
		assert_ok!(XcavateStaking::stake(RuntimeOrigin::signed([1; 32].into()), 400));
		assert_ok!(XcavateStaking::stake(RuntimeOrigin::signed([2; 32].into()), 500));
		let total_stake = XcavateStaking::total_stake();
		assert_eq!(total_stake, 1000);
		let stakers = XcavateStaking::active_stakers();
		assert_eq!(stakers.len(), 3);
	})
}

#[test]
fn person_cant_stake_0_token() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_noop!(
			XcavateStaking::stake(RuntimeOrigin::signed([0; 32].into()), 0),
			Error::<Test>::StakingWithNoValue
		);
	})
}

#[test]
fn unstake_works() {
	new_test_ext().execute_with(|| {
		//System::set_block_number(1);
		assert_ok!(XcavateStaking::stake(RuntimeOrigin::signed([0; 32].into()), 100));
		assert_ok!(XcavateStaking::unstake(RuntimeOrigin::signed([0; 32].into()), 100));
		let total_stake = XcavateStaking::total_stake();
		assert_eq!(total_stake, 0);
		let stakers = XcavateStaking::active_stakers();
		assert_eq!(stakers.len(), 0);
	})
}

#[test]
fn unstake_doesnt_work_for_nonstaker() {
	new_test_ext().execute_with(|| {
		assert_ok!(XcavateStaking::stake(RuntimeOrigin::signed([0; 32].into()), 100));
		assert_noop!(
			XcavateStaking::unstake(RuntimeOrigin::signed([1; 32].into()), 100),
			Error::<Test>::NoStaker
		);
	})
}
