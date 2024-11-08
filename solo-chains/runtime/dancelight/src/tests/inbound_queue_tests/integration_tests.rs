use crate::symbiotic_message_processor::{Command, Payload, MAGIC_BYTES};
use crate::tests::inbound_queue_tests::mock::{
    mock_ext, AccountId, ExternalValidators as MockExternalValidators, InboundQueue,
    Test as TestRuntime, MOCK_CHANNEL_ID,
};
use alloy_sol_types::SolEvent;
use frame_system::pallet_prelude::OriginFor;
use keyring::AccountKeyring;
use parity_scale_codec::Encode;
use snowbridge_beacon_primitives::types::deneb;
use snowbridge_beacon_primitives::{ExecutionProof, VersionedExecutionPayloadHeader};
use snowbridge_core::inbound::{Log, Message, Proof};
use snowbridge_router_primitives::inbound::envelope::OutboundMessageAccepted;
use sp_core::H256;
use sp_runtime::DispatchError;

#[test]
fn test_inbound_queue_message_passing() {
    mock_ext().execute_with(|| {
        let current_nonce = 1;

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

        assert_eq!(InboundQueue::submit(OriginFor::<TestRuntime>::signed(AccountId::new([0; 32])), Message {
            event_log: Log {
                address: <TestRuntime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                topics: event_with_empty_payload.encode_topics().into_iter().map(|word| H256::from(word.0.0)).collect(),
                data: event_with_empty_payload.encode_data(),
            },
            proof: dummy_proof.clone(),
        }), Err(DispatchError::Other("No handler for message found")));

        assert_eq!(MockExternalValidators::validators(), MockExternalValidators::whitelisted_validators());

        let payload_validators = vec![
            AccountKeyring::Charlie.to_account_id(),
            AccountKeyring::Ferdie.to_account_id(),
            AccountKeyring::BobStash.to_account_id()
        ];

        let payload = Payload {
            magic_bytes: MAGIC_BYTES,
            message: crate::symbiotic_message_processor::Message::V1(Command::<TestRuntime>::ReceiveValidators {
                validators: payload_validators.clone()
            }),
        };

        let event_with_valid_payload = OutboundMessageAccepted {
            channel_id: MOCK_CHANNEL_ID.into(),
            nonce: current_nonce,
            message_id: Default::default(),
            payload: payload.encode(),
        };

        assert_eq!(InboundQueue::submit(OriginFor::<TestRuntime>::signed(AccountId::new([0; 32])), Message {
            event_log: Log {
                address: <TestRuntime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                topics: event_with_valid_payload.encode_topics().into_iter().map(|word| H256::from(word.0.0)).collect(),
                data: event_with_valid_payload.encode_data(),
            },
            proof: dummy_proof.clone(),
        }), Ok(()));


        let expected_validators = [MockExternalValidators::whitelisted_validators(), payload_validators].concat();
        assert_eq!(MockExternalValidators::validators(), expected_validators);
    });
}
