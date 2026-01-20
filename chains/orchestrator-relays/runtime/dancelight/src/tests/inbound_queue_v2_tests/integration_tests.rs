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

use crate::{RuntimeCall, UncheckedExtrinsic, Utility};
use frame_support::dispatch::DispatchErrorWithPostInfo;
use frame_support::dispatch::{CallableCallFor, GetDispatchInfo};
use frame_support::pallet_prelude::Pays;
use sp_runtime::traits::Dispatchable;
use {
    crate::{
        bridge_to_ethereum_config::EthereumGatewayAddress,
        tests::common::{
            mock_snowbridge_message_proof, root_origin, ExtBuilder, ALICE, BOB, FERDIE, UNIT,
        },
        xcm_config::UniversalLocation,
        AccountId, Balances, EthereumInboundQueueV2, EthereumSovereignAccount, EthereumSystemV2,
        ExternalValidators, ForeignAssets, ForeignAssetsCreator, Runtime, RuntimeEvent, System,
    },
    alloy_core::{
        primitives::{Address, FixedBytes},
        sol_types::{SolEvent, SolValue},
    },
    dancelight_runtime_constants::snowbridge::{EthereumLocation, EthereumNetwork},
    frame_support::assert_ok,
    frame_support::dispatch::PostDispatchInfo,
    frame_system::pallet_prelude::OriginFor,
    keyring::Sr25519Keyring,
    parity_scale_codec::Encode,
    snowbridge_inbound_queue_primitives::v2::message::IGatewayV2,
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

const ETH_CHAIN_ID: u64 = 11155111;

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
        let nonce_val = 1u64;

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

        let event = IGatewayV2::OutboundMessageAccepted {
            nonce: nonce_val,
            payload: IGatewayV2::Payload {
                origin: Address::from_slice(EthereumGatewayAddress::get().as_bytes()),
                assets: vec![],
                xcm: IGatewayV2::Xcm { kind: 0, data: symbiotic_bytes.encode().into() },
                claimer: vec![].into(),
                value: 0,
                executionFee: 0,
                relayerFee: 0,
            },
        };

        assert!(matches!(EthereumInboundQueueV2::submit(OriginFor::<Runtime>::signed(AccountId::new([0; 32])), Box::new(EventProof {
            event_log: Log {
                address: <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                topics: event
                    .encode_topics()
                    .into_iter()
                    .map(|word| H256::from(word.0.0))
                    .collect(),
                data: event.encode_data(),
            },
            proof: dummy_proof.clone(),
        })), Ok(PostDispatchInfo { actual_weight: Some(Weight { .. }), pays_fee: Pays::Yes })));

        let expected_validators = [ExternalValidators::whitelisted_validators(), payload_validators].concat();
        assert_eq!(ExternalValidators::validators(), expected_validators);
    });
}

#[test]
fn test_inbound_queue_message_symbiotic_incorrect_magic_bytes() {
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
        let nonce_val = 1u64;

        let payload_validators = vec![
            Sr25519Keyring::Charlie.to_account_id(),
            Sr25519Keyring::Ferdie.to_account_id(),
            Sr25519Keyring::BobStash.to_account_id()
        ];

        let payload = SymbioticPayload {
            magic_bytes: [1, 2, 3, 4],
            message: SymbioticMessage::V1(InboundCommand::<Runtime>::ReceiveValidators {
                validators: payload_validators.clone(),
                external_index: 0u64,
            }),
        };

        let symbiotic_bytes = RawPayload::Symbiotic(payload.encode());

        let event = IGatewayV2::OutboundMessageAccepted {
            nonce: nonce_val,
            payload: IGatewayV2::Payload {
                origin: Address::from_slice(EthereumGatewayAddress::get().as_bytes()),
                assets: vec![],
                xcm: IGatewayV2::Xcm { kind: 0, data: symbiotic_bytes.encode().into() },
                claimer: vec![].into(),
                value: 0,
                executionFee: 0,
                relayerFee: 0,
            },
        };

        let result = EthereumInboundQueueV2::submit(
            OriginFor::<Runtime>::signed(AccountId::new([0; 32])),
            Box::new(EventProof {
                event_log: Log {
                    address: <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                    topics: event.encode_topics().into_iter().map(|word| H256::from(word.0.0)).collect(),
                    data: event.encode_data(),
                },
                proof: dummy_proof.clone(),
            }),
        );

        assert!(matches!(result, Err(DispatchErrorWithPostInfo{ error: sp_runtime::DispatchError::Other(_), ..})));
    });
}

#[test]
fn test_inbound_queue_message_symbiotic_incorrect_gateway_origin() {
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
        let nonce_val = 1u64;

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

        let event = IGatewayV2::OutboundMessageAccepted {
            nonce: nonce_val,
            payload: IGatewayV2::Payload {
                origin: Address::from_slice(H160(hex_literal::hex!("abcdefabcd1234567890abcdefabcd1234567890")).as_bytes()), // <-- Incorrect origin
                assets: vec![],
                xcm: IGatewayV2::Xcm { kind: 0, data: symbiotic_bytes.encode().into() },
                claimer: vec![].into(),
                value: 0,
                executionFee: 0,
                relayerFee: 0,
            },
        };

        let result = EthereumInboundQueueV2::submit(
            OriginFor::<Runtime>::signed(AccountId::new([0; 32])),
            Box::new(EventProof {
                event_log: Log {
                    address: <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                    topics: event.encode_topics().into_iter().map(|word| H256::from(word.0.0)).collect(),
                    data: event.encode_data(),
                },
                proof: dummy_proof.clone(),
            }),
        );
        assert!(matches!(result, Ok(PostDispatchInfo { actual_weight: Some(Weight { .. }), pays_fee: Pays::Yes })));

        // Validators have not been set
        let validators = pallet_external_validators::ExternalValidators::<Runtime>::get();
        assert_ne!(validators, payload_validators);

        // And no events from pallet external validators have been emitted
        let events = frame_system::Pallet::<Runtime>::events();

        for record in events {
            match &record.event {
                RuntimeEvent::ExternalValidators(
                    ..
                ) => {
                    panic!("Got unexpected ExternalValidators event: {:?}", record.event);
                }

                _ => {}
            }
        }
    });
}

#[test]
fn test_inbound_queue_message_symbiotic_incorrect_payload() {
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
        let nonce_val = 1u64;

        let payload_bytes: Vec<u8> = vec![1, 2, 3];

        let symbiotic_bytes = RawPayload::Symbiotic(payload_bytes);

        let event = IGatewayV2::OutboundMessageAccepted {
            nonce: nonce_val,
            payload: IGatewayV2::Payload {
                origin: Address::from_slice(EthereumGatewayAddress::get().as_bytes()),
                assets: vec![],
                xcm: IGatewayV2::Xcm { kind: 0, data: symbiotic_bytes.encode().into() },
                claimer: vec![].into(),
                value: 0,
                executionFee: 0,
                relayerFee: 0,
            },
        };

        let result = EthereumInboundQueueV2::submit(
            OriginFor::<Runtime>::signed(AccountId::new([0; 32])),
            Box::new(EventProof {
                event_log: Log {
                    address: <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                    topics: event.encode_topics().into_iter().map(|word| H256::from(word.0.0)).collect(),
                    data: event.encode_data(),
                },
                proof: dummy_proof.clone(),
            }),
        );

        assert!(matches!(result, Err(DispatchErrorWithPostInfo{ error: sp_runtime::DispatchError::Other(_), ..})));
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
        let nonce_val = 1u64;

        let dummy_proof = mock_snowbridge_message_proof();

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

        // let assets = vec![];
        let beneficiary = AccountId::from(FERDIE);

        let instructions = vec![DepositAsset {
            assets: Wild(AllCounted(1)),
            beneficiary: Location::new(
                0,
                AccountId32 {
                    network: None,
                    id: beneficiary.clone().into(),
                },
            ),
        }];

        let xcm: Xcm<()> = instructions.into();
        let versioned_message_xcm = VersionedXcm::V5(xcm);

        let xcm_bytes = RawPayload::Xcm(versioned_message_xcm.encode());
        let token_value = 12345u128;

        let foreign_asset_balance_before = ForeignAssets::balance(asset_id, AccountId::from(beneficiary.clone()));

        let event = IGatewayV2::OutboundMessageAccepted {
            nonce: nonce_val,
            payload: IGatewayV2::Payload {
                origin: Address::from_slice(EthereumGatewayAddress::get().as_bytes()),
                assets: vec![],
                xcm: IGatewayV2::Xcm { kind: 0, data: xcm_bytes.encode().into() },
                claimer: vec![].into(),
                value: token_value,
                executionFee: 0,
                relayerFee: 0,
            },
        };

        assert!(matches!(EthereumInboundQueueV2::submit(OriginFor::<Runtime>::signed(AccountId::new([0; 32])), Box::new(EventProof {
            event_log: Log {
                address: <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                topics: event
                    .encode_topics()
                    .into_iter()
                    .map(|word| H256::from(word.0.0))
                    .collect(),
                data: event.encode_data(),
            },
            proof: dummy_proof.clone(),
        })), Ok(PostDispatchInfo { actual_weight: Some(Weight { .. }), pays_fee: Pays::Yes })));

        let foreign_asset_balance_after = ForeignAssets::balance(asset_id, AccountId::from(beneficiary.clone()));

        assert_eq!(foreign_asset_balance_after - foreign_asset_balance_before, token_value);

        let events = frame_system::Pallet::<Runtime>::events();

        let mut found_message = false;
        let mut found_issued = false;

        for record in events {
            match &record.event {
                RuntimeEvent::EthereumInboundQueueV2(
                    snowbridge_pallet_inbound_queue_v2::Event::MessageReceived { nonce, .. }
                ) if *nonce == nonce_val => {
                    found_message = true;
                }

                RuntimeEvent::ForeignAssets(
                    pallet_assets::Event::Issued { amount, .. }
                ) if *amount == token_value => {
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
        let nonce_val = 1u64;

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

        assert_ok!(
            Balances::force_set_balance(
                root_origin(),
                EthereumSovereignAccount::get().into(),
                10_000_000_000_000_000_000u128
            )
        );

        let token_location_reanchored = token_location
            .clone()
            .reanchored(&EthereumLocation::get(), &UniversalLocation::get())
            .expect("unable to reanchor token");

        let tanssi_token_id =
            snowbridge_pallet_system::NativeToForeignId::<Runtime>::get(&token_location_reanchored).unwrap();

        let tanssi_token_transfer_value = 123_456_789_000u128;

        let beneficiary = AccountId::from(FERDIE);
        let instructions = vec![DepositAsset {
            assets: Wild(AllCounted(1)),
            beneficiary: Location::new(
                0,
                AccountId32 {
                    network: None,
                    id: beneficiary.clone().into(),
                },
            ),
        }];

        let xcm: Xcm<()> = instructions.into();
        let versioned_message_xcm = VersionedXcm::V5(xcm);

        let xcm_bytes = RawPayload::Xcm(versioned_message_xcm.encode());

        let asset = IGatewayV2::AsForeignTokenERC20 {token_id: FixedBytes(tanssi_token_id.into()), value: tanssi_token_transfer_value};
        let event = IGatewayV2::OutboundMessageAccepted {
            nonce: nonce_val,
            payload: IGatewayV2::Payload {
                origin: Address::from_slice(EthereumGatewayAddress::get().as_bytes()),
                assets: vec![IGatewayV2::EthereumAsset {
                    kind: 1,
                    data: asset.abi_encode().into(),
                }],
                xcm: IGatewayV2::Xcm { kind: 0, data: xcm_bytes.encode().into() },
                claimer: vec![].into(),
                value: 0,
                executionFee: 0,
                relayerFee: 0,
            },
        };

        let native_balance_before = System::account(beneficiary.clone()).data.free;
        assert!(matches!(EthereumInboundQueueV2::submit(OriginFor::<Runtime>::signed(AccountId::new([0; 32])), Box::new(EventProof {
            event_log: Log {
                address: <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                topics: event
                    .encode_topics()
                    .into_iter()
                    .map(|word| H256::from(word.0.0))
                    .collect(),
                data: event.encode_data(),
            },
            proof: dummy_proof.clone(),
        })), Ok(PostDispatchInfo { actual_weight: Some(Weight { .. }), pays_fee: Pays::Yes })));
        let native_balance_after = System::account(beneficiary.clone()).data.free;
        assert_eq!(native_balance_after - native_balance_before, tanssi_token_transfer_value);

        let events = frame_system::Pallet::<Runtime>::events();

        let mut found_message = false;
        let mut found_minted = false;

        for record in events {
            match &record.event {
                RuntimeEvent::EthereumInboundQueueV2(
                    snowbridge_pallet_inbound_queue_v2::Event::MessageReceived { nonce, .. }
                ) if *nonce == nonce_val => {
                    found_message = true;
                }

                RuntimeEvent::Balances(
                    pallet_balances::Event::Minted { amount, .. }
                ) if *amount == tanssi_token_transfer_value => {
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
        let nonce_val = 1u64;

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

        let beneficiary = AccountId::from(FERDIE);
        let instructions = vec![DepositAsset {
            assets: Wild(AllCounted(2)),
            beneficiary: Location::new(
                0,
                AccountId32 {
                    network: None,
                    id: beneficiary.clone().into(),
                },
            ),
        }];

        let xcm: Xcm<()> = instructions.into();
        let versioned_message_xcm = VersionedXcm::V5(xcm);

        let xcm_bytes = RawPayload::Xcm(versioned_message_xcm.encode());
        let eth_value = 789_000_000u128;

        let asset = IGatewayV2::AsForeignTokenERC20 {token_id: FixedBytes(tanssi_token_id.into()), value: tanssi_token_transfer_value};
        let event = IGatewayV2::OutboundMessageAccepted {
            nonce: nonce_val,
            payload: IGatewayV2::Payload {
                origin: Address::from_slice(EthereumGatewayAddress::get().as_bytes()),
                assets: vec![IGatewayV2::EthereumAsset {
                    kind: 1,
                    data: asset.abi_encode().into(),
                }],
                xcm: IGatewayV2::Xcm { kind: 0, data: xcm_bytes.encode().into() },
                claimer: vec![].into(),
                value: eth_value,
                executionFee: 0,
                relayerFee: 0,
            },
        };

        let native_balance_before = System::account(beneficiary.clone()).data.free;
        let foreign_asset_balance_before = ForeignAssets::balance(asset_id, AccountId::from(beneficiary.clone()));

        assert!(matches!(EthereumInboundQueueV2::submit(OriginFor::<Runtime>::signed(AccountId::new([0; 32])), Box::new(EventProof {
            event_log: Log {
                address: <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                topics: event
                    .encode_topics()
                    .into_iter()
                    .map(|word| H256::from(word.0.0))
                    .collect(),
                data: event.encode_data(),
            },
            proof: dummy_proof.clone(),
        })), Ok(PostDispatchInfo { actual_weight: Some(Weight { .. }), pays_fee: Pays::Yes })));

        let native_balance_after = System::account(beneficiary.clone()).data.free;
        let foreign_asset_balance_after = ForeignAssets::balance(asset_id, AccountId::from(beneficiary.clone()));

        assert_eq!(foreign_asset_balance_after - foreign_asset_balance_before, eth_value);
        assert_eq!(native_balance_after - native_balance_before, tanssi_token_transfer_value);

        let events = frame_system::Pallet::<Runtime>::events();

        let mut found_message = false;
        let mut found_minted = false;
        let mut found_issued = false;

        for record in events {
            match &record.event {
                RuntimeEvent::EthereumInboundQueueV2(
                    snowbridge_pallet_inbound_queue_v2::Event::MessageReceived { nonce, .. }
                ) if *nonce == nonce_val => {
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
        let nonce_val = 1u64;

        let dummy_proof = mock_snowbridge_message_proof();
        let erc20_token_id = hex_literal::hex!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");

        let erc20_asset_location = Location {
            parents: 1,
            interior: X2([
                GlobalConsensus(Ethereum { chain_id: ETH_CHAIN_ID }),
                AccountKey20 {
                    network: Some(Ethereum { chain_id: ETH_CHAIN_ID }),
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

        let beneficiary = AccountId::from(FERDIE);
        let instructions = vec![DepositAsset {
            assets: Wild(AllCounted(1)),
            beneficiary: Location::new(
                0,
                AccountId32 {
                    network: None,
                    id: beneficiary.clone().into(),
                },
            ),
        }];

        let xcm: Xcm<()> = instructions.into();
        let versioned_message_xcm = VersionedXcm::V5(xcm);

        let xcm_bytes = RawPayload::Xcm(versioned_message_xcm.encode());

        let asset = IGatewayV2::AsNativeTokenERC20 {token_id: Address::from_slice(&erc20_token_id), value: token_value};
        let event = IGatewayV2::OutboundMessageAccepted {
            nonce: nonce_val,
            payload: IGatewayV2::Payload {
                origin: Address::from_slice(EthereumGatewayAddress::get().as_bytes()),
                assets: vec![IGatewayV2::EthereumAsset {
                    kind: 0,
                    data: asset.abi_encode().into(),
                }],
                xcm: IGatewayV2::Xcm { kind: 0, data: xcm_bytes.encode().into() },
                claimer: vec![].into(),
                value: 0,
                executionFee: 0,
                relayerFee: 0,
            },
        };

        let foreign_asset_balance_before = ForeignAssets::balance(asset_id, AccountId::from(beneficiary.clone()));
        assert!(matches!(EthereumInboundQueueV2::submit(OriginFor::<Runtime>::signed(AccountId::new([0; 32])), Box::new(EventProof {
            event_log: Log {
                address: <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                topics: event
                    .encode_topics()
                    .into_iter()
                    .map(|word| H256::from(word.0.0))
                    .collect(),
                data: event.encode_data(),
            },
            proof: dummy_proof.clone(),
        })), Ok(PostDispatchInfo { actual_weight: Some(Weight { .. }), pays_fee: Pays::Yes })));
        let foreign_asset_balance_after = ForeignAssets::balance(asset_id, AccountId::from(beneficiary.clone()));
        assert_eq!(foreign_asset_balance_after - foreign_asset_balance_before, token_value);

        let events = frame_system::Pallet::<Runtime>::events();

        let mut found_message = false;
        let mut found_issued = false;

        for record in events {
            match &record.event {
                RuntimeEvent::EthereumInboundQueueV2(
                    snowbridge_pallet_inbound_queue_v2::Event::MessageReceived { nonce, .. }
                ) if *nonce == nonce_val => {
                    found_message = true;
                }

                RuntimeEvent::ForeignAssets(
                    pallet_assets::Event::Issued { amount, .. }
                ) if *amount == token_value => {
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
fn test_inbound_queue_transfer_erc20_as_msg_sender_works() {
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
        let nonce_val = 1u64;

        let dummy_proof = mock_snowbridge_message_proof();
        let erc20_token_id = hex_literal::hex!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");

        let erc20_asset_location = Location {
            parents: 1,
            interior: X2([
                GlobalConsensus(Ethereum { chain_id: ETH_CHAIN_ID }),
                AccountKey20 {
                    network: Some(Ethereum { chain_id: ETH_CHAIN_ID }),
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

        let beneficiary = AccountId::from(FERDIE);
        let instructions = vec![DepositAsset {
            assets: Wild(AllCounted(1)),
            beneficiary: Location::new(
                0,
                AccountId32 {
                    network: None,
                    id: beneficiary.clone().into(),
                },
            ),
        }];

        let xcm: Xcm<()> = instructions.into();
        let versioned_message_xcm = VersionedXcm::V5(xcm);

        let xcm_bytes = RawPayload::Xcm(versioned_message_xcm.encode());

        let asset = IGatewayV2::AsNativeTokenERC20 {token_id: Address::from(erc20_token_id), value: token_value};
        let event = IGatewayV2::OutboundMessageAccepted {
            nonce: nonce_val,
            payload: IGatewayV2::Payload {
                // TODO: how does this account pay for fees?
                origin: Address::from([0x11; 20]),
                assets: vec![IGatewayV2::EthereumAsset {
                    kind: 0,
                    data: asset.abi_encode().into(),
                }],
                xcm: IGatewayV2::Xcm { kind: 0, data: xcm_bytes.encode().into() },
                claimer: vec![].into(),
                value: 0,
                executionFee: 0,
                relayerFee: 0,
            },
        };

        let foreign_asset_balance_before = ForeignAssets::balance(asset_id, AccountId::from(beneficiary.clone()));
        assert!(matches!(EthereumInboundQueueV2::submit(OriginFor::<Runtime>::signed(AccountId::new([0; 32])), Box::new(EventProof {
            event_log: Log {
                address: <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                topics: event
                    .encode_topics()
                    .into_iter()
                    .map(|word| H256::from(word.0.0))
                    .collect(),
                data: event.encode_data(),
            },
            proof: dummy_proof.clone(),
        })), Ok(PostDispatchInfo { actual_weight: Some(Weight { .. }), pays_fee: Pays::Yes })));
        let foreign_asset_balance_after = ForeignAssets::balance(asset_id, AccountId::from(beneficiary.clone()));
        assert_eq!(foreign_asset_balance_after - foreign_asset_balance_before, token_value);

        let events = frame_system::Pallet::<Runtime>::events();

        let mut found_message = false;
        let mut found_issued = false;

        for record in events {
            match &record.event {
                RuntimeEvent::EthereumInboundQueueV2(
                    snowbridge_pallet_inbound_queue_v2::Event::MessageReceived { nonce, .. }
                ) if *nonce == nonce_val => {
                    found_message = true;
                }

                RuntimeEvent::ForeignAssets(
                    pallet_assets::Event::Issued { amount, .. }
                ) if *amount == token_value => {
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
fn test_inbound_queue_xcm_transact_system_remark_as_msg_sender_works() {
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
        let nonce_val = 1u64;

        let dummy_proof = mock_snowbridge_message_proof();
        let erc20_token_id = hex_literal::hex!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");

        let erc20_asset_location = Location {
            parents: 1,
            interior: X2([
                GlobalConsensus(Ethereum { chain_id: ETH_CHAIN_ID }),
                AccountKey20 {
                    network: Some(Ethereum { chain_id: ETH_CHAIN_ID }),
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

        let beneficiary = AccountId::from(FERDIE);
        let transact_call = RuntimeCall::System(CallableCallFor::<System, Runtime>::remark_with_event { remark: b"hello from ethereum?".to_vec() });
        let instructions = vec![Transact {
            origin_kind: OriginKind::SovereignAccount,
            fallback_max_weight: None,
            call: transact_call.encode().into(),
        }];

        let xcm: Xcm<()> = instructions.into();
        let versioned_message_xcm = VersionedXcm::V5(xcm);

        let xcm_bytes = RawPayload::Xcm(versioned_message_xcm.encode());

        let asset = IGatewayV2::AsNativeTokenERC20 {token_id: Address::from(erc20_token_id), value: token_value};
        let old_assets = vec![IGatewayV2::EthereumAsset {
            kind: 0,
            data: asset.abi_encode().into(),
        }];
        // Event with no assets and 0 value, shouldn't be able to call transact
        // origin is msg.sender from ethereum, doesn't have any tokens in tanssi
        let event = IGatewayV2::OutboundMessageAccepted {
            nonce: nonce_val,
            payload: IGatewayV2::Payload {
                origin: Address::from([0x11; 20]),
                assets: vec![],
                xcm: IGatewayV2::Xcm { kind: 0, data: xcm_bytes.encode().into() },
                claimer: vec![].into(),
                value: 0,
                executionFee: 0,
                relayerFee: 0,
            },
        };

        let foreign_asset_balance_before = ForeignAssets::balance(asset_id, AccountId::from(beneficiary.clone()));
        // relayer can be anything
        let relayer = AccountId::new([0xaa; 32]);
        let relayer_balance_before = System::account(relayer.clone()).data.free;
        let call = RuntimeCall::EthereumInboundQueueV2(CallableCallFor::<EthereumInboundQueueV2, Runtime>::submit { event: Box::new(EventProof {
            event_log: Log {
                address: <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                topics: event
                    .encode_topics()
                    .into_iter()
                    .map(|word| H256::from(word.0.0))
                    .collect(),
                data: event.encode_data(),
            },
            proof: dummy_proof.clone(),
        })});
        let info = call.get_dispatch_info();
        assert!(info.call_weight.ref_time() > 0, "weight is zero");
        let post = call.dispatch(
            <Runtime as frame_system::Config>::RuntimeOrigin::signed(relayer.clone())
        );
        assert_ok!(post);
        let used = post.unwrap().actual_weight.unwrap_or(info.call_weight);
        assert!(used.ref_time() > 0, "actual weight is zero");
        println!("used weight: {:?}", used);
        let events = frame_system::Pallet::<Runtime>::events();

        println!("{:?}", events);

        let relayer_balance_after = System::account(relayer.clone()).data.free;

        println!("relayer_balance_before: {}", relayer_balance_before);
        println!("relayer_balance_after:  {}", relayer_balance_after);

        let mut found_message = false;
        let mut found_remark = false;

        for record in events {
            match &record.event {
                RuntimeEvent::EthereumInboundQueueV2(
                    snowbridge_pallet_inbound_queue_v2::Event::MessageReceived { nonce, .. }
                ) if *nonce == nonce_val => {
                    found_message = true;
                }

                RuntimeEvent::System(
                    frame_system::Event::Remarked { sender, hash }
                ) => {
                    found_remark = true;
                    // TODO: how does this account pay for fees?
                    // We see the system remarked event so it worked
                    // Remarked! 9075bd281ac2116eb06d6f6cecde02b437306b90e377900e4377272f96e90fae (5FL7jCJc...) 0xda45538cacc877e1393e1cabf1bbef93f23945cfa2b4f881b20ff53a964ebbb6
                    println!("Remarked! {:?} {:?}", sender, hash);
                }

                _ => {}
            }
        }

        assert!(found_message, "MessageReceived event not found");
        assert!(found_remark, "Remarked event for ETH not found");
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
        let nonce_val = 1u64;

        let dummy_proof = mock_snowbridge_message_proof();
        let erc20_token_id = hex_literal::hex!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");

        let erc20_asset_location = Location {
            parents: 1,
            interior: X2([
                GlobalConsensus(Ethereum { chain_id: ETH_CHAIN_ID }),
                AccountKey20 {
                    network: Some(Ethereum { chain_id: ETH_CHAIN_ID }),
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

        let instructions = vec![];

        let xcm: Xcm<()> = instructions.into();
        let versioned_message_xcm = VersionedXcm::V5(xcm);

        let xcm_bytes = RawPayload::Xcm(versioned_message_xcm.encode());

        let asset = IGatewayV2::AsNativeTokenERC20 {token_id: Address::from_slice(&erc20_token_id), value: token_value};
        let event = IGatewayV2::OutboundMessageAccepted {
            nonce: nonce_val,
            payload: IGatewayV2::Payload {
                origin: Address::from_slice(EthereumGatewayAddress::get().as_bytes()),
                assets: vec![IGatewayV2::EthereumAsset {
                    kind: 0,
                    data: asset.abi_encode().into(),
                }],
                xcm: IGatewayV2::Xcm { kind: 0, data: xcm_bytes.encode().into() },
                claimer: vec![].into(),
                value: 0,
                executionFee: 0,
                relayerFee: 0,
            },
        };

        assert!(matches!(EthereumInboundQueueV2::submit(OriginFor::<Runtime>::signed(AccountId::new([0; 32])), Box::new(EventProof {
            event_log: Log {
                address: <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                topics: event
                    .encode_topics()
                    .into_iter()
                    .map(|word| H256::from(word.0.0))
                    .collect(),
                data: event.encode_data(),
            },
            proof: dummy_proof.clone(),
        })), Ok(PostDispatchInfo { actual_weight: Some(Weight { .. }), pays_fee: Pays::Yes })));

        let events = frame_system::Pallet::<Runtime>::events();

        let mut found_message = false;
        let mut found_trapped = false;

        for record in events {
            match &record.event {
                RuntimeEvent::EthereumInboundQueueV2(
                    snowbridge_pallet_inbound_queue_v2::Event::MessageReceived { nonce, .. }
                ) if *nonce == nonce_val => {
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
        let nonce_val = 1u64;

        let dummy_proof = mock_snowbridge_message_proof();
        let erc20_token_id = hex_literal::hex!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");

        let erc20_asset_location = Location {
            parents: 1,
            interior: X2([
                GlobalConsensus(Ethereum { chain_id: ETH_CHAIN_ID }),
                AccountKey20 {
                    network: Some(Ethereum { chain_id: ETH_CHAIN_ID }),
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

        let instructions = vec![];

        let xcm: Xcm<()> = instructions.into();
        let versioned_message_xcm = VersionedXcm::V5(xcm);

        let xcm_bytes = RawPayload::Xcm(versioned_message_xcm.encode());

        let eth_amount = 789_000_000u128;

        let event = IGatewayV2::OutboundMessageAccepted {
            nonce: nonce_val,
            payload: IGatewayV2::Payload {
                origin: Address::from_slice(EthereumGatewayAddress::get().as_bytes()),
                assets: vec![],
                xcm: IGatewayV2::Xcm { kind: 0, data: xcm_bytes.encode().into() },
                claimer: vec![].into(),
                value: eth_amount,
                executionFee: 0,
                relayerFee: 0,
            },
        };

        assert!(matches!(EthereumInboundQueueV2::submit(OriginFor::<Runtime>::signed(AccountId::new([0; 32])), Box::new(EventProof {
            event_log: Log {
                address: <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                topics: event
                    .encode_topics()
                    .into_iter()
                    .map(|word| H256::from(word.0.0))
                    .collect(),
                data: event.encode_data(),
            },
            proof: dummy_proof.clone(),
        })), Ok(PostDispatchInfo { actual_weight: Some(Weight { .. }), pays_fee: Pays::Yes })));

        let events = frame_system::Pallet::<Runtime>::events();

        let mut found_message = false;
        let mut found_trapped = false;

        for record in events {
            match &record.event {
                RuntimeEvent::EthereumInboundQueueV2(
                    snowbridge_pallet_inbound_queue_v2::Event::MessageReceived { nonce, .. }
                ) if *nonce == nonce_val => {
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
        let nonce_val = 1u64;

        let dummy_proof = mock_snowbridge_message_proof();
        let erc20_token_id = hex_literal::hex!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");

        let erc20_asset_location = Location {
            parents: 1,
            interior: X2([
                GlobalConsensus(Ethereum { chain_id: ETH_CHAIN_ID }),
                AccountKey20 {
                    network: Some(Ethereum { chain_id: ETH_CHAIN_ID }),
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

        let instructions = vec![
            // Just random incorrect instruction
            WithdrawAsset(Assets::new())
        ];

        let xcm: Xcm<()> = instructions.into();
        let versioned_message_xcm = VersionedXcm::V5(xcm);

        let xcm_bytes = RawPayload::Xcm(versioned_message_xcm.encode());

        let eth_amount = 789_000_000u128;

        let event = IGatewayV2::OutboundMessageAccepted {
            nonce: nonce_val,
            payload: IGatewayV2::Payload {
                origin: Address::from_slice(EthereumGatewayAddress::get().as_bytes()),
                assets: vec![],
                xcm: IGatewayV2::Xcm { kind: 0, data: xcm_bytes.encode().into() },
                claimer: vec![].into(),
                value: eth_amount,
                executionFee: 0,
                relayerFee: 0,
            },
        };

        assert!(matches!(EthereumInboundQueueV2::submit(OriginFor::<Runtime>::signed(AccountId::new([0; 32])), Box::new(EventProof {
            event_log: Log {
                address: <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                topics: event
                    .encode_topics()
                    .into_iter()
                    .map(|word| H256::from(word.0.0))
                    .collect(),
                data: event.encode_data(),
            },
            proof: dummy_proof.clone(),
        })), Ok(PostDispatchInfo { actual_weight: Some(Weight { .. }), pays_fee: Pays::Yes })));

        let events = frame_system::Pallet::<Runtime>::events();

        let mut found_message = false;
        let mut found_trapped = false;

        for record in events {
            match &record.event {
                RuntimeEvent::EthereumInboundQueueV2(
                    snowbridge_pallet_inbound_queue_v2::Event::MessageReceived { nonce, .. }
                ) if *nonce == nonce_val => {
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
