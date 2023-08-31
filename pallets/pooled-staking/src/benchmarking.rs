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
    crate::PendingOperationKey::{JoiningAutoCompounding, JoiningManualRewards},
    frame_benchmarking::{account, impl_benchmark_test_suite, v2::*, BenchmarkError},
    frame_support::{
        dispatch::RawOrigin,
        traits::{fungible::Mutate, Get},
    },
    frame_system::{EventRecord, Pallet as System},
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

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn request_delegate() -> Result<(), BenchmarkError> {
        const USER_SEED: u32 = 1;
        let (caller, _deposit_amount) =
            create_funded_user::<T>("caller", USER_SEED, min_candidate_stk::<T>());

        #[extrinsic_call]
        _(
            RawOrigin::Signed(caller.clone()),
            caller.clone(),
            TargetPool::AutoCompounding,
            min_candidate_stk::<T>(),
        );

        // assert that it comes out sorted
        assert_last_event::<T>(
            Event::RequestedDelegate {
                candidate: caller.clone(),
                delegator: caller,
                towards: TargetPool::AutoCompounding,
                pending: min_candidate_stk::<T>(),
            }
            .into(),
        );
        Ok(())
    }

    #[benchmark]
    fn execute_pending_operations() -> Result<(), BenchmarkError> {
        const USER_SEED: u32 = 1;
        let (caller, _deposit_amount) =
            create_funded_user::<T>("caller", USER_SEED, min_candidate_stk::<T>());
        PooledStaking::<T>::request_delegate(
            RawOrigin::Signed(caller.clone()).into(),
            caller.clone(),
            TargetPool::AutoCompounding,
            min_candidate_stk::<T>(),
        )?;

        // Initialize the block at which we should do stuff
        let block_number = frame_system::Pallet::<T>::block_number();

        // TODO: make this parametric by instead of using contains use
        // a custom trait
        // Right now we know this is going to be correct with fast-runtime
        System::<T>::set_block_number(block_number + 10u32.into());
        #[extrinsic_call]
        _(
            RawOrigin::Signed(caller.clone()),
            vec![PendingOperationQuery {
                delegator: caller.clone(),
                operation: JoiningAutoCompounding {
                    candidate: caller.clone(),
                    at_block: block_number,
                },
            }],
        );

        // assert that it comes out sorted
        assert_last_event::<T>(
            Event::ExecutedDelegate {
                candidate: caller.clone(),
                delegator: caller,
                towards: TargetPool::AutoCompounding,
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
        PooledStaking::<T>::request_delegate(
            RawOrigin::Signed(caller.clone()).into(),
            caller.clone(),
            TargetPool::AutoCompounding,
            min_candidate_stk::<T>(),
        )?;

        // Initialize the block at which we should do stuff
        let block_number = frame_system::Pallet::<T>::block_number();

        // TODO: make this parametric by instead of using contains use
        // a custom trait
        // Right now we know this is going to be correct with fast-runtime
        System::<T>::set_block_number(block_number + 10u32.into());

        PooledStaking::<T>::execute_pending_operations(
            RawOrigin::Signed(caller.clone()).into(),
            vec![PendingOperationQuery {
                delegator: caller.clone(),
                operation: JoiningAutoCompounding {
                    candidate: caller.clone(),
                    at_block: block_number,
                },
            }],
        )?;

        // We now have a working delegation, and we can request to undelegate
        // This should take the candidate out from being eligible

        #[extrinsic_call]
        _(
            RawOrigin::Signed(caller.clone()),
            caller.clone(),
            TargetPool::AutoCompounding,
            SharesOrStake::Stake(min_candidate_stk::<T>() / 2u32.into()),
        );

        // assert that it comes out sorted
        // TODO: hardcoded numbers should dissapear
        assert_last_event::<T>(
            Event::RequestedUndelegate {
                candidate: caller.clone(),
                delegator: caller,
                from: TargetPool::AutoCompounding,
                pending: 4999998u32.into(),
                released: 2u32.into(),
            }
            .into(),
        );
        Ok(())
    }

    #[benchmark]
    fn claim_manual_rewards(
        b: Linear<1, { T::EligibleCandidatesBufferSize::get() }>,
    ) -> Result<(), BenchmarkError> {
        const USER_SEED: u32 = 1;
        let (caller, _deposit_amount) =
            create_funded_user::<T>("caller", USER_SEED, min_candidate_stk::<T>() * b.into());

        let mut candidate_delegator = vec![];
        // Create as many delegations as one can
        for i in 0..b {
            let (candidate, _deposit) = create_funded_user::<T>(
                "caller",
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

            PooledStaking::<T>::request_delegate(
                RawOrigin::Signed(caller.clone()).into(),
                candidate.clone(),
                TargetPool::ManualRewards,
                min_candidate_stk::<T>(),
            )?;

            candidate_delegator.push((candidate.clone(), caller.clone()))
        }

        // Initialize the block at which we should do stuff
        let block_number = frame_system::Pallet::<T>::block_number();

        // TODO: make this parametric by instead of using contains use
        // a custom trait
        // Right now we know this is going to be correct with fast-runtime
        System::<T>::set_block_number(block_number + 10u32.into());

        // Execute as many pending operations as posible
        for i in 0..b {
            let candidate: T::AccountId = account("caller", USER_SEED - i - 1, 0);

            PooledStaking::<T>::execute_pending_operations(
                RawOrigin::Signed(caller.clone()).into(),
                vec![PendingOperationQuery {
                    delegator: caller.clone(),
                    operation: JoiningManualRewards {
                        candidate: candidate.clone(),
                        at_block: block_number,
                    },
                }],
            )?;

            // Set counter to simulate rewards.
            let counter = 100u32;
            crate::Pools::<T>::set(candidate, &PoolsKey::ManualRewardsCounter, counter.into());
        }

        #[extrinsic_call]
        _(
            RawOrigin::Signed(caller.clone()),
            candidate_delegator.clone(),
        );

        let (candidate, delegator) = &candidate_delegator[candidate_delegator.len() - 1];
        // We should have the last pairs event as the last event
        assert_last_event::<T>(
            Event::ClaimedManualRewards {
                candidate: candidate.clone(),
                delegator: delegator.clone(),
                rewards: 1000u32.into(),
            }
            .into(),
        );
        Ok(())
    }

    impl_benchmark_test_suite!(
        PooledStaking,
        crate::mock::ExtBuilder::default().build(),
        crate::mock::Runtime,
    );
}
