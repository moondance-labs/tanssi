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

mod delegator_flow;
mod manual_rewards;
mod rebalance;

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

pub type Joining = pools::Joining<Runtime>;
pub type Leaving = pools::Leaving<Runtime>;

pub(crate) fn operation_stake(
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

pub(crate) fn do_request_delegation(
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

pub(crate) fn do_execute_delegation<P: PoolExt<Runtime>>(
    candidate: AccountId,
    delegator: AccountId,
    block_number: u64,
    expected_increase: Balance,
    expected_manual_rewards: Balance,
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

    // Actual balances changes only due to manuyal rewards.
    assert_eq!(
        before.delegator_balance + expected_manual_rewards,
        after.delegator_balance
    );
    assert_eq!(
        before.staking_balance - expected_manual_rewards,
        after.staking_balance
    );
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

pub(crate) fn do_full_delegation<P: PoolExt<Runtime>>(
    candidate: AccountId,
    delegator: AccountId,
    request_amount: Balance,
    expected_increase: Balance,
    expected_manual_rewards: Balance,
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
        expected_manual_rewards,
    );
}

pub(crate) fn do_request_undelegation<P: PoolExt<Runtime>>(
    candidate: AccountId,
    delegator: AccountId,
    request_amount: SharesOrStake<Balance>,
    expected_removed: Balance,
    expected_leaving: Balance,
    expected_manual_rewards: Balance,
    expected_hold_rebalance: Balance,
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
        request_amount,
    ));

    let after = State::extract(candidate, delegator);
    let pool_after = PoolState::extract::<P>(candidate, delegator);
    let leaving_after = PoolState::extract::<Leaving>(candidate, delegator);

    // Actual balances changes due to manual rewards and hold rebalance.
    assert_eq!(
        before.delegator_balance + expected_manual_rewards + expected_hold_rebalance,
        after.delegator_balance
    );
    assert_eq!(
        before.staking_balance - expected_manual_rewards - expected_hold_rebalance,
        after.staking_balance
    );
    // Dust is released immediately.
    assert_eq!(
        before.delegator_hold - dust + expected_hold_rebalance,
        after.delegator_hold
    );
    // Pool decrease.
    assert_eq!(pool_before.stake - expected_removed, pool_after.stake);
    assert_eq!(
        pool_before.hold + expected_hold_rebalance - expected_removed,
        pool_after.stake
    );
    // Leaving increase.
    assert_eq!(leaving_before.stake + expected_leaving, leaving_after.stake);
    assert_eq!(leaving_before.hold + expected_leaving, leaving_after.stake);
    // Stake no longer contribute to election
    assert_eq!(
        before.candidate_total_stake - expected_removed,
        after.candidate_total_stake
    );
}

pub(crate) fn do_execute_undelegation(
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

pub(crate) fn do_full_undelegation<P: PoolExt<Runtime>>(
    candidate: AccountId,
    delegator: AccountId,
    request_amount: SharesOrStake<Balance>,
    expected_removed: Balance,
    expected_leaving: Balance,
    expected_hold_rebalance: Balance,
) {
    let block_number = block_number();
    do_request_undelegation::<P>(
        candidate,
        delegator,
        request_amount,
        expected_removed,
        expected_leaving,
        0,
        expected_hold_rebalance,
    );
    roll_to(block_number + BLOCKS_TO_WAIT);
    do_execute_undelegation(candidate, delegator, block_number, expected_leaving);
}

pub(crate) fn do_rebalance_hold<P: Pool<Runtime>>(
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
