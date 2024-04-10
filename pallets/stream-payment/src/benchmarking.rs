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
    crate::{
        Assets, Call, ChangeKind, Config, DepositChange, Event, Pallet, Party, StreamConfig,
        Streams, TimeProvider,
    },
    frame_benchmarking::{account, impl_benchmark_test_suite, v2::*, BenchmarkError},
    frame_support::{assert_ok, dispatch::RawOrigin},
    frame_system::EventRecord,
};

/// Create a funded user.
fn create_funded_user<T: Config>(
    string: &'static str,
    n: u32,
    asset_id: &T::AssetId,
    // amount: T::Balance,
) -> T::AccountId {
    const SEED: u32 = 0;
    let user = account(string, n, SEED);

    // create a large amount that should be greater than ED
    let amount: T::Balance = 1_000_000_000u32.into();
    let amount: T::Balance = amount * T::Balance::from(1_000_000_000u32);
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
        let asset_id = T::Assets::bench_worst_case_asset_id();
        let time_unit = T::TimeProvider::bench_worst_case_time_unit();

        let source = create_funded_user::<T>("source", 1, &asset_id);
        let target = create_funded_user::<T>("target", 2, &asset_id);

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
        let time_unit = T::TimeProvider::bench_worst_case_time_unit();
        let asset_id = T::Assets::bench_worst_case_asset_id();

        let source = create_funded_user::<T>("source", 1, &asset_id);
        let target = create_funded_user::<T>("target", 2, &asset_id);

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

    #[benchmark]
    fn perform_payment() -> Result<(), BenchmarkError> {
        let time_unit = T::TimeProvider::bench_worst_case_time_unit();
        let asset_id = T::Assets::bench_worst_case_asset_id();

        let source = create_funded_user::<T>("source", 1, &asset_id);
        let target = create_funded_user::<T>("target", 2, &asset_id);

        let rate = 100u32.into();
        let initial_deposit = 1_000_000u32.into();

        assert_ok!(Pallet::<T>::open_stream(
            RawOrigin::Signed(source.clone()).into(),
            target.clone(),
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
            Event::StreamPayment {
                stream_id: 0u32.into(),
                source,
                target,
                amount: rate * delta,
                stalled: false,
            }
            .into(),
        );

        Ok(())
    }

    #[benchmark]
    fn request_change_immediate() -> Result<(), BenchmarkError> {
        let time_unit = T::TimeProvider::bench_worst_case_time_unit();
        let asset_id = T::Assets::bench_worst_case_asset_id();

        let source = create_funded_user::<T>("source", 1, &asset_id);
        let target = create_funded_user::<T>("target", 2, &asset_id);

        let rate = 100u32.into();
        let initial_deposit = 1_000_000u32.into();
        let config = StreamConfig {
            time_unit: time_unit.clone(),
            asset_id,
            rate,
        };

        assert_ok!(Pallet::<T>::open_stream(
            RawOrigin::Signed(source.clone()).into(),
            target,
            config.clone(),
            initial_deposit,
        ));

        let new_config = StreamConfig {
            rate: 101u32.into(),
            ..config.clone()
        };

        #[extrinsic_call]
        Pallet::<T>::request_change(
            RawOrigin::Signed(source.clone()),
            0u32.into(),
            ChangeKind::Suggestion,
            new_config.clone(),
            Some(DepositChange::Increase(1_000u32.into())),
        );

        assert_last_event::<T>(
            Event::StreamConfigChanged {
                stream_id: 0u32.into(),
                old_config: config,
                new_config,
                deposit_change: Some(DepositChange::Increase(1_000u32.into())),
            }
            .into(),
        );

        Ok(())
    }

    #[benchmark]
    fn request_change_delayed() -> Result<(), BenchmarkError> {
        let time_unit = T::TimeProvider::bench_worst_case_time_unit();
        let asset_id = T::Assets::bench_worst_case_asset_id();
        let asset_id2 = T::Assets::bench_worst_case_asset_id2();

        let source = create_funded_user::<T>("source", 1, &asset_id);
        let target = create_funded_user::<T>("target", 2, &asset_id);

        let rate = 100u32.into();
        let initial_deposit = 1_000_000u32.into();
        let config = StreamConfig {
            time_unit: time_unit.clone(),
            asset_id,
            rate,
        };

        assert_ok!(Pallet::<T>::open_stream(
            RawOrigin::Signed(source.clone()).into(),
            target,
            config.clone(),
            initial_deposit,
        ));

        // Change the asset id. In the case asset_id == asset_id2, we decrease the rate so that
        // the request is not executed immediately.
        let new_config = StreamConfig {
            asset_id: asset_id2,
            rate: 99u32.into(),
            ..config.clone()
        };

        let stream_id = 0u32.into();

        #[extrinsic_call]
        Pallet::<T>::request_change(
            RawOrigin::Signed(source.clone()),
            stream_id,
            ChangeKind::Suggestion,
            new_config.clone(),
            Some(DepositChange::Absolute(500u32.into())),
        );

        assert_last_event::<T>(
            Event::StreamConfigChangeRequested {
                stream_id,
                request_nonce: 1,
                requester: Party::Source,
                old_config: config,
                new_config,
            }
            .into(),
        );

        Ok(())
    }

    #[benchmark]
    fn accept_requested_change() -> Result<(), BenchmarkError> {
        let time_unit = T::TimeProvider::bench_worst_case_time_unit();
        let asset_id = T::Assets::bench_worst_case_asset_id();
        let asset_id2 = T::Assets::bench_worst_case_asset_id2();

        let source = create_funded_user::<T>("source", 1, &asset_id);
        let target = create_funded_user::<T>("target", 2, &asset_id);

        let rate = 100u32.into();
        let initial_deposit = 1_000_000u32.into();
        let config = StreamConfig {
            time_unit: time_unit.clone(),
            asset_id,
            rate,
        };

        assert_ok!(Pallet::<T>::open_stream(
            RawOrigin::Signed(source.clone()).into(),
            target.clone(),
            config.clone(),
            initial_deposit,
        ));

        // Change the asset id. In the case asset_id == asset_id2, we decrease the rate so that
        // the request is not executed immediately.
        let new_config = StreamConfig {
            asset_id: asset_id2,
            rate: 99u32.into(),
            ..config.clone()
        };

        assert_ok!(Pallet::<T>::request_change(
            RawOrigin::Signed(source.clone()).into(),
            0u32.into(),
            ChangeKind::Suggestion,
            new_config.clone(),
            Some(DepositChange::Absolute(500u32.into())),
        ));

        #[extrinsic_call]
        _(RawOrigin::Signed(target.clone()), 0u32.into(), 1, None);

        assert_last_event::<T>(
            Event::StreamConfigChanged {
                stream_id: 0u32.into(),
                old_config: config,
                new_config,
                deposit_change: Some(DepositChange::Absolute(500u32.into())),
            }
            .into(),
        );

        Ok(())
    }

    #[benchmark]
    fn cancel_change_request() -> Result<(), BenchmarkError> {
        let time_unit = T::TimeProvider::bench_worst_case_time_unit();
        let asset_id = T::Assets::bench_worst_case_asset_id();
        let asset_id2 = T::Assets::bench_worst_case_asset_id2();

        let source = create_funded_user::<T>("source", 1, &asset_id);
        let target = create_funded_user::<T>("target", 2, &asset_id);

        let rate = 100u32.into();
        let initial_deposit = 1_000_000u32.into();
        let config = StreamConfig {
            time_unit: time_unit.clone(),
            asset_id,
            rate,
        };

        assert_ok!(Pallet::<T>::open_stream(
            RawOrigin::Signed(source.clone()).into(),
            target.clone(),
            config.clone(),
            initial_deposit,
        ));

        // Change the asset id. In the case asset_id == asset_id2, we decrease the rate so that
        // the request is not executed immediately.
        let new_config = StreamConfig {
            asset_id: asset_id2,
            rate: 99u32.into(),
            ..config.clone()
        };

        assert_ok!(Pallet::<T>::request_change(
            RawOrigin::Signed(source.clone()).into(),
            0u32.into(),
            ChangeKind::Suggestion,
            new_config.clone(),
            Some(DepositChange::Absolute(500u32.into())),
        ));

        #[extrinsic_call]
        _(RawOrigin::Signed(source), 0u32.into());

        let stream_id: T::StreamId = 0u32.into();
        assert!(Streams::<T>::get(stream_id)
            .expect("to be a stream")
            .pending_request
            .is_none());

        Ok(())
    }

    #[benchmark]
    fn immediately_change_deposit() -> Result<(), BenchmarkError> {
        let time_unit = T::TimeProvider::bench_worst_case_time_unit();
        let asset_id = T::Assets::bench_worst_case_asset_id();

        let source = create_funded_user::<T>("source", 1, &asset_id);
        let target = create_funded_user::<T>("target", 2, &asset_id);

        let rate = 100u32.into();
        let initial_deposit = 1_000_000u32.into();
        let config = StreamConfig {
            time_unit: time_unit.clone(),
            asset_id: asset_id.clone(),
            rate,
        };

        assert_ok!(Pallet::<T>::open_stream(
            RawOrigin::Signed(source.clone()).into(),
            target.clone(),
            config.clone(),
            initial_deposit,
        ));

        #[extrinsic_call]
        _(
            RawOrigin::Signed(source),
            0u32.into(),
            asset_id,
            DepositChange::Absolute(500u32.into()),
        );

        assert_last_event::<T>(
            Event::StreamConfigChanged {
                stream_id: 0u32.into(),
                old_config: config.clone(),
                new_config: config,
                deposit_change: Some(DepositChange::Absolute(500u32.into())),
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
