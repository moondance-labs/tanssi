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
    crate::{
        tests::common::{mock_snowbridge_message_proof, ExtBuilder, ALICE, BOB, UNIT},
        AccountId, EthereumInboundQueue, ExternalValidators, Runtime,
    },
    alloy_sol_types::SolEvent,
    frame_system::pallet_prelude::OriginFor,
    keyring::Sr25519Keyring,
    parity_scale_codec::Encode,
    snowbridge_core::{
        inbound::{Log, Message},
        Channel, PRIMARY_GOVERNANCE_CHANNEL,
    },
    snowbridge_inbound_queue_primitives::inbound::envelope::OutboundMessageAccepted,
    sp_core::H256,
    sp_runtime::DispatchError,
    tp_bridge::symbiotic_message_processor::{
        InboundCommand, Message as SymbioticMessage, Payload, MAGIC_BYTES,
    },
};

#[test]
fn test_inbound_queue_message_passing() {
    ExtBuilder::default()
    .with_validators(
        vec![]
    )
    .with_external_validators(
        vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ]
    ).build().execute_with(|| {
        let current_nonce = 1;

        snowbridge_pallet_system::Channels::<Runtime>::set(PRIMARY_GOVERNANCE_CHANNEL, Some(Channel {
            agent_id: Default::default(),
            para_id: Default::default()
        }));

        let dummy_proof = mock_snowbridge_message_proof();

        let event_with_empty_payload = OutboundMessageAccepted {
            channel_id: <[u8; 32]>::from(PRIMARY_GOVERNANCE_CHANNEL).into(),
            nonce: current_nonce,
            message_id: Default::default(),
            payload: vec![],
        };

        assert_eq!(EthereumInboundQueue::submit(OriginFor::<Runtime>::signed(AccountId::new([0; 32])), Message {
            event_log: Log {
                address: <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                topics: event_with_empty_payload.encode_topics().into_iter().map(|word| H256::from(word.0.0)).collect(),
                data: event_with_empty_payload.encode_data(),
            },
            proof: dummy_proof.clone(),
        }), Err(DispatchError::Other("No handler for message found")));

        let payload_validators = vec![
            Sr25519Keyring::Charlie.to_account_id(),
            Sr25519Keyring::Ferdie.to_account_id(),
            Sr25519Keyring::BobStash.to_account_id()
        ];

        let payload = Payload {
            magic_bytes: MAGIC_BYTES,
            message: SymbioticMessage::V1(InboundCommand::<Runtime>::ReceiveValidators {
                validators: payload_validators.clone(),
                external_index: 0u64
            }),
        };

        let event_with_valid_payload = OutboundMessageAccepted {
            channel_id: <[u8; 32]>::from(PRIMARY_GOVERNANCE_CHANNEL).into(),
            nonce: current_nonce,
            message_id: Default::default(),
            payload: payload.encode(),
        };

        assert_eq!(EthereumInboundQueue::submit(OriginFor::<Runtime>::signed(AccountId::new([0; 32])), Message {
            event_log: Log {
                address: <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                topics: event_with_valid_payload.encode_topics().into_iter().map(|word| H256::from(word.0.0)).collect(),
                data: event_with_valid_payload.encode_data(),
            },
            proof: dummy_proof.clone(),
        }), Ok(()));

        let expected_validators = [ExternalValidators::whitelisted_validators(), payload_validators].concat();
        assert_eq!(ExternalValidators::validators(), expected_validators);
    });
}
