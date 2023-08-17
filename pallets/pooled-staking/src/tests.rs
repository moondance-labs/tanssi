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
        pool_test,
        pools::{self, Pool},
        Error, Event, PendingOperationQuery, Shares, Stake,
    },
    frame_support::{assert_noop, assert_ok, traits::tokens::fungible::Mutate},
};

type Joining = pools::Joining<Runtime>;

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
            assert_ok!(Staking::request_delegate(
                RuntimeOrigin::signed(ACCOUNT_DELEGATOR_1),
                ACCOUNT_CANDIDATE_1,
                P::target_pool(),
                amount,
            ),);

            // Actual balances don't change
            assert_eq!(total_balance(&ACCOUNT_DELEGATOR_1), 1 * DEFAULT_BALANCE);
            assert_eq!(total_balance(&ACCOUNT_STAKING), DEFAULT_BALANCE);
            // However funds are held
            assert_eq!(balance_hold(&ACCOUNT_DELEGATOR_1), amount);
            assert_eq!(
                Joining::hold(&ACCOUNT_CANDIDATE_1, &ACCOUNT_DELEGATOR_1).0,
                amount
            );
            assert_eq!(
                Joining::computed_stake(&ACCOUNT_CANDIDATE_1, &ACCOUNT_DELEGATOR_1)
                    .unwrap()
                    .0,
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
                    towards: P::target_pool(),
                },
            ]);
        })
    }
);

pool_test!(
    fn delegation_execution<P>() {
        ExtBuilder::default().build().execute_with(|| {
            let block_number = block_number();
            let final_amount = 2 * InitialManualClaimShareValue::get();
            let requested_amount = final_amount + 10;
            // test share rounding
            assert_ok!(Staking::request_delegate(
                RuntimeOrigin::signed(ACCOUNT_DELEGATOR_1),
                ACCOUNT_CANDIDATE_1,
                P::target_pool(),
                requested_amount,
            ),);

            // Actual balances don't change
            assert_eq!(total_balance(&ACCOUNT_DELEGATOR_1), 1 * DEFAULT_BALANCE);
            assert_eq!(total_balance(&ACCOUNT_STAKING), DEFAULT_BALANCE);
            // However funds are held
            assert_eq!(balance_hold(&ACCOUNT_DELEGATOR_1), requested_amount);
            assert_eq!(
                Joining::hold(&ACCOUNT_CANDIDATE_1, &ACCOUNT_DELEGATOR_1),
                Stake(requested_amount)
            );
            assert_eq!(
                P::hold(&ACCOUNT_CANDIDATE_1, &ACCOUNT_DELEGATOR_1),
                Stake(0)
            );
            assert_eq!(
                Candidates::<Runtime>::total_stake(&ACCOUNT_CANDIDATE_1),
                Stake(requested_amount)
            );

            // ---- Execution
            roll_to(block_number + 2);
            assert_ok!(Staking::execute_pending_operations(
                RuntimeOrigin::signed(ACCOUNT_DELEGATOR_1),
                vec![PendingOperationQuery {
                    delegator: ACCOUNT_DELEGATOR_1,
                    operation: P::joining_operation_key(ACCOUNT_CANDIDATE_1, block_number,)
                }]
            ),);

            // Actual balances don't change
            assert_eq!(total_balance(&ACCOUNT_DELEGATOR_1), 1 * DEFAULT_BALANCE);
            assert_eq!(total_balance(&ACCOUNT_STAKING), DEFAULT_BALANCE);
            // However funds are held (with share rounding released)
            assert_eq!(balance_hold(&ACCOUNT_DELEGATOR_1), final_amount);
            assert_eq!(
                Joining::hold(&ACCOUNT_CANDIDATE_1, &ACCOUNT_DELEGATOR_1),
                Stake(0)
            );
            assert_eq!(
                P::hold(&ACCOUNT_CANDIDATE_1, &ACCOUNT_DELEGATOR_1),
                Stake(final_amount)
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
            let block_number = block_number();
            let initial_amount = 2 * InitialManualClaimShareValue::get();
            let rewards = 5 * KILO;
            let final_amount = initial_amount + rewards;

            assert_ok!(Staking::request_delegate(
                RuntimeOrigin::signed(ACCOUNT_DELEGATOR_1),
                ACCOUNT_CANDIDATE_1,
                P::target_pool(),
                initial_amount,
            ));
            roll_to(block_number + 2);
            assert_ok!(Staking::execute_pending_operations(
                RuntimeOrigin::signed(ACCOUNT_DELEGATOR_1),
                vec![PendingOperationQuery {
                    delegator: ACCOUNT_DELEGATOR_1,
                    operation: P::joining_operation_key(ACCOUNT_CANDIDATE_1, block_number,)
                }]
            ),);

            // Pre-check
            assert_eq!(total_balance(&ACCOUNT_DELEGATOR_1), 1 * DEFAULT_BALANCE);
            assert_eq!(total_balance(&ACCOUNT_STAKING), DEFAULT_BALANCE);
            assert_eq!(balance_hold(&ACCOUNT_DELEGATOR_1), initial_amount);
            assert_eq!(
                Joining::hold(&ACCOUNT_CANDIDATE_1, &ACCOUNT_DELEGATOR_1),
                Stake(0)
            );
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
                initial_amount
            );
            assert_eq!(
                Candidates::<Runtime>::total_stake(&ACCOUNT_CANDIDATE_1),
                Stake(initial_amount)
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
            assert_ok!(Staking::rebalance_hold(
                RuntimeOrigin::signed(ACCOUNT_DELEGATOR_1),
                ACCOUNT_CANDIDATE_1,
                ACCOUNT_DELEGATOR_1,
                P::target_pool().into()
            ));
            assert_eq!(total_balance(&ACCOUNT_STAKING), DEFAULT_BALANCE,);
            assert_eq!(
                total_balance(&ACCOUNT_DELEGATOR_1),
                DEFAULT_BALANCE + rewards
            );
            assert_eq!(balance_hold(&ACCOUNT_DELEGATOR_1), initial_amount + rewards);
            assert_eq!(
                P::hold(&ACCOUNT_CANDIDATE_1, &ACCOUNT_DELEGATOR_1),
                Stake(initial_amount + rewards)
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

            assert_ok!(Staking::request_delegate(
                RuntimeOrigin::signed(ACCOUNT_DELEGATOR_1),
                ACCOUNT_CANDIDATE_1,
                P::target_pool(),
                initial_amount,
            ));
            roll_to(block_number + 2);
            assert_ok!(Staking::execute_pending_operations(
                RuntimeOrigin::signed(ACCOUNT_DELEGATOR_1),
                vec![PendingOperationQuery {
                    delegator: ACCOUNT_DELEGATOR_1,
                    operation: P::joining_operation_key(ACCOUNT_CANDIDATE_1, block_number,)
                }]
            ),);

            // Pre-check
            assert_eq!(total_balance(&ACCOUNT_DELEGATOR_1), 1 * DEFAULT_BALANCE);
            assert_eq!(total_balance(&ACCOUNT_STAKING), DEFAULT_BALANCE);
            assert_eq!(balance_hold(&ACCOUNT_DELEGATOR_1), initial_amount);
            assert_eq!(
                Joining::hold(&ACCOUNT_CANDIDATE_1, &ACCOUNT_DELEGATOR_1),
                Stake(0)
            );
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
                initial_amount
            );
            assert_eq!(
                Candidates::<Runtime>::total_stake(&ACCOUNT_CANDIDATE_1),
                Stake(initial_amount)
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
            assert_ok!(Staking::rebalance_hold(
                RuntimeOrigin::signed(ACCOUNT_DELEGATOR_1),
                ACCOUNT_CANDIDATE_1,
                ACCOUNT_DELEGATOR_1,
                P::target_pool().into()
            ));
            assert_eq!(total_balance(&ACCOUNT_STAKING), DEFAULT_BALANCE + slash,);
            assert_eq!(total_balance(&ACCOUNT_DELEGATOR_1), DEFAULT_BALANCE - slash);
            assert_eq!(balance_hold(&ACCOUNT_DELEGATOR_1), initial_amount - slash);
            assert_eq!(
                P::hold(&ACCOUNT_CANDIDATE_1, &ACCOUNT_DELEGATOR_1),
                Stake(initial_amount - slash)
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
        })
    }
);
