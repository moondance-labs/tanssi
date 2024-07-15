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
    crate::{mock::*, *},
    frame_support::{assert_noop, assert_ok, assert_storage_noop},
    nimbus_primitives::NimbusId,
    sp_runtime::{traits::BadOrigin, RuntimeAppPublic},
    tp_traits::ContainerChainBlockInfo,
};

#[test]
fn core_buying_nonce_behaviour_is_correct() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            run_to_block(1);

            let para_id = 3333.into();
            let another_para_id = 4444.into();

            {
                // Add another chain apart from default
                let mut mocks = MockData::mock();
                let another_chain_nimbus_id = NimbusId::generate_pair(None);
                mocks
                    .container_chain_collators
                    .insert(another_para_id, vec![another_chain_nimbus_id]);
                MockData::mutate(|stored_mock_data| {
                    stored_mock_data.container_chain_collators = mocks.container_chain_collators;
                    stored_mock_data.parathread_params.insert(
                        another_para_id,
                        ParathreadParams {
                            slot_frequency: Default::default(),
                        },
                    );
                });
            }

            let mocks = MockData::mock();
            let collator_data = mocks
                .container_chain_collators
                .get(&para_id)
                .expect("Collator data for test paraid must exists");
            assert!(
                !collator_data.is_empty(),
                "collator data must contain at least one element"
            );
            let collator = collator_data[0].clone();
            let proof = BuyCoreCollatorProof::new(0, para_id, collator)
                .expect("creating collator proof must succeed");

            let call = Call::buy_core {
                para_id,
                proof: proof.clone(),
            };

            // Initial nonce should be zero
            assert_eq!(CollatorSignatureNonce::<Test>::get(para_id), 0);

            assert_ok!(<XcmCoreBuyer as ValidateUnsigned>::validate_unsigned(
                TransactionSource::External,
                &call
            ));
            assert_ok!(XcmCoreBuyer::buy_core(
                RuntimeOrigin::none(),
                para_id,
                proof
            ));

            assert_noop!(
                <XcmCoreBuyer as ValidateUnsigned>::validate_unsigned(
                    TransactionSource::External,
                    &call
                ),
                TransactionValidityError::Invalid(InvalidTransaction::Call)
            );

            // Now, it should be 1
            assert_eq!(CollatorSignatureNonce::<Test>::get(para_id), 1);

            // We should be able to purchase core for another para id with 0 nonce
            let collator_data = mocks
                .container_chain_collators
                .get(&another_para_id)
                .expect("Collator data for test paraid must exists");
            let another_collator = collator_data[0].clone();
            let proof = BuyCoreCollatorProof::new(0, another_para_id, another_collator)
                .expect("creating collator proof must succeed");
            let call = Call::buy_core {
                para_id: another_para_id,
                proof: proof.clone(),
            };

            // Initial nonce should be zero
            assert_eq!(CollatorSignatureNonce::<Test>::get(another_para_id), 0);

            assert_ok!(<XcmCoreBuyer as ValidateUnsigned>::validate_unsigned(
                TransactionSource::External,
                &call
            ));
            assert_ok!(XcmCoreBuyer::buy_core(
                RuntimeOrigin::none(),
                another_para_id,
                proof
            ));

            // Now, it should be 1
            assert_eq!(CollatorSignatureNonce::<Test>::get(another_para_id), 1);
        })
}

#[test]
fn core_buying_proof_is_validated_correctly() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            run_to_block(1);

            let para_id = 3333.into();
            let another_para_id = 4444.into();

            {
                // Add another chain apart from default
                let mut mocks = MockData::mock();
                let another_chain_nimbus_id = NimbusId::generate_pair(None);
                mocks
                    .container_chain_collators
                    .insert(another_para_id, vec![another_chain_nimbus_id]);
                MockData::mutate(|stored_mock_data| {
                    stored_mock_data.container_chain_collators = mocks.container_chain_collators;
                    stored_mock_data.parathread_params.insert(
                        another_para_id,
                        ParathreadParams {
                            slot_frequency: Default::default(),
                        },
                    );
                });
            }

            let mocks = MockData::mock();
            let collator_data = mocks
                .container_chain_collators
                .get(&para_id)
                .expect("Collator data for test paraid must exists");
            assert!(
                !collator_data.is_empty(),
                "collator data must contain at least one element"
            );
            let collator = collator_data[0].clone();
            let proof = BuyCoreCollatorProof::new(0, para_id, collator.clone())
                .expect("creating collator proof must succeed");

            let call = Call::buy_core {
                para_id,
                proof: proof.clone(),
            };

            assert_ok!(<XcmCoreBuyer as ValidateUnsigned>::validate_unsigned(
                TransactionSource::External,
                &call
            ));

            // If we change the public key in the proof, it should not work
            let mut proof = BuyCoreCollatorProof::new(0, para_id, collator.clone())
                .expect("creating collator proof must succeed");
            proof.public_key = NimbusId::generate_pair(None);

            let call = Call::buy_core {
                para_id,
                proof: proof.clone(),
            };

            assert_noop!(
                <XcmCoreBuyer as ValidateUnsigned>::validate_unsigned(
                    TransactionSource::External,
                    &call
                ),
                TransactionValidityError::Invalid(InvalidTransaction::Call)
            );

            // If we change the signature, it should not work
            let mut proof = BuyCoreCollatorProof::new(0, para_id, collator.clone())
                .expect("creating collator proof must succeed");
            let incorrect_signature = collator
                .sign(&vec![1, 2, 3])
                .expect("signature creation must succeed.");
            proof.signature = incorrect_signature;
            let call = Call::buy_core {
                para_id,
                proof: proof.clone(),
            };

            assert_noop!(
                <XcmCoreBuyer as ValidateUnsigned>::validate_unsigned(
                    TransactionSource::External,
                    &call
                ),
                TransactionValidityError::Invalid(InvalidTransaction::Call)
            );

            // If we change the nonce, it should not work
            let mut proof = BuyCoreCollatorProof::new(0, para_id, collator.clone())
                .expect("creating collator proof must succeed");
            proof.nonce = 12;
            let call = Call::buy_core {
                para_id,
                proof: proof.clone(),
            };
            assert_noop!(
                <XcmCoreBuyer as ValidateUnsigned>::validate_unsigned(
                    TransactionSource::External,
                    &call
                ),
                TransactionValidityError::Invalid(InvalidTransaction::Call)
            );

            // If we change para id, it should not work
            let proof = BuyCoreCollatorProof::new(0, para_id, collator.clone())
                .expect("creating collator proof must succeed");
            let call = Call::buy_core {
                para_id: another_para_id,
                proof: proof.clone(),
            };
            assert_noop!(
                <XcmCoreBuyer as ValidateUnsigned>::validate_unsigned(
                    TransactionSource::External,
                    &call
                ),
                TransactionValidityError::Invalid(InvalidTransaction::Call)
            );

            let proof = BuyCoreCollatorProof::new(0, another_para_id, collator.clone())
                .expect("creating collator proof must succeed");
            let call = Call::buy_core {
                para_id: another_para_id,
                proof: proof.clone(),
            };
            assert_noop!(
                <XcmCoreBuyer as ValidateUnsigned>::validate_unsigned(
                    TransactionSource::External,
                    &call
                ),
                TransactionValidityError::Invalid(InvalidTransaction::Call)
            );

            let proof = BuyCoreCollatorProof::new(0, another_para_id, collator.clone())
                .expect("creating collator proof must succeed");
            let call = Call::buy_core {
                para_id,
                proof: proof.clone(),
            };
            assert_noop!(
                <XcmCoreBuyer as ValidateUnsigned>::validate_unsigned(
                    TransactionSource::External,
                    &call
                ),
                TransactionValidityError::Invalid(InvalidTransaction::Call)
            );
        })
}

#[test]
fn slot_frequency_is_taken_into_account() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            run_to_block(1);

            let para_id = 3333.into();
            let another_para_id = 4444.into();

            {
                // Add another chain apart from default
                let mut mocks = MockData::mock();
                let another_chain_nimbus_id = NimbusId::generate_pair(None);
                mocks.container_chain_collators.insert(another_para_id, vec![another_chain_nimbus_id]);
                MockData::mutate(|stored_mock_data| {
                    stored_mock_data.container_chain_collators = mocks.container_chain_collators;
                    stored_mock_data.parathread_params.insert(another_para_id, ParathreadParams { slot_frequency: SlotFrequency { min: 10, max: 10 } });
                });
            }

            // SlotFrequency with min: 1 slot works
            let mocks = MockData::mock();
            let collator_data = mocks.container_chain_collators.get(&para_id).expect("Collator data for test paraid must exists");
            assert!(!collator_data.is_empty(), "collator data must contain at least one element");
            let collator = collator_data[0].clone();
            let proof = BuyCoreCollatorProof::new(0, para_id, collator.clone()).expect("creating collator proof must succeed");

            let call = Call::buy_core {
                para_id,
                proof: proof.clone(),
            };

            assert_ok!(<XcmCoreBuyer as ValidateUnsigned>::validate_unsigned(TransactionSource::External, &call));
            assert_ok!(XcmCoreBuyer::buy_core(RuntimeOrigin::none(), para_id, proof));


            // Clear data to able to attempt to buy core again
            let system_events = events();
            assert_eq!(system_events.len(), 1);
            matches!(system_events[0], Event::BuyCoreXcmSent { para_id: event_para_id, .. } if event_para_id == para_id);
            let query_id = match system_events[0] {
                Event::BuyCoreXcmSent { transaction_status_query_id, .. } => transaction_status_query_id,
                _ => panic!("We checked for the event variant above; qed")
            };
            assert_ok!(XcmCoreBuyer::query_response(RuntimeOrigin::root(), query_id, Response::DispatchResult(MaybeErrorCode::Error(BoundedVec::new()))));

            // We should be able to buy slot once again for min: 1 Slot frequency
            let proof = BuyCoreCollatorProof::new(1, para_id, collator.clone()).expect("creating collator proof must succeed");

            let call = Call::buy_core {
                para_id,
                proof: proof.clone(),
            };

            assert_ok!(<XcmCoreBuyer as ValidateUnsigned>::validate_unsigned(TransactionSource::External, &call));
            assert_ok!(XcmCoreBuyer::buy_core(RuntimeOrigin::none(), para_id, proof));


            // For a para id with min: 10 slot frequency, only possible to buy after 10 - 2(advance slot allowed to buy)

            let collator_data = mocks.container_chain_collators.get(&another_para_id).expect("Collator data for test paraid must exists");
            assert!(!collator_data.is_empty(), "collator data must contain at least one element");
            let collator = collator_data[0].clone();

            let proof = BuyCoreCollatorProof::new(0, another_para_id, collator.clone()).expect("creating collator proof must succeed");

            let call = Call::buy_core {
                para_id: another_para_id,
                proof: proof.clone(),
            };

            assert_ok!(<XcmCoreBuyer as ValidateUnsigned>::validate_unsigned(TransactionSource::External, &call));
            assert_ok!(XcmCoreBuyer::buy_core(RuntimeOrigin::none(), another_para_id, proof));

            let mut system_events = events();
            system_events.reverse(); // Hack to get the newest query id
            matches!(system_events[0], Event::BuyCoreXcmSent { para_id: event_para_id, .. } if event_para_id == para_id);
            let query_id = match system_events[0] {
                Event::BuyCoreXcmSent { transaction_status_query_id, .. } => transaction_status_query_id,
                _ => panic!("We checked for the event variant above; qed")
            };
            assert_ok!(XcmCoreBuyer::query_response(RuntimeOrigin::root(), query_id, Response::DispatchResult(MaybeErrorCode::Success)));
            Pallet::<Test>::on_container_author_noted(&1u64, 5, another_para_id);

            // Add latest author info entry to indicate block was produced
            MockData::mutate(|stored_mock_data| {
                stored_mock_data.latest_author_info.insert(another_para_id, ContainerChainBlockInfo {
                    block_number: 0,
                    author: BOB,
                    latest_slot_number: Default::default(),
                });
            });

            let proof = BuyCoreCollatorProof::new(1, another_para_id, collator.clone()).expect("creating collator proof must succeed");

            let call = Call::buy_core {
                para_id: another_para_id,
                proof: proof.clone(),
            };

            assert_ok!(<XcmCoreBuyer as ValidateUnsigned>::validate_unsigned(TransactionSource::External, &call));
            // We are not able to buy due to slot frequency being min: 10-2 = 8
            assert_noop!(XcmCoreBuyer::buy_core(RuntimeOrigin::none(), another_para_id, proof.clone()), Error::<Test>::NotAllowedToProduceBlockRightNow);

            // We are still one slot short
            run_to_block(7);
            assert_noop!(XcmCoreBuyer::buy_core(RuntimeOrigin::none(), another_para_id, proof.clone()), Error::<Test>::NotAllowedToProduceBlockRightNow);

            // We should be able to produce block at slot: 8
            run_to_block(8);
            assert_ok!(XcmCoreBuyer::buy_core(RuntimeOrigin::none(), another_para_id, proof));
        })
}

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
                m.container_chain_collators
                    .insert(2000.into(), vec![NimbusId::generate_pair(None)]);
            });

            assert_noop!(
                XcmCoreBuyer::force_buy_core(RuntimeOrigin::root(), para_id),
                Error::<Test>::NotAParathread
            );
        });
}

#[test]
fn able_to_force_buy_para_id_with_no_collators() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            run_to_block(1);
            let para_id = 3333.into();
            MockData::mutate(|m| {
                m.container_chain_collators = Default::default();
            });

            assert_ok!(XcmCoreBuyer::force_buy_core(RuntimeOrigin::root(), para_id));
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

            let (rest, first) = absolute_mloc.interior.clone().split_first();
            assert_eq!(first, Some(Parachain(1000)));
            assert_eq!(rest, interior_mloc);

            // Print debug representation for informative purposes
            // The account id is `concat(b"para", u32::from(3333).to_le_bytes(), [0; 32 - 8])`
            assert_eq!(format!("{:?}", interior_mloc), "X1([AccountId32 { network: None, id: [112, 97, 114, 97, 5, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }])");
            assert_eq!(format!("{:?}", absolute_mloc), "Location { parents: 0, interior: X2([Parachain(1000), AccountId32 { network: None, id: [112, 97, 114, 97, 5, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }]) }");
        });
}
