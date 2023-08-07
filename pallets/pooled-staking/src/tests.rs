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
        assert_eq_events,
        candidate::Candidates,
        mock::*,
        pools::{self, Pool},
        Error, Event, PendingOperationKey, PendingOperationQuery, Shares, Stake, TargetPool,
    },
    frame_support::{assert_noop, assert_ok},
};

#[test]
fn manual_rewards_empty_delegation() {
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(
            Staking::request_delegate(
                RuntimeOrigin::signed(ACCOUNT_DELEGATOR_1),
                ACCOUNT_CANDIDATE_1,
                TargetPool::ManualRewards,
                0
            ),
            Error::<Runtime>::StakeMustBeNonZero
        );

        // No change
        assert_eq!(total_balance(&ACCOUNT_DELEGATOR_1), 1 * DEFAULT_BALANCE);
        assert_eq!(total_balance(&ACCOUNT_STAKING), 0);
        assert_eq!(balance_hold(&ACCOUNT_DELEGATOR_1), 0);
        assert_eq!(
            pools::Joining::<Runtime>::hold(&ACCOUNT_CANDIDATE_1, &ACCOUNT_DELEGATOR_1).0,
            0
        );
        assert_eq!(
            Candidates::<Runtime>::total_stake(&ACCOUNT_CANDIDATE_1).0,
            0
        );

        assert_eq_events!(Vec::<Event<Runtime>>::new());
    });
}

#[test]
fn manual_rewards_delegation_request() {
    ExtBuilder::default().build().execute_with(|| {
        let amount = 4949;
        assert_ok!(Staking::request_delegate(
            RuntimeOrigin::signed(ACCOUNT_DELEGATOR_1),
            ACCOUNT_CANDIDATE_1,
            TargetPool::ManualRewards,
            amount,
        ),);

        // Actual balances don't change
        assert_eq!(total_balance(&ACCOUNT_DELEGATOR_1), 1 * DEFAULT_BALANCE);
        assert_eq!(total_balance(&ACCOUNT_STAKING), 0);
        // However funds are held
        assert_eq!(balance_hold(&ACCOUNT_DELEGATOR_1), amount);
        assert_eq!(
            pools::Joining::<Runtime>::hold(&ACCOUNT_CANDIDATE_1, &ACCOUNT_DELEGATOR_1).0,
            amount
        );
        assert_eq!(
            Candidates::<Runtime>::total_stake(&ACCOUNT_CANDIDATE_1).0,
            amount
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
                towards: TargetPool::ManualRewards,
            },
        ]);
    });
}

#[test]
fn manual_rewards_delegation_execution() {
    ExtBuilder::default().build().execute_with(|| {
        let block_number = block_number();
        let amount = 2 * InitialManualClaimShareValue::get() + 10; // test share rounding
        assert_ok!(Staking::request_delegate(
            RuntimeOrigin::signed(ACCOUNT_DELEGATOR_1),
            ACCOUNT_CANDIDATE_1,
            TargetPool::ManualRewards,
            amount,
        ),);

        // Actual balances don't change
        assert_eq!(total_balance(&ACCOUNT_DELEGATOR_1), 1 * DEFAULT_BALANCE);
        assert_eq!(total_balance(&ACCOUNT_STAKING), 0);
        // However funds are held
        assert_eq!(balance_hold(&ACCOUNT_DELEGATOR_1), amount);
        assert_eq!(
            pools::Joining::<Runtime>::hold(&ACCOUNT_CANDIDATE_1, &ACCOUNT_DELEGATOR_1),
            Stake(amount)
        );
        assert_eq!(
            pools::ManualRewards::<Runtime>::hold(&ACCOUNT_CANDIDATE_1, &ACCOUNT_DELEGATOR_1),
            Stake(0)
        );
        assert_eq!(
            Candidates::<Runtime>::total_stake(&ACCOUNT_CANDIDATE_1),
            Stake(amount)
        );

        // ---- Execution
        roll_to(block_number + 2);
        assert_ok!(Staking::execute_pending_operations(
            RuntimeOrigin::signed(ACCOUNT_DELEGATOR_1),
            vec![PendingOperationQuery {
                delegator: ACCOUNT_DELEGATOR_1,
                operation: PendingOperationKey::JoiningManualRewards {
                    candidate: ACCOUNT_CANDIDATE_1,
                    at_block: block_number,
                }
            }]
        ),);

        // Actual balances don't change
        assert_eq!(total_balance(&ACCOUNT_DELEGATOR_1), 1 * DEFAULT_BALANCE);
        assert_eq!(total_balance(&ACCOUNT_STAKING), 0);
        // However funds are held (with share rounding released)
        assert_eq!(balance_hold(&ACCOUNT_DELEGATOR_1), amount - 10);
        assert_eq!(
            pools::Joining::<Runtime>::hold(&ACCOUNT_CANDIDATE_1, &ACCOUNT_DELEGATOR_1),
            Stake(0)
        );
        assert_eq!(
            pools::ManualRewards::<Runtime>::hold(&ACCOUNT_CANDIDATE_1, &ACCOUNT_DELEGATOR_1),
            Stake(amount - 10)
        );
        assert_eq!(
            pools::ManualRewards::<Runtime>::shares(&ACCOUNT_CANDIDATE_1, &ACCOUNT_DELEGATOR_1),
            Shares(2)
        );
        assert_eq!(
            Candidates::<Runtime>::total_stake(&ACCOUNT_CANDIDATE_1),
            Stake(amount - 10)
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
                towards: TargetPool::ManualRewards,
            },
            Event::DecreasedStake {
                candidate: ACCOUNT_CANDIDATE_1,
                stake: 10,
            },
            Event::UpdatedCandidatePosition {
                candidate: ACCOUNT_CANDIDATE_1,
                stake: amount - 10,
                self_delegation: 0,
                before: None,
                after: None,
            },
            Event::StakedManualRewards {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_DELEGATOR_1,
                shares: 2,
                stake: amount - 10
            }
        ]);
    });
}
