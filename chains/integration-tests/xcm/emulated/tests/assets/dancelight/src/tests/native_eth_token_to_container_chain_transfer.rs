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
    dancelight_emulated_chain::DancelightRelayPallet,
    dancelight_system_emulated_network::{
        DancelightRelay as Dancelight, DancelightSender, FrontierTemplatePara as FrontierTemplate,
        SimpleTemplatePara as SimpleTemplate, SimpleTemplateReceiver, SimpleTemplateSender,
    },
    fp_account::AccountId20,
    frame_support::{assert_ok, pallet_prelude::DispatchResult, traits::PalletInfoAccess},
    frontier_template_emulated_chain::{EthereumSender, FrontierTemplateParaPallet},
    hex_literal::hex,
    simple_template_emulated_chain::SimpleTemplateParaPallet,
    snowbridge_beacon_primitives::{
        types::deneb, AncestryProof, BeaconHeader, ExecutionProof, VersionedExecutionPayloadHeader,
    },
    snowbridge_core::ChannelId,
    snowbridge_inbound_queue_primitives::{EventFixture, EventProof, Log, Proof},
    sp_core::H160,
    sp_core::U256,
    sp_runtime::AccountId32,
    xcm::latest::prelude::*,
    xcm_emulator::{Chain, TestExt},
    xcm_executor::traits::ConvertLocation,
};

#[test]
fn check_native_eth_token_to_frontier_container_chain_transfer_works() {
    const PARA_ID_FOR_CHANNEL: u32 = 2000;

    const CONTAINER_PARA_ID: u32 = 2001;

    let token_receiver: AccountId20 = H160([
        0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05,
        0x05, 0x05, 0x05, 0x05, 0x05,
    ])
    .into();

    let mut ethereum_sovereign_container_balance_before = 0;
    let mut receiver_native_container_balance_before = 0;

    let ethereum_sovereign_account_address = FrontierTemplate::execute_with(|| {
        container_chain_template_frontier_runtime::xcm_config::LocationToAccountId::convert_location(
            &Location::new(2, container_chain_template_frontier_runtime::EthereumNetwork::get()),
        )
            .unwrap()
    });

    let transfer_amount = 100_000_000;

    // Amount in native container tokens to charge on destination.
    let container_fee = 500_000_000_000_000;

    Dancelight::execute_with(|| {
        let root_origin = <Dancelight as Chain>::RuntimeOrigin::root();

        let asset_location = Location {
            parents: 0,
            interior: Junctions::X2([Parachain(CONTAINER_PARA_ID), PalletInstance(<<FrontierTemplate as FrontierTemplateParaPallet>::Balances as PalletInfoAccess>::index() as u8)].into()),
        };

        assert_ok!(
            <Dancelight as DancelightRelayPallet>::EthereumTokenTransfers::set_token_transfer_channel(
                root_origin.clone(),
                ChannelId::new(hex!("0000000000000000000000000000000000000000000000000000000000000004")), 
                hex!("0000000000000000000000000000000000000000000000000000000000000005").into(), 
                PARA_ID_FOR_CHANNEL.into()),
            );

        assert_ok!(
            <Dancelight as DancelightRelayPallet>::EthereumSystem::register_token(
                root_origin.clone(),
                Box::new(asset_location.into()),
                snowbridge_core::AssetMetadata {
                    name: "container".as_bytes().to_vec().try_into().unwrap(),
                    symbol: "container".as_bytes().to_vec().try_into().unwrap(),
                    decimals: 12,
                }
            )
        );
    });

    FrontierTemplate::execute_with(|| {
        let origin = <FrontierTemplate as Chain>::RuntimeOrigin::signed(EthereumSender::get());

        assert_ok!(
            <FrontierTemplate as FrontierTemplateParaPallet>::Balances::transfer_allow_death(
                origin,
                ethereum_sovereign_account_address,
                20_000_000_000_000_000
            )
        );

        ethereum_sovereign_container_balance_before =
            <FrontierTemplate as FrontierTemplateParaPallet>::System::account(
                ethereum_sovereign_account_address,
            )
            .data
            .free;

        receiver_native_container_balance_before =
            <FrontierTemplate as FrontierTemplateParaPallet>::System::account(token_receiver)
                .data
                .free;
    });

    Dancelight::execute_with(|| {
        assert_ok!(send_inbound_message(
            make_send_token_message_frontier_template()
        ));
    });

    FrontierTemplate::execute_with(|| {
        let ethereum_sovereign_container_balance_after =
            <FrontierTemplate as FrontierTemplateParaPallet>::System::account(
                ethereum_sovereign_account_address,
            )
            .data
            .free;

        let receiver_native_countainer_balance_after =
            <FrontierTemplate as FrontierTemplateParaPallet>::System::account(token_receiver)
                .data
                .free;

        assert_eq!(
            ethereum_sovereign_container_balance_after,
            ethereum_sovereign_container_balance_before - (container_fee + transfer_amount)
        );

        assert_eq!(
            receiver_native_countainer_balance_after,
            receiver_native_container_balance_before + transfer_amount
        );
    });
}

#[test]
fn check_native_eth_token_to_simple_container_chain_transfer_works() {
    const PARA_ID_FOR_CHANNEL: u32 = 2000;

    const CONTAINER_PARA_ID: u32 = 2002;

    let token_receiver: AccountId32 = SimpleTemplateReceiver::get();

    let mut ethereum_sovereign_container_balance_before = 0;
    let mut receiver_native_container_balance_before = 0;

    let ethereum_sovereign_account_address = SimpleTemplate::execute_with(|| {
        container_chain_template_simple_runtime::xcm_config::LocationToAccountId::convert_location(
            &Location::new(
                2,
                container_chain_template_simple_runtime::EthereumNetwork::get(),
            ),
        )
        .unwrap()
    });

    let transfer_amount = 100_000_000;

    // Amount in native container tokens to charge on destination.
    let container_fee = 500_000_000_000_000;

    Dancelight::execute_with(|| {
        let root_origin = <Dancelight as Chain>::RuntimeOrigin::root();

        let asset_location = Location {
            parents: 0,
            interior: Junctions::X2([Parachain(CONTAINER_PARA_ID), PalletInstance(<<SimpleTemplate as SimpleTemplateParaPallet>::Balances as PalletInfoAccess>::index() as u8)].into()),
        };

        assert_ok!(
            <Dancelight as DancelightRelayPallet>::EthereumTokenTransfers::set_token_transfer_channel(
                root_origin.clone(),
                ChannelId::new(hex!("0000000000000000000000000000000000000000000000000000000000000004")),
                hex!("0000000000000000000000000000000000000000000000000000000000000005").into(),
                PARA_ID_FOR_CHANNEL.into()),
            );

        assert_ok!(
            <Dancelight as DancelightRelayPallet>::EthereumSystem::register_token(
                root_origin.clone(),
                Box::new(asset_location.into()),
                snowbridge_core::AssetMetadata {
                    name: "container".as_bytes().to_vec().try_into().unwrap(),
                    symbol: "container".as_bytes().to_vec().try_into().unwrap(),
                    decimals: 12,
                }
            )
        );
    });

    SimpleTemplate::execute_with(|| {
        let origin = <SimpleTemplate as Chain>::RuntimeOrigin::signed(SimpleTemplateSender::get());

        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::Balances::transfer_allow_death(
                origin.clone(),
                sp_runtime::MultiAddress::Id(ethereum_sovereign_account_address.clone()),
                20_000_000_000_000_000
            )
        );

        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::Balances::transfer_allow_death(
                origin,
                sp_runtime::MultiAddress::Id(token_receiver.clone()),
                100_000_000_000
            )
        );

        ethereum_sovereign_container_balance_before =
            <SimpleTemplate as SimpleTemplateParaPallet>::System::account(
                ethereum_sovereign_account_address.clone(),
            )
            .data
            .free;

        receiver_native_container_balance_before =
            <SimpleTemplate as SimpleTemplateParaPallet>::System::account(&token_receiver)
                .data
                .free;
    });

    Dancelight::execute_with(|| {
        assert_ok!(send_inbound_message(
            make_send_token_message_simple_template()
        ));
    });

    SimpleTemplate::execute_with(|| {
        let ethereum_sovereign_container_balance_after =
            <SimpleTemplate as SimpleTemplateParaPallet>::System::account(
                ethereum_sovereign_account_address.clone(),
            )
            .data
            .free;

        let receiver_native_countainer_balance_after =
            <SimpleTemplate as SimpleTemplateParaPallet>::System::account(&token_receiver)
                .data
                .free;

        assert_eq!(
            ethereum_sovereign_container_balance_after,
            ethereum_sovereign_container_balance_before - (container_fee + transfer_amount)
        );

        assert_eq!(
            receiver_native_countainer_balance_after,
            receiver_native_container_balance_before + transfer_amount
        );
    });
}

pub fn make_send_token_message_simple_template() -> EventFixture {
    make_send_token_fixture(hex!("00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000007f00010000000000000002c97f6a848a8e7895b55dc9b894e4f552ea33203bfbdb478d506f05b62d9d5fd101d2070000050505050505050505050505050505050505050505050505050505050505050500406352bfc60100000000000000000000e1f50500000000000000000000000000c029f73d540500000000000000000000").into())
}

pub fn make_send_token_message_frontier_template() -> EventFixture {
    make_send_token_fixture(hex!("00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000007300010000000000000002485f805cb9de38b4324485447c664e16035aa9d28e8723df192fa02ad353088902d1070000050505050505050505050505050505050505050500406352bfc60100000000000000000000e1f50500000000000000000000000000c029f73d540500000000000000000000000000000000000000000000").into())
}

pub fn make_send_token_fixture(data: Vec<u8>) -> EventFixture {
    EventFixture {
        event: EventProof {
            event_log: Log {
                address: hex!("eda338e4dc46038493b885327842fd3e301cab39").into(),
                topics: vec![
                    hex!("7153f9357c8ea496bba60bf82e67143e27b64462b49041f8e689e1b05728f84f").into(), // Signature for event OutboundMessageAccepted(bytes32 indexed channel_id, uint64 nonce, bytes32 indexed message_id, bytes payload);
                    hex!("0000000000000000000000000000000000000000000000000000000000000004").into(), // channel ID
                    hex!("0000000000000000000000000000000000000000000000000000000000000000").into(), // message ID
                ],
                data,
            },
            // We just need a valid struct here, anyway we have mocked Verifier, that will ignore it
            proof: Proof {
                receipt_proof: (vec![
                    hex!("dccdfceea05036f7b61dcdabadc937945d31e68a8d3dfd4dc85684457988c284").to_vec(),
                    hex!("4a98e45a319168b0fc6005ce6b744ee9bf54338e2c0784b976a8578d241ced0f").to_vec(),
                ], vec![
                    hex!("f851a09c01dd6d2d8de951c45af23d3ad00829ce021c04d6c8acbe1612d456ee320d4980808080808080a04a98e45a319168b0fc6005ce6b744ee9bf54338e2c0784b976a8578d241ced0f8080808080808080").to_vec(),
                    hex!("f9028c30b9028802f90284018301d205b9010000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000080000000000000000000000000000004000000000080000000000000000000000000000000000010100000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000040004000000000000002000002000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000200000000000010f90179f85894eda338e4dc46038493b885327842fd3e301cab39e1a0f78bb28d4b1d7da699e5c0bc2be29c2b04b5aab6aacf6298fe5304f9db9c6d7ea000000000000000000000000087d1f7fdfee7f651fabc8bfcb6e086c278b77a7df9011c94eda338e4dc46038493b885327842fd3e301cab39f863a07153f9357c8ea496bba60bf82e67143e27b64462b49041f8e689e1b05728f84fa0c173fac324158e77fb5840738a1a541f633cbec8884c6a601c567d2b376a0539a05f7060e971b0dc81e63f0aa41831091847d97c1a4693ac450cc128c7214e65e0b8a000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000002e00a736aa00000000000087d1f7fdfee7f651fabc8bfcb6e086c278b77a7d00e40b54020000000000000000000000000000000000000000000000000000000000").to_vec(),
                ]),
                execution_proof: ExecutionProof {
                    header: BeaconHeader {
                        slot: 393,
                        proposer_index: 4,
                        parent_root: hex!("6545b47a614a1dd4cad042a0cdbbf5be347e8ffcdc02c6c64540d5153acebeef").into(),
                        state_root: hex!("b62ac34a8cb82497be9542fe2114410c9f6021855b766015406101a1f3d86434").into(),
                        body_root: hex!("04005fe231e11a5b7b1580cb73b177ae8b338bedd745497e6bb7122126a806db").into(),
                    },
                    ancestry_proof: Some(AncestryProof {
                        header_branch: vec![
                            hex!("6545b47a614a1dd4cad042a0cdbbf5be347e8ffcdc02c6c64540d5153acebeef").into(),
                            hex!("fa84cc88ca53a72181599ff4eb07d8b444bce023fe2347c3b4f51004c43439d3").into(),
                            hex!("cadc8ae211c6f2221c9138e829249adf902419c78eb4727a150baa4d9a02cc9d").into(),
                            hex!("33a89962df08a35c52bd7e1d887cd71fa7803e68787d05c714036f6edf75947c").into(),
                            hex!("2c9760fce5c2829ef3f25595a703c21eb22d0186ce223295556ed5da663a82cf").into(),
                            hex!("e1aa87654db79c8a0ecd6c89726bb662fcb1684badaef5cd5256f479e3c622e1").into(),
                            hex!("aa70d5f314e4a1fbb9c362f3db79b21bf68b328887248651fbd29fc501d0ca97").into(),
                            hex!("160b6c235b3a1ed4ef5f80b03ee1c76f7bf3f591c92fca9d8663e9221b9f9f0f").into(),
                            hex!("f68d7dcd6a07a18e9de7b5d2aa1980eb962e11d7dcb584c96e81a7635c8d2535").into(),
                            hex!("1d5f912dfd6697110dd1ecb5cb8e77952eef57d85deb373572572df62bb157fc").into(),
                            hex!("ffff0ad7e659772f9534c195c815efc4014ef1e1daed4404c06385d11192e92b").into(),
                            hex!("6cf04127db05441cd833107a52be852868890e4317e6a02ab47683aa75964220").into(),
                            hex!("b7d05f875f140027ef5118a2247bbb84ce8f2f0f1123623085daf7960c329f5f").into(),
                        ],
                        finalized_block_root: hex!("751414cd97c0624f922b3e80285e9f776b08fa22fd5f87391f2ed7ef571a8d46").into(),
                    }),
                    execution_header: VersionedExecutionPayloadHeader::Deneb(deneb::ExecutionPayloadHeader {
                        parent_hash: hex!("8092290aa21b7751576440f77edd02a94058429ce50e63a92d620951fb25eda2").into(),
                        fee_recipient: hex!("0000000000000000000000000000000000000000").into(),
                        state_root: hex!("96a83e9ddf745346fafcb0b03d57314623df669ed543c110662b21302a0fae8b").into(),
                        receipts_root: hex!("dccdfceea05036f7b61dcdabadc937945d31e68a8d3dfd4dc85684457988c284").into(),
                        logs_bloom: hex!("00000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000080000000400000000000000000000004000000000080000000000000000000000000000000000010100000000000000000000000000000000020000000000000000000000000000000000080000000000000000000000000000040004000000000000002002002000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000080000000000000000000000000000000000100000000000000000200000200000010").into(),
                        prev_randao: hex!("62e309d4f5119d1f5c783abc20fc1a549efbab546d8d0b25ff1cfd58be524e67").into(),
                        block_number: 393,
                        gas_limit: 54492273,
                        gas_used: 199644,
                        timestamp: 1710552813,
                        extra_data: hex!("d983010d0b846765746888676f312e32312e368664617277696e").into(),
                        base_fee_per_gas: U256::from(7u64),
                        block_hash: hex!("6a9810efb9581d30c1a5c9074f27c68ea779a8c1ae31c213241df16225f4e131").into(),
                        transactions_root: hex!("2cfa6ed7327e8807c7973516c5c32a68ef2459e586e8067e113d081c3bd8c07d").into(),
                        withdrawals_root: hex!("792930bbd5baac43bcc798ee49aa8185ef76bb3b44ba62b91d86ae569e4bb535").into(),
                        blob_gas_used: 0,
                        excess_blob_gas: 0,
                    }),
                    execution_branch: vec![
                        hex!("a6833fa629f3286b6916c6e50b8bf089fc9126bee6f64d0413b4e59c1265834d").into(),
                        hex!("b46f0c01805fe212e15907981b757e6c496b0cb06664224655613dcec82505bb").into(),
                        hex!("db56114e00fdd4c1f85c892bf35ac9a89289aaecb1ebd0a96cde606a748b5d71").into(),
                        hex!("d3af7c05c516726be7505239e0b9c7cb53d24abce6b91cdb3b3995f0164a75da").into(),
                    ],
                },
            },
        },
        finalized_header: BeaconHeader {
            slot: 864,
            proposer_index: 4,
            parent_root: hex!("614e7672f991ac268cd841055973f55e1e42228831a211adef207bb7329be614").into(),
            state_root: hex!("5fa8dfca3d760e4242ab46d529144627aa85348a19173b6e081172c701197a4a").into(),
            body_root: hex!("0f34c083b1803666bb1ac5e73fa71582731a2cf37d279ff0a3b0cad5a2ff371e").into(),
        },
        block_roots_root: hex!("b9aab9c388c4e4fcd899b71f62c498fc73406e38e8eb14aa440e9affa06f2a10").into(),
    }
}

pub fn send_inbound_message(fixture: EventFixture) -> DispatchResult {
    dancelight_runtime::EthereumBeaconClient::store_finalized_header(
        fixture.finalized_header,
        fixture.block_roots_root,
    )
    .unwrap();
    <Dancelight as DancelightRelayPallet>::EthereumInboundQueue::submit(
        <Dancelight as Chain>::RuntimeOrigin::signed(DancelightSender::get()),
        fixture.event,
    )
}
