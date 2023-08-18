// Copyright (C) Moondance Labs Ltd.
// This file is part of Tanssi.

// Tanssi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tanssi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>

use crate::{assert_fields_eq, AllTargetPool, TargetPool};

use {
    crate::{
        assert_eq_events,
        candidate::Candidates,
        mock::*,
        pool_test,
        pools::{self, Pool},
        Error, Event, PendingOperationQuery, Shares, Stake,
    },
    frame_support::{assert_noop, assert_ok, traits::tokens::fungible::Mutate},
};

type Joining = pools::Joining<Runtime>;

fn do_request_delegation(
    candidate: AccountId,
    delegator: AccountId,
    pool: TargetPool,
    amount: Balance,
) {
    let before = State::extract(candidate, delegator);
    let pool_before = PoolState::extract::<Joining>(candidate, delegator);

    assert_ok!(Staking::request_delegate(
        RuntimeOrigin::signed(delegator),
        candidate,
        pool,
        amount,
    ));

    let after = State::extract(candidate, delegator);
    let pool_after = PoolState::extract::<Joining>(candidate, delegator);

    // Actual balances don't change
    assert_fields_eq!(before, after, [delegator_balance, staking_balance]);
    assert_eq!(before.delegator_hold + amount, after.delegator_hold);
    assert_eq!(pool_before.hold + amount, pool_after.hold);
    assert_eq!(pool_before.stake + amount, pool_after.stake);
    assert_eq!(
        before.candidate_total_stake + amount,
        after.candidate_total_stake
    );
}

fn do_execute_delegation<P: PoolExt<Runtime>>(
    candidate: AccountId,
    delegator: AccountId,
    block_number: u64,
    expected_increase: Balance,
) {
    let before = State::extract(candidate, delegator);
    let joining_before = PoolState::extract::<Joining>(candidate, delegator);
    let pool_before = PoolState::extract::<P>(candidate, delegator);
    let request_before = crate::PendingOperations::<Runtime>::get(
        delegator,
        P::joining_operation_key(candidate, block_number),
    );

    let refund = request_before
        .checked_sub(expected_increase)
        .expect("expected increase should be <= requested amount");

    assert_ok!(Staking::execute_pending_operations(
        RuntimeOrigin::signed(delegator),
        vec![PendingOperationQuery {
            delegator: delegator,
            operation: P::joining_operation_key(candidate, block_number)
        }]
    ));

    let after = State::extract(candidate, delegator);
    let joining_after = PoolState::extract::<Joining>(candidate, delegator);
    let pool_after = PoolState::extract::<P>(candidate, delegator);
    let request_after = crate::PendingOperations::<Runtime>::get(
        delegator,
        P::joining_operation_key(candidate, block_number),
    );

    // Actual balances don't change
    assert_fields_eq!(before, after, [delegator_balance, staking_balance]);
    // However funds are held (with share rounding released)
    assert_eq!(request_after, 0);

    assert_eq!(before.delegator_hold - refund, after.delegator_hold);
    assert_eq!(
        before.candidate_total_stake - refund,
        after.candidate_total_stake
    );

    assert_eq!(joining_before.hold - request_before, joining_after.hold);
    assert_eq!(joining_before.stake - request_before, joining_after.stake);

    assert_eq!(pool_before.hold + expected_increase, pool_after.hold);
    assert_eq!(pool_before.stake + expected_increase, pool_after.stake);
}

fn do_full_delegation<P: PoolExt<Runtime>>(
    candidate: AccountId,
    delegator: AccountId,
    request_amount: Balance,
    expected_increase: Balance,
) {
    let block_number = block_number();
    do_request_delegation(candidate, delegator, P::target_pool(), request_amount);
    roll_to(block_number + 2);
    do_execute_delegation::<P>(
        ACCOUNT_CANDIDATE_1,
        ACCOUNT_DELEGATOR_1,
        block_number,
        expected_increase,
    );
}

fn do_rebalance_hold<P: Pool<Runtime>>(
    candidate: AccountId,
    delegator: AccountId,
    target_pool: AllTargetPool,
    expected_rebalance: SignedBalance,
) {
    let before = State::extract(candidate, delegator);
    let pool_before = PoolState::extract::<P>(candidate, delegator);

    assert_ok!(Staking::rebalance_hold(
        RuntimeOrigin::signed(ACCOUNT_DELEGATOR_1),
        ACCOUNT_CANDIDATE_1,
        ACCOUNT_DELEGATOR_1,
        target_pool
    ));

    let after = State::extract(candidate, delegator);
    let pool_after = PoolState::extract::<P>(candidate, delegator);

    // Balances should update
    match expected_rebalance {
        SignedBalance::Positive(balance) => {
            assert_eq!(pool_before.hold + balance, pool_after.hold);
            assert_eq!(before.delegator_balance + balance, after.delegator_balance);
            assert_eq!(before.staking_balance - balance, after.staking_balance);
        }
        SignedBalance::Negative(balance) => {
            assert_eq!(pool_before.hold - balance, pool_after.hold);
            assert_eq!(before.delegator_balance - balance, after.delegator_balance);
            assert_eq!(before.staking_balance + balance, after.staking_balance);
        }
    }

    // Stake stay the same.
    assert_fields_eq!(pool_before, pool_after, stake);
}

pool_test!(
    fn empty_delegation<P>() {
        ExtBuilder::default().build().execute_with(|| {
            assert_noop!(
                Staking::request_delegate(
                    RuntimeOrigin::signed(ACCOUNT_DELEGATOR_1),
                    ACCOUNT_CANDIDATE_1,
                    P::target_pool(),
                    0
                ),
                Error::<Runtime>::StakeMustBeNonZero
            );

            // No change
            assert_eq!(total_balance(&ACCOUNT_DELEGATOR_1), 1 * DEFAULT_BALANCE);
            assert_eq!(total_balance(&ACCOUNT_STAKING), DEFAULT_BALANCE);
            assert_eq!(balance_hold(&ACCOUNT_DELEGATOR_1), 0);
            assert_eq!(
                Joining::hold(&ACCOUNT_CANDIDATE_1, &ACCOUNT_DELEGATOR_1).0,
                0
            );
            assert_eq!(
                Joining::computed_stake(&ACCOUNT_CANDIDATE_1, &ACCOUNT_DELEGATOR_1)
                    .unwrap()
                    .0,
                0
            );
            assert_eq!(
                Candidates::<Runtime>::total_stake(&ACCOUNT_CANDIDATE_1).0,
                0
            );

            assert_eq_events!(Vec::<Event<Runtime>>::new());
        })
    }
);

pool_test!(
    fn delegation_request<P>() {
        ExtBuilder::default().build().execute_with(|| {
            let amount = 4949;
            do_request_delegation(
                ACCOUNT_CANDIDATE_1,
                ACCOUNT_DELEGATOR_1,
                P::target_pool(),
                amount,
            );

            assert_eq_events!(vec![
                Event::IncreasedStake {
                    candidate: ACCOUNT_CANDIDATE_1,
                    stake: amount,
                },
                Event::UpdatedCandidatePosition {
                    candidate: ACCOUNT_CANDIDATE_1,
                    stake: amount,
                    self_delegation: 0,
                    before: None,
                    after: None,
                },
                Event::RequestedDelegate {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_DELEGATOR_1,
                    towards: P::target_pool(),
                },
            ]);
        })
    }
);

pool_test!(
    fn delegation_execution<P>() {
        ExtBuilder::default().build().execute_with(|| {
            let final_amount = 2 * InitialManualClaimShareValue::get();
            let requested_amount = final_amount + 10; // test share rounding

            do_full_delegation::<P>(
                ACCOUNT_CANDIDATE_1,
                ACCOUNT_DELEGATOR_1,
                requested_amount,
                final_amount,
            );

            assert_eq_events!(vec![
                Event::IncreasedStake {
                    candidate: ACCOUNT_CANDIDATE_1,
                    stake: requested_amount,
                },
                Event::UpdatedCandidatePosition {
                    candidate: ACCOUNT_CANDIDATE_1,
                    stake: requested_amount,
                    self_delegation: 0,
                    before: None,
                    after: None,
                },
                Event::RequestedDelegate {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_DELEGATOR_1,
                    towards: P::target_pool(),
                },
                Event::DecreasedStake {
                    candidate: ACCOUNT_CANDIDATE_1,
                    stake: 10,
                },
                Event::UpdatedCandidatePosition {
                    candidate: ACCOUNT_CANDIDATE_1,
                    stake: final_amount,
                    self_delegation: 0,
                    before: None,
                    after: None,
                },
                P::event_staked(ACCOUNT_CANDIDATE_1, ACCOUNT_DELEGATOR_1, 2, final_amount)
            ]);
        })
    }
);

pool_test!(
    fn rebalance_increase<P>() {
        ExtBuilder::default().build().execute_with(|| {
            // Preparation:
            // We naturaly delegate towards a candidate.
            let initial_amount = 2 * InitialManualClaimShareValue::get();
            let rewards = 5 * KILO;
            let final_amount = initial_amount + rewards;

            do_full_delegation::<P>(
                ACCOUNT_CANDIDATE_1,
                ACCOUNT_DELEGATOR_1,
                initial_amount,
                initial_amount,
            );

            // We then artificialy distribute rewards by increasing the value of the pool
            // and minting currency to the staking account (this is not how manual rewards would
            // be distributed but whatever).
            assert_ok!(Balances::mint_into(&ACCOUNT_STAKING, rewards));
            assert_ok!(P::share_stake_among_holders(
                &ACCOUNT_CANDIDATE_1,
                Stake(rewards)
            ));
            assert_ok!(Candidates::<Runtime>::add_total_stake(
                &ACCOUNT_CANDIDATE_1,
                &Stake(rewards)
            ));
            assert_eq!(total_balance(&ACCOUNT_STAKING), DEFAULT_BALANCE + rewards);

            // Holds should not change but the computed stake should increase.
            assert_eq!(total_balance(&ACCOUNT_DELEGATOR_1), 1 * DEFAULT_BALANCE);
            assert_eq!(balance_hold(&ACCOUNT_DELEGATOR_1), initial_amount);
            assert_eq!(
                P::hold(&ACCOUNT_CANDIDATE_1, &ACCOUNT_DELEGATOR_1),
                Stake(initial_amount)
            );
            assert_eq!(
                P::shares(&ACCOUNT_CANDIDATE_1, &ACCOUNT_DELEGATOR_1),
                Shares(2)
            );
            assert_eq!(
                P::computed_stake(&ACCOUNT_CANDIDATE_1, &ACCOUNT_DELEGATOR_1)
                    .unwrap()
                    .0,
                final_amount
            );
            assert_eq!(
                Candidates::<Runtime>::total_stake(&ACCOUNT_CANDIDATE_1),
                Stake(final_amount)
            );

            // We perform the rebalancing and check it works.
            do_rebalance_hold::<P>(
                ACCOUNT_CANDIDATE_1,
                ACCOUNT_DELEGATOR_1,
                P::target_pool().into(),
                SignedBalance::Positive(rewards),
            );
        })
    }
);

pool_test!(
    fn rebalance_decrease<P>() {
        ExtBuilder::default().build().execute_with(|| {
            // Preparation:
            // We naturaly delegate towards a candidate.
            let block_number = block_number();
            let initial_amount = 2 * InitialManualClaimShareValue::get();
            let slash = 5 * KILO;
            let final_amount = initial_amount - slash;

            do_request_delegation(
                ACCOUNT_CANDIDATE_1,
                ACCOUNT_DELEGATOR_1,
                P::target_pool(),
                initial_amount,
            );

            // ---- Execution
            roll_to(block_number + 2);
            do_execute_delegation::<P>(
                ACCOUNT_CANDIDATE_1,
                ACCOUNT_DELEGATOR_1,
                block_number,
                initial_amount,
            );

            // We then artificialy slash by decreasing the value of the pool.
            assert_ok!(P::slash_stake_among_holders(
                &ACCOUNT_CANDIDATE_1,
                Stake(slash)
            ));
            assert_ok!(Candidates::<Runtime>::sub_total_stake(
                &ACCOUNT_CANDIDATE_1,
                Stake(slash)
            ));
            assert_eq!(total_balance(&ACCOUNT_STAKING), DEFAULT_BALANCE); // didn't change

            // Holds should not change but the computed stake should decrease.
            assert_eq!(total_balance(&ACCOUNT_DELEGATOR_1), 1 * DEFAULT_BALANCE);
            assert_eq!(balance_hold(&ACCOUNT_DELEGATOR_1), initial_amount);
            assert_eq!(
                P::hold(&ACCOUNT_CANDIDATE_1, &ACCOUNT_DELEGATOR_1),
                Stake(initial_amount)
            );
            assert_eq!(
                P::shares(&ACCOUNT_CANDIDATE_1, &ACCOUNT_DELEGATOR_1),
                Shares(2)
            );
            assert_eq!(
                P::computed_stake(&ACCOUNT_CANDIDATE_1, &ACCOUNT_DELEGATOR_1)
                    .unwrap()
                    .0,
                final_amount
            );
            assert_eq!(
                Candidates::<Runtime>::total_stake(&ACCOUNT_CANDIDATE_1),
                Stake(final_amount)
            );

            // We perform the rebalancing and check it works.
            do_rebalance_hold::<P>(
                ACCOUNT_CANDIDATE_1,
                ACCOUNT_DELEGATOR_1,
                P::target_pool().into(),
                SignedBalance::Negative(slash),
            );
        })
    }
);
