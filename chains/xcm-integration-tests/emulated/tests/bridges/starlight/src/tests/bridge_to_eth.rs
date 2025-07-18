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

use crate::tests::imports::*;

#[test]
fn send_msg_to_eth_should_be_process_by_the_bridge() {
    let root_origin = <Starlight as Chain>::RuntimeOrigin::root();
    let nonce: H256 = sp_core::blake2_256(b"nonce").into();
    let msg_size = 32; // For simplicity since we are hashing the nonce in 32bytes

    // Send message to eth
    Starlight::execute_with(|| {
        assert_ok!(
            <Starlight as StarlightRelayPallet>::ExternalValidatorSlashes::root_test_send_msg_to_eth(
                root_origin,
                nonce,
                1,
                msg_size
            )
        );
    });

    // Force process bridge messages
    tanssi_emulated_integration_tests_common::force_process_bridge::<Starlight, StarlightPara>(
        starlight_runtime::MessageQueueServiceWeight::get(),
    );

    // xcm command generated by root_test_send_msg_to_eth
    let payload = sp_core::blake2_256((nonce, 0).encode().as_ref()).to_vec();
    let command = Command::Test(payload);

    // msg sent in bridge
    let msgs = tanssi_emulated_integration_tests_common::impls::eth_bridge_sent_msgs();
    let sent_message = msgs.first().unwrap();

    assert_eq!(
        sent_message.channel_id,
        snowbridge_core::PRIMARY_GOVERNANCE_CHANNEL
    );
    assert_eq!(sent_message.command, command.index());
    assert_eq!(sent_message.params, command.abi_encode());
}

#[test]
fn receive_msg_from_eth_validators_are_updated() {
    Starlight::execute_with(|| {
        let origin =
            <Runtime as frame_system::Config>::RuntimeOrigin::signed(StarlightSender::get());

        // New validators to be added
        let payload_validators = vec![
            Sr25519Keyring::Alice.to_account_id(),
            Sr25519Keyring::Charlie.to_account_id(),
            Sr25519Keyring::Bob.to_account_id(),
        ];

        let payload = Payload {
            magic_bytes: MAGIC_BYTES,
            message: SymbioticMessage::V1(InboundCommand::<Runtime>::ReceiveValidators {
                validators: payload_validators.clone(),
                external_index: 0u64,
            }),
        };

        let event = OutboundMessageAccepted {
            channel_id: <[u8; 32]>::from(PRIMARY_GOVERNANCE_CHANNEL).into(),
            nonce: 1,
            message_id: Default::default(),
            payload: payload.encode(),
        };

        let message = EventProof {
            event_log: Log {
                // gateway address
                address: <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(
                ),
                topics: event
                    .encode_topics()
                    .into_iter()
                    .map(|word| H256::from(word.0 .0))
                    .collect(),
                data: event.encode_data(),
            },
            proof: mock_snowbridge_message_proof(),
        };

        // Submit message to the queue
        assert_ok!(
            <Starlight as StarlightRelayPallet>::EthereumInboundQueue::submit(origin, message)
        );

        let whitelisted =
            <Starlight as StarlightRelayPallet>::ExternalValidators::whitelisted_validators();

        // Ignore whitelisted
        let new_validators = <Starlight as StarlightRelayPallet>::ExternalValidators::validators()
            .into_iter()
            .filter(|v| !whitelisted.contains(v))
            .collect::<Vec<_>>();

        let expected_validators = payload_validators.clone();

        assert_eq!(new_validators, expected_validators);
    });
}
