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
    crate::PendingOperationKey::JoiningAutoCompounding,
    frame_benchmarking::{account, impl_benchmark_test_suite, v2::*, BenchmarkError},
    frame_support::{
        dispatch::RawOrigin,
        traits::{
            fungible::Mutate,
            Get,
        },
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

        System::<T>::set_block_number(block_number + 5u32.into());
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

    impl_benchmark_test_suite!(
        PooledStaking,
        crate::mock::ExtBuilder::default().build(),
        crate::mock::Runtime,
    );
}
