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
    super::*,
    crate::{
        assert_eq_last_events,
        pools::{AutoCompounding, ManualRewards},
        Pallet, TargetPool,
    },
    frame_support::assert_err,
    sp_runtime::DispatchError,
    tp_traits::DistributeRewards,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DelegatorState {
    candidate: AccountId,
    delegator: AccountId,
    auto_stake: Balance,
    auto_shares: Balance,
    manual_stake: Balance,
    manual_shares: Balance,
    pending_rewards: Balance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Distribution {
    collator_auto: Balance,
    collator_manual: Balance,
    delegators_auto: Balance,
    delegators_manual: Balance,
}

fn test_distribution(
    delegations: &[Delegation],
    reward: RewardRequest,
    stakes: &[DelegatorState],
    distribution: Distribution,
) {
    use crate::traits::Timer;
    let block_number = <Runtime as crate::Config>::JoiningRequestTimer::now();

    // Create new supply for rewards
    let new_supply = currency_issue(reward.rewards);
    use frame_support::traits::Imbalance;
    let new_supply_amount = new_supply.peek();

    // Request all delegations
    for d in delegations {
        assert_ok!(Staking::request_delegate(
            RuntimeOrigin::signed(d.delegator),
            d.candidate,
            d.pool,
            d.stake,
        ));
    }

    // Wait for delegation to be executable
    for _ in 0..BLOCKS_TO_WAIT {
        roll_one_block();
    }

    // Execute delegations
    for d in delegations {
        assert_ok!(Staking::execute_pending_operations(
            RuntimeOrigin::signed(d.delegator),
            vec![PendingOperationQuery {
                delegator: d.delegator,
                operation: match d.pool {
                    TargetPool::AutoCompounding => PendingOperationKey::JoiningAutoCompounding {
                        candidate: d.candidate,
                        at: block_number
                    },
                    TargetPool::ManualRewards => PendingOperationKey::JoiningManualRewards {
                        candidate: d.candidate,
                        at: block_number
                    },
                }
            }]
        ));
    }

    // Distribute rewards
    let candidate_balance_before = total_balance(&ACCOUNT_CANDIDATE_1);
    assert_ok!(Pallet::<Runtime>::distribute_rewards(
        reward.collator,
        new_supply
    ));
    let candidate_balance_after = total_balance(&ACCOUNT_CANDIDATE_1);

    // Check events matches the expected distribution.
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

    // Check the state of each delegate match the expected values.
    for expected in stakes {
        let actual = DelegatorState {
            candidate: expected.candidate,
            delegator: expected.delegator,
            auto_stake: AutoCompounding::<Runtime>::computed_stake(
                &expected.candidate,
                &expected.delegator,
            )
            .expect("to have stake")
            .0,
            auto_shares: AutoCompounding::<Runtime>::shares(
                &expected.candidate,
                &expected.delegator,
            )
            .0,
            manual_stake: ManualRewards::<Runtime>::computed_stake(
                &expected.candidate,
                &expected.delegator,
            )
            .expect("to have stake")
            .0,
            manual_shares: ManualRewards::<Runtime>::shares(
                &expected.candidate,
                &expected.delegator,
            )
            .0,
            pending_rewards: ManualRewards::<Runtime>::pending_rewards(
                &expected.candidate,
                &expected.delegator,
            )
            .expect("no overflow")
            .0,
        };

        similar_asserts::assert_eq!(&actual, expected);
    }

    // Additional checks.
    assert_eq!(
        distribution.collator_auto
            + distribution.collator_manual
            + distribution.delegators_auto
            + distribution.delegators_manual,
        new_supply_amount,
        "Distribution total doesn't match requested reward"
    );

    assert_eq!(
        candidate_balance_before + distribution.collator_manual,
        candidate_balance_after,
        "candidate balance should be increased by collator_manual"
    );

    let sum_manual: Balance = stakes.iter().map(|s| s.pending_rewards).sum();
    assert_eq!(
        sum_manual, distribution.delegators_manual,
        "sum of pending rewards should match distributed delegators manual rewards"
    );

    let sum_auto_stake_before: Balance = delegations
        .iter()
        .filter_map(|d| match d.pool {
            TargetPool::AutoCompounding => Some(d.stake),
            _ => None,
        })
        .sum();

    let sum_auto_stake_after = AutoCompounding::<Runtime>::total_staked(&reward.collator).0;
    assert_eq!(
        sum_auto_stake_after - sum_auto_stake_before,
        distribution.collator_auto + distribution.delegators_auto,
        "diff between sum of auto stake before and after distribution should match distributed auto rewards"
    );
}

#[test]
fn candidate_only_manual_only() {
    ExtBuilder::default().build().execute_with(|| {
        test_distribution(
            &[Delegation {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_CANDIDATE_1,
                pool: TargetPool::ManualRewards,
                stake: 1_000_000_000,
            }],
            RewardRequest {
                collator: ACCOUNT_CANDIDATE_1,
                rewards: 1_000_000,
            },
            &[DelegatorState {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_CANDIDATE_1,
                auto_shares: 0,
                auto_stake: 0,
                manual_shares: 1_000,
                manual_stake: 1_000_000_000,
                pending_rewards: 800_000,
            }],
            Distribution {
                collator_auto: 0,
                collator_manual: 200_000, // 20% of rewards
                delegators_auto: 0,
                delegators_manual: 800_000, // 80% of rewards
            },
        )
    });
}

#[test]
fn candidate_only_auto_only() {
    ExtBuilder::default().build().execute_with(|| {
        test_distribution(
            &[Delegation {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_CANDIDATE_1,
                pool: TargetPool::AutoCompounding,
                stake: 1_000_000_000,
            }],
            RewardRequest {
                collator: ACCOUNT_CANDIDATE_1,
                rewards: 10_000_000,
            },
            &[DelegatorState {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_CANDIDATE_1,
                auto_shares: 1_001,
                // initial auto stake is 1_000_000_000 for
                // 8_000_000 is shared between all delegators, so 1 share
                // is now worth 1_008_000_000 / 1000 = 1_008_000 now
                // collator is should be rewarded 2_000_000 in auto shares,
                // which allows to get 1 more share, so the collator now
                // have 1_001 shares worth
                // 1_008_000_000 + 1_008_000 = 1_009_008_000
                auto_stake: 1_009_008_000,
                manual_shares: 0,
                manual_stake: 0,
                pending_rewards: 0,
            }],
            Distribution {
                // 20% of rewards, rounded down to closest amount of Auto shares
                // AFTER delegators rewards has been rewarded
                collator_auto: 1_008_000,
                // dust from collator_auto
                collator_manual: 992_000,
                delegators_auto: 8_000_000, // 80% of rewards
                delegators_manual: 0,
            },
        )
    });
}

#[test]
fn candidate_only_mixed() {
    ExtBuilder::default().build().execute_with(|| {
        test_distribution(
            &[
                Delegation {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_CANDIDATE_1,
                    pool: TargetPool::AutoCompounding,
                    stake: 1_000_000_000,
                },
                Delegation {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_CANDIDATE_1,
                    pool: TargetPool::ManualRewards,
                    stake: 250_000_000,
                },
            ],
            RewardRequest {
                collator: ACCOUNT_CANDIDATE_1,
                rewards: 10_000_000,
            },
            &[DelegatorState {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_CANDIDATE_1,
                auto_shares: 1_001,
                auto_stake: 1_007_406_400,
                manual_shares: 250,
                manual_stake: 250_000_000,
                pending_rewards: 1_600_000,
            }],
            Distribution {
                collator_auto: 1_006_400,
                collator_manual: 993_600,
                delegators_auto: 6_400_000,
                delegators_manual: 1_600_000,
            },
        )
    });
}

#[test]
fn delegators_manual_only() {
    ExtBuilder::default().build().execute_with(|| {
        test_distribution(
            &[
                Delegation {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_CANDIDATE_1,
                    pool: TargetPool::ManualRewards,
                    stake: 1_000_000_000,
                },
                Delegation {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_DELEGATOR_1,
                    pool: TargetPool::ManualRewards,
                    stake: 250_000_000,
                },
            ],
            RewardRequest {
                collator: ACCOUNT_CANDIDATE_1,
                rewards: 10_000_000,
            },
            &[
                DelegatorState {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_CANDIDATE_1,
                    auto_shares: 0,
                    auto_stake: 0,
                    manual_shares: 1_000,
                    manual_stake: 1_000_000_000,
                    pending_rewards: 6_400_000,
                },
                DelegatorState {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_DELEGATOR_1,
                    auto_shares: 0,
                    auto_stake: 0,
                    manual_shares: 250,
                    manual_stake: 250_000_000,
                    pending_rewards: 1_600_000,
                },
            ],
            Distribution {
                collator_auto: 0,
                collator_manual: 2_000_000,
                delegators_auto: 0,
                delegators_manual: 8_000_000,
            },
        )
    });
}

#[test]
fn delegators_auto_only() {
    ExtBuilder::default().build().execute_with(|| {
        test_distribution(
            &[
                Delegation {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_CANDIDATE_1,
                    pool: TargetPool::AutoCompounding,
                    stake: 1_000_000_000,
                },
                Delegation {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_DELEGATOR_1,
                    pool: TargetPool::AutoCompounding,
                    stake: 250_000_000,
                },
            ],
            RewardRequest {
                collator: ACCOUNT_CANDIDATE_1,
                rewards: 10_000_000,
            },
            &[
                DelegatorState {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_CANDIDATE_1,
                    auto_shares: 1_001,
                    auto_stake: 1_007_406_400,
                    manual_shares: 0,
                    manual_stake: 0,
                    pending_rewards: 0,
                },
                DelegatorState {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_DELEGATOR_1,
                    auto_shares: 250,
                    auto_stake: 251_600_000,
                    manual_shares: 0,
                    manual_stake: 0,
                    pending_rewards: 0,
                },
            ],
            Distribution {
                collator_auto: 1_006_400,
                collator_manual: 993_600,
                delegators_auto: 8_000_000,
                delegators_manual: 0,
            },
        )
    });
}

#[test]
fn delegators_mixed() {
    ExtBuilder::default().build().execute_with(|| {
        test_distribution(
            &[
                Delegation {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_CANDIDATE_1,
                    pool: TargetPool::AutoCompounding,
                    stake: 1_000_000_000,
                },
                Delegation {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_CANDIDATE_1,
                    pool: TargetPool::ManualRewards,
                    stake: 500_000_000,
                },
                Delegation {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_DELEGATOR_1,
                    pool: TargetPool::ManualRewards,
                    stake: 250_000_000,
                },
                Delegation {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_DELEGATOR_1,
                    pool: TargetPool::AutoCompounding,
                    stake: 500_000_000,
                },
            ],
            RewardRequest {
                collator: ACCOUNT_CANDIDATE_1,
                rewards: 10_000_000,
            },
            &[
                DelegatorState {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_CANDIDATE_1,
                    auto_shares: 1_001,
                    auto_stake: 1_004_559_388,
                    manual_shares: 500,
                    manual_stake: 500_000_000,
                    pending_rewards: 1_777_500,
                },
                DelegatorState {
                    candidate: ACCOUNT_CANDIDATE_1,
                    delegator: ACCOUNT_DELEGATOR_1,
                    auto_shares: 500,
                    auto_stake: 501_777_916,
                    manual_shares: 250,
                    manual_stake: 250_000_000,
                    pending_rewards: 888_750,
                },
            ],
            Distribution {
                collator_auto: 1_003_555,
                collator_manual: 996_445,
                // Total stake: 2_250_000_000
                // Auto stake: 1_500_000_000
                // Manual stake: 750_000_000
                // Manual shares: 750
                // Rewards towards delegators: 80% of 10_000_000 = 8_000_000
                // Rewards towards manual deleg
                //   = 8_000_000 * 750_000_000 / 2_250_000_000
                //   = 2_666_666
                //   => 2_666_250 (rounding down to closest multiple of 750)
                //   gives dust of 2_666_666 - 2_666_250 = 416
                delegators_manual: 2_666_250,
                // Rewards towards auto deleg
                // = Rewards deleg - Rewards manual deleg
                // = 8_000_000 - 2_666_250
                // = 5_333_750
                delegators_auto: 5_333_750,
            },
        );
    });
}

#[test]
fn candidate_only_no_stake() {
    // Rewarding a candidate that does not have any stake works
    ExtBuilder::default().build().execute_with(|| {
        test_distribution(
            &[],
            RewardRequest {
                collator: ACCOUNT_CANDIDATE_1,
                rewards: 1_000_000,
            },
            &[],
            Distribution {
                collator_auto: 0,
                collator_manual: 1_000_000, // 100% of rewards
                delegators_auto: 0,
                delegators_manual: 0, // 0% of rewards
            },
        )
    });
}

#[test]
fn delegator_only_candidate_zero() {
    // Rewarding a candidate that does not have any stake works
    ExtBuilder::default().build().execute_with(|| {
        test_distribution(
            &[Delegation {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_DELEGATOR_1,
                pool: TargetPool::ManualRewards,
                stake: 250_000_000,
            }],
            RewardRequest {
                collator: ACCOUNT_CANDIDATE_1,
                rewards: 1_000_000,
            },
            &[DelegatorState {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_DELEGATOR_1,
                auto_shares: 0,
                auto_stake: 0,
                manual_shares: 250,
                manual_stake: 250_000_000,
                pending_rewards: 800_000,
            }],
            Distribution {
                collator_auto: 0,
                collator_manual: 200_000, // 20% of rewards
                delegators_auto: 0,
                delegators_manual: 800_000, // 80% of rewards
            },
        )
    });
}

#[test]
fn delegator_only_candidate_no_stake_auto_compounding() {
    // Rewarding a candidate that does not have any stake, while some delegator
    // has stake for that candidate
    ExtBuilder::default().build().execute_with(|| {
        test_distribution(
            &[Delegation {
                candidate: ACCOUNT_CANDIDATE_1,
                delegator: ACCOUNT_DELEGATOR_1,
                pool: TargetPool::AutoCompounding,
                stake: 250_000_000,
            }],
            RewardRequest {
                collator: ACCOUNT_CANDIDATE_1,
                rewards: 1_000_000,
            },
            &[],
            Distribution {
                collator_auto: 0,
                collator_manual: 200_000, // 20% of rewards
                delegators_auto: 800_000, // 80% of rewards
                delegators_manual: 0,
            },
        )
    });
}

#[test]
fn reward_distribution_is_transactional() {
    ExtBuilder::default().build().execute_with(|| {
        use crate::traits::Timer;
        let request_time = <Runtime as crate::Config>::JoiningRequestTimer::now();

        assert_ok!(Staking::request_delegate(
            RuntimeOrigin::signed(ACCOUNT_CANDIDATE_1),
            ACCOUNT_CANDIDATE_1,
            TargetPool::AutoCompounding,
            1_000_000_000,
        ));

        // Wait for delegation to be executable
        for _ in 0..BLOCKS_TO_WAIT {
            roll_one_block();
        }

        assert_ok!(Staking::execute_pending_operations(
            RuntimeOrigin::signed(ACCOUNT_CANDIDATE_1),
            vec![PendingOperationQuery {
                delegator: ACCOUNT_CANDIDATE_1,
                operation: PendingOperationKey::JoiningAutoCompounding {
                    candidate: ACCOUNT_CANDIDATE_1,
                    at: request_time
                },
            }]
        ));

        let total_staked_before =
            pools::AutoCompounding::<Runtime>::total_staked(&ACCOUNT_CANDIDATE_1);

        // Increase ED to make reward destribution fail when resolving
        // credit to Staking account.
        MockExistentialDeposit::set(u128::MAX);

        let rewards = Balances::issue(1_000_000_000);
        assert_err!(
            Staking::distribute_rewards(ACCOUNT_CANDIDATE_1, rewards),
            DispatchError::NoProviders
        );

        let total_staked_after =
            pools::AutoCompounding::<Runtime>::total_staked(&ACCOUNT_CANDIDATE_1);
        assert_eq!(
            total_staked_before, total_staked_after,
            "distribution should be reverted"
        );
    })
}
