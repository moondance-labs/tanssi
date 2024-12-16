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

#![cfg(test)]

use {
    crate::{tests::common::*, MinimumSelfDelegation, PooledStaking},
    frame_support::{assert_noop, assert_ok, error::BadOrigin},
    pallet_pooled_staking::{
        traits::IsCandidateEligible, AllTargetPool, EligibleCandidate, PendingOperationKey,
        PendingOperationQuery, PoolsKey, SharesOrStake, TargetPool,
    },
    sp_std::vec,
};

#[test]
fn test_staking_no_candidates_in_genesis() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .with_empty_parachains(vec![1001, 1002])
        .build()
        .execute_with(|| {
            run_to_block(2);

            let initial_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();

            assert_eq!(initial_candidates, vec![]);
        });
}

#[test]
fn test_staking_join() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .with_empty_parachains(vec![1001, 1002])
        .build()
        .execute_with(|| {
            run_to_block(2);

            let balance_before = System::account(AccountId::from(ALICE)).data.free;
            assert_eq!(System::account(AccountId::from(ALICE)).data.reserved, 0);
            let stake = MinimumSelfDelegation::get() * 10;
            assert_ok!(PooledStaking::request_delegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                stake
            ));

            // Immediately after joining, Alice is the top candidate
            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(
                eligible_candidates,
                vec![EligibleCandidate {
                    candidate: ALICE.into(),
                    stake
                }]
            );

            // And staked amount is immediately marked as "reserved"
            let balance_after = System::account(AccountId::from(ALICE)).data.free;
            assert_eq!(balance_before - balance_after, stake);
            assert_eq!(System::account(AccountId::from(ALICE)).data.reserved, stake);
        });
}

#[test]
fn test_staking_join_no_keys_registered() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .with_empty_parachains(vec![
            1001,
            1002,
        ])

        .build()
        .execute_with(|| {
            run_to_block(2);

            let stake = MinimumSelfDelegation::get() * 10;
            let new_account = AccountId::from([42u8; 32]);
            assert_ok!(Balances::transfer_allow_death(
                origin_of(ALICE.into()),
                new_account.clone().into(),
                stake * 2
            ));
            let balance_before = System::account(new_account.clone()).data.free;
            assert_eq!(System::account(new_account.clone()).data.reserved, 0);
            assert_ok!(PooledStaking::request_delegate(
                origin_of(new_account.clone()),
                new_account.clone(),
                TargetPool::AutoCompounding,
                stake
            ));

            // The new account should be the top candidate but it has no keys registered in
            // pallet_session, so it is not eligible
            assert!(!<Runtime as pallet_pooled_staking::Config>::EligibleCandidatesFilter::is_candidate_eligible(&new_account));
            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();

            assert_eq!(eligible_candidates, vec![]);

            // And staked amount is immediately marked as "reserved"
            let balance_after = System::account(new_account.clone()).data.free;
            assert_eq!(balance_before - balance_after, stake);
            assert_eq!(System::account(new_account.clone()).data.reserved, stake);
        });
}

#[test]
fn test_staking_register_keys_after_joining() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .with_empty_parachains(vec![
            1001,
            1002,
        ])

        .build()
        .execute_with(|| {
            run_to_block(2);

            let stake = MinimumSelfDelegation::get() * 10;
            let new_account = AccountId::from([42u8; 32]);
            assert_ok!(Balances::transfer_allow_death(
                origin_of(ALICE.into()),
                new_account.clone().into(),
                stake * 2
            ));
            let balance_before = System::account(new_account.clone()).data.free;
            assert_eq!(System::account(new_account.clone()).data.reserved, 0);
            assert_ok!(PooledStaking::request_delegate(
                origin_of(new_account.clone()),
                new_account.clone(),
                TargetPool::AutoCompounding,
                stake
            ));

            // The new account should be the top candidate but it has no keys registered in
            // pallet_session, so it is not eligible
            assert!(!<Runtime as pallet_pooled_staking::Config>::EligibleCandidatesFilter::is_candidate_eligible(&new_account));
            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(eligible_candidates, vec![]);

            // And staked amount is immediately marked as "reserved"
            let balance_after = System::account(new_account.clone()).data.free;
            assert_eq!(balance_before - balance_after, stake);
            assert_eq!(System::account(new_account.clone()).data.reserved, stake);

            // Now register the keys
            let new_keys = get_authority_keys_from_seed(&new_account.to_string(), None);
            assert_ok!(Session::set_keys(
                origin_of(new_account.clone()),
                crate::SessionKeys {
                    grandpa: new_keys.grandpa,babe: new_keys.babe,para_validator: new_keys.para_validator,para_assignment: new_keys.para_assignment,authority_discovery: new_keys.authority_discovery,beefy: new_keys.beefy,nimbus: new_keys.nimbus,
                },
                vec![]
            ));

            // Now eligible according to filter
            assert!(<Runtime as pallet_pooled_staking::Config>::EligibleCandidatesFilter::is_candidate_eligible(&new_account));
            // But not eligible according to pallet_pooled_staking, need to manually update candidate list
            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(eligible_candidates, vec![]);

            // Update candidate list
            assert_ok!(PooledStaking::update_candidate_position(
                origin_of(BOB.into()),
                vec![new_account.clone()]
            ));

            // Now it is eligible
            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(
                eligible_candidates,
                vec![EligibleCandidate {
                    candidate: new_account.clone(),
                    stake
                }]
            );
        });
}

#[test]
fn test_staking_join_bad_origin() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .with_empty_parachains(vec![1001, 1002])
        .build()
        .execute_with(|| {
            run_to_block(2);

            let stake = 10 * MinimumSelfDelegation::get();
            assert_noop!(
                PooledStaking::request_delegate(
                    root_origin(),
                    ALICE.into(),
                    TargetPool::AutoCompounding,
                    stake
                ),
                BadOrigin,
            );
        });
}

#[test]
fn test_staking_join_below_self_delegation_min() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .with_empty_parachains(vec![1001, 1002])
        .build()
        .execute_with(|| {
            run_to_block(2);

            let stake1 = MinimumSelfDelegation::get() / 3;
            assert_ok!(PooledStaking::request_delegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                stake1
            ));

            // Since stake is below MinimumSelfDelegation, the join operation succeeds
            // but the candidate is not eligible
            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(eligible_candidates, vec![],);

            let stake2 = MinimumSelfDelegation::get() - stake1 - 1;
            assert_ok!(PooledStaking::request_delegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                stake2,
            ));

            // Still below, missing 1 unit
            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(eligible_candidates, vec![],);

            let stake3 = 1;
            assert_ok!(PooledStaking::request_delegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                stake3,
            ));

            // Increasing the stake to above MinimumSelfDelegation makes the candidate eligible
            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(
                eligible_candidates,
                vec![EligibleCandidate {
                    candidate: ALICE.into(),
                    stake: stake1 + stake2 + stake3
                }],
            );
        });
}

#[test]
fn test_staking_join_no_self_delegation() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .with_empty_parachains(vec![1001, 1002])
        .build()
        .execute_with(|| {
            run_to_block(2);

            // Bob delegates to Alice, but Alice is not a valid candidate (not enough self-delegation)
            let stake = 10 * MinimumSelfDelegation::get();
            assert_ok!(PooledStaking::request_delegate(
                origin_of(BOB.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                stake,
            ));

            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(eligible_candidates, vec![],);
        });
}

#[test]
fn test_staking_join_before_self_delegation() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .with_empty_parachains(vec![1001, 1002])
        .build()
        .execute_with(|| {
            run_to_block(2);

            // Bob delegates to Alice, but Alice is not a valid candidate (not enough self-delegation)
            let stake = 10 * MinimumSelfDelegation::get();
            assert_ok!(PooledStaking::request_delegate(
                origin_of(BOB.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                stake
            ));

            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(eligible_candidates, vec![],);

            run_to_session(2);
            assert_ok!(PooledStaking::execute_pending_operations(
                origin_of(ALICE.into()),
                vec![PendingOperationQuery {
                    delegator: BOB.into(),
                    operation: PendingOperationKey::JoiningAutoCompounding {
                        candidate: ALICE.into(),
                        at: 0,
                    }
                }]
            ),);

            // Now Alice joins with enough self-delegation
            assert_ok!(PooledStaking::request_delegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                stake
            ));

            // Alice is a valid candidate, and Bob's stake is also counted
            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(
                eligible_candidates,
                vec![EligibleCandidate {
                    candidate: ALICE.into(),
                    stake: stake * 2,
                }],
            );
        });
}

#[test]
fn test_staking_join_twice_in_same_block() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .with_empty_parachains(vec![1001, 1002])
        .build()
        .execute_with(|| {
            run_to_block(2);

            let stake1 = 10 * MinimumSelfDelegation::get();
            assert_ok!(PooledStaking::request_delegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                stake1
            ));

            let stake2 = 9 * MinimumSelfDelegation::get();
            assert_ok!(PooledStaking::request_delegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                stake2
            ));

            // Both operations succeed and the total stake is the sum of the individual stakes
            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();

            assert_eq!(
                eligible_candidates,
                vec![EligibleCandidate {
                    candidate: ALICE.into(),
                    stake: stake1 + stake2,
                }]
            );

            run_to_session(2);

            assert_ok!(PooledStaking::execute_pending_operations(
                origin_of(ALICE.into()),
                vec![PendingOperationQuery {
                    delegator: ALICE.into(),
                    operation: PendingOperationKey::JoiningAutoCompounding {
                        candidate: ALICE.into(),
                        at: 0,
                    }
                }]
            ),);

            // TODO: ensure the total stake has been moved to auto compounding pool
        });
}

#[test]
fn test_staking_join_execute_before_time() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .with_empty_parachains(vec![1001, 1002])
        .build()
        .execute_with(|| {
            run_to_block(2);

            let stake = 10 * MinimumSelfDelegation::get();
            assert_ok!(PooledStaking::request_delegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                stake
            ));

            // Immediately after joining, Alice is the top candidate
            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(
                eligible_candidates,
                vec![EligibleCandidate {
                    candidate: ALICE.into(),
                    stake
                }]
            );

            // We called request_delegate in session 0, we will be able to execute it starting from session 2
            let start_of_session_2 = session_to_block(2);
            // Session 2 starts at block 600, but run_to_session runs to block 601, so subtract 2 here to go to 599
            run_to_block(start_of_session_2 - 2);
            assert_noop!(
                PooledStaking::execute_pending_operations(
                    origin_of(ALICE.into()),
                    vec![PendingOperationQuery {
                        delegator: ALICE.into(),
                        operation: PendingOperationKey::JoiningAutoCompounding {
                            candidate: ALICE.into(),
                            at: 0,
                        }
                    }]
                ),
                pallet_pooled_staking::Error::<Runtime>::RequestCannotBeExecuted(0),
            );

            run_to_block(start_of_session_2);
            assert_ok!(PooledStaking::execute_pending_operations(
                origin_of(ALICE.into()),
                vec![PendingOperationQuery {
                    delegator: ALICE.into(),
                    operation: PendingOperationKey::JoiningAutoCompounding {
                        candidate: ALICE.into(),
                        at: 0,
                    }
                }]
            ),);
        });
}

#[test]
fn test_staking_join_execute_any_origin() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .with_empty_parachains(vec![1001, 1002])
        .build()
        .execute_with(|| {
            run_to_block(2);

            let stake = 10 * MinimumSelfDelegation::get();
            assert_ok!(PooledStaking::request_delegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                stake
            ));

            // Immediately after joining, Alice is the top candidate
            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(
                eligible_candidates,
                vec![EligibleCandidate {
                    candidate: ALICE.into(),
                    stake
                }]
            );

            // We called request_delegate in session 0, we will be able to execute it starting from session 2
            run_to_session(2);
            // Anyone can execute pending operations for anyone else
            assert_ok!(PooledStaking::execute_pending_operations(
                origin_of(BOB.into()),
                vec![PendingOperationQuery {
                    delegator: ALICE.into(),
                    operation: PendingOperationKey::JoiningAutoCompounding {
                        candidate: ALICE.into(),
                        at: 0,
                    }
                }]
            ),);
        });
}

#[test]
fn test_staking_join_execute_bad_origin() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .with_empty_parachains(vec![1001, 1002])
        .build()
        .execute_with(|| {
            run_to_block(2);

            let stake = 10 * MinimumSelfDelegation::get();
            assert_ok!(PooledStaking::request_delegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                stake
            ));

            // Immediately after joining, Alice is the top candidate
            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(
                eligible_candidates,
                vec![EligibleCandidate {
                    candidate: ALICE.into(),
                    stake
                }]
            );

            // We called request_delegate in session 0, we will be able to execute it starting from session 2
            run_to_session(2);
            assert_noop!(
                PooledStaking::execute_pending_operations(
                    root_origin(),
                    vec![PendingOperationQuery {
                        delegator: ALICE.into(),
                        operation: PendingOperationKey::JoiningAutoCompounding {
                            candidate: ALICE.into(),
                            at: 0,
                        }
                    }]
                ),
                BadOrigin,
            );
        });
}

struct A {
    delegator: AccountId,
    candidate: AccountId,
    target_pool: TargetPool,
    stake: u128,
}

// Setup test environment with provided delegations already being executed. Input function f gets executed at start session 2
fn setup_staking_join_and_execute<R>(ops: Vec<A>, f: impl FnOnce() -> R) {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .with_empty_parachains(vec![1001, 1002])
        .build()
        .execute_with(|| {
            run_to_block(2);

            for op in ops.iter() {
                assert_ok!(PooledStaking::request_delegate(
                    origin_of(op.delegator.clone()),
                    op.candidate.clone(),
                    op.target_pool,
                    op.stake,
                ));
            }

            // We called request_delegate in session 0, we will be able to execute it starting from session 2
            run_to_session(2);

            for op in ops.iter() {
                let operation = match op.target_pool {
                    TargetPool::AutoCompounding => PendingOperationKey::JoiningAutoCompounding {
                        candidate: op.candidate.clone(),
                        at: 0,
                    },
                    TargetPool::ManualRewards => PendingOperationKey::JoiningManualRewards {
                        candidate: op.candidate.clone(),
                        at: 0,
                    },
                };

                assert_ok!(PooledStaking::execute_pending_operations(
                    origin_of(op.delegator.clone()),
                    vec![PendingOperationQuery {
                        delegator: op.delegator.clone(),
                        operation,
                    }]
                ));
            }

            f()
        });
}

#[test]
fn test_staking_leave_exact_amount() {
    setup_staking_join_and_execute(
        vec![A {
            delegator: ALICE.into(),
            candidate: ALICE.into(),
            target_pool: TargetPool::AutoCompounding,
            stake: 10 * MinimumSelfDelegation::get(),
        }],
        || {
            let stake = 10 * MinimumSelfDelegation::get();
            assert_ok!(PooledStaking::request_undelegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                SharesOrStake::Stake(stake),
            ));

            // Immediately after calling request_undelegate, Alice is no longer a candidate
            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(eligible_candidates, vec![]);
        },
    )
}

#[test]
fn test_staking_leave_bad_origin() {
    setup_staking_join_and_execute(
        vec![A {
            delegator: ALICE.into(),
            candidate: ALICE.into(),
            target_pool: TargetPool::AutoCompounding,
            stake: 10 * MinimumSelfDelegation::get(),
        }],
        || {
            let stake = 10 * MinimumSelfDelegation::get();
            assert_noop!(
                PooledStaking::request_undelegate(
                    root_origin(),
                    ALICE.into(),
                    TargetPool::AutoCompounding,
                    SharesOrStake::Stake(stake),
                ),
                BadOrigin
            );
        },
    )
}

#[test]
fn test_staking_leave_more_than_allowed() {
    setup_staking_join_and_execute(
        vec![A {
            delegator: ALICE.into(),
            candidate: ALICE.into(),
            target_pool: TargetPool::AutoCompounding,
            stake: 10 * MinimumSelfDelegation::get(),
        }],
        || {
            let stake = 10 * MinimumSelfDelegation::get();
            assert_noop!(
                PooledStaking::request_undelegate(
                    origin_of(ALICE.into()),
                    ALICE.into(),
                    TargetPool::AutoCompounding,
                    SharesOrStake::Stake(stake + 1 * MinimumSelfDelegation::get()),
                ),
                pallet_pooled_staking::Error::<Runtime>::MathUnderflow,
            );
        },
    );
}

#[test]
fn test_staking_leave_in_separate_transactions() {
    let stake = 10 * MinimumSelfDelegation::get();

    setup_staking_join_and_execute(
        vec![A {
            delegator: ALICE.into(),
            candidate: ALICE.into(),
            target_pool: TargetPool::AutoCompounding,
            stake,
        }],
        || {
            let half_stake = stake / 2;
            assert_ok!(PooledStaking::request_undelegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                SharesOrStake::Stake(half_stake),
            ));

            // Alice is still a valid candidate, now with less stake
            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            let remaining_stake = stake - half_stake;
            assert_eq!(
                eligible_candidates,
                vec![EligibleCandidate {
                    candidate: ALICE.into(),
                    stake: remaining_stake,
                }],
            );

            assert_ok!(PooledStaking::request_undelegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                SharesOrStake::Stake(remaining_stake),
            ));

            // Unstaked remaining stake, so no longer a valid candidate
            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(eligible_candidates, vec![],);
        },
    );
}

#[test]
fn test_staking_leave_all_except_some_dust() {
    let stake = 10 * MinimumSelfDelegation::get();

    setup_staking_join_and_execute(
        vec![A {
            delegator: ALICE.into(),
            candidate: ALICE.into(),
            target_pool: TargetPool::AutoCompounding,
            stake,
        }],
        || {
            let dust = MinimumSelfDelegation::get() / 2;
            assert_ok!(PooledStaking::request_undelegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                SharesOrStake::Stake(stake - dust),
            ));

            // Alice still has some stake left, but not enough to reach MinimumSelfDelegation
            assert_eq!(
                pallet_pooled_staking::Pools::<Runtime>::get(
                    AccountId::from(ALICE),
                    PoolsKey::CandidateTotalStake
                ),
                dust,
            );

            let eligible_candidates =
                pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
            assert_eq!(eligible_candidates, vec![],);

            // Leave with remaining stake
            assert_ok!(PooledStaking::request_undelegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                SharesOrStake::Stake(dust),
            ));

            // Alice has no more stake left
            assert_eq!(
                pallet_pooled_staking::Pools::<Runtime>::get(
                    AccountId::from(ALICE),
                    PoolsKey::CandidateTotalStake
                ),
                0,
            );
        },
    );
}

#[test]
fn test_staking_leave_execute_before_time() {
    let stake = 10 * MinimumSelfDelegation::get();

    setup_staking_join_and_execute(
        vec![A {
            delegator: ALICE.into(),
            candidate: ALICE.into(),
            target_pool: TargetPool::AutoCompounding,
            stake,
        }],
        || {
            let balance_before = System::account(AccountId::from(ALICE)).data.free;
            let at = Session::current_index();
            assert_ok!(PooledStaking::request_undelegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                SharesOrStake::Stake(stake),
            ));

            // Request undelegate does not change account balance
            assert_eq!(
                balance_before,
                System::account(AccountId::from(ALICE)).data.free
            );

            // We called request_delegate in session 0, we will be able to execute it starting from session 2
            let start_of_session_4 = session_to_block(4);
            // Session 4 starts at block 1200, but run_to_session runs to block 1201, so subtract 2 here to go to 1999
            run_to_block(start_of_session_4 - 2);

            assert_noop!(
                PooledStaking::execute_pending_operations(
                    origin_of(ALICE.into()),
                    vec![PendingOperationQuery {
                        delegator: ALICE.into(),
                        operation: PendingOperationKey::Leaving {
                            candidate: ALICE.into(),
                            at,
                        }
                    }]
                ),
                pallet_pooled_staking::Error::<Runtime>::RequestCannotBeExecuted(0)
            );
        },
    );
}

#[test]
fn test_staking_leave_execute_any_origin() {
    let stake = 10 * MinimumSelfDelegation::get();

    setup_staking_join_and_execute(
        vec![A {
            delegator: ALICE.into(),
            candidate: ALICE.into(),
            target_pool: TargetPool::AutoCompounding,
            stake,
        }],
        || {
            let balance_before = System::account(AccountId::from(ALICE)).data.free;
            let at = Session::current_index();
            assert_ok!(PooledStaking::request_undelegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                SharesOrStake::Stake(stake),
            ));

            // Request undelegate does not change account balance
            assert_eq!(
                balance_before,
                System::account(AccountId::from(ALICE)).data.free
            );

            run_to_session(4);

            let balance_before = System::account(AccountId::from(ALICE)).data.free;

            assert_ok!(PooledStaking::execute_pending_operations(
                // Any signed origin can execute this, the stake will go to Alice account
                origin_of(BOB.into()),
                vec![PendingOperationQuery {
                    delegator: ALICE.into(),
                    operation: PendingOperationKey::Leaving {
                        candidate: ALICE.into(),
                        at,
                    }
                }]
            ),);

            let balance_after = System::account(AccountId::from(ALICE)).data.free;
            assert_eq!(balance_after - balance_before, stake);
        },
    );
}

#[test]
fn test_staking_leave_execute_bad_origin() {
    let stake = 10 * MinimumSelfDelegation::get();

    setup_staking_join_and_execute(
        vec![A {
            delegator: ALICE.into(),
            candidate: ALICE.into(),
            target_pool: TargetPool::AutoCompounding,
            stake,
        }],
        || {
            let at = Session::current_index();
            assert_ok!(PooledStaking::request_undelegate(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                SharesOrStake::Stake(stake),
            ));

            run_to_session(4);

            assert_noop!(
                PooledStaking::execute_pending_operations(
                    root_origin(),
                    vec![PendingOperationQuery {
                        delegator: ALICE.into(),
                        operation: PendingOperationKey::Leaving {
                            candidate: ALICE.into(),
                            at,
                        }
                    }]
                ),
                BadOrigin
            );
        },
    );
}

#[test]
fn test_staking_swap() {
    setup_staking_join_and_execute(
        vec![A {
            delegator: ALICE.into(),
            candidate: ALICE.into(),
            target_pool: TargetPool::AutoCompounding,
            stake: 10 * MinimumSelfDelegation::get(),
        }],
        || {
            let stake = 10 * MinimumSelfDelegation::get();
            assert_ok!(PooledStaking::swap_pool(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::AutoCompounding,
                SharesOrStake::Stake(stake),
            ));

            assert_eq!(
                PooledStaking::computed_stake(
                    ALICE.into(),
                    ALICE.into(),
                    AllTargetPool::AutoCompounding
                ),
                Some(0u32.into())
            );
            assert_eq!(
                PooledStaking::computed_stake(
                    ALICE.into(),
                    ALICE.into(),
                    AllTargetPool::ManualRewards
                ),
                Some(stake)
            );

            assert_ok!(PooledStaking::swap_pool(
                origin_of(ALICE.into()),
                ALICE.into(),
                TargetPool::ManualRewards,
                SharesOrStake::Stake(stake),
            ));

            assert_eq!(
                PooledStaking::computed_stake(
                    ALICE.into(),
                    ALICE.into(),
                    AllTargetPool::AutoCompounding
                ),
                Some(stake)
            );
            assert_eq!(
                PooledStaking::computed_stake(
                    ALICE.into(),
                    ALICE.into(),
                    AllTargetPool::ManualRewards
                ),
                Some(0u32.into())
            );
        },
    )
}
