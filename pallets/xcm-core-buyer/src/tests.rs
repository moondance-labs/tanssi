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

use frame_support::assert_storage_noop;
use {
    crate::{mock::*, *},
    frame_support::{assert_noop, assert_ok},
    sp_runtime::traits::BadOrigin,
};

#[test]
fn root_origin_can_force_buy_xcm() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            run_to_block(1);
            let para_id = 3333.into();

            assert_ok!(XcmCoreBuyer::force_buy_core(RuntimeOrigin::root(), para_id,));

            let events = events();
            assert_eq!(events.len(), 1);
            matches!(events[0], Event::BuyCoreXcmSent { para_id: event_para_id, .. } if event_para_id == para_id);
        });
}

#[test]
fn signed_origin_cannot_force_buy_xcm() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            run_to_block(1);
            let para_id = 3333.into();

            assert_noop!(
                XcmCoreBuyer::force_buy_core(RuntimeOrigin::signed(ALICE), para_id),
                BadOrigin
            );
        });
}

#[test]
fn force_buy_two_messages_in_one_block_fails() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            run_to_block(1);
            let para_id = 3333.into();

            assert_ok!(XcmCoreBuyer::force_buy_core(RuntimeOrigin::root(), para_id,));

            let events = events();
            assert_eq!(events.len(), 1);
            matches!(events[0], Event::BuyCoreXcmSent { para_id: event_para_id, .. } if event_para_id == para_id);

            assert_noop!(
                XcmCoreBuyer::force_buy_core(RuntimeOrigin::root(), para_id),
                Error::<Test>::OrderAlreadyExists
            );
        });
}

#[test]
fn force_buy_two_messages_in_two_consecutive_blocks_failes() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            run_to_block(1);
            let para_id = 3333.into();

            assert_ok!(XcmCoreBuyer::force_buy_core(RuntimeOrigin::root(), para_id,));

            let events = events();
            assert_eq!(events.len(), 1);
            matches!(events[0], Event::BuyCoreXcmSent { para_id: event_para_id, .. } if event_para_id == para_id);

            run_to_block(2);

            assert_noop!(
                XcmCoreBuyer::force_buy_core(RuntimeOrigin::root(), para_id),
                Error::<Test>::OrderAlreadyExists
            );
        });
}

#[test]
fn force_buy_two_messages_succeds_after_receiving_order_failure_response() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            run_to_block(1);
            let para_id = 3333.into();

            assert_ok!(XcmCoreBuyer::force_buy_core(RuntimeOrigin::root(), para_id,));

            let system_events = events();
            assert_eq!(system_events.len(), 1);
            matches!(system_events[0], Event::BuyCoreXcmSent { para_id: event_para_id, .. } if event_para_id == para_id);
            let query_id = match system_events[0] {
                Event::BuyCoreXcmSent { transaction_status_query_id, .. } => transaction_status_query_id,
                _ => panic!("We checked for the event variant above; qed")
            };

            // QueryId -> ParaId mapping should exists
            assert_eq!(QueryIdToParaId::<Test>::get(query_id), Some(para_id));

            run_to_block(2);

            assert_noop!(
                XcmCoreBuyer::force_buy_core(RuntimeOrigin::root(), para_id),
                Error::<Test>::OrderAlreadyExists
            );

            assert_ok!(XcmCoreBuyer::query_response(RuntimeOrigin::root(), query_id, Response::DispatchResult(MaybeErrorCode::Error(BoundedVec::new()))));

            // In flight order entry should be removed
            assert!(InFlightOrders::<Test>::get(para_id).is_none());

            // Query id to para id entry should not exists
            assert!(QueryIdToParaId::<Test>::get(query_id).is_none());

            // We should not be adding entry into pending blocks data structure
            assert!(PendingBlocks::<Test>::get(para_id).is_none());

            run_to_block(4);

            assert_ok!(XcmCoreBuyer::force_buy_core(RuntimeOrigin::root(), para_id,));

            let system_events = events();
            assert_eq!(system_events.len(), 1);
            matches!(system_events[0], Event::BuyCoreXcmSent { para_id: event_para_id, .. } if event_para_id == para_id);
        });
}

#[test]
fn force_buy_two_messages_fails_after_receiving_order_success_response() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            run_to_block(2);
            let para_id = 3333.into();

            assert_ok!(XcmCoreBuyer::force_buy_core(RuntimeOrigin::root(), para_id,));

            let system_events = events();
            assert_eq!(system_events.len(), 1);
            matches!(system_events[0], Event::BuyCoreXcmSent { para_id: event_para_id, .. } if event_para_id == para_id);
            let query_id = match system_events[0] {
                Event::BuyCoreXcmSent { transaction_status_query_id, .. } => transaction_status_query_id,
                _ => panic!("We checked for the event variant above; qed")
            };

            // In flight order entry should exists
            let maybe_in_flight_order = InFlightOrders::<Test>::get(para_id);
            assert!(maybe_in_flight_order.is_some());
            let in_flight_order = maybe_in_flight_order.expect("Checked above for None; qed");
            assert_eq!(in_flight_order, InFlightCoreBuyingOrder {
                para_id,
                query_id,
                ttl: 2 + <Test as Config>::AdditionalTtlForInflightOrders::get() as u64 + <Test as Config>::CoreBuyingXCMQueryTtl::get() as u64,
            });

            // QueryId -> ParaId mapping should exists
            assert_eq!(QueryIdToParaId::<Test>::get(query_id), Some(para_id));

            run_to_block(3);

            assert_noop!(
                XcmCoreBuyer::force_buy_core(RuntimeOrigin::root(), para_id),
                Error::<Test>::OrderAlreadyExists
            );

            assert_ok!(XcmCoreBuyer::query_response(RuntimeOrigin::root(), query_id, Response::DispatchResult(MaybeErrorCode::Success)));

            // In flight order entry should be removed
            assert!(InFlightOrders::<Test>::get(para_id).is_none());

            // Query id to para id entry should not exists
            assert!(QueryIdToParaId::<Test>::get(query_id).is_none());

            // We should be adding entry into pending blocks data structure
            // We should not be adding entry into pending blocks data structure
            assert!(PendingBlocks::<Test>::get(para_id).is_some());

            run_to_block(4);

            assert_noop!(XcmCoreBuyer::force_buy_core(RuntimeOrigin::root(), para_id,), Error::<Test>::BlockProductionPending);

            // Now if the pallet gets notification that pending block for that para id is incremented it is possible to buy again.
            Pallet::<Test>::on_container_author_noted(&1u64, 5, para_id);
            assert_ok!(XcmCoreBuyer::force_buy_core(RuntimeOrigin::root(), para_id,));

            assert_ok!(XcmCoreBuyer::query_response(RuntimeOrigin::root(), query_id, Response::DispatchResult(MaybeErrorCode::Success)));

            // We should not be able to buy the core again since we have pending block
            assert_noop!(XcmCoreBuyer::force_buy_core(RuntimeOrigin::root(), para_id,), Error::<Test>::BlockProductionPending);

            // We can buy again after pending block ttl is passed
            let pending_blocks_ttl = PendingBlocks::<Test>::get(para_id).expect("We must have an entry for pending block ttl mapping as checked above; qed");
            run_to_block(4 + pending_blocks_ttl + 1);
            assert_ok!(XcmCoreBuyer::force_buy_core(RuntimeOrigin::root(), para_id,));
        });
}

#[test]
fn clean_up_in_flight_orders_extrinsic_works() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            run_to_block(2);
            let para_id = 3333.into();

            assert_ok!(XcmCoreBuyer::force_buy_core(RuntimeOrigin::root(), para_id,));

            let system_events = events();
            assert_eq!(system_events.len(), 1);
            matches!(system_events[0], Event::BuyCoreXcmSent { para_id: event_para_id, .. } if event_para_id == para_id);
            let query_id = match system_events[0] {
                Event::BuyCoreXcmSent { transaction_status_query_id, .. } => transaction_status_query_id,
                _ => panic!("We checked for the event variant above; qed")
            };

            // In flight order entry should exists
            let maybe_in_flight_order = InFlightOrders::<Test>::get(para_id);
            assert!(maybe_in_flight_order.is_some());
            let in_flight_order = maybe_in_flight_order.expect("Checked above for None; qed");
            assert_eq!(in_flight_order, InFlightCoreBuyingOrder {
                para_id,
                query_id,
                ttl: 2 + <Test as Config>::AdditionalTtlForInflightOrders::get() as u64 + <Test as Config>::CoreBuyingXCMQueryTtl::get() as u64,
            });

            // QueryId -> ParaId mapping should exists
            assert_eq!(QueryIdToParaId::<Test>::get(query_id), Some(para_id));

            run_to_block(3);

            // Cleaning up before ttl should be ignored
            assert_ok!(XcmCoreBuyer::clean_up_expired_in_flight_orders(RuntimeOrigin::signed(AccountId::default()), vec![para_id]));
            let system_events = events();
            assert_eq!(system_events.len(), 1);
            let para_ids_cleaned_up = match &system_events[0] {
                Event::CleanedUpExpiredInFlightOrderEntries { para_ids } => para_ids.clone(),
                _ => panic!("We checked for the event variant above; qed")
            };
            assert!(para_ids_cleaned_up.is_empty());

            // In flight order entry should still exists
            let maybe_in_flight_order = InFlightOrders::<Test>::get(para_id);
            assert!(maybe_in_flight_order.is_some());
            let query_id = maybe_in_flight_order.expect("Already checked existance above; qed").query_id;
            // Query id to para id entry should exists
            assert!(QueryIdToParaId::<Test>::get(query_id).is_some());

            // Cleaning up after ttl should work
            run_to_block(3 + <Test as Config>::AdditionalTtlForInflightOrders::get() as u64 + <Test as Config>::CoreBuyingXCMQueryTtl::get() as u64 + 1);
            assert_ok!(XcmCoreBuyer::clean_up_expired_in_flight_orders(RuntimeOrigin::signed(AccountId::default()), vec![para_id]));
            let system_events = events();
            assert_eq!(system_events.len(), 1);
            let para_ids_cleaned_up = match &system_events[0] {
                Event::CleanedUpExpiredInFlightOrderEntries { para_ids } => para_ids.clone(),
                _ => panic!("We checked for the event variant above; qed")
            };
            assert_eq!(para_ids_cleaned_up, vec![para_id]);

            // In flight order entry should not exist anymore
            let maybe_in_flight_order = InFlightOrders::<Test>::get(para_id);
            assert!(maybe_in_flight_order.is_none());

            // Query id to para id entry should not exists
            assert!(QueryIdToParaId::<Test>::get(query_id).is_none());
        });
}

#[test]
fn clean_up_pending_block_entries_extrinsic_works() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            run_to_block(2);
            let para_id = 3333.into();

            assert_ok!(XcmCoreBuyer::force_buy_core(RuntimeOrigin::root(), para_id,));

            let system_events = events();
            assert_eq!(system_events.len(), 1);
            matches!(system_events[0], Event::BuyCoreXcmSent { para_id: event_para_id, .. } if event_para_id == para_id);
            let query_id = match system_events[0] {
                Event::BuyCoreXcmSent { transaction_status_query_id, .. } => transaction_status_query_id,
                _ => panic!("We checked for the event variant above; qed")
            };

            // In flight order entry should exists
            let maybe_in_flight_order = InFlightOrders::<Test>::get(para_id);
            assert!(maybe_in_flight_order.is_some());
            let in_flight_order = maybe_in_flight_order.expect("Checked above for None; qed");
            assert_eq!(in_flight_order, InFlightCoreBuyingOrder {
                para_id,
                query_id,
                ttl: 2 + <Test as Config>::AdditionalTtlForInflightOrders::get() as u64 + <Test as Config>::CoreBuyingXCMQueryTtl::get() as u64,
            });

            assert_ok!(XcmCoreBuyer::query_response(RuntimeOrigin::root(), query_id, Response::DispatchResult(MaybeErrorCode::Success)));

            run_to_block(3);

            // Cleaning up before ttl should be ignored
            assert_ok!(XcmCoreBuyer::clean_up_expired_pending_blocks(RuntimeOrigin::signed(AccountId::default()), vec![para_id]));
            let system_events = events();
            assert_eq!(system_events.len(), 1);
            let para_ids_cleaned_up = match &system_events[0] {
                Event::CleanedUpExpiredPendingBlocksEntries { para_ids } => para_ids.clone(),
                _ => panic!("We checked for the event variant above; qed")
            };
            assert!(para_ids_cleaned_up.is_empty());

            // In flight order entry should still exists
            let maybe_pending_block_ttl = PendingBlocks::<Test>::get(para_id);
            assert!(maybe_pending_block_ttl.is_some());

            // Cleaning up after ttl should work
            run_to_block(3 + <Test as Config>::PendingBlocksTtl::get() as u64 + 1);
            assert_ok!(XcmCoreBuyer::clean_up_expired_pending_blocks(RuntimeOrigin::signed(AccountId::default()), vec![para_id]));
            let system_events = events();
            assert_eq!(system_events.len(), 1);
            let para_ids_cleaned_up = match &system_events[0] {
                Event::CleanedUpExpiredPendingBlocksEntries { para_ids } => para_ids.clone(),
                _ => panic!("We checked for the event variant above; qed")
            };
            assert_eq!(para_ids_cleaned_up, vec![para_id]);

            // In flight order entry should not exist anymore
            let maybe_pending_block_ttl = PendingBlocks::<Test>::get(para_id);
            assert!(maybe_pending_block_ttl.is_none());
        });
}

#[test]
fn core_order_expires_after_ttl() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            run_to_block(1);
            let para_id = 3333.into();

            assert_ok!(XcmCoreBuyer::force_buy_core(RuntimeOrigin::root(), para_id,));

            let system_events = events();
            assert_eq!(system_events.len(), 1);
            matches!(system_events[0], Event::BuyCoreXcmSent { para_id: event_para_id, .. } if event_para_id == para_id);

            run_to_block(2);

            assert_noop!(
                XcmCoreBuyer::force_buy_core(RuntimeOrigin::root(), para_id),
                Error::<Test>::OrderAlreadyExists
            );

            // We run to the ttl + 1 block, now even without query response received the order should have been expired
            let ttl = InFlightOrders::<Test>::get(para_id).expect("In flight order for para id must be there").ttl;
            run_to_block(ttl + 1);

            assert_ok!(XcmCoreBuyer::force_buy_core(RuntimeOrigin::root(), para_id,));
        });
}

#[test]
fn paraid_data_is_cleaned_up_at_deregistration() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            run_to_block(1);
            let para_id = 3333.into();

            assert_ok!(XcmCoreBuyer::force_buy_core(RuntimeOrigin::root(), para_id,));

            let system_events = events();
            assert_eq!(system_events.len(), 1);
            matches!(system_events[0], Event::BuyCoreXcmSent { para_id: event_para_id, .. } if event_para_id == para_id);

            run_to_block(2);

            let maybe_in_flight_order = InFlightOrders::<Test>::get(para_id);
            assert!(maybe_in_flight_order.is_some());
            let query_id = maybe_in_flight_order.expect("We already asserted that the value exists.").query_id;
            assert!(QueryIdToParaId::<Test>::get(query_id).is_some());

            XcmCoreBuyer::para_deregistered(para_id);

            // After de-registration the in flight order should be cleared
            assert!(InFlightOrders::<Test>::get(para_id).is_none());

            // After de-registration query id to para id mapping should be cleared
            assert!(QueryIdToParaId::<Test>::get(query_id).is_none());

            // It is no-op when para id is already de-registered
            assert_storage_noop!(XcmCoreBuyer::para_deregistered(para_id));

            // Adding a dummy pending block entry for para id
            PendingBlocks::<Test>::insert(para_id, 1u64);

            XcmCoreBuyer::para_deregistered(para_id);

            // After de-registration pending block entry should be cleared
            assert!(PendingBlocks::<Test>::get(para_id).is_none());
        });
}

#[test]
fn cannot_force_buy_invalid_para_id() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            run_to_block(1);
            let para_id = 2000.into();

            MockData::mutate(|m| {
                // Mock para_id 2000 as a container chain with collators, but not a parathread
                m.container_chain_collators.insert(2000.into(), vec![ALICE]);
            });

            assert_noop!(
                XcmCoreBuyer::force_buy_core(RuntimeOrigin::root(), para_id),
                Error::<Test>::NotAParathread
            );
        });
}

#[test]
fn cannot_force_buy_para_id_with_no_collators() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            run_to_block(1);
            let para_id = 3333.into();
            MockData::mutate(|m| {
                m.container_chain_collators = Default::default();
            });

            assert_noop!(
                XcmCoreBuyer::force_buy_core(RuntimeOrigin::root(), para_id),
                Error::<Test>::NoAssignedCollators
            );
        });
}

#[test]
fn cannot_buy_if_no_weights_storage_set() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            run_to_block(1);
            assert_ok!(XcmCoreBuyer::set_relay_xcm_weight_config(
                RuntimeOrigin::root(),
                None
            ));

            let para_id = 3333.into();

            assert_noop!(
                XcmCoreBuyer::force_buy_core(RuntimeOrigin::root(), para_id),
                Error::<Test>::XcmWeightStorageNotSet
            );
        });
}

#[test]
fn xcm_locations() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            run_to_block(1);

            let para_id = 3333.into();
            let interior_mloc = XcmCoreBuyer::interior_multilocation(para_id);
            let absolute_mloc = XcmCoreBuyer::relay_relative_multilocation(interior_mloc.clone()).expect("reanchor failed");

            assert_eq!(interior_mloc.len(), 1);
            assert_eq!(absolute_mloc.len(), 2);

            let (rest, first) = absolute_mloc.interior.split_first();
            assert_eq!(first, Some(Parachain(1000)));
            assert_eq!(rest, interior_mloc);

            // Print debug representation for informative purposes
            // The account id is `concat(b"para", u32::from(3333).to_le_bytes(), [0; 32 - 8])`
            assert_eq!(format!("{:?}", interior_mloc), "X1(AccountId32 { network: None, id: [112, 97, 114, 97, 5, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] })");
            assert_eq!(format!("{:?}", absolute_mloc), "MultiLocation { parents: 0, interior: X2(Parachain(1000), AccountId32 { network: None, id: [112, 97, 114, 97, 5, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }) }");
        });
}
