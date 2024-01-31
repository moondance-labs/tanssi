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
        assert_event_emitted, assert_event_not_emitted,
        mock::{
            roll_to, Balances, ExtBuilder, Runtime, RuntimeOrigin, StreamPayment,
            StreamPaymentAssetId, StreamPaymentAssets, TimeUnit, ALICE, BOB, CHARLIE,
            DEFAULT_BALANCE, KILO, MEGA,
        },
        Assets, ChangeKind, DepositChange, Error, Event, FreezeReason, LookupStreamsWithSource,
        LookupStreamsWithTarget, NextStreamId, Stream, StreamConfig, Streams,
    },
    frame_support::{assert_err, assert_ok, traits::fungible::InspectFreeze},
    sp_runtime::TokenError,
    tap::tap::Tap,
};

mod open_stream {

    use super::*;

    #[test]
    fn cant_be_both_source_and_target() {
        ExtBuilder::default().build().execute_with(|| {
            assert_err!(
                StreamPayment::open_stream(
                    RuntimeOrigin::signed(ALICE),
                    ALICE,
                    StreamConfig {
                        time_unit: TimeUnit::BlockNumber,
                        asset_id: StreamPaymentAssetId::Native,
                        rate: 100,
                    },
                    0
                ),
                Error::<Runtime>::CantBeBothSourceAndTarget
            );
        })
    }

    #[test]
    fn stream_id_cannot_overflow() {
        ExtBuilder::default().build().execute_with(|| {
            NextStreamId::<Runtime>::set(u64::MAX);

            assert_err!(
                StreamPayment::open_stream(
                    RuntimeOrigin::signed(ALICE),
                    BOB,
                    StreamConfig {
                        time_unit: TimeUnit::BlockNumber,
                        asset_id: StreamPaymentAssetId::Native,
                        rate: 100,
                    },
                    0
                ),
                Error::<Runtime>::StreamIdOverflow
            );
        })
    }

    #[test]
    fn balance_too_low_for_deposit() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_000_000)])
            .build()
            .execute_with(|| {
                assert_err!(
                    StreamPayment::open_stream(
                        RuntimeOrigin::signed(ALICE),
                        BOB,
                        StreamConfig {
                            time_unit: TimeUnit::BlockNumber,
                            asset_id: StreamPaymentAssetId::Native,
                            rate: 100,
                        },
                        1_000_001
                    ),
                    TokenError::FundsUnavailable,
                );
            })
    }

    #[test]
    fn time_can_be_fetched() {
        ExtBuilder::default().build().execute_with(|| {
            assert_err!(
                StreamPayment::open_stream(
                    RuntimeOrigin::signed(ALICE),
                    BOB,
                    StreamConfig {
                        time_unit: TimeUnit::Never,
                        asset_id: StreamPaymentAssetId::Native,
                        rate: 100,
                    },
                    1 * MEGA
                ),
                Error::<Runtime>::CantFetchCurrentTime,
            );
        })
    }

    #[test]
    fn stream_opened() {
        ExtBuilder::default().build().execute_with(|| {
            assert!(Streams::<Runtime>::get(0).is_none());

            assert_ok!(StreamPayment::open_stream(
                RuntimeOrigin::signed(ALICE),
                BOB,
                StreamConfig {
                    time_unit: TimeUnit::BlockNumber,
                    asset_id: StreamPaymentAssetId::Native,
                    rate: 100,
                },
                1 * MEGA
            ));

            assert_event_emitted!(Event::<Runtime>::StreamOpened { stream_id: 0 });

            assert!(Streams::<Runtime>::get(0).is_some());
            assert!(Streams::<Runtime>::get(1).is_none());

            assert_eq!(
                LookupStreamsWithSource::<Runtime>::iter_key_prefix(ALICE).collect::<Vec<_>>(),
                &[0]
            );
            assert!(LookupStreamsWithSource::<Runtime>::iter_key_prefix(BOB)
                .collect::<Vec<_>>()
                .is_empty());
            assert!(LookupStreamsWithTarget::<Runtime>::iter_key_prefix(ALICE)
                .collect::<Vec<_>>()
                .is_empty());
            assert_eq!(
                LookupStreamsWithTarget::<Runtime>::iter_key_prefix(BOB).collect::<Vec<_>>(),
                &[0]
            );

            assert_eq!(
                StreamPaymentAssets::get_deposit(&StreamPaymentAssetId::Native, &ALICE),
                1 * MEGA
            );
            assert_eq!(
                StreamPaymentAssets::get_deposit(&StreamPaymentAssetId::Native, &BOB),
                0
            );
        })
    }

    #[test]
    fn multiple_streams_opened() {
        ExtBuilder::default().build().execute_with(|| {
            assert!(Streams::<Runtime>::get(0).is_none());

            assert_ok!(StreamPayment::open_stream(
                RuntimeOrigin::signed(ALICE),
                BOB,
                StreamConfig {
                    time_unit: TimeUnit::BlockNumber,
                    asset_id: StreamPaymentAssetId::Native,
                    rate: 100,
                },
                1 * MEGA
            ));

            assert_ok!(StreamPayment::open_stream(
                RuntimeOrigin::signed(ALICE),
                CHARLIE,
                StreamConfig {
                    time_unit: TimeUnit::Timestamp,
                    asset_id: StreamPaymentAssetId::Native,
                    rate: 500,
                },
                2 * MEGA
            ));

            assert_ok!(StreamPayment::open_stream(
                RuntimeOrigin::signed(BOB),
                ALICE,
                StreamConfig {
                    time_unit: TimeUnit::Timestamp,
                    asset_id: StreamPaymentAssetId::Native,
                    rate: 200,
                },
                3 * MEGA
            ));

            assert_ok!(StreamPayment::open_stream(
                RuntimeOrigin::signed(ALICE),
                BOB,
                StreamConfig {
                    time_unit: TimeUnit::BlockNumber,
                    asset_id: StreamPaymentAssetId::Native,
                    rate: 300,
                },
                1 * MEGA
            ));

            assert_event_emitted!(Event::<Runtime>::StreamOpened { stream_id: 0 });
            assert_event_emitted!(Event::<Runtime>::StreamOpened { stream_id: 1 });
            assert_event_emitted!(Event::<Runtime>::StreamOpened { stream_id: 2 });
            assert_event_emitted!(Event::<Runtime>::StreamOpened { stream_id: 3 });

            assert!(Streams::<Runtime>::get(0).is_some());
            assert!(Streams::<Runtime>::get(1).is_some());
            assert!(Streams::<Runtime>::get(2).is_some());
            assert!(Streams::<Runtime>::get(3).is_some());
            assert!(Streams::<Runtime>::get(4).is_none());

            let lookup_source = |account| {
                LookupStreamsWithSource::<Runtime>::iter_key_prefix(account)
                    .collect::<Vec<_>>()
                    .tap_mut(|v| v.sort())
            };

            let lookup_target = |account| {
                LookupStreamsWithTarget::<Runtime>::iter_key_prefix(account)
                    .collect::<Vec<_>>()
                    .tap_mut(|v| v.sort())
            };

            assert_eq!(lookup_source(ALICE), &[0, 1, 3]);
            assert_eq!(lookup_source(BOB), &[2]);
            assert!(lookup_source(CHARLIE).is_empty());

            assert_eq!(lookup_target(ALICE), &[2]);
            assert_eq!(lookup_target(BOB), &[0, 3]);
            assert_eq!(lookup_target(CHARLIE), &[1]);

            assert_eq!(
                StreamPaymentAssets::get_deposit(&StreamPaymentAssetId::Native, &ALICE),
                (1 + 2 + 1) * MEGA
            );
            assert_eq!(
                StreamPaymentAssets::get_deposit(&StreamPaymentAssetId::Native, &BOB),
                3 * MEGA
            );
            assert_eq!(
                StreamPaymentAssets::get_deposit(&StreamPaymentAssetId::Native, &CHARLIE),
                0
            );
        })
    }
}

mod perform_payment {
    use super::*;

    #[test]
    fn cannot_update_unknown_stream() {
        ExtBuilder::default().build().execute_with(|| {
            assert_err!(
                StreamPayment::perform_payment(RuntimeOrigin::signed(ALICE), 0),
                Error::<Runtime>::UnknownStreamId
            );
        })
    }

    #[test]
    fn perform_payment_works() {
        ExtBuilder::default().build().execute_with(|| {
            let initial_deposit = 1 * MEGA;
            let config = StreamConfig {
                time_unit: TimeUnit::BlockNumber,
                asset_id: StreamPaymentAssetId::Native,
                rate: 100,
            };

            assert_ok!(StreamPayment::open_stream(
                RuntimeOrigin::signed(ALICE),
                BOB,
                config,
                initial_deposit
            ));

            assert_eq!(
                Balances::free_balance(ALICE),
                DEFAULT_BALANCE - initial_deposit
            );
            assert_eq!(
                StreamPaymentAssets::get_deposit(&StreamPaymentAssetId::Native, &ALICE),
                initial_deposit
            );

            let delta = roll_to(10) as u128;
            let payment = delta * config.rate;
            let deposit_left = initial_deposit - payment;

            assert_ok!(StreamPayment::perform_payment(
                // Anyone can dispatch an update.
                RuntimeOrigin::signed(CHARLIE),
                0
            ));

            assert_event_emitted!(Event::<Runtime>::StreamPayment {
                stream_id: 0,
                source: ALICE,
                target: BOB,
                amount: payment,
                drained: false
            });
            assert_eq!(
                Streams::<Runtime>::get(0),
                Some(Stream {
                    source: ALICE,
                    target: BOB,
                    config,
                    deposit: deposit_left,
                    last_time_updated: 10,
                    request_nonce: 0,
                    pending_request: None,
                })
            );

            assert_eq!(
                StreamPaymentAssets::get_deposit(&StreamPaymentAssetId::Native, &ALICE),
                deposit_left
            );

            assert_eq!(
                Balances::free_balance(ALICE),
                DEFAULT_BALANCE - initial_deposit
            );
            assert_eq!(Balances::free_balance(BOB), DEFAULT_BALANCE + payment);
        })
    }

    #[test]
    fn perform_payment_works_with_zero_rate() {
        ExtBuilder::default().build().execute_with(|| {
            let initial_deposit = 1 * MEGA;
            let config = StreamConfig {
                time_unit: TimeUnit::BlockNumber,
                asset_id: StreamPaymentAssetId::Native,
                rate: 0,
            };

            assert_ok!(StreamPayment::open_stream(
                RuntimeOrigin::signed(ALICE),
                BOB,
                config,
                initial_deposit
            ));

            assert_eq!(
                Balances::free_balance(ALICE),
                DEFAULT_BALANCE - initial_deposit
            );
            assert_eq!(
                StreamPaymentAssets::get_deposit(&StreamPaymentAssetId::Native, &ALICE),
                initial_deposit
            );

            roll_to(10);
            let payment = 0;
            let deposit_left = initial_deposit;

            assert_ok!(StreamPayment::perform_payment(
                // Anyone can dispatch an update.
                RuntimeOrigin::signed(CHARLIE),
                0
            ));

            // No event for payment of 0.
            assert_event_not_emitted!(Event::<Runtime>::StreamPayment {
                stream_id: 0,
                source: ALICE,
                target: BOB,
                amount: payment,
                drained: false
            });
            assert_eq!(
                Streams::<Runtime>::get(0),
                Some(Stream {
                    source: ALICE,
                    target: BOB,
                    config,
                    deposit: deposit_left,
                    // Time is updated correctly, which will prevent any issue
                    // when changing rate.
                    last_time_updated: 10,
                    request_nonce: 0,
                    pending_request: None,
                })
            );

            assert_eq!(
                StreamPaymentAssets::get_deposit(&StreamPaymentAssetId::Native, &ALICE),
                deposit_left
            );

            assert_eq!(
                Balances::free_balance(ALICE),
                DEFAULT_BALANCE - initial_deposit
            );
            assert_eq!(Balances::free_balance(BOB), DEFAULT_BALANCE + payment);
        })
    }

    #[test]
    fn perform_payment_works_with_max_rate() {
        ExtBuilder::default().build().execute_with(|| {
            let initial_deposit = 1 * MEGA;
            let config = StreamConfig {
                time_unit: TimeUnit::BlockNumber,
                asset_id: StreamPaymentAssetId::Native,
                rate: u128::MAX,
            };

            assert_ok!(StreamPayment::open_stream(
                RuntimeOrigin::signed(ALICE),
                BOB,
                config,
                initial_deposit
            ));

            assert_eq!(
                Balances::free_balance(ALICE),
                DEFAULT_BALANCE - initial_deposit
            );
            assert_eq!(
                StreamPaymentAssets::get_deposit(&StreamPaymentAssetId::Native, &ALICE),
                initial_deposit
            );

            roll_to(10);
            let payment = initial_deposit;
            let deposit_left = 0;

            assert_ok!(StreamPayment::perform_payment(
                // Anyone can dispatch an update.
                RuntimeOrigin::signed(CHARLIE),
                0
            ));

            assert_event_emitted!(Event::<Runtime>::StreamPayment {
                stream_id: 0,
                source: ALICE,
                target: BOB,
                amount: payment,
                drained: true
            });
            assert_eq!(
                Streams::<Runtime>::get(0),
                Some(Stream {
                    source: ALICE,
                    target: BOB,
                    config,
                    deposit: deposit_left,
                    last_time_updated: 10,
                    request_nonce: 0,
                    pending_request: None,
                })
            );

            assert_eq!(
                StreamPaymentAssets::get_deposit(&StreamPaymentAssetId::Native, &ALICE),
                deposit_left
            );

            assert_eq!(
                Balances::free_balance(ALICE),
                DEFAULT_BALANCE - initial_deposit
            );
            assert_eq!(Balances::free_balance(BOB), DEFAULT_BALANCE + payment);
        })
    }

    #[test]
    fn perform_payment_works_with_overflow() {
        ExtBuilder::default().build().execute_with(|| {
            let initial_deposit = 1 * MEGA;
            let config = StreamConfig {
                time_unit: TimeUnit::BlockNumber,
                asset_id: StreamPaymentAssetId::Native,
                rate: u128::MAX / 10,
            };

            assert_ok!(StreamPayment::open_stream(
                RuntimeOrigin::signed(ALICE),
                BOB,
                config,
                initial_deposit
            ));

            assert_eq!(
                Balances::free_balance(ALICE),
                DEFAULT_BALANCE - initial_deposit
            );
            assert_eq!(
                StreamPaymentAssets::get_deposit(&StreamPaymentAssetId::Native, &ALICE),
                initial_deposit
            );

            roll_to(20);
            let payment = initial_deposit;
            let deposit_left = 0;

            assert_ok!(StreamPayment::perform_payment(
                // Anyone can dispatch an update.
                RuntimeOrigin::signed(CHARLIE),
                0
            ));

            assert_event_emitted!(Event::<Runtime>::StreamPayment {
                stream_id: 0,
                source: ALICE,
                target: BOB,
                amount: payment,
                drained: true
            });
            assert_eq!(
                Streams::<Runtime>::get(0),
                Some(Stream {
                    source: ALICE,
                    target: BOB,
                    config,
                    deposit: deposit_left,
                    last_time_updated: 20,
                    request_nonce: 0,
                    pending_request: None,
                })
            );

            assert_eq!(
                StreamPaymentAssets::get_deposit(&StreamPaymentAssetId::Native, &ALICE),
                deposit_left
            );

            assert_eq!(
                Balances::free_balance(ALICE),
                DEFAULT_BALANCE - initial_deposit
            );
            assert_eq!(Balances::free_balance(BOB), DEFAULT_BALANCE + payment);
        })
    }

    #[test]
    fn payment_matching_deposit_is_considered_drained() {
        ExtBuilder::default().build().execute_with(|| {
            let config = StreamConfig {
                time_unit: TimeUnit::BlockNumber,
                asset_id: StreamPaymentAssetId::Native,
                rate: 100,
            };
            let initial_deposit = 9 * config.rate;

            assert_ok!(StreamPayment::open_stream(
                RuntimeOrigin::signed(ALICE),
                BOB,
                config,
                initial_deposit
            ));

            assert_eq!(
                Balances::free_balance(ALICE),
                DEFAULT_BALANCE - initial_deposit
            );
            assert_eq!(
                StreamPaymentAssets::get_deposit(&StreamPaymentAssetId::Native, &ALICE),
                initial_deposit
            );

            roll_to(10);
            let payment = initial_deposit;
            let deposit_left = 0;

            assert_ok!(StreamPayment::perform_payment(
                // Anyone can dispatch an update.
                RuntimeOrigin::signed(CHARLIE),
                0
            ));

            assert_event_emitted!(Event::<Runtime>::StreamPayment {
                stream_id: 0,
                source: ALICE,
                target: BOB,
                amount: payment,
                drained: true
            });
            assert_eq!(
                Streams::<Runtime>::get(0),
                Some(Stream {
                    source: ALICE,
                    target: BOB,
                    config,
                    deposit: deposit_left,
                    last_time_updated: 10,
                    request_nonce: 0,
                    pending_request: None,
                })
            );

            assert_eq!(
                StreamPaymentAssets::get_deposit(&StreamPaymentAssetId::Native, &ALICE),
                deposit_left
            );

            assert_eq!(
                Balances::free_balance(ALICE),
                DEFAULT_BALANCE - initial_deposit
            );
            assert_eq!(Balances::free_balance(BOB), DEFAULT_BALANCE + payment);
        })
    }

    #[test]
    fn perform_payment_works_alt_unit() {
        ExtBuilder::default().build().execute_with(|| {
            let initial_deposit = 1 * MEGA;
            let config = StreamConfig {
                time_unit: TimeUnit::Timestamp,
                asset_id: StreamPaymentAssetId::Native,
                rate: 100,
            };

            assert_ok!(StreamPayment::open_stream(
                RuntimeOrigin::signed(ALICE),
                BOB,
                config,
                initial_deposit
            ));

            assert_eq!(
                Balances::free_balance(ALICE),
                DEFAULT_BALANCE - initial_deposit
            );
            assert_eq!(
                StreamPaymentAssets::get_deposit(&StreamPaymentAssetId::Native, &ALICE),
                initial_deposit
            );

            let delta = roll_to(10) as u128;
            let payment = delta * config.rate * 12; // 12 sec per block
            let deposit_left = initial_deposit - payment;

            assert_ok!(StreamPayment::perform_payment(
                // Anyone can dispatch an update.
                RuntimeOrigin::signed(CHARLIE),
                0
            ));

            assert_event_emitted!(Event::<Runtime>::StreamPayment {
                stream_id: 0,
                source: ALICE,
                target: BOB,
                amount: payment,
                drained: false
            });
            assert_eq!(
                Streams::<Runtime>::get(0),
                Some(Stream {
                    source: ALICE,
                    target: BOB,
                    config,
                    deposit: deposit_left,
                    last_time_updated: 120,
                    request_nonce: 0,
                    pending_request: None,
                })
            );

            assert_eq!(
                StreamPaymentAssets::get_deposit(&StreamPaymentAssetId::Native, &ALICE),
                deposit_left
            );

            assert_eq!(
                Balances::free_balance(ALICE),
                DEFAULT_BALANCE - initial_deposit
            );
            assert_eq!(Balances::free_balance(BOB), DEFAULT_BALANCE + payment);
        })
    }

    #[test]
    fn protect_from_decreasing_time() {
        ExtBuilder::default().build().execute_with(|| {
            let rate = 100;
            let initial_deposit = 1 * MEGA;

            assert_ok!(StreamPayment::open_stream(
                RuntimeOrigin::signed(ALICE),
                BOB,
                StreamConfig {
                    time_unit: TimeUnit::Decreasing,
                    asset_id: StreamPaymentAssetId::Native,
                    rate,
                },
                initial_deposit
            ));

            assert_eq!(
                Balances::free_balance(ALICE),
                DEFAULT_BALANCE - initial_deposit
            );
            assert_eq!(
                StreamPaymentAssets::get_deposit(&StreamPaymentAssetId::Native, &ALICE),
                initial_deposit
            );

            roll_to(10);

            assert_err!(
                StreamPayment::perform_payment(RuntimeOrigin::signed(CHARLIE), 0),
                Error::<Runtime>::TimeMustBeIncreasing
            );
        })
    }
}

mod close_stream {
    use super::*;

    #[test]
    fn cannot_close_unknown_stream() {
        ExtBuilder::default().build().execute_with(|| {
            assert_err!(
                StreamPayment::close_stream(RuntimeOrigin::signed(ALICE), 0),
                Error::<Runtime>::UnknownStreamId
            );
        })
    }

    #[test]
    fn stream_cant_be_closed_by_third_party() {
        ExtBuilder::default().build().execute_with(|| {
            let rate = 100;
            let initial_deposit = 1 * MEGA;

            assert_ok!(StreamPayment::open_stream(
                RuntimeOrigin::signed(ALICE),
                BOB,
                StreamConfig {
                    time_unit: TimeUnit::BlockNumber,
                    asset_id: StreamPaymentAssetId::Native,
                    rate,
                },
                initial_deposit
            ));

            assert_eq!(
                Balances::free_balance(ALICE),
                DEFAULT_BALANCE - initial_deposit
            );
            assert_eq!(
                StreamPaymentAssets::get_deposit(&StreamPaymentAssetId::Native, &ALICE),
                initial_deposit
            );

            assert_err!(
                StreamPayment::close_stream(RuntimeOrigin::signed(CHARLIE), 0),
                Error::<Runtime>::UnauthorizedOrigin
            );
        })
    }

    #[test]
    fn stream_can_be_closed_by_source() {
        ExtBuilder::default().build().execute_with(|| {
            let rate = 100;
            let initial_deposit = 1 * MEGA;

            assert_ok!(StreamPayment::open_stream(
                RuntimeOrigin::signed(ALICE),
                BOB,
                StreamConfig {
                    time_unit: TimeUnit::BlockNumber,
                    asset_id: StreamPaymentAssetId::Native,
                    rate,
                },
                initial_deposit
            ));

            assert_eq!(
                Balances::free_balance(ALICE),
                DEFAULT_BALANCE - initial_deposit
            );
            assert_eq!(
                StreamPaymentAssets::get_deposit(&StreamPaymentAssetId::Native, &ALICE),
                initial_deposit
            );

            assert_ok!(StreamPayment::close_stream(RuntimeOrigin::signed(ALICE), 0),);
            assert_event_emitted!(Event::<Runtime>::StreamClosed {
                stream_id: 0,
                refunded: initial_deposit
            });
            assert_eq!(Streams::<Runtime>::get(0), None);
        })
    }

    #[test]
    fn stream_can_be_closed_by_target() {
        ExtBuilder::default().build().execute_with(|| {
            let rate = 100;
            let initial_deposit = 1 * MEGA;

            assert_ok!(StreamPayment::open_stream(
                RuntimeOrigin::signed(ALICE),
                BOB,
                StreamConfig {
                    time_unit: TimeUnit::BlockNumber,
                    asset_id: StreamPaymentAssetId::Native,
                    rate,
                },
                initial_deposit
            ));

            assert_eq!(
                Balances::free_balance(ALICE),
                DEFAULT_BALANCE - initial_deposit
            );
            assert_eq!(
                StreamPaymentAssets::get_deposit(&StreamPaymentAssetId::Native, &ALICE),
                initial_deposit
            );

            assert_ok!(StreamPayment::close_stream(RuntimeOrigin::signed(BOB), 0),);
            assert_event_emitted!(Event::<Runtime>::StreamClosed {
                stream_id: 0,
                refunded: initial_deposit
            });
            assert_eq!(Streams::<Runtime>::get(0), None);
        })
    }

    #[test]
    fn close_stream_with_payment() {
        ExtBuilder::default().build().execute_with(|| {
            let rate = 100;
            let initial_deposit = 1 * MEGA;

            assert_ok!(StreamPayment::open_stream(
                RuntimeOrigin::signed(ALICE),
                BOB,
                StreamConfig {
                    time_unit: TimeUnit::BlockNumber,
                    asset_id: StreamPaymentAssetId::Native,
                    rate,
                },
                initial_deposit
            ));

            assert_eq!(
                Balances::free_balance(ALICE),
                DEFAULT_BALANCE - initial_deposit
            );
            assert_eq!(
                StreamPaymentAssets::get_deposit(&StreamPaymentAssetId::Native, &ALICE),
                initial_deposit
            );

            let delta = roll_to(10) as u128;
            let payment = delta * rate;
            let deposit_left = initial_deposit - payment;

            assert_ok!(StreamPayment::close_stream(RuntimeOrigin::signed(ALICE), 0));

            assert_event_emitted!(Event::<Runtime>::StreamPayment {
                stream_id: 0,
                source: ALICE,
                target: BOB,
                amount: payment,
                drained: false
            });
            assert_event_emitted!(Event::<Runtime>::StreamClosed {
                stream_id: 0,
                refunded: deposit_left
            });
            assert_eq!(Streams::<Runtime>::get(0), None);

            assert_eq!(
                StreamPaymentAssets::get_deposit(&StreamPaymentAssetId::Native, &ALICE),
                0
            );

            assert_eq!(Balances::free_balance(ALICE), DEFAULT_BALANCE - payment);
            assert_eq!(Balances::free_balance(BOB), DEFAULT_BALANCE + payment);
        })
    }
}

mod change_deposit {
    use super::*;

    #[test]
    fn cannot_change_deposit_of_unknown_stream() {
        ExtBuilder::default().build().execute_with(|| {
            assert_err!(
                StreamPayment::change_deposit(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    DepositChange::Absolute(500)
                ),
                Error::<Runtime>::UnknownStreamId
            );
        })
    }

    #[test]
    fn third_party_cannot_change_deposit() {
        ExtBuilder::default().build().execute_with(|| {
            let rate = 100;
            let initial_deposit = 1 * MEGA;

            assert_ok!(StreamPayment::open_stream(
                RuntimeOrigin::signed(ALICE),
                BOB,
                StreamConfig {
                    time_unit: TimeUnit::BlockNumber,
                    asset_id: StreamPaymentAssetId::Native,
                    rate,
                },
                initial_deposit
            ));

            assert_err!(
                StreamPayment::change_deposit(
                    RuntimeOrigin::signed(CHARLIE),
                    0,
                    DepositChange::Absolute(500)
                ),
                Error::<Runtime>::UnauthorizedOrigin
            );
        })
    }

    #[test]
    fn target_cannot_change_deposit() {
        ExtBuilder::default().build().execute_with(|| {
            let rate = 100;
            let initial_deposit = 1 * MEGA;

            assert_ok!(StreamPayment::open_stream(
                RuntimeOrigin::signed(ALICE),
                BOB,
                StreamConfig {
                    time_unit: TimeUnit::BlockNumber,
                    asset_id: StreamPaymentAssetId::Native,
                    rate,
                },
                initial_deposit
            ));

            assert_err!(
                StreamPayment::change_deposit(
                    RuntimeOrigin::signed(BOB),
                    0,
                    DepositChange::Absolute(500)
                ),
                Error::<Runtime>::UnauthorizedOrigin
            );
        })
    }

    #[test]
    fn source_can_change_deposit_without_payment() {
        ExtBuilder::default().build().execute_with(|| {
            let rate = 100;
            let initial_deposit = 1 * MEGA;

            assert_ok!(StreamPayment::open_stream(
                RuntimeOrigin::signed(ALICE),
                BOB,
                StreamConfig {
                    time_unit: TimeUnit::BlockNumber,
                    asset_id: StreamPaymentAssetId::Native,
                    rate,
                },
                initial_deposit
            ));

            assert_ok!(StreamPayment::change_deposit(
                RuntimeOrigin::signed(ALICE),
                0,
                DepositChange::Absolute(initial_deposit * 2),
            ));

            assert_event_emitted!(Event::<Runtime>::StreamDepositChanged {
                stream_id: 0,
                new_deposit: 2 * initial_deposit
            });

            assert_eq!(
                StreamPaymentAssets::get_deposit(&StreamPaymentAssetId::Native, &ALICE),
                2 * initial_deposit
            );
        })
    }

    #[test]
    fn source_can_change_deposit_with_payment() {
        ExtBuilder::default().build().execute_with(|| {
            let rate = 100;
            let initial_deposit = 1 * MEGA;

            assert_ok!(StreamPayment::open_stream(
                RuntimeOrigin::signed(ALICE),
                BOB,
                StreamConfig {
                    time_unit: TimeUnit::BlockNumber,
                    asset_id: StreamPaymentAssetId::Native,
                    rate,
                },
                initial_deposit
            ));

            let delta = roll_to(10) as u128;
            let payment = delta * rate;

            let decrease = 2 * KILO;
            assert_ok!(StreamPayment::change_deposit(
                RuntimeOrigin::signed(ALICE),
                0,
                DepositChange::Decrease(decrease)
            ));

            assert_event_emitted!(Event::<Runtime>::StreamPayment {
                stream_id: 0,
                source: ALICE,
                target: BOB,
                amount: payment,
                drained: false
            });
            assert_event_emitted!(Event::<Runtime>::StreamDepositChanged {
                stream_id: 0,
                new_deposit: initial_deposit - payment - decrease,
            });

            assert_eq!(
                StreamPaymentAssets::get_deposit(&StreamPaymentAssetId::Native, &ALICE),
                initial_deposit - payment - decrease
            );

            assert_eq!(
                Balances::free_balance(ALICE),
                DEFAULT_BALANCE - initial_deposit + decrease
            );
            assert_eq!(Balances::free_balance(BOB), DEFAULT_BALANCE + payment);
        })
    }

    #[test]
    fn source_can_change_deposit_with_payment_not_retroactive() {
        ExtBuilder::default().build().execute_with(|| {
            let rate = 100 * KILO;
            let initial_deposit = 1 * MEGA;

            assert_ok!(StreamPayment::open_stream(
                RuntimeOrigin::signed(ALICE),
                BOB,
                StreamConfig {
                    time_unit: TimeUnit::BlockNumber,
                    asset_id: StreamPaymentAssetId::Native,
                    rate,
                },
                initial_deposit
            ));

            let delta = roll_to(15) as u128;
            let payment = delta * rate;
            assert!(payment > initial_deposit);

            let increase = 300 * KILO;

            assert_ok!(StreamPayment::change_deposit(
                RuntimeOrigin::signed(ALICE),
                0,
                DepositChange::Increase(increase)
            ));

            assert_event_emitted!(Event::<Runtime>::StreamPayment {
                stream_id: 0,
                source: ALICE,
                target: BOB,
                amount: initial_deposit,
                drained: true
            });
            assert_event_emitted!(Event::<Runtime>::StreamDepositChanged {
                stream_id: 0,
                new_deposit: increase, // stream got drained, so there is only the increase
            });

            assert_eq!(
                StreamPaymentAssets::get_deposit(&StreamPaymentAssetId::Native, &ALICE),
                increase
            );

            assert_eq!(
                Balances::free_balance(ALICE),
                DEFAULT_BALANCE - initial_deposit - increase
            );
            assert_eq!(
                Balances::free_balance(BOB),
                DEFAULT_BALANCE + initial_deposit
            );
        })
    }
}

mod request_change {
    use super::*;

    #[test]
    fn cannot_change_rate_of_unknown_stream() {
        ExtBuilder::default().build().execute_with(|| {
            let config = StreamConfig {
                time_unit: TimeUnit::BlockNumber,
                asset_id: StreamPaymentAssetId::Native,
                rate: 100,
            };
            assert_err!(
                StreamPayment::request_change(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    ChangeKind::Suggestion,
                    config
                ),
                Error::<Runtime>::UnknownStreamId
            );
        })
    }

    #[test]
    fn third_party_cannot_change_rate() {
        ExtBuilder::default().build().execute_with(|| {
            let config = StreamConfig {
                time_unit: TimeUnit::BlockNumber,
                asset_id: StreamPaymentAssetId::Native,
                rate: 100,
            };
            let initial_deposit = 1 * MEGA;

            assert_ok!(StreamPayment::open_stream(
                RuntimeOrigin::signed(ALICE),
                BOB,
                config,
                initial_deposit
            ));

            assert_err!(
                StreamPayment::request_change(
                    RuntimeOrigin::signed(CHARLIE),
                    0,
                    ChangeKind::Suggestion,
                    config
                ),
                Error::<Runtime>::UnauthorizedOrigin
            );
        })
    }

    #[test]
    fn source_can_immediately_increase_rate() {
        ExtBuilder::default().build().execute_with(|| {
            let config = StreamConfig {
                time_unit: TimeUnit::BlockNumber,
                asset_id: StreamPaymentAssetId::Native,
                rate: 100,
            };
            let new_config = StreamConfig {
                rate: 101,
                ..config
            };
            let initial_deposit = 1 * MEGA;

            assert_ok!(StreamPayment::open_stream(
                RuntimeOrigin::signed(ALICE),
                BOB,
                config,
                initial_deposit
            ));

            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(ALICE),
                0,
                ChangeKind::Suggestion,
                new_config
            ),);

            assert_event_emitted!(Event::<Runtime>::StreamConfigChanged {
                stream_id: 0,
                old_config: config,
                new_config
            });
        })
    }

    #[test]
    fn request_same_config_is_noop() {
        ExtBuilder::default().build().execute_with(|| {
            let config = StreamConfig {
                time_unit: TimeUnit::BlockNumber,
                asset_id: StreamPaymentAssetId::Native,
                rate: 100,
            };
            let initial_deposit = 1 * MEGA;

            assert_ok!(StreamPayment::open_stream(
                RuntimeOrigin::signed(ALICE),
                BOB,
                config,
                initial_deposit
            ));

            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(ALICE),
                0,
                ChangeKind::Suggestion,
                config,
            ),);

            assert_event_not_emitted!(Event::<Runtime>::StreamConfigChanged {
                stream_id: 0,
                old_config: config,
                new_config: config
            });
        })
    }

    #[test]
    fn target_can_immediately_decrease_rate() {
        ExtBuilder::default().build().execute_with(|| {
            let config = StreamConfig {
                time_unit: TimeUnit::BlockNumber,
                asset_id: StreamPaymentAssetId::Native,
                rate: 100,
            };
            let new_config = StreamConfig { rate: 99, ..config };
            let initial_deposit = 1 * MEGA;

            assert_ok!(StreamPayment::open_stream(
                RuntimeOrigin::signed(ALICE),
                BOB,
                config,
                initial_deposit
            ));

            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(BOB),
                0,
                ChangeKind::Suggestion,
                new_config
            ),);

            assert_event_emitted!(Event::<Runtime>::StreamConfigChanged {
                stream_id: 0,
                old_config: config,
                new_config
            });
        })
    }

    #[test]
    fn override_cannot_trigger_retroactive_payment() {
        ExtBuilder::default().build().execute_with(|| {
            // Initial stream config
            let config = StreamConfig {
                time_unit: TimeUnit::BlockNumber,
                asset_id: StreamPaymentAssetId::Native,
                rate: 100,
            };

            let initial_deposit = 1 * MEGA;

            assert_ok!(StreamPayment::open_stream(
                RuntimeOrigin::signed(ALICE),
                BOB,
                config,
                initial_deposit
            ));

            // Target requets a change.
            let change1 = StreamConfig {
                rate: 101,
                ..config
            };
            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(BOB),
                0,
                ChangeKind::Mandatory { deadline: 10 },
                change1,
            ));

            // Roll to block after deadline, payment should stop at deadline.
            let delta = roll_to(11) as u128;
            let payment = (delta - 1) * config.rate;

            assert_ok!(StreamPayment::perform_payment(
                RuntimeOrigin::signed(CHARLIE),
                0
            ));
            assert_event_emitted!(Event::<Runtime>::StreamPayment {
                stream_id: 0,
                source: ALICE,
                target: BOB,
                amount: payment,
                drained: false
            });

            // Target requets a new change that moves the deadline.
            let change1 = StreamConfig {
                rate: 102,
                ..config
            };
            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(BOB),
                0,
                ChangeKind::Mandatory { deadline: 20 },
                change1,
            ));

            let deposit_before = Streams::<Runtime>::get(0).unwrap().deposit;
            assert_ok!(StreamPayment::perform_payment(
                RuntimeOrigin::signed(CHARLIE),
                0
            ));
            let deposit_after = Streams::<Runtime>::get(0).unwrap().deposit;

            assert_eq!(
                deposit_before, deposit_after,
                "no payment should be performed"
            );
        })
    }

    #[test]
    fn source_can_override_target_suggestion() {
        ExtBuilder::default().build().execute_with(|| {
            // Initial stream config
            let config = StreamConfig {
                time_unit: TimeUnit::BlockNumber,
                asset_id: StreamPaymentAssetId::Native,
                rate: 100,
            };

            let initial_deposit = 1 * MEGA;

            assert_ok!(StreamPayment::open_stream(
                RuntimeOrigin::signed(ALICE),
                BOB,
                config,
                initial_deposit
            ));

            // Target requests a change.
            let change1 = StreamConfig {
                rate: 101,
                ..config
            };

            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(BOB),
                0,
                ChangeKind::Suggestion,
                change1,
            ));

            assert_event_emitted!(Event::<Runtime>::StreamConfigChangeRequested {
                stream_id: 0,
                request_nonce: 1,
                old_config: config,
                new_config: change1,
            });

            // Source override the request
            let change2 = StreamConfig {
                time_unit: TimeUnit::Timestamp,
                ..config
            };

            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(ALICE),
                0,
                ChangeKind::Suggestion,
                change2,
            ));

            assert_event_emitted!(Event::<Runtime>::StreamConfigChangeRequested {
                stream_id: 0,
                request_nonce: 2,
                old_config: config,
                new_config: change2,
            });
        })
    }

    #[test]
    fn target_can_override_source_suggestion() {
        ExtBuilder::default().build().execute_with(|| {
            // Initial stream config
            let config = StreamConfig {
                time_unit: TimeUnit::BlockNumber,
                asset_id: StreamPaymentAssetId::Native,
                rate: 100,
            };

            let initial_deposit = 1 * MEGA;

            assert_ok!(StreamPayment::open_stream(
                RuntimeOrigin::signed(ALICE),
                BOB,
                config,
                initial_deposit
            ));

            // Source requests a change.
            let change1 = StreamConfig { rate: 99, ..config };

            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(ALICE),
                0,
                ChangeKind::Suggestion,
                change1,
            ));

            assert_event_emitted!(Event::<Runtime>::StreamConfigChangeRequested {
                stream_id: 0,
                request_nonce: 1,
                old_config: config,
                new_config: change1,
            });

            // Target override the request
            let change2 = StreamConfig {
                time_unit: TimeUnit::Timestamp,
                ..config
            };

            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(BOB),
                0,
                ChangeKind::Suggestion,
                change2,
            ));

            assert_event_emitted!(Event::<Runtime>::StreamConfigChangeRequested {
                stream_id: 0,
                request_nonce: 2,
                old_config: config,
                new_config: change2,
            });
        })
    }

    #[test]
    fn source_cant_override_target_mandatory_request() {
        ExtBuilder::default().build().execute_with(|| {
            // Initial stream config
            let config = StreamConfig {
                time_unit: TimeUnit::BlockNumber,
                asset_id: StreamPaymentAssetId::Native,
                rate: 100,
            };

            let initial_deposit = 1 * MEGA;

            assert_ok!(StreamPayment::open_stream(
                RuntimeOrigin::signed(ALICE),
                BOB,
                config,
                initial_deposit
            ));

            // Target requests a change.
            let change1 = StreamConfig {
                rate: 101,
                ..config
            };

            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(BOB),
                0,
                ChangeKind::Mandatory { deadline: 10 },
                change1,
            ));

            assert_event_emitted!(Event::<Runtime>::StreamConfigChangeRequested {
                stream_id: 0,
                request_nonce: 1,
                old_config: config,
                new_config: change1,
            });

            // Source tries to override the request
            let change2 = StreamConfig {
                time_unit: TimeUnit::Timestamp,
                ..config
            };

            assert_err!(
                StreamPayment::request_change(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    ChangeKind::Suggestion,
                    change2,
                ),
                Error::<Runtime>::CantOverrideMandatoryChange
            );

            assert_event_not_emitted!(Event::<Runtime>::StreamConfigChangeRequested {
                stream_id: 0,
                request_nonce: 2,
                old_config: config,
                new_config: change2,
            });
        })
    }

    #[test]
    fn target_cant_override_source_mandatory_request() {
        ExtBuilder::default().build().execute_with(|| {
            // Initial stream config
            let config = StreamConfig {
                time_unit: TimeUnit::BlockNumber,
                asset_id: StreamPaymentAssetId::Native,
                rate: 100,
            };

            let initial_deposit = 1 * MEGA;

            assert_ok!(StreamPayment::open_stream(
                RuntimeOrigin::signed(ALICE),
                BOB,
                config,
                initial_deposit
            ));

            // Source requests a change.
            let change1 = StreamConfig { rate: 99, ..config };

            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(ALICE),
                0,
                ChangeKind::Mandatory { deadline: 10 },
                change1,
            ));

            assert_event_emitted!(Event::<Runtime>::StreamConfigChangeRequested {
                stream_id: 0,
                request_nonce: 1,
                old_config: config,
                new_config: change1,
            });

            // Target tries to override the request
            let change2 = StreamConfig {
                time_unit: TimeUnit::Timestamp,
                ..config
            };

            assert_err!(
                StreamPayment::request_change(
                    RuntimeOrigin::signed(BOB),
                    0,
                    ChangeKind::Suggestion,
                    change2,
                ),
                Error::<Runtime>::CantOverrideMandatoryChange
            );

            assert_event_not_emitted!(Event::<Runtime>::StreamConfigChangeRequested {
                stream_id: 0,
                request_nonce: 2,
                old_config: config,
                new_config: change2,
            });
        })
    }
}
