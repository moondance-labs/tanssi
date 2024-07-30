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
        assert_expected_events,
        tests::common::xcm::{
            core_buyer_common::*,
            mocknets::{DanceboxRococoPara as Dancebox, RococoRelay as Rococo, RococoRelayPallet},
            *,
        },
    },
    polkadot_runtime_parachains::assigner_on_demand as parachains_assigner_on_demand,
    staging_xcm::latest::{MaybeErrorCode, Response},
    tp_traits::ParaId,
    xcm_emulator::Chain,
};

#[test]
fn xcm_core_buyer_zero_balance() {
    let parathread_tank_in_relay = get_parathread_tank_relay_address();
    let balance_before = 0;

    // Invariant: if balance_before < BUY_EXECUTION_COST, then balance_after == balance_before
    let query_id = do_test(balance_before, None, true);

    // Receive XCM message in Relay Chain
    Rococo::execute_with(|| {
        type RuntimeEvent = <Rococo as Chain>::RuntimeEvent;
        let balance_after =
            <Rococo as RococoRelayPallet>::System::account(parathread_tank_in_relay.clone())
                .data
                .free;
        assert_expected_events!(
            Rococo,
            vec![
                RuntimeEvent::MessageQueue(pallet_message_queue::Event::Processed { success: false, .. }) => {},
            ]
        );
        assert_relay_order_event_not_emitted();
        assert_eq!(balance_before, balance_after);
    });

    Dancebox::execute_with(|| {
        assert_xcm_notification_event_not_emitted();
        assert_query_response_not_received(ParaId::from(PARATHREAD_ID), query_id);
    });
}

#[test]
fn xcm_core_buyer_only_enough_balance_for_buy_execution() {
    let parathread_tank_in_relay = get_parathread_tank_relay_address();
    let balance_before = BUY_EXECUTION_COST;

    // Invariant: if balance_before >= BUY_EXECUTION_COST then
    // balance_after <= (balance_before + BUY_EXECUTION_REFUND - BUY_EXECUTION_COST)
    // In this case the balance_after is 0 because BUY_EXECUTION_REFUND < ROCOCO_ED,
    // so the account gets the refund but it is immediatelly killed.
    let query_id = do_test(balance_before, None, true);

    // Receive XCM message in Relay Chain
    Rococo::execute_with(|| {
        type RuntimeEvent = <Rococo as Chain>::RuntimeEvent;
        let balance_after =
            <Rococo as RococoRelayPallet>::System::account(parathread_tank_in_relay.clone())
                .data
                .free;
        assert_expected_events!(
            Rococo,
            vec![
                RuntimeEvent::Balances(
                    pallet_balances::Event::Burned {
                        who,
                        amount: BUY_EXECUTION_COST,
                    }
                ) => {
                    who: *who == parathread_tank_in_relay,
                },
                RuntimeEvent::System(
                    frame_system::Event::KilledAccount {
                        account,
                    }
                ) => {
                    account: *account == parathread_tank_in_relay,
                },
                RuntimeEvent::MessageQueue(pallet_message_queue::Event::Processed { success: false, .. }) => {},
            ]
        );
        assert_relay_order_event_not_emitted();
        assert_eq!(balance_after, 0);
    });

    Dancebox::execute_with(|| {
        type RuntimeEvent = <Dancebox as Chain>::RuntimeEvent;
        assert_expected_events!(
            Dancebox,
            vec![
                RuntimeEvent::XcmCoreBuyer(
                    pallet_xcm_core_buyer::Event::ReceivedBuyCoreXCMResult {
                        para_id,
                        response,
                    }
                ) => {
                    para_id: *para_id == ParaId::from(PARATHREAD_ID),
                    response: *response != Response::DispatchResult(MaybeErrorCode::Success),
                },
            ]
        );
        assert_query_response_failure(ParaId::from(PARATHREAD_ID), query_id);
    });
}

#[test]
fn xcm_core_buyer_enough_balance_except_for_existential_deposit() {
    let parathread_tank_in_relay = get_parathread_tank_relay_address();
    // Core price must be greater than existential deposit
    let spot_price = ROCOCO_ED + 1;
    set_on_demand_base_fee(spot_price);
    let spot_price2 = spot_price;
    let balance_before = BUY_EXECUTION_COST + spot_price;

    let query_id = do_test(balance_before, None, true);

    // Receive XCM message in Relay Chain
    Rococo::execute_with(|| {
        type RuntimeEvent = <Rococo as Chain>::RuntimeEvent;

        let balance_after =
            <Rococo as RococoRelayPallet>::System::account(parathread_tank_in_relay.clone())
                .data
                .free;
        assert_expected_events!(
            Rococo,
            vec![
                RuntimeEvent::Balances(
                    pallet_balances::Event::Burned {
                        who,
                        amount: BUY_EXECUTION_COST,
                    }
                ) => {
                    who: *who == parathread_tank_in_relay,
                },
                RuntimeEvent::Balances(
                    pallet_balances::Event::Withdraw {
                        who,
                        amount,
                    }
                ) => {
                    who: *who == parathread_tank_in_relay,
                    amount: *amount == spot_price,
                },
                RuntimeEvent::System(
                    frame_system::Event::KilledAccount {
                        account,
                    }
                ) => {
                    account: *account == parathread_tank_in_relay,
                },
                RuntimeEvent::OnDemandAssignmentProvider(
                    parachains_assigner_on_demand::Event::OnDemandOrderPlaced {
                        para_id,
                        spot_price,
                        ordered_by,
                    }
                ) => {
                    para_id: *para_id == ParaId::from(PARATHREAD_ID),
                    spot_price: *spot_price == spot_price2,
                    ordered_by: *ordered_by == parathread_tank_in_relay,
                },
                // TODO: this now emits "success: false" even though the on demand order was placed, will
                // that break pallet_xcm_core_buyer?
                RuntimeEvent::MessageQueue(pallet_message_queue::Event::Processed { success: false, .. }) => {},
            ]
        );
        assert_eq!(balance_after, 0);
    });

    Dancebox::execute_with(|| {
        type RuntimeEvent = <Dancebox as Chain>::RuntimeEvent;
        assert_expected_events!(
            Dancebox,
            vec![
                RuntimeEvent::XcmCoreBuyer(
                    pallet_xcm_core_buyer::Event::ReceivedBuyCoreXCMResult {
                        para_id,
                        response,
                    }
                ) => {
                    para_id: *para_id == ParaId::from(PARATHREAD_ID),
                    response: *response == Response::DispatchResult(MaybeErrorCode::Success),
                },
            ]
        );
        assert_query_response_success(ParaId::from(PARATHREAD_ID), query_id);
    });
}

#[test]
fn xcm_core_buyer_enough_balance() {
    let parathread_tank_in_relay = get_parathread_tank_relay_address();
    let spot_price = get_on_demand_base_fee();
    let spot_price2 = spot_price;
    let balance_before = ROCOCO_ED + BUY_EXECUTION_COST + spot_price + 1;

    let query_id = do_test(balance_before, None, true);

    // Receive XCM message in Relay Chain
    Rococo::execute_with(|| {
        type RuntimeEvent = <Rococo as Chain>::RuntimeEvent;

        let balance_after =
            <Rococo as RococoRelayPallet>::System::account(parathread_tank_in_relay.clone())
                .data
                .free;
        assert_expected_events!(
            Rococo,
            vec![
                RuntimeEvent::Balances(
                    pallet_balances::Event::Burned {
                        who,
                        amount: BUY_EXECUTION_COST,
                    }
                ) => {
                    who: *who == parathread_tank_in_relay,
                },
                RuntimeEvent::Balances(
                    pallet_balances::Event::Withdraw {
                        who,
                        amount,
                    }
                ) => {
                    who: *who == parathread_tank_in_relay,
                    amount: *amount == spot_price,
                },
                RuntimeEvent::OnDemandAssignmentProvider(
                    parachains_assigner_on_demand::Event::OnDemandOrderPlaced {
                        para_id,
                        spot_price,
                        ordered_by,
                    }
                ) => {
                    para_id: *para_id == ParaId::from(PARATHREAD_ID),
                    spot_price: *spot_price == spot_price2,
                    ordered_by: *ordered_by == parathread_tank_in_relay,
                },
                RuntimeEvent::Balances(
                    pallet_balances::Event::Minted {
                        who,
                        amount: BUY_EXECUTION_REFUND,
                    }
                ) => {
                    who: *who == parathread_tank_in_relay,
                },
                RuntimeEvent::MessageQueue(pallet_message_queue::Event::Processed { success: true, .. }) => {},
            ]
        );
        assert_eq!(balance_after, ROCOCO_ED + 1 + BUY_EXECUTION_REFUND);
    });

    // Receive notification on dancebox
    Dancebox::execute_with(|| {
        type RuntimeEvent = <Dancebox as Chain>::RuntimeEvent;
        assert_expected_events!(
            Dancebox,
            vec![
                RuntimeEvent::XcmCoreBuyer(
                    pallet_xcm_core_buyer::Event::ReceivedBuyCoreXCMResult {
                        para_id,
                        response,
                    }
                ) => {
                    para_id: *para_id == ParaId::from(PARATHREAD_ID),
                    response: *response == Response::DispatchResult(MaybeErrorCode::Success),
                },
            ]
        );

        assert_query_response_success(ParaId::from(PARATHREAD_ID), query_id);
    });
}

#[test]
fn xcm_core_buyer_core_too_expensive() {
    let parathread_tank_in_relay = get_parathread_tank_relay_address();
    let balance_before = ROCOCO_ED + BUY_EXECUTION_COST + 1;
    set_on_demand_base_fee(balance_before * 2);

    let query_id = do_test(balance_before, None, true);

    // Receive XCM message in Relay Chain
    Rococo::execute_with(|| {
        type RuntimeEvent = <Rococo as Chain>::RuntimeEvent;

        let balance_after =
            <Rococo as RococoRelayPallet>::System::account(parathread_tank_in_relay.clone())
                .data
                .free;
        assert_expected_events!(
            Rococo,
            vec![
                RuntimeEvent::Balances(
                    pallet_balances::Event::Burned {
                        who,
                        amount: BUY_EXECUTION_COST,
                    }
                ) => {
                    who: *who == parathread_tank_in_relay,
                },
                RuntimeEvent::Balances(
                    pallet_balances::Event::Minted {
                        who,
                        amount: BUY_EXECUTION_REFUND_ON_FAILURE,
                    }
                ) => {
                    who: *who == parathread_tank_in_relay,
                },
                RuntimeEvent::MessageQueue(pallet_message_queue::Event::Processed { success: true, .. }) => {},
            ]
        );
        assert_relay_order_event_not_emitted();
        assert_eq!(
            balance_after,
            balance_before + BUY_EXECUTION_REFUND_ON_FAILURE - BUY_EXECUTION_COST
        );
    });

    // Receive notification on dancebox
    Dancebox::execute_with(|| {
        type RuntimeEvent = <Dancebox as Chain>::RuntimeEvent;
        assert_expected_events!(
            Dancebox,
            vec![
                RuntimeEvent::XcmCoreBuyer(
                    pallet_xcm_core_buyer::Event::ReceivedBuyCoreXCMResult {
                        para_id,
                        response,
                    }
                ) => {
                    para_id: *para_id == ParaId::from(PARATHREAD_ID),
                    response: *response != Response::DispatchResult(MaybeErrorCode::Success),
                },
            ]
        );

        assert_query_response_failure(ParaId::from(PARATHREAD_ID), query_id);
    });
}

#[test]
fn xcm_core_buyer_set_max_core_price() {
    let parathread_tank_in_relay = get_parathread_tank_relay_address();
    let spot_price = get_on_demand_base_fee();
    let balance_before = ROCOCO_ED + BUY_EXECUTION_COST + spot_price + 1;
    // Set max core price lower than spot_price, will result in no core bought even though the
    // account has enough balance
    let max_core_price = spot_price / 2;

    Dancebox::execute_with(|| {});

    let query_id = do_test(balance_before, Some(max_core_price), true);

    // Receive XCM message in Relay Chain
    Rococo::execute_with(|| {
        type RuntimeEvent = <Rococo as Chain>::RuntimeEvent;

        let balance_after =
            <Rococo as RococoRelayPallet>::System::account(parathread_tank_in_relay.clone())
                .data
                .free;
        assert_expected_events!(
            Rococo,
            vec![
                RuntimeEvent::Balances(
                    pallet_balances::Event::Burned {
                        who,
                        amount: BUY_EXECUTION_COST,
                    }
                ) => {
                    who: *who == parathread_tank_in_relay,
                },
                RuntimeEvent::Balances(
                    pallet_balances::Event::Minted {
                        who,
                        amount: BUY_EXECUTION_REFUND_ON_FAILURE,
                    }
                ) => {
                    who: *who == parathread_tank_in_relay,
                },
                RuntimeEvent::MessageQueue(pallet_message_queue::Event::Processed { success: true, .. }) => {},
            ]
        );
        assert_relay_order_event_not_emitted();
        assert_eq!(
            balance_after,
            ROCOCO_ED + 1 + BUY_EXECUTION_REFUND_ON_FAILURE + spot_price
        );
    });

    Dancebox::execute_with(|| {
        type RuntimeEvent = <Dancebox as Chain>::RuntimeEvent;
        assert_expected_events!(
            Dancebox,
            vec![
                RuntimeEvent::XcmCoreBuyer(
                    pallet_xcm_core_buyer::Event::ReceivedBuyCoreXCMResult {
                        para_id,
                        response,
                    }
                ) => {
                    para_id: *para_id == ParaId::from(PARATHREAD_ID),
                    response: *response != Response::DispatchResult(MaybeErrorCode::Success),
                },
            ]
        );

        assert_query_response_failure(ParaId::from(PARATHREAD_ID), query_id);
    });
}
