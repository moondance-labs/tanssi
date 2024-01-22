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
        assert_event_emitted,
        mock::{
            roll_to, Balances, ExtBuilder, Runtime, RuntimeOrigin, StreamPayment,
            StreamPaymentAssetId, TimeUnit, ALICE, BOB, CHARLIE, DEFAULT_BALANCE, KILO, MEGA,
        },
        Error, Event, FreezeReason, LookupStreamsWithSource, LookupStreamsWithTarget, NextStreamId,
        Stream, StreamConfig, Streams,
    },
    frame_support::{assert_err, assert_ok, traits::fungible::InspectFreeze},
    sp_runtime::TokenError,
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
                Balances::balance_frozen(&FreezeReason::StreamPayment.into(), &ALICE),
                1 * MEGA
            );
            assert_eq!(
                Balances::balance_frozen(&FreezeReason::StreamPayment.into(), &BOB),
                0
            );
        })
    }
}

mod update_stream {

    use super::*;

    #[test]
    fn cannot_update_unknown_stream() {
        ExtBuilder::default().build().execute_with(|| {
            assert_err!(
                StreamPayment::update_stream(RuntimeOrigin::signed(ALICE), 0),
                Error::<Runtime>::UnknownStreamId
            );
        })
    }

    #[test]
    fn update_stream_works() {
        ExtBuilder::default().build().execute_with(|| {
            let rate = 100;
            let initial_deposit = 1 * MEGA;

            assert_ok!(StreamPayment::open_stream(
                RuntimeOrigin::signed(ALICE),
                BOB,
                StreamConfig {
                    time_unit: TimeUnit::BlockNumber,
                    asset_id: StreamPaymentAssetId::Native,
                    rate: rate,
                },
                initial_deposit
            ));

            assert_eq!(Balances::free_balance(&ALICE), DEFAULT_BALANCE);
            assert_eq!(
                Balances::balance_frozen(&FreezeReason::StreamPayment.into(), &ALICE),
                initial_deposit
            );

            let delta = roll_to(10) as u128;
            let payment = delta * rate;
            let deposit_left = initial_deposit - payment;

            assert_ok!(StreamPayment::update_stream(
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
                    config: StreamConfig {
                        time_unit: TimeUnit::BlockNumber,
                        asset_id: StreamPaymentAssetId::Native,
                        rate: rate,
                    },
                    deposit: deposit_left,
                    last_time_updated: 10,
                    request_nonce: 0,
                    pending_request: None,
                })
            );

            assert_eq!(
                Balances::balance_frozen(&FreezeReason::StreamPayment.into(), &ALICE),
                deposit_left
            );

            assert_eq!(Balances::free_balance(ALICE), DEFAULT_BALANCE - payment);
            assert_eq!(Balances::free_balance(BOB), DEFAULT_BALANCE + payment);
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
                    rate: rate,
                },
                initial_deposit
            ));

            assert_eq!(Balances::free_balance(&ALICE), DEFAULT_BALANCE);
            assert_eq!(
                Balances::balance_frozen(&FreezeReason::StreamPayment.into(), &ALICE),
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
                    rate: rate,
                },
                initial_deposit
            ));

            assert_eq!(Balances::free_balance(&ALICE), DEFAULT_BALANCE);
            assert_eq!(
                Balances::balance_frozen(&FreezeReason::StreamPayment.into(), &ALICE),
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
                    rate: rate,
                },
                initial_deposit
            ));

            assert_eq!(Balances::free_balance(&ALICE), DEFAULT_BALANCE);
            assert_eq!(
                Balances::balance_frozen(&FreezeReason::StreamPayment.into(), &ALICE),
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

            assert_eq!(Balances::free_balance(&ALICE), DEFAULT_BALANCE);
            assert_eq!(
                Balances::balance_frozen(&FreezeReason::StreamPayment.into(), &ALICE),
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
                Balances::balance_frozen(&FreezeReason::StreamPayment.into(), &ALICE),
                0
            );

            assert_eq!(Balances::free_balance(ALICE), DEFAULT_BALANCE - payment);
            assert_eq!(Balances::free_balance(BOB), DEFAULT_BALANCE + payment);
        })
    }
}

mod refill_stream {
    use super::*;

    #[test]
    fn cannot_refill_unknown_stream() {
        ExtBuilder::default().build().execute_with(|| {
            assert_err!(
                StreamPayment::refill_stream(RuntimeOrigin::signed(ALICE), 0, 500),
                Error::<Runtime>::UnknownStreamId
            );
        })
    }

    #[test]
    fn third_party_cannot_refill() {
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
                StreamPayment::refill_stream(RuntimeOrigin::signed(CHARLIE), 0, initial_deposit),
                Error::<Runtime>::UnauthorizedOrigin
            );
        })
    }

    #[test]
    fn target_cannot_refill() {
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
                StreamPayment::refill_stream(RuntimeOrigin::signed(BOB), 0, initial_deposit),
                Error::<Runtime>::UnauthorizedOrigin
            );
        })
    }

    #[test]
    fn source_can_refill_without_payment() {
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

            assert_ok!(StreamPayment::refill_stream(
                RuntimeOrigin::signed(ALICE),
                0,
                initial_deposit
            ));

            assert_event_emitted!(Event::<Runtime>::StreamRefilled {
                stream_id: 0,
                increase: initial_deposit,
                new_deposit: 2 * initial_deposit
            });

            assert_eq!(
                Balances::balance_frozen(&FreezeReason::StreamPayment.into(), &ALICE),
                2 * initial_deposit
            );
        })
    }

    #[test]
    fn source_can_refill_with_payment() {
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

            assert_ok!(StreamPayment::refill_stream(
                RuntimeOrigin::signed(ALICE),
                0,
                initial_deposit
            ));

            assert_event_emitted!(Event::<Runtime>::StreamPayment {
                stream_id: 0,
                source: ALICE,
                target: BOB,
                amount: payment,
                drained: false
            });
            assert_event_emitted!(Event::<Runtime>::StreamRefilled {
                stream_id: 0,
                increase: initial_deposit,
                new_deposit: 2 * initial_deposit - payment
            });

            assert_eq!(
                Balances::balance_frozen(&FreezeReason::StreamPayment.into(), &ALICE),
                2 * initial_deposit - payment
            );

            assert_eq!(Balances::free_balance(ALICE), DEFAULT_BALANCE - payment);
            assert_eq!(Balances::free_balance(BOB), DEFAULT_BALANCE + payment);
        })
    }

    #[test]
    fn source_can_refill_with_payment_not_retroactive() {
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

            let new_deposit = 500 * KILO;

            assert_ok!(StreamPayment::refill_stream(
                RuntimeOrigin::signed(ALICE),
                0,
                new_deposit
            ));

            assert_event_emitted!(Event::<Runtime>::StreamPayment {
                stream_id: 0,
                source: ALICE,
                target: BOB,
                amount: initial_deposit,
                drained: true
            });
            assert_event_emitted!(Event::<Runtime>::StreamRefilled {
                stream_id: 0,
                increase: new_deposit,
                new_deposit: new_deposit
            });

            assert_eq!(
                Balances::balance_frozen(&FreezeReason::StreamPayment.into(), &ALICE),
                new_deposit
            );

            assert_eq!(
                Balances::free_balance(ALICE),
                DEFAULT_BALANCE - initial_deposit
            );
            assert_eq!(
                Balances::free_balance(BOB),
                DEFAULT_BALANCE + initial_deposit
            );
        })
    }
}

mod change_stream_rate {
    use super::*;

    #[test]
    fn cannot_change_rate_of_unknown_stream() {
        ExtBuilder::default().build().execute_with(|| {
            assert_err!(
                StreamPayment::change_stream_rate(RuntimeOrigin::signed(ALICE), 0, 500),
                Error::<Runtime>::UnknownStreamId
            );
        })
    }

    #[test]
    fn third_party_cannot_change_rate() {
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
                StreamPayment::change_stream_rate(RuntimeOrigin::signed(CHARLIE), 0, rate),
                Error::<Runtime>::UnauthorizedOrigin
            );
        })
    }

    #[test]
    fn source_must_increase_rate() {
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
                StreamPayment::change_stream_rate(RuntimeOrigin::signed(ALICE), 0, rate - 1),
                Error::<Runtime>::SourceCantDecreaseRate
            );
        })
    }

    #[test]
    fn target_must_decrease_rate() {
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
                StreamPayment::change_stream_rate(RuntimeOrigin::signed(BOB), 0, rate + 1),
                Error::<Runtime>::TargetCantIncreaseRate
            );
        })
    }

    #[test]
    fn source_can_increase_rate() {
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

            assert_ok!(StreamPayment::change_stream_rate(
                RuntimeOrigin::signed(ALICE),
                0,
                rate * 2
            ),);

            assert_event_emitted!(Event::<Runtime>::StreamPayment {
                stream_id: 0,
                source: ALICE,
                target: BOB,
                amount: payment,
                drained: false
            });
            assert_event_emitted!(Event::<Runtime>::StreamRateChanged {
                stream_id: 0,
                old_rate: rate,
                new_rate: rate * 2
            });

            assert_eq!(
                Balances::balance_frozen(&FreezeReason::StreamPayment.into(), &ALICE),
                initial_deposit - payment
            );

            assert_eq!(Balances::free_balance(ALICE), DEFAULT_BALANCE - payment);
            assert_eq!(Balances::free_balance(BOB), DEFAULT_BALANCE + payment);

            assert_eq!(
                Streams::<Runtime>::get(0),
                Some(Stream {
                    source: ALICE,
                    target: BOB,
                    config: StreamConfig {
                        time_unit: TimeUnit::BlockNumber,
                        asset_id: StreamPaymentAssetId::Native,
                        rate: rate * 2,
                    },

                    deposit: initial_deposit - payment,
                    last_time_updated: 10,
                    request_nonce: 0,
                    pending_request: None,
                })
            );
        })
    }

    #[test]
    fn target_can_decrease_rate() {
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

            assert_ok!(StreamPayment::change_stream_rate(
                RuntimeOrigin::signed(BOB),
                0,
                rate / 2
            ),);

            assert_event_emitted!(Event::<Runtime>::StreamPayment {
                stream_id: 0,
                source: ALICE,
                target: BOB,
                amount: payment,
                drained: false
            });
            assert_event_emitted!(Event::<Runtime>::StreamRateChanged {
                stream_id: 0,
                old_rate: rate,
                new_rate: rate / 2
            });

            assert_eq!(
                Balances::balance_frozen(&FreezeReason::StreamPayment.into(), &ALICE),
                initial_deposit - payment
            );

            assert_eq!(Balances::free_balance(ALICE), DEFAULT_BALANCE - payment);
            assert_eq!(Balances::free_balance(BOB), DEFAULT_BALANCE + payment);

            assert_eq!(
                Streams::<Runtime>::get(0),
                Some(Stream {
                    source: ALICE,
                    target: BOB,
                    config: StreamConfig {
                        time_unit: TimeUnit::BlockNumber,
                        asset_id: StreamPaymentAssetId::Native,
                        rate: rate / 2,
                    },
                    deposit: initial_deposit - payment,
                    last_time_updated: 10,
                    request_nonce: 0,
                    pending_request: None,
                })
            );
        })
    }
}
