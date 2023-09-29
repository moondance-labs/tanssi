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

use {super::*, crate::assert_eq_last_events};

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
            RequestDelegation {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_DELEGATOR_1,
                pool: P::target_pool(),
                amount: amount + 1, // to test joining rounding
                expected_joining: amount,
            }
            .test();

            assert_eq_events!(vec![
                Event::IncreasedStake {
                    candidate: ACCOUNT_CANDIDATE_1,
                    stake_diff: amount,
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
                    pool: P::target_pool(),
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
            let final_amount = 2 * SHARE_INIT;
            let requested_amount = final_amount + 10; // test share rounding

            FullDelegation {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_DELEGATOR_1,
                request_amount: requested_amount,
                expected_increase: final_amount,
                ..default()
            }
            .test::<P>();

            assert_eq_events!(vec![
                Event::IncreasedStake {
                    candidate: ACCOUNT_CANDIDATE_1,
                    stake_diff: requested_amount,
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
                    pool: P::target_pool(),
                    pending: requested_amount,
                },
                Event::DecreasedStake {
                    candidate: ACCOUNT_CANDIDATE_1,
                    stake_diff: 10,
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
                    pool: P::target_pool(),
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
            let final_amount = 2 * SHARE_INIT;
            let block_number = block_number();

            RequestDelegation {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_DELEGATOR_1,
                pool: P::target_pool(),
                amount: final_amount,
                expected_joining: final_amount,
            }
            .test();
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
            let final_amount = 2 * SHARE_INIT;
            let leaving_amount = round_down(final_amount, 3); // test leaving rounding

            FullDelegation {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_DELEGATOR_1,
                request_amount: final_amount,
                expected_increase: final_amount,
                ..default()
            }
            .test::<P>();

            let block_number = block_number();

            RequestUndelegation {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_DELEGATOR_1,
                request_amount: SharesOrStake::Stake(final_amount),
                expected_removed: final_amount,
                expected_leaving: leaving_amount,
                ..default()
            }
            .test::<P>();

            roll_to(block_number + BLOCKS_TO_WAIT - 1); // too soon
            assert_noop!(
                Staking::execute_pending_operations(
                    RuntimeOrigin::signed(ACCOUNT_DELEGATOR_1),
                    vec![PendingOperationQuery {
                        delegator: ACCOUNT_DELEGATOR_1,
                        operation: PendingOperationKey::Leaving {
                            candidate: ACCOUNT_CANDIDATE_1,
                            at: block_number,
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
            let final_amount = 2 * SHARE_INIT;
            let requested_amount = final_amount + 10; // test share rounding
            let leaving_amount = round_down(final_amount, 3); // test leaving rounding

            assert_eq!(leaving_amount, 1_999_998);

            FullDelegation {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_DELEGATOR_1,
                request_amount: requested_amount,
                expected_increase: final_amount,
                ..default()
            }
            .test::<P>();

            FullUndelegation {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_DELEGATOR_1,
                request_amount: SharesOrStake::Stake(final_amount),
                expected_removed: final_amount,
                expected_leaving: leaving_amount,
                ..default()
            }
            .test::<P>();

            assert_eq_events!(vec![
                // delegate request
                Event::IncreasedStake {
                    candidate: ACCOUNT_CANDIDATE_1,
                    stake_diff: requested_amount,
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
                    pool: P::target_pool(),
                    pending: requested_amount
                },
                // delegate exec
                Event::DecreasedStake {
                    candidate: ACCOUNT_CANDIDATE_1,
                    stake_diff: 10,
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
                    pool: P::target_pool(),
                    staked: final_amount,
                    released: 10,
                },
                // undelegate request
                Event::DecreasedStake {
                    candidate: ACCOUNT_CANDIDATE_1,
                    stake_diff: final_amount,
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
    fn undelegation_execution_amount_in_shares<P>() {
        ExtBuilder::default().build().execute_with(|| {
            let joining_amount = 2 * SHARE_INIT;
            let joining_requested_amount = joining_amount + 10; // test share rounding

            let leaving_requested_amount = SHARE_INIT;
            let leaving_amount = round_down(leaving_requested_amount, 3); // test leaving rounding

            assert_eq!(leaving_amount, 999_999);

            FullDelegation {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_DELEGATOR_1,
                request_amount: joining_requested_amount,
                expected_increase: joining_amount,
                ..default()
            }
            .test::<P>();

            FullUndelegation {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_DELEGATOR_1,
                request_amount: SharesOrStake::Shares(1),
                expected_removed: leaving_requested_amount,
                expected_leaving: leaving_amount,
                ..default()
            }
            .test::<P>();

            assert_eq_events!(vec![
                // delegate request
                Event::IncreasedStake {
                    candidate: ACCOUNT_CANDIDATE_1,
                    stake_diff: joining_requested_amount,
                },
                Event::UpdatedCandidatePosition {
                    candidate: ACCOUNT_CANDIDATE_1,
                    stake: joining_requested_amount,
                    self_delegation: 0,
                    before: None,
                    after: None,
                },
                Event::RequestedDelegate {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_DELEGATOR_1,
                    pool: P::target_pool(),
                    pending: joining_requested_amount
                },
                // delegate exec
                Event::DecreasedStake {
                    candidate: ACCOUNT_CANDIDATE_1,
                    stake_diff: 10,
                },
                Event::UpdatedCandidatePosition {
                    candidate: ACCOUNT_CANDIDATE_1,
                    stake: joining_amount,
                    self_delegation: 0,
                    before: None,
                    after: None,
                },
                P::event_staked(ACCOUNT_CANDIDATE_1, ACCOUNT_DELEGATOR_1, 2, joining_amount),
                Event::ExecutedDelegate {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_DELEGATOR_1,
                    pool: P::target_pool(),
                    staked: joining_amount,
                    released: 10,
                },
                // undelegate request
                Event::DecreasedStake {
                    candidate: ACCOUNT_CANDIDATE_1,
                    stake_diff: leaving_requested_amount,
                },
                Event::UpdatedCandidatePosition {
                    candidate: ACCOUNT_CANDIDATE_1,
                    stake: joining_amount - leaving_requested_amount,
                    self_delegation: 0,
                    before: None,
                    after: None,
                },
                Event::RequestedUndelegate {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_DELEGATOR_1,
                    from: P::target_pool(),
                    pending: leaving_amount,
                    released: 1
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
    fn swap_works<P>() {
        ExtBuilder::default().build().execute_with(|| {
            FullDelegation {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_DELEGATOR_1,
                request_amount: 10 * SHARE_INIT,
                expected_increase: 10 * SHARE_INIT,
                ..default()
            }
            .test::<P>();

            Swap {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_DELEGATOR_1,
                requested_amount: SharesOrStake::Stake(5 * SHARE_INIT + 10),
                expected_removed: 5 * SHARE_INIT,
                expected_restaked: 5 * SHARE_INIT,
                ..default()
            }
            .test::<P>();

            assert_eq_last_events!(vec![Event::<Runtime>::SwappedPool {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_DELEGATOR_1,
                source_pool: P::target_pool(),
                source_shares: 5,
                source_stake: 5 * SHARE_INIT,
                target_shares: 5,
                target_stake: 5 * SHARE_INIT,
                pending_leaving: 0,
                released: 0,
            }]);
        })
    }
);

pool_test!(
    fn swap_too_much<P>() {
        ExtBuilder::default().build().execute_with(|| {
            FullDelegation {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_DELEGATOR_1,
                request_amount: 10 * SHARE_INIT,
                expected_increase: 10 * SHARE_INIT,
                ..default()
            }
            .test::<P>();

            assert_noop!(
                Staking::swap_pool(
                    RuntimeOrigin::signed(ACCOUNT_DELEGATOR_1),
                    ACCOUNT_CANDIDATE_1,
                    P::target_pool(),
                    SharesOrStake::Shares(11),
                ),
                Error::<Runtime>::MathUnderflow
            );
        })
    }
);

pool_test!(
    fn swap_with_rounding<P>() {
        ExtBuilder::default().build().execute_with(|| {
            FullDelegation {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_DELEGATOR_1,
                request_amount: 10 * SHARE_INIT,
                expected_increase: 10 * SHARE_INIT,
                ..default()
            }
            .test::<P>();

            FullDelegation {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_DELEGATOR_1,
                request_amount: 1 * SHARE_INIT,
                expected_increase: 1 * SHARE_INIT,
                ..default()
            }
            .test::<P::OppositePool>();

            // We then artificialy distribute rewards to the target by increasing the value of the pool
            // and minting currency to the staking account (this is not how manual rewards would
            // be distributed but whatever).
            let rewards = 5 * KILO;
            assert_ok!(Balances::mint_into(&ACCOUNT_STAKING, rewards));
            assert_ok!(P::OppositePool::share_stake_among_holders(
                &ACCOUNT_CANDIDATE_1,
                Stake(rewards)
            ));
            assert_ok!(Candidates::<Runtime>::add_total_stake(
                &ACCOUNT_CANDIDATE_1,
                &Stake(rewards)
            ));

            Swap {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_DELEGATOR_1,
                requested_amount: SharesOrStake::Stake(5 * SHARE_INIT + 10),
                expected_removed: 5 * SHARE_INIT,
                // due to 1 target share now being worth a bit more than SHARE_INIT,
                // only 4 target shares can be restaked
                expected_restaked: 4_020_000,
                // remaining amount is put in the leaving pool, rounded down
                // to the closest multiple of 3 (test leaving share init value)
                expected_leaving: 979_998,
                // thus the 2 stake that could not be put in the leaving pool
                // are directly released
                expected_released: 2,
                ..default()
            }
            .test::<P>();

            assert_eq_last_events!(vec![Event::<Runtime>::SwappedPool {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_DELEGATOR_1,
                source_pool: P::target_pool(),
                source_shares: 5,
                source_stake: 5 * SHARE_INIT,
                target_shares: 4,
                target_stake: 4_020_000,
                pending_leaving: 979_998,
                released: 2,
            }]);
        })
    }
);
