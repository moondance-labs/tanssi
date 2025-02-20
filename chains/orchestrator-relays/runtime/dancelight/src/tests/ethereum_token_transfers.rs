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
        bridge_to_ethereum_config::{EthereumGatewayAddress, TokenTransferMessageProcessor},
        tests::common::*,
        Balances, EthereumInboundQueue, EthereumSovereignAccount, EthereumSystem,
        EthereumTokenTransfers, RuntimeEvent, TokenLocationReanchored, TreasuryAccount,
    },
    alloy_sol_types::SolEvent,
    frame_support::{assert_noop, assert_ok},
    hex_literal::hex,
    parity_scale_codec::Encode,
    snowbridge_core::{
        inbound::{Log, Message},
        AgentId, Channel, ChannelId, ParaId,
    },
    snowbridge_router_primitives::inbound::{
        envelope::{Envelope, OutboundMessageAccepted},
        Command, Destination, MessageProcessor, MessageV1, VersionedXcmMessage,
    },
    sp_core::{H160, H256},
    sp_runtime::{traits::MaybeEquivalence, TokenError},
    sp_std::vec,
    xcm::{latest::Location, VersionedLocation},
};

#[test]
fn test_set_token_transfer_channel_reflects_changes_in_ethereum_system() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);
            let channel_id = ChannelId::new([5u8; 32]);
            let agent_id = AgentId::from_low_u64_be(10);
            let para_id: ParaId = 2000u32.into();

            assert!(EthereumTokenTransfers::current_channel_info().is_none());

            assert!(EthereumSystem::agents(agent_id).is_none());
            assert!(EthereumSystem::channels(channel_id).is_none());

            assert_ok!(EthereumTokenTransfers::set_token_transfer_channel(
                root_origin(),
                channel_id,
                agent_id,
                para_id
            ));

            let new_channel_info = EthereumTokenTransfers::current_channel_info();

            assert_eq!(new_channel_info.clone().unwrap().channel_id, channel_id);
            assert_eq!(new_channel_info.clone().unwrap().para_id, para_id);
            assert_eq!(new_channel_info.unwrap().agent_id, agent_id);

            assert!(EthereumSystem::agents(agent_id).is_some());
            assert!(EthereumSystem::channels(channel_id).is_some());

            // PartialEq is not implemented for Channel, so we compare each element individually.
            let expected_channel = EthereumSystem::channels(channel_id).unwrap();
            assert_eq!(expected_channel.para_id, para_id);
            assert_eq!(expected_channel.agent_id, agent_id);
        });
}

#[test]
fn test_set_token_transfer_channel_works_with_existing_channels_from_eth_system() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);
            let channel_id = ChannelId::new([5u8; 32]);
            let agent_id = AgentId::from_low_u64_be(10);
            let para_id: ParaId = 2000u32.into();

            assert!(EthereumTokenTransfers::current_channel_info().is_none());

            // Let's first insert both agent_id and channel_id into storage.
            assert!(EthereumSystem::agents(agent_id).is_none());
            assert!(EthereumSystem::channels(channel_id).is_none());

            snowbridge_pallet_system::Agents::<Runtime>::insert(agent_id, ());
            snowbridge_pallet_system::Channels::<Runtime>::insert(
                channel_id,
                Channel { para_id, agent_id },
            );
            assert!(EthereumSystem::agents(agent_id).is_some());
            assert!(EthereumSystem::channels(channel_id).is_some());

            // Call should work with the existing channels from EthereumSystem.
            assert_ok!(EthereumTokenTransfers::set_token_transfer_channel(
                root_origin(),
                channel_id,
                agent_id,
                para_id
            ));

            assert!(EthereumTokenTransfers::current_channel_info().is_some());

            let expected_channel_info = EthereumTokenTransfers::current_channel_info().unwrap();

            assert_eq!(expected_channel_info.channel_id, channel_id);
            assert_eq!(expected_channel_info.para_id, para_id);
            assert_eq!(expected_channel_info.agent_id, agent_id);

            // Ensure that everything remains the same in EthereumSystem.
            assert!(EthereumSystem::agents(agent_id).is_some());
            assert!(EthereumSystem::channels(channel_id).is_some());

            let expected_channel = EthereumSystem::channels(channel_id).unwrap();
            assert_eq!(expected_channel.para_id, para_id);
            assert_eq!(expected_channel.agent_id, agent_id);
        });
}

#[test]
fn test_transfer_native_token() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);
            let token_location: VersionedLocation = Location::here().into();

            assert_ok!(EthereumSystem::register_token(
                root_origin(),
                Box::new(token_location),
                snowbridge_core::AssetMetadata {
                    name: "dance".as_bytes().to_vec().try_into().unwrap(),
                    symbol: "dance".as_bytes().to_vec().try_into().unwrap(),
                    decimals: 12,
                }
            ));

            run_to_block(4);
            let channel_id = ChannelId::new([5u8; 32]);
            let agent_id = AgentId::from_low_u64_be(10);
            let para_id: ParaId = 2000u32.into();

            assert_ok!(EthereumTokenTransfers::set_token_transfer_channel(
                root_origin(),
                channel_id,
                agent_id,
                para_id
            ));

            let alice_balance_before = Balances::free_balance(AccountId::from(ALICE));

            // No balance in Ethereum sovereign and treasury accounts yet.
            assert_eq!(
                Balances::free_balance(EthereumSovereignAccount::get()),
                0u128
            );
            assert_eq!(Balances::free_balance(TreasuryAccount::get()), 0u128);

            let amount_to_transfer = 100 * UNIT;
            let recipient = H160::random();
            assert_ok!(EthereumTokenTransfers::transfer_native_token(
                origin_of(AccountId::from(ALICE)),
                amount_to_transfer,
                recipient
            ));

            let outbound_msg_queue_event = System::events()
                .iter()
                .filter(|r| match r.event {
                    RuntimeEvent::EthereumOutboundQueue(
                        snowbridge_pallet_outbound_queue::Event::MessageQueued { .. },
                    ) => true,
                    _ => false,
                })
                .count();

            assert_eq!(
                outbound_msg_queue_event, 1,
                "MessageQueued event should be emitted!"
            );

            // Check event's integrity.
            let mut message_id_found: Option<H256> = None;
            let mut channel_id_found = ChannelId::default();
            let mut source_found = AccountId::from([0u8; 32]);
            let mut recipient_found = H160::default();
            let mut token_id_found = H256::default();
            let mut amount_found = 0u128;
            let mut fee_found = 0u128;

            let token_transfer_event = System::events()
                .iter()
                .filter(|r| match &r.event {
                    RuntimeEvent::EthereumTokenTransfers(
                        pallet_ethereum_token_transfers::Event::NativeTokenTransferred {
                            message_id,
                            channel_id,
                            source,
                            recipient,
                            token_id,
                            amount,
                            fee,
                        },
                    ) => {
                        message_id_found = Some(*message_id);
                        channel_id_found = *channel_id;
                        source_found = source.clone();
                        recipient_found = *recipient;
                        token_id_found = *token_id;
                        amount_found = *amount;
                        fee_found = *fee;
                        true
                    }
                    _ => false,
                })
                .count();

            assert_eq!(
                token_transfer_event, 1,
                "NativeTokenTransferred event should be emitted!"
            );

            let expected_token_id = EthereumSystem::convert_back(&TokenLocationReanchored::get());

            // Check channel data and transfer info.
            assert_eq!(message_id_found.unwrap(), read_last_entropy().into());
            assert_eq!(channel_id_found, channel_id);
            assert_eq!(source_found, AccountId::from(ALICE));
            assert_eq!(recipient_found, recipient);
            assert_eq!(token_id_found, expected_token_id.unwrap());
            assert_eq!(amount_found, amount_to_transfer);

            // Check balances consistency.
            assert_eq!(
                Balances::free_balance(AccountId::from(ALICE)),
                alice_balance_before - amount_to_transfer - fee_found
            );
            assert_eq!(
                Balances::free_balance(EthereumSovereignAccount::get()),
                amount_to_transfer
            );
            assert_eq!(Balances::free_balance(TreasuryAccount::get()), fee_found);
        });
}

#[test]
fn test_transfer_native_token_fails_if_channel_info_not_set() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);
            let token_location: VersionedLocation = Location::here().into();

            assert_ok!(EthereumSystem::register_token(
                root_origin(),
                Box::new(token_location),
                snowbridge_core::AssetMetadata {
                    name: "dance".as_bytes().to_vec().try_into().unwrap(),
                    symbol: "dance".as_bytes().to_vec().try_into().unwrap(),
                    decimals: 12,
                }
            ));

            run_to_block(4);

            let amount_to_transfer = 100 * UNIT;
            let recipient = H160::random();

            assert_noop!(
                EthereumTokenTransfers::transfer_native_token(
                    origin_of(AccountId::from(ALICE)),
                    amount_to_transfer,
                    recipient
                ),
                pallet_ethereum_token_transfers::Error::<Runtime>::ChannelInfoNotSet
            );
        });
}

#[test]
fn test_transfer_native_token_fails_if_token_not_registered_in_ethereum_system() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);

            let channel_id = ChannelId::new([5u8; 32]);
            let agent_id = AgentId::from_low_u64_be(10);
            let para_id: ParaId = 2000u32.into();

            assert_ok!(EthereumTokenTransfers::set_token_transfer_channel(
                root_origin(),
                channel_id,
                agent_id,
                para_id
            ));

            let amount_to_transfer = 100 * UNIT;
            let recipient = H160::random();

            assert_noop!(
                EthereumTokenTransfers::transfer_native_token(
                    origin_of(AccountId::from(ALICE)),
                    amount_to_transfer,
                    recipient
                ),
                pallet_ethereum_token_transfers::Error::<Runtime>::UnknownLocationForToken
            );
        });
}

#[test]
fn test_transfer_native_token_fails_if_not_enough_balance() {
    ExtBuilder::default()
        .with_balances(vec![
            (AccountId::from(ALICE), 50_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);
            let token_location: VersionedLocation = Location::here().into();

            assert_ok!(EthereumSystem::register_token(
                root_origin(),
                Box::new(token_location),
                snowbridge_core::AssetMetadata {
                    name: "dance".as_bytes().to_vec().try_into().unwrap(),
                    symbol: "dance".as_bytes().to_vec().try_into().unwrap(),
                    decimals: 12,
                }
            ));

            run_to_block(4);

            let channel_id = ChannelId::new([5u8; 32]);
            let agent_id = AgentId::from_low_u64_be(10);
            let para_id: ParaId = 2000u32.into();

            assert_ok!(EthereumTokenTransfers::set_token_transfer_channel(
                root_origin(),
                channel_id,
                agent_id,
                para_id
            ));

            // Try to send more than the account's balance.
            let amount_to_transfer = 70_000 * UNIT;
            let recipient = H160::random();

            assert_noop!(
                EthereumTokenTransfers::transfer_native_token(
                    origin_of(AccountId::from(ALICE)),
                    amount_to_transfer,
                    recipient
                ),
                TokenError::FundsUnavailable
            );
        });
}

#[test]
fn receive_native_tokens_from_eth_happy_path() {
    ExtBuilder::default()
        .with_balances(vec![
            (EthereumSovereignAccount::get(), 100_000 * UNIT),
            (TreasuryAccount::get(), 100_000 * UNIT),
            (AccountId::from(ALICE), 100_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            let relayer =
                <Runtime as frame_system::Config>::RuntimeOrigin::signed(AccountId::from(ALICE));

            let channel_id: ChannelId = ChannelId::new(hex!(
                "00000000000000000000006e61746976655f746f6b656e5f7472616e73666572"
            ));
            let agent_id = AgentId::from_low_u64_be(10);
            let para_id: ParaId = 2000u32.into();
            let amount_to_transfer = 10_000;
            let fee = 1000;

            assert_ok!(EthereumTokenTransfers::set_token_transfer_channel(
                root_origin(),
                channel_id,
                agent_id,
                para_id
            ));

            let payload = VersionedXcmMessage::V1(MessageV1 {
                chain_id: 1,
                command: Command::SendNativeToken {
                    token_id: Default::default(),
                    destination: Destination::AccountId32 {
                        id: AccountId::from(BOB).into(),
                    },
                    amount: amount_to_transfer,
                    fee,
                },
            });

            let event = OutboundMessageAccepted {
                channel_id: <[u8; 32]>::from(channel_id).into(),
                nonce: 1,
                message_id: Default::default(),
                payload: payload.encode(),
            };

            let message = Message {
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

            let sovereign_balance_before = Balances::free_balance(EthereumSovereignAccount::get());
            let treasury_balance_before = Balances::free_balance(TreasuryAccount::get());
            let relayer_balance_before = Balances::free_balance(AccountId::from(ALICE));
            let bob_balance_before = Balances::free_balance(AccountId::from(BOB));

            assert_ok!(EthereumInboundQueue::submit(relayer, message));

            // Amount reduced from sovereign account
            assert_eq!(
                Balances::free_balance(EthereumSovereignAccount::get()),
                sovereign_balance_before - amount_to_transfer
            );

            // Amount added in destination account
            assert_eq!(
                Balances::free_balance(AccountId::from(BOB)),
                bob_balance_before + amount_to_transfer
            );

            // Fees are payed
            assert_eq!(
                Balances::free_balance(TreasuryAccount::get()),
                treasury_balance_before - fee
            );

            assert_eq!(
                Balances::free_balance(AccountId::from(ALICE)),
                relayer_balance_before + fee
            );
        });
}

#[test]
fn can_process_message_returns_false_for_none_channel_info() {
    ExtBuilder::default().build().execute_with(|| {
        let channel_id = ChannelId::new([1; 32]);
        let agent_id = AgentId::from_low_u64_be(10);
        let para_id: ParaId = 2000u32.into();

        let channel = Channel { para_id, agent_id };

        let envelope = Envelope {
            channel_id,
            gateway: EthereumGatewayAddress::get(),
            payload: create_valid_payload(),
            nonce: 1,
            message_id: H256::zero(),
        };

        assert!(
            !<TokenTransferMessageProcessor<Runtime> as MessageProcessor>::can_process_message(
                &channel, &envelope
            )
        );
    });
}

#[test]
fn can_process_message_returns_false_for_wrong_channel_id() {
    ExtBuilder::default().build().execute_with(|| {
        let channel_id = ChannelId::new([1; 32]);
        let wrong_channel_id = ChannelId::new([2; 32]);
        let agent_id = AgentId::from_low_u64_be(10);
        let para_id: ParaId = 2000u32.into();

        assert_ok!(EthereumTokenTransfers::set_token_transfer_channel(
            root_origin(),
            channel_id,
            agent_id,
            para_id
        ));

        let channel = Channel { para_id, agent_id };

        let envelope = Envelope {
            channel_id: wrong_channel_id,
            gateway: EthereumGatewayAddress::get(),
            payload: create_valid_payload(),
            nonce: 1,
            message_id: H256::zero(),
        };

        assert!(
            !<TokenTransferMessageProcessor<Runtime> as MessageProcessor>::can_process_message(
                &channel, &envelope
            )
        );
    });
}

#[test]
fn can_process_message_returns_false_for_wrong_para_id() {
    ExtBuilder::default().build().execute_with(|| {
        let channel_id = ChannelId::new([1; 32]);
        let agent_id = AgentId::from_low_u64_be(10);
        let para_id: ParaId = 2000u32.into();
        let wrong_para_id: ParaId = 2002u32.into();

        assert_ok!(EthereumTokenTransfers::set_token_transfer_channel(
            root_origin(),
            channel_id,
            agent_id,
            para_id
        ));

        let channel = Channel {
            para_id: wrong_para_id,
            agent_id,
        };

        let envelope = Envelope {
            channel_id,
            gateway: EthereumGatewayAddress::get(),
            payload: create_valid_payload(),
            nonce: 1,
            message_id: H256::zero(),
        };

        assert!(
            !<TokenTransferMessageProcessor<Runtime> as MessageProcessor>::can_process_message(
                &channel, &envelope
            )
        );
    });
}

#[test]
fn can_process_message_returns_false_for_wrong_agent_id() {
    ExtBuilder::default().build().execute_with(|| {
        let channel_id = ChannelId::new([1; 32]);
        let agent_id = AgentId::from_low_u64_be(10);
        let para_id: ParaId = 2000u32.into();
        let wrong_agent_id = AgentId::from_low_u64_be(1010);

        assert_ok!(EthereumTokenTransfers::set_token_transfer_channel(
            root_origin(),
            channel_id,
            agent_id,
            para_id
        ));

        let channel = Channel {
            para_id,
            agent_id: wrong_agent_id,
        };

        let envelope = Envelope {
            channel_id,
            gateway: EthereumGatewayAddress::get(),
            payload: create_valid_payload(),
            nonce: 1,
            message_id: H256::zero(),
        };

        assert!(
            !<TokenTransferMessageProcessor<Runtime> as MessageProcessor>::can_process_message(
                &channel, &envelope
            )
        );
    });
}

#[test]
fn can_process_message_returns_false_for_wrong_gateway() {
    ExtBuilder::default().build().execute_with(|| {
        let channel_id = ChannelId::new([1; 32]);
        let agent_id = AgentId::from_low_u64_be(10);
        let para_id: ParaId = 2000u32.into();

        assert_ok!(EthereumTokenTransfers::set_token_transfer_channel(
            root_origin(),
            channel_id,
            agent_id,
            para_id
        ));

        let channel = Channel { para_id, agent_id };

        let envelope = Envelope {
            channel_id,
            gateway: H160(hex!("EDa338E4dC46038493b885327842fD3E301C0000")),
            payload: create_valid_payload(),
            nonce: 1,
            message_id: H256::zero(),
        };

        assert!(
            !<TokenTransferMessageProcessor<Runtime> as MessageProcessor>::can_process_message(
                &channel, &envelope
            )
        );
    });
}

#[test]
fn can_process_message_returns_false_for_wrong_message_type() {
    ExtBuilder::default().build().execute_with(|| {
        let channel_id = ChannelId::new([1; 32]);
        let agent_id = AgentId::from_low_u64_be(10);
        let para_id: ParaId = 2000u32.into();

        assert_ok!(EthereumTokenTransfers::set_token_transfer_channel(
            root_origin(),
            channel_id,
            agent_id,
            para_id
        ));

        let channel = Channel { para_id, agent_id };

        let envelope = Envelope {
            channel_id,
            gateway: EthereumGatewayAddress::get(),
            payload: VersionedXcmMessage::V1(MessageV1 {
                chain_id: 1,
                command: Command::RegisterToken {
                    token: Default::default(),
                    fee: 0,
                },
            })
            .encode(),
            nonce: 1,
            message_id: H256::zero(),
        };

        assert!(
            !<TokenTransferMessageProcessor<Runtime> as MessageProcessor>::can_process_message(
                &channel, &envelope
            )
        );
    });
}

fn create_valid_payload() -> Vec<u8> {
    let token_location = TokenLocationReanchored::get();
    let token_id = EthereumSystem::convert_back(&token_location).unwrap_or_default();

    create_payload_with_token_id(token_id)
}

fn create_payload_with_token_id(token_id: H256) -> Vec<u8> {
    let message = VersionedXcmMessage::V1(MessageV1 {
        chain_id: 1,
        command: Command::SendNativeToken {
            token_id,
            destination: Destination::AccountId32 {
                id: AccountId::from(ALICE).into(),
            },
            amount: 100,
            fee: 0,
        },
    });

    message.encode()
}
