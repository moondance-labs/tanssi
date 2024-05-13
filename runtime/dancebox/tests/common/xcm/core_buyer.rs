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

use crate::assert_expected_events;
use staging_xcm::latest::{MaybeErrorCode, Response};
use staging_xcm::v3::QueryId;
use {
    crate::common::{
        dummy_boot_nodes, empty_genesis_data, run_to_session, start_block,
        xcm::{
            mocknets::{
                DanceboxRococoPara as Dancebox, DanceboxSender, RococoRelay as Rococo,
                RococoRelayPallet, RococoSender,
            },
            *,
        },
    },
    core::marker::PhantomData,
    cumulus_primitives_core::Weight,
    dancebox_runtime::{DataPreservers, Registrar, ServicesPayment, XcmCoreBuyer},
    frame_support::assert_ok,
    pallet_xcm_core_buyer::RelayXcmWeightConfigInner,
    polkadot_runtime_parachains::assigner_on_demand as parachains_assigner_on_demand,
    sp_runtime::AccountId32,
    staging_xcm_executor::traits::ConvertLocation,
    tp_traits::{ParaId, SlotFrequency},
    xcm_emulator::{Chain, RelayChain},
};

const PARATHREAD_ID: u32 = 3333;
const ROCOCO_ED: u128 = rococo_runtime_constants::currency::EXISTENTIAL_DEPOSIT;
const BUY_EXECUTION_COST: u128 = dancebox_runtime::xcm_config::XCM_BUY_EXECUTION_COST_ROCOCO;
// Difference between BUY_EXECUTION_COST and the actual cost that depends on the weight of the XCM
// message, gets refunded on successful execution of core buying extrinsic.
const BUY_EXECUTION_REFUND: u128 = 3334777;
// Difference between BUY_EXECUTION_COST and the actual cost that depends on the weight of the XCM
// message, gets refunded on un-successful execution of core buying extrinsic.
const BUY_EXECUTION_REFUND_ON_FAILURE: u128 = 1001467;

const PLACE_ORDER_WEIGHT_AT_MOST: Weight = Weight::from_parts(1_000_000_000, 100_000);

#[test]
fn constants() {
    // If these constants change, some tests may break
    assert_eq!(ROCOCO_ED, 100_000_000 / 3);
    assert_eq!(BUY_EXECUTION_COST, 70_000_000 + 1_266_663_99);
}

/// The tests in this module all use this function to trigger an XCM message to buy a core.
///
/// Each test has a different value of
/// * tank_account_balance: the balance of the parachain tank account in the relay chain
/// * spot_price: the price of a core
fn do_test(tank_account_balance: u128, set_max_core_price: Option<u128>) -> QueryId {
    let mut query_id = QueryId::MAX;

    Dancebox::execute_with(|| {
        // Register parathread
        let alice_origin = <Dancebox as Chain>::RuntimeOrigin::signed(DanceboxSender::get());
        assert_ok!(Registrar::register_parathread(
            alice_origin.clone(),
            PARATHREAD_ID.into(),
            SlotFrequency { min: 1, max: 1 },
            empty_genesis_data()
        ));
        assert_ok!(DataPreservers::set_boot_nodes(
            alice_origin,
            PARATHREAD_ID.into(),
            dummy_boot_nodes()
        ));
        let root_origin = <Dancebox as Chain>::RuntimeOrigin::root();
        assert_ok!(Registrar::mark_valid_for_collating(
            root_origin.clone(),
            PARATHREAD_ID.into()
        ));

        // TODO: xcm emulator breaks with the run_to_session function, but it works if we manually
        // call on_initialize here...
        start_block();
        run_to_session(2);

        if let Some(max_core_price) = set_max_core_price {
            assert_ok!(ServicesPayment::set_max_core_price(
                root_origin,
                PARATHREAD_ID.into(),
                Some(max_core_price)
            ));
        }
    });

    let parathread_tank_in_relay = get_parathread_tank_relay_address();

    // Pre-fund parathread tank in Relay Chain
    Rococo::execute_with(|| {
        let alice_origin = <Rococo as Chain>::RuntimeOrigin::signed(RococoSender::get());
        let destination = sp_runtime::MultiAddress::Id(parathread_tank_in_relay.clone());
        let value = tank_account_balance;

        // Add funds to parathread tank account in relay
        if value != 0 {
            assert_ok!(
                <Rococo as RococoRelayPallet>::Balances::transfer_keep_alive(
                    alice_origin,
                    destination,
                    value
                )
            );
        }
    });

    // If this test fails, uncomment this and try to debug the call without XCM first.
    /*
    Rococo::execute_with(|| {
        let alice_origin = <Rococo as Chain>::RuntimeOrigin::signed(RococoSender::get());
        let max_amount = u128::MAX;
        let para_id = PARATHREAD_ID.into();
        assert_ok!(
            <Rococo as RococoRelayPallet>::OnDemandAssignmentProvider::place_order_allow_death(
                alice_origin,
                max_amount,
                para_id,
            )
        );
    });
     */

    // Send XCM message from Dancebox pallet XcmCoreBuyer
    Dancebox::execute_with(|| {
        let root_origin = <Dancebox as Chain>::RuntimeOrigin::root();
        assert_ok!(XcmCoreBuyer::set_relay_xcm_weight_config(
            root_origin.clone(),
            Some(RelayXcmWeightConfigInner {
                buy_execution_cost: BUY_EXECUTION_COST,
                weight_at_most: PLACE_ORDER_WEIGHT_AT_MOST,
                _phantom: PhantomData,
            }),
        ));
        assert_ok!(XcmCoreBuyer::set_relay_chain(
            root_origin.clone(),
            Some(dancebox_runtime::xcm_config::RelayChain::Rococo),
        ));
        assert_ok!(XcmCoreBuyer::force_buy_core(
            root_origin,
            PARATHREAD_ID.into()
        ));

        type RuntimeEvent = <Dancebox as Chain>::RuntimeEvent;
        assert_expected_events!(
            Dancebox,
            vec![
                RuntimeEvent::XcmCoreBuyer(
                    pallet_xcm_core_buyer::Event::BuyCoreXcmSent { para_id, .. }
                ) => {
                    para_id: *para_id == ParaId::from(PARATHREAD_ID),
                },
            ]
        );

        query_id = find_query_id_for_para_id(ParaId::from(PARATHREAD_ID));
    });

    query_id
}

fn assert_relay_order_event_not_emitted() {
    type RuntimeEvent = <Rococo as Chain>::RuntimeEvent;

    let events = <Rococo as Chain>::events();
    for event in events {
        match event {
            RuntimeEvent::OnDemandAssignmentProvider(
                parachains_assigner_on_demand::Event::OnDemandOrderPlaced { .. },
            ) => {
                panic!("Event should not have been emitted: {:?}", event);
            }
            _ => (),
        }
    }
}

fn assert_xcm_notification_event_not_emitted() {
    type RuntimeEvent = <Dancebox as Chain>::RuntimeEvent;

    let events = <Dancebox as Chain>::events();
    for event in events {
        match event {
            RuntimeEvent::XcmCoreBuyer(
                pallet_xcm_core_buyer::Event::ReceivedBuyCoreXCMResult { .. },
            ) => {
                panic!("Event should not have been emitted: {:?}", event);
            }
            _ => (),
        }
    }
}

fn find_query_id_for_para_id(para_id: ParaId) -> QueryId {
    type RuntimeEvent = <Dancebox as Chain>::RuntimeEvent;

    let events = <Dancebox as Chain>::events();
    for event in events {
        match event {
            RuntimeEvent::XcmCoreBuyer(pallet_xcm_core_buyer::Event::BuyCoreXcmSent {
                para_id: event_para_id,
                transaction_status_query_id,
            }) => {
                if event_para_id == para_id {
                    return transaction_status_query_id;
                }
            }
            _ => (),
        }
    }

    panic!(
        "We should be able to find query_id for para_id: {:?}",
        para_id
    );
}

fn assert_query_response_success(para_id: ParaId, query_id: QueryId) {
    assert_query_response(para_id, query_id, true, true);
}

fn assert_query_response_failure(para_id: ParaId, query_id: QueryId) {
    assert_query_response(para_id, query_id, true, false);
}

fn assert_query_response_not_received(para_id: ParaId, query_id: QueryId) {
    assert_query_response(para_id, query_id, false, false);
}

fn assert_query_response(
    para_id: ParaId,
    query_id: QueryId,
    response_received: bool,
    is_successful: bool,
) {
    if is_successful && !response_received {
        panic!("Invalid input: If response is not received it cannot be successful.");
    }

    let maybe_query_id =
        pallet_xcm_core_buyer::QueryIdToParaId::<<Dancebox as Chain>::Runtime>::get(query_id);
    // Entry should only exists if we have not received response and vice versa.
    if maybe_query_id.is_some() == response_received {
        panic!(
            "There should not be any query_id<->para_id mapping existing for para_id: {:?}",
            para_id
        );
    }

    let maybe_in_flight_order =
        pallet_xcm_core_buyer::InFlightOrders::<<Dancebox as Chain>::Runtime>::get(para_id);
    // Entry should only exists if we have not received response and vice versa.
    if maybe_in_flight_order.is_some() == response_received {
        panic!(
            "There should not be any para_id<->in_flight_order mapping existing for para_id: {:?}",
            para_id
        );
    }

    // Entry should only exists if we have got successful response and vice versa.
    let maybe_pending_blocks_entry =
        pallet_xcm_core_buyer::PendingBlocks::<<Dancebox as Chain>::Runtime>::get(para_id);
    if maybe_pending_blocks_entry.is_some() != is_successful {
        if is_successful {
            panic!(
                "There should be a pending block entry for para_id: {:?}",
                para_id
            );
        } else {
            panic!(
                "There should not be a pending block entry for para_id: {:?}",
                para_id
            );
        }
    }
}

/// Get parathread tank address in relay chain. This is derived from the Dancebox para id and the
/// parathread para id.
fn get_parathread_tank_relay_address() -> AccountId32 {
    let parathread_tank_in_relay = Dancebox::execute_with(|| {
        let parathread_tank_multilocation = XcmCoreBuyer::relay_relative_multilocation(
            XcmCoreBuyer::interior_multilocation(PARATHREAD_ID.into()),
        )
        .expect("reanchor failed");
        let parathread_tank_in_relay =
            <Rococo as RelayChain>::SovereignAccountOf::convert_location(
                &parathread_tank_multilocation,
            )
            .expect("probably this relay chain does not allow DescendOrigin");

        parathread_tank_in_relay
    });
    parathread_tank_in_relay
}

fn get_on_demand_base_fee() -> u128 {
    Rococo::execute_with(|| {
        let config = <Rococo as RococoRelayPallet>::Configuration::config();

        config.on_demand_base_fee
    })
}

fn set_on_demand_base_fee(on_demand_base_fee: u128) {
    Rococo::execute_with(|| {
        let mut config = <Rococo as RococoRelayPallet>::Configuration::config();
        config.on_demand_base_fee = on_demand_base_fee;
        <Rococo as RococoRelayPallet>::Configuration::force_set_active_config(config);
    });
}

#[test]
fn xcm_core_buyer_zero_balance() {
    let parathread_tank_in_relay = get_parathread_tank_relay_address();
    let balance_before = 0;

    // Invariant: if balance_before < BUY_EXECUTION_COST, then balance_after == balance_before
    let query_id = do_test(balance_before, None);

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
    let query_id = do_test(balance_before, None);

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
                    pallet_balances::Event::Withdraw {
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
                RuntimeEvent::MessageQueue(pallet_message_queue::Event::Processed { success: true, .. }) => {},
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

    let query_id = do_test(balance_before, None);

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
                    pallet_balances::Event::Withdraw {
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
                    }
                ) => {
                    para_id: *para_id == ParaId::from(PARATHREAD_ID),
                    spot_price: *spot_price == spot_price2,
                },
                RuntimeEvent::MessageQueue(pallet_message_queue::Event::Processed { success: true, .. }) => {},
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

    let query_id = do_test(balance_before, None);

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
                    pallet_balances::Event::Withdraw {
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
                    }
                ) => {
                    para_id: *para_id == ParaId::from(PARATHREAD_ID),
                    spot_price: *spot_price == spot_price2,
                },
                RuntimeEvent::Balances(
                    pallet_balances::Event::Deposit {
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

    let query_id = do_test(balance_before, None);

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
                    pallet_balances::Event::Withdraw {
                        who,
                        amount: BUY_EXECUTION_COST,
                    }
                ) => {
                    who: *who == parathread_tank_in_relay,
                },
                RuntimeEvent::Balances(
                    pallet_balances::Event::Deposit {
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

    let query_id = do_test(balance_before, Some(max_core_price));

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
                    pallet_balances::Event::Withdraw {
                        who,
                        amount: BUY_EXECUTION_COST,
                    }
                ) => {
                    who: *who == parathread_tank_in_relay,
                },
                RuntimeEvent::Balances(
                    pallet_balances::Event::Deposit {
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
