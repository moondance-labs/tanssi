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
        tests::common::*, Balances, EthereumSovereignAccount, EthereumSystem,
        EthereumTokenTransfers, RuntimeEvent, TokenLocationReanchored, TreasuryAccount,
    },
    frame_support::assert_ok,
    snowbridge_core::{AgentId, ChannelId, ParaId},
    sp_core::{H160, H256},
    sp_runtime::traits::MaybeEquivalence,
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

            assert_eq!(EthereumTokenTransfers::current_channel_info(), None);

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

            assert_eq!(EthereumSystem::agents(agent_id).unwrap(), ());

            // PartialEq is not implemented for Channel, so we compare each element individually.
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
