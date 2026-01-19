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

use crate::bridge_to_ethereum_config::MaxXcmWeight;
use {
    super::ALICE,
    crate as dancelight_runtime,
    crate::tests::common::ExtBuilder,
    dancelight_runtime::{xcm_config, AccountId, Runtime},
    frame_support::{parameter_types, BoundedVec},
    parity_scale_codec::Encode,
    snowbridge_core::{AgentId, ChannelId},
    snowbridge_inbound_queue_primitives::v2::{message::Message, EthereumAsset, Payload},
    sp_core::{H160, H256},
    tanssi_runtime_common::processors::v2::{
        MessageExtractionError, MessageProcessorWithFallback, RawMessageProcessor, RawPayload,
    },
    xcm::{
        latest::{prelude::*, Location},
        opaque::latest::AssetTransferFilter::ReserveDeposit,
        VersionedXcm,
    },
};

parameter_types! {
    const EthereumNetwork: NetworkId = Ethereum { chain_id: 11155111 };
    const BridgeChannelInfo: Option<(ChannelId, AgentId)> = Some((ChannelId::new([1u8; 32]), H256([2u8; 32])));
    pub EthereumUniversalLocation: InteriorLocation = GlobalConsensus(EthereumNetwork::get()).into();
    pub TanssiUniversalLocation: InteriorLocation = GlobalConsensus(ByGenesis(dancelight_runtime_constants::DANCELIGHT_GENESIS_HASH)).into();
    pub GatewayAddress: H160 = H160(hex_literal::hex!("EDa338E4dC46038493b885327842fD3E301CaB39"));
    pub DefaultClaimer: AccountId = AccountId::from(ALICE);
}

#[test]
fn raw_message_processor_works() {
    let origin = GatewayAddress::get();

    let token_transfer_value = 2_000_000_000_000u128;

    let token: H160 = H160::random();
    let assets = vec![EthereumAsset::NativeTokenERC20 {
        token_id: token,
        value: token_transfer_value,
    }];
    let relayer_reward = 1_500_000_000_000u128;

    let claimer_acc_id = H256::random();
    let claimer = AccountId32 {
        network: None,
        id: claimer_acc_id.into(),
    };
    let claimer_bytes = claimer.encode();
    let eth_location = Location::new(1, [GlobalConsensus(Ethereum { chain_id: 11155111 })]);
    let beneficiary_acc_id: H256 = H256::random();
    let eth_fee: xcm::prelude::Asset = (eth_location.clone(), 2_000_000_000_000u128).into();
    let ether_asset: xcm::prelude::Asset = (eth_location.clone(), 4_000_000_000_000u128).into();
    let beneficiary = Location::new(
        0,
        AccountId32 {
            network: None,
            id: beneficiary_acc_id.into(),
        },
    );

    let instructions = vec![
        InitiateTransfer {
            destination: Location::new(0, [Parachain(2000u32)]),
            remote_fees: Some(ReserveDeposit(Definite(vec![eth_fee.clone()].into()))),
            preserve_origin: false,
            assets: BoundedVec::truncate_from(vec![ReserveDeposit(Definite(
                vec![ether_asset.clone()].into(),
            ))]),
            remote_xcm: vec![
                RefundSurplus,
                DepositAsset {
                    assets: Wild(AllCounted(3)),
                    beneficiary: beneficiary.clone(),
                },
                SetTopic(H256::random().into()),
            ]
            .into(),
        },
        RefundSurplus,
        DepositAsset {
            assets: Wild(AllOf {
                id: AssetId(eth_location.clone()),
                fun: WildFungibility::Fungible,
            }),
            beneficiary,
        },
    ];

    let xcm: Xcm<()> = instructions.into();
    let versioned_message_xcm = VersionedXcm::V5(xcm);
    let raw_payload = RawPayload::Xcm(versioned_message_xcm.encode());
    let message = Message {
        gateway: origin,
        nonce: 1,
        origin,
        assets,
        payload: Payload::Raw(raw_payload.encode()),
        claimer: Some(claimer_bytes),
        value: 3_500_000_000_000u128,
        execution_fee: 1_500_000_000_000u128,
        relayer_fee: relayer_reward,
    };

    let result = RawMessageProcessor::<
        Runtime,
        GatewayAddress,
        DefaultClaimer,
        EthereumNetwork,
        EthereumUniversalLocation,
        TanssiUniversalLocation,
        xcm_executor::XcmExecutor<xcm_config::XcmConfig>,
        <xcm_config::XcmConfig as xcm_executor::Config>::Weigher,
        MaxXcmWeight,
    >::try_extract_message(&AccountId::from(ALICE), &message);

    assert!(result.is_ok());
}

#[test]
fn message_processor_succeeds_even_if_xcm_is_invalid() {
    ExtBuilder::default().build().execute_with(|| {
        let sender: AccountId = AccountId::from(ALICE);
        let origin = GatewayAddress::get();

        let token: H160 = H160::random();
        let assets = vec![EthereumAsset::NativeTokenERC20 {
            token_id: token,
            value: 1_000_000_000_000u128,
        }];

        let claimer_acc_id = H256::random();
        let claimer = AccountId32 {
            network: None,
            id: claimer_acc_id.into(),
        };
        let claimer_bytes = claimer.encode();
        let raw_payload = RawPayload::Xcm(vec![0xAA, 0xBB, 0xCC].encode()); // invalid XCM
        let message = Message {
            gateway: origin,
            nonce: 1,
            origin,
            assets: assets.clone(),
            payload: Payload::Raw(raw_payload.encode()),
            claimer: Some(claimer_bytes),
            value: 1_000_000_000_000u128,
            execution_fee: 0,
            relayer_fee: 0,
        };

        type Processor = RawMessageProcessor<
            Runtime,
            GatewayAddress,
            DefaultClaimer,
            EthereumNetwork,
            EthereumUniversalLocation,
            TanssiUniversalLocation,
            xcm_executor::XcmExecutor<xcm_config::XcmConfig>,
            <xcm_config::XcmConfig as xcm_executor::Config>::Weigher,
            MaxXcmWeight,
        >;

        let result = <Processor as snowbridge_inbound_queue_primitives::v2::MessageProcessor<
            AccountId,
        >>::process_message(sender.clone(), message.clone());

        assert!(
            result.is_ok(),
            "Incorrect XCM still results in successful message processing"
        );
    });
}

#[test]
fn message_processor_fails_with_invalid_symbiotic_payload() {
    ExtBuilder::default().build().execute_with(|| {
        let sender: AccountId = AccountId::from(ALICE);
        let origin = GatewayAddress::get();

        let token: H160 = H160::random();
        let assets = vec![EthereumAsset::NativeTokenERC20 {
            token_id: token,
            value: 1_000_000_000_000u128,
        }];

        let claimer_acc_id = H256::random();
        let claimer = AccountId32 {
            network: None,
            id: claimer_acc_id.into(),
        };
        let claimer_bytes = claimer.encode();
        let raw_payload = RawPayload::Symbiotic(vec![0xAA, 0xBB, 0xCC].encode()); // invalid symbiotic payload
        let message = Message {
            gateway: origin,
            nonce: 1,
            origin,
            assets: assets.clone(),
            payload: Payload::Raw(raw_payload.encode()),
            claimer: Some(claimer_bytes),
            value: 1_000_000_000_000u128,
            execution_fee: 0,
            relayer_fee: 0,
        };

        type Processor = RawMessageProcessor<
            Runtime,
            GatewayAddress,
            DefaultClaimer,
            EthereumNetwork,
            EthereumUniversalLocation,
            TanssiUniversalLocation,
            xcm_executor::XcmExecutor<xcm_config::XcmConfig>,
            <xcm_config::XcmConfig as xcm_executor::Config>::Weigher,
            MaxXcmWeight,
        >;

        let result = <Processor as snowbridge_inbound_queue_primitives::v2::MessageProcessor<
            AccountId,
        >>::process_message(sender.clone(), message.clone());

        assert!(
            result.is_err(),
            "Incorrect Symbiotic payload should result in error"
        );
    });
}

#[test]
fn try_extract_message_fails_with_invalid_xcm_payload() {
    ExtBuilder::default().build().execute_with(|| {
        let origin = GatewayAddress::get();
        let sender: AccountId = AccountId::from(ALICE);

        let token: H160 = H160::random();
        let assets = vec![EthereumAsset::NativeTokenERC20 {
            token_id: token,
            value: 1_000_000_000_000u128,
        }];

        let claimer_acc_id = H256::random();
        let claimer = AccountId32 {
            network: None,
            id: claimer_acc_id.into(),
        };
        let claimer_bytes = claimer.encode();

        let raw_payload = RawPayload::Xcm(vec![0xAA, 0xBB, 0xCC].encode());

        let message = Message {
            gateway: origin,
            nonce: 1,
            origin,
            assets: assets.clone(),
            payload: Payload::Raw(raw_payload.encode()),
            claimer: Some(claimer_bytes),
            value: 1_000_000_000_000u128,
            execution_fee: 0,
            relayer_fee: 0,
        };

        type Processor = RawMessageProcessor<
            Runtime,
            GatewayAddress,
            DefaultClaimer,
            EthereumNetwork,
            EthereumUniversalLocation,
            TanssiUniversalLocation,
            xcm_executor::XcmExecutor<xcm_config::XcmConfig>,
            <xcm_config::XcmConfig as xcm_executor::Config>::Weigher,
            MaxXcmWeight,
        >;

        let result = <Processor as MessageProcessorWithFallback<AccountId>>::try_extract_message(
            &sender, &message,
        );

        // Confirm that we receive InvalidMessage, so Fallback should be triggered for XCM message
        assert!(
            matches!(result, Err(MessageExtractionError::InvalidMessage { .. })),
            "Invalid XCM payload should result in InvalidMessage error, got: {:?}",
            result
        );
    });
}

#[test]
fn try_extract_message_fails_with_invalid_symbiotic_payload() {
    ExtBuilder::default().build().execute_with(|| {
        let origin = GatewayAddress::get();
        let sender: AccountId = AccountId::from(ALICE);

        let token: H160 = H160::random();
        let assets = vec![EthereumAsset::NativeTokenERC20 {
            token_id: token,
            value: 1_000_000_000_000u128,
        }];

        let claimer_acc_id = H256::random();
        let claimer = AccountId32 {
            network: None,
            id: claimer_acc_id.into(),
        };
        let claimer_bytes = claimer.encode();

        let raw_payload = RawPayload::Symbiotic(vec![0xAA, 0xBB, 0xCC].encode());

        let message = Message {
            gateway: origin,
            nonce: 1,
            origin,
            assets: assets.clone(),
            payload: Payload::Raw(raw_payload.encode()),
            claimer: Some(claimer_bytes),
            value: 1_000_000_000_000u128,
            execution_fee: 0,
            relayer_fee: 0,
        };

        type Processor = RawMessageProcessor<
            Runtime,
            GatewayAddress,
            DefaultClaimer,
            EthereumNetwork,
            EthereumUniversalLocation,
            TanssiUniversalLocation,
            xcm_executor::XcmExecutor<xcm_config::XcmConfig>,
            <xcm_config::XcmConfig as xcm_executor::Config>::Weigher,
            MaxXcmWeight,
        >;

        let result = <Processor as MessageProcessorWithFallback<AccountId>>::try_extract_message(
            &sender, &message,
        );

        // Confirm that we receive UnsupportedMessage, so Fallback won't be triggered for Symbiotic message
        assert!(
            matches!(
                result,
                Err(MessageExtractionError::UnsupportedMessage { .. })
            ),
            "Invalid Symbiotic payload should result in UnsupportedMessage error, got: {:?}",
            result
        );
    });
}
