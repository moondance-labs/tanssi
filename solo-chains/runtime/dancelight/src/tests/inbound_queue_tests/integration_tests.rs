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

use crate::tests::common::ExtBuilder;
use crate::{AccountId, EthereumInboundQueue, ExternalValidators, Runtime};
use alloy_sol_types::SolEvent;
use frame_system::pallet_prelude::OriginFor;
use keyring::AccountKeyring;
use parity_scale_codec::Encode;
use snowbridge_beacon_primitives::types::deneb;
use snowbridge_beacon_primitives::{ExecutionProof, VersionedExecutionPayloadHeader};
use snowbridge_core::{
    inbound::{Log, Message, Proof},
    Channel, ChannelId,
};
use snowbridge_router_primitives::inbound::envelope::OutboundMessageAccepted;
use sp_core::H256;
use sp_runtime::DispatchError;
use tp_bridge::symbiotic_message_processor::{
    InboundCommand, Message as SymbioticMessage, Payload, MAGIC_BYTES,
};

const MOCK_CHANNEL_ID: [u8; 32] = [0; 32];

#[test]
fn test_inbound_queue_message_passing() {
    ExtBuilder::default().build().execute_with(|| {
        let current_nonce = 1;

        snowbridge_pallet_system::Channels::<Runtime>::set(ChannelId::from(MOCK_CHANNEL_ID), Some(Channel {
            agent_id: Default::default(),
            para_id: Default::default()
        }));

        let dummy_proof = Proof { receipt_proof: (vec![], vec![]), execution_proof: ExecutionProof {
            header: Default::default(),
            ancestry_proof: None,
            execution_header: VersionedExecutionPayloadHeader::Deneb(deneb::ExecutionPayloadHeader {
                parent_hash: Default::default(),
                fee_recipient: Default::default(),
                state_root: Default::default(),
                receipts_root: Default::default(),
                logs_bloom: vec![],
                prev_randao: Default::default(),
                block_number: 0,
                gas_limit: 0,
                gas_used: 0,
                timestamp: 0,
                extra_data: vec![],
                base_fee_per_gas: Default::default(),
                block_hash: Default::default(),
                transactions_root: Default::default(),
                withdrawals_root: Default::default(),
                blob_gas_used: 0,
                excess_blob_gas: 0,
            }),
            execution_branch: vec![],
        } };

        let event_with_empty_payload = OutboundMessageAccepted {
            channel_id: MOCK_CHANNEL_ID.into(),
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

        assert_eq!(ExternalValidators::validators(), ExternalValidators::whitelisted_validators());

        let payload_validators = vec![
            AccountKeyring::Charlie.to_account_id(),
            AccountKeyring::Ferdie.to_account_id(),
            AccountKeyring::BobStash.to_account_id()
        ];

        let payload = Payload {
            magic_bytes: MAGIC_BYTES,
            message: SymbioticMessage::V1(InboundCommand::<Runtime>::ReceiveValidators {
                validators: payload_validators.clone()
            }),
        };

        let event_with_valid_payload = OutboundMessageAccepted {
            channel_id: MOCK_CHANNEL_ID.into(),
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
