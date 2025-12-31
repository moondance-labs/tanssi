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
    super::ALICE,
    crate as dancelight_runtime,
    crate::tests::common::ExtBuilder,
    alloy_core::sol_types::SolType,
    dancelight_runtime::{xcm_config, AccountId, Runtime},
    frame_support::parameter_types,
    parity_scale_codec::Encode,
    snowbridge_core::{AgentId, ChannelId},
    snowbridge_inbound_queue_primitives::v2::{message::Message, Payload},
    sp_core::{H160, H256},
    tanssi_runtime_common::processors::v2::{
        LayerZeroMessageProcessor, MessageExtractionError, MessageProcessorWithFallback, RawPayload,
    },
    tp_bridge::layerzero_message::{
        InboundSolMessage as LayerZeroInboundSolMessage,
        InboundSolPayload as LayerZeroInboundSolPayload, MAGIC_BYTES as LZ_MAGIC_BYTES,
    },
    xcm::latest::prelude::*,
};

parameter_types! {
    const EthereumNetwork: NetworkId = Ethereum { chain_id: 11155111 };
    const BridgeChannelInfo: Option<(ChannelId, AgentId)> = Some((ChannelId::new([1u8; 32]), H256([2u8; 32])));
    pub EthereumUniversalLocation: InteriorLocation = GlobalConsensus(EthereumNetwork::get()).into();
    pub TanssiUniversalLocation: InteriorLocation = GlobalConsensus(ByGenesis(dancelight_runtime_constants::DANCELIGHT_GENESIS_HASH)).into();
    pub GatewayAddress: H160 = H160(hex_literal::hex!("EDa338E4dC46038493b885327842fD3E301CaB39"));
    pub DefaultClaimer: AccountId = AccountId::from(ALICE);
}

type Processor = LayerZeroMessageProcessor<
    Runtime,
    GatewayAddress,
    DefaultClaimer,
    EthereumNetwork,
    EthereumUniversalLocation,
    TanssiUniversalLocation,
    xcm_executor::XcmExecutor<xcm_config::XcmConfig>,
    <xcm_config::XcmConfig as xcm_executor::Config>::Weigher,
>;

/// Helper function to create a valid LayerZero ABI-encoded payload
fn create_layerzero_payload(
    source_address: [u8; 32],
    source_endpoint: u32,
    destination_chain: u32,
    message: Vec<u8>,
) -> Vec<u8> {
    let sol_message = LayerZeroInboundSolMessage {
        lzSourceAddress: source_address.into(),
        lzSourceEndpoint: source_endpoint,
        destinationChain: destination_chain,
        message: message.into(),
    };

    let sol_payload = LayerZeroInboundSolPayload {
        magicBytes: (*LZ_MAGIC_BYTES).into(),
        message: sol_message,
    };

    LayerZeroInboundSolPayload::abi_encode(&sol_payload)
}

/// Helper function to create a valid LayerZero ABI-encoded payload with custom magic bytes
fn create_layerzero_payload_with_magic(
    magic_bytes: [u8; 4],
    source_address: [u8; 32],
    source_endpoint: u32,
    destination_chain: u32,
    message: Vec<u8>,
) -> Vec<u8> {
    let sol_message = LayerZeroInboundSolMessage {
        lzSourceAddress: source_address.into(),
        lzSourceEndpoint: source_endpoint,
        destinationChain: destination_chain,
        message: message.into(),
    };

    let sol_payload = LayerZeroInboundSolPayload {
        magicBytes: magic_bytes.into(),
        message: sol_message,
    };

    LayerZeroInboundSolPayload::abi_encode(&sol_payload)
}

#[test]
fn layerzero_try_extract_message_succeeds_with_valid_payload() {
    ExtBuilder::default().build().execute_with(|| {
        let gateway_address = GatewayAddress::get();
        let sender: AccountId = AccountId::from(ALICE);
        let source_address = [1u8; 32];
        let source_endpoint = 30101u32; // Example LayerZero endpoint ID
        let destination_chain = 2000u32; // Example container chain para ID
        let inner_message = vec![0xDE, 0xAD, 0xBE, 0xEF];

        let lz_payload = create_layerzero_payload(
            source_address,
            source_endpoint,
            destination_chain,
            inner_message.clone(),
        );

        let raw_payload = RawPayload::LayerZero(lz_payload);

        let message = Message {
            gateway: gateway_address,
            nonce: 1,
            origin: gateway_address,
            assets: vec![],
            payload: Payload::Raw(raw_payload.encode()),
            claimer: None,
            value: 0,
            execution_fee: 0,
            relayer_fee: 0,
        };

        let result = <Processor as MessageProcessorWithFallback<AccountId>>::try_extract_message(
            &sender, &message,
        );

        assert!(result.is_ok(), "Expected Ok, got: {:?}", result);

        let extracted = result.unwrap();
        assert_eq!(
            extracted.lz_source_address.to_vec(),
            source_address.to_vec()
        );
        assert_eq!(extracted.lz_source_endpoint, source_endpoint);
        assert_eq!(extracted.destination_chain, destination_chain);
        assert_eq!(extracted.message, inner_message);
    });
}

#[test]
fn layerzero_try_extract_message_fails_with_invalid_origin() {
    ExtBuilder::default().build().execute_with(|| {
        let gateway_address = GatewayAddress::get();
        let sender: AccountId = AccountId::from(ALICE);
        let wrong_origin = H160::random();

        let lz_payload = create_layerzero_payload([1u8; 32], 30101, 2000, vec![0xAA]);

        let raw_payload = RawPayload::LayerZero(lz_payload);

        let message = Message {
            gateway: gateway_address,
            nonce: 1,
            origin: wrong_origin, // Wrong origin
            assets: vec![],
            payload: Payload::Raw(raw_payload.encode()),
            claimer: None,
            value: 0,
            execution_fee: 0,
            relayer_fee: 0,
        };

        let result =
            <Processor as MessageProcessorWithFallback<AccountId>>::try_extract_message(
                &sender, &message,
            );

        assert!(
            matches!(result, Err(MessageExtractionError::InvalidMessage { ref context, .. }) if context.contains("origin")),
            "Expected InvalidMessage error about origin, got: {:?}",
            result
        );
    });
}

#[test]
fn layerzero_try_extract_message_fails_with_assets() {
    ExtBuilder::default().build().execute_with(|| {
        let gateway_address = GatewayAddress::get();
        let sender: AccountId = AccountId::from(ALICE);

        let lz_payload = create_layerzero_payload([1u8; 32], 30101, 2000, vec![0xAA]);

        let raw_payload = RawPayload::LayerZero(lz_payload);

        let message = Message {
            gateway: gateway_address,
            nonce: 1,
            origin: gateway_address,
            assets: vec![snowbridge_inbound_queue_primitives::v2::EthereumAsset::NativeTokenERC20 {
                token_id: H160::random(),
                value: 1000,
            }],
            payload: Payload::Raw(raw_payload.encode()),
            claimer: None,
            value: 0,
            execution_fee: 0,
            relayer_fee: 0,
        };

        let result =
            <Processor as MessageProcessorWithFallback<AccountId>>::try_extract_message(
                &sender, &message,
            );

        assert!(
            matches!(result, Err(MessageExtractionError::InvalidMessage { ref context, .. }) if context.contains("assets")),
            "Expected InvalidMessage error about assets, got: {:?}",
            result
        );
    });
}

#[test]
fn layerzero_try_extract_message_fails_with_value() {
    ExtBuilder::default().build().execute_with(|| {
        let gateway_address = GatewayAddress::get();
        let sender: AccountId = AccountId::from(ALICE);

        let lz_payload = create_layerzero_payload([1u8; 32], 30101, 2000, vec![0xAA]);

        let raw_payload = RawPayload::LayerZero(lz_payload);

        let message = Message {
            gateway: gateway_address,
            nonce: 1,
            origin: gateway_address,
            assets: vec![],
            payload: Payload::Raw(raw_payload.encode()),
            claimer: None,
            value: 1000, // Non-zero value
            execution_fee: 0,
            relayer_fee: 0,
        };

        let result =
            <Processor as MessageProcessorWithFallback<AccountId>>::try_extract_message(
                &sender, &message,
            );

        assert!(
            matches!(result, Err(MessageExtractionError::InvalidMessage { ref context, .. }) if context.contains("assets")),
            "Expected InvalidMessage error about assets/value, got: {:?}",
            result
        );
    });
}

#[test]
fn layerzero_try_extract_message_fails_with_wrong_magic_bytes() {
    ExtBuilder::default().build().execute_with(|| {
        let gateway_address = GatewayAddress::get();
        let sender: AccountId = AccountId::from(ALICE);

        let lz_payload = create_layerzero_payload_with_magic(
            [0xBA, 0xAD, 0xF0, 0x0D], // Wrong magic bytes
            [1u8; 32],
            30101,
            2000,
            vec![0xAA],
        );

        let raw_payload = RawPayload::LayerZero(lz_payload);

        let message = Message {
            gateway: gateway_address,
            nonce: 1,
            origin: gateway_address,
            assets: vec![],
            payload: Payload::Raw(raw_payload.encode()),
            claimer: None,
            value: 0,
            execution_fee: 0,
            relayer_fee: 0,
        };

        let result =
            <Processor as MessageProcessorWithFallback<AccountId>>::try_extract_message(
                &sender, &message,
            );

        assert!(
            matches!(result, Err(MessageExtractionError::InvalidMessage { ref context, .. }) if context.contains("magic bytes")),
            "Expected InvalidMessage error about magic bytes, got: {:?}",
            result
        );
    });
}

#[test]
fn layerzero_try_extract_message_fails_with_invalid_abi_payload() {
    ExtBuilder::default().build().execute_with(|| {
        let gateway_address = GatewayAddress::get();
        let sender: AccountId = AccountId::from(ALICE);

        // Invalid ABI-encoded payload (just random bytes)
        let invalid_lz_payload = vec![0xAA, 0xBB, 0xCC, 0xDD];

        let raw_payload = RawPayload::LayerZero(invalid_lz_payload);

        let message = Message {
            gateway: gateway_address,
            nonce: 1,
            origin: gateway_address,
            assets: vec![],
            payload: Payload::Raw(raw_payload.encode()),
            claimer: None,
            value: 0,
            execution_fee: 0,
            relayer_fee: 0,
        };

        let result = <Processor as MessageProcessorWithFallback<AccountId>>::try_extract_message(
            &sender, &message,
        );

        assert!(
            matches!(result, Err(MessageExtractionError::InvalidMessage { .. })),
            "Expected InvalidMessage error for invalid ABI payload, got: {:?}",
            result
        );
    });
}

#[test]
fn layerzero_try_extract_message_fails_with_invalid_raw_payload_encoding() {
    ExtBuilder::default().build().execute_with(|| {
        let gateway_address = GatewayAddress::get();
        let sender: AccountId = AccountId::from(ALICE);

        // Use invalid bytes that can't be decoded as RawPayload
        let message = Message {
            gateway: gateway_address,
            nonce: 1,
            origin: gateway_address,
            assets: vec![],
            payload: Payload::Raw(vec![0xFF, 0xFF, 0xFF, 0xFF]),
            claimer: None,
            value: 0,
            execution_fee: 0,
            relayer_fee: 0,
        };

        let result = <Processor as MessageProcessorWithFallback<AccountId>>::try_extract_message(
            &sender, &message,
        );

        assert!(
            matches!(result, Err(MessageExtractionError::InvalidMessage { .. })),
            "Expected InvalidMessage error for invalid RawPayload encoding, got: {:?}",
            result
        );
    });
}

#[test]
fn layerzero_try_extract_message_fails_with_wrong_raw_payload_variant() {
    ExtBuilder::default().build().execute_with(|| {
        let gateway_address = GatewayAddress::get();
        let sender: AccountId = AccountId::from(ALICE);

        // Use Symbiotic variant instead of LayerZero
        let raw_payload = RawPayload::Symbiotic(vec![0xAA, 0xBB, 0xCC]);

        let message = Message {
            gateway: gateway_address,
            nonce: 1,
            origin: gateway_address,
            assets: vec![],
            payload: Payload::Raw(raw_payload.encode()),
            claimer: None,
            value: 0,
            execution_fee: 0,
            relayer_fee: 0,
        };

        let result = <Processor as MessageProcessorWithFallback<AccountId>>::try_extract_message(
            &sender, &message,
        );

        assert!(
            matches!(
                result,
                Err(MessageExtractionError::UnsupportedMessage { .. })
            ),
            "Expected UnsupportedMessage error for wrong RawPayload variant, got: {:?}",
            result
        );
    });
}
