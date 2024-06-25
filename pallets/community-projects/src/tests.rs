use crate::{mock::*, Error};
use frame_support::{
	assert_noop, assert_ok,
	traits::{OnFinalize, OnInitialize},
};

use crate::Config;
use crate::{BalanceOf, BoundedNftDonationTypes, NftDonationTypes};
use sp_core::bounded::BoundedVec;

macro_rules! bvec {
	($( $x:tt )*) => {
		vec![$( $x )*].try_into().unwrap()
	}
}

fn get_project_nfts(mut n: u32) -> BoundedNftDonationTypes<Test> {
	let max = <Test as Config>::MaxNftTypes::get();
	if n > max {
		n = max
	}
	(1..=n)
		.map(|x| NftDonationTypes::<BalanceOf<Test>> { price: (100 * x).into(), amount: x })
		.collect::<Vec<NftDonationTypes<BalanceOf<Test>>>>()
		.try_into()
		.expect("bound is ensured; qed")
}

fn get_nft_metadata(
	mut n: u32,
) -> BoundedVec<
	BoundedVec<u8, <Test as pallet_nfts::Config>::StringLimit>,
	<Test as Config>::MaxNftTypes,
> {
	let max = <Test as Config>::MaxNftTypes::get();
	if n > max {
		n = max
	}
	(1..=n)
		.map(|_| bvec![22, 22])
		.collect::<Vec<BoundedVec<u8, <Test as pallet_nfts::Config>::StringLimit>>>()
		.try_into()
		.expect("bound is ensured; qed")
}

fn run_to_block(n: u64) {
	while System::block_number() < n {
		if System::block_number() > 0 {
			CommunityProjects::on_finalize(System::block_number());
			System::on_finalize(System::block_number());
		}
		System::reset_events();
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		CommunityProjects::on_initialize(System::block_number());
	}
}

#[test]
fn list_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			get_nft_metadata(3),
			5,
			400,
			bvec![22, 22]
		));
		//assert_eq!(CommunityProjects::listed_nfts().len(), 6);
	});
}

#[test]
fn list_fails_with_not_enough_metadata() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_noop!(
			CommunityProjects::list_project(
				RuntimeOrigin::signed([0; 32].into()),
				get_project_nfts(3),
				get_nft_metadata(4),
				5,
				400,
				bvec![22, 22]
			),
			Error::<Test>::WrongAmountOfMetadata
		);
	});
}

#[test]
fn list_fails_with_price_too_high() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_noop!(
			CommunityProjects::list_project(
				RuntimeOrigin::signed([0; 32].into()),
				get_project_nfts(4),
				get_nft_metadata(4),
				5,
				3100,
				bvec![22, 22]
			),
			Error::<Test>::PriceCannotBeReached
		);
	});
}

#[test]
fn buy_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			get_nft_metadata(3),
			5,
			400,
			bvec![22, 22]
		));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 2, 1));
		//assert_eq!(CommunityProjects::listed_nfts().len(), 5);
		assert_eq!(Assets::balance(1, &[1; 32].into()), 1300);
		assert_eq!(Assets::balance(1, &CommunityProjects::account_id()), 200);
	});
}

#[test]
fn buy_works_multiple_nfts_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			get_nft_metadata(3),
			5,
			600,
			bvec![22, 22]
		));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 3, 3));
		//assert_eq!(CommunityProjects::listed_nfts().len(), 0);
		assert_eq!(Assets::balance(1, &[1; 32].into()), 900);
		assert_eq!(Assets::balance(1, &CommunityProjects::account_id()), 600);
		assert_eq!(CommunityProjects::listed_nft_types(0, 1), None);
		assert_eq!(CommunityProjects::listed_nft_types(0, 1), None);
	});
}

#[test]
fn buy_fails_multiple_nfts() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [5; 32].into()));
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			get_nft_metadata(3),
			5,
			600,
			bvec![22, 22]
		));
		assert_noop!(
			CommunityProjects::buy_nft(RuntimeOrigin::signed([5; 32].into()), 0, 3, 3),
			Error::<Test>::NotEnoughFunds
		);
		assert_eq!(Assets::balance(1, &[5; 32].into()), 500);
	});
}

#[test]
fn buy_fails_nft_not_available() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			get_nft_metadata(3),
			5,
			400,
			bvec![22, 22]
		));
		assert_noop!(
			CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 2, 1, 1),
			Error::<Test>::InvalidIndex
		);
	});
}

#[test]
fn buy_fails_nft_not_enough_assets() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [4; 32].into()));
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			get_nft_metadata(3),
			5,
			400,
			bvec![22, 22]
		));
		assert_noop!(
			CommunityProjects::buy_nft(RuntimeOrigin::signed([4; 32].into()), 0, 1, 1),
			Error::<Test>::NotEnoughFunds
		);
		//assert_eq!(CommunityProjects::listed_nfts().len(), 6);
	});
}

#[test]
fn launch_project_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [2; 32].into()));
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			get_nft_metadata(3),
			5,
			300,
			bvec![22, 22]
		));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 1, 1));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([2; 32].into()), 0, 2, 1));
		//assert_eq!(CommunityProjects::listed_nfts().len(), 0);
	});
}

#[test]
fn voting_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [2; 32].into()));
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			get_nft_metadata(3),
			5,
			300,
			bvec![22, 22]
		));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 2, 1));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([2; 32].into()), 0, 1, 1));
		//assert_eq!(CommunityProjects::listed_nfts().len(), 0);
		run_to_block(11);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([2; 32].into()),
			0,
			crate::Vote::Yes
		));
		assert_eq!(CommunityProjects::ongoing_votes(0).unwrap().yes_votes, 100);
	});
}

#[test]
fn rejecting_vote_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [2; 32].into()));
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(4),
			get_nft_metadata(4),
			4,
			900,
			bvec![22, 22]
		));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 3, 1));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 3, 1));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([2; 32].into()), 0, 3, 1));
		assert_eq!(Assets::balance(1, &[1; 32].into()), 900);
		assert_eq!(Assets::balance(1, &[2; 32].into()), 149_700);
		assert_eq!(Assets::balance(1, &CommunityProjects::account_id()), 900);
		run_to_block(11);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([2; 32].into()),
			0,
			crate::Vote::Yes
		));
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([1; 32].into()),
			0,
			crate::Vote::Yes
		));
		assert_eq!(CommunityProjects::ongoing_votes(0).unwrap().no_votes, 0);
		run_to_block(31);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([1; 32].into()),
			0,
			crate::Vote::No
		));
		run_to_block(51);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([1; 32].into()),
			0,
			crate::Vote::No
		));
		run_to_block(71);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([1; 32].into()),
			0,
			crate::Vote::No
		));
		run_to_block(81);
		assert_eq!(CommunityProjects::ongoing_projects(0), None);
		assert_ok!(CommunityProjects::claim_refunded_token(
			RuntimeOrigin::signed([1; 32].into()),
			0,
		));
		assert_ok!(CommunityProjects::claim_refunded_token(
			RuntimeOrigin::signed([2; 32].into()),
			0,
		));
		assert_eq!(Assets::balance(1, &[1; 32].into()), 1_350);
		assert_eq!(Assets::balance(1, &[2; 32].into()), 149_925);
		assert_eq!(Assets::balance(1, &CommunityProjects::account_id()), 0);
	})
}

#[test]
fn voting_fails_with_no_permission() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [2; 32].into()));
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			get_nft_metadata(3),
			3,
			300,
			bvec![22, 22]
		));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 3, 1));
		run_to_block(11);
		assert_noop!(
			CommunityProjects::vote_on_milestone(
				RuntimeOrigin::signed([2; 32].into()),
				0,
				crate::Vote::Yes
			),
			Error::<Test>::InsufficientPermission
		);
	})
}

#[test]
fn voting_fails_with_double_voting() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			get_nft_metadata(3),
			3,
			300,
			bvec![22, 22]
		));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 3, 1));
		run_to_block(11);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([1; 32].into()),
			0,
			crate::Vote::Yes
		));
		assert_noop!(
			CommunityProjects::vote_on_milestone(
				RuntimeOrigin::signed([1; 32].into()),
				0,
				crate::Vote::Yes
			),
			Error::<Test>::AlreadyVoted
		);
	})
}

#[test]
fn voting_fails_with_no_ongoing_voting() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			get_nft_metadata(3),
			3,
			300,
			bvec![22, 22]
		));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 3, 1));
		assert_noop!(
			CommunityProjects::vote_on_milestone(
				RuntimeOrigin::signed([1; 32].into()),
				0,
				crate::Vote::Yes
			),
			Error::<Test>::NoOngoingVotingPeriod
		);
	})
}

#[test]
fn set_strikes_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			get_nft_metadata(3),
			3,
			300,
			bvec![22, 22]
		));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 3, 1));
		run_to_block(11);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([1; 32].into()),
			0,
			crate::Vote::No
		));
		run_to_block(31);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([1; 32].into()),
			0,
			crate::Vote::No
		));
		run_to_block(41);
		assert_eq!(CommunityProjects::ongoing_projects(0).unwrap().strikes, 2);
		run_to_block(51);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([1; 32].into()),
			0,
			crate::Vote::Yes
		));
		run_to_block(61);
		assert_eq!(CommunityProjects::ongoing_projects(0).unwrap().strikes, 0);
	})
}

#[test]
fn distributing_funds_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [2; 32].into()));
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			get_nft_metadata(3),
			3,
			300,
			bvec![22, 22]
		));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 1, 1));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([2; 32].into()), 0, 2, 1));
		//assert_eq!(CommunityProjects::listed_nfts().len(), 0);
		run_to_block(11);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([2; 32].into()),
			0,
			crate::Vote::Yes
		));
		run_to_block(22);
		assert_eq!(Assets::balance(1, &[0; 32].into()), 20_000_100);
		assert_eq!(Assets::balance(1, &CommunityProjects::account_id()), 200);
	});
}

#[test]
fn distributing_funds_for_2_rounds_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [2; 32].into()));
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			get_nft_metadata(3),
			3,
			300,
			bvec![22, 22]
		));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 1, 1));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([2; 32].into()), 0, 2, 1));
		//assert_eq!(CommunityProjects::listed_nfts().len(), 0);
		run_to_block(11);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([2; 32].into()),
			0,
			crate::Vote::Yes
		));
		run_to_block(22);
		assert_eq!(Assets::balance(1, &[0; 32].into()), 20_000_100);
		assert_eq!(Assets::balance(1, &CommunityProjects::account_id()), 200);
		run_to_block(32);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([2; 32].into()),
			0,
			crate::Vote::Yes
		));
		run_to_block(42);
		assert_eq!(Assets::balance(1, &[0; 32].into()), 20_000_200);
		assert_eq!(Assets::balance(1, &CommunityProjects::account_id()), 100);
	});
}

#[test]
fn delete_project_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [2; 32].into()));
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			get_nft_metadata(3),
			1,
			300,
			bvec![22, 22]
		));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 2, 1));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([2; 32].into()), 0, 1, 1));
		run_to_block(11);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([2; 32].into()),
			0,
			crate::Vote::Yes
		));
		run_to_block(22);
		assert_eq!(Assets::balance(1, &[0; 32].into()), 20_000_300);
		assert_eq!(Assets::balance(1, &CommunityProjects::account_id()), 0);
		assert_eq!(CommunityProjects::ongoing_projects(0), None);
	})
}

#[test]
fn create_project_duration_longer_12() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [2; 32].into()));
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			get_nft_metadata(3),
			24,
			300,
			bvec![22, 22]
		));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 2, 1));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([2; 32].into()), 0, 1, 1));
		run_to_block(18);
		assert_noop!(
			CommunityProjects::vote_on_milestone(
				RuntimeOrigin::signed([2; 32].into()),
				0,
				crate::Vote::Yes
			),
			Error::<Test>::NoOngoingVotingPeriod
		);
		run_to_block(21);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([2; 32].into()),
			0,
			crate::Vote::Yes
		));
		run_to_block(32);
		assert_eq!(Assets::balance(1, &[0; 32].into()), 20_000_025);
		assert_eq!(Assets::balance(1, &CommunityProjects::account_id()), 275);
		run_to_block(52);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([2; 32].into()),
			0,
			crate::Vote::Yes
		));
	})
}

#[test]
fn bonding_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [2; 32].into()));
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			get_nft_metadata(3),
			2,
			320,
			bvec![22, 22]
		));
		assert_eq!(Balances::free_balance(&[0; 32].into()), 19_999_998);
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 2, 1));
		assert_ok!(CommunityProjects::bond_token(RuntimeOrigin::signed([1; 32].into()), 0, 30));
		assert_eq!(CommunityProjects::total_bonded(), 30);
		assert_eq!(
			CommunityProjects::project_bonding::<u32, AccountId>(0, [1; 32].into()).unwrap(),
			30
		);
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([2; 32].into()), 0, 1, 1));
		run_to_block(11);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([2; 32].into()),
			0,
			crate::Vote::Yes
		));
		run_to_block(31);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([2; 32].into()),
			0,
			crate::Vote::Yes
		));
		assert_eq!(
			CommunityProjects::project_bonding::<u32, AccountId>(0, [1; 32].into()).unwrap(),
			30
		);
		assert_eq!(CommunityProjects::user_bonded_amount::<AccountId>([1; 32].into()).unwrap(), 30);
		run_to_block(42);
		assert_eq!(Assets::balance(1, &[0; 32].into()), 20_000_300);
		assert_eq!(Balances::free_balance(&[0; 32].into()), 20_000_028);
		assert_eq!(CommunityProjects::total_bonded(), 30);
		//assert_eq!(CommunityProjects::project_bonding::<u32, AccountId>(0, [1; 32].into()), None);
		assert_eq!(CommunityProjects::user_bonded_amount::<AccountId>([1; 32].into()).unwrap(), 30);
	})
}

#[test]
fn bonding_on_more_projects_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [2; 32].into()));
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			get_nft_metadata(3),
			1,
			320,
			bvec![22, 22]
		));
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(4),
			get_nft_metadata(4),
			2,
			320,
			bvec![22, 22]
		));
		assert_eq!(Balances::free_balance(&[0; 32].into()), 19_999_996);
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 2, 1));
		assert_ok!(CommunityProjects::bond_token(RuntimeOrigin::signed([1; 32].into()), 0, 30));
		assert_ok!(CommunityProjects::bond_token(RuntimeOrigin::signed([1; 32].into()), 1, 10));
		assert_eq!(CommunityProjects::total_bonded(), 40);
		assert_eq!(
			CommunityProjects::project_bonding::<u32, AccountId>(0, [1; 32].into()).unwrap(),
			30
		);
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([2; 32].into()), 0, 1, 1));
		run_to_block(11);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([2; 32].into()),
			0,
			crate::Vote::Yes
		));
		run_to_block(22);
		assert_eq!(Assets::balance(1, &[0; 32].into()), 20_000_300);
		assert_eq!(Balances::free_balance(&[0; 32].into()), 20_000_026);
		assert_eq!(CommunityProjects::total_bonded(), 40);
		//assert_eq!(CommunityProjects::project_bonding::<u32, AccountId>(0, [1; 32].into()), None);
		assert_eq!(CommunityProjects::user_bonded_amount::<AccountId>([1; 32].into()).unwrap(), 40);
	})
}

#[test]
fn bonding_fails_not_enough_funds() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(4),
			get_nft_metadata(4),
			2,
			800,
			bvec![22, 22]
		));
		assert_noop!(
			CommunityProjects::bond_token(RuntimeOrigin::signed([1; 32].into()), 0, 50),
			Error::<Test>::NotEnoughBondingFundsAvailable
		);
	})
}

#[test]
fn bonding_fails_over_10_percent() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(4),
			get_nft_metadata(4),
			2,
			200,
			bvec![22, 22]
		));
		assert_noop!(
			CommunityProjects::bond_token(RuntimeOrigin::signed([1; 32].into()), 0, 30),
			Error::<Test>::ProjectCanOnlyHave10PercentBonding
		);
	})
}

#[test]
fn bonding_fails_project_ongoing() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [2; 32].into()));
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			get_nft_metadata(3),
			2,
			300,
			bvec![22, 22]
		));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 2, 1));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([2; 32].into()), 0, 1, 1));
		assert_noop!(
			CommunityProjects::bond_token(RuntimeOrigin::signed([1; 32].into()), 0, 30),
			Error::<Test>::ProjectOngoing
		);
	})
}

#[test]
fn bonding_fails_user_not_enough_funds() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [6; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [2; 32].into()));
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			get_nft_metadata(3),
			2,
			300,
			bvec![22, 22]
		));
		assert_ok!(CommunityProjects::bond_token(RuntimeOrigin::signed([6; 32].into()), 0, 30));
		assert_eq!(CommunityProjects::total_bonded(), 0);
	})
}

#[test]
fn claim_refunded_token_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [2; 32].into()));
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			get_nft_metadata(3),
			3,
			300,
			bvec![22, 22]
		));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 1, 1));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([2; 32].into()), 0, 2, 1));
		//assert_eq!(CommunityProjects::listed_nfts().len(), 0);
		run_to_block(11);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([2; 32].into()),
			0,
			crate::Vote::Yes
		));
		run_to_block(22);
		assert_eq!(Assets::balance(1, &[0; 32].into()), 20_000_100);
		assert_eq!(Assets::balance(1, &CommunityProjects::account_id()), 200);
		assert_eq!(Assets::balance(1, &[2; 32].into()), 149_800);
		run_to_block(100);
		assert_eq!(CommunityProjects::ended_projects(0).unwrap().project_success, false);
		assert_ok!(CommunityProjects::claim_refunded_token(
			RuntimeOrigin::signed([2; 32].into()),
			0
		));
		assert_eq!(Assets::balance(1, &[2; 32].into()), 149_933);
	})
}

#[test]
fn claim_refunded_token_fails() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [2; 32].into()));
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			get_nft_metadata(3),
			3,
			300,
			bvec![22, 22]
		));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 1, 1));
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([2; 32].into()), 0, 2, 1));
		run_to_block(11);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([2; 32].into()),
			0,
			crate::Vote::Yes
		));
		run_to_block(22);
		assert_eq!(Assets::balance(1, &[0; 32].into()), 20_000_100);
		assert_eq!(Assets::balance(1, &CommunityProjects::account_id()), 200);
		assert_eq!(Assets::balance(1, &[2; 32].into()), 149_800);
		assert_noop!(
			CommunityProjects::claim_refunded_token(RuntimeOrigin::signed([2; 32].into()), 0),
			Error::<Test>::InvalidIndex
		);
		run_to_block(100);
		assert_eq!(CommunityProjects::ended_projects(0).unwrap().project_success, false);
		assert_noop!(
			CommunityProjects::claim_refunded_token(RuntimeOrigin::signed([3; 32].into()), 0),
			Error::<Test>::InsufficientPermission
		);
	})
}

#[test]
fn claim_bonding_for_failed_project_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [2; 32].into()));
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			get_nft_metadata(3),
			2,
			320,
			bvec![22, 22]
		));
		assert_eq!(Balances::free_balance(&[0; 32].into()), 19_999_998);
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 2, 1));
		assert_ok!(CommunityProjects::bond_token(RuntimeOrigin::signed([1; 32].into()), 0, 30));
		assert_eq!(CommunityProjects::total_bonded(), 30);
		assert_eq!(
			CommunityProjects::project_bonding::<u32, AccountId>(0, [1; 32].into()).unwrap(),
			30
		);
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([2; 32].into()), 0, 1, 1));
		run_to_block(11);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([2; 32].into()),
			0,
			crate::Vote::Yes
		));
		run_to_block(100);
		assert_eq!(CommunityProjects::ended_projects(0).unwrap().project_success, false);
		assert_ok!(CommunityProjects::claim_bonding(RuntimeOrigin::signed([1; 32].into()), 0));
		assert_eq!(CommunityProjects::user_bonded_amount::<AccountId>([1; 32].into()), None);
	})
}

#[test]
fn claim_bonding_for_succeed_project_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [2; 32].into()));
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			get_nft_metadata(3),
			1,
			320,
			bvec![22, 22]
		));
		assert_eq!(Balances::free_balance(&[0; 32].into()), 19_999_998);
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 2, 1));
		assert_ok!(CommunityProjects::bond_token(RuntimeOrigin::signed([1; 32].into()), 0, 30));
		assert_eq!(CommunityProjects::total_bonded(), 30);
		assert_eq!(
			CommunityProjects::project_bonding::<u32, AccountId>(0, [1; 32].into()).unwrap(),
			30
		);
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([2; 32].into()), 0, 1, 1));
		run_to_block(11);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([2; 32].into()),
			0,
			crate::Vote::Yes
		));
		run_to_block(31);
		assert_eq!(CommunityProjects::ended_projects(0).unwrap().project_success, true);
		assert_ok!(CommunityProjects::claim_bonding(RuntimeOrigin::signed([1; 32].into()), 0));
		assert_eq!(CommunityProjects::user_bonded_amount::<AccountId>([1; 32].into()), None);
	})
}

#[test]
fn claim_bonding_for_failed_project_fails() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [0; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [1; 32].into()));
		assert_ok!(XcavateWhitelist::add_to_whitelist(RuntimeOrigin::root(), [2; 32].into()));
		assert_ok!(CommunityProjects::list_project(
			RuntimeOrigin::signed([0; 32].into()),
			get_project_nfts(3),
			get_nft_metadata(3),
			2,
			320,
			bvec![22, 22]
		));
		assert_eq!(Balances::free_balance(&[0; 32].into()), 19_999_998);
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([1; 32].into()), 0, 2, 1));
		assert_ok!(CommunityProjects::bond_token(RuntimeOrigin::signed([1; 32].into()), 0, 30));
		assert_eq!(CommunityProjects::total_bonded(), 30);
		assert_eq!(
			CommunityProjects::project_bonding::<u32, AccountId>(0, [1; 32].into()).unwrap(),
			30
		);
		assert_ok!(CommunityProjects::buy_nft(RuntimeOrigin::signed([2; 32].into()), 0, 1, 1));
		run_to_block(11);
		assert_ok!(CommunityProjects::vote_on_milestone(
			RuntimeOrigin::signed([2; 32].into()),
			0,
			crate::Vote::Yes
		));
		assert_noop!(
			CommunityProjects::claim_bonding(RuntimeOrigin::signed([1; 32].into()), 0),
			Error::<Test>::InvalidIndex
		);
		run_to_block(100);
		assert_eq!(CommunityProjects::ended_projects(0).unwrap().project_success, false);
		assert_noop!(
			CommunityProjects::claim_bonding(RuntimeOrigin::signed([2; 32].into()), 0),
			Error::<Test>::NoBondingYet
		);
	})
}
