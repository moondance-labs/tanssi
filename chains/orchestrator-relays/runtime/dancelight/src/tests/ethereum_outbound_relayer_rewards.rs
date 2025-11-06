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

#![cfg(test)]

use {
    crate::{
        bridge_to_ethereum_config::BridgeReward, tests::common::*, BridgeRelayers,
        EthereumOutboundQueueV2, SnowbridgeFeesAccount,
    },
    alloc::vec,
    alloy_core::sol_types::SolEvent,
    frame_support::{assert_noop, assert_ok},
    snowbridge_outbound_queue_primitives::{v2::InboundMessageDispatched, EventProof, Log},
    snowbridge_pallet_outbound_queue_v2::PendingOrder,
    sp_core::H256,
    sp_runtime::traits::BlockNumberProvider,
};

#[test]
fn test_rewards_are_paid_in_tanssi_after_succesful_claim() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
            (SnowbridgeFeesAccount::get(), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);
            // First we insert a reward
            let order = PendingOrder {
                nonce: 0,
                fee: 1 * UNIT,
                block_number: frame_system::Pallet::<Runtime>::current_block_number(),
            };
            snowbridge_pallet_outbound_queue_v2::PendingOrders::<Runtime>::insert(0, order);

            // The topic does not matter, it does not play a role.
            // The reward address does though
            let event = InboundMessageDispatched {
                topic: BOB.into(),
                nonce: 0,
                success: true,
                reward_address: BOB.into(),
            };

            // Then we claim
            // this generates the snowbridge event faked to be believed
            let message = EventProof {
                event_log: Log {
                    address:
                        <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                    topics: event
                        .encode_topics()
                        .into_iter()
                        .map(|word| H256::from(word.0 .0))
                        .collect(),
                    data: event.encode_data(),
                },
                proof: mock_snowbridge_message_proof(),
            };

            let fees_account_balance_before = Balances::free_balance(SnowbridgeFeesAccount::get());
            let bob_balance_before = Balances::free_balance(AccountId::from(BOB));

            assert_ok!(EthereumOutboundQueueV2::submit_delivery_receipt(
                RuntimeOrigin::signed(AccountId::from(ALICE)),
                Box::new(message)
            ));

            assert_ok!(BridgeRelayers::claim_rewards(
                RuntimeOrigin::signed(AccountId::from(BOB)),
                BridgeReward::SnowbridgeRewardOutbound
            ));

            let fees_account_balance_after = Balances::free_balance(SnowbridgeFeesAccount::get());
            let bob_balance_after = Balances::free_balance(AccountId::from(BOB));

            assert_eq!(
                fees_account_balance_after,
                fees_account_balance_before - 1 * UNIT
            );
            assert_eq!(bob_balance_after, bob_balance_before + 1 * UNIT);
        });
}

#[test]
fn test_rewards_are_accumulated() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
            (SnowbridgeFeesAccount::get(), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);
            // First we insert a reward
            let order = PendingOrder {
                nonce: 0,
                fee: 1 * UNIT,
                block_number: frame_system::Pallet::<Runtime>::current_block_number(),
            };
            snowbridge_pallet_outbound_queue_v2::PendingOrders::<Runtime>::insert(0, &order);
            snowbridge_pallet_outbound_queue_v2::PendingOrders::<Runtime>::insert(1, &order);

            // The topic does not matter, it does not play a role.
            // The reward address does though
            let event = InboundMessageDispatched {
                topic: BOB.into(),
                nonce: 0,
                success: true,
                reward_address: BOB.into(),
            };

            let event2 = InboundMessageDispatched {
                topic: BOB.into(),
                nonce: 1,
                success: true,
                reward_address: BOB.into(),
            };

            // Then we claim
            // this generates the snowbridge event faked to be believed
            let message = EventProof {
                event_log: Log {
                    address:
                        <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                    topics: event
                        .encode_topics()
                        .into_iter()
                        .map(|word| H256::from(word.0 .0))
                        .collect(),
                    data: event.encode_data(),
                },
                proof: mock_snowbridge_message_proof(),
            };

            // Then we claim
            // this generates the snowbridge event faked to be believed
            let message2 = EventProof {
                event_log: Log {
                    address:
                        <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                    topics: event2
                        .encode_topics()
                        .into_iter()
                        .map(|word| H256::from(word.0 .0))
                        .collect(),
                    data: event2.encode_data(),
                },
                proof: mock_snowbridge_message_proof(),
            };

            let fees_account_balance_before = Balances::free_balance(SnowbridgeFeesAccount::get());
            let bob_balance_before = Balances::free_balance(AccountId::from(BOB));

            assert_ok!(EthereumOutboundQueueV2::submit_delivery_receipt(
                RuntimeOrigin::signed(AccountId::from(ALICE)),
                Box::new(message)
            ));

            assert_ok!(EthereumOutboundQueueV2::submit_delivery_receipt(
                RuntimeOrigin::signed(AccountId::from(ALICE)),
                Box::new(message2)
            ));

            assert_ok!(BridgeRelayers::claim_rewards(
                RuntimeOrigin::signed(AccountId::from(BOB)),
                BridgeReward::SnowbridgeRewardOutbound
            ));

            let fees_account_balance_after = Balances::free_balance(SnowbridgeFeesAccount::get());
            let bob_balance_after = Balances::free_balance(AccountId::from(BOB));

            // The reward is accumulated, 1 unit per message, single claim receives everything
            assert_eq!(
                fees_account_balance_after,
                fees_account_balance_before - 2 * UNIT
            );
            assert_eq!(bob_balance_after, bob_balance_before + 2 * UNIT);
        });
}

#[test]
fn test_message_is_not_deliverable_if_pending_order_is_non_existent() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
            (SnowbridgeFeesAccount::get(), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);
            // We intentionally do not insert any order

            // The topic does not matter, it does not play a role.
            // The reward address does though
            let event = InboundMessageDispatched {
                topic: BOB.into(),
                nonce: 0,
                success: true,
                reward_address: BOB.into(),
            };

            // Then we claim
            // this generates the snowbridge event faked to be believed
            let message = EventProof {
                event_log: Log {
                    address:
                        <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                    topics: event
                        .encode_topics()
                        .into_iter()
                        .map(|word| H256::from(word.0 .0))
                        .collect(),
                    data: event.encode_data(),
                },
                proof: mock_snowbridge_message_proof(),
            };

            assert_noop!(
                EthereumOutboundQueueV2::submit_delivery_receipt(
                    RuntimeOrigin::signed(AccountId::from(ALICE)),
                    Box::new(message)
                ),
                snowbridge_pallet_outbound_queue_v2::Error::<Runtime>::InvalidPendingNonce
            );
        });
}

#[test]
fn test_rewards_are_not_payable_if_account_does_not_have_enough_funds() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
            (SnowbridgeFeesAccount::get(), 1 * UNIT),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);
            // First we insert a reward
            // the snowbridge fees account is initialized with 1 unit
            // let's put 2
            let order = PendingOrder {
                nonce: 0,
                fee: 2 * UNIT,
                block_number: frame_system::Pallet::<Runtime>::current_block_number(),
            };
            snowbridge_pallet_outbound_queue_v2::PendingOrders::<Runtime>::insert(0, order);

            // The topic does not matter, it does not play a role.
            // The reward address does though
            let event = InboundMessageDispatched {
                topic: BOB.into(),
                nonce: 0,
                success: true,
                reward_address: BOB.into(),
            };

            // Then we claim
            // this generates the snowbridge event faked to be believed
            let message = EventProof {
                event_log: Log {
                    address:
                        <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                    topics: event
                        .encode_topics()
                        .into_iter()
                        .map(|word| H256::from(word.0 .0))
                        .collect(),
                    data: event.encode_data(),
                },
                proof: mock_snowbridge_message_proof(),
            };

            assert_ok!(EthereumOutboundQueueV2::submit_delivery_receipt(
                RuntimeOrigin::signed(AccountId::from(ALICE)),
                Box::new(message)
            ));

            assert_noop!(
                BridgeRelayers::claim_rewards(
                    RuntimeOrigin::signed(AccountId::from(BOB)),
                    BridgeReward::SnowbridgeRewardOutbound
                ),
                pallet_bridge_relayers::Error::<Runtime>::FailedToPayReward
            );
        });
}
