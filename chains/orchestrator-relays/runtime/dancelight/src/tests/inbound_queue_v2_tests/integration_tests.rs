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
        bridge_to_ethereum_config::EthereumGatewayAddress,
        tests::common::{
            encode_message_to_eth_payload, mock_snowbridge_message_proof, root_origin, ExtBuilder,
            ALICE, BOB, UNIT,
        },
        xcm_config::UniversalLocation,
        AccountId, Balances, EthereumInboundQueueV2, EthereumSovereignAccount, EthereumSystemV2,
        ExternalValidators, ForeignAssetsCreator, Runtime, RuntimeEvent,
    },
    dancelight_runtime_constants::snowbridge::{EthereumLocation, EthereumNetwork},
    frame_support::assert_ok,
    frame_system::pallet_prelude::OriginFor,
    hex_literal::hex,
    keyring::Sr25519Keyring,
    parity_scale_codec::Encode,
    snowbridge_core::{Channel, PRIMARY_GOVERNANCE_CHANNEL},
    snowbridge_inbound_queue_primitives::v2::{
        EthereumAsset::{ForeignTokenERC20, NativeTokenERC20},
        Message, Payload,
    },
    snowbridge_verification_primitives::{EventProof, Log},
    sp_core::{H160, H256},
    tanssi_runtime_common::processors::v2::RawPayload,
    tp_bridge::symbiotic_message_processor::{
        InboundCommand, Message as SymbioticMessage, Payload as SymbioticPayload, MAGIC_BYTES,
    },
    xcm::{
        latest::prelude::{Junctions::*, *},
        VersionedXcm,
    },
};

#[test]
fn test_inbound_queue_message_symbiotic_passing() {
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

        let dummy_proof = mock_snowbridge_message_proof();

        let payload_validators = vec![
            Sr25519Keyring::Charlie.to_account_id(),
            Sr25519Keyring::Ferdie.to_account_id(),
            Sr25519Keyring::BobStash.to_account_id()
        ];

        let payload = SymbioticPayload {
            magic_bytes: MAGIC_BYTES,
            message: SymbioticMessage::V1(InboundCommand::<Runtime>::ReceiveValidators {
                validators: payload_validators.clone(),
                external_index: 0u64,
            }),
        };

        let symbiotic_bytes = RawPayload::Symbiotic(payload.encode());

        let encoded_data = encode_message_to_eth_payload(&Message {
            gateway: H160::random(),
            nonce: 1,
            origin: EthereumGatewayAddress::get(),
            assets: vec![],
            payload: Payload::Raw(symbiotic_bytes.encode()),
            claimer: None,
            value: 0,
            execution_fee: 0,
            relayer_fee: 0,
        });

        assert_eq!(EthereumInboundQueueV2::submit(OriginFor::<Runtime>::signed(AccountId::new([0; 32])), Box::new(EventProof {
            event_log: Log {
                address: <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                topics: vec![hex!("550e2067494b1736ea5573f2d19cdc0ac95b410fff161bf16f11c6229655ec9c").into()],
                data: encoded_data,
            },
            proof: dummy_proof.clone(),
        })), Ok(()));

        let expected_validators = [ExternalValidators::whitelisted_validators(), payload_validators].concat();
        assert_eq!(ExternalValidators::validators(), expected_validators);
    });
}

#[test]
fn test_inbound_queue_transfer_eth_works() {
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
        let nonce = 1;

        let dummy_proof = mock_snowbridge_message_proof();
        let token_location = Location::here();

        assert_ok!(EthereumSystemV2::register_token(
            root_origin(),
            Box::new(token_location.clone().into()),
            Box::new(token_location.clone().into()),
            snowbridge_core::AssetMetadata {
                name: "relay".as_bytes().to_vec().try_into().unwrap(),
                symbol: "relay".as_bytes().to_vec().try_into().unwrap(),
                decimals: 12,
            },
            1
        ));

        let eth_native_asset_location = Location {
            parents: 1,
            interior: X1([GlobalConsensus(EthereumNetwork::get())].into()),
        };

        let asset_id = 42u16;

        assert_ok!(ForeignAssetsCreator::create_foreign_asset(
            root_origin(),
            eth_native_asset_location.clone(),
            asset_id,
            AccountId::from(ALICE),
            true,
            1
        ));

        let assets = vec![];

        println!("assets: {:?}", assets);

        let _execution_fee = 0;

        let instructions = vec![DepositAsset {
            assets: Wild(AllCounted(1)),
            beneficiary: Location::new(
                0,
                AccountId32 {
                    network: None,
                    id: H256::random().into(),
                },
            ),
        }];

        let xcm: Xcm<()> = instructions.into();
        let versioned_message_xcm = VersionedXcm::V5(xcm);

        let xcm_bytes = RawPayload::Xcm(versioned_message_xcm.encode());

        let encoded_data = encode_message_to_eth_payload(&Message {
            gateway: H160::random(),
            nonce,
            origin: EthereumGatewayAddress::get(),
            assets,
            payload: Payload::Raw(xcm_bytes.encode()),
            claimer: None,
            value: 12345,
            execution_fee: 0,
            relayer_fee: 0,
        });

        assert_eq!(EthereumInboundQueueV2::submit(OriginFor::<Runtime>::signed(AccountId::new([0; 32])), Box::new(EventProof {
            event_log: Log {
                address: <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                topics: vec![hex!("550e2067494b1736ea5573f2d19cdc0ac95b410fff161bf16f11c6229655ec9c").into()],
                data: encoded_data,
            },
            proof: dummy_proof.clone(),
        })), Ok(()));

        let events = frame_system::Pallet::<Runtime>::events();
        for e in events.clone() {
            println!("Event: {:?}", e.event);
        }

        let mut found_message = false;
        let mut found_issued = false;

        for record in events {
            match &record.event {
                RuntimeEvent::EthereumInboundQueueV2(
                    snowbridge_pallet_inbound_queue_v2::Event::MessageReceived { nonce, .. }
                ) if *nonce == *nonce => {
                    found_message = true;
                }

                RuntimeEvent::ForeignAssets(
                    pallet_assets::Event::Issued { amount, .. }
                ) if *amount == 12345 => {
                    found_issued = true;
                }

                _ => {}
            }
        }

        assert!(found_message, "MessageReceived event not found");
        assert!(found_issued, "Issued event for ETH not found");
    });
}

#[test]
fn test_inbound_queue_transfer_tanssi_works() {
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
        let nonce = 1;

        let dummy_proof = mock_snowbridge_message_proof();
        let token_location = Location::here();

        assert_ok!(EthereumSystemV2::register_token(
            root_origin(),
            Box::new(token_location.clone().into()),
            Box::new(token_location.clone().into()),
            snowbridge_core::AssetMetadata {
                name: "relay".as_bytes().to_vec().try_into().unwrap(),
                symbol: "relay".as_bytes().to_vec().try_into().unwrap(),
                decimals: 12,
            },
            1
        ));

        let eth_native_asset_location = Location {
            parents: 1,
            interior: X1([GlobalConsensus(EthereumNetwork::get())].into()),
        };

        let asset_id = 42u16;

        assert_ok!(
            Balances::force_set_balance(
                root_origin(),
                EthereumSovereignAccount::get().into(),
                10_000_000_000_000_000_000u128
            )
        );

        assert_ok!(ForeignAssetsCreator::create_foreign_asset(
            root_origin(),
            eth_native_asset_location.clone(),
            asset_id,
            AccountId::from(ALICE),
            true,
            1
        ));

        let token_location_reanchored = token_location
            .clone()
            .reanchored(&EthereumLocation::get(), &UniversalLocation::get())
            .expect("unable to reanchor token");

        let tanssi_token_id =
            snowbridge_pallet_system::NativeToForeignId::<Runtime>::get(&token_location_reanchored).unwrap();

        let tanssi_token_transfer_value = 123_456_789_000u128;
        let assets = vec![
            ForeignTokenERC20 { token_id: tanssi_token_id.into(), value: tanssi_token_transfer_value },
        ];

        let _execution_fee = 0;

        let instructions = vec![DepositAsset {
            assets: Wild(AllCounted(1)),
            beneficiary: Location::new(
                0,
                AccountId32 {
                    network: None,
                    id: H256::random().into(),
                },
            ),
        }];

        let xcm: Xcm<()> = instructions.into();
        let versioned_message_xcm = VersionedXcm::V5(xcm);

        let xcm_bytes = RawPayload::Xcm(versioned_message_xcm.encode());

        let encoded_data = encode_message_to_eth_payload(&Message {
            gateway: H160::random(),
            nonce,
            origin: EthereumGatewayAddress::get(),
            assets,
            payload: Payload::Raw(xcm_bytes.encode()),
            claimer: None,
            value: 0,
            execution_fee: 0,
            relayer_fee: 0,
        });

        assert_eq!(EthereumInboundQueueV2::submit(OriginFor::<Runtime>::signed(AccountId::new([0; 32])), Box::new(EventProof {
            event_log: Log {
                address: <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                topics: vec![hex!("550e2067494b1736ea5573f2d19cdc0ac95b410fff161bf16f11c6229655ec9c").into()],
                data: encoded_data,
            },
            proof: dummy_proof.clone(),
        })), Ok(()));

        let events = frame_system::Pallet::<Runtime>::events();
        for e in events.clone() {
            println!("Event: {:?}", e.event);
        }

        let mut found_message = false;
        let mut found_minted = false;

        for record in events {
            match &record.event {
                RuntimeEvent::EthereumInboundQueueV2(
                    snowbridge_pallet_inbound_queue_v2::Event::MessageReceived { nonce, .. }
                ) if *nonce == *nonce => {
                    found_message = true;
                }

                RuntimeEvent::Balances(
                    pallet_balances::Event::Minted { amount, .. }
                ) if *amount == 123_456_789_000u128 => {
                    found_minted = true;
                }

                _ => {}
            }
        }

        assert!(found_message, "MessageReceived event not found");
        assert!(found_minted, "Minted event for native token not found");
    });
}

#[test]
fn test_inbound_queue_transfer_tanssi_and_eth_works() {
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
        let nonce = 1;

        let dummy_proof = mock_snowbridge_message_proof();
        let token_location = Location::here();

        assert_ok!(EthereumSystemV2::register_token(
            root_origin(),
            Box::new(token_location.clone().into()),
            Box::new(token_location.clone().into()),
            snowbridge_core::AssetMetadata {
                name: "relay".as_bytes().to_vec().try_into().unwrap(),
                symbol: "relay".as_bytes().to_vec().try_into().unwrap(),
                decimals: 12,
            },
            1
        ));

        let eth_native_asset_location = Location {
            parents: 1,
            interior: X1([GlobalConsensus(EthereumNetwork::get())].into()),
        };

        let asset_id = 42u16;

        assert_ok!(
            Balances::force_set_balance(
                root_origin(),
                EthereumSovereignAccount::get().into(),
                10_000_000_000_000_000_000u128
            )
        );

        assert_ok!(ForeignAssetsCreator::create_foreign_asset(
            root_origin(),
            eth_native_asset_location.clone(),
            asset_id,
            AccountId::from(ALICE),
            true,
            1
        ));

        let token_location_reanchored = token_location
            .clone()
            .reanchored(&EthereumLocation::get(), &UniversalLocation::get())
            .expect("unable to reanchor token");

        let tanssi_token_id =
            snowbridge_pallet_system::NativeToForeignId::<Runtime>::get(&token_location_reanchored).unwrap();

        let tanssi_token_transfer_value = 123_456_000u128;
        let assets = vec![
            ForeignTokenERC20 { token_id: tanssi_token_id.into(), value: tanssi_token_transfer_value },
        ];

        println!("assets: {:?}", assets);

        let _execution_fee = 0;

        let instructions = vec![DepositAsset {
            assets: Wild(AllCounted(2)),
            beneficiary: Location::new(
                0,
                AccountId32 {
                    network: None,
                    id: H256::random().into(),
                },
            ),
        }];

        let xcm: Xcm<()> = instructions.into();
        let versioned_message_xcm = VersionedXcm::V5(xcm);

        let xcm_bytes = RawPayload::Xcm(versioned_message_xcm.encode());
        let eth_value = 789_000_000u128;

        let encoded_data = encode_message_to_eth_payload(&Message {
            gateway: H160::random(),
            nonce,
            origin: EthereumGatewayAddress::get(),
            assets,
            payload: Payload::Raw(xcm_bytes.encode()),
            claimer: None,
            value: eth_value,
            execution_fee: 0,
            relayer_fee: 0,
        });

        assert_eq!(EthereumInboundQueueV2::submit(OriginFor::<Runtime>::signed(AccountId::new([0; 32])), Box::new(EventProof {
            event_log: Log {
                address: <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                topics: vec![hex!("550e2067494b1736ea5573f2d19cdc0ac95b410fff161bf16f11c6229655ec9c").into()],
                data: encoded_data,
            },
            proof: dummy_proof.clone(),
        })), Ok(()));

        let events = frame_system::Pallet::<Runtime>::events();
        for e in events.clone() {
            println!("Event: {:?}", e.event);
        }

        let mut found_message = false;
        let mut found_minted = false;
        let mut found_issued = false;

        for record in events {
            match &record.event {
                RuntimeEvent::EthereumInboundQueueV2(
                    snowbridge_pallet_inbound_queue_v2::Event::MessageReceived { nonce, .. }
                ) if *nonce == *nonce => {
                    found_message = true;
                }

                RuntimeEvent::Balances(
                    pallet_balances::Event::Minted { amount, .. }
                ) if *amount == tanssi_token_transfer_value => {
                    found_minted = true;
                }

                RuntimeEvent::ForeignAssets(
                    pallet_assets::Event::Issued { amount, .. }
                ) if *amount == eth_value => {
                    found_issued = true;
                }

                _ => {}
            }
        }

        assert!(found_message, "MessageReceived event not found");
        assert!(found_minted, "Minted event for native token not found");
        assert!(found_issued, "Issued event for ETH not found");
    });
}

#[test]
fn test_inbound_queue_transfer_erc20_works() {
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
        let nonce = 1;

        let dummy_proof = mock_snowbridge_message_proof();
        let erc20_token_id = hex_literal::hex!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");

        let erc20_asset_location = Location {
            parents: 1,
            interior: X2([
                GlobalConsensus(Ethereum { chain_id: 11155111 }),
                AccountKey20 {
                    network: Some(Ethereum { chain_id: 11155111 }),
                    key: erc20_token_id,
                }
            ]
                .into()),
        };

        let asset_id = 43u16;

        assert_ok!(ForeignAssetsCreator::create_foreign_asset(
            root_origin(),
            erc20_asset_location.clone(),
            asset_id,
            AccountId::from(ALICE),
            true,
            1
        ));

        let token_value = 123_456_000u128;
        let assets = vec![
            NativeTokenERC20 { token_id: erc20_token_id.into(), value: token_value },
        ];

        println!("assets: {:?}", assets);

        let _execution_fee = 0;

        let instructions = vec![DepositAsset {
            assets: Wild(AllCounted(1)),
            beneficiary: Location::new(
                0,
                AccountId32 {
                    network: None,
                    id: H256::random().into(),
                },
            ),
        }];

        let xcm: Xcm<()> = instructions.into();
        let versioned_message_xcm = VersionedXcm::V5(xcm);

        let xcm_bytes = RawPayload::Xcm(versioned_message_xcm.encode());

        let encoded_data = encode_message_to_eth_payload(&Message {
            gateway: H160::random(),
            nonce,
            origin: EthereumGatewayAddress::get(),
            assets,
            payload: Payload::Raw(xcm_bytes.encode()),
            claimer: None,
            value: 0,
            execution_fee: 0,
            relayer_fee: 0,
        });

        assert_eq!(EthereumInboundQueueV2::submit(OriginFor::<Runtime>::signed(AccountId::new([0; 32])), Box::new(EventProof {
            event_log: Log {
                address: <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                topics: vec![hex!("550e2067494b1736ea5573f2d19cdc0ac95b410fff161bf16f11c6229655ec9c").into()],
                data: encoded_data,
            },
            proof: dummy_proof.clone(),
        })), Ok(()));

        let events = frame_system::Pallet::<Runtime>::events();
        for e in events.clone() {
            println!("Event: {:?}", e.event);
        }

        let mut found_message = false;
        let mut found_issued = false;

        for record in events {
            match &record.event {
                RuntimeEvent::EthereumInboundQueueV2(
                    snowbridge_pallet_inbound_queue_v2::Event::MessageReceived { nonce, .. }
                ) if *nonce == *nonce => {
                    found_message = true;
                }

                RuntimeEvent::ForeignAssets(
                    pallet_assets::Event::Issued { amount, .. }
                ) if *amount == 123_456_000u128 => {
                    found_issued = true;
                }

                _ => {}
            }
        }

        assert!(found_message, "MessageReceived event not found");
        assert!(found_issued, "Issued event for ETH not found");
    });
}

#[test]
fn test_inbound_queue_tanssi_assets_trapped_incorrect_xcm_works() {
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
        let nonce = 1;

        let dummy_proof = mock_snowbridge_message_proof();
        let erc20_token_id = hex_literal::hex!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");

        let erc20_asset_location = Location {
            parents: 1,
            interior: X2([
                GlobalConsensus(Ethereum { chain_id: 11155111 }),
                AccountKey20 {
                    network: Some(Ethereum { chain_id: 11155111 }),
                    key: erc20_token_id,
                }
            ]
                .into()),
        };

        let asset_id = 43u16;

        assert_ok!(ForeignAssetsCreator::create_foreign_asset(
            root_origin(),
            erc20_asset_location.clone(),
            asset_id,
            AccountId::from(ALICE),
            true,
            1
        ));

        let token_value = 123_456_000u128;
        let assets = vec![
            NativeTokenERC20 { token_id: erc20_token_id.into(), value: token_value },
        ];

        println!("assets: {:?}", assets);

        let _execution_fee = 0;

        let instructions = vec![];

        let xcm: Xcm<()> = instructions.into();
        let versioned_message_xcm = VersionedXcm::V5(xcm);

        let xcm_bytes = RawPayload::Xcm(versioned_message_xcm.encode());

        let encoded_data = encode_message_to_eth_payload(&Message {
            gateway: H160::random(),
            nonce,
            origin: EthereumGatewayAddress::get(),
            assets,
            payload: Payload::Raw(xcm_bytes.encode()),
            claimer: None,
            value: 0,
            execution_fee: 0,
            relayer_fee: 0,
        });

        assert_eq!(EthereumInboundQueueV2::submit(OriginFor::<Runtime>::signed(AccountId::new([0; 32])), Box::new(EventProof {
            event_log: Log {
                address: <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                topics: vec![hex!("550e2067494b1736ea5573f2d19cdc0ac95b410fff161bf16f11c6229655ec9c").into()],
                data: encoded_data,
            },
            proof: dummy_proof.clone(),
        })), Ok(()));

        let events = frame_system::Pallet::<Runtime>::events();
        for e in events.clone() {
            println!("Event: {:?}", e.event);
        }

        let mut found_message = false;
        let mut found_trapped = false;

        for record in events {
            match &record.event {
                RuntimeEvent::EthereumInboundQueueV2(
                    snowbridge_pallet_inbound_queue_v2::Event::MessageReceived { nonce, .. }
                ) if *nonce == *nonce => {
                    found_message = true;
                }

                RuntimeEvent::XcmPallet(
                    pallet_xcm::Event::AssetsTrapped { assets, .. }
                ) => {
                    let Ok(assets) = <Assets as TryFrom<_>>::try_from(assets.clone()) else {
                        panic!("Unsupported assets version");
                    };

                    if let Some(asset) = assets.get(0) {
                        if let Fungibility::Fungible(f) = asset.fun {
                            if f == token_value {
                                found_trapped = true;
                            }
                        }
                    }
                }

                _ => {}
            }
        }

        assert!(found_message, "MessageReceived event not found");
        assert!(found_trapped, "AssetsTrapped event not found");
    });
}

#[test]
fn test_inbound_queue_erc20_assets_trapped_incorrect_xcm_works() {
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
        let nonce = 1;

        let dummy_proof = mock_snowbridge_message_proof();
        let erc20_token_id = hex_literal::hex!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");

        let erc20_asset_location = Location {
            parents: 1,
            interior: X2([
                GlobalConsensus(Ethereum { chain_id: 11155111 }),
                AccountKey20 {
                    network: Some(Ethereum { chain_id: 11155111 }),
                    key: erc20_token_id,
                }
            ]
                .into()),
        };

        let asset_id = 43u16;

        assert_ok!(ForeignAssetsCreator::create_foreign_asset(
            root_origin(),
            erc20_asset_location.clone(),
            asset_id,
            AccountId::from(ALICE),
            true,
            1
        ));

        let assets = vec![];

        println!("assets: {:?}", assets);

        let _execution_fee = 0;

        let instructions = vec![];

        let xcm: Xcm<()> = instructions.into();
        let versioned_message_xcm = VersionedXcm::V5(xcm);

        let xcm_bytes = RawPayload::Xcm(versioned_message_xcm.encode());

        let eth_amount = 789_000_000u128;
        let encoded_data = encode_message_to_eth_payload(&Message {
            gateway: H160::random(),
            nonce,
            origin: EthereumGatewayAddress::get(),
            assets,
            payload: Payload::Raw(xcm_bytes.encode()),
            claimer: None,
            value: eth_amount,
            execution_fee: 0,
            relayer_fee: 0,
        });

        assert_eq!(EthereumInboundQueueV2::submit(OriginFor::<Runtime>::signed(AccountId::new([0; 32])), Box::new(EventProof {
            event_log: Log {
                address: <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                topics: vec![hex!("550e2067494b1736ea5573f2d19cdc0ac95b410fff161bf16f11c6229655ec9c").into()],
                data: encoded_data,
            },
            proof: dummy_proof.clone(),
        })), Ok(()));

        let events = frame_system::Pallet::<Runtime>::events();
        for e in events.clone() {
            println!("Event: {:?}", e.event);
        }

        let mut found_message = false;
        let mut found_trapped = false;

        for record in events {
            match &record.event {
                RuntimeEvent::EthereumInboundQueueV2(
                    snowbridge_pallet_inbound_queue_v2::Event::MessageReceived { nonce, .. }
                ) if *nonce == *nonce => {
                    found_message = true;
                }

                RuntimeEvent::XcmPallet(
                    pallet_xcm::Event::AssetsTrapped { assets, .. }
                ) => {
                    let Ok(assets) = <Assets as TryFrom<_>>::try_from(assets.clone()) else {
                        panic!("Unsupported assets version");
                    };

                    if let Some(asset) = assets.get(0) {
                        if let Fungibility::Fungible(f) = asset.fun {
                            if f == eth_amount {
                                found_trapped = true;
                            }
                        }
                    }
                }

                _ => {}
            }
        }

        assert!(found_message, "MessageReceived event not found");
        assert!(found_trapped, "AssetsTrapped event not found");
    });
}

#[test]
fn test_inbound_queue_incorrect_xcm_trap_assets_works() {
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
        let nonce = 1;

        let dummy_proof = mock_snowbridge_message_proof();
        let erc20_token_id = hex_literal::hex!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");

        let erc20_asset_location = Location {
            parents: 1,
            interior: X2([
                GlobalConsensus(Ethereum { chain_id: 11155111 }),
                AccountKey20 {
                    network: Some(Ethereum { chain_id: 11155111 }),
                    key: erc20_token_id,
                }
            ]
                .into()),
        };

        let asset_id = 43u16;

        assert_ok!(ForeignAssetsCreator::create_foreign_asset(
            root_origin(),
            erc20_asset_location.clone(),
            asset_id,
            AccountId::from(ALICE),
            true,
            1
        ));

        let assets = vec![];

        println!("assets: {:?}", assets);

        let _execution_fee = 0;

        let instructions = vec![
            // Just random incorrect instruction
            WithdrawAsset(Assets::new())
        ];

        let xcm: Xcm<()> = instructions.into();
        let versioned_message_xcm = VersionedXcm::V5(xcm);

        let xcm_bytes = RawPayload::Xcm(versioned_message_xcm.encode());

        let eth_amount = 789_000_000u128;
        let encoded_data = encode_message_to_eth_payload(&Message {
            gateway: H160::random(),
            nonce,
            origin: EthereumGatewayAddress::get(),
            assets,
            payload: Payload::Raw(xcm_bytes.encode()),
            claimer: None,
            value: eth_amount,
            execution_fee: 0,
            relayer_fee: 0,
        });

        assert_eq!(EthereumInboundQueueV2::submit(OriginFor::<Runtime>::signed(AccountId::new([0; 32])), Box::new(EventProof {
            event_log: Log {
                address: <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                topics: vec![hex!("550e2067494b1736ea5573f2d19cdc0ac95b410fff161bf16f11c6229655ec9c").into()],
                data: encoded_data,
            },
            proof: dummy_proof.clone(),
        })), Ok(()));

        let events = frame_system::Pallet::<Runtime>::events();
        for e in events.clone() {
            println!("Event: {:?}", e.event);
        }

        let mut found_message = false;
        let mut found_trapped = false;

        for record in events {
            match &record.event {
                RuntimeEvent::EthereumInboundQueueV2(
                    snowbridge_pallet_inbound_queue_v2::Event::MessageReceived { nonce, .. }
                ) if *nonce == *nonce => {
                    found_message = true;
                }

                RuntimeEvent::XcmPallet(
                    pallet_xcm::Event::AssetsTrapped { assets, .. }
                ) => {
                    let Ok(assets) = <Assets as TryFrom<_>>::try_from(assets.clone()) else {
                        panic!("Unsupported assets version");
                    };

                    if let Some(asset) = assets.get(0) {
                        if let Fungibility::Fungible(f) = asset.fun {
                            if f == eth_amount {
                                found_trapped = true;
                            }
                        }
                    }
                }

                _ => {}
            }
        }

        assert!(found_message, "MessageReceived event not found");
        assert!(found_trapped, "AssetsTrapped event not found");
    });
}
