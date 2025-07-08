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
    crate::{tests::common::ExtBuilder, ExternalValidators, Runtime},
    frame_support::pallet_prelude::*,
    hex_literal::hex,
    keyring::Sr25519Keyring,
    snowbridge_core::{Channel, PRIMARY_GOVERNANCE_CHANNEL},
    snowbridge_inbound_queue_primitives::v1::{Envelope, MessageProcessor},
    sp_core::{H160, H256},
    sp_runtime::DispatchError,
    tp_bridge::symbiotic_message_processor::{
        InboundCommand, Message, Payload, SymbioticMessageProcessor, MAGIC_BYTES,
    },
};

#[test]
fn test_symbiotic_message_processor() {
    ExtBuilder::default().build().execute_with(|| {
        let default_channel = Channel {
            agent_id: H256::default(),
            para_id: 0.into(),
        };

        let envelope_with_invalid_payload = Envelope {
            channel_id: PRIMARY_GOVERNANCE_CHANNEL,
            gateway: H160::default(),
            message_id: Default::default(),
            nonce: 0,
            payload: vec![0, 1, 2],
        };

        assert_eq!(
            SymbioticMessageProcessor::<Runtime>::can_process_message(
                &default_channel,
                &envelope_with_invalid_payload
            ),
            false
        );
        assert_eq!(
            SymbioticMessageProcessor::<Runtime>::process_message(
                default_channel.clone(),
                envelope_with_invalid_payload
            ),
            Err(DispatchError::Other("unable to parse the envelope payload"))
        );

        let payload_with_incorrect_magic_bytes = Payload {
            magic_bytes: [1, 2, 3, 4],
            message: Message::V1(InboundCommand::<Runtime>::ReceiveValidators {
                validators: vec![],
                external_index: 0u64,
            }),
        };
        let envelope = Envelope {
            channel_id: PRIMARY_GOVERNANCE_CHANNEL,
            gateway: H160::default(),
            message_id: Default::default(),
            nonce: 0,
            payload: payload_with_incorrect_magic_bytes.encode(),
        };
        assert_eq!(
            SymbioticMessageProcessor::<Runtime>::can_process_message(&default_channel, &envelope),
            false
        );

        // No external validators are set right now
        assert_eq!(
            ExternalValidators::validators(),
            ExternalValidators::whitelisted_validators()
        );

        let payload_validators = vec![
            Sr25519Keyring::Alice.to_account_id(),
            Sr25519Keyring::Charlie.to_account_id(),
            Sr25519Keyring::Bob.to_account_id(),
        ];

        let payload_with_correct_magic_bytes = Payload {
            magic_bytes: MAGIC_BYTES,
            message: Message::V1(InboundCommand::<Runtime>::ReceiveValidators {
                validators: payload_validators.clone(),
                external_index: 10u64,
            }),
        };
        let envelope = Envelope {
            channel_id: PRIMARY_GOVERNANCE_CHANNEL,
            gateway: H160::default(),
            message_id: Default::default(),
            nonce: 0,
            payload: payload_with_correct_magic_bytes.encode(),
        };
        assert_eq!(
            SymbioticMessageProcessor::<Runtime>::can_process_message(&default_channel, &envelope),
            true
        );
        assert_eq!(
            SymbioticMessageProcessor::<Runtime>::process_message(
                default_channel.clone(),
                envelope
            ),
            Ok(())
        );

        let expected_validators = [
            ExternalValidators::whitelisted_validators(),
            payload_validators,
        ]
        .concat();
        assert_eq!(ExternalValidators::validators(), expected_validators);
    });
}

#[test]
fn test_symbiotic_message_processor_rejects_invalid_channel_id() {
    ExtBuilder::default().build().execute_with(|| {
        let default_channel = Channel {
            agent_id: H256::default(),
            para_id: 0.into(),
        };

        // No external validators are set right now
        assert_eq!(
            ExternalValidators::validators(),
            ExternalValidators::whitelisted_validators()
        );

        let payload_validators = vec![
            Sr25519Keyring::Alice.to_account_id(),
            Sr25519Keyring::Charlie.to_account_id(),
            Sr25519Keyring::Bob.to_account_id(),
        ];

        let payload_with_correct_magic_bytes = Payload {
            magic_bytes: MAGIC_BYTES,
            message: Message::V1(InboundCommand::<Runtime>::ReceiveValidators {
                validators: payload_validators.clone(),
                external_index: 0u64,
            }),
        };
        let envelope = Envelope {
            channel_id: H256::default().into(),
            gateway: H160::default(),
            message_id: Default::default(),
            nonce: 0,
            payload: payload_with_correct_magic_bytes.encode(),
        };
        assert_eq!(
            SymbioticMessageProcessor::<Runtime>::can_process_message(&default_channel, &envelope),
            true
        );
        assert_eq!(
            SymbioticMessageProcessor::<Runtime>::process_message(
                default_channel.clone(),
                envelope
            ),
            Err(DispatchError::Other(
                "Received governance message from invalid channel id"
            ))
        );

        let expected_validators = [ExternalValidators::whitelisted_validators()].concat();
        assert_eq!(ExternalValidators::validators(), expected_validators);
    });
}

#[test]
fn test_symbiotic_message_processor_as_payload() {
    ExtBuilder::default().build().execute_with(|| {
        let default_channel = Channel {
            agent_id: H256::default(),
            para_id: 0.into(),
        };
        // No external validators are set right now
        assert_eq!(
            ExternalValidators::validators(),
            ExternalValidators::whitelisted_validators()
        );

        // 50 validators injected
        let payload_encoded =hex!("701500380000c8000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000003000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000050000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000700000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000009000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000b000000000000000000000000000000000000000000000000000000000000000c000000000000000000000000000000000000000000000000000000000000000d000000000000000000000000000000000000000000000000000000000000000e000000000000000000000000000000000000000000000000000000000000000f0000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001100000000000000000000000000000000000000000000000000000000000000120000000000000000000000000000000000000000000000000000000000000013000000000000000000000000000000000000000000000000000000000000001400000000000000000000000000000000000000000000000000000000000000150000000000000000000000000000000000000000000000000000000000000016000000000000000000000000000000000000000000000000000000000000001700000000000000000000000000000000000000000000000000000000000000180000000000000000000000000000000000000000000000000000000000000019000000000000000000000000000000000000000000000000000000000000001a000000000000000000000000000000000000000000000000000000000000001b000000000000000000000000000000000000000000000000000000000000001c000000000000000000000000000000000000000000000000000000000000001d000000000000000000000000000000000000000000000000000000000000001e000000000000000000000000000000000000000000000000000000000000001f0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000002100000000000000000000000000000000000000000000000000000000000000220000000000000000000000000000000000000000000000000000000000000023000000000000000000000000000000000000000000000000000000000000002400000000000000000000000000000000000000000000000000000000000000250000000000000000000000000000000000000000000000000000000000000026000000000000000000000000000000000000000000000000000000000000002700000000000000000000000000000000000000000000000000000000000000280000000000000000000000000000000000000000000000000000000000000029000000000000000000000000000000000000000000000000000000000000002a000000000000000000000000000000000000000000000000000000000000002b000000000000000000000000000000000000000000000000000000000000002c000000000000000000000000000000000000000000000000000000000000002d000000000000000000000000000000000000000000000000000000000000002e000000000000000000000000000000000000000000000000000000000000002f000000000000000000000000000000000000000000000000000000000000003000000000000000000000000000000000000000000000000000000000000000310000000000000000000000000000000000000000000000000000000000000032f055885f94010000");
        let envelope = Envelope {
            channel_id: PRIMARY_GOVERNANCE_CHANNEL,
            gateway: H160::default(),
            message_id: Default::default(),
            nonce: 0,
            payload: payload_encoded.to_vec(),
        };
        assert_eq!(
            SymbioticMessageProcessor::<Runtime>::can_process_message(&default_channel, &envelope),
            true
        );
        assert_eq!(
            SymbioticMessageProcessor::<Runtime>::process_message(
                default_channel.clone(),
                envelope
            ),
            Ok(())
        );

        // 50 validators injected
        assert_eq!(ExternalValidators::validators().len(), 50+ExternalValidators::whitelisted_validators().len());
    });
}
