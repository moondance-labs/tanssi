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
    core::marker::PhantomData,
    cumulus_primitives_core::Weight,
    dancebox_runtime::{Registrar, RuntimeOrigin, ServicesPayment, XcmCoreBuyer},
    dancebox_runtime_test_utils::{
        empty_genesis_data, run_to_session, set_dummy_boot_node, start_block,
    },
    frame_support::assert_ok,
    nimbus_primitives::NimbusId,
    pallet_xcm_core_buyer::RelayXcmWeightConfigInner,
    parity_scale_codec::Encode,
    polkadot_runtime_parachains::{configuration, on_demand as parachains_assigner_on_demand},
    rococo_emulated_chain::RococoRelayPallet,
    rococo_system_emulated_network::{
        DanceboxPara as Dancebox, RococoRelay as Rococo, RococoSender,
    },
    sp_core::Pair,
    sp_runtime::{traits::ValidateUnsigned, AccountId32},
    tanssi_emulated_integration_tests_common::accounts::BOB,
    tp_traits::{ParaId, SlotFrequency},
    westend_system_emulated_network::DanceboxSender,
    xcm::v3::QueryId,
    xcm_emulator::{assert_expected_events, Chain, RelayChain, TestExt},
    xcm_executor::traits::ConvertLocation,
};

pub const PARATHREAD_ID: u32 = 3333;
pub const ROCOCO_ED: u128 = rococo_runtime_constants::currency::EXISTENTIAL_DEPOSIT;
pub const BUY_EXECUTION_COST: u128 = dancebox_runtime::xcm_config::XCM_BUY_EXECUTION_COST_ROCOCO;
// Difference between BUY_EXECUTION_COST and the actual cost that depends on the weight of the XCM
// message, gets refunded on successful execution of core buying extrinsic.
pub const BUY_EXECUTION_REFUND: u128 = 19533231;
// Difference between BUY_EXECUTION_COST and the actual cost that depends on the weight of the XCM
// message, gets refunded on un-successful execution of core buying extrinsic.
pub const BUY_EXECUTION_REFUND_ON_FAILURE: u128 = 17199921;

pub const PLACE_ORDER_WEIGHT_AT_MOST: Weight = Weight::from_parts(1_000_000_000, 100_000);

pub fn assert_relay_order_event_not_emitted() {
    type RuntimeEvent = <Rococo as Chain>::RuntimeEvent;

    let events = <Rococo as Chain>::events();
    for event in events {
        if let RuntimeEvent::OnDemandAssignmentProvider(
            parachains_assigner_on_demand::Event::OnDemandOrderPlaced { .. },
        ) = event
        {
            panic!("Event should not have been emitted: {:?}", event);
        }
    }
}

pub fn assert_xcm_notification_event_not_emitted() {
    type RuntimeEvent = <Dancebox as Chain>::RuntimeEvent;

    let events = <Dancebox as Chain>::events();
    for event in events {
        if let RuntimeEvent::XcmCoreBuyer(
            pallet_xcm_core_buyer::Event::ReceivedBuyCoreXCMResult { .. },
        ) = event
        {
            panic!("Event should not have been emitted: {:?}", event);
        }
    }
}

pub fn find_query_id_for_para_id(para_id: ParaId) -> QueryId {
    type RuntimeEvent = <Dancebox as Chain>::RuntimeEvent;

    let events = <Dancebox as Chain>::events();
    for event in events {
        if let RuntimeEvent::XcmCoreBuyer(pallet_xcm_core_buyer::Event::BuyCoreXcmSent {
            para_id: event_para_id,
            transaction_status_query_id,
        }) = event
        {
            if event_para_id == para_id {
                return transaction_status_query_id;
            }
        }
    }

    panic!(
        "We should be able to find query_id for para_id: {:?}",
        para_id
    );
}

pub fn assert_query_response_success(para_id: ParaId, query_id: QueryId) {
    assert_query_response(para_id, query_id, true, true);
}

pub fn assert_query_response_failure(para_id: ParaId, query_id: QueryId) {
    assert_query_response(para_id, query_id, true, false);
}

pub fn assert_query_response_not_received(para_id: ParaId, query_id: QueryId) {
    assert_query_response(para_id, query_id, false, false);
}

pub fn assert_query_response(
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
pub fn get_parathread_tank_relay_address() -> AccountId32 {
    Dancebox::execute_with(|| {
        let parathread_tank_multilocation = XcmCoreBuyer::relay_relative_multilocation(
            XcmCoreBuyer::interior_multilocation(PARATHREAD_ID.into()),
        )
        .expect("reanchor failed");

        <Rococo as RelayChain>::SovereignAccountOf::convert_location(&parathread_tank_multilocation)
            .expect("probably this relay chain does not allow DescendOrigin")
    })
}

pub fn get_on_demand_base_fee() -> u128 {
    Rococo::execute_with(|| {
        let config = configuration::ActiveConfig::<<Rococo as Chain>::Runtime>::get();

        config.scheduler_params.on_demand_base_fee
    })
}

pub fn set_on_demand_base_fee(on_demand_base_fee: u128) {
    Rococo::execute_with(|| {
        let mut config = configuration::ActiveConfig::<<Rococo as Chain>::Runtime>::get();
        config.scheduler_params.on_demand_base_fee = on_demand_base_fee;
        <Rococo as RococoRelayPallet>::Configuration::force_set_active_config(config);
    });
}

/// The tests in this module all use this function to trigger an XCM message to buy a core.
///
/// Each test has a different value of
/// * tank_account_balance: the balance of the parachain tank account in the relay chain
/// * spot_price: the price of a core
pub fn do_test(
    tank_account_balance: u128,
    set_max_core_price: Option<u128>,
    is_forced: bool,
) -> QueryId {
    let mut query_id = QueryId::MAX;

    Dancebox::execute_with(|| {
        // Register parathread
        let alice_origin = <Dancebox as Chain>::RuntimeOrigin::signed(DanceboxSender::get());
        assert_ok!(Registrar::register_parathread(
            alice_origin.clone(),
            PARATHREAD_ID.into(),
            SlotFrequency { min: 1, max: 1 },
            empty_genesis_data(),
            None
        ));
        set_dummy_boot_node(alice_origin, PARATHREAD_ID.into());
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
                max_core_price
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
        if is_forced {
            assert_ok!(XcmCoreBuyer::force_buy_core(
                root_origin,
                PARATHREAD_ID.into()
            ));
        } else {
            core_buyer_sign_collator_nonce(
                PARATHREAD_ID.into(),
                get_aura_pair_from_seed(&dancebox_runtime::AccountId::from(BOB).to_string()),
            );
        }

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

fn core_buyer_sign_collator_nonce(para_id: ParaId, id: nimbus_primitives::NimbusPair) {
    let nonce =
        pallet_xcm_core_buyer::CollatorSignatureNonce::<dancebox_runtime::Runtime>::get(para_id);

    let payload = (nonce, para_id).encode();
    let signature = id.sign(&payload);
    let public_key = id.public();

    let proof = tp_xcm_core_buyer::BuyCoreCollatorProof::<NimbusId> {
        nonce,
        public_key: public_key.clone().into_inner().into(),
        signature,
    };
    XcmCoreBuyer::pre_dispatch(&pallet_xcm_core_buyer::Call::buy_core {
        para_id,
        proof: proof.clone(),
    })
    .expect("collator signature predispatch should go through");
    assert_ok!(XcmCoreBuyer::buy_core(
        RuntimeOrigin::none(),
        para_id,
        proof
    ));
}

pub fn get_aura_pair_from_seed(seed: &str) -> nimbus_primitives::NimbusPair {
    sp_core::sr25519::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .into()
}
