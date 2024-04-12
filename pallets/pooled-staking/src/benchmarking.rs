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

#![cfg(feature = "runtime-benchmarks")]

use {super::*, crate::Pallet as PooledStaking};

use {
    crate::{
        pools::Pool,
        traits::{IsCandidateEligible, Timer},
        HoldReason,
        PendingOperationKey::{JoiningAutoCompounding, JoiningManualRewards},
    },
    frame_benchmarking::{account, impl_benchmark_test_suite, v2::*, BenchmarkError},
    frame_support::{
        dispatch::RawOrigin,
        traits::{
            fungible::{InspectHold, Mutate, MutateHold},
            tokens::{fungible::Balanced, Precision},
            Get,
        },
    },
    frame_system::EventRecord,
    sp_std::prelude::*,
};

/// Minimum collator candidate stake
fn min_candidate_stk<T: Config>() -> T::Balance {
    <<T as Config>::MinimumSelfDelegation as Get<T::Balance>>::get()
}

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
    let events = frame_system::Pallet::<T>::events();
    let system_event: <T as frame_system::Config>::RuntimeEvent = generic_event.into();
    // compare to the last event record
    let EventRecord { event, .. } = &events[events.len() - 1];
    assert_eq!(event, &system_event);
}

/// Create a funded user.
/// Extra + min_candidate_stk is total minted funds
/// Returns tuple (id, balance)
fn create_funded_user<T: Config>(
    string: &'static str,
    n: u32,
    extra: T::Balance,
) -> (T::AccountId, T::Balance) {
    const SEED: u32 = 0;
    let user = account(string, n, SEED);
    let min_candidate_stk = min_candidate_stk::<T>();
    let total = min_candidate_stk + extra;
    T::Currency::set_balance(&user, total);
    (user, total)
}

pub(crate) fn currency_issue<T: Config + frame_system::Config>(
    amount: T::Balance,
) -> crate::CreditOf<T> {
    <<T as crate::Config>::Currency as Balanced<T::AccountId>>::issue(amount)
}

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn request_delegate() -> Result<(), BenchmarkError> {
        const USER_SEED: u32 = 1;
        let (caller_candidate, _deposit_amount) = create_funded_user::<T>(
            "candidate",
            USER_SEED,
            min_candidate_stk::<T>() * 10u32.into(),
        );

        let (caller_delegator, _deposit_amount) = create_funded_user::<T>(
            "delegator",
            USER_SEED,
            min_candidate_stk::<T>() * 10u32.into(),
        );

        T::EligibleCandidatesFilter::make_candidate_eligible(&caller_candidate, true);
        // self delegation
        PooledStaking::<T>::request_delegate(
            RawOrigin::Signed(caller_candidate.clone()).into(),
            caller_candidate.clone(),
            TargetPool::AutoCompounding,
            min_candidate_stk::<T>(),
        )?;

        // self delegation
        PooledStaking::<T>::request_delegate(
            RawOrigin::Signed(caller_candidate.clone()).into(),
            caller_candidate.clone(),
            TargetPool::ManualRewards,
            min_candidate_stk::<T>(),
        )?;

        let timer = T::JoiningRequestTimer::now();

        T::JoiningRequestTimer::skip_to_elapsed();

        PooledStaking::<T>::execute_pending_operations(
            RawOrigin::Signed(caller_candidate.clone()).into(),
            vec![PendingOperationQuery {
                delegator: caller_candidate.clone(),
                operation: JoiningAutoCompounding {
                    candidate: caller_candidate.clone(),
                    at: timer.clone(),
                },
            }],
        )?;

        PooledStaking::<T>::execute_pending_operations(
            RawOrigin::Signed(caller_candidate.clone()).into(),
            vec![PendingOperationQuery {
                delegator: caller_candidate.clone(),
                operation: JoiningManualRewards {
                    candidate: caller_candidate.clone(),
                    at: timer.clone(),
                },
            }],
        )?;

        // self delegation to have something in joining
        PooledStaking::<T>::request_delegate(
            RawOrigin::Signed(caller_candidate.clone()).into(),
            caller_candidate.clone(),
            TargetPool::ManualRewards,
            min_candidate_stk::<T>(),
        )?;

        // Worst case scenario is: we have already shares in both pools, and we delegate again
        // but we delegate with a different account
        #[extrinsic_call]
        _(
            RawOrigin::Signed(caller_delegator.clone()),
            caller_candidate.clone(),
            TargetPool::AutoCompounding,
            min_candidate_stk::<T>() * 2u32.into(),
        );

        // assert that it comes out sorted
        assert_last_event::<T>(
            Event::RequestedDelegate {
                candidate: caller_candidate.clone(),
                delegator: caller_delegator,
                pool: TargetPool::AutoCompounding,
                pending: min_candidate_stk::<T>() * 2u32.into(),
            }
            .into(),
        );
        Ok(())
    }

    #[benchmark]
    fn execute_pending_operations(
        b: Linear<1, { T::EligibleCandidatesBufferSize::get() }>,
    ) -> Result<(), BenchmarkError> {
        const USER_SEED: u32 = 1000;
        let (caller, _deposit_amount) =
            create_funded_user::<T>("caller", USER_SEED, min_candidate_stk::<T>() * b.into());

        let mut pending_operations = vec![];
        let mut candidates = vec![];

        T::Currency::set_balance(&T::StakingAccount::get(), min_candidate_stk::<T>());

        let timer = T::JoiningRequestTimer::now();

        // Create as many delegations as one can
        for i in 0..b {
            let (candidate, _deposit) = create_funded_user::<T>(
                "candidate",
                USER_SEED - i - 1,
                min_candidate_stk::<T>() * 2u32.into(),
            );
            T::EligibleCandidatesFilter::make_candidate_eligible(&candidate, true);

            // self delegation
            PooledStaking::<T>::request_delegate(
                RawOrigin::Signed(caller.clone()).into(),
                candidate.clone(),
                TargetPool::AutoCompounding,
                min_candidate_stk::<T>(),
            )?;

            pending_operations.push(PendingOperationQuery {
                delegator: caller.clone(),
                operation: JoiningAutoCompounding {
                    candidate: candidate.clone(),
                    at: timer.clone(),
                },
            });
            candidates.push(candidate);
        }

        T::JoiningRequestTimer::skip_to_elapsed();
        #[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), pending_operations);

        let last_candidate = &candidates[candidates.len() - 1];
        // assert that it comes out sorted
        assert_last_event::<T>(
            Event::ExecutedDelegate {
                candidate: last_candidate.clone(),
                delegator: caller,
                pool: TargetPool::AutoCompounding,
                staked: min_candidate_stk::<T>(),
                released: 0u32.into(),
            }
            .into(),
        );
        Ok(())
    }

    #[benchmark]
    fn request_undelegate() -> Result<(), BenchmarkError> {
        const USER_SEED: u32 = 1;
        let (caller, _deposit_amount) =
            create_funded_user::<T>("caller", USER_SEED, min_candidate_stk::<T>());

        T::EligibleCandidatesFilter::make_candidate_eligible(&caller, true);

        PooledStaking::<T>::request_delegate(
            RawOrigin::Signed(caller.clone()).into(),
            caller.clone(),
            TargetPool::AutoCompounding,
            min_candidate_stk::<T>(),
        )?;

        let timer = T::JoiningRequestTimer::now();

        T::JoiningRequestTimer::skip_to_elapsed();

        PooledStaking::<T>::execute_pending_operations(
            RawOrigin::Signed(caller.clone()).into(),
            vec![PendingOperationQuery {
                delegator: caller.clone(),
                operation: JoiningAutoCompounding {
                    candidate: caller.clone(),
                    at: timer.clone(),
                },
            }],
        )?;

        let stake_to_remove = min_candidate_stk::<T>() / 2u32.into();

        // We now have a working delegation, and we can request to undelegate
        // This should take the candidate out from being eligible

        #[extrinsic_call]
        _(
            RawOrigin::Signed(caller.clone()),
            caller.clone(),
            TargetPool::AutoCompounding,
            SharesOrStake::Stake(stake_to_remove),
        );

        // lets get the hold amount to know dust
        let on_hold = T::Currency::balance_on_hold(&HoldReason::PooledStake.into(), &caller);
        // dust gets released immediatly
        let dust = min_candidate_stk::<T>() - on_hold;

        // assert that it comes out sorted
        // TODO: hardcoded numbers should dissapear
        assert_last_event::<T>(
            Event::RequestedUndelegate {
                candidate: caller.clone(),
                delegator: caller,
                from: TargetPool::AutoCompounding,
                pending: stake_to_remove - dust,
                released: dust,
            }
            .into(),
        );
        Ok(())
    }

    #[benchmark]
    fn claim_manual_rewards(
        b: Linear<1, { T::EligibleCandidatesBufferSize::get() }>,
    ) -> Result<(), BenchmarkError> {
        const USER_SEED: u32 = 1000;
        let (caller, _deposit_amount) =
            create_funded_user::<T>("caller", USER_SEED, min_candidate_stk::<T>() * b.into());

        let mut candidate_delegator = vec![];
        T::Currency::set_balance(&T::StakingAccount::get(), min_candidate_stk::<T>());
        // Create as many delegations as one can
        for i in 0..b {
            let (candidate, _deposit) = create_funded_user::<T>(
                "candidate",
                USER_SEED - i - 1,
                min_candidate_stk::<T>() * 2u32.into(),
            );
            T::EligibleCandidatesFilter::make_candidate_eligible(&candidate, true);

            // self delegation
            PooledStaking::<T>::request_delegate(
                RawOrigin::Signed(candidate.clone()).into(),
                candidate.clone(),
                TargetPool::AutoCompounding,
                min_candidate_stk::<T>(),
            )?;

            PooledStaking::<T>::request_delegate(
                RawOrigin::Signed(caller.clone()).into(),
                candidate.clone(),
                TargetPool::ManualRewards,
                min_candidate_stk::<T>(),
            )?;

            candidate_delegator.push((candidate.clone(), caller.clone()))
        }

        let timer = T::JoiningRequestTimer::now();

        T::JoiningRequestTimer::skip_to_elapsed();

        // Set counter to simulate rewards.
        let counter = 100u32;
        // Execute as many pending operations as posible
        for i in 0..b {
            let candidate: T::AccountId = account("candidate", USER_SEED - i - 1, 0);

            PooledStaking::<T>::execute_pending_operations(
                RawOrigin::Signed(caller.clone()).into(),
                vec![PendingOperationQuery {
                    delegator: caller.clone(),
                    operation: JoiningManualRewards {
                        candidate: candidate.clone(),
                        at: timer.clone(),
                    },
                }],
            )?;

            crate::Pools::<T>::set(candidate, &PoolsKey::ManualRewardsCounter, counter.into());
        }

        #[extrinsic_call]
        _(
            RawOrigin::Signed(caller.clone()),
            candidate_delegator.clone(),
        );

        let (candidate, delegator) = &candidate_delegator[candidate_delegator.len() - 1];
        let shares = min_candidate_stk::<T>() / T::InitialManualClaimShareValue::get();
        // We should have the last pairs event as the last event
        assert_last_event::<T>(
            Event::ClaimedManualRewards {
                candidate: candidate.clone(),
                delegator: delegator.clone(),
                rewards: shares * counter.into(),
            }
            .into(),
        );

        Ok(())
    }

    #[benchmark]
    fn rebalance_hold() -> Result<(), BenchmarkError> {
        const USER_SEED: u32 = 1000;
        let (caller, _deposit_amount) =
            create_funded_user::<T>("caller", USER_SEED, min_candidate_stk::<T>() * 2u32.into());

        T::Currency::set_balance(&T::StakingAccount::get(), min_candidate_stk::<T>());
        // Create as many delegations as one can

        let (candidate, _deposit) = create_funded_user::<T>(
            "caller",
            USER_SEED - 1,
            min_candidate_stk::<T>() * 2u32.into(),
        );

        let (caller_2, _deposit_amount) = create_funded_user::<T>(
            "caller",
            USER_SEED - 2u32,
            min_candidate_stk::<T>() * 2u32.into(),
        );

        T::EligibleCandidatesFilter::make_candidate_eligible(&candidate, true);
        // self delegation
        PooledStaking::<T>::request_delegate(
            RawOrigin::Signed(candidate.clone()).into(),
            candidate.clone(),
            TargetPool::AutoCompounding,
            min_candidate_stk::<T>(),
        )?;

        PooledStaking::<T>::request_delegate(
            RawOrigin::Signed(caller.clone()).into(),
            candidate.clone(),
            TargetPool::AutoCompounding,
            min_candidate_stk::<T>(),
        )?;

        PooledStaking::<T>::request_delegate(
            RawOrigin::Signed(caller_2.clone()).into(),
            candidate.clone(),
            TargetPool::AutoCompounding,
            min_candidate_stk::<T>(),
        )?;

        let fake_hold = min_candidate_stk::<T>() / 2u32.into();

        // We manually hack it such that hold!=stake
        pools::Joining::<T>::set_hold(&candidate, &caller, Stake(fake_hold));
        let on_hold_before = T::Currency::balance_on_hold(&HoldReason::PooledStake.into(), &caller);
        T::Currency::release(
            &HoldReason::PooledStake.into(),
            &caller,
            on_hold_before - fake_hold,
            Precision::Exact,
        )?;

        #[extrinsic_call]
        _(
            RawOrigin::Signed(caller.clone()),
            candidate.clone(),
            caller.clone(),
            AllTargetPool::Joining,
        );

        // After this hold should have been rebalanced
        let on_hold = T::Currency::balance_on_hold(&HoldReason::PooledStake.into(), &caller);
        assert_eq!(on_hold, min_candidate_stk::<T>());
        Ok(())
    }

    #[benchmark]
    fn update_candidate_position(
        b: Linear<1, { T::EligibleCandidatesBufferSize::get() }>,
    ) -> Result<(), BenchmarkError> {
        const USER_SEED: u32 = 1000;
        let (caller, _deposit_amount) =
            create_funded_user::<T>("caller", USER_SEED, min_candidate_stk::<T>());

        T::Currency::set_balance(&T::StakingAccount::get(), min_candidate_stk::<T>());
        let mut candidates = vec![];

        // Create as many candidates as one can
        for i in 0..b {
            let (candidate, _deposit) = create_funded_user::<T>(
                "candidate",
                USER_SEED - i - 1,
                min_candidate_stk::<T>() * 2u32.into(),
            );

            // self delegation
            PooledStaking::<T>::request_delegate(
                RawOrigin::Signed(candidate.clone()).into(),
                candidate.clone(),
                TargetPool::AutoCompounding,
                min_candidate_stk::<T>(),
            )?;

            // Make candidate eligible
            T::EligibleCandidatesFilter::make_candidate_eligible(&candidate, true);

            candidates.push(candidate.clone())
        }

        #[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), candidates);

        Ok(())
    }

    #[benchmark]
    fn swap_pool() -> Result<(), BenchmarkError> {
        const USER_SEED: u32 = 1;

        let source_stake = min_candidate_stk::<T>() * 10u32.into();

        let (caller, _deposit_amount) = create_funded_user::<T>("caller", USER_SEED, source_stake);

        T::EligibleCandidatesFilter::make_candidate_eligible(&caller, true);

        PooledStaking::<T>::request_delegate(
            RawOrigin::Signed(caller.clone()).into(),
            caller.clone(),
            TargetPool::AutoCompounding,
            source_stake,
        )?;

        let timer = T::JoiningRequestTimer::now();

        T::JoiningRequestTimer::skip_to_elapsed();

        PooledStaking::<T>::execute_pending_operations(
            RawOrigin::Signed(caller.clone()).into(),
            vec![PendingOperationQuery {
                delegator: caller.clone(),
                operation: JoiningAutoCompounding {
                    candidate: caller.clone(),
                    at: timer.clone(),
                },
            }],
        )?;

        #[extrinsic_call]
        _(
            RawOrigin::Signed(caller.clone()),
            caller.clone(),
            TargetPool::AutoCompounding,
            SharesOrStake::Stake(source_stake),
        );

        let target_stake = source_stake;
        let source_shares = crate::pools::AutoCompounding::<T>::stake_to_shares_or_init(
            &caller,
            Stake(source_stake),
        )
        .unwrap()
        .0;

        let target_shares =
            crate::pools::ManualRewards::<T>::stake_to_shares_or_init(&caller, Stake(target_stake))
                .unwrap()
                .0;

        assert_last_event::<T>(
            Event::SwappedPool {
                candidate: caller.clone(),
                delegator: caller,
                source_pool: TargetPool::AutoCompounding,
                source_shares,
                source_stake,
                target_shares,
                target_stake,
                pending_leaving: 0u32.into(),
                released: 0u32.into(),
            }
            .into(),
        );

        Ok(())
    }

    #[benchmark]
    fn distribute_rewards() -> Result<(), BenchmarkError> {
        const USER_SEED: u32 = 1;

        let source_stake = min_candidate_stk::<T>() * 10u32.into();

        let (caller, _deposit_amount) =
            create_funded_user::<T>("caller", USER_SEED, source_stake * 2u32.into());

        T::EligibleCandidatesFilter::make_candidate_eligible(&caller, true);

        PooledStaking::<T>::request_delegate(
            RawOrigin::Signed(caller.clone()).into(),
            caller.clone(),
            TargetPool::AutoCompounding,
            source_stake,
        )?;
        PooledStaking::<T>::request_delegate(
            RawOrigin::Signed(caller.clone()).into(),
            caller.clone(),
            TargetPool::ManualRewards,
            source_stake,
        )?;

        let timer = T::JoiningRequestTimer::now();

        T::JoiningRequestTimer::skip_to_elapsed();

        PooledStaking::<T>::execute_pending_operations(
            RawOrigin::Signed(caller.clone()).into(),
            vec![
                PendingOperationQuery {
                    delegator: caller.clone(),
                    operation: JoiningAutoCompounding {
                        candidate: caller.clone(),
                        at: timer.clone(),
                    },
                },
                PendingOperationQuery {
                    delegator: caller.clone(),
                    operation: JoiningManualRewards {
                        candidate: caller.clone(),
                        at: timer.clone(),
                    },
                },
            ],
        )?;

        T::Currency::mint_into(&T::StakingAccount::get(), source_stake).unwrap();

        #[block]
        {
            crate::pools::distribute_rewards::<T>(&caller, currency_issue::<T>(source_stake))?;
        }

        Ok(())
    }

    impl_benchmark_test_suite!(
        PooledStaking,
        crate::mock::ExtBuilder::default().build(),
        crate::mock::Runtime,
    );
}
