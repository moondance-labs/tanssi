use crate::{mock::*, Error};
use frame_support::{
	assert_noop, assert_ok,
	traits::{OnFinalize, OnInitialize},
	BoundedVec,
};

macro_rules! bvec {
	($( $x:tt )*) => {
		vec![$( $x )*].try_into().unwrap()
	}
}

fn run_to_block(n: u64) {
	while System::block_number() < n {
		if System::block_number() > 0 {
			PropertyGovernance::on_finalize(System::block_number());
			System::on_finalize(System::block_number());
		}
		System::reset_events();
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		PropertyGovernance::on_initialize(System::block_number());
	}
}

#[test]
fn propose_works() {
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
		assert_ok!(PropertyGovernance::propose(RuntimeOrigin::signed([1; 32].into()), 0));
		assert_eq!(PropertyGovernance::proposals(1).unwrap().asset_id, 0);
		assert_eq!(PropertyGovernance::ongoing_votes(1).is_some(), true);
	});
}

#[test]
fn propose_fails() {
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
		assert_noop!(PropertyGovernance::propose(RuntimeOrigin::signed([2; 32].into()), 0), Error::<Test>::NoPermission);
	});
}

#[test]
fn inquery_against_letting_agent_works() {
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
		assert_ok!(PropertyGovernance::inquery_against_letting_agent(RuntimeOrigin::signed([1; 32].into()), 0));
		assert_eq!(PropertyGovernance::inqueries(1).is_some(), true);
	});
}

#[test]
fn vote_on_proposal_works() {
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
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([1; 32].into()), 0, 30));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([1; 32].into()), 0, 20));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([2; 32].into()), 0, 10));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([3; 32].into()), 0, 40));
		assert_ok!(PropertyGovernance::propose(RuntimeOrigin::signed([1; 32].into()), 0));
		assert_ok!(PropertyGovernance::vote_on_proposal(RuntimeOrigin::signed([1; 32].into()), 1, crate::Vote::Yes));
		assert_ok!(PropertyGovernance::vote_on_proposal(RuntimeOrigin::signed([2; 32].into()), 1, crate::Vote::Yes));
		assert_ok!(PropertyGovernance::vote_on_proposal(RuntimeOrigin::signed([3; 32].into()), 1, crate::Vote::No));
		assert_eq!(PropertyGovernance::ongoing_votes(1).unwrap().yes_votes, 60);
		assert_eq!(PropertyGovernance::ongoing_votes(1).unwrap().no_votes, 40);
	});
}

#[test]
fn proposal_pass() {
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
		assert_ok!(PropertyGovernance::propose(RuntimeOrigin::signed([1; 32].into()), 0));
		assert_ok!(PropertyGovernance::vote_on_proposal(RuntimeOrigin::signed([1; 32].into()), 1, crate::Vote::Yes));
		assert_eq!(PropertyGovernance::proposals(1).is_some(), true);
		run_to_block(31);
		assert_eq!(PropertyGovernance::proposals(1).is_none(), true);
	});
}

#[test]
fn proposal_not_pass() {
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
		assert_ok!(PropertyGovernance::propose(RuntimeOrigin::signed([1; 32].into()), 0));
		assert_ok!(PropertyGovernance::vote_on_proposal(RuntimeOrigin::signed([1; 32].into()), 1, crate::Vote::No));
		assert_eq!(PropertyGovernance::proposals(1).is_some(), true);
		run_to_block(31);
		assert_eq!(PropertyGovernance::proposals(1).is_none(), true);
	});
}

#[test]
fn vote_on_proposal_fails() {
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
		assert_noop!(PropertyGovernance::vote_on_proposal(RuntimeOrigin::signed([1; 32].into()), 1, crate::Vote::Yes), Error::<Test>::NotOngoing);
		assert_ok!(PropertyGovernance::propose(RuntimeOrigin::signed([1; 32].into()), 0));
		assert_ok!(PropertyGovernance::vote_on_proposal(RuntimeOrigin::signed([1; 32].into()), 1, crate::Vote::Yes));
		assert_noop!(PropertyGovernance::vote_on_proposal(RuntimeOrigin::signed([2; 32].into()), 1, crate::Vote::Yes), Error::<Test>::NoPermission);
		assert_noop!(PropertyGovernance::vote_on_proposal(RuntimeOrigin::signed([1; 32].into()), 1, crate::Vote::Yes), Error::<Test>::AlreadyVoted);
	});
}

#[test]
fn vote_on_inquery_works() {
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
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([2; 32].into()), 0, 10));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([3; 32].into()), 0, 40));
		assert_ok!(PropertyGovernance::inquery_against_letting_agent(RuntimeOrigin::signed([1; 32].into()), 0));
		assert_ok!(PropertyGovernance::vote_on_letting_agent_inquery(RuntimeOrigin::signed([1; 32].into()), 1, crate::Vote::Yes));
		assert_ok!(PropertyGovernance::vote_on_letting_agent_inquery(RuntimeOrigin::signed([3; 32].into()), 1, crate::Vote::Yes));
		assert_ok!(PropertyGovernance::vote_on_letting_agent_inquery(RuntimeOrigin::signed([2; 32].into()), 1, crate::Vote::No));
		assert_eq!(PropertyGovernance::ongoing_inquery_votes(1).unwrap().yes_votes, 60);
		assert_eq!(PropertyGovernance::ongoing_inquery_votes(1).unwrap().no_votes, 40);
	});
}

#[test]
fn inquery_pass() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(NftMarketplace::create_new_region(RuntimeOrigin::root()));
		assert_ok!(NftMarketplace::create_new_location(RuntimeOrigin::root(), 0, bvec![10, 10]));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [2; 32].into()));
		assert_ok!(PropertyManagement::add_letting_agent(
			RuntimeOrigin::root(),
			0,
			bvec![10, 10],
			[0; 32].into(),
		));
		assert_ok!(PropertyManagement::letting_agent_deposit(RuntimeOrigin::signed([0; 32].into())));
		assert_ok!(PropertyManagement::add_letting_agent(
			RuntimeOrigin::root(),
			0,
			bvec![10, 10],
			[1; 32].into(),
		));
		assert_ok!(PropertyManagement::letting_agent_deposit(RuntimeOrigin::signed([1; 32].into())));
		assert_ok!(NftMarketplace::list_object(
			RuntimeOrigin::signed([0; 32].into()),
			0,
			bvec![10, 10],
			10_000,
			100,
			bvec![22, 22]
		));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([1; 32].into()), 0, 30));
		assert_ok!(NftMarketplace::buy_token(RuntimeOrigin::signed([2; 32].into()), 0, 70));
		assert_ok!(PropertyManagement::set_letting_agent(RuntimeOrigin::signed([0; 32].into()), 0));
		assert_eq!(PropertyManagement::letting_storage(0).unwrap(), [0; 32].into());
		assert_eq!(PropertyManagement::letting_agent_locations::<u32, BoundedVec<u8, Postcode>>(0, bvec![10, 10]).len(), 2);
		assert_ok!(PropertyGovernance::inquery_against_letting_agent(RuntimeOrigin::signed([1; 32].into()), 0));
		assert_eq!(PropertyGovernance::inqueries(1).unwrap().asset_id, 0);
		assert_ok!(PropertyGovernance::vote_on_letting_agent_inquery(RuntimeOrigin::signed([1; 32].into()), 1, crate::Vote::Yes));
		assert_ok!(PropertyGovernance::vote_on_letting_agent_inquery(RuntimeOrigin::signed([2; 32].into()), 1, crate::Vote::Yes));
		assert_eq!(PropertyGovernance::inquery_rounds_expiring(31).len(), 1);
		run_to_block(31);
		assert_eq!(PropertyManagement::letting_storage(0).unwrap(), [1; 32].into());
		assert_eq!(PropertyManagement::letting_agent_locations::<u32, BoundedVec<u8, Postcode>>(0, bvec![10, 10]).len(), 1);
		assert_eq!(PropertyGovernance::inqueries(1).is_none(), true);
	});
}

#[test]
fn inquery_not_pass() {
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
		assert_ok!(PropertyGovernance::inquery_against_letting_agent(RuntimeOrigin::signed([1; 32].into()), 0));
		assert_ok!(PropertyGovernance::vote_on_letting_agent_inquery(RuntimeOrigin::signed([1; 32].into()), 1, crate::Vote::No));
		assert_eq!(PropertyGovernance::inqueries(1).is_some(), true);
		run_to_block(31);
		assert_eq!(PropertyGovernance::inqueries(1).is_none(), true);
	});
}

#[test]
fn vote_on_inquery_fails() {
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
		assert_noop!(PropertyGovernance::vote_on_letting_agent_inquery(RuntimeOrigin::signed([1; 32].into()), 1, crate::Vote::Yes), Error::<Test>::NotOngoing);
		assert_ok!(PropertyGovernance::inquery_against_letting_agent(RuntimeOrigin::signed([1; 32].into()), 0));
		assert_ok!(PropertyGovernance::vote_on_letting_agent_inquery(RuntimeOrigin::signed([1; 32].into()), 1, crate::Vote::Yes));
		assert_noop!(PropertyGovernance::vote_on_letting_agent_inquery(RuntimeOrigin::signed([2; 32].into()), 1, crate::Vote::Yes), Error::<Test>::NoPermission);
		assert_noop!(PropertyGovernance::vote_on_letting_agent_inquery(RuntimeOrigin::signed([1; 32].into()), 1, crate::Vote::Yes), Error::<Test>::AlreadyVoted);
	});
}

