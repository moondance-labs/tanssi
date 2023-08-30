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
    fn self_delegation_below_minimum<P>() {
        ExtBuilder::default().build().execute_with(|| {
            let requested_amount = MinimumSelfDelegation::get() - 1;
            let final_amount = round_down(
                requested_amount,
                P::shares_to_stake_or_init(&ACCOUNT_CANDIDATE_1, Shares(1))
                    .unwrap()
                    .0,
            );

            FullDelegation {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_CANDIDATE_1,
                request_amount: requested_amount,
                expected_increase: final_amount,
                ..default()
            }
            .test::<P>();

            assert_eq_events!(vec![
                Event::IncreasedStake {
                    candidate: ACCOUNT_CANDIDATE_1,
                    stake_diff: round_down(requested_amount, 2),
                },
                Event::UpdatedCandidatePosition {
                    candidate: ACCOUNT_CANDIDATE_1,
                    stake: round_down(requested_amount, 2),
                    self_delegation: round_down(requested_amount, 2),
                    before: None,
                    after: None,
                },
                Event::RequestedDelegate {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_CANDIDATE_1,
                    towards: P::target_pool(),
                    pending: round_down(requested_amount, 2),
                },
                Event::DecreasedStake {
                    candidate: ACCOUNT_CANDIDATE_1,
                    stake_diff: round_down(requested_amount, 2) - final_amount,
                },
                Event::UpdatedCandidatePosition {
                    candidate: ACCOUNT_CANDIDATE_1,
                    stake: final_amount,
                    self_delegation: final_amount,
                    before: None,
                    after: None,
                },
                P::event_staked(ACCOUNT_CANDIDATE_1, ACCOUNT_CANDIDATE_1, 9, final_amount),
                Event::ExecutedDelegate {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_CANDIDATE_1,
                    towards: P::target_pool(),
                    staked: final_amount,
                    released: round_down(requested_amount, 2) - final_amount,
                },
            ]);
        })
    }
);

pool_test!(
    fn self_delegation_above_minimum<P>() {
        ExtBuilder::default().build().execute_with(|| {
            let requested_amount = MinimumSelfDelegation::get();
            let final_amount = round_down(
                requested_amount,
                P::shares_to_stake_or_init(&ACCOUNT_CANDIDATE_1, Shares(1))
                    .unwrap()
                    .0,
            );

            FullDelegation {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_CANDIDATE_1,
                request_amount: requested_amount,
                expected_increase: final_amount,
                ..default()
            }
            .test::<P>();

            FullDelegation {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_CANDIDATE_1,
                request_amount: requested_amount,
                expected_increase: final_amount,
                ..default()
            }
            .test::<P>();

            FullUndelegation {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_CANDIDATE_1,
                request_amount: SharesOrStake::Stake(requested_amount * 2),
                expected_removed: requested_amount * 2,
                expected_leaving: round_down(requested_amount * 2, 3),
                ..default()
            }
            .test::<P>();

            assert_eq_events!(vec![
                // delegation 1
                Event::IncreasedStake {
                    candidate: ACCOUNT_CANDIDATE_1,
                    stake_diff: round_down(requested_amount, 2),
                },
                Event::UpdatedCandidatePosition {
                    candidate: ACCOUNT_CANDIDATE_1,
                    stake: round_down(requested_amount, 2),
                    self_delegation: round_down(requested_amount, 2),
                    before: None,
                    after: Some(0),
                },
                Event::RequestedDelegate {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_CANDIDATE_1,
                    towards: P::target_pool(),
                    pending: round_down(requested_amount, 2),
                },
                P::event_staked(ACCOUNT_CANDIDATE_1, ACCOUNT_CANDIDATE_1, 10, final_amount),
                Event::ExecutedDelegate {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_CANDIDATE_1,
                    towards: P::target_pool(),
                    staked: final_amount,
                    released: round_down(requested_amount, 2) - final_amount,
                },
                // delegation 2
                Event::IncreasedStake {
                    candidate: ACCOUNT_CANDIDATE_1,
                    stake_diff: round_down(requested_amount, 2),
                },
                Event::UpdatedCandidatePosition {
                    candidate: ACCOUNT_CANDIDATE_1,
                    stake: round_down(requested_amount * 2, 2),
                    self_delegation: round_down(requested_amount * 2, 2),
                    before: Some(0),
                    after: Some(0),
                },
                Event::RequestedDelegate {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_CANDIDATE_1,
                    towards: P::target_pool(),
                    pending: round_down(requested_amount, 2),
                },
                P::event_staked(ACCOUNT_CANDIDATE_1, ACCOUNT_CANDIDATE_1, 10, final_amount),
                Event::ExecutedDelegate {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_CANDIDATE_1,
                    towards: P::target_pool(),
                    staked: final_amount,
                    released: round_down(requested_amount, 2) - final_amount,
                },
                // undelegation
                Event::DecreasedStake {
                    candidate: ACCOUNT_CANDIDATE_1,
                    stake_diff: requested_amount * 2,
                },
                Event::UpdatedCandidatePosition {
                    candidate: ACCOUNT_CANDIDATE_1,
                    stake: 0,
                    self_delegation: 0,
                    before: Some(0),
                    after: None,
                },
                Event::RequestedUndelegate {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_CANDIDATE_1,
                    from: P::target_pool(),
                    pending: round_down(requested_amount * 2, 3),
                    released: 2,
                },
                Event::ExecutedUndelegate {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_CANDIDATE_1,
                    released: round_down(requested_amount * 2, 3)
                }
            ]);
        })
    }
);
