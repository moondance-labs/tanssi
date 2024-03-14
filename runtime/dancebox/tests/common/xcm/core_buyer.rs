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
    crate::common::{
        dummy_boot_nodes, empty_genesis_data, run_to_session,
        xcm::mocknets::{
            DanceboxRococoPara as Dancebox, DanceboxSender, RococoRelay as Rococo,
            RococoRelayPallet, RococoSender,
        },
        xcm::*,
    },
    core::marker::PhantomData,
    cumulus_primitives_core::Weight,
    dancebox_runtime::{DataPreservers, Registrar, XcmCoreBuyer},
    frame_support::assert_ok,
    pallet_xcm_core_buyer::XcmWeightsTy,
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
// message, gets refunded.
const BUY_EXECUTION_REFUND: u128 = 5115980;
const PLACE_ORDER_WEIGHT_AT_MOST: Weight = Weight::from_parts(1_000_000_000, 100_000);

#[test]
fn constants() {
    // If these constants change, some tests may break
    assert_eq!(ROCOCO_ED, 100_000_000 / 3);
    assert_eq!(BUY_EXECUTION_COST, 50_000_000);
}

// TODO: modify tests to assert that the OnDemandOrderPlaced event was not emitted
// when it shouldn't

/// The tests in this module all use this function to trigger an XCM message to buy a core.
///
/// Each test has a different value of
/// * tank_account_balance: the balance of the parachain tank account in the relay chain
/// * spot_price: the price of a core
fn do_test(tank_account_balance: u128) {
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
            root_origin,
            PARATHREAD_ID.into()
        ));

        run_to_session(2);
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
        assert_ok!(XcmCoreBuyer::set_xcm_weights(
            root_origin.clone(),
            Some(XcmWeightsTy {
                buy_execution_cost: BUY_EXECUTION_COST,
                weight_at_most: PLACE_ORDER_WEIGHT_AT_MOST,
                _phantom: PhantomData,
            }),
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
                    pallet_xcm_core_buyer::Event::BuyCoreXcmSent { para_id }
                ) => {
                    para_id: *para_id == ParaId::from(PARATHREAD_ID),
                },
            ]
        );
    });
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
    // OnDemandOrderPlaced
}

/// Get parathread tank address in relay chain. This is derived from the Dancebox para id and the
/// parathread para id.
fn get_parathread_tank_relay_address() -> AccountId32 {
    let parathread_tank_in_relay = Dancebox::execute_with(|| {
        let parathread_tank_multilocation = XcmCoreBuyer::absolute_multilocation(
            XcmCoreBuyer::interior_multilocation(PARATHREAD_ID.into()),
        );
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
    do_test(balance_before);

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
}

#[test]
fn xcm_core_buyer_only_enough_balance_for_buy_execution() {
    let parathread_tank_in_relay = get_parathread_tank_relay_address();
    let balance_before = BUY_EXECUTION_COST;

    // Invariant: if balance_before >= BUY_EXECUTION_COST then
    // balance_after <= (balance_before + BUY_EXECUTION_REFUND - BUY_EXECUTION_COST)
    // In this case the balance_after is 0 because BUY_EXECUTION_REFUND < ROCOCO_ED,
    // so the account gets the refund but it is immediatelly killed.
    do_test(balance_before);

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
}

#[test]
fn xcm_core_buyer_enough_balance_except_for_existential_deposit() {
    let parathread_tank_in_relay = get_parathread_tank_relay_address();
    // Core price must be greater than existential deposit
    let spot_price = ROCOCO_ED + 1;
    set_on_demand_base_fee(spot_price);
    let spot_price2 = spot_price;
    let balance_before = BUY_EXECUTION_COST + spot_price;

    do_test(balance_before);

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
}

#[test]
fn xcm_core_buyer_enough_balance() {
    let parathread_tank_in_relay = get_parathread_tank_relay_address();
    let spot_price = get_on_demand_base_fee();
    let spot_price2 = spot_price;
    let balance_before = ROCOCO_ED + BUY_EXECUTION_COST + spot_price + 1;

    do_test(balance_before);

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
}

#[test]
fn xcm_core_buyer_core_too_expensive() {
    let parathread_tank_in_relay = get_parathread_tank_relay_address();
    let balance_before = ROCOCO_ED + BUY_EXECUTION_COST + 1;
    set_on_demand_base_fee(balance_before * 2);

    do_test(balance_before);

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
                        amount: BUY_EXECUTION_REFUND,
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
            balance_before + BUY_EXECUTION_REFUND - BUY_EXECUTION_COST
        );
    });
}
