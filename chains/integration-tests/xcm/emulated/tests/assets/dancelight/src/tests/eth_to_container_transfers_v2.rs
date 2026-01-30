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

//! V2 Inbound Queue tests for token transfers from Ethereum to container chains.
//!
//! These tests use the V2 message format with `IGatewayV2::OutboundMessageAccepted`
//! and `RawPayload::Xcm` for the XCM payload encoding.
//!
//! ## Test Scenarios
//!
//! The tests cover four types of token transfers from Ethereum to container chains:
//!
//! 1. **ERC20 tokens** (kind: 0) - Native Ethereum ERC20 tokens transferred to containers
//!    - `check_erc20_token_to_frontier_container_via_v2_works`
//!    - `check_erc20_token_to_simple_container_via_v2_works`
//!
//! 2. **Native ETH** (via `value` field) - Ether transferred to containers as a foreign asset
//!    - `check_eth_to_frontier_container_via_v2_works`
//!    - `check_eth_to_simple_container_via_v2_works`
//!
//! 3. **Tanssi (relay) tokens** (kind: 1) - Relay chain tokens previously bridged to Ethereum,
//!    now being sent back to container chains
//!    - `check_tanssi_to_frontier_container_via_v2_works`
//!    - `check_tanssi_to_simple_container_via_v2_works`
//!
//! 4. **Container native tokens** (kind: 1) - Container chain native tokens previously bridged
//!    to Ethereum, now being sent back to their origin chain
//!    - `check_container_native_to_frontier_container_via_v2_works`
//!    - `check_container_native_to_simple_container_via_v2_works`
//!
//! Each scenario is tested against both container chain templates:
//! - **Frontier** (EVM-compatible, uses `AccountId20`)
//! - **Simple** (Substrate-native, uses `AccountId32`)

use {
    alloy_core::{
        primitives::{Address, FixedBytes},
        sol_types::{SolEvent, SolValue},
    },
    dancelight_emulated_chain::DancelightRelayPallet,
    dancelight_runtime::{
        bridge_to_ethereum_config::EthereumGatewayAddress, xcm_config::UniversalLocation,
        EthereumSovereignAccount,
    },
    dancelight_runtime_constants::snowbridge::EthereumLocation,
    dancelight_system_emulated_network::{
        DancelightRelay as Dancelight, DancelightSender, FrontierTemplatePara as FrontierTemplate,
        FrontierTemplateSender, SimpleTemplatePara as SimpleTemplate, SimpleTemplateSender,
    },
    fp_account::AccountId20,
    frame_support::{
        assert_ok, dispatch::DispatchResultWithPostInfo, pallet_prelude::PalletInfoAccess,
        traits::fungible::Mutate,
    },
    frontier_template_emulated_chain::FrontierTemplateParaPallet,
    hex_literal::hex,
    parity_scale_codec::Encode,
    simple_template_emulated_chain::SimpleTemplateParaPallet,
    snowbridge_beacon_primitives::{
        types::deneb, AncestryProof, BeaconHeader, ExecutionProof, VersionedExecutionPayloadHeader,
    },
    snowbridge_inbound_queue_primitives::v2::message::IGatewayV2,
    snowbridge_pallet_system,
    snowbridge_verification_primitives::{EventProof, Log, Proof},
    sp_core::{H160, H256, U256},
    sp_runtime::FixedU128,
    tanssi_runtime_common::processors::v2::RawPayload,
    xcm::{
        latest::prelude::{Junctions::*, *},
        opaque::latest::AssetTransferFilter,
        v5::Xcm,
        VersionedXcm,
    },
    xcm_emulator::{assert_expected_events, Chain, ConvertLocation, TestExt},
};

const RELAY_NATIVE_TOKEN_ASSET_ID: u16 = 42;
const ERC20_ASSET_ID: u16 = 24;
const TRANSFER_AMOUNT: u128 = 100_000_000;
const TOKEN_ADDRESS: H160 = H160::repeat_byte(0x11);
const RELAY_TOKEN_ASSET_LOCATION: Location = Location::parent();
/// Fee amount in Tanssi native token for XCM execution
const TANSSI_FEE_AMOUNT: u128 = 10_000_000_000_000;

/// Creates a V2 event proof structure for the inbound queue
fn make_v2_event_proof(event: &IGatewayV2::OutboundMessageAccepted) -> EventProof {
    EventProof {
        event_log: Log {
            address: hex!("eda338e4dc46038493b885327842fd3e301cab39").into(),
            topics: event
                .encode_topics()
                .into_iter()
                .map(|word| H256::from(word.0 .0))
                .collect(),
            data: event.encode_data(),
        },
        proof: make_mock_proof(),
    }
}

/// Helper to create a mock proof (same structure as V1 tests, but content doesn't matter since verifier is mocked)
fn make_mock_proof() -> Proof {
    Proof {
        receipt_proof: (
            vec![
                hex!("dccdfceea05036f7b61dcdabadc937945d31e68a8d3dfd4dc85684457988c284")
                    .to_vec(),
                hex!("4a98e45a319168b0fc6005ce6b744ee9bf54338e2c0784b976a8578d241ced0f")
                    .to_vec(),
            ],
            vec![
                hex!("f851a09c01dd6d2d8de951c45af23d3ad00829ce021c04d6c8acbe1612d456ee320d4980808080808080a04a98e45a319168b0fc6005ce6b744ee9bf54338e2c0784b976a8578d241ced0f8080808080808080").to_vec(),
                hex!("f9028c30b9028802f90284018301d205b9010000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000080000000000000000000000000000004000000000080000000000000000000000000000000000010100000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000040004000000000000002000002000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000200000000000010f90179f85894eda338e4dc46038493b885327842fd3e301cab39e1a0f78bb28d4b1d7da699e5c0bc2be29c2b04b5aab6aacf6298fe5304f9db9c6d7ea000000000000000000000000087d1f7fdfee7f651fabc8bfcb6e086c278b77a7df9011c94eda338e4dc46038493b885327842fd3e301cab39f863a07153f9357c8ea496bba60bf82e67143e27b64462b49041f8e689e1b05728f84fa0c173fac324158e77fb5840738a1a541f633cbec8884c6a601c567d2b376a0539a05f7060e971b0dc81e63f0aa41831091847d97c1a4693ac450cc128c7214e65e0b8a000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000002e00a736aa00000000000087d1f7fdfee7f651fabc8bfcb6e086c278b77a7d00e40b54020000000000000000000000000000000000000000000000000000000000").to_vec(),
            ],
        ),
        execution_proof: ExecutionProof {
            header: BeaconHeader {
                slot: 393,
                proposer_index: 4,
                parent_root: hex!(
                    "6545b47a614a1dd4cad042a0cdbbf5be347e8ffcdc02c6c64540d5153acebeef"
                )
                .into(),
                state_root: hex!(
                    "b62ac34a8cb82497be9542fe2114410c9f6021855b766015406101a1f3d86434"
                )
                .into(),
                body_root: hex!(
                    "04005fe231e11a5b7b1580cb73b177ae8b338bedd745497e6bb7122126a806db"
                )
                .into(),
            },
            ancestry_proof: Some(AncestryProof {
                header_branch: vec![
                    hex!("6545b47a614a1dd4cad042a0cdbbf5be347e8ffcdc02c6c64540d5153acebeef")
                        .into(),
                    hex!("fa84cc88ca53a72181599ff4eb07d8b444bce023fe2347c3b4f51004c43439d3")
                        .into(),
                    hex!("cadc8ae211c6f2221c9138e829249adf902419c78eb4727a150baa4d9a02cc9d")
                        .into(),
                    hex!("33a89962df08a35c52bd7e1d887cd71fa7803e68787d05c714036f6edf75947c")
                        .into(),
                    hex!("2c9760fce5c2829ef3f25595a703c21eb22d0186ce223295556ed5da663a82cf")
                        .into(),
                    hex!("e1aa87654db79c8a0ecd6c89726bb662fcb1684badaef5cd5256f479e3c622e1")
                        .into(),
                    hex!("aa70d5f314e4a1fbb9c362f3db79b21bf68b328887248651fbd29fc501d0ca97")
                        .into(),
                    hex!("160b6c235b3a1ed4ef5f80b03ee1c76f7bf3f591c92fca9d8663e9221b9f9f0f")
                        .into(),
                    hex!("f68d7dcd6a07a18e9de7b5d2aa1980eb962e11d7dcb584c96e81a7635c8d2535")
                        .into(),
                    hex!("1d5f912dfd6697110dd1ecb5cb8e77952eef57d85deb373572572df62bb157fc")
                        .into(),
                    hex!("ffff0ad7e659772f9534c195c815efc4014ef1e1daed4404c06385d11192e92b")
                        .into(),
                    hex!("6cf04127db05441cd833107a52be852868890e4317e6a02ab47683aa75964220")
                        .into(),
                    hex!("b7d05f875f140027ef5118a2247bbb84ce8f2f0f1123623085daf7960c329f5f")
                        .into(),
                ],
                finalized_block_root: hex!(
                    "751414cd97c0624f922b3e80285e9f776b08fa22fd5f87391f2ed7ef571a8d46"
                )
                .into(),
            }),
            execution_header: VersionedExecutionPayloadHeader::Deneb(
                deneb::ExecutionPayloadHeader {
                    parent_hash: hex!(
                        "8092290aa21b7751576440f77edd02a94058429ce50e63a92d620951fb25eda2"
                    )
                    .into(),
                    fee_recipient: hex!("0000000000000000000000000000000000000000").into(),
                    state_root: hex!(
                        "96a83e9ddf745346fafcb0b03d57314623df669ed543c110662b21302a0fae8b"
                    )
                    .into(),
                    receipts_root: hex!(
                        "dccdfceea05036f7b61dcdabadc937945d31e68a8d3dfd4dc85684457988c284"
                    )
                    .into(),
                    logs_bloom: hex!("00000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000080000000400000000000000000000004000000000080000000000000000000000000000000000010100000000000000000000000000000000020000000000000000000000000000000000080000000000000000000000000000040004000000000000002002002000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000080000000000000000000000000000000000100000000000000000200000200000010").into(),
                    prev_randao: hex!(
                        "62e309d4f5119d1f5c783abc20fc1a549efbab546d8d0b25ff1cfd58be524e67"
                    )
                    .into(),
                    block_number: 393,
                    gas_limit: 54492273,
                    gas_used: 199644,
                    timestamp: 1710552813,
                    extra_data: hex!("d983010d0b846765746888676f312e32312e368664617277696e").into(),
                    base_fee_per_gas: U256::from(7u64),
                    block_hash: hex!(
                        "6a9810efb9581d30c1a5c9074f27c68ea779a8c1ae31c213241df16225f4e131"
                    )
                    .into(),
                    transactions_root: hex!(
                        "2cfa6ed7327e8807c7973516c5c32a68ef2459e586e8067e113d081c3bd8c07d"
                    )
                    .into(),
                    withdrawals_root: hex!(
                        "792930bbd5baac43bcc798ee49aa8185ef76bb3b44ba62b91d86ae569e4bb535"
                    )
                    .into(),
                    blob_gas_used: 0,
                    excess_blob_gas: 0,
                },
            ),
            execution_branch: vec![
                hex!("a6833fa629f3286b6916c6e50b8bf089fc9126bee6f64d0413b4e59c1265834d").into(),
                hex!("b46f0c01805fe212e15907981b757e6c496b0cb06664224655613dcec82505bb").into(),
                hex!("db56114e00fdd4c1f85c892bf35ac9a89289aaecb1ebd0a96cde606a748b5d71").into(),
                hex!("d3af7c05c516726be7505239e0b9c7cb53d24abce6b91cdb3b3995f0164a75da").into(),
            ],
        },
    }
}

/// Returns the finalized header for the mock proof
fn get_finalized_header() -> BeaconHeader {
    BeaconHeader {
        slot: 864,
        proposer_index: 4,
        parent_root: hex!("614e7672f991ac268cd841055973f55e1e42228831a211adef207bb7329be614")
            .into(),
        state_root: hex!("5fa8dfca3d760e4242ab46d529144627aa85348a19173b6e081172c701197a4a").into(),
        body_root: hex!("0f34c083b1803666bb1ac5e73fa71582731a2cf37d279ff0a3b0cad5a2ff371e").into(),
    }
}

/// Returns the block roots root for the mock proof
fn get_block_roots_root() -> H256 {
    hex!("b9aab9c388c4e4fcd899b71f62c498fc73406e38e8eb14aa440e9affa06f2a10").into()
}

/// Submits a V2 inbound message to the Dancelight relay chain
pub fn send_inbound_message_v2(
    event: IGatewayV2::OutboundMessageAccepted,
) -> DispatchResultWithPostInfo {
    let finalized_header = get_finalized_header();
    let block_roots_root = get_block_roots_root();

    dancelight_runtime::EthereumBeaconClient::store_finalized_header(
        finalized_header,
        block_roots_root,
    )
    .unwrap();

    let event_proof = make_v2_event_proof(&event);

    <Dancelight as DancelightRelayPallet>::EthereumInboundQueueV2::submit(
        <Dancelight as Chain>::RuntimeOrigin::signed(DancelightSender::get()),
        Box::new(event_proof),
    )
}

/// Test: ERC20 token transfer from Ethereum to Frontier container chain via V2 inbound queue
#[test]
fn check_erc20_token_to_frontier_container_via_v2_works() {
    sp_tracing::try_init_simple();

    let token_receiver: AccountId20 = [5u8; 20].into();
    let container_para_id: u32 = <FrontierTemplate as xcm_emulator::Parachain>::para_id().into();

    let mut receiver_erc20_balance_before = 0;
    let mut dest_fee_amount = 0;

    let container_location = Location::new(0, Parachain(container_para_id));
    let container_sovereign_account =
        dancelight_runtime::xcm_config::LocationConverter::convert_location(&container_location)
            .unwrap();

    let ethereum_network_id = FrontierTemplate::execute_with(|| {
        container_chain_template_frontier_runtime::EthereumNetwork::get()
    });

    // Store the Tanssi token ID for use in the V2 message
    let tanssi_token_id: [u8; 32] = Dancelight::execute_with(|| {
        let root_origin = <Dancelight as Chain>::RuntimeOrigin::root();
        let alice_account = DancelightSender::get();

        // Register Tanssi native token in EthereumSystemV2 (for fees)
        let token_location = Location::here();
        assert_ok!(
            <Dancelight as DancelightRelayPallet>::EthereumSystemV2::register_token(
                root_origin.clone(),
                Box::new(token_location.clone().into()),
                Box::new(token_location.clone().into()),
                snowbridge_core::AssetMetadata {
                    name: "relay".as_bytes().to_vec().try_into().unwrap(),
                    symbol: "relay".as_bytes().to_vec().try_into().unwrap(),
                    decimals: 12,
                },
                1
            )
        );

        // Fund Ethereum sovereign account for token withdrawals
        assert_ok!(
            <<Dancelight as DancelightRelayPallet>::Balances as Mutate<_>>::mint_into(
                &EthereumSovereignAccount::get(),
                10_000_000_000_000_000_000u128
            )
        );

        // Get Tanssi token ID for the V2 message
        let token_location_reanchored = token_location
            .clone()
            .reanchored(&EthereumLocation::get(), &UniversalLocation::get())
            .expect("unable to reanchor token");

        let tanssi_token_id: [u8; 32] = snowbridge_pallet_system::NativeToForeignId::<
            dancelight_runtime::Runtime,
        >::get(&token_location_reanchored)
        .unwrap()
        .into();

        // Register ERC20 foreign token in ForeignAssetsCreator
        let erc20_asset_location_relay: Location = Location {
            parents: 1,
            interior: X2([
                GlobalConsensus(ethereum_network_id),
                AccountKey20 {
                    network: Some(ethereum_network_id),
                    key: TOKEN_ADDRESS.into(),
                },
            ]
            .into()),
        };

        assert_ok!(
            <Dancelight as DancelightRelayPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                erc20_asset_location_relay,
                ERC20_ASSET_ID,
                alice_account,
                true,
                1
            )
        );

        tanssi_token_id
    });

    // Setup on FrontierTemplate container
    FrontierTemplate::execute_with(|| {
        let root_origin = <FrontierTemplate as Chain>::RuntimeOrigin::root();
        let alice_account = FrontierTemplateSender::get();

        receiver_erc20_balance_before =
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssets::balance(
                ERC20_ASSET_ID,
                &token_receiver,
            );

        assert_ok!(
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                RELAY_TOKEN_ASSET_LOCATION,
                RELAY_NATIVE_TOKEN_ASSET_ID,
                alice_account.clone().into(),
                true,
                1
            )
        );

        // Create asset rate for relay token
        assert_ok!(
            <FrontierTemplate as FrontierTemplateParaPallet>::AssetRate::create(
                root_origin.clone(),
                Box::new(RELAY_NATIVE_TOKEN_ASSET_ID),
                FixedU128::from_u32(500_000_000)
            )
        );

        // Register ERC20 foreign token in ForeignAssetsCreator
        let erc20_asset_location_container: Location = Location {
            parents: 2,
            interior: X2([
                GlobalConsensus(ethereum_network_id),
                AccountKey20 {
                    network: Some(ethereum_network_id),
                    key: TOKEN_ADDRESS.into(),
                },
            ]
            .into()),
        };

        assert_ok!(
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                erc20_asset_location_container,
                ERC20_ASSET_ID,
                alice_account.into(),
                true,
                1
            )
        );
    });

    // Send V2 inbound message
    Dancelight::execute_with(|| {
        // Build V2 asset encoding for ERC20
        let erc20_asset = IGatewayV2::AsNativeTokenERC20 {
            token_id: Address::from_slice(TOKEN_ADDRESS.as_bytes()),
            value: TRANSFER_AMOUNT,
        };

        // Build Tanssi native token asset for fees (kind: 1 = foreign token registered via EthereumSystemV2)
        let tanssi_fee_asset = IGatewayV2::AsForeignTokenERC20 {
            token_id: FixedBytes(tanssi_token_id),
            value: TANSSI_FEE_AMOUNT,
        };

        // We send the half of the TANSSI_FEE_AMOUNT and leave the rest to pay for delivery fees in Tanssi.
        dest_fee_amount = 5_000_000_000_000;
        let fee_asset =
            AssetId(Location::here()).into_asset(Fungibility::Fungible(dest_fee_amount));
        let erc20_asset_location_relay: Location = Location {
            parents: 1,
            interior: X2([
                GlobalConsensus(ethereum_network_id),
                AccountKey20 {
                    network: Some(ethereum_network_id),
                    key: TOKEN_ADDRESS.into(),
                },
            ]
            .into()),
        };
        let erc20_asset_for_transfer =
            AssetId(erc20_asset_location_relay).into_asset(Fungibility::Fungible(TRANSFER_AMOUNT));

        let custom_xcm: Vec<Instruction<()>> = vec![
            // Transfer to parachain
            InitiateTransfer {
                destination: Location::new(0, Parachain(container_para_id)),
                remote_fees: Some(AssetTransferFilter::ReserveDeposit(
                    fee_asset.clone().into(),
                )),
                // We clear the origin in destination.
                preserve_origin: false,
                assets: sp_runtime::BoundedVec::truncate_from(vec![
                    AssetTransferFilter::ReserveDeposit(Definite(
                        erc20_asset_for_transfer.clone().into(),
                    )),
                ]),
                remote_xcm: Xcm(vec![
                    RefundSurplus,
                    DepositAsset {
                        assets: Wild(AllCounted(2)),
                        beneficiary: Location::new(
                            0,
                            AccountKey20 {
                                network: None,
                                key: token_receiver.into(),
                            },
                        ),
                    },
                ]),
            },
        ];

        let versioned_destination_xcm: VersionedXcm<()> = VersionedXcm::V5(Xcm(custom_xcm));
        let destination_xcm_bytes = RawPayload::Xcm(versioned_destination_xcm.encode());

        let gateway_address = EthereumGatewayAddress::get();

        let event = IGatewayV2::OutboundMessageAccepted {
            nonce: 1u64,
            payload: IGatewayV2::Payload {
                origin: Address::from_slice(&gateway_address.as_bytes()),
                assets: vec![
                    // Tanssi native token for fees (kind: 1)
                    IGatewayV2::EthereumAsset {
                        kind: 1,
                        data: tanssi_fee_asset.abi_encode().into(),
                    },
                    // ERC20 token for transfer (kind: 0)
                    IGatewayV2::EthereumAsset {
                        kind: 0,
                        data: erc20_asset.abi_encode().into(),
                    },
                ],
                xcm: IGatewayV2::Xcm {
                    kind: 0,
                    data: destination_xcm_bytes.encode().into(),
                },
                claimer: vec![].into(),
                value: 0,
                executionFee: 0,
                relayerFee: 0,
            },
        };

        assert_ok!(send_inbound_message_v2(event));

        // ERC20 tokens should be deposited to container sovereign account on relay
        let container_sovereign_erc20_balance =
            <Dancelight as DancelightRelayPallet>::ForeignAssets::balance(
                ERC20_ASSET_ID,
                &container_sovereign_account,
            );

        // Tanssi tokens should also be deposited to container sovereign account
        let container_sovereign_native_balance =
            <Dancelight as DancelightRelayPallet>::System::account(&container_sovereign_account)
                .data
                .free;

        assert_eq!(container_sovereign_erc20_balance, TRANSFER_AMOUNT);
        assert!(container_sovereign_native_balance == dest_fee_amount);

        // Check events in Dancelight
        type RuntimeEvent = <Dancelight as Chain>::RuntimeEvent;
        assert_expected_events!(
            Dancelight,
            vec![
                // EthereumInboundQueueV2 MessageReceived event
                RuntimeEvent::EthereumInboundQueueV2(
                    snowbridge_pallet_inbound_queue_v2::Event::MessageReceived { nonce: 1, .. }
                ) => {},
                // ForeignAssets Issued event for ERC20 tokens
                RuntimeEvent::ForeignAssets(
                    pallet_assets::Event::Issued { asset_id, amount, .. }
                ) => {
                    asset_id: *asset_id == ERC20_ASSET_ID,
                    amount: *amount == TRANSFER_AMOUNT,
                },
                // Tanssi fees minted to container sovereign account
                RuntimeEvent::Balances(
                    pallet_balances::Event::Minted { amount, .. }
                ) => {
                    amount: *amount == dest_fee_amount,
                },
            ]
        );
    });

    // Check ERC20 token is received in container
    FrontierTemplate::execute_with(|| {
        let receiver_erc20_balance_after =
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssets::balance(
                ERC20_ASSET_ID,
                &token_receiver,
            );

        let receiver_tanssi_balance_after =
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssets::balance(
                RELAY_NATIVE_TOKEN_ASSET_ID,
                &token_receiver,
            );

        assert!(receiver_tanssi_balance_after > 0);
        assert!(receiver_tanssi_balance_after < dest_fee_amount);

        assert_eq!(
            receiver_erc20_balance_after,
            receiver_erc20_balance_before + TRANSFER_AMOUNT
        );

        // Check events in FrontierTemplate
        type RuntimeEvent = <FrontierTemplate as Chain>::RuntimeEvent;
        assert_expected_events!(
            FrontierTemplate,
            vec![
                // ForeignAssets Issued event for ERC20 tokens received
                RuntimeEvent::ForeignAssets(
                    pallet_assets::Event::Issued { asset_id, amount, .. }
                ) => {
                    asset_id: *asset_id == ERC20_ASSET_ID,
                    amount: *amount == TRANSFER_AMOUNT,
                },
                RuntimeEvent::ForeignAssets(
                    pallet_assets::Event::Issued { asset_id, amount, .. }
                ) => {
                    asset_id: *asset_id == RELAY_NATIVE_TOKEN_ASSET_ID,
                    amount: *amount == receiver_tanssi_balance_after,
                },
            ]
        );
    });
}

/// Test: ERC20 token transfer from Ethereum to Simple container chain via V2 inbound queue
#[test]
fn check_erc20_token_to_simple_container_via_v2_works() {
    sp_tracing::try_init_simple();

    let token_receiver: [u8; 32] = [5u8; 32];
    let token_receiver_account: sp_runtime::AccountId32 = token_receiver.into();
    let container_para_id: u32 = <SimpleTemplate as xcm_emulator::Parachain>::para_id().into();

    let mut receiver_erc20_balance_before = 0;
    let mut dest_fee_amount = 0;

    let container_location = Location::new(0, Parachain(container_para_id));
    let container_sovereign_account =
        dancelight_runtime::xcm_config::LocationConverter::convert_location(&container_location)
            .unwrap();

    let ethereum_network_id = SimpleTemplate::execute_with(|| {
        container_chain_template_simple_runtime::EthereumNetwork::get()
    });

    // Store the Tanssi token ID for use in the V2 message
    let tanssi_token_id: [u8; 32] = Dancelight::execute_with(|| {
        let root_origin = <Dancelight as Chain>::RuntimeOrigin::root();
        let alice_account = DancelightSender::get();

        // Register Tanssi native token in EthereumSystemV2 (for fees)
        let token_location = Location::here();
        assert_ok!(
            <Dancelight as DancelightRelayPallet>::EthereumSystemV2::register_token(
                root_origin.clone(),
                Box::new(token_location.clone().into()),
                Box::new(token_location.clone().into()),
                snowbridge_core::AssetMetadata {
                    name: "relay".as_bytes().to_vec().try_into().unwrap(),
                    symbol: "relay".as_bytes().to_vec().try_into().unwrap(),
                    decimals: 12,
                },
                1
            )
        );

        // Fund Ethereum sovereign account for token withdrawals
        assert_ok!(
            <<Dancelight as DancelightRelayPallet>::Balances as Mutate<_>>::mint_into(
                &EthereumSovereignAccount::get(),
                10_000_000_000_000_000_000u128
            )
        );

        // Get Tanssi token ID for the V2 message
        let token_location_reanchored = token_location
            .clone()
            .reanchored(&EthereumLocation::get(), &UniversalLocation::get())
            .expect("unable to reanchor token");

        let tanssi_token_id: [u8; 32] = snowbridge_pallet_system::NativeToForeignId::<
            dancelight_runtime::Runtime,
        >::get(&token_location_reanchored)
        .unwrap()
        .into();

        // Register ERC20 foreign token in ForeignAssetsCreator
        let erc20_asset_location_relay: Location = Location {
            parents: 1,
            interior: X2([
                GlobalConsensus(ethereum_network_id),
                AccountKey20 {
                    network: Some(ethereum_network_id),
                    key: TOKEN_ADDRESS.into(),
                },
            ]
            .into()),
        };

        assert_ok!(
            <Dancelight as DancelightRelayPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                erc20_asset_location_relay,
                ERC20_ASSET_ID,
                alice_account,
                true,
                1
            )
        );

        tanssi_token_id
    });

    // Setup on SimpleTemplate container
    SimpleTemplate::execute_with(|| {
        let root_origin = <SimpleTemplate as Chain>::RuntimeOrigin::root();
        let alice_account = SimpleTemplateSender::get();

        receiver_erc20_balance_before =
            <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssets::balance(
                ERC20_ASSET_ID,
                &token_receiver_account,
            );

        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                RELAY_TOKEN_ASSET_LOCATION,
                RELAY_NATIVE_TOKEN_ASSET_ID,
                alice_account.clone(),
                true,
                1
            )
        );

        // Create asset rate for relay token
        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::AssetRate::create(
                root_origin.clone(),
                Box::new(RELAY_NATIVE_TOKEN_ASSET_ID),
                FixedU128::from_u32(500_000_000)
            )
        );

        // Register ERC20 foreign token in ForeignAssetsCreator
        let erc20_asset_location_container: Location = Location {
            parents: 2,
            interior: X2([
                GlobalConsensus(ethereum_network_id),
                AccountKey20 {
                    network: Some(ethereum_network_id),
                    key: TOKEN_ADDRESS.into(),
                },
            ]
            .into()),
        };

        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                erc20_asset_location_container,
                ERC20_ASSET_ID,
                alice_account,
                true,
                1
            )
        );
    });

    // Send V2 inbound message
    Dancelight::execute_with(|| {
        // Build V2 asset encoding for ERC20
        let erc20_asset = IGatewayV2::AsNativeTokenERC20 {
            token_id: Address::from_slice(TOKEN_ADDRESS.as_bytes()),
            value: TRANSFER_AMOUNT,
        };

        // Build Tanssi native token asset for fees (kind: 1 = foreign token registered via EthereumSystemV2)
        let tanssi_fee_asset = IGatewayV2::AsForeignTokenERC20 {
            token_id: FixedBytes(tanssi_token_id),
            value: TANSSI_FEE_AMOUNT,
        };

        // We send half of the TANSSI_FEE_AMOUNT and leave the rest to pay for delivery fees in Tanssi.
        dest_fee_amount = 5_000_000_000_000;
        let fee_asset =
            AssetId(Location::here()).into_asset(Fungibility::Fungible(dest_fee_amount));
        let erc20_asset_location_relay: Location = Location {
            parents: 1,
            interior: X2([
                GlobalConsensus(ethereum_network_id),
                AccountKey20 {
                    network: Some(ethereum_network_id),
                    key: TOKEN_ADDRESS.into(),
                },
            ]
            .into()),
        };
        let erc20_asset_for_transfer =
            AssetId(erc20_asset_location_relay).into_asset(Fungibility::Fungible(TRANSFER_AMOUNT));

        let custom_xcm: Vec<Instruction<()>> = vec![
            // Transfer to parachain
            InitiateTransfer {
                destination: Location::new(0, Parachain(container_para_id)),
                remote_fees: Some(AssetTransferFilter::ReserveDeposit(
                    fee_asset.clone().into(),
                )),
                // We clear the origin in destination.
                preserve_origin: false,
                assets: sp_runtime::BoundedVec::truncate_from(vec![
                    AssetTransferFilter::ReserveDeposit(Definite(
                        erc20_asset_for_transfer.clone().into(),
                    )),
                ]),
                remote_xcm: Xcm(vec![
                    RefundSurplus,
                    DepositAsset {
                        assets: Wild(AllCounted(2)),
                        beneficiary: Location::new(
                            0,
                            xcm::latest::Junction::AccountId32 {
                                network: None,
                                id: token_receiver,
                            },
                        ),
                    },
                ]),
            },
        ];

        let versioned_destination_xcm: VersionedXcm<()> = VersionedXcm::V5(Xcm(custom_xcm));
        let destination_xcm_bytes = RawPayload::Xcm(versioned_destination_xcm.encode());

        let gateway_address = EthereumGatewayAddress::get();

        let event = IGatewayV2::OutboundMessageAccepted {
            nonce: 1u64,
            payload: IGatewayV2::Payload {
                origin: Address::from_slice(&gateway_address.as_bytes()),
                assets: vec![
                    // Tanssi native token for fees (kind: 1)
                    IGatewayV2::EthereumAsset {
                        kind: 1,
                        data: tanssi_fee_asset.abi_encode().into(),
                    },
                    // ERC20 token for transfer (kind: 0)
                    IGatewayV2::EthereumAsset {
                        kind: 0,
                        data: erc20_asset.abi_encode().into(),
                    },
                ],
                xcm: IGatewayV2::Xcm {
                    kind: 0,
                    data: destination_xcm_bytes.encode().into(),
                },
                claimer: vec![].into(),
                value: 0,
                executionFee: 0,
                relayerFee: 0,
            },
        };

        assert_ok!(send_inbound_message_v2(event));

        // ERC20 tokens should be deposited to container sovereign account on relay
        let container_sovereign_erc20_balance =
            <Dancelight as DancelightRelayPallet>::ForeignAssets::balance(
                ERC20_ASSET_ID,
                &container_sovereign_account,
            );

        // Tanssi tokens should also be deposited to container sovereign account
        let container_sovereign_native_balance =
            <Dancelight as DancelightRelayPallet>::System::account(&container_sovereign_account)
                .data
                .free;

        assert_eq!(container_sovereign_erc20_balance, TRANSFER_AMOUNT);
        assert!(container_sovereign_native_balance == dest_fee_amount);

        // Check events in Dancelight
        type RuntimeEvent = <Dancelight as Chain>::RuntimeEvent;
        assert_expected_events!(
            Dancelight,
            vec![
                // EthereumInboundQueueV2 MessageReceived event
                RuntimeEvent::EthereumInboundQueueV2(
                    snowbridge_pallet_inbound_queue_v2::Event::MessageReceived { nonce: 1, .. }
                ) => {},
                // ForeignAssets Issued event for ERC20 tokens
                RuntimeEvent::ForeignAssets(
                    pallet_assets::Event::Issued { asset_id, amount, .. }
                ) => {
                    asset_id: *asset_id == ERC20_ASSET_ID,
                    amount: *amount == TRANSFER_AMOUNT,
                },
                // Tanssi fees minted to container sovereign account
                RuntimeEvent::Balances(
                    pallet_balances::Event::Minted { amount, .. }
                ) => {
                    amount: *amount == dest_fee_amount,
                },
            ]
        );
    });

    // Check ERC20 token is received in container
    SimpleTemplate::execute_with(|| {
        let receiver_erc20_balance_after =
            <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssets::balance(
                ERC20_ASSET_ID,
                &token_receiver_account,
            );

        let receiver_tanssi_balance_after =
            <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssets::balance(
                RELAY_NATIVE_TOKEN_ASSET_ID,
                &token_receiver_account,
            );

        assert!(receiver_tanssi_balance_after > 0);
        assert!(receiver_tanssi_balance_after < dest_fee_amount);

        assert_eq!(
            receiver_erc20_balance_after,
            receiver_erc20_balance_before + TRANSFER_AMOUNT
        );

        // Check events in SimpleTemplate
        type RuntimeEvent = <SimpleTemplate as Chain>::RuntimeEvent;
        assert_expected_events!(
            SimpleTemplate,
            vec![
                // ForeignAssets Issued event for ERC20 tokens received
                RuntimeEvent::ForeignAssets(
                    pallet_assets::Event::Issued { asset_id, amount, .. }
                ) => {
                    asset_id: *asset_id == ERC20_ASSET_ID,
                    amount: *amount == TRANSFER_AMOUNT,
                },
                RuntimeEvent::ForeignAssets(
                    pallet_assets::Event::Issued { asset_id, amount, .. }
                ) => {
                    asset_id: *asset_id == RELAY_NATIVE_TOKEN_ASSET_ID,
                    amount: *amount == receiver_tanssi_balance_after,
                },
            ]
        );
    });
}

/// Test: Native ETH transfer from Ethereum to Frontier container chain via V2 inbound queue
#[test]
fn check_eth_to_frontier_container_via_v2_works() {
    sp_tracing::try_init_simple();

    let token_receiver: AccountId20 = [5u8; 20].into();
    let container_para_id: u32 = <FrontierTemplate as xcm_emulator::Parachain>::para_id().into();

    let eth_transfer_amount: u128 = 50_000_000_000;
    let eth_asset_id: u16 = 99;

    let mut receiver_eth_balance_before = 0;
    let mut dest_fee_amount = 0;

    let container_location = Location::new(0, Parachain(container_para_id));
    let container_sovereign_account =
        dancelight_runtime::xcm_config::LocationConverter::convert_location(&container_location)
            .unwrap();

    let ethereum_network_id = FrontierTemplate::execute_with(|| {
        container_chain_template_frontier_runtime::EthereumNetwork::get()
    });

    // Store the Tanssi token ID for use in the V2 message
    let tanssi_token_id: [u8; 32] = Dancelight::execute_with(|| {
        let root_origin = <Dancelight as Chain>::RuntimeOrigin::root();
        let alice_account = DancelightSender::get();

        // Register Tanssi native token in EthereumSystemV2 (for fees)
        let token_location = Location::here();
        assert_ok!(
            <Dancelight as DancelightRelayPallet>::EthereumSystemV2::register_token(
                root_origin.clone(),
                Box::new(token_location.clone().into()),
                Box::new(token_location.clone().into()),
                snowbridge_core::AssetMetadata {
                    name: "relay".as_bytes().to_vec().try_into().unwrap(),
                    symbol: "relay".as_bytes().to_vec().try_into().unwrap(),
                    decimals: 12,
                },
                1
            )
        );

        // Fund Ethereum sovereign account for token withdrawals
        assert_ok!(
            <<Dancelight as DancelightRelayPallet>::Balances as Mutate<_>>::mint_into(
                &EthereumSovereignAccount::get(),
                10_000_000_000_000_000_000u128
            )
        );

        // Get Tanssi token ID for the V2 message
        let token_location_reanchored = token_location
            .clone()
            .reanchored(&EthereumLocation::get(), &UniversalLocation::get())
            .expect("unable to reanchor token");

        let tanssi_token_id: [u8; 32] = snowbridge_pallet_system::NativeToForeignId::<
            dancelight_runtime::Runtime,
        >::get(&token_location_reanchored)
        .unwrap()
        .into();

        // Register native ETH in ForeignAssetsCreator
        let eth_asset_location_relay: Location = Location {
            parents: 1,
            interior: X1([GlobalConsensus(ethereum_network_id)].into()),
        };

        assert_ok!(
            <Dancelight as DancelightRelayPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                eth_asset_location_relay,
                eth_asset_id,
                alice_account,
                true,
                1
            )
        );

        tanssi_token_id
    });

    // Setup on FrontierTemplate container
    FrontierTemplate::execute_with(|| {
        let root_origin = <FrontierTemplate as Chain>::RuntimeOrigin::root();
        let alice_account = FrontierTemplateSender::get();

        receiver_eth_balance_before =
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssets::balance(
                eth_asset_id,
                &token_receiver,
            );

        assert_ok!(
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                RELAY_TOKEN_ASSET_LOCATION,
                RELAY_NATIVE_TOKEN_ASSET_ID,
                alice_account.clone().into(),
                true,
                1
            )
        );

        // Create asset rate for relay token
        assert_ok!(
            <FrontierTemplate as FrontierTemplateParaPallet>::AssetRate::create(
                root_origin.clone(),
                Box::new(RELAY_NATIVE_TOKEN_ASSET_ID),
                FixedU128::from_u32(500_000_000)
            )
        );

        // Register native ETH in container
        let eth_asset_location_container: Location = Location {
            parents: 2,
            interior: X1([GlobalConsensus(ethereum_network_id)].into()),
        };

        assert_ok!(
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                eth_asset_location_container,
                eth_asset_id,
                alice_account.into(),
                true,
                1
            )
        );
    });

    // Send V2 inbound message with ETH value
    Dancelight::execute_with(|| {
        // Build Tanssi native token asset for fees (kind: 1 = foreign token registered via EthereumSystemV2)
        let tanssi_fee_asset = IGatewayV2::AsForeignTokenERC20 {
            token_id: FixedBytes(tanssi_token_id),
            value: TANSSI_FEE_AMOUNT,
        };

        // We send half of the TANSSI_FEE_AMOUNT and leave the rest to pay for delivery fees in Tanssi.
        dest_fee_amount = 5_000_000_000_000;
        let fee_asset =
            AssetId(Location::here()).into_asset(Fungibility::Fungible(dest_fee_amount));

        let eth_asset_location_relay: Location = Location {
            parents: 1,
            interior: X1([GlobalConsensus(ethereum_network_id)].into()),
        };
        let eth_asset_for_transfer = AssetId(eth_asset_location_relay)
            .into_asset(Fungibility::Fungible(eth_transfer_amount));

        let custom_xcm: Vec<Instruction<()>> = vec![
            // Transfer to parachain
            InitiateTransfer {
                destination: Location::new(0, Parachain(container_para_id)),
                remote_fees: Some(AssetTransferFilter::ReserveDeposit(
                    fee_asset.clone().into(),
                )),
                // We clear the origin in destination.
                preserve_origin: false,
                assets: sp_runtime::BoundedVec::truncate_from(vec![
                    AssetTransferFilter::ReserveDeposit(Definite(
                        eth_asset_for_transfer.clone().into(),
                    )),
                ]),
                remote_xcm: Xcm(vec![
                    RefundSurplus,
                    DepositAsset {
                        assets: Wild(AllCounted(2)),
                        beneficiary: Location::new(
                            0,
                            AccountKey20 {
                                network: None,
                                key: token_receiver.into(),
                            },
                        ),
                    },
                ]),
            },
        ];

        let versioned_destination_xcm: VersionedXcm<()> = VersionedXcm::V5(Xcm(custom_xcm));
        let destination_xcm_bytes = RawPayload::Xcm(versioned_destination_xcm.encode());

        let gateway_address = EthereumGatewayAddress::get();

        // V2 uses `value` field for native ETH transfers
        let event = IGatewayV2::OutboundMessageAccepted {
            nonce: 1u64,
            payload: IGatewayV2::Payload {
                origin: Address::from_slice(&gateway_address.as_bytes()),
                assets: vec![
                    // Tanssi native token for fees (kind: 1)
                    IGatewayV2::EthereumAsset {
                        kind: 1,
                        data: tanssi_fee_asset.abi_encode().into(),
                    },
                ],
                xcm: IGatewayV2::Xcm {
                    kind: 0,
                    data: destination_xcm_bytes.encode().into(),
                },
                claimer: vec![].into(),
                value: eth_transfer_amount,
                executionFee: 0,
                relayerFee: 0,
            },
        };

        assert_ok!(send_inbound_message_v2(event));

        // Native ETH should be minted to container sovereign account
        let container_sovereign_eth_balance =
            <Dancelight as DancelightRelayPallet>::ForeignAssets::balance(
                eth_asset_id,
                &container_sovereign_account,
            );

        // Tanssi tokens should also be deposited to container sovereign account
        let container_sovereign_native_balance =
            <Dancelight as DancelightRelayPallet>::System::account(&container_sovereign_account)
                .data
                .free;

        assert_eq!(container_sovereign_eth_balance, eth_transfer_amount);
        assert!(container_sovereign_native_balance == dest_fee_amount);

        // Check events in Dancelight
        type RuntimeEvent = <Dancelight as Chain>::RuntimeEvent;
        assert_expected_events!(
            Dancelight,
            vec![
                // EthereumInboundQueueV2 MessageReceived event
                RuntimeEvent::EthereumInboundQueueV2(
                    snowbridge_pallet_inbound_queue_v2::Event::MessageReceived { nonce: 1, .. }
                ) => {},
                // ForeignAssets Issued event for native ETH
                RuntimeEvent::ForeignAssets(
                    pallet_assets::Event::Issued { asset_id, amount, .. }
                ) => {
                    asset_id: *asset_id == eth_asset_id,
                    amount: *amount == eth_transfer_amount,
                },
                // Tanssi fees minted to container sovereign account
                RuntimeEvent::Balances(
                    pallet_balances::Event::Minted { amount, .. }
                ) => {
                    amount: *amount == dest_fee_amount,
                },
            ]
        );
    });

    // Check ETH is received in container
    FrontierTemplate::execute_with(|| {
        let receiver_eth_balance_after =
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssets::balance(
                eth_asset_id,
                &token_receiver,
            );

        let receiver_tanssi_balance_after =
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssets::balance(
                RELAY_NATIVE_TOKEN_ASSET_ID,
                &token_receiver,
            );

        assert!(receiver_tanssi_balance_after > 0);
        assert!(receiver_tanssi_balance_after < dest_fee_amount);

        assert_eq!(
            receiver_eth_balance_after,
            receiver_eth_balance_before + eth_transfer_amount
        );

        // Check events in FrontierTemplate
        type RuntimeEvent = <FrontierTemplate as Chain>::RuntimeEvent;
        assert_expected_events!(
            FrontierTemplate,
            vec![
                // ForeignAssets Issued event for native ETH received
                RuntimeEvent::ForeignAssets(
                    pallet_assets::Event::Issued { asset_id, amount, .. }
                ) => {
                    asset_id: *asset_id == eth_asset_id,
                    amount: *amount == eth_transfer_amount,
                },
                RuntimeEvent::ForeignAssets(
                    pallet_assets::Event::Issued { asset_id, amount, .. }
                ) => {
                    asset_id: *asset_id == RELAY_NATIVE_TOKEN_ASSET_ID,
                    amount: *amount == receiver_tanssi_balance_after,
                },
            ]
        );
    });
}

/// Test: Native ETH transfer from Ethereum to Simple container chain via V2 inbound queue
#[test]
fn check_eth_to_simple_container_via_v2_works() {
    sp_tracing::try_init_simple();

    let token_receiver: [u8; 32] = [5u8; 32];
    let token_receiver_account: sp_runtime::AccountId32 = token_receiver.into();
    let container_para_id: u32 = <SimpleTemplate as xcm_emulator::Parachain>::para_id().into();

    let eth_transfer_amount: u128 = 50_000_000_000;
    let eth_asset_id: u16 = 99;

    let mut receiver_eth_balance_before = 0;
    let mut dest_fee_amount = 0;

    let container_location = Location::new(0, Parachain(container_para_id));
    let container_sovereign_account =
        dancelight_runtime::xcm_config::LocationConverter::convert_location(&container_location)
            .unwrap();

    let ethereum_network_id = SimpleTemplate::execute_with(|| {
        container_chain_template_simple_runtime::EthereumNetwork::get()
    });

    // Store the Tanssi token ID for use in the V2 message
    let tanssi_token_id: [u8; 32] = Dancelight::execute_with(|| {
        let root_origin = <Dancelight as Chain>::RuntimeOrigin::root();
        let alice_account = DancelightSender::get();

        // Register Tanssi native token in EthereumSystemV2 (for fees)
        let token_location = Location::here();
        assert_ok!(
            <Dancelight as DancelightRelayPallet>::EthereumSystemV2::register_token(
                root_origin.clone(),
                Box::new(token_location.clone().into()),
                Box::new(token_location.clone().into()),
                snowbridge_core::AssetMetadata {
                    name: "relay".as_bytes().to_vec().try_into().unwrap(),
                    symbol: "relay".as_bytes().to_vec().try_into().unwrap(),
                    decimals: 12,
                },
                1
            )
        );

        // Fund Ethereum sovereign account for token withdrawals
        assert_ok!(
            <<Dancelight as DancelightRelayPallet>::Balances as Mutate<_>>::mint_into(
                &EthereumSovereignAccount::get(),
                10_000_000_000_000_000_000u128
            )
        );

        // Get Tanssi token ID for the V2 message
        let token_location_reanchored = token_location
            .clone()
            .reanchored(&EthereumLocation::get(), &UniversalLocation::get())
            .expect("unable to reanchor token");

        let tanssi_token_id: [u8; 32] = snowbridge_pallet_system::NativeToForeignId::<
            dancelight_runtime::Runtime,
        >::get(&token_location_reanchored)
        .unwrap()
        .into();

        // Register native ETH in ForeignAssetsCreator
        let eth_asset_location_relay: Location = Location {
            parents: 1,
            interior: X1([GlobalConsensus(ethereum_network_id)].into()),
        };

        assert_ok!(
            <Dancelight as DancelightRelayPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                eth_asset_location_relay,
                eth_asset_id,
                alice_account,
                true,
                1
            )
        );

        tanssi_token_id
    });

    // Setup on SimpleTemplate container
    SimpleTemplate::execute_with(|| {
        let root_origin = <SimpleTemplate as Chain>::RuntimeOrigin::root();
        let alice_account = SimpleTemplateSender::get();

        receiver_eth_balance_before =
            <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssets::balance(
                eth_asset_id,
                &token_receiver_account,
            );

        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                RELAY_TOKEN_ASSET_LOCATION,
                RELAY_NATIVE_TOKEN_ASSET_ID,
                alice_account.clone(),
                true,
                1
            )
        );

        // Create asset rate for relay token
        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::AssetRate::create(
                root_origin.clone(),
                Box::new(RELAY_NATIVE_TOKEN_ASSET_ID),
                FixedU128::from_u32(500_000_000)
            )
        );

        // Register native ETH in container
        let eth_asset_location_container: Location = Location {
            parents: 2,
            interior: X1([GlobalConsensus(ethereum_network_id)].into()),
        };

        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                eth_asset_location_container,
                eth_asset_id,
                alice_account,
                true,
                1
            )
        );
    });

    // Send V2 inbound message with ETH value
    Dancelight::execute_with(|| {
        // Build Tanssi native token asset for fees (kind: 1 = foreign token registered via EthereumSystemV2)
        let tanssi_fee_asset = IGatewayV2::AsForeignTokenERC20 {
            token_id: FixedBytes(tanssi_token_id),
            value: TANSSI_FEE_AMOUNT,
        };

        // We send half of the TANSSI_FEE_AMOUNT and leave the rest to pay for delivery fees in Tanssi.
        dest_fee_amount = 5_000_000_000_000;
        let fee_asset =
            AssetId(Location::here()).into_asset(Fungibility::Fungible(dest_fee_amount));

        let eth_asset_location_relay: Location = Location {
            parents: 1,
            interior: X1([GlobalConsensus(ethereum_network_id)].into()),
        };
        let eth_asset_for_transfer = AssetId(eth_asset_location_relay)
            .into_asset(Fungibility::Fungible(eth_transfer_amount));

        let custom_xcm: Vec<Instruction<()>> = vec![
            // Transfer to parachain
            InitiateTransfer {
                destination: Location::new(0, Parachain(container_para_id)),
                remote_fees: Some(AssetTransferFilter::ReserveDeposit(
                    fee_asset.clone().into(),
                )),
                // We clear the origin in destination.
                preserve_origin: false,
                assets: sp_runtime::BoundedVec::truncate_from(vec![
                    AssetTransferFilter::ReserveDeposit(Definite(
                        eth_asset_for_transfer.clone().into(),
                    )),
                ]),
                remote_xcm: Xcm(vec![
                    RefundSurplus,
                    DepositAsset {
                        assets: Wild(AllCounted(2)),
                        beneficiary: Location::new(
                            0,
                            xcm::latest::Junction::AccountId32 {
                                network: None,
                                id: token_receiver,
                            },
                        ),
                    },
                ]),
            },
        ];

        let versioned_destination_xcm: VersionedXcm<()> = VersionedXcm::V5(Xcm(custom_xcm));
        let destination_xcm_bytes = RawPayload::Xcm(versioned_destination_xcm.encode());

        let gateway_address = EthereumGatewayAddress::get();

        // V2 uses `value` field for native ETH transfers
        let event = IGatewayV2::OutboundMessageAccepted {
            nonce: 1u64,
            payload: IGatewayV2::Payload {
                origin: Address::from_slice(&gateway_address.as_bytes()),
                assets: vec![
                    // Tanssi native token for fees (kind: 1)
                    IGatewayV2::EthereumAsset {
                        kind: 1,
                        data: tanssi_fee_asset.abi_encode().into(),
                    },
                ],
                xcm: IGatewayV2::Xcm {
                    kind: 0,
                    data: destination_xcm_bytes.encode().into(),
                },
                claimer: vec![].into(),
                value: eth_transfer_amount,
                executionFee: 0,
                relayerFee: 0,
            },
        };

        assert_ok!(send_inbound_message_v2(event));

        // Native ETH should be minted to container sovereign account
        let container_sovereign_eth_balance =
            <Dancelight as DancelightRelayPallet>::ForeignAssets::balance(
                eth_asset_id,
                &container_sovereign_account,
            );

        // Tanssi tokens should also be deposited to container sovereign account
        let container_sovereign_native_balance =
            <Dancelight as DancelightRelayPallet>::System::account(&container_sovereign_account)
                .data
                .free;

        assert_eq!(container_sovereign_eth_balance, eth_transfer_amount);
        assert!(container_sovereign_native_balance == dest_fee_amount);

        // Check events in Dancelight
        type RuntimeEvent = <Dancelight as Chain>::RuntimeEvent;
        assert_expected_events!(
            Dancelight,
            vec![
                // EthereumInboundQueueV2 MessageReceived event
                RuntimeEvent::EthereumInboundQueueV2(
                    snowbridge_pallet_inbound_queue_v2::Event::MessageReceived { nonce: 1, .. }
                ) => {},
                // ForeignAssets Issued event for native ETH
                RuntimeEvent::ForeignAssets(
                    pallet_assets::Event::Issued { asset_id, amount, .. }
                ) => {
                    asset_id: *asset_id == eth_asset_id,
                    amount: *amount == eth_transfer_amount,
                },
                // Tanssi fees minted to container sovereign account
                RuntimeEvent::Balances(
                    pallet_balances::Event::Minted { amount, .. }
                ) => {
                    amount: *amount == dest_fee_amount,
                },
            ]
        );
    });

    // Check ETH is received in container
    SimpleTemplate::execute_with(|| {
        let receiver_eth_balance_after =
            <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssets::balance(
                eth_asset_id,
                &token_receiver_account,
            );

        let receiver_tanssi_balance_after =
            <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssets::balance(
                RELAY_NATIVE_TOKEN_ASSET_ID,
                &token_receiver_account,
            );

        assert!(receiver_tanssi_balance_after > 0);
        assert!(receiver_tanssi_balance_after < dest_fee_amount);

        assert_eq!(
            receiver_eth_balance_after,
            receiver_eth_balance_before + eth_transfer_amount
        );

        // Check events in SimpleTemplate
        type RuntimeEvent = <SimpleTemplate as Chain>::RuntimeEvent;
        assert_expected_events!(
            SimpleTemplate,
            vec![
                // ForeignAssets Issued event for native ETH received
                RuntimeEvent::ForeignAssets(
                    pallet_assets::Event::Issued { asset_id, amount, .. }
                ) => {
                    asset_id: *asset_id == eth_asset_id,
                    amount: *amount == eth_transfer_amount,
                },
                RuntimeEvent::ForeignAssets(
                    pallet_assets::Event::Issued { asset_id, amount, .. }
                ) => {
                    asset_id: *asset_id == RELAY_NATIVE_TOKEN_ASSET_ID,
                    amount: *amount == receiver_tanssi_balance_after,
                },
            ]
        );
    });
}

/// Test: Tanssi native token transfer from Ethereum to Frontier container chain via V2 inbound queue
/// This tests the scenario where Tanssi tokens that were bridged to Ethereum are sent back to a container
#[test]
fn check_tanssi_to_frontier_container_via_v2_works() {
    sp_tracing::try_init_simple();

    let token_receiver: AccountId20 = [5u8; 20].into();
    let container_para_id: u32 = <FrontierTemplate as xcm_emulator::Parachain>::para_id().into();

    let tanssi_amount_to_withdraw: u128 = 50_000_000_000_000; // 50 Tanssi tokens

    // We will leave a bit on holding for delivery fees + dest fees.
    let tanssi_amount_to_transfer: u128 = 43_000_000_000_000; // 43 Tanssi tokens

    let mut receiver_tanssi_balance_before = 0;

    let container_location = Location::new(0, Parachain(container_para_id));
    let container_sovereign_account =
        dancelight_runtime::xcm_config::LocationConverter::convert_location(&container_location)
            .unwrap();

    // Setup on Dancelight
    Dancelight::execute_with(|| {
        let root_origin = <Dancelight as Chain>::RuntimeOrigin::root();

        // Register Tanssi native token with EthereumSystemV2
        let token_location = Location::here();
        assert_ok!(dancelight_runtime::EthereumSystemV2::register_token(
            root_origin.clone(),
            Box::new(token_location.clone().into()),
            Box::new(token_location.clone().into()),
            snowbridge_core::AssetMetadata {
                name: "relay".as_bytes().to_vec().try_into().unwrap(),
                symbol: "relay".as_bytes().to_vec().try_into().unwrap(),
                decimals: 12,
            },
            1
        ));

        // Add funds to Ethereum sovereign account for token transfers
        assert_ok!(
            <<Dancelight as DancelightRelayPallet>::Balances as Mutate<_>>::mint_into(
                &EthereumSovereignAccount::get(),
                100_000_000_000_000_000_000u128
            )
        );
    });

    // Setup on FrontierTemplate container
    FrontierTemplate::execute_with(|| {
        let root_origin = <FrontierTemplate as Chain>::RuntimeOrigin::root();
        let alice_account = FrontierTemplateSender::get();

        // Register relay token (Tanssi) as foreign asset
        assert_ok!(
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                RELAY_TOKEN_ASSET_LOCATION,
                RELAY_NATIVE_TOKEN_ASSET_ID,
                alice_account.clone().into(),
                true,
                1
            )
        );

        // Create asset rate for relay token
        assert_ok!(
            <FrontierTemplate as FrontierTemplateParaPallet>::AssetRate::create(
                root_origin.clone(),
                Box::new(RELAY_NATIVE_TOKEN_ASSET_ID),
                FixedU128::from_u32(500_000_000)
            )
        );

        receiver_tanssi_balance_before =
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssets::balance(
                RELAY_NATIVE_TOKEN_ASSET_ID,
                &token_receiver,
            );
    });

    // Send V2 inbound message
    Dancelight::execute_with(|| {
        // Get Tanssi token ID for the transfer
        let tanssi_token_id: [u8; 32] =
            snowbridge_pallet_system::NativeToForeignId::<dancelight_runtime::Runtime>::get(
                &Location::here()
                    .reanchored(&EthereumLocation::get(), &UniversalLocation::get())
                    .expect("unable to reanchor token"),
            )
            .unwrap()
            .into();

        // Build Tanssi native token asset for transfer (kind: 1 = foreign token registered via EthereumSystemV2)
        let tanssi_transfer_asset = IGatewayV2::AsForeignTokenERC20 {
            token_id: FixedBytes(tanssi_token_id),
            value: tanssi_amount_to_withdraw,
        };

        let dest_fee_amount = 5_000_000_000_000;
        let fee_asset =
            AssetId(Location::here()).into_asset(Fungibility::Fungible(dest_fee_amount));

        // Build the transfer asset (Tanssi from relay perspective is Location::here())
        let tanssi_for_transfer =
            AssetId(Location::here()).into_asset(Fungibility::Fungible(tanssi_amount_to_transfer));

        // Build the XCM to transfer Tanssi to container
        let custom_xcm: Vec<Instruction<()>> = vec![
            // Transfer to parachain
            InitiateTransfer {
                destination: Location::new(0, Parachain(container_para_id)),
                remote_fees: Some(AssetTransferFilter::ReserveDeposit(
                    fee_asset.clone().into(),
                )),
                preserve_origin: false,
                assets: sp_runtime::BoundedVec::truncate_from(vec![
                    AssetTransferFilter::ReserveDeposit(Definite(
                        tanssi_for_transfer.clone().into(),
                    )),
                ]),
                remote_xcm: Xcm(vec![
                    // We don't add refundSurplus here as we want to test the exact transfer amount we sent.
                    // RefundSurplus,
                    DepositAsset {
                        assets: Wild(AllCounted(1)),
                        beneficiary: Location::new(
                            0,
                            AccountKey20 {
                                network: None,
                                key: token_receiver.into(),
                            },
                        ),
                    },
                ]),
            },
        ];

        let versioned_destination_xcm: VersionedXcm<()> = VersionedXcm::V5(Xcm(custom_xcm));
        let destination_xcm_bytes = RawPayload::Xcm(versioned_destination_xcm.encode());

        let gateway_address = EthereumGatewayAddress::get();

        let event = IGatewayV2::OutboundMessageAccepted {
            nonce: 1u64,
            payload: IGatewayV2::Payload {
                origin: Address::from_slice(&gateway_address.as_bytes()),
                assets: vec![
                    // Tanssi native token for transfer (kind: 1)
                    IGatewayV2::EthereumAsset {
                        kind: 1,
                        data: tanssi_transfer_asset.abi_encode().into(),
                    },
                ],
                xcm: IGatewayV2::Xcm {
                    kind: 0,
                    data: destination_xcm_bytes.encode().into(),
                },
                claimer: vec![].into(),
                value: 0,
                executionFee: 0,
                relayerFee: 0,
            },
        };

        assert_ok!(send_inbound_message_v2(event));

        // Tanssi tokens should be deposited to container sovereign account
        let container_sovereign_native_balance =
            <Dancelight as DancelightRelayPallet>::System::account(&container_sovereign_account)
                .data
                .free;

        // Should have both fee + transfer amount deposited
        assert!(container_sovereign_native_balance == tanssi_amount_to_transfer + dest_fee_amount);

        // Check events in Dancelight
        type RuntimeEvent = <Dancelight as Chain>::RuntimeEvent;
        assert_expected_events!(
            Dancelight,
            vec![
                // EthereumInboundQueueV2 MessageReceived event
                RuntimeEvent::EthereumInboundQueueV2(
                    snowbridge_pallet_inbound_queue_v2::Event::MessageReceived { nonce: 1, .. }
                ) => {},
                // Tanssi transfer amount minted to container sovereign account
                RuntimeEvent::Balances(
                    pallet_balances::Event::Minted { amount, .. }
                ) => {
                    amount: *amount == tanssi_amount_to_transfer,
                },
                // Tanssi fees minted to container sovereign account
                RuntimeEvent::Balances(
                    pallet_balances::Event::Minted { amount, .. }
                ) => {
                    amount: *amount == dest_fee_amount,
                },
            ]
        );
    });

    // Check Tanssi is received in container
    FrontierTemplate::execute_with(|| {
        let receiver_tanssi_balance_after =
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssets::balance(
                RELAY_NATIVE_TOKEN_ASSET_ID,
                &token_receiver,
            );

        assert!(
            receiver_tanssi_balance_after > receiver_tanssi_balance_before,
            "Receiver should have received Tanssi tokens"
        );
        // Should receive transfer amount + leftover from fees
        assert!(
            receiver_tanssi_balance_after == tanssi_amount_to_transfer,
            "Receiver should have the transfer amount"
        );

        // Check events in FrontierTemplate
        type RuntimeEvent = <FrontierTemplate as Chain>::RuntimeEvent;
        assert_expected_events!(
            FrontierTemplate,
            vec![
                // ForeignAssets Issued event for Tanssi tokens received
                RuntimeEvent::ForeignAssets(
                    pallet_assets::Event::Issued { asset_id, amount, .. }
                ) => {
                    asset_id: *asset_id == RELAY_NATIVE_TOKEN_ASSET_ID,
                    amount: *amount == tanssi_amount_to_transfer,
                },
            ]
        );
    });
}

/// Test: Tanssi native token transfer from Ethereum to Simple container chain via V2 inbound queue
#[test]
fn check_tanssi_to_simple_container_via_v2_works() {
    sp_tracing::try_init_simple();

    let token_receiver: [u8; 32] = [5u8; 32];
    let token_receiver_account: sp_runtime::AccountId32 = token_receiver.into();
    let container_para_id: u32 = <SimpleTemplate as xcm_emulator::Parachain>::para_id().into();

    let tanssi_amount_to_withdraw: u128 = 50_000_000_000_000; // 50 Tanssi tokens

    // We will leave a bit on holding for delivery fees + dest fees.
    let tanssi_amount_to_transfer: u128 = 43_000_000_000_000; // 43 Tanssi tokens

    let mut receiver_tanssi_balance_before = 0;

    let container_location = Location::new(0, Parachain(container_para_id));
    let container_sovereign_account =
        dancelight_runtime::xcm_config::LocationConverter::convert_location(&container_location)
            .unwrap();

    // Setup on Dancelight
    Dancelight::execute_with(|| {
        let root_origin = <Dancelight as Chain>::RuntimeOrigin::root();

        // Register Tanssi native token with EthereumSystemV2
        let token_location = Location::here();
        assert_ok!(dancelight_runtime::EthereumSystemV2::register_token(
            root_origin.clone(),
            Box::new(token_location.clone().into()),
            Box::new(token_location.clone().into()),
            snowbridge_core::AssetMetadata {
                name: "relay".as_bytes().to_vec().try_into().unwrap(),
                symbol: "relay".as_bytes().to_vec().try_into().unwrap(),
                decimals: 12,
            },
            1
        ));

        // Add funds to Ethereum sovereign account for token transfers
        assert_ok!(
            <<Dancelight as DancelightRelayPallet>::Balances as Mutate<_>>::mint_into(
                &EthereumSovereignAccount::get(),
                100_000_000_000_000_000_000u128
            )
        );
    });

    // Setup on SimpleTemplate container
    SimpleTemplate::execute_with(|| {
        let root_origin = <SimpleTemplate as Chain>::RuntimeOrigin::root();
        let alice_account = SimpleTemplateSender::get();

        // Register relay token (Tanssi) as foreign asset
        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                RELAY_TOKEN_ASSET_LOCATION,
                RELAY_NATIVE_TOKEN_ASSET_ID,
                alice_account.clone(),
                true,
                1
            )
        );

        // Create asset rate for relay token
        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::AssetRate::create(
                root_origin.clone(),
                Box::new(RELAY_NATIVE_TOKEN_ASSET_ID),
                FixedU128::from_u32(500_000_000)
            )
        );

        receiver_tanssi_balance_before =
            <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssets::balance(
                RELAY_NATIVE_TOKEN_ASSET_ID,
                &token_receiver_account,
            );
    });

    // Send V2 inbound message
    Dancelight::execute_with(|| {
        // Get Tanssi token ID for the transfer
        let tanssi_token_id: [u8; 32] =
            snowbridge_pallet_system::NativeToForeignId::<dancelight_runtime::Runtime>::get(
                &Location::here()
                    .reanchored(&EthereumLocation::get(), &UniversalLocation::get())
                    .expect("unable to reanchor token"),
            )
            .unwrap()
            .into();

        // Build Tanssi native token asset for transfer (kind: 1 = foreign token registered via EthereumSystemV2)
        let tanssi_transfer_asset = IGatewayV2::AsForeignTokenERC20 {
            token_id: FixedBytes(tanssi_token_id),
            value: tanssi_amount_to_withdraw,
        };

        let dest_fee_amount = 5_000_000_000_000;
        let fee_asset =
            AssetId(Location::here()).into_asset(Fungibility::Fungible(dest_fee_amount));

        // Build the transfer asset (Tanssi from relay perspective is Location::here())
        let tanssi_for_transfer =
            AssetId(Location::here()).into_asset(Fungibility::Fungible(tanssi_amount_to_transfer));

        // Build the XCM to transfer Tanssi to container
        let custom_xcm: Vec<Instruction<()>> = vec![
            // Transfer to parachain
            InitiateTransfer {
                destination: Location::new(0, Parachain(container_para_id)),
                remote_fees: Some(AssetTransferFilter::ReserveDeposit(
                    fee_asset.clone().into(),
                )),
                preserve_origin: false,
                assets: sp_runtime::BoundedVec::truncate_from(vec![
                    AssetTransferFilter::ReserveDeposit(Definite(
                        tanssi_for_transfer.clone().into(),
                    )),
                ]),
                remote_xcm: Xcm(vec![
                    // We don't add refundSurplus here as we want to test the exact transfer amount we sent.
                    // RefundSurplus,
                    DepositAsset {
                        assets: Wild(AllCounted(1)),
                        beneficiary: Location::new(
                            0,
                            xcm::latest::Junction::AccountId32 {
                                network: None,
                                id: token_receiver,
                            },
                        ),
                    },
                ]),
            },
        ];

        let versioned_destination_xcm: VersionedXcm<()> = VersionedXcm::V5(Xcm(custom_xcm));
        let destination_xcm_bytes = RawPayload::Xcm(versioned_destination_xcm.encode());

        let gateway_address = EthereumGatewayAddress::get();

        let event = IGatewayV2::OutboundMessageAccepted {
            nonce: 1u64,
            payload: IGatewayV2::Payload {
                origin: Address::from_slice(&gateway_address.as_bytes()),
                assets: vec![
                    // Tanssi native token for transfer (kind: 1)
                    IGatewayV2::EthereumAsset {
                        kind: 1,
                        data: tanssi_transfer_asset.abi_encode().into(),
                    },
                ],
                xcm: IGatewayV2::Xcm {
                    kind: 0,
                    data: destination_xcm_bytes.encode().into(),
                },
                claimer: vec![].into(),
                value: 0,
                executionFee: 0,
                relayerFee: 0,
            },
        };

        assert_ok!(send_inbound_message_v2(event));

        // Tanssi tokens should be deposited to container sovereign account
        let container_sovereign_native_balance =
            <Dancelight as DancelightRelayPallet>::System::account(&container_sovereign_account)
                .data
                .free;

        // Should have both fee + transfer amount deposited
        assert!(container_sovereign_native_balance == tanssi_amount_to_transfer + dest_fee_amount);

        // Check events in Dancelight
        type RuntimeEvent = <Dancelight as Chain>::RuntimeEvent;
        assert_expected_events!(
            Dancelight,
            vec![
                // EthereumInboundQueueV2 MessageReceived event
                RuntimeEvent::EthereumInboundQueueV2(
                    snowbridge_pallet_inbound_queue_v2::Event::MessageReceived { nonce: 1, .. }
                ) => {},
                // Tanssi transfer amount minted to container sovereign account
                RuntimeEvent::Balances(
                    pallet_balances::Event::Minted { amount, .. }
                ) => {
                    amount: *amount == tanssi_amount_to_transfer,
                },
                // Tanssi fees minted to container sovereign account
                RuntimeEvent::Balances(
                    pallet_balances::Event::Minted { amount, .. }
                ) => {
                    amount: *amount == dest_fee_amount,
                },
            ]
        );
    });

    // Check Tanssi is received in container
    SimpleTemplate::execute_with(|| {
        let receiver_tanssi_balance_after =
            <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssets::balance(
                RELAY_NATIVE_TOKEN_ASSET_ID,
                &token_receiver_account,
            );

        assert!(
            receiver_tanssi_balance_after > receiver_tanssi_balance_before,
            "Receiver should have received Tanssi tokens"
        );
        // Should receive the exact transfer amount
        assert!(
            receiver_tanssi_balance_after == tanssi_amount_to_transfer,
            "Receiver should have the transfer amount"
        );

        // Check events in SimpleTemplate
        type RuntimeEvent = <SimpleTemplate as Chain>::RuntimeEvent;
        assert_expected_events!(
            SimpleTemplate,
            vec![
                // ForeignAssets Issued event for Tanssi tokens received
                RuntimeEvent::ForeignAssets(
                    pallet_assets::Event::Issued { asset_id, amount, .. }
                ) => {
                    asset_id: *asset_id == RELAY_NATIVE_TOKEN_ASSET_ID,
                    amount: *amount == tanssi_amount_to_transfer,
                },
            ]
        );
    });
}

/// Test: Container native token (kind: 1) transfer from Ethereum to Frontier container chain via V2 inbound queue
/// This tests the scenario where a container's native token (bridged to Ethereum) is sent back to its origin chain
#[test]
fn check_container_native_to_frontier_container_via_v2_works() {
    sp_tracing::try_init_simple();

    let token_receiver: AccountId20 = [5u8; 20].into();
    let container_para_id: u32 = <FrontierTemplate as xcm_emulator::Parachain>::para_id().into();

    let container_token_transfer_amount: u128 = 25_000_000_000_000; // Amount of container native token to transfer
    let dest_fee_amount: u128 = 5_000_000_000_000; // Relay tokens for container execution fees

    let mut receiver_native_balance_before = 0u128;

    // Container native token location from relay perspective
    let container_native_token_location_relay = Location::new(0, [Parachain(container_para_id), PalletInstance(<<FrontierTemplate as FrontierTemplateParaPallet>::Balances as PalletInfoAccess>::index() as u8)]);

    // Get Ethereum network ID
    let ethereum_network_id = FrontierTemplate::execute_with(|| {
        container_chain_template_frontier_runtime::EthereumNetwork::get()
    });

    // Get Ethereum sovereign account on the container chain
    let ethereum_sovereign_on_container = FrontierTemplate::execute_with(|| {
        container_chain_template_frontier_runtime::xcm_config::LocationToAccountId::convert_location(
            &Location::new(1, [GlobalConsensus(ethereum_network_id)])
        ).unwrap()
    });

    // Setup on Dancelight
    Dancelight::execute_with(|| {
        let root_origin = <Dancelight as Chain>::RuntimeOrigin::root();

        // Register Tanssi native token with EthereumSystemV2 (for fees)
        let tanssi_location = Location::here();
        assert_ok!(dancelight_runtime::EthereumSystemV2::register_token(
            root_origin.clone(),
            Box::new(tanssi_location.clone().into()),
            Box::new(tanssi_location.clone().into()),
            snowbridge_core::AssetMetadata {
                name: "relay".as_bytes().to_vec().try_into().unwrap(),
                symbol: "relay".as_bytes().to_vec().try_into().unwrap(),
                decimals: 12,
            },
            1
        ));

        // Register container native token with EthereumSystemV2
        assert_ok!(dancelight_runtime::EthereumSystemV2::register_token(
            root_origin.clone(),
            Box::new(container_native_token_location_relay.clone().into()),
            Box::new(container_native_token_location_relay.clone().into()),
            snowbridge_core::AssetMetadata {
                name: "container".as_bytes().to_vec().try_into().unwrap(),
                symbol: "CTR".as_bytes().to_vec().try_into().unwrap(),
                decimals: 18,
            },
            2
        ));

        // Add funds to Ethereum sovereign account for relay token fees
        assert_ok!(
            <<Dancelight as DancelightRelayPallet>::Balances as Mutate<_>>::mint_into(
                &EthereumSovereignAccount::get(),
                100_000_000_000_000_000_000u128
            )
        );
    });

    // Setup on FrontierTemplate container
    FrontierTemplate::execute_with(|| {
        let root_origin = <FrontierTemplate as Chain>::RuntimeOrigin::root();
        let alice_account = FrontierTemplateSender::get();

        // Register relay token (Tanssi) as foreign asset for fees
        assert_ok!(
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                RELAY_TOKEN_ASSET_LOCATION,
                RELAY_NATIVE_TOKEN_ASSET_ID,
                alice_account.clone().into(),
                true,
                1
            )
        );

        // Create asset rate for relay token
        assert_ok!(
            <FrontierTemplate as FrontierTemplateParaPallet>::AssetRate::create(
                root_origin.clone(),
                Box::new(RELAY_NATIVE_TOKEN_ASSET_ID),
                FixedU128::from_u32(500_000_000)
            )
        );

        // Fund Ethereum sovereign account ON THE CONTAINER with native balance
        // This simulates the container native tokens that were bridged to Ethereum and are now being sent back
        assert_ok!(
            <<FrontierTemplate as FrontierTemplateParaPallet>::Balances as Mutate<_>>::mint_into(
                &ethereum_sovereign_on_container,
                container_token_transfer_amount * 2
            )
        );

        receiver_native_balance_before =
            <FrontierTemplate as FrontierTemplateParaPallet>::Balances::free_balance(
                &token_receiver,
            );
    });

    // Send V2 inbound message
    Dancelight::execute_with(|| {
        // Get Tanssi token ID for fee asset
        let tanssi_token_id: [u8; 32] =
            snowbridge_pallet_system::NativeToForeignId::<dancelight_runtime::Runtime>::get(
                &Location::here()
                    .reanchored(&EthereumLocation::get(), &UniversalLocation::get())
                    .expect("unable to reanchor token"),
            )
            .unwrap()
            .into();

        // Total amount to withdraw from Ethereum sovereign for fees (relay + container execution)
        let relay_fee_amount = 10_000_000_000_000u128;

        // Build Tanssi fee asset for V2 message (kind: 1 = foreign token)
        let tanssi_fee_asset = IGatewayV2::AsForeignTokenERC20 {
            token_id: FixedBytes(tanssi_token_id),
            value: relay_fee_amount,
        };

        // Get container native token ID
        let container_token_id: [u8; 32] =
            snowbridge_pallet_system::NativeToForeignId::<dancelight_runtime::Runtime>::get(
                &container_native_token_location_relay
                    .clone()
                    .reanchored(&EthereumLocation::get(), &UniversalLocation::get())
                    .expect("unable to reanchor token"),
            )
            .unwrap()
            .into();

        // Build container native token asset for V2 message (kind: 1 = foreign token)
        let container_asset_v2 = IGatewayV2::AsForeignTokenERC20 {
            token_id: FixedBytes(container_token_id),
            value: container_token_transfer_amount,
        };

        // Fee asset for execution on container (relay token)
        let fee_asset =
            AssetId(Location::here()).into_asset(Fungibility::Fungible(dest_fee_amount));

        // Container native token reanchored to container's perspective (becomes Location::here())
        let container_native_reanchored = Location::new(0, PalletInstance(<<FrontierTemplate as FrontierTemplateParaPallet>::Balances as PalletInfoAccess>::index() as u8));
        let container_asset: Asset = (
            container_native_reanchored.clone(),
            container_token_transfer_amount,
        )
            .into();

        // Build the XCM following native_container_tokens_processor pattern
        let custom_xcm: Vec<Instruction<()>> = vec![InitiateTransfer {
            destination: Location::new(0, Parachain(container_para_id)),
            remote_fees: Some(AssetTransferFilter::ReserveDeposit(
                fee_asset.clone().into(),
            )),
            preserve_origin: true,
            assets: sp_runtime::BoundedVec::truncate_from(vec![]),
            remote_xcm: Xcm(vec![
                WithdrawAsset(vec![container_asset.clone()].into()),
                DepositAsset {
                    assets: Definite(container_asset.into()),
                    beneficiary: Location::new(
                        0,
                        AccountKey20 {
                            network: None,
                            key: token_receiver.into(),
                        },
                    ),
                },
            ]),
        }];

        let versioned_destination_xcm: VersionedXcm<()> = VersionedXcm::V5(Xcm(custom_xcm));
        let destination_xcm_bytes = RawPayload::Xcm(versioned_destination_xcm.encode());

        let gateway_address = EthereumGatewayAddress::get();

        // V2 message with relay fee token (Tanssi) and container native token in assets
        let event = IGatewayV2::OutboundMessageAccepted {
            nonce: 1u64,
            payload: IGatewayV2::Payload {
                origin: Address::from_slice(&gateway_address.as_bytes()),
                assets: vec![
                    // Relay fee token (Tanssi) for InitiateTransfer fees
                    IGatewayV2::EthereumAsset {
                        kind: 1,
                        data: tanssi_fee_asset.abi_encode().into(),
                    },
                    // Container native token for transfer
                    IGatewayV2::EthereumAsset {
                        kind: 1,
                        data: container_asset_v2.abi_encode().into(),
                    },
                ],
                xcm: IGatewayV2::Xcm {
                    kind: 0,
                    data: destination_xcm_bytes.encode().into(),
                },
                claimer: vec![].into(),
                value: 0,
                executionFee: 0,
                relayerFee: 0,
            },
        };

        assert_ok!(send_inbound_message_v2(event));

        // Check events in Dancelight
        type RuntimeEvent = <Dancelight as Chain>::RuntimeEvent;
        assert_expected_events!(
            Dancelight,
            vec![
                // EthereumInboundQueueV2 MessageReceived event
                RuntimeEvent::EthereumInboundQueueV2(
                    snowbridge_pallet_inbound_queue_v2::Event::MessageReceived { nonce: 1, .. }
                ) => {},
                // Tanssi fees minted to container sovereign account
                RuntimeEvent::Balances(
                    pallet_balances::Event::Minted { amount, .. }
                ) => {
                    amount: *amount == dest_fee_amount,
                },
            ]
        );
    });

    // Check container native token is received in container
    FrontierTemplate::execute_with(|| {
        let receiver_native_balance_after =
            <FrontierTemplate as FrontierTemplateParaPallet>::Balances::free_balance(
                &token_receiver,
            );

        assert!(
            receiver_native_balance_after > receiver_native_balance_before,
            "Receiver should have received container native tokens"
        );
        assert_eq!(
            receiver_native_balance_after - receiver_native_balance_before,
            container_token_transfer_amount,
            "Receiver should have received the exact transfer amount"
        );

        // Check events in FrontierTemplate
        type RuntimeEvent = <FrontierTemplate as Chain>::RuntimeEvent;
        assert_expected_events!(
            FrontierTemplate,
            vec![
                // Balances Minted event for container native tokens received
                RuntimeEvent::Balances(
                    pallet_balances::Event::Minted { who, amount }
                ) => {
                    who: *who == token_receiver,
                    amount: *amount == container_token_transfer_amount,
                },
            ]
        );
    });
}

/// Test: Container native token transfer from Ethereum back to Simple container chain via V2 inbound queue
/// This tests the scenario where a container's native token (bridged to Ethereum) is sent back to its origin chain
#[test]
fn check_container_native_to_simple_container_via_v2_works() {
    sp_tracing::try_init_simple();

    let token_receiver: [u8; 32] = [5u8; 32];
    let token_receiver_account: sp_runtime::AccountId32 = token_receiver.into();
    let container_para_id: u32 = <SimpleTemplate as xcm_emulator::Parachain>::para_id().into();

    let container_token_transfer_amount: u128 = 25_000_000_000_000; // Amount of container native token to transfer
    let dest_fee_amount: u128 = 5_000_000_000_000; // Relay tokens for container execution fees

    let mut receiver_native_balance_before = 0u128;

    // Container native token location from relay perspective (Simple template uses Parachain + PalletInstance for native token)
    let container_native_token_location_relay = Location::new(
        0,
        [
            Parachain(container_para_id),
            PalletInstance(
                <<SimpleTemplate as SimpleTemplateParaPallet>::Balances as PalletInfoAccess>::index(
                ) as u8,
            ),
        ],
    );

    // Get Ethereum network ID from SimpleTemplate
    let ethereum_network_id = SimpleTemplate::execute_with(|| {
        container_chain_template_simple_runtime::EthereumNetwork::get()
    });

    // Get Ethereum sovereign account on the container chain
    let ethereum_sovereign_on_container = SimpleTemplate::execute_with(|| {
        container_chain_template_simple_runtime::xcm_config::LocationToAccountId::convert_location(
            &Location::new(1, [GlobalConsensus(ethereum_network_id)]),
        )
        .unwrap()
    });

    // Setup on Dancelight
    Dancelight::execute_with(|| {
        let root_origin = <Dancelight as Chain>::RuntimeOrigin::root();

        // Register Tanssi native token with EthereumSystemV2 (for fees)
        let tanssi_location = Location::here();
        assert_ok!(dancelight_runtime::EthereumSystemV2::register_token(
            root_origin.clone(),
            Box::new(tanssi_location.clone().into()),
            Box::new(tanssi_location.clone().into()),
            snowbridge_core::AssetMetadata {
                name: "relay".as_bytes().to_vec().try_into().unwrap(),
                symbol: "relay".as_bytes().to_vec().try_into().unwrap(),
                decimals: 12,
            },
            1
        ));

        // Register container native token with EthereumSystemV2
        assert_ok!(dancelight_runtime::EthereumSystemV2::register_token(
            root_origin.clone(),
            Box::new(container_native_token_location_relay.clone().into()),
            Box::new(container_native_token_location_relay.clone().into()),
            snowbridge_core::AssetMetadata {
                name: "container".as_bytes().to_vec().try_into().unwrap(),
                symbol: "CTR".as_bytes().to_vec().try_into().unwrap(),
                decimals: 18,
            },
            2
        ));

        // Add funds to Ethereum sovereign account for relay token fees
        assert_ok!(
            <<Dancelight as DancelightRelayPallet>::Balances as Mutate<_>>::mint_into(
                &EthereumSovereignAccount::get(),
                100_000_000_000_000_000_000u128
            )
        );
    });

    // Setup on SimpleTemplate container
    SimpleTemplate::execute_with(|| {
        let root_origin = <SimpleTemplate as Chain>::RuntimeOrigin::root();
        let alice_account = SimpleTemplateSender::get();

        // Register relay token (Tanssi) as foreign asset for fees
        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                RELAY_TOKEN_ASSET_LOCATION,
                RELAY_NATIVE_TOKEN_ASSET_ID,
                alice_account.clone(),
                true,
                1
            )
        );

        // Create asset rate for relay token
        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::AssetRate::create(
                root_origin.clone(),
                Box::new(RELAY_NATIVE_TOKEN_ASSET_ID),
                FixedU128::from_u32(500_000_000)
            )
        );

        // Fund Ethereum sovereign account ON THE CONTAINER with native balance
        // This simulates the container native tokens that were bridged to Ethereum and are now being sent back
        assert_ok!(
            <<SimpleTemplate as SimpleTemplateParaPallet>::Balances as Mutate<_>>::mint_into(
                &ethereum_sovereign_on_container,
                container_token_transfer_amount * 2
            )
        );

        receiver_native_balance_before =
            <SimpleTemplate as SimpleTemplateParaPallet>::Balances::free_balance(
                &token_receiver_account,
            );
    });

    // Send V2 inbound message
    Dancelight::execute_with(|| {
        // Get Tanssi token ID for fee asset
        let tanssi_token_id: [u8; 32] =
            snowbridge_pallet_system::NativeToForeignId::<dancelight_runtime::Runtime>::get(
                &Location::here()
                    .reanchored(&EthereumLocation::get(), &UniversalLocation::get())
                    .expect("unable to reanchor token"),
            )
            .unwrap()
            .into();

        // Total amount to withdraw from Ethereum sovereign for fees (relay + container execution)
        let relay_fee_amount = 10_000_000_000_000u128;

        // Build Tanssi fee asset for V2 message (kind: 1 = foreign token)
        let tanssi_fee_asset = IGatewayV2::AsForeignTokenERC20 {
            token_id: FixedBytes(tanssi_token_id),
            value: relay_fee_amount,
        };

        // Get container native token ID
        let container_token_id: [u8; 32] =
            snowbridge_pallet_system::NativeToForeignId::<dancelight_runtime::Runtime>::get(
                &container_native_token_location_relay
                    .clone()
                    .reanchored(&EthereumLocation::get(), &UniversalLocation::get())
                    .expect("unable to reanchor token"),
            )
            .unwrap()
            .into();

        // Build container native token asset for V2 message (kind: 1 = foreign token)
        let container_asset_v2 = IGatewayV2::AsForeignTokenERC20 {
            token_id: FixedBytes(container_token_id),
            value: container_token_transfer_amount,
        };

        // Fee asset for execution on container (relay token)
        let fee_asset =
            AssetId(Location::here()).into_asset(Fungibility::Fungible(dest_fee_amount));

        // Container native token reanchored to container's perspective (becomes PalletInstance for simple template)
        let container_native_reanchored = Location::new(
            0,
            PalletInstance(
                <<SimpleTemplate as SimpleTemplateParaPallet>::Balances as PalletInfoAccess>::index(
                ) as u8,
            ),
        );
        let container_asset: Asset = (
            container_native_reanchored.clone(),
            container_token_transfer_amount,
        )
            .into();

        // Build the XCM following native_container_tokens_processor pattern
        let custom_xcm: Vec<Instruction<()>> = vec![InitiateTransfer {
            destination: Location::new(0, Parachain(container_para_id)),
            remote_fees: Some(AssetTransferFilter::ReserveDeposit(
                fee_asset.clone().into(),
            )),
            preserve_origin: true,
            assets: sp_runtime::BoundedVec::truncate_from(vec![]),
            remote_xcm: Xcm(vec![
                WithdrawAsset(vec![container_asset.clone()].into()),
                DepositAsset {
                    assets: Definite(container_asset.into()),
                    beneficiary: Location::new(
                        0,
                        xcm::latest::Junction::AccountId32 {
                            network: None,
                            id: token_receiver,
                        },
                    ),
                },
            ]),
        }];

        let versioned_destination_xcm: VersionedXcm<()> = VersionedXcm::V5(Xcm(custom_xcm));
        let destination_xcm_bytes = RawPayload::Xcm(versioned_destination_xcm.encode());

        let gateway_address = EthereumGatewayAddress::get();

        // V2 message with relay fee token (Tanssi) and container native token in assets
        let event = IGatewayV2::OutboundMessageAccepted {
            nonce: 1u64,
            payload: IGatewayV2::Payload {
                origin: Address::from_slice(&gateway_address.as_bytes()),
                assets: vec![
                    // Relay fee token (Tanssi) for InitiateTransfer fees
                    IGatewayV2::EthereumAsset {
                        kind: 1,
                        data: tanssi_fee_asset.abi_encode().into(),
                    },
                    // Container native token for transfer
                    IGatewayV2::EthereumAsset {
                        kind: 1,
                        data: container_asset_v2.abi_encode().into(),
                    },
                ],
                xcm: IGatewayV2::Xcm {
                    kind: 0,
                    data: destination_xcm_bytes.encode().into(),
                },
                claimer: vec![].into(),
                value: 0,
                executionFee: 0,
                relayerFee: 0,
            },
        };

        assert_ok!(send_inbound_message_v2(event));

        // Check events in Dancelight
        type RuntimeEvent = <Dancelight as Chain>::RuntimeEvent;
        assert_expected_events!(
            Dancelight,
            vec![
                // EthereumInboundQueueV2 MessageReceived event
                RuntimeEvent::EthereumInboundQueueV2(
                    snowbridge_pallet_inbound_queue_v2::Event::MessageReceived { nonce: 1, .. }
                ) => {},
                // Tanssi fees minted to container sovereign account
                RuntimeEvent::Balances(
                    pallet_balances::Event::Minted { amount, .. }
                ) => {
                    amount: *amount == dest_fee_amount,
                },
            ]
        );
    });

    // Check container native token is received in container
    SimpleTemplate::execute_with(|| {
        let receiver_native_balance_after =
            <SimpleTemplate as SimpleTemplateParaPallet>::Balances::free_balance(
                &token_receiver_account,
            );

        assert!(
            receiver_native_balance_after > receiver_native_balance_before,
            "Receiver should have received container native tokens"
        );
        assert_eq!(
            receiver_native_balance_after - receiver_native_balance_before,
            container_token_transfer_amount,
            "Receiver should have received the exact transfer amount"
        );

        // Check events in SimpleTemplate
        type RuntimeEvent = <SimpleTemplate as Chain>::RuntimeEvent;
        assert_expected_events!(
            SimpleTemplate,
            vec![
                // Balances Minted event for container native tokens received
                RuntimeEvent::Balances(
                    pallet_balances::Event::Minted { who, amount }
                ) => {
                    who: *who == token_receiver.into(),
                    amount: *amount == container_token_transfer_amount,
                },
            ]
        );
    });
}

#[test]
fn check_container_native_to_frontier_container_via_v2_fails_if_user_tries_draining_eth_acc() {
    sp_tracing::try_init_simple();

    let token_receiver: AccountId20 = [5u8; 20].into();
    let container_para_id: u32 = <FrontierTemplate as xcm_emulator::Parachain>::para_id().into();

    let container_token_transfer_amount: u128 = 25_000_000_000_000; // Amount of container native token to transfer
    let dest_fee_amount: u128 = 5_000_000_000_000; // Relay tokens for container execution fees

    let mut receiver_native_balance_before = 0u128;

    // Container native token location from relay perspective
    let container_native_token_location_relay = Location::new(0, [Parachain(container_para_id), PalletInstance(<<FrontierTemplate as FrontierTemplateParaPallet>::Balances as PalletInfoAccess>::index() as u8)]);

    // Get Ethereum network ID
    let ethereum_network_id = FrontierTemplate::execute_with(|| {
        container_chain_template_frontier_runtime::EthereumNetwork::get()
    });

    // Get Ethereum sovereign account on the container chain
    let ethereum_sovereign_on_container = FrontierTemplate::execute_with(|| {
        container_chain_template_frontier_runtime::xcm_config::LocationToAccountId::convert_location(
            &Location::new(1, [GlobalConsensus(ethereum_network_id)])
        ).unwrap()
    });

    // Setup on Dancelight
    Dancelight::execute_with(|| {
        let root_origin = <Dancelight as Chain>::RuntimeOrigin::root();

        // Register Tanssi native token with EthereumSystemV2 (for fees)
        let tanssi_location = Location::here();
        assert_ok!(dancelight_runtime::EthereumSystemV2::register_token(
            root_origin.clone(),
            Box::new(tanssi_location.clone().into()),
            Box::new(tanssi_location.clone().into()),
            snowbridge_core::AssetMetadata {
                name: "relay".as_bytes().to_vec().try_into().unwrap(),
                symbol: "relay".as_bytes().to_vec().try_into().unwrap(),
                decimals: 12,
            },
            1
        ));

        // Register container native token with EthereumSystemV2
        assert_ok!(dancelight_runtime::EthereumSystemV2::register_token(
            root_origin.clone(),
            Box::new(container_native_token_location_relay.clone().into()),
            Box::new(container_native_token_location_relay.clone().into()),
            snowbridge_core::AssetMetadata {
                name: "container".as_bytes().to_vec().try_into().unwrap(),
                symbol: "CTR".as_bytes().to_vec().try_into().unwrap(),
                decimals: 18,
            },
            2
        ));

        // Add funds to Ethereum sovereign account for relay token fees
        assert_ok!(
            <<Dancelight as DancelightRelayPallet>::Balances as Mutate<_>>::mint_into(
                &EthereumSovereignAccount::get(),
                100_000_000_000_000_000_000u128
            )
        );
    });

    // Setup on FrontierTemplate container
    FrontierTemplate::execute_with(|| {
        let root_origin = <FrontierTemplate as Chain>::RuntimeOrigin::root();
        let alice_account = FrontierTemplateSender::get();

        // Register relay token (Tanssi) as foreign asset for fees
        assert_ok!(
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                RELAY_TOKEN_ASSET_LOCATION,
                RELAY_NATIVE_TOKEN_ASSET_ID,
                alice_account.clone().into(),
                true,
                1
            )
        );

        // Create asset rate for relay token
        assert_ok!(
            <FrontierTemplate as FrontierTemplateParaPallet>::AssetRate::create(
                root_origin.clone(),
                Box::new(RELAY_NATIVE_TOKEN_ASSET_ID),
                FixedU128::from_u32(500_000_000)
            )
        );

        // Fund Ethereum sovereign account ON THE CONTAINER with native balance
        // This simulates the container native tokens that were bridged to Ethereum and are now being sent back
        assert_ok!(
            <<FrontierTemplate as FrontierTemplateParaPallet>::Balances as Mutate<_>>::mint_into(
                &ethereum_sovereign_on_container,
                container_token_transfer_amount * 2
            )
        );

        receiver_native_balance_before =
            <FrontierTemplate as FrontierTemplateParaPallet>::Balances::free_balance(
                &token_receiver,
            );
    });

    // Send V2 inbound message
    Dancelight::execute_with(|| {
        // Get Tanssi token ID for fee asset
        let tanssi_token_id: [u8; 32] =
            snowbridge_pallet_system::NativeToForeignId::<dancelight_runtime::Runtime>::get(
                &Location::here()
                    .reanchored(&EthereumLocation::get(), &UniversalLocation::get())
                    .expect("unable to reanchor token"),
            )
            .unwrap()
            .into();

        // Total amount to withdraw from Ethereum sovereign for fees (relay + container execution)
        let relay_fee_amount = 10_000_000_000_000u128;

        // Build Tanssi fee asset for V2 message (kind: 1 = foreign token)
        let tanssi_fee_asset = IGatewayV2::AsForeignTokenERC20 {
            token_id: FixedBytes(tanssi_token_id),
            value: relay_fee_amount,
        };

        // Get container native token ID
        let container_token_id: [u8; 32] =
            snowbridge_pallet_system::NativeToForeignId::<dancelight_runtime::Runtime>::get(
                &container_native_token_location_relay
                    .clone()
                    .reanchored(&EthereumLocation::get(), &UniversalLocation::get())
                    .expect("unable to reanchor token"),
            )
            .unwrap()
            .into();

        // Build container native token asset for V2 message (kind: 1 = foreign token)
        let container_asset_v2 = IGatewayV2::AsForeignTokenERC20 {
            token_id: FixedBytes(container_token_id),
            value: container_token_transfer_amount,
        };

        // Fee asset for execution on container (relay token)
        let fee_asset =
            AssetId(Location::here()).into_asset(Fungibility::Fungible(dest_fee_amount));

        // Container native token reanchored to container's perspective (becomes Location::here())
        let container_native_reanchored = Location::new(0, PalletInstance(<<FrontierTemplate as FrontierTemplateParaPallet>::Balances as PalletInfoAccess>::index() as u8));
        let container_asset: Asset = (
            container_native_reanchored.clone(),
            // User tries to drain the Ethereum sovereign account entirely
            container_token_transfer_amount * 2,
        )
            .into();

        // Build the XCM following native_container_tokens_processor pattern
        let custom_xcm: Vec<Instruction<()>> = vec![InitiateTransfer {
            destination: Location::new(0, Parachain(container_para_id)),
            remote_fees: Some(AssetTransferFilter::ReserveDeposit(
                fee_asset.clone().into(),
            )),
            preserve_origin: true,
            assets: sp_runtime::BoundedVec::truncate_from(vec![]),
            remote_xcm: Xcm(vec![
                WithdrawAsset(vec![container_asset.clone()].into()),
                DepositAsset {
                    assets: Definite(container_asset.into()),
                    beneficiary: Location::new(
                        0,
                        AccountKey20 {
                            network: None,
                            key: token_receiver.into(),
                        },
                    ),
                },
            ]),
        }];

        let versioned_destination_xcm: VersionedXcm<()> = VersionedXcm::V5(Xcm(custom_xcm));
        let destination_xcm_bytes = RawPayload::Xcm(versioned_destination_xcm.encode());

        // V2 message with relay fee token (Tanssi) and container native token in assets
        let event = IGatewayV2::OutboundMessageAccepted {
            nonce: 1u64,
            payload: IGatewayV2::Payload {
                // User tries to drain the Ethereum sovereign account on the container chain
                origin: Address::from_slice(&[5u8; 20]),
                assets: vec![
                    // Relay fee token (Tanssi) for InitiateTransfer fees
                    IGatewayV2::EthereumAsset {
                        kind: 1,
                        data: tanssi_fee_asset.abi_encode().into(),
                    },
                    // Container native token for transfer
                    IGatewayV2::EthereumAsset {
                        kind: 1,
                        data: container_asset_v2.abi_encode().into(),
                    },
                ],
                xcm: IGatewayV2::Xcm {
                    kind: 0,
                    data: destination_xcm_bytes.encode().into(),
                },
                claimer: vec![].into(),
                value: 0,
                executionFee: 0,
                relayerFee: 0,
            },
        };

        assert_ok!(send_inbound_message_v2(event));

        // Check events in Dancelight - message should still be received even though XCM will fail on container
        type RuntimeEvent = <Dancelight as Chain>::RuntimeEvent;
        assert_expected_events!(
            Dancelight,
            vec![
                // EthereumInboundQueueV2 MessageReceived event - message is received but XCM execution will fail
                RuntimeEvent::EthereumInboundQueueV2(
                    snowbridge_pallet_inbound_queue_v2::Event::MessageReceived { nonce: 1, .. }
                ) => {},
            ]
        );
    });

    // Check container native token is not received in container
    FrontierTemplate::execute_with(|| {
        let receiver_native_balance_after =
            <FrontierTemplate as FrontierTemplateParaPallet>::Balances::free_balance(
                &token_receiver,
            );

        assert_eq!(
            receiver_native_balance_after, receiver_native_balance_before,
            "Receiver should not have received container native tokens"
        );
        assert_eq!(
            receiver_native_balance_after, 0,
            "Receiver should not have received container native tokens"
        );

        // For this negative test, we verify no Balances::Minted event was emitted for the malicious transfer amount
        // The XCM execution should fail because the origin doesn't have permission to withdraw from Ethereum sovereign
        type RuntimeEvent = <FrontierTemplate as Chain>::RuntimeEvent;
        let events = <FrontierTemplate as FrontierTemplateParaPallet>::System::events();
        let balance_minted_events: Vec<_> = events
            .iter()
            .filter(|e| {
                matches!(
                    e.event,
                    RuntimeEvent::Balances(pallet_balances::Event::Minted { .. })
                )
            })
            .collect();
        assert!(
            balance_minted_events.is_empty(),
            "No Balances::Minted event should be emitted for failed drain attempt"
        );
    });
}
