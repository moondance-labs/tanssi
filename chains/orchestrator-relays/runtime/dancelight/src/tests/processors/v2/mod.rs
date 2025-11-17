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
    crate as dancelight_runtime,
    crate::tests::common::{root_origin, ExtBuilder},
    dancelight_runtime::{AccountId, Runtime},
    dancelight_runtime_constants::DANCELIGHT_GENESIS_HASH,
    frame_support::{assert_ok, parameter_types, BoundedVec},
    parity_scale_codec::Encode,
    snowbridge_core::{AgentId, ChannelId},
    snowbridge_inbound_queue_primitives::v2::EthereumAsset,
    sp_core::{H160, H256},
    tanssi_runtime_common::processors::v2::{
        prepare_raw_message_xcm_instructions, ExtractedXcmConstructionInfo,
        RAW_MESSAGE_PROCESSOR_TOPIC_PREFIX,
    },
    xcm::latest::{prelude::*, Junctions::*, Location},
};

mod raw_message_processor;

// TODO: Move later to dancelight-runtime-test-utils after refactoring
pub const ALICE: [u8; 32] = [4u8; 32];
pub const BOB: [u8; 32] = [5u8; 32];

// TODO: Get from runtime once wired
parameter_types! {
    const EthereumNetwork: NetworkId = Ethereum { chain_id: 11155111 };
    const BridgeChannelInfo: Option<(ChannelId, AgentId)> = Some((ChannelId::new([1u8; 32]), H256([2u8; 32])));
    pub EthereumUniversalLocation: InteriorLocation = GlobalConsensus(EthereumNetwork::get()).into();
    pub TanssiUniversalLocation: InteriorLocation = GlobalConsensus(ByGenesis(dancelight_runtime_constants::DANCELIGHT_GENESIS_HASH)).into();
    pub GatewayAddress: H160 = H160::random();
    pub DefaultClaimer: AccountId = AccountId::from(ALICE);
}

#[test]
fn prepare_raw_message_xcm_instructions_without_claimer_works() {
    let nonce = 1u64;
    let extracted_message: ExtractedXcmConstructionInfo<dancelight_runtime::RuntimeCall> =
        ExtractedXcmConstructionInfo {
            origin: H160::random(),
            maybe_claimer: None,
            assets: vec![EthereumAsset::NativeTokenERC20 {
                token_id: H160::random(),
                value: 12345,
            }],
            eth_value: 1000000,
            execution_fee_in_eth: 100,
            nonce,
            user_xcm: Xcm::new(),
        };

    let res = prepare_raw_message_xcm_instructions::<Runtime>(
        EthereumNetwork::get(),
        &EthereumUniversalLocation::get(),
        &TanssiUniversalLocation::get(),
        GatewayAddress::get(),
        DefaultClaimer::get(),
        RAW_MESSAGE_PROCESSOR_TOPIC_PREFIX,
        extracted_message,
    );

    assert_ok!(&res);
    let instructions = res.unwrap();

    assert!(!instructions.is_empty(), "instructions must not be empty");

    if let Some(SetHints { hints }) = instructions.first() {
        let hints: &BoundedVec<Hint, _> = hints;

        assert_eq!(hints.len(), 1);

        match &hints[0] {
            Hint::AssetClaimer { location } => {
                assert_eq!(location.parents, 0);
                assert_eq!(
                    location.interior,
                    X1([AccountId32 {
                        network: None,
                        id: DefaultClaimer::get().clone().into(),
                    }]
                    .into())
                );
            }
        }
    } else {
        panic!("Expected SetHints instruction first");
    }

    assert!(instructions
        .iter()
        .any(|i| matches!(i, ReserveAssetDeposited(_))));

    assert!(instructions.iter().any(|i| matches!(i, PayFees { .. })));

    assert!(instructions
        .iter()
        .any(|i| matches!(i, ReserveAssetDeposited(_))));

    assert!(
        instructions.iter().any(|i| matches!(i, DescendOrigin(_))),
        "expected DescendOrigin"
    );

    if let Some(SetTopic(topic_hash)) = instructions.last() {
        let expected = sp_core::blake2_256(&(RAW_MESSAGE_PROCESSOR_TOPIC_PREFIX, nonce).encode());
        assert_eq!(*topic_hash, expected, "topic hash mismatch");
    } else {
        panic!("last instruction must be SetTopic");
    }
}

#[test]
fn prepare_raw_message_xcm_instructions_with_claimer_works() {
    let nonce = 1u64;
    let extracted_message: ExtractedXcmConstructionInfo<dancelight_runtime::RuntimeCall> =
        ExtractedXcmConstructionInfo {
            origin: H160::random(),
            maybe_claimer: Some(vec![
                0, 1, 1, 0, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
                5, 5, 5, 5, 5, 5, 5, 5,
            ]),
            assets: vec![EthereumAsset::NativeTokenERC20 {
                token_id: H160::random(),
                value: 12345,
            }],
            eth_value: 1000000,
            execution_fee_in_eth: 100,
            nonce,
            user_xcm: Xcm::new(),
        };

    let res = prepare_raw_message_xcm_instructions::<Runtime>(
        EthereumNetwork::get(),
        &EthereumUniversalLocation::get(),
        &TanssiUniversalLocation::get(),
        GatewayAddress::get(),
        DefaultClaimer::get(),
        RAW_MESSAGE_PROCESSOR_TOPIC_PREFIX,
        extracted_message,
    );

    assert_ok!(&res);
    let instructions = res.unwrap();

    assert!(!instructions.is_empty(), "instructions must not be empty");

    if let Some(SetHints { hints }) = instructions.first() {
        let hints: &BoundedVec<Hint, _> = hints;

        assert_eq!(hints.len(), 1);

        match &hints[0] {
            Hint::AssetClaimer { location } => {
                assert_eq!(location.parents, 0);
                assert_eq!(
                    location.interior,
                    X1([AccountId32 {
                        network: None,
                        id: AccountId::from(BOB).clone().into(),
                    }]
                    .into())
                );
            }
        }
    } else {
        panic!("Expected SetHints instruction first");
    }

    assert!(instructions
        .iter()
        .any(|i| matches!(i, ReserveAssetDeposited(_))));

    assert!(instructions.iter().any(|i| matches!(i, PayFees { .. })));

    assert!(instructions
        .iter()
        .any(|i| matches!(i, ReserveAssetDeposited(_))));

    assert!(
        instructions.iter().any(|i| matches!(i, DescendOrigin(_))),
        "expected DescendOrigin"
    );

    if let Some(SetTopic(topic_hash)) = instructions.last() {
        let expected = sp_core::blake2_256(&(RAW_MESSAGE_PROCESSOR_TOPIC_PREFIX, nonce).encode());
        assert_eq!(*topic_hash, expected, "topic hash mismatch");
    } else {
        panic!("last instruction must be SetTopic");
    }
}

#[test]
fn prepare_raw_message_xcm_instructions_with_foreign_asset_works() {
    ExtBuilder::default().build().execute_with(|| {
        let nonce = 1u64;

        let token_location = Location::here();
        let reanchored_location = Location {
            parents: 1,
            interior: X1([GlobalConsensus(NetworkId::ByGenesis(
                DANCELIGHT_GENESIS_HASH,
            ))]
            .into()),
        };

        assert_ok!(snowbridge_pallet_system::Pallet::<Runtime>::register_token(
            root_origin(),
            Box::new(token_location.clone().into()),
            snowbridge_core::AssetMetadata {
                name: "para".as_bytes().to_vec().try_into().unwrap(),
                symbol: "para".as_bytes().to_vec().try_into().unwrap(),
                decimals: 12,
            }
        ));

        let token_id =
            snowbridge_pallet_system::NativeToForeignId::<Runtime>::get(reanchored_location)
                .unwrap();

        let extracted_message: ExtractedXcmConstructionInfo<dancelight_runtime::RuntimeCall> =
            ExtractedXcmConstructionInfo {
                origin: H160::random(),
                maybe_claimer: Some(vec![
                    0, 1, 1, 0, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
                    5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
                ]),
                assets: vec![EthereumAsset::ForeignTokenERC20 {
                    token_id,
                    value: 12345,
                }],
                eth_value: 1000000,
                execution_fee_in_eth: 100,
                nonce,
                user_xcm: Xcm::new(),
            };

        let res = prepare_raw_message_xcm_instructions::<Runtime>(
            EthereumNetwork::get(),
            &EthereumUniversalLocation::get(),
            &TanssiUniversalLocation::get(),
            GatewayAddress::get(),
            DefaultClaimer::get(),
            RAW_MESSAGE_PROCESSOR_TOPIC_PREFIX,
            extracted_message,
        );

        assert_ok!(&res);
        let instructions = res.unwrap();

        assert!(!instructions.is_empty(), "instructions must not be empty");

        if let Some(SetHints { hints }) = instructions.first() {
            let hints: &BoundedVec<Hint, _> = hints;

            assert_eq!(hints.len(), 1);

            match &hints[0] {
                Hint::AssetClaimer { location } => {
                    assert_eq!(location.parents, 0);
                    assert_eq!(
                        location.interior,
                        X1([AccountId32 {
                            network: None,
                            id: AccountId::from(BOB).clone().into(),
                        }]
                        .into())
                    );
                }
            }
        } else {
            panic!("Expected SetHints instruction first");
        }

        assert!(instructions
            .iter()
            .any(|i| matches!(i, ReserveAssetDeposited(_))));

        assert!(instructions.iter().any(|i| matches!(i, PayFees { .. })));

        assert!(instructions
            .iter()
            .any(|i| matches!(i, ReserveAssetDeposited(_))));

        assert!(instructions.iter().any(|i| matches!(i, WithdrawAsset(_))));

        assert!(
            instructions.iter().any(|i| matches!(i, DescendOrigin(_))),
            "expected DescendOrigin"
        );

        if let Some(SetTopic(topic_hash)) = instructions.last() {
            let expected =
                sp_core::blake2_256(&(RAW_MESSAGE_PROCESSOR_TOPIC_PREFIX, nonce).encode());
            assert_eq!(*topic_hash, expected, "topic hash mismatch");
        } else {
            panic!("last instruction must be SetTopic");
        }
    });
}
