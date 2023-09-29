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

use super::*;

pool_test!(
    fn rebalance_increase<P>() {
        ExtBuilder::default().build().execute_with(|| {
            // Preparation:
            // We naturaly delegate towards a candidate.
            let initial_amount = 2 * SHARE_INIT;
            let rewards = 5 * KILO;
            let final_amount = initial_amount + rewards;

            FullDelegation {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_DELEGATOR_1,
                request_amount: initial_amount,
                expected_increase: initial_amount,
                ..default()
            }
            .test::<P>();

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
            let initial_amount = 2 * SHARE_INIT;
            let slash = 5 * KILO;
            let final_amount = initial_amount - slash;

            FullDelegation {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_DELEGATOR_1,
                request_amount: initial_amount,
                expected_increase: initial_amount,
                ..default()
            }
            .test::<P>();

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

pool_test!(
    fn rebalance_noop<P>() {
        ExtBuilder::default().build().execute_with(|| {
            // Preparation:
            // We naturaly delegate towards a candidate.
            let initial_amount = 2 * SHARE_INIT;

            FullDelegation {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_DELEGATOR_1,
                request_amount: initial_amount,
                expected_increase: initial_amount,
                ..default()
            }
            .test::<P>();

            // We perform the rebalancing and check nothing happen.
            do_rebalance_hold::<P>(
                ACCOUNT_CANDIDATE_1,
                ACCOUNT_DELEGATOR_1,
                P::target_pool().into(),
                SignedBalance::Positive(0),
            );
        })
    }
);

pool_test!(
    fn rebalance_in_undelegation_request<P>() {
        ExtBuilder::default().build().execute_with(|| {
            let joining_amount = 2 * SHARE_INIT;
            let rewards = 5 * KILO;
            let leaving_requested_amount = joining_amount + rewards;
            let leaving_amount = round_down(leaving_requested_amount, 3); // test leaving rounding

            FullDelegation {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_DELEGATOR_1,
                request_amount: joining_amount,
                expected_increase: joining_amount,
                ..default()
            }
            .test::<P>();

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

            // We then do the undelegation
            RequestUndelegation {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_DELEGATOR_1,
                request_amount: SharesOrStake::Stake(leaving_requested_amount),
                expected_removed: leaving_requested_amount,
                expected_leaving: leaving_amount,
                expected_hold_rebalance: rewards,
                ..default()
            }
            .test::<P>();
        })
    }
);

pool_test!(
    fn rebalance_in_swap<P>() {
        ExtBuilder::default().build().execute_with(|| {
            FullDelegation {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_DELEGATOR_1,
                request_amount: 10 * SHARE_INIT,
                expected_increase: 10 * SHARE_INIT,
                ..default()
            }
            .test::<P>();

            // We then artificialy distribute rewards to the source pool by increasing the value of the pool
            // and minting currency to the staking account (this is not how manual rewards would
            // be distributed but whatever).
            let rewards = 2 * SHARE_INIT;
            assert_ok!(Balances::mint_into(&ACCOUNT_STAKING, rewards));
            assert_ok!(P::share_stake_among_holders(
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
                requested_amount: SharesOrStake::Shares(9),
                expected_removed: 10_800_000,
                expected_restaked: 10_000_000,
                expected_leaving: 799998,
                expected_released: 2,
                expected_hold_rebalance: rewards,
            }
            .test::<P>();
        })
    }
);
