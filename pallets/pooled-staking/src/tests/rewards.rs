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

use crate::pools::AutoCompounding;

use {
    super::*,
    crate::{assert_eq_last_events, Pallet, TargetPool},
    tp_core::DistributeRewards,
};

struct Delegation {
    candidate: AccountId,
    delegator: AccountId,
    pool: TargetPool,
    stake: Balance,
}

struct RewardRequest {
    collator: AccountId,
    rewards: Balance,
}

struct ExpectedStake {
    candidate: AccountId,
    delegator: AccountId,
    auto_stake: Balance,
    auto_shares: Balance,
    manual_stake: Balance,
    manual_shares: Balance,
    claimable_rewards: Balance,
}

struct ExpectedDistribution {
    collator_auto: Balance,
    collator_manual: Balance,
    delegators_auto: Balance,
    delegators_manual: Balance,
}

fn test_distribution(
    delegations: &[Delegation],
    reward: RewardRequest,
    stakes: &[ExpectedStake],
    distribution: ExpectedDistribution,
) {
    use crate::traits::Timer;
    let block_number = <Runtime as crate::Config>::JoiningRequestTimer::now();

    for d in delegations {
        assert_ok!(Staking::request_delegate(
            RuntimeOrigin::signed(d.delegator),
            d.candidate,
            d.pool,
            d.stake,
        ));
    }

    for _ in 0..BLOCKS_TO_WAIT {
        roll_one_block();
    }

    for d in delegations {
        assert_ok!(Staking::execute_pending_operations(
            RuntimeOrigin::signed(d.delegator),
            vec![PendingOperationQuery {
                delegator: d.delegator,
                operation: match d.pool {
                    TargetPool::AutoCompounding => PendingOperationKey::JoiningAutoCompounding {
                        candidate: d.candidate, at: block_number
                    },
                    TargetPool::ManualRewards => PendingOperationKey::JoiningManualRewards {
                        candidate: d.candidate, at: block_number
                    },
                }
            }]
        ));
    }

    assert_ok!(Pallet::<Runtime>::distribute_rewards(
        reward.collator,
        reward.rewards
    ));

    assert_eq_last_events!(vec![
        Event::<Runtime>::RewardedCollator {
            collator: reward.collator,
            auto_compounding_rewards: distribution.collator_auto,
            manual_claim_rewards: distribution.collator_manual,
        },
        Event::RewardedDelegators {
            collator: reward.collator,
            auto_compounding_rewards: distribution.delegators_auto,
            manual_claim_rewards: distribution.delegators_manual,
        },
    ]);

    // TODO: Test new stake/shares quantities
}

// #[test]
// candidate_only_manual_only() {
//     ExtBuilder::default().build().execute_with(|| {
//         test_distribution(&[
//             Delegation {
//                 candidate: ACCOUNT_CANDIDATE_1,
//                 delegator: ACCOUNT_CANDIDATE_1,
//                 pool: TargetPool::ManualRewards,
//                 stake: 1_000_000_000
//             }
//         ], RewardRequest {
//             collator: ACCOUNT_CANDIDATE_1,
//         }, stakes, distribution)
//     });
// }

// pool_test!(
//     fn candidate_only_single_pool<P>() {
//         ExtBuilder::default().build().execute_with(|| {
//             let share_value = InitialManualClaimShareValue::get();
//             let stake = 1_000 * share_value;
//             let rewards = 200 * share_value + 42;

//             FullDelegation {
//                 candidate: ACCOUNT_CANDIDATE_1,
//                 delegator: ACCOUNT_CANDIDATE_1,
//                 request_amount: stake,
//                 expected_increase: stake,
//                 ..default()
//             }
//             .test::<P>();

//             assert_ok!(Pallet::<Runtime>::distribute_rewards(
//                 ACCOUNT_CANDIDATE_1,
//                 rewards
//             ));

//             let rewards_collator = rewards * 2 / 10; // 20%
//             let rewards_delegators = rewards - rewards_collator;

//             let (
//                 rewards_collator_manual,
//                 rewards_collator_auto,
//                 rewards_delegators_manual,
//                 rewards_delegators_auto,
//             ) = match P::target_pool() {
//                 TargetPool::AutoCompounding => (0, rewards_collator, 0, rewards_delegators),
//                 TargetPool::ManualRewards => (rewards_collator, 0, rewards_delegators, 0),
//             };

//             assert_eq_last_events!(vec![
//                 Event::<Runtime>::RewardedCollator {
//                     collator: ACCOUNT_CANDIDATE_1,
//                     auto_compounding_rewards: rewards_collator_auto,
//                     manual_claim_rewards: rewards_collator_manual,
//                 },
//                 Event::RewardedDelegators {
//                     collator: ACCOUNT_CANDIDATE_1,
//                     auto_compounding_rewards: rewards_delegators_auto,
//                     manual_claim_rewards: rewards_delegators_manual,
//                 },
//             ])
//         })
//     }
// );
