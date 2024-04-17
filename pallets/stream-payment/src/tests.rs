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
            roll_to, AccountId, Balance, Balances, ExtBuilder, OpenStreamHoldAmount, Runtime,
            RuntimeOrigin, StreamPayment, StreamPaymentAssetId, StreamPaymentAssets, TimeUnit,
            ALICE, BOB, CHARLIE, DEFAULT_BALANCE, MEGA,
        },
        ArithmeticError, Assets, ChangeKind, DepositChange, DispatchResultWithPostInfo, Event,
        LookupStreamsWithSource, LookupStreamsWithTarget, NextStreamId, Party, Stream,
        StreamConfig, StreamConfigOf, StreamOf, Streams,
    },
    frame_support::{assert_err, assert_ok},
    sp_runtime::TokenError,
    tap::tap::Tap,
};

macro_rules! assert_balance_change {
    ( +, $account:expr, $amount:expr) => {
        assert_eq!(Balances::free_balance($account), DEFAULT_BALANCE + $amount,);
    };
    ( -, $account:expr, $amount:expr) => {
        assert_eq!(Balances::free_balance($account), DEFAULT_BALANCE - $amount,);
    };
}

fn default<T: Default>() -> T {
    Default::default()
}

fn default_config() -> StreamConfigOf<Runtime> {
    StreamConfig {
        time_unit: TimeUnit::BlockNumber,
        asset_id: StreamPaymentAssetId::Native,
        rate: 100,
    }
}

fn default_stream() -> StreamOf<Runtime> {
    Stream {
        source: ALICE,
        target: BOB,
        config: default_config(),
        deposit: 0u32.into(),
        opening_deposit: OpenStreamHoldAmount::get(),
        last_time_updated: 0u32.into(),
        request_nonce: 0,
        pending_request: None,
    }
}

fn get_deposit(account: AccountId) -> Balance {
    StreamPaymentAssets::get_deposit(&StreamPaymentAssetId::Native, &account)
}

type Error = crate::Error<Runtime>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct OpenStream {
    from: AccountId,
    to: AccountId,
    config: StreamConfigOf<Runtime>,
    deposit: Balance,
}

impl Default for OpenStream {
    fn default() -> Self {
        Self {
            from: ALICE,
            to: BOB,
            config: default_config(),
            deposit: 1 * MEGA,
        }
    }
}

impl OpenStream {
    fn call(&self) -> DispatchResultWithPostInfo {
        StreamPayment::open_stream(
            RuntimeOrigin::signed(self.from),
            self.to,
            self.config,
            self.deposit,
        )
    }
}

struct PaymentEvent {
    stream_id: u64,
    source: AccountId,
    target: AccountId,
    amount: Balance,
    stalled: bool,
}

impl Default for PaymentEvent {
    fn default() -> Self {
        Self {
            stream_id: 0,
            source: ALICE,
            target: BOB,
            amount: 0,
            stalled: false,
        }
    }
}

impl From<PaymentEvent> for Event<Runtime> {
    fn from(e: PaymentEvent) -> Event<Runtime> {
        let PaymentEvent {
            stream_id,
            source,
            target,
            amount,
            stalled,
        } = e;
        Self::StreamPayment {
            stream_id,
            source,
            target,
            amount,
            stalled,
        }
    }
}

mod open_stream {
    use super::*;

    #[test]
    fn cant_be_both_source_and_target() {
        ExtBuilder::default().build().execute_with(|| {
            assert_err!(
                OpenStream {
                    from: ALICE,
                    to: ALICE,
                    ..default()
                }
                .call(),
                Error::CantBeBothSourceAndTarget
            );
        })
    }

    #[test]
    fn stream_id_cannot_overflow() {
        ExtBuilder::default().build().execute_with(|| {
            NextStreamId::<Runtime>::set(u64::MAX);

            assert_err!(OpenStream::default().call(), Error::StreamIdOverflow);
        })
    }

    #[test]
    fn balance_too_low_for_deposit() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_000_000)])
            .build()
            .execute_with(|| {
                assert_err!(
                    OpenStream {
                        from: ALICE,
                        deposit: 1_000_001,
                        ..default()
                    }
                    .call(),
                    TokenError::FundsUnavailable,
                );
            })
    }

    #[test]
    fn time_can_be_fetched() {
        ExtBuilder::default().build().execute_with(|| {
            assert_err!(
                OpenStream {
                    config: StreamConfig {
                        time_unit: TimeUnit::Never,
                        ..default_config()
                    },
                    ..default()
                }
                .call(),
                Error::CantFetchCurrentTime,
            );
        })
    }

    #[test]
    fn stream_opened() {
        ExtBuilder::default().build().execute_with(|| {
            assert!(Streams::<Runtime>::get(0).is_none());

            let open_stream = OpenStream::default();
            assert_ok!(open_stream.call());

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

            assert_eq!(get_deposit(ALICE), open_stream.deposit);
            assert_eq!(get_deposit(BOB), 0);

            assert_eq!(
                Streams::<Runtime>::get(0),
                Some(Stream {
                    deposit: open_stream.deposit,
                    last_time_updated: 1, // tests starts in block 1
                    ..default_stream()
                })
            );
        })
    }

    #[test]
    fn multiple_streams_opened() {
        ExtBuilder::default().build().execute_with(|| {
            assert!(Streams::<Runtime>::get(0).is_none());

            let open_streams = [
                OpenStream {
                    from: ALICE,
                    to: BOB,
                    deposit: 1 * MEGA,
                    config: StreamConfig {
                        time_unit: TimeUnit::BlockNumber,
                        rate: 100,
                        ..default_config()
                    },
                },
                OpenStream {
                    from: ALICE,
                    to: CHARLIE,
                    deposit: 2 * MEGA,
                    config: StreamConfig {
                        time_unit: TimeUnit::Timestamp,
                        rate: 500,
                        ..default_config()
                    },
                },
                OpenStream {
                    from: BOB,
                    to: ALICE,
                    deposit: 3 * MEGA,
                    config: StreamConfig {
                        time_unit: TimeUnit::Timestamp,
                        rate: 200,
                        ..default_config()
                    },
                },
                OpenStream {
                    from: ALICE,
                    to: BOB,
                    deposit: 1 * MEGA,
                    config: StreamConfig {
                        time_unit: TimeUnit::BlockNumber,
                        rate: 300,
                        ..default_config()
                    },
                },
            ];

            for s in &open_streams {
                assert_ok!(s.call());
            }

            for (i, s) in open_streams.iter().enumerate() {
                assert_event_emitted!(Event::<Runtime>::StreamOpened {
                    stream_id: i as u64
                });
                assert_eq!(
                    Streams::<Runtime>::get(i as u64),
                    Some(Stream {
                        source: s.from,
                        target: s.to,
                        deposit: s.deposit,
                        config: s.config,
                        // Tests are run on 1st block with timestamp 12.
                        last_time_updated: match s.config.time_unit {
                            TimeUnit::BlockNumber => 1,
                            TimeUnit::Timestamp => 12,
                            _ => unreachable!("not used in test"),
                        },
                        ..default_stream()
                    })
                )
            }
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

            assert_eq!(get_deposit(ALICE), (1 + 2 + 1) * MEGA);
            assert_eq!(get_deposit(BOB), 3 * MEGA);
            assert_eq!(get_deposit(CHARLIE), 0);
        })
    }

    #[test]
    fn balance_too_low_for_storage_hold() {
        ExtBuilder::default()
            // ED 1 + deposit 1_000_000 + storage hold - 1
            .with_balances(vec![(ALICE, 1_000_000 + OpenStreamHoldAmount::get())])
            .build()
            .execute_with(|| {
                assert_err!(
                    OpenStream {
                        from: ALICE,
                        deposit: 1_000_000,
                        ..default()
                    }
                    .call(),
                    TokenError::FundsUnavailable,
                );
            })
    }

    #[test]
    fn balance_enough_for_storage_hold() {
        ExtBuilder::default()
            // ED 1 + deposit 1_000_000 + storage hold 100
            .with_balances(vec![(ALICE, 1_000_001 + OpenStreamHoldAmount::get())])
            .build()
            .execute_with(|| {
                assert_ok!(OpenStream {
                    from: ALICE,
                    deposit: 1_000_000,
                    ..default()
                }
                .call(),);
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
                Error::UnknownStreamId
            );
        })
    }

    #[test]
    fn perform_payment_works() {
        ExtBuilder::default().build().execute_with(|| {
            let opening_deposit = OpenStreamHoldAmount::get();
            let open_stream = OpenStream::default();
            assert_ok!(open_stream.call());

            assert_balance_change!(-, ALICE, open_stream.deposit + opening_deposit);
            assert_eq!(get_deposit(ALICE), open_stream.deposit);

            let delta = u128::from(roll_to(10));
            let payment = delta * open_stream.config.rate;
            let deposit_left = open_stream.deposit - payment;

            assert_ok!(StreamPayment::perform_payment(
                // Anyone can dispatch an update.
                RuntimeOrigin::signed(CHARLIE),
                0
            ));

            assert_event_emitted!(PaymentEvent {
                amount: payment,
                ..default()
            });

            assert_eq!(
                Streams::<Runtime>::get(0),
                Some(Stream {
                    deposit: deposit_left,
                    last_time_updated: 10,
                    ..default_stream()
                })
            );

            assert_eq!(get_deposit(ALICE), deposit_left);
            assert_balance_change!(-, ALICE, open_stream.deposit + opening_deposit);
            assert_eq!(Balances::free_balance(BOB), DEFAULT_BALANCE + payment);
        })
    }

    #[test]
    fn perform_payment_works_with_zero_rate() {
        ExtBuilder::default().build().execute_with(|| {
            let opening_deposit = OpenStreamHoldAmount::get();
            let open_stream = OpenStream {
                config: StreamConfig {
                    rate: 0,
                    ..default_config()
                },
                ..default()
            };
            assert_ok!(open_stream.call());

            assert_balance_change!(-, ALICE, open_stream.deposit + opening_deposit);
            assert_eq!(get_deposit(ALICE), open_stream.deposit);

            roll_to(10);
            let payment = 0;
            let deposit_left = open_stream.deposit;

            assert_ok!(StreamPayment::perform_payment(
                // Anyone can dispatch an update.
                RuntimeOrigin::signed(CHARLIE),
                0
            ));

            // No event for payment of 0.
            assert_event_not_emitted!(PaymentEvent {
                amount: payment,
                ..default()
            });

            assert_eq!(
                Streams::<Runtime>::get(0),
                Some(Stream {
                    config: open_stream.config,
                    deposit: deposit_left,
                    // Time is updated correctly, which will prevent any issue
                    // when changing rate.
                    last_time_updated: 10,
                    ..default_stream()
                })
            );

            assert_eq!(get_deposit(ALICE), deposit_left);
            assert_balance_change!(-, ALICE, open_stream.deposit + opening_deposit);
            assert_balance_change!(+, BOB, payment);
        })
    }

    #[test]
    fn perform_payment_works_with_max_rate() {
        ExtBuilder::default().build().execute_with(|| {
            let opening_deposit = OpenStreamHoldAmount::get();
            let open_stream = OpenStream {
                config: StreamConfig {
                    rate: u128::MAX,
                    ..default_config()
                },
                ..default()
            };
            assert_ok!(open_stream.call());

            assert_balance_change!(-, ALICE, open_stream.deposit + opening_deposit);
            assert_eq!(get_deposit(ALICE), open_stream.deposit);

            roll_to(10);
            let payment = open_stream.deposit;
            let deposit_left = 0;

            assert_ok!(StreamPayment::perform_payment(
                // Anyone can dispatch an update.
                RuntimeOrigin::signed(CHARLIE),
                0
            ));

            assert_event_emitted!(PaymentEvent {
                amount: payment,
                stalled: true,
                ..default()
            });

            assert_eq!(
                Streams::<Runtime>::get(0),
                Some(Stream {
                    config: open_stream.config,
                    deposit: deposit_left,
                    last_time_updated: 10,
                    ..default_stream()
                })
            );

            assert_eq!(get_deposit(ALICE), deposit_left);
            assert_balance_change!(-, ALICE, open_stream.deposit + opening_deposit);
            assert_balance_change!(+, BOB, payment);
        })
    }

    #[test]
    fn perform_payment_works_with_overflow() {
        ExtBuilder::default().build().execute_with(|| {
            let opening_deposit = OpenStreamHoldAmount::get();
            let open_stream = OpenStream {
                config: StreamConfig {
                    rate: u128::MAX / 10,
                    ..default_config()
                },
                ..default()
            };
            assert_ok!(open_stream.call());

            assert_balance_change!(-, ALICE, open_stream.deposit + opening_deposit);
            assert_eq!(get_deposit(ALICE), open_stream.deposit);

            roll_to(20);
            let payment = open_stream.deposit;
            let deposit_left = 0;

            assert_ok!(StreamPayment::perform_payment(
                // Anyone can dispatch an update.
                RuntimeOrigin::signed(CHARLIE),
                0
            ));

            assert_event_emitted!(PaymentEvent {
                amount: payment,
                stalled: true,
                ..default()
            });
            assert_eq!(
                Streams::<Runtime>::get(0),
                Some(Stream {
                    config: open_stream.config,
                    deposit: deposit_left,
                    last_time_updated: 20,
                    ..default_stream()
                })
            );

            assert_eq!(get_deposit(ALICE), deposit_left);
            assert_balance_change!(-, ALICE, open_stream.deposit + opening_deposit);
            assert_balance_change!(+, BOB, payment);
        })
    }

    #[test]
    fn payment_matching_deposit_is_considered_stalled() {
        ExtBuilder::default().build().execute_with(|| {
            let opening_deposit = OpenStreamHoldAmount::get();
            let config = default_config();
            let open_stream = OpenStream {
                config,
                deposit: config.rate * 9,
                ..default()
            };

            assert_ok!(open_stream.call());

            assert_balance_change!(-, ALICE, open_stream.deposit + opening_deposit);
            assert_eq!(get_deposit(ALICE), open_stream.deposit,);

            roll_to(10);
            let payment = open_stream.deposit;
            let deposit_left = 0;

            assert_ok!(StreamPayment::perform_payment(
                // Anyone can dispatch an update.
                RuntimeOrigin::signed(CHARLIE),
                0
            ));

            assert_event_emitted!(PaymentEvent {
                amount: payment,
                stalled: true,
                ..default()
            });
            assert_eq!(
                Streams::<Runtime>::get(0),
                Some(Stream {
                    config: open_stream.config,
                    deposit: deposit_left,
                    last_time_updated: 10,
                    ..default_stream()
                })
            );

            assert_eq!(get_deposit(ALICE), deposit_left);
            assert_balance_change!(-, ALICE, open_stream.deposit + opening_deposit);
            assert_balance_change!(+, BOB, payment);
        })
    }

    #[test]
    fn perform_payment_works_alt_unit() {
        ExtBuilder::default().build().execute_with(|| {
            let opening_deposit = OpenStreamHoldAmount::get();
            let open_stream = OpenStream {
                config: StreamConfig {
                    time_unit: TimeUnit::Timestamp,
                    ..default_config()
                },
                ..default()
            };
            assert_ok!(open_stream.call());

            assert_balance_change!(-, ALICE, open_stream.deposit + opening_deposit);
            assert_eq!(get_deposit(ALICE), open_stream.deposit);

            let delta = u128::from(roll_to(10));
            let payment = delta * open_stream.config.rate * 12; // 12 sec per block
            let deposit_left = open_stream.deposit - payment;

            assert_ok!(StreamPayment::perform_payment(
                // Anyone can dispatch an update.
                RuntimeOrigin::signed(CHARLIE),
                0
            ));

            assert_event_emitted!(PaymentEvent {
                amount: payment,
                ..default()
            });
            assert_eq!(
                Streams::<Runtime>::get(0),
                Some(Stream {
                    config: open_stream.config,
                    deposit: deposit_left,
                    last_time_updated: 120,
                    ..default_stream()
                })
            );

            assert_eq!(get_deposit(ALICE), deposit_left);
            assert_balance_change!(-, ALICE, open_stream.deposit + opening_deposit);
            assert_balance_change!(+, BOB, payment);
        })
    }

    #[test]
    fn protect_from_decreasing_time() {
        ExtBuilder::default().build().execute_with(|| {
            let opening_deposit = OpenStreamHoldAmount::get();
            let initial_deposit = 1 * MEGA;
            let config = StreamConfig {
                time_unit: TimeUnit::Decreasing,
                ..default_config()
            };

            assert_ok!(OpenStream {
                config,
                ..default()
            }
            .call());

            assert_balance_change!(-, ALICE, initial_deposit + opening_deposit);
            assert_eq!(get_deposit(ALICE), initial_deposit);

            roll_to(10);

            assert_ok!(StreamPayment::perform_payment(
                RuntimeOrigin::signed(CHARLIE),
                0
            ));

            assert_eq!(get_deposit(ALICE), initial_deposit);
            assert_balance_change!(-, ALICE, initial_deposit + opening_deposit);
            assert_balance_change!(+, BOB, 0); // no payment
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
                Error::UnknownStreamId
            );
        })
    }

    #[test]
    fn stream_cant_be_closed_by_third_party() {
        ExtBuilder::default().build().execute_with(|| {
            let opening_deposit = OpenStreamHoldAmount::get();
            let open_stream = OpenStream::default();
            assert_ok!(open_stream.call());

            assert_balance_change!(-, ALICE, open_stream.deposit + opening_deposit);
            assert_eq!(get_deposit(ALICE), open_stream.deposit);

            assert_err!(
                StreamPayment::close_stream(RuntimeOrigin::signed(CHARLIE), 0),
                Error::UnauthorizedOrigin
            );
        })
    }

    fn stream_can_be_closed_by(account: AccountId) {
        ExtBuilder::default().build().execute_with(|| {
            let opening_deposit = OpenStreamHoldAmount::get();
            let open_stream = OpenStream::default();
            assert_ok!(open_stream.call());

            assert_balance_change!(-, ALICE, open_stream.deposit + opening_deposit);
            assert_eq!(get_deposit(ALICE), open_stream.deposit);

            assert_ok!(StreamPayment::close_stream(
                RuntimeOrigin::signed(account),
                0
            ),);
            assert_event_emitted!(Event::<Runtime>::StreamClosed {
                stream_id: 0,
                refunded: open_stream.deposit + opening_deposit
            });
            assert_eq!(Streams::<Runtime>::get(0), None);
        })
    }

    #[test]
    fn stream_can_be_closed_by_source() {
        stream_can_be_closed_by(ALICE)
    }

    #[test]
    fn stream_can_be_closed_by_target() {
        stream_can_be_closed_by(BOB)
    }

    #[test]
    fn close_stream_with_payment() {
        ExtBuilder::default().build().execute_with(|| {
            let opening_deposit = OpenStreamHoldAmount::get();
            let open_stream = OpenStream::default();
            assert_ok!(open_stream.call());

            assert_balance_change!(-, ALICE, open_stream.deposit + opening_deposit);
            assert_eq!(get_deposit(ALICE), open_stream.deposit);

            let delta = u128::from(roll_to(10));
            let payment = delta * open_stream.config.rate;
            let deposit_left = open_stream.deposit - payment;

            assert_ok!(StreamPayment::close_stream(RuntimeOrigin::signed(ALICE), 0));

            assert_event_emitted!(PaymentEvent {
                amount: payment,
                ..default()
            });
            assert_event_emitted!(Event::<Runtime>::StreamClosed {
                stream_id: 0,
                refunded: deposit_left + opening_deposit
            });
            assert_eq!(Streams::<Runtime>::get(0), None);

            assert_eq!(get_deposit(ALICE), 0);
            assert_balance_change!(-, ALICE,  payment);
            assert_balance_change!(+, BOB, payment);
        })
    }
}

mod request_change {
    use super::*;

    #[test]
    fn cannot_request_change_of_unknown_stream() {
        ExtBuilder::default().build().execute_with(|| {
            assert_err!(
                StreamPayment::request_change(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    ChangeKind::Suggestion,
                    default_config(),
                    None,
                ),
                Error::UnknownStreamId
            );
        })
    }

    #[test]
    fn third_party_cannot_request_change() {
        ExtBuilder::default().build().execute_with(|| {
            let open_stream = OpenStream::default();
            assert_ok!(open_stream.call());

            assert_err!(
                StreamPayment::request_change(
                    RuntimeOrigin::signed(CHARLIE),
                    0,
                    ChangeKind::Suggestion,
                    open_stream.config,
                    None,
                ),
                Error::UnauthorizedOrigin
            );
        })
    }

    #[test]
    fn target_cant_change_deposit() {
        ExtBuilder::default().build().execute_with(|| {
            let open_stream = OpenStream::default();
            assert_ok!(open_stream.call());

            assert_err!(
                StreamPayment::request_change(
                    RuntimeOrigin::signed(BOB),
                    0,
                    ChangeKind::Suggestion,
                    open_stream.config,
                    Some(DepositChange::Absolute(100)),
                ),
                Error::TargetCantChangeDeposit
            );
        })
    }

    #[test]
    fn request_same_config_is_noop() {
        ExtBuilder::default().build().execute_with(|| {
            let open_stream = OpenStream::default();
            assert_ok!(open_stream.call());

            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(ALICE),
                0,
                ChangeKind::Suggestion,
                open_stream.config,
                None,
            ));

            assert_event_not_emitted!(Event::<Runtime>::StreamConfigChanged {
                stream_id: 0,
                old_config: open_stream.config,
                new_config: open_stream.config,
                deposit_change: None,
            });
        })
    }

    #[test]
    fn source_can_immediately_increase_rate() {
        ExtBuilder::default().build().execute_with(|| {
            let open_stream = OpenStream::default();
            assert_ok!(open_stream.call());

            let new_config = StreamConfig {
                rate: 101,
                ..open_stream.config
            };

            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(ALICE),
                0,
                ChangeKind::Suggestion,
                new_config,
                None,
            ),);

            assert_event_emitted!(Event::<Runtime>::StreamConfigChanged {
                stream_id: 0,
                old_config: open_stream.config,
                new_config,
                deposit_change: None,
            });
        })
    }

    #[test]
    fn target_can_immediately_decrease_rate() {
        ExtBuilder::default().build().execute_with(|| {
            let open_stream = OpenStream::default();
            assert_ok!(open_stream.call());

            let new_config = StreamConfig {
                rate: 99,
                ..open_stream.config
            };
            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(BOB),
                0,
                ChangeKind::Suggestion,
                new_config,
                None,
            ),);

            assert_event_emitted!(Event::<Runtime>::StreamConfigChanged {
                stream_id: 0,
                old_config: open_stream.config,
                new_config,
                deposit_change: None,
            });
        })
    }

    fn source_can_immediately_change_deposit(change: DepositChange<Balance>) {
        ExtBuilder::default().build().execute_with(|| {
            let opening_deposit = OpenStreamHoldAmount::get();
            let open_stream = OpenStream::default();
            assert_ok!(open_stream.call());

            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(ALICE),
                0,
                ChangeKind::Suggestion,
                open_stream.config,
                Some(change),
            ));

            assert_event_emitted!(Event::<Runtime>::StreamConfigChanged {
                stream_id: 0,
                old_config: open_stream.config,
                new_config: open_stream.config,
                deposit_change: Some(change),
            });

            assert_balance_change!(-, ALICE, match change {
                DepositChange::Absolute(amount) => amount + opening_deposit,
                DepositChange::Increase(amount) => open_stream.deposit + opening_deposit + amount,
                DepositChange::Decrease(amount) => open_stream.deposit + opening_deposit - amount,
            });
        })
    }

    #[test]
    fn source_can_immediately_change_deposit_absolute() {
        source_can_immediately_change_deposit(DepositChange::Absolute(100))
    }

    #[test]
    fn source_can_immediately_increase_deposit() {
        source_can_immediately_change_deposit(DepositChange::Increase(100))
    }

    #[test]
    fn source_can_immediately_decrease_deposit() {
        source_can_immediately_change_deposit(DepositChange::Decrease(100))
    }

    #[test]
    fn immediate_deposit_change_underflow() {
        ExtBuilder::default().build().execute_with(|| {
            let open_stream = OpenStream::default();
            assert_ok!(open_stream.call());

            assert_err!(
                StreamPayment::request_change(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    ChangeKind::Suggestion,
                    open_stream.config,
                    Some(DepositChange::Decrease(open_stream.deposit + 1)),
                ),
                ArithmeticError::Underflow
            );
        })
    }

    #[test]
    fn immediate_deposit_change_overflow() {
        ExtBuilder::default().build().execute_with(|| {
            let open_stream = OpenStream::default();
            assert_ok!(open_stream.call());

            assert_err!(
                StreamPayment::request_change(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    ChangeKind::Suggestion,
                    open_stream.config,
                    Some(DepositChange::Increase(u128::MAX - open_stream.deposit + 1)),
                ),
                ArithmeticError::Overflow
            );
        })
    }

    #[test]
    fn change_of_asset_requires_absolute_deposit_change() {
        ExtBuilder::default().build().execute_with(|| {
            let open_stream = OpenStream::default();
            assert_ok!(open_stream.call());

            let new_config = StreamConfig {
                asset_id: StreamPaymentAssetId::Dummy,
                ..open_stream.config
            };

            assert_err!(
                StreamPayment::request_change(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    ChangeKind::Suggestion,
                    new_config,
                    None,
                ),
                Error::ChangingAssetRequiresAbsoluteDepositChange,
            );

            assert_err!(
                StreamPayment::request_change(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    ChangeKind::Suggestion,
                    new_config,
                    Some(DepositChange::Increase(5)),
                ),
                Error::ChangingAssetRequiresAbsoluteDepositChange,
            );

            assert_err!(
                StreamPayment::request_change(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    ChangeKind::Suggestion,
                    new_config,
                    Some(DepositChange::Decrease(5)),
                ),
                Error::ChangingAssetRequiresAbsoluteDepositChange,
            );

            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(ALICE),
                0,
                ChangeKind::Suggestion,
                new_config,
                Some(DepositChange::Absolute(5)),
            ));
        })
    }

    #[test]
    fn override_cannot_trigger_retroactive_payment() {
        ExtBuilder::default().build().execute_with(|| {
            let open_stream = OpenStream::default();
            assert_ok!(open_stream.call());

            // Target requets a change.
            let change1 = StreamConfig {
                rate: 101,
                ..open_stream.config
            };
            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(BOB),
                0,
                ChangeKind::Mandatory { deadline: 10 },
                change1,
                None,
            ));

            // Roll to block after deadline, payment should stop at deadline.
            let delta = u128::from(roll_to(11));
            let payment = (delta - 1) * open_stream.config.rate;

            assert_ok!(StreamPayment::perform_payment(
                RuntimeOrigin::signed(CHARLIE),
                0
            ));
            assert_event_emitted!(PaymentEvent {
                amount: payment,
                ..default()
            });

            // Target requets a new change that moves the deadline in the future.
            let change1 = StreamConfig {
                rate: 102,
                ..open_stream.config
            };
            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(BOB),
                0,
                ChangeKind::Mandatory { deadline: 20 },
                change1,
                None,
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
    fn deadline_in_past_is_fine() {
        ExtBuilder::default().build().execute_with(|| {
            let open_stream = OpenStream::default();
            assert_ok!(open_stream.call());

            // Target requets a change.
            let change1 = StreamConfig {
                rate: 101,
                ..open_stream.config
            };
            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(BOB),
                0,
                ChangeKind::Mandatory { deadline: 10 },
                change1,
                None,
            ));

            // Roll to block after deadline, payment should stop at deadline.
            let delta = u128::from(roll_to(11));
            let payment = (delta - 1) * open_stream.config.rate;

            assert_ok!(StreamPayment::perform_payment(
                RuntimeOrigin::signed(CHARLIE),
                0
            ));
            assert_event_emitted!(PaymentEvent {
                amount: payment,
                ..default()
            });

            // Target requets a new change that moves the deadline in the future.
            let change1 = StreamConfig {
                rate: 102,
                ..open_stream.config
            };
            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(BOB),
                0,
                ChangeKind::Mandatory { deadline: 5 },
                change1,
                None,
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

    fn can_override_suggestion(requester: Party) {
        ExtBuilder::default().build().execute_with(|| {
            let (caller1, caller2) = match requester {
                Party::Source => (ALICE, BOB),
                Party::Target => (BOB, ALICE),
            };

            let open_stream = OpenStream::default();
            assert_ok!(open_stream.call());

            // Caller1 requests a change.
            let change1 = StreamConfig {
                time_unit: TimeUnit::Timestamp,
                ..open_stream.config
            };
            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(caller1),
                0,
                ChangeKind::Suggestion,
                change1,
                None,
            ));

            assert_event_emitted!(Event::<Runtime>::StreamConfigChangeRequested {
                stream_id: 0,
                request_nonce: 1,
                requester,
                old_config: open_stream.config,
                new_config: change1,
            });

            // Caller2 override the request
            let change2 = StreamConfig {
                time_unit: TimeUnit::Timestamp,
                ..open_stream.config
            };

            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(caller2),
                0,
                ChangeKind::Suggestion,
                change2,
                None,
            ));

            assert_event_emitted!(Event::<Runtime>::StreamConfigChangeRequested {
                stream_id: 0,
                request_nonce: 2,
                requester: requester.inverse(),
                old_config: open_stream.config,
                new_config: change2,
            });
        });
    }

    #[test]
    fn source_can_override_target_suggestion() {
        can_override_suggestion(Party::Source)
    }

    #[test]
    fn target_can_override_source_suggestion() {
        can_override_suggestion(Party::Target)
    }

    fn cant_override_mandatory_request(requester: Party) {
        ExtBuilder::default().build().execute_with(|| {
            let (caller1, caller2) = match requester {
                Party::Source => (ALICE, BOB),
                Party::Target => (BOB, ALICE),
            };

            let open_stream = OpenStream::default();
            assert_ok!(open_stream.call());

            // Caller1 requests a change.
            let change1 = StreamConfig {
                time_unit: TimeUnit::Timestamp,
                ..open_stream.config
            };

            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(caller1),
                0,
                ChangeKind::Mandatory { deadline: 10 },
                change1,
                None,
            ));

            assert_event_emitted!(Event::<Runtime>::StreamConfigChangeRequested {
                stream_id: 0,
                request_nonce: 1,
                requester,
                old_config: open_stream.config,
                new_config: change1,
            });

            // Caller2 tries to override the request
            let change2 = StreamConfig {
                time_unit: TimeUnit::Timestamp,
                ..open_stream.config
            };

            assert_err!(
                StreamPayment::request_change(
                    RuntimeOrigin::signed(caller2),
                    0,
                    ChangeKind::Suggestion,
                    change2,
                    None,
                ),
                Error::CantOverrideMandatoryChange
            );

            assert_event_not_emitted!(Event::<Runtime>::StreamConfigChangeRequested {
                stream_id: 0,
                request_nonce: 2,
                requester: requester.inverse(),
                old_config: open_stream.config,
                new_config: change2,
            });
        })
    }

    #[test]
    fn source_cant_override_target_mandatory_request() {
        cant_override_mandatory_request(Party::Source)
    }

    #[test]
    fn target_cant_override_source_mandatory_request() {
        cant_override_mandatory_request(Party::Target)
    }
}

mod accept_requested_change {
    use super::*;

    #[test]
    fn cannot_accept_requested_change_of_unknown_stream() {
        ExtBuilder::default().build().execute_with(|| {
            assert_err!(
                StreamPayment::accept_requested_change(RuntimeOrigin::signed(ALICE), 0, 0, None),
                Error::UnknownStreamId
            );
        })
    }

    #[test]
    fn third_party_cant_accept_change() {
        ExtBuilder::default().build().execute_with(|| {
            let open_stream = OpenStream::default();
            assert_ok!(open_stream.call());

            let change = StreamConfig {
                time_unit: TimeUnit::Timestamp,
                ..open_stream.config
            };

            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(ALICE),
                0,
                ChangeKind::Suggestion,
                change,
                None
            ));

            assert_err!(
                StreamPayment::accept_requested_change(RuntimeOrigin::signed(CHARLIE), 0, 1, None),
                Error::UnauthorizedOrigin
            );
        })
    }

    #[test]
    fn party_cant_accept_own_change() {
        ExtBuilder::default().build().execute_with(|| {
            let open_stream = OpenStream::default();
            assert_ok!(open_stream.call());

            let change = StreamConfig {
                time_unit: TimeUnit::Timestamp,
                ..open_stream.config
            };

            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(ALICE),
                0,
                ChangeKind::Suggestion,
                change,
                None
            ));

            assert_err!(
                StreamPayment::accept_requested_change(RuntimeOrigin::signed(ALICE), 0, 1, None),
                Error::CantAcceptOwnRequest
            );
        })
    }

    #[test]
    fn wrong_nonce() {
        ExtBuilder::default().build().execute_with(|| {
            let open_stream = OpenStream::default();
            assert_ok!(open_stream.call());

            let change = StreamConfig {
                time_unit: TimeUnit::Timestamp,
                ..open_stream.config
            };

            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(ALICE),
                0,
                ChangeKind::Suggestion,
                change,
                None
            ));

            assert_err!(
                StreamPayment::accept_requested_change(RuntimeOrigin::signed(BOB), 0, 0, None),
                Error::WrongRequestNonce
            );
        })
    }

    #[test]
    fn target_cant_change_deposit() {
        ExtBuilder::default().build().execute_with(|| {
            let open_stream = OpenStream::default();
            assert_ok!(open_stream.call());

            let change = StreamConfig {
                time_unit: TimeUnit::Timestamp,
                ..open_stream.config
            };

            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(ALICE),
                0,
                ChangeKind::Suggestion,
                change,
                None
            ));

            assert_err!(
                StreamPayment::accept_requested_change(
                    RuntimeOrigin::signed(BOB),
                    0,
                    1,
                    Some(DepositChange::Absolute(500)),
                ),
                Error::TargetCantChangeDeposit
            );
        })
    }

    fn can_accept_other_party_request(party1: AccountId, party2: AccountId) {
        ExtBuilder::default().build().execute_with(|| {
            let open_stream = OpenStream::default();
            assert_ok!(open_stream.call());

            let change = StreamConfig {
                time_unit: TimeUnit::Timestamp,
                ..open_stream.config
            };

            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(party1),
                0,
                ChangeKind::Suggestion,
                change,
                None
            ));

            assert_ok!(StreamPayment::accept_requested_change(
                RuntimeOrigin::signed(party2),
                0,
                1,
                None,
            ));

            let stream = Streams::<Runtime>::get(0).unwrap();
            assert_eq!(
                stream,
                Stream {
                    config: change,
                    request_nonce: 1,
                    pending_request: None,
                    deposit: open_stream.deposit,
                    last_time_updated: 12, // 1st block = 12s
                    ..default_stream()
                }
            );
        })
    }

    #[test]
    fn target_can_accept_source_request() {
        can_accept_other_party_request(ALICE, BOB)
    }

    #[test]
    fn source_can_accept_target_request() {
        can_accept_other_party_request(BOB, ALICE)
    }

    #[test]
    fn change_of_asset_requires_absolute_deposit_change() {
        ExtBuilder::default().build().execute_with(|| {
            let open_stream = OpenStream::default();
            assert_ok!(open_stream.call());

            let new_config = StreamConfig {
                asset_id: StreamPaymentAssetId::Dummy,
                ..open_stream.config
            };

            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(BOB),
                0,
                ChangeKind::Suggestion,
                new_config,
                None,
            ));

            assert_err!(
                StreamPayment::accept_requested_change(RuntimeOrigin::signed(ALICE), 0, 1, None,),
                Error::ChangingAssetRequiresAbsoluteDepositChange,
            );

            assert_err!(
                StreamPayment::accept_requested_change(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    1,
                    Some(DepositChange::Increase(5)),
                ),
                Error::ChangingAssetRequiresAbsoluteDepositChange,
            );

            assert_err!(
                StreamPayment::accept_requested_change(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    1,
                    Some(DepositChange::Decrease(5)),
                ),
                Error::ChangingAssetRequiresAbsoluteDepositChange,
            );

            assert_ok!(StreamPayment::accept_requested_change(
                RuntimeOrigin::signed(ALICE),
                0,
                1,
                Some(DepositChange::Absolute(5)),
            ));
        })
    }

    #[test]
    fn accept_deadline_in_past_doesnt_pay_retroactively() {
        ExtBuilder::default().build().execute_with(|| {
            let open_stream = OpenStream::default();
            assert_ok!(open_stream.call());

            // Target requets a change.
            let change1 = StreamConfig {
                rate: 101,
                ..open_stream.config
            };
            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(BOB),
                0,
                ChangeKind::Mandatory { deadline: 10 },
                change1,
                None,
            ));

            // Roll to block after deadline, payment should stop at deadline.
            let delta = u128::from(roll_to(11));
            let payment = (delta - 1) * open_stream.config.rate;

            assert_ok!(StreamPayment::perform_payment(
                RuntimeOrigin::signed(CHARLIE),
                0
            ));
            assert_event_emitted!(PaymentEvent {
                amount: payment,
                ..default()
            });

            // Accepting the request shouldn't not pay retroactively
            roll_to(20);

            let deposit_before = Streams::<Runtime>::get(0).unwrap().deposit;
            let increase = 42;
            assert_ok!(StreamPayment::accept_requested_change(
                RuntimeOrigin::signed(ALICE),
                0,
                1,
                Some(DepositChange::Increase(increase)),
            ));
            let deposit_after = Streams::<Runtime>::get(0).unwrap().deposit;

            assert_eq!(
                deposit_before,
                deposit_after - increase,
                "no payment should be performed"
            );
        })
    }
}

mod cancel_change_request {
    use super::*;

    #[test]
    fn cannot_cancel_request_of_unknown_stream() {
        ExtBuilder::default().build().execute_with(|| {
            assert_err!(
                StreamPayment::cancel_change_request(RuntimeOrigin::signed(ALICE), 0),
                Error::UnknownStreamId
            );
        })
    }

    #[test]
    fn can_only_cancel_if_there_is_a_pending_request() {
        ExtBuilder::default().build().execute_with(|| {
            let open_stream = OpenStream::default();
            assert_ok!(open_stream.call());

            assert_err!(
                StreamPayment::cancel_change_request(RuntimeOrigin::signed(ALICE), 0),
                Error::NoPendingRequest
            );
        })
    }

    #[test]
    fn third_party_cant_cancel_change() {
        ExtBuilder::default().build().execute_with(|| {
            let open_stream = OpenStream::default();
            assert_ok!(open_stream.call());

            let change = StreamConfig {
                time_unit: TimeUnit::Timestamp,
                ..open_stream.config
            };

            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(ALICE),
                0,
                ChangeKind::Suggestion,
                change,
                None
            ));

            assert_err!(
                StreamPayment::cancel_change_request(RuntimeOrigin::signed(CHARLIE), 0),
                Error::UnauthorizedOrigin
            );
        })
    }

    fn can_only_cancel_own_request(caller1: AccountId, caller2: AccountId) {
        ExtBuilder::default().build().execute_with(|| {
            let open_stream = OpenStream::default();
            assert_ok!(open_stream.call());

            let change = StreamConfig {
                time_unit: TimeUnit::Timestamp,
                ..open_stream.config
            };

            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(caller1),
                0,
                ChangeKind::Suggestion,
                change,
                None
            ));

            assert_err!(
                StreamPayment::cancel_change_request(RuntimeOrigin::signed(caller2), 0),
                Error::CanOnlyCancelOwnRequest
            );
        })
    }

    #[test]
    fn source_can_only_cancel_own_request() {
        can_only_cancel_own_request(ALICE, BOB)
    }

    #[test]
    fn target_can_only_cancel_own_request() {
        can_only_cancel_own_request(BOB, ALICE)
    }
}

mod immediately_change_deposit {
    use super::*;

    #[test]
    fn cannot_immediately_change_deposit_of_unknown_stream() {
        ExtBuilder::default().build().execute_with(|| {
            assert_err!(
                StreamPayment::immediately_change_deposit(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    StreamPaymentAssetId::Native,
                    DepositChange::Absolute(500),
                ),
                Error::UnknownStreamId
            );
        })
    }

    fn cant_change_deposit(account: AccountId) {
        ExtBuilder::default().build().execute_with(|| {
            let open_stream = OpenStream::default();
            assert_ok!(open_stream.call());

            let change = StreamConfig {
                time_unit: TimeUnit::Timestamp,
                ..open_stream.config
            };

            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(ALICE),
                0,
                ChangeKind::Suggestion,
                change,
                None
            ));

            assert_err!(
                StreamPayment::immediately_change_deposit(
                    RuntimeOrigin::signed(account),
                    0,
                    StreamPaymentAssetId::Native,
                    DepositChange::Absolute(500)
                ),
                Error::UnauthorizedOrigin
            );
        })
    }

    #[test]
    fn target_cant_change_deposit() {
        cant_change_deposit(BOB)
    }

    #[test]
    fn third_party_cant_change_deposit() {
        cant_change_deposit(CHARLIE)
    }

    #[test]
    fn source_can_change_deposit() {
        ExtBuilder::default().build().execute_with(|| {
            let open_stream = OpenStream::default();
            assert_ok!(open_stream.call());

            let change = StreamConfig {
                time_unit: TimeUnit::Timestamp,
                ..open_stream.config
            };

            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(ALICE),
                0,
                ChangeKind::Suggestion,
                change,
                None
            ));

            assert_ok!(StreamPayment::immediately_change_deposit(
                RuntimeOrigin::signed(ALICE),
                0,
                StreamPaymentAssetId::Native,
                DepositChange::Absolute(500)
            ));

            assert_eq!(get_deposit(ALICE), 500);
        })
    }

    #[test]
    fn change_deposit_funds_unavailable() {
        ExtBuilder::default().build().execute_with(|| {
            let open_stream = OpenStream::default();
            assert_ok!(open_stream.call());

            let change = StreamConfig {
                time_unit: TimeUnit::Timestamp,
                ..open_stream.config
            };

            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(ALICE),
                0,
                ChangeKind::Suggestion,
                change,
                None
            ));

            assert_err!(
                StreamPayment::immediately_change_deposit(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    StreamPaymentAssetId::Native,
                    DepositChange::Absolute(DEFAULT_BALANCE + 1)
                ),
                TokenError::FundsUnavailable
            );
        })
    }

    #[test]
    fn change_deposit_increase_overflow() {
        ExtBuilder::default().build().execute_with(|| {
            let open_stream = OpenStream::default();
            assert_ok!(open_stream.call());

            let change = StreamConfig {
                time_unit: TimeUnit::Timestamp,
                ..open_stream.config
            };

            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(ALICE),
                0,
                ChangeKind::Suggestion,
                change,
                None
            ));

            assert_err!(
                StreamPayment::immediately_change_deposit(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    StreamPaymentAssetId::Native,
                    DepositChange::Increase(u128::MAX)
                ),
                ArithmeticError::Overflow
            );
        })
    }

    #[test]
    fn change_deposit_decrease_underflow() {
        ExtBuilder::default().build().execute_with(|| {
            let open_stream = OpenStream::default();
            assert_ok!(open_stream.call());

            let change = StreamConfig {
                time_unit: TimeUnit::Timestamp,
                ..open_stream.config
            };

            assert_ok!(StreamPayment::request_change(
                RuntimeOrigin::signed(ALICE),
                0,
                ChangeKind::Suggestion,
                change,
                None
            ));

            assert_err!(
                StreamPayment::immediately_change_deposit(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    StreamPaymentAssetId::Native,
                    DepositChange::Decrease(open_stream.deposit + 1)
                ),
                ArithmeticError::Underflow
            );
        })
    }
}
