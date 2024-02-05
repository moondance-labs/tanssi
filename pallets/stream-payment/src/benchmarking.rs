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
    crate::{Assets, Call, Config, Event, Pallet, StreamConfig, TimeProvider},
    frame_benchmarking::{account, impl_benchmark_test_suite, v2::*, BenchmarkError},
    frame_support::{assert_ok, dispatch::RawOrigin},
    frame_system::EventRecord,
    sp_std::vec,
};

/// Create a funded user.
fn create_funded_user<T: Config>(
    string: &'static str,
    n: u32,
    asset_id: &T::AssetId,
    amount: T::Balance,
) -> T::AccountId {
    const SEED: u32 = 0;
    let user = account(string, n, SEED);
    T::Assets::bench_set_balance(asset_id, &user, amount);
    user
}

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
    let events = frame_system::Pallet::<T>::events();
    let system_event: <T as frame_system::Config>::RuntimeEvent = generic_event.into();
    // compare to the last event record
    let EventRecord { event, .. } = &events[events.len() - 1];
    assert_eq!(event, &system_event);
}

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn open_stream() -> Result<(), BenchmarkError> {
        let asset_id = T::Assets::bench_asset_id();
        let time_unit = T::TimeProvider::bench_time_unit();

        let source = create_funded_user::<T>("source", 1, &asset_id, 1_000_000_000u32.into());
        let target = create_funded_user::<T>("target", 2, &asset_id, 1_000_000_000u32.into());

        #[extrinsic_call]
        _(
            RawOrigin::Signed(source.clone()),
            target,
            StreamConfig {
                time_unit,
                asset_id,
                rate: 100u32.into(),
            },
            1_000_000u32.into(),
        );

        assert_last_event::<T>(
            Event::StreamOpened {
                stream_id: 0u32.into(),
            }
            .into(),
        );

        Ok(())
    }

    #[benchmark]
    fn close_stream() -> Result<(), BenchmarkError> {
        // Worst case is closing a stream with a pending payment.
        let time_unit = T::TimeProvider::bench_time_unit();
        let asset_id = T::Assets::bench_asset_id();

        let source = create_funded_user::<T>("source", 1, &asset_id, 1_000_000_000u32.into());
        let target = create_funded_user::<T>("target", 2, &asset_id, 1_000_000_000u32.into());

        let rate = 100u32.into();
        let initial_deposit = 1_000_000u32.into();

        assert_ok!(Pallet::<T>::open_stream(
            RawOrigin::Signed(source.clone()).into(),
            target,
            StreamConfig {
                time_unit: time_unit.clone(),
                asset_id,
                rate,
            },
            initial_deposit,
        ));

        // Change time to trigger payment.
        let now = T::TimeProvider::now(&time_unit).expect("can fetch time");
        let delta: T::Balance = 10u32.into();
        T::TimeProvider::bench_set_now(now + delta);

        #[extrinsic_call]
        _(RawOrigin::Signed(source.clone()), 0u32.into());

        assert_last_event::<T>(
            Event::StreamClosed {
                stream_id: 0u32.into(),
                refunded: initial_deposit - (rate * delta),
            }
            .into(),
        );

        Ok(())
    }

    impl_benchmark_test_suite!(
        Pallet,
        crate::mock::ExtBuilder::default().build(),
        crate::mock::Runtime,
    );
}
