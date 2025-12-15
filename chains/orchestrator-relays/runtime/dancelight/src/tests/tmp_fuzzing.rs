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

use crate::bridge_to_ethereum_config::EthereumGatewayAddress;
use crate::{EthereumInboundQueueV2, UseSnowbridgeV2};
use arbitrary::Arbitrary;
use arbtest::arbtest;
use frame_support::storage::with_storage_layer;
use sp_core::H256;
use {
    crate::{tests::common::*, Sudo},
    alloc::vec,
    frame_support::{assert_noop, assert_ok},
};

#[derive(Arbitrary, Debug)]
pub struct DataFuzzInboundV2 {
    origin: u8,
    data: Vec<u8>,
    custom_topics: [TopicsMode; 5],
    msg_kind: FuzzSnowbridgeMsgKind,
}
#[derive(Arbitrary, Debug)]
pub enum TopicsMode {
    Hardcoded,
    Zero,
    Empty,
    Raw([u8; 32]),
}

#[derive(Arbitrary, Debug)]
pub enum FuzzSnowbridgeMsgKind {
    // TODO: revisit fields, maybe not needed
    System { data: Vec<u8> },
    User { data: Vec<u8> },
    UserXcm { data: Vec<u8> },
}

// Build the topics vec, with some help from hardcoded values
// TODO: put the correct hardcoded values here
fn build_topics(x: &[TopicsMode]) -> Vec<H256> {
    // Event signature according to rust docs
    // OutboundMessageAccepted
    // TODO: this doesn't match the polkadot-sdk docs, they say the signature is
    // 0x550e2067494b1736ea5573f2d19cdc0ac95b410fff161bf16f11c6229655ec9c
    // Signature for event OutboundMessageAccepted(bytes32 indexed channel_id, uint64 nonce, bytes32 indexed message_id, bytes payload);
    let event_topic =
        hex_literal::hex!("7153f9357c8ea496bba60bf82e67143e27b64462b49041f8e689e1b05728f84f");
    let event_topic: H256 = event_topic.into();
    // There are no channels in snowbridge v2?
    let topic_channel_id = H256::default();
    let topic_message_id = H256::default();
    let topics = vec![
        event_topic,      // event signature
        topic_channel_id, // channel id
        topic_message_id, // message id
    ];

    let mut topics: Vec<H256> = topics
        .into_iter()
        .zip(x.into_iter())
        .filter_map(|(topic, mode)| match mode {
            TopicsMode::Hardcoded => Some(topic),
            TopicsMode::Zero => Some(Default::default()),
            TopicsMode::Empty => None,
            TopicsMode::Raw(x) => Some(x.into()),
        })
        .collect();

    // First 3 TopicsMode handled above, rest here
    for mode3 in x.iter().skip(3) {
        let topic3 = match mode3 {
            TopicsMode::Zero => Some(H256::default()),
            TopicsMode::Raw(x) => Some(x.into()),
            _ => None,
        };
        if let Some(topic) = topic3 {
            topics.push(topic);
        }
    }

    topics
}

#[test]
fn tmp_fuzzing_example_inbound_v2() {
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
            UseSnowbridgeV2::set(&true);
            run_to_block(2);

            arbtest(|u| {
                let data: DataFuzzInboundV2 = u.arbitrary()?;
                let DataFuzzInboundV2 {
                    origin,
                    data,
                    custom_topics,
                    msg_kind,
                } = data;
                let topics = build_topics(&custom_topics);
                let relayer = AccountId::from(ALICE);

                let log = snowbridge_verification_primitives::Log {
                    address: EthereumGatewayAddress::get(),
                    topics,
                    data,
                };
                match (&log).try_into() {
                    Ok(msg) => {
                        // Don't test anything yet, the message may be valid or invalid
                        let res = with_storage_layer(|| {
                            EthereumInboundQueueV2::process_message(relayer.clone(), msg)
                        });
                        if res.is_ok() {
                            panic!("found a valid message :D");
                        }
                    }
                    Err(_e) => {}
                }

                Ok(())
            })
            .run()
        });
}
