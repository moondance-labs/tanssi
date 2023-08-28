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

use {
    crate::{
        assert_eq_events, assert_fields_eq,
        candidate::Candidates,
        mock::*,
        pool_test,
        pools::{self, Pool},
        AllTargetPool, Error, Event, PendingOperationKey, PendingOperationQuery, PendingOperations,
        Shares, SharesOrStake, Stake, TargetPool,
    },
    frame_support::{assert_noop, assert_ok, traits::tokens::fungible::Mutate},
    sp_runtime::TokenError,
};

type Joining = pools::Joining<Runtime>;
type Leaving = pools::Leaving<Runtime>;

fn operation_stake(
    candidate: AccountId,
    delegator: AccountId,
    pool: TargetPool,
    at_block: u64,
) -> Balance {
    let operation_key = match pool {
        TargetPool::AutoCompounding => PendingOperationKey::JoiningAutoCompounding {
            candidate: candidate.clone(),
            at_block,
        },
        TargetPool::ManualRewards => PendingOperationKey::JoiningManualRewards {
            candidate: candidate.clone(),
            at_block,
        },
    };

    let shares = PendingOperations::<Runtime>::get(&delegator, &operation_key);
    if shares == 0 {
        return 0;
    }

    Joining::shares_to_stake(&candidate, Shares(shares))
        .unwrap()
        .0
}

fn do_request_delegation(
    candidate: AccountId,
    delegator: AccountId,
    pool: TargetPool,
    amount: Balance,
    expected_joining: Balance,
) {
    let now = block_number();

    let before = State::extract(candidate, delegator);
    let pool_before = PoolState::extract::<Joining>(candidate, delegator);
    let operation_before = operation_stake(candidate, delegator, pool, now);

    assert_ok!(Staking::request_delegate(
        RuntimeOrigin::signed(delegator),
        candidate,
        pool,
        amount,
    ));

    let after = State::extract(candidate, delegator);
    let pool_after = PoolState::extract::<Joining>(candidate, delegator);
    let operation_after = operation_stake(candidate, delegator, pool, now);

    // Actual balances don't change
    assert_fields_eq!(before, after, [delegator_balance, staking_balance]);
    assert_eq!(
        before.delegator_hold + expected_joining,
        after.delegator_hold
    );
    assert_eq!(pool_before.hold + expected_joining, pool_after.hold);
    assert_eq!(pool_before.stake + expected_joining, pool_after.stake);
    assert_eq!(
        before.candidate_total_stake + expected_joining,
        after.candidate_total_stake
    );
    assert_eq!(operation_before + expected_joining, operation_after);
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
    let request_before =
        pools::Joining::<Runtime>::shares_to_stake(&candidate, Shares(request_before))
            .unwrap()
            .0;

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
    do_request_delegation(
        candidate,
        delegator,
        P::target_pool(),
        request_amount,
        round_down(request_amount, 2),
    );
    roll_to(block_number + BLOCKS_TO_WAIT);
    do_execute_delegation::<P>(
        ACCOUNT_CANDIDATE_1,
        ACCOUNT_DELEGATOR_1,
        block_number,
        expected_increase,
    );
}

fn do_request_undelegation<P: PoolExt<Runtime>>(
    candidate: AccountId,
    delegator: AccountId,
    request_amount: Balance,
    expected_removed: Balance,
    expected_leaving: Balance,
) {
    let dust = expected_removed
        .checked_sub(expected_leaving)
        .expect("should removed >= leaving");

    let before = State::extract(candidate, delegator);
    let pool_before = PoolState::extract::<P>(candidate, delegator);
    let leaving_before = PoolState::extract::<Leaving>(candidate, delegator);

    assert_ok!(Staking::request_undelegate(
        RuntimeOrigin::signed(delegator),
        candidate,
        P::target_pool(),
        SharesOrStake::Stake(request_amount),
    ));

    let after = State::extract(candidate, delegator);
    let pool_after = PoolState::extract::<P>(candidate, delegator);
    let leaving_after = PoolState::extract::<Leaving>(candidate, delegator);

    // Actual balances don't change
    assert_fields_eq!(before, after, [delegator_balance, staking_balance]);
    // Dust is released immediately.
    assert_eq!(before.delegator_hold - dust, after.delegator_hold);
    // Pool decrease.
    assert_eq!(pool_before.stake - expected_removed, pool_after.stake);
    assert_eq!(pool_before.hold - expected_removed, pool_after.stake);
    // Leaving increase.
    assert_eq!(leaving_before.stake + expected_leaving, leaving_after.stake);
    assert_eq!(leaving_before.hold + expected_leaving, leaving_after.stake);
    // Stake no longer contribute to election
    assert_eq!(
        before.candidate_total_stake - expected_removed,
        after.candidate_total_stake
    );
}

fn do_execute_undelegation(
    candidate: AccountId,
    delegator: AccountId,
    block_number: u64,
    expected_decrease: Balance,
) {
    let before = State::extract(candidate, delegator);
    let leaving_before = PoolState::extract::<Leaving>(candidate, delegator);

    assert_ok!(Staking::execute_pending_operations(
        RuntimeOrigin::signed(delegator),
        vec![PendingOperationQuery {
            delegator: delegator,
            operation: PendingOperationKey::Leaving {
                candidate,
                at_block: block_number
            }
        }]
    ));

    let after = State::extract(candidate, delegator);
    let leaving_after = PoolState::extract::<Joining>(candidate, delegator);
    let request_after = crate::PendingOperations::<Runtime>::get(
        delegator,
        PendingOperationKey::Leaving {
            candidate,
            at_block: block_number,
        },
    );

    // Actual balances don't change
    assert_fields_eq!(before, after, [delegator_balance, staking_balance]);
    assert_eq!(request_after, 0);
    assert_eq!(
        before.delegator_hold - expected_decrease,
        after.delegator_hold
    );
    assert_fields_eq!(before, after, candidate_total_stake);
    assert_eq!(leaving_before.hold - expected_decrease, leaving_after.hold);
    assert_eq!(
        leaving_before.stake - expected_decrease,
        leaving_after.stake
    );
}

fn do_full_undelegation<P: PoolExt<Runtime>>(
    candidate: AccountId,
    delegator: AccountId,
    request_amount: Balance,
    expected_removed: Balance,
    expected_leaving: Balance,
) {
    let block_number = block_number();
    do_request_undelegation::<P>(
        candidate,
        delegator,
        request_amount,
        expected_removed,
        expected_leaving,
    );
    roll_to(block_number + BLOCKS_TO_WAIT);
    do_execute_undelegation(candidate, delegator, block_number, expected_leaving);
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
            let before = State::extract(ACCOUNT_CANDIDATE_1, ACCOUNT_DELEGATOR_1);
            let pool_before =
                PoolState::extract::<Joining>(ACCOUNT_CANDIDATE_1, ACCOUNT_DELEGATOR_1);

            assert_noop!(
                Staking::request_delegate(
                    RuntimeOrigin::signed(ACCOUNT_DELEGATOR_1),
                    ACCOUNT_CANDIDATE_1,
                    P::target_pool(),
                    0
                ),
                Error::<Runtime>::StakeMustBeNonZero
            );

            let after = State::extract(ACCOUNT_CANDIDATE_1, ACCOUNT_DELEGATOR_1);
            let pool_after =
                PoolState::extract::<Joining>(ACCOUNT_CANDIDATE_1, ACCOUNT_DELEGATOR_1);

            assert_eq!(before, after);
            assert_eq!(pool_before, pool_after);

            assert_eq_events!(Vec::<Event<Runtime>>::new());
        })
    }
);

pool_test!(
    fn delegation_request<P>() {
        ExtBuilder::default().build().execute_with(|| {
            let amount = 3324;
            do_request_delegation(
                ACCOUNT_CANDIDATE_1,
                ACCOUNT_DELEGATOR_1,
                P::target_pool(),
                amount + 1, // to test joining rounding
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
                    pending: amount
                },
            ]);
        })
    }
);

pool_test!(
    fn delegation_request_more_than_available<P>() {
        ExtBuilder::default().build().execute_with(|| {
            let amount = DEFAULT_BALANCE; // not enough to keep ED

            let before = State::extract(ACCOUNT_CANDIDATE_1, ACCOUNT_DELEGATOR_1);
            let pool_before =
                PoolState::extract::<Joining>(ACCOUNT_CANDIDATE_1, ACCOUNT_DELEGATOR_1);

            assert_noop!(
                Staking::request_delegate(
                    RuntimeOrigin::signed(ACCOUNT_DELEGATOR_1),
                    ACCOUNT_CANDIDATE_1,
                    P::target_pool(),
                    amount,
                ),
                TokenError::FundsUnavailable
            );

            let after = State::extract(ACCOUNT_CANDIDATE_1, ACCOUNT_DELEGATOR_1);
            let pool_after =
                PoolState::extract::<Joining>(ACCOUNT_CANDIDATE_1, ACCOUNT_DELEGATOR_1);

            assert_eq!(before, after);
            assert_eq!(pool_before, pool_after);

            assert_eq_events!(Vec::<Event<Runtime>>::new());
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
                    pending: requested_amount,
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
                P::event_staked(ACCOUNT_CANDIDATE_1, ACCOUNT_DELEGATOR_1, 2, final_amount),
                Event::ExecutedDelegate {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_DELEGATOR_1,
                    towards: P::target_pool(),
                    staked: final_amount,
                    released: 10,
                },
            ]);
        })
    }
);

pool_test!(
    fn delegation_execution_too_soon<P>() {
        ExtBuilder::default().build().execute_with(|| {
            let final_amount = 2 * InitialManualClaimShareValue::get();
            let block_number = block_number();
            do_request_delegation(
                ACCOUNT_CANDIDATE_1,
                ACCOUNT_DELEGATOR_1,
                P::target_pool(),
                final_amount,
                final_amount,
            );
            roll_to(block_number + BLOCKS_TO_WAIT - 1); // too soon

            assert_noop!(
                Staking::execute_pending_operations(
                    RuntimeOrigin::signed(ACCOUNT_DELEGATOR_1),
                    vec![PendingOperationQuery {
                        delegator: ACCOUNT_DELEGATOR_1,
                        operation: P::joining_operation_key(ACCOUNT_CANDIDATE_1, block_number)
                    }]
                ),
                Error::<Runtime>::RequestCannotBeExecuted(0)
            );
        })
    }
);

pool_test!(
    fn undelegation_execution_too_soon<P>() {
        ExtBuilder::default().build().execute_with(|| {
            let final_amount = 2 * InitialManualClaimShareValue::get();
            let leaving_amount = round_down(final_amount, 3); // test leaving rounding

            do_full_delegation::<P>(
                ACCOUNT_CANDIDATE_1,
                ACCOUNT_DELEGATOR_1,
                final_amount,
                final_amount,
            );

            let block_number = block_number();
            do_request_undelegation::<P>(
                ACCOUNT_CANDIDATE_1,
                ACCOUNT_DELEGATOR_1,
                final_amount,
                final_amount,
                leaving_amount,
            );

            roll_to(block_number + BLOCKS_TO_WAIT - 1); // too soon
            assert_noop!(
                Staking::execute_pending_operations(
                    RuntimeOrigin::signed(ACCOUNT_DELEGATOR_1),
                    vec![PendingOperationQuery {
                        delegator: ACCOUNT_DELEGATOR_1,
                        operation: PendingOperationKey::Leaving {
                            candidate: ACCOUNT_CANDIDATE_1,
                            at_block: block_number,
                        }
                    }]
                ),
                Error::<Runtime>::RequestCannotBeExecuted(0)
            );
        })
    }
);

pool_test!(
    fn undelegation_execution<P>() {
        ExtBuilder::default().build().execute_with(|| {
            let final_amount = 2 * InitialManualClaimShareValue::get();
            let requested_amount = final_amount + 10; // test share rounding
            let leaving_amount = round_down(final_amount, 3); // test leaving rounding

            assert_eq!(leaving_amount, 1_999_998);

            do_full_delegation::<P>(
                ACCOUNT_CANDIDATE_1,
                ACCOUNT_DELEGATOR_1,
                requested_amount,
                final_amount,
            );

            do_full_undelegation::<P>(
                ACCOUNT_CANDIDATE_1,
                ACCOUNT_DELEGATOR_1,
                final_amount,
                final_amount,
                leaving_amount,
            );

            assert_eq_events!(vec![
                // delegate request
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
                    pending: requested_amount
                },
                // delegate exec
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
                P::event_staked(ACCOUNT_CANDIDATE_1, ACCOUNT_DELEGATOR_1, 2, final_amount),
                Event::ExecutedDelegate {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_DELEGATOR_1,
                    towards: P::target_pool(),
                    staked: final_amount,
                    released: 10,
                },
                // undelegate request
                Event::DecreasedStake {
                    candidate: ACCOUNT_CANDIDATE_1,
                    stake: final_amount,
                },
                Event::UpdatedCandidatePosition {
                    candidate: ACCOUNT_CANDIDATE_1,
                    stake: 0,
                    self_delegation: 0,
                    before: None,
                    after: None,
                },
                Event::RequestedUndelegate {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_DELEGATOR_1,
                    from: P::target_pool(),
                    pending: leaving_amount,
                    released: 2
                },
                // undelegate exec
                Event::ExecutedUndelegate {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_DELEGATOR_1,
                    released: leaving_amount,
                },
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
            let initial_amount = 2 * InitialManualClaimShareValue::get();
            let slash = 5 * KILO;
            let final_amount = initial_amount - slash;

            do_full_delegation::<P>(
                ACCOUNT_CANDIDATE_1,
                ACCOUNT_DELEGATOR_1,
                initial_amount,
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
