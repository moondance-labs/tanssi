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

use crate::{assert_eq_last_events, candidate::EligibleCandidate, SortedEligibleCandidates};

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
                    pool: P::target_pool(),
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
                    pool: P::target_pool(),
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
                    pool: P::target_pool(),
                    pending: round_down(requested_amount, 2),
                },
                P::event_staked(ACCOUNT_CANDIDATE_1, ACCOUNT_CANDIDATE_1, 10, final_amount),
                Event::ExecutedDelegate {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_CANDIDATE_1,
                    pool: P::target_pool(),
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
                    pool: P::target_pool(),
                    pending: round_down(requested_amount, 2),
                },
                P::event_staked(ACCOUNT_CANDIDATE_1, ACCOUNT_CANDIDATE_1, 10, final_amount),
                Event::ExecutedDelegate {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_CANDIDATE_1,
                    pool: P::target_pool(),
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

#[test]
fn many_candidates_mixed_pools() {
    ExtBuilder::default().build().execute_with(|| {
        let share = InitialAutoCompoundingShareValue::get();
        // for simplicity we consider shares of both pools have the same value.
        assert_eq!(
            InitialAutoCompoundingShareValue::get(),
            InitialManualClaimShareValue::get()
        );

        struct Action {
            candidate: AccountId,
            delegator: AccountId,
            join: bool,
            auto: bool,
            amount: Balance,
            total_stake: Balance,
            total_self: Balance,

            rank_before: Option<u32>,
            rank_after: Option<u32>,
        }

        fn perform_actions(actions: &[Action]) {
            let share = InitialAutoCompoundingShareValue::get();
            for action in actions {
                match action {
                    Action {
                        join: true,
                        auto: true,
                        ..
                    } => {
                        FullDelegation {
                            candidate: action.candidate,
                            delegator: action.delegator,
                            request_amount: action.amount,
                            expected_increase: action.amount,
                            ..default()
                        }
                        .test::<pools::AutoCompounding<Runtime>>();

                        assert_eq_last_events!(vec![
                            Event::<Runtime>::IncreasedStake {
                                candidate: action.candidate,
                                stake_diff: action.amount,
                            },
                            Event::UpdatedCandidatePosition {
                                candidate: action.candidate,
                                stake: action.total_stake,
                                self_delegation: action.total_self,
                                before: action.rank_before,
                                after: action.rank_after,
                            },
                            Event::RequestedDelegate {
                                candidate: action.candidate,
                                delegator: action.delegator,
                                pool: TargetPool::AutoCompounding,
                                pending: action.amount,
                            },
                            Event::StakedAutoCompounding {
                                candidate: action.candidate,
                                delegator: action.delegator,
                                shares: action.amount / share,
                                stake: action.amount,
                            },
                            Event::ExecutedDelegate {
                                candidate: action.candidate,
                                delegator: action.delegator,
                                pool: TargetPool::AutoCompounding,
                                staked: action.amount,
                                released: 0,
                            },
                        ])
                    }
                    Action {
                        join: true,
                        auto: false,
                        ..
                    } => {
                        FullDelegation {
                            candidate: action.candidate,
                            delegator: action.delegator,
                            request_amount: action.amount,
                            expected_increase: action.amount,
                            ..default()
                        }
                        .test::<pools::ManualRewards<Runtime>>();

                        assert_eq_last_events!(vec![
                            Event::<Runtime>::IncreasedStake {
                                candidate: action.candidate,
                                stake_diff: action.amount,
                            },
                            Event::UpdatedCandidatePosition {
                                candidate: action.candidate,
                                stake: action.total_stake,
                                self_delegation: action.total_self,
                                before: action.rank_before,
                                after: action.rank_after,
                            },
                            Event::RequestedDelegate {
                                candidate: action.candidate,
                                delegator: action.delegator,
                                pool: TargetPool::ManualRewards,
                                pending: action.amount,
                            },
                            Event::StakedManualRewards {
                                candidate: action.candidate,
                                delegator: action.delegator,
                                shares: action.amount / share,
                                stake: action.amount,
                            },
                            Event::ExecutedDelegate {
                                candidate: action.candidate,
                                delegator: action.delegator,
                                pool: TargetPool::ManualRewards,
                                staked: action.amount,
                                released: 0,
                            },
                        ])
                    }
                    Action {
                        join: false,
                        auto: true,
                        ..
                    } => {
                        FullUndelegation {
                            candidate: action.candidate,
                            delegator: action.delegator,
                            request_amount: SharesOrStake::Stake(action.amount),
                            expected_removed: action.amount,
                            expected_leaving: round_down(action.amount, 3),
                            ..default()
                        }
                        .test::<pools::AutoCompounding<Runtime>>();

                        assert_eq_last_events!(vec![
                            Event::<Runtime>::DecreasedStake {
                                candidate: action.candidate,
                                stake_diff: action.amount,
                            },
                            Event::UpdatedCandidatePosition {
                                candidate: action.candidate,
                                stake: action.total_stake,
                                self_delegation: action.total_self,
                                before: action.rank_before,
                                after: action.rank_after,
                            },
                            Event::RequestedUndelegate {
                                candidate: action.candidate,
                                delegator: action.delegator,
                                from: TargetPool::AutoCompounding,
                                pending: round_down(action.amount, 3),
                                released: action.amount - round_down(action.amount, 3),
                            },
                            Event::ExecutedUndelegate {
                                candidate: action.candidate,
                                delegator: action.delegator,
                                released: round_down(action.amount, 3),
                            },
                        ])
                    }
                    _ => todo!(),
                }
            }
        }

        perform_actions(&[
            Action {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_CANDIDATE_1,
                join: true,
                auto: true,
                amount: share * 11,
                total_stake: share * 11,
                total_self: share * 11,
                rank_before: None,
                rank_after: Some(0),
            },
            Action {
                candidate: ACCOUNT_CANDIDATE_2,
                delegator: ACCOUNT_CANDIDATE_2,
                join: true,
                auto: false,
                amount: share * 10,
                total_stake: share * 10,
                total_self: share * 10,
                rank_before: None,
                rank_after: Some(1),
            },
            Action {
                candidate: ACCOUNT_CANDIDATE_2,
                delegator: ACCOUNT_DELEGATOR_1,
                join: true,
                auto: true,
                amount: share * 3,
                total_stake: share * 13,
                total_self: share * 10,
                rank_before: Some(1),
                rank_after: Some(0),
            },
            Action {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_DELEGATOR_2,
                join: true,
                auto: false,
                amount: share,
                total_stake: share * 12,
                total_self: share * 11,
                rank_before: Some(1),
                rank_after: Some(1),
            },
            Action {
                candidate: ACCOUNT_DELEGATOR_1,
                delegator: ACCOUNT_DELEGATOR_1,
                join: true,
                auto: true,
                amount: share * 11,
                total_stake: share * 11,
                total_self: share * 11,
                rank_before: None,
                rank_after: Some(2),
            },
            Action {
                candidate: ACCOUNT_DELEGATOR_2,
                delegator: ACCOUNT_DELEGATOR_2,
                join: true,
                auto: true,
                amount: share * 10,
                total_stake: share * 10,
                total_self: share * 10,
                rank_before: None,
                rank_after: None, // list is full
            },
        ]);

        assert_eq!(
            SortedEligibleCandidates::<Runtime>::get().into_inner(),
            vec![
                EligibleCandidate {
                    candidate: ACCOUNT_CANDIDATE_2,
                    stake: share * 13,
                },
                EligibleCandidate {
                    candidate: ACCOUNT_CANDIDATE_1,
                    stake: share * 12,
                },
                EligibleCandidate {
                    candidate: ACCOUNT_DELEGATOR_1,
                    stake: share * 11,
                },
            ]
        );

        // We make candidate 1 leave, which doesn't make the out of list
        // candidate back in the list.
        perform_actions(&[Action {
            candidate: ACCOUNT_CANDIDATE_1,
            delegator: ACCOUNT_CANDIDATE_1,
            join: false,
            auto: true,
            amount: share * 11,
            total_stake: share * 1,
            total_self: 0,
            rank_before: Some(1),
            rank_after: None,
        }]);

        assert_eq!(
            SortedEligibleCandidates::<Runtime>::get().into_inner(),
            vec![
                EligibleCandidate {
                    candidate: ACCOUNT_CANDIDATE_2,
                    stake: share * 13,
                },
                EligibleCandidate {
                    candidate: ACCOUNT_DELEGATOR_1,
                    stake: share * 11,
                },
            ]
        );

        // It needs to be done manually.
        assert_ok!(Staking::update_candidate_position(
            RuntimeOrigin::signed(ACCOUNT_DELEGATOR_2),
            vec![ACCOUNT_DELEGATOR_2]
        ));

        assert_eq!(
            SortedEligibleCandidates::<Runtime>::get().into_inner(),
            vec![
                EligibleCandidate {
                    candidate: ACCOUNT_CANDIDATE_2,
                    stake: share * 13,
                },
                EligibleCandidate {
                    candidate: ACCOUNT_DELEGATOR_1,
                    stake: share * 11,
                },
                EligibleCandidate {
                    candidate: ACCOUNT_DELEGATOR_2,
                    stake: share * 10,
                },
            ]
        );
    })
}
