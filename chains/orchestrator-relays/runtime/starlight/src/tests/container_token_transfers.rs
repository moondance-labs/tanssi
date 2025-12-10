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

#![cfg(test)]

use {
    crate::{
        bridge_to_ethereum_config::{EthereumGatewayAddress, InboundQueuePalletInstance},
        filter_events,
        tests::common::*,
        xcm_config::UniversalLocation,
        EthereumInboundQueue, EthereumLocation, EthereumSystem, EthereumTokenTransfers, Paras,
        RuntimeEvent, SnowbridgeFeesAccount, XcmPallet,
    },
    alloc::vec,
    alloy_sol_types::SolEvent,
    frame_support::{assert_noop, assert_ok},
    hex_literal::hex,
    parity_scale_codec::Encode,
    polkadot_parachain_primitives::primitives::HeadData,
    snowbridge_core::{AgentId, Channel, ChannelId, ParaId},
    snowbridge_inbound_queue_primitives::v1::{
        Command, Destination, Envelope, MessageProcessor, MessageV1, OutboundMessageAccepted,
        VersionedXcmMessage,
    },
    snowbridge_inbound_queue_primitives::{EventProof, Log},
    sp_core::{H160, H256},
    sp_runtime::traits::MaybeEquivalence,
    starlight_runtime_constants::snowbridge::EthereumNetwork,
    xcm::latest::{prelude::*, Asset as XcmAsset, Junctions::*, Location},
};

#[test]
fn receive_container_native_tokens_from_eth_works() {
    ExtBuilder::default()
        .with_balances(vec![
            (AccountId::from(ALICE), 100_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (
                SnowbridgeFeesAccount::get(),
                100_000_000_000_000_000_000 * UNIT,
            ),
        ])
        .build()
        .execute_with(|| {
            let relayer =
                <Runtime as frame_system::Config>::RuntimeOrigin::signed(AccountId::from(ALICE));

            let channel_id: ChannelId = ChannelId::new(hex!(
                "0000000000000000000000000000000000000000000000000000000000000004"
            ));
            let agent_id = AgentId::from(hex!(
                "0000000000000000000000000000000000000000000000000000000000000005"
            ));
            let para_id: ParaId = 2000u32.into();

            let amount_to_transfer = 100_000_000;
            let fee = 1_500_000_000_000_000;
            let container_fee = 500_000_000_000_000;

            let container_para_id = 2001u32;

            assert_ok!(XcmPallet::force_default_xcm_version(
                root_origin(),
                Some(5u32)
            ));

            assert_ok!(Paras::force_set_current_head(
                root_origin(),
                container_para_id.into(),
                HeadData::from(vec![1u8, 2u8, 3u8])
            ));

            assert_ok!(EthereumTokenTransfers::set_token_transfer_channel(
                root_origin(),
                channel_id,
                agent_id,
                para_id
            ));

            let token_location =
                Location::new(0, [Parachain(container_para_id), PalletInstance(10)]);

            assert_ok!(EthereumSystem::register_token(
                root_origin(),
                Box::new(token_location.clone().into()),
                snowbridge_core::AssetMetadata {
                    name: "para".as_bytes().to_vec().try_into().unwrap(),
                    symbol: "para".as_bytes().to_vec().try_into().unwrap(),
                    decimals: 12,
                }
            ));

            let token_location_reanchored = token_location
                .clone()
                .reanchored(&EthereumLocation::get(), &UniversalLocation::get())
                .expect("unable to reanchor token");

            let token_id = EthereumSystem::convert_back(&token_location_reanchored).unwrap();
            let beneficiary_key = [5u8; 20];
            let payload = VersionedXcmMessage::V1(MessageV1 {
                chain_id: 1,
                command: Command::SendNativeToken {
                    token_id,
                    destination: Destination::ForeignAccountId20 {
                        para_id: container_para_id,
                        id: beneficiary_key,
                        fee: container_fee,
                    },
                    amount: amount_to_transfer,
                    fee,
                },
            });

            let event = OutboundMessageAccepted {
                channel_id: <[u8; 32]>::from(channel_id).into(),
                nonce: 1,
                message_id: Default::default(),
                payload: payload.encode(),
            };

            let message = EventProof {
                event_log: Log {
                    address:
                        <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                    topics: event
                        .encode_topics()
                        .into_iter()
                        .map(|word| H256::from(word.0 .0))
                        .collect(),
                    data: event.encode_data(),
                },
                proof: mock_snowbridge_message_proof(),
            };

            assert_ok!(EthereumInboundQueue::submit(relayer, message.clone()));

            let tanssi_location = Location::here();
            let container_location = Location::new(0, [Parachain(container_para_id)]);
            let bridge_location = Location::new(2, GlobalConsensus(EthereumNetwork::get()));
            let inbound_queue_pallet_index = InboundQueuePalletInstance::get();
            let network = EthereumNetwork::get();
            let token_split = token_location_reanchored
                .interior()
                .clone()
                .split_global()
                .unwrap();
            let container_token_from_tanssi = Location::new(0, token_split.1);
            let container_token_location_reanchored = container_token_from_tanssi
                .reanchored(&container_location, &UniversalLocation::get())
                .unwrap();

            let container_asset: XcmAsset = (
                container_token_location_reanchored.clone(),
                amount_to_transfer,
            )
                .into();

            let container_asset_fee: XcmAsset = (Location::parent(), container_fee).into();

            let beneficiary = Location::new(
                0,
                [AccountKey20 {
                    network: None,
                    key: beneficiary_key,
                }],
            );

            let remote_xcm = Xcm::<()>(vec![
                ReserveAssetDeposited(vec![container_asset_fee.clone()].into()),
                BuyExecution {
                    fees: container_asset_fee,
                    weight_limit: Unlimited,
                },
                DescendOrigin(PalletInstance(inbound_queue_pallet_index).into()),
                UniversalOrigin(GlobalConsensus(network)),
                WithdrawAsset(vec![container_asset.clone()].into()),
                DepositAsset {
                    assets: Definite(container_asset.into()),
                    beneficiary,
                },
                SetAppendix(Xcm(vec![DepositAsset {
                    assets: Wild(AllOf {
                        id: Location::parent().into(),
                        fun: WildFungibility::Fungible,
                    }),
                    beneficiary: bridge_location,
                }])),
            ]);

            let xcm_sent_event = System::events()
                .iter()
                .filter(|r| match &r.event {
                    RuntimeEvent::XcmPallet(pallet_xcm::Event::Sent {
                        origin,
                        destination,
                        message,
                        message_id: _,
                    }) => {
                        *origin == tanssi_location
                            && *destination == container_location
                            && *message == remote_xcm
                    }
                    _ => false,
                })
                .count();

            assert_eq!(xcm_sent_event, 1, "XCM Sent event should be emitted!");
        });
}

#[test]
fn receive_container_native_tokens_from_eth_doesnt_error_if_error_sending_xcm() {
    ExtBuilder::default()
        .with_balances(vec![
            (AccountId::from(ALICE), 100_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            let relayer =
                <Runtime as frame_system::Config>::RuntimeOrigin::signed(AccountId::from(ALICE));

            let channel_id: ChannelId = ChannelId::new(hex!(
                "0000000000000000000000000000000000000000000000000000000000000004"
            ));
            let agent_id = AgentId::from(hex!(
                "0000000000000000000000000000000000000000000000000000000000000005"
            ));
            let para_id: ParaId = 2000u32.into();

            let amount_to_transfer = 100_000_000;
            let fee = 1_500_000_000_000_000;
            let container_fee = 500_000_000_000_000;

            let container_para_id = 2001u32;

            assert_ok!(XcmPallet::force_default_xcm_version(
                root_origin(),
                Some(5u32)
            ));

            // We don't set the current head on purpose, so the XCM sending will fail
            // assert_ok!(Paras::force_set_current_head(
            //     root_origin(),
            //     container_para_id.into(),
            //     HeadData::from(vec![1u8, 2u8, 3u8])
            // ));

            assert_ok!(EthereumTokenTransfers::set_token_transfer_channel(
                root_origin(),
                channel_id,
                agent_id,
                para_id
            ));

            let token_location =
                Location::new(0, [Parachain(container_para_id), PalletInstance(10)]);

            assert_ok!(EthereumSystem::register_token(
                root_origin(),
                Box::new(token_location.clone().into()),
                snowbridge_core::AssetMetadata {
                    name: "para".as_bytes().to_vec().try_into().unwrap(),
                    symbol: "para".as_bytes().to_vec().try_into().unwrap(),
                    decimals: 12,
                }
            ));

            let token_location_reanchored = token_location
                .clone()
                .reanchored(&EthereumLocation::get(), &UniversalLocation::get())
                .expect("unable to reanchor token");

            let token_id = EthereumSystem::convert_back(&token_location_reanchored).unwrap();
            let beneficiary_key = [5u8; 20];
            let payload = VersionedXcmMessage::V1(MessageV1 {
                chain_id: 1,
                command: Command::SendNativeToken {
                    token_id,
                    destination: Destination::ForeignAccountId20 {
                        para_id: container_para_id,
                        id: beneficiary_key,
                        fee: container_fee,
                    },
                    amount: amount_to_transfer,
                    fee,
                },
            });

            let event = OutboundMessageAccepted {
                channel_id: <[u8; 32]>::from(channel_id).into(),
                nonce: 1,
                message_id: Default::default(),
                payload: payload.encode(),
            };

            let message = EventProof {
                event_log: Log {
                    address:
                        <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                    topics: event
                        .encode_topics()
                        .into_iter()
                        .map(|word| H256::from(word.0 .0))
                        .collect(),
                    data: event.encode_data(),
                },
                proof: mock_snowbridge_message_proof(),
            };

            // This should fail to send the XCM, but should not error
            assert_ok!(EthereumInboundQueue::submit(relayer, message.clone()));

            assert_eq!(
                filter_events!(RuntimeEvent::XcmPallet(pallet_xcm::Event::Sent { .. })).count(),
                0,
                "XCM Sent event should NOT be emitted!"
            );
        });
}

#[test]
fn receive_container_native_tokens_fails_if_account_id_32() {
    ExtBuilder::default()
        .with_balances(vec![
            (AccountId::from(ALICE), 100_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            let relayer =
                <Runtime as frame_system::Config>::RuntimeOrigin::signed(AccountId::from(ALICE));

            let channel_id: ChannelId = ChannelId::new(hex!(
                "0000000000000000000000000000000000000000000000000000000000000004"
            ));
            let agent_id = AgentId::from(hex!(
                "0000000000000000000000000000000000000000000000000000000000000005"
            ));
            let para_id: ParaId = 2000u32.into();

            let amount_to_transfer = 100_000_000;
            let fee = 1_500_000_000_000_000;

            let container_para_id = 3000u32;

            assert_ok!(XcmPallet::force_default_xcm_version(
                root_origin(),
                Some(5u32)
            ));

            assert_ok!(Paras::force_set_current_head(
                root_origin(),
                container_para_id.into(),
                HeadData::from(vec![1u8, 2u8, 3u8])
            ));

            assert_ok!(EthereumTokenTransfers::set_token_transfer_channel(
                root_origin(),
                channel_id,
                agent_id,
                para_id
            ));

            let token_location =
                Location::new(0, [Parachain(container_para_id), PalletInstance(3)]);

            assert_ok!(EthereumSystem::register_token(
                root_origin(),
                Box::new(token_location.clone().into()),
                snowbridge_core::AssetMetadata {
                    name: "para".as_bytes().to_vec().try_into().unwrap(),
                    symbol: "para".as_bytes().to_vec().try_into().unwrap(),
                    decimals: 12,
                }
            ));

            let token_location_reanchored = token_location
                .reanchored(&EthereumLocation::get(), &UniversalLocation::get())
                .expect("unable to reanchor token");

            let token_id = EthereumSystem::convert_back(&token_location_reanchored).unwrap();
            let payload = VersionedXcmMessage::V1(MessageV1 {
                chain_id: 1,
                command: Command::SendNativeToken {
                    token_id,
                    destination: Destination::AccountId32 {
                        id: AccountId::from(BOB).into(),
                    },
                    amount: amount_to_transfer,
                    fee,
                },
            });

            let event = OutboundMessageAccepted {
                channel_id: <[u8; 32]>::from(channel_id).into(),
                nonce: 1,
                message_id: Default::default(),
                payload: payload.encode(),
            };

            let message = EventProof {
                event_log: Log {
                    address:
                        <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                    topics: event
                        .encode_topics()
                        .into_iter()
                        .map(|word| H256::from(word.0 .0))
                        .collect(),
                    data: event.encode_data(),
                },
                proof: mock_snowbridge_message_proof(),
            };

            assert_noop!(
                EthereumInboundQueue::submit(relayer, message.clone()),
                sp_runtime::DispatchError::Other("No handler for message found")
            );
        });
}

#[test]
fn receive_container_native_tokens_fails_if_token_not_registered() {
    ExtBuilder::default()
        .with_balances(vec![
            (AccountId::from(ALICE), 100_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            let relayer =
                <Runtime as frame_system::Config>::RuntimeOrigin::signed(AccountId::from(ALICE));

            let channel_id: ChannelId = ChannelId::new(hex!(
                "0000000000000000000000000000000000000000000000000000000000000004"
            ));
            let agent_id = AgentId::from(hex!(
                "0000000000000000000000000000000000000000000000000000000000000005"
            ));
            let para_id: ParaId = 2000u32.into();

            let amount_to_transfer = 100_000_000;
            let fee = 1_500_000_000_000_000;

            let container_para_id = 3000u32;

            assert_ok!(XcmPallet::force_default_xcm_version(
                root_origin(),
                Some(5u32)
            ));

            assert_ok!(Paras::force_set_current_head(
                root_origin(),
                container_para_id.into(),
                HeadData::from(vec![1u8, 2u8, 3u8])
            ));

            assert_ok!(EthereumTokenTransfers::set_token_transfer_channel(
                root_origin(),
                channel_id,
                agent_id,
                para_id
            ));

            // We don't register the token
            //
            // assert_ok!(EthereumSystem::register_token(
            //     root_origin(),
            //     Box::new(token_location.clone().into()),
            //     snowbridge_core::AssetMetadata {
            //         name: "para".as_bytes().to_vec().try_into().unwrap(),
            //         symbol: "para".as_bytes().to_vec().try_into().unwrap(),
            //         decimals: 12,
            //     }
            // ));

            // We generate a random token id, as we cannot fetch it from
            // EthereumSystem since the token is not registered.
            let token_id = H256::random();
            let payload = VersionedXcmMessage::V1(MessageV1 {
                chain_id: 1,
                command: Command::SendNativeToken {
                    token_id,
                    destination: Destination::AccountId32 {
                        id: AccountId::from(BOB).into(),
                    },
                    amount: amount_to_transfer,
                    fee,
                },
            });

            let event = OutboundMessageAccepted {
                channel_id: <[u8; 32]>::from(channel_id).into(),
                nonce: 1,
                message_id: Default::default(),
                payload: payload.encode(),
            };

            let message = EventProof {
                event_log: Log {
                    address:
                        <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                    topics: event
                        .encode_topics()
                        .into_iter()
                        .map(|word| H256::from(word.0 .0))
                        .collect(),
                    data: event.encode_data(),
                },
                proof: mock_snowbridge_message_proof(),
            };

            assert_noop!(
                EthereumInboundQueue::submit(relayer, message.clone()),
                sp_runtime::DispatchError::Other("No handler for message found")
            );
        });
}

#[test]
fn receive_container_native_tokens_fails_if_destination_doesnt_own_token() {
    ExtBuilder::default()
        .with_balances(vec![
            (AccountId::from(ALICE), 100_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            sp_tracing::try_init_simple();

            let relayer =
                <Runtime as frame_system::Config>::RuntimeOrigin::signed(AccountId::from(ALICE));

            let channel_id: ChannelId = ChannelId::new(hex!(
                "0000000000000000000000000000000000000000000000000000000000000004"
            ));
            let agent_id = AgentId::from(hex!(
                "0000000000000000000000000000000000000000000000000000000000000005"
            ));
            let para_id: ParaId = 2000u32.into();

            let amount_to_transfer = 100_000_000;
            let fee = 1_500_000_000_000_000;

            let container_para_id_1 = 3000u32;
            let container_fee = 500_000_000_000_000;

            assert_ok!(XcmPallet::force_default_xcm_version(
                root_origin(),
                Some(5u32)
            ));

            assert_ok!(Paras::force_set_current_head(
                root_origin(),
                container_para_id_1.into(),
                HeadData::from(vec![1u8, 2u8, 3u8])
            ));

            assert_ok!(EthereumTokenTransfers::set_token_transfer_channel(
                root_origin(),
                channel_id,
                agent_id,
                para_id
            ));

            let token_location =
                Location::new(0, [Parachain(container_para_id_1), PalletInstance(3)]);

            assert_ok!(EthereumSystem::register_token(
                root_origin(),
                Box::new(token_location.clone().into()),
                snowbridge_core::AssetMetadata {
                    name: "para".as_bytes().to_vec().try_into().unwrap(),
                    symbol: "para".as_bytes().to_vec().try_into().unwrap(),
                    decimals: 12,
                }
            ));

            // Now let's register another token, but with a different owner
            let container_para_id_2 = 4000u32;
            let token_location_2 =
                Location::new(0, [Parachain(container_para_id_2), PalletInstance(3)]);

            assert_ok!(EthereumSystem::register_token(
                root_origin(),
                Box::new(token_location_2.clone().into()),
                snowbridge_core::AssetMetadata {
                    name: "para2".as_bytes().to_vec().try_into().unwrap(),
                    symbol: "para2".as_bytes().to_vec().try_into().unwrap(),
                    decimals: 12,
                }
            ));

            let token_location_para_2_reanchored = token_location_2
                .reanchored(&EthereumLocation::get(), &UniversalLocation::get())
                .expect("unable to reanchor token");

            let token_id_para_2 =
                EthereumSystem::convert_back(&token_location_para_2_reanchored).unwrap();

            let payload = VersionedXcmMessage::V1(MessageV1 {
                chain_id: 1,
                command: Command::SendNativeToken {
                    token_id: token_id_para_2,
                    destination: Destination::ForeignAccountId32 {
                        para_id: container_para_id_1,
                        id: AccountId::from(BOB).into(),
                        fee: container_fee,
                    },
                    amount: amount_to_transfer,
                    fee,
                },
            });

            let event = OutboundMessageAccepted {
                channel_id: <[u8; 32]>::from(channel_id).into(),
                nonce: 1,
                message_id: Default::default(),
                payload: payload.encode(),
            };

            let message = EventProof {
                event_log: Log {
                    address:
                        <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                    topics: event
                        .encode_topics()
                        .into_iter()
                        .map(|word| H256::from(word.0 .0))
                        .collect(),
                    data: event.encode_data(),
                },
                proof: mock_snowbridge_message_proof(),
            };

            assert_noop!(
                EthereumInboundQueue::submit(relayer, message.clone()),
                sp_runtime::DispatchError::Other("No handler for message found")
            );
        });
}

#[cfg(not(feature = "runtime-benchmarks"))]
#[test]
fn native_container_can_process_message_returns_false_for_wrong_channel() {
    ExtBuilder::default().build().execute_with(|| {
        let channel_id = ChannelId::new([1; 32]);
        let agent_id = AgentId::from_low_u64_be(10);
        let para_id: ParaId = 2000u32.into();
        let container_para_id = 3000u32;

        // Register a container token in EthereumSystem
        let token_location = Location::new(0, [Parachain(container_para_id), PalletInstance(3)]);
        assert_ok!(EthereumSystem::register_token(
            root_origin(),
            Box::new(token_location.clone().into()),
            snowbridge_core::AssetMetadata {
                name: "para".as_bytes().to_vec().try_into().unwrap(),
                symbol: "para".as_bytes().to_vec().try_into().unwrap(),
                decimals: 12,
            }
        ));

        let token_location_reanchored = token_location
            .reanchored(&EthereumLocation::get(), &crate::xcm_config::UniversalLocation::get())
            .expect("unable to reanchor token");
        let token_id = EthereumSystem::convert_back(&token_location_reanchored).unwrap();

        // DO NOT register the channel with EthereumTokenTransfers to test failure
        // assert_ok!(EthereumTokenTransfers::set_token_transfer_channel(
        //     root_origin(),
        //     channel_id,
        //     agent_id,
        //     para_id
        // ));

        let channel = Channel { para_id, agent_id };

        // Create container token payload inline
        let payload = VersionedXcmMessage::V1(MessageV1 {
            chain_id: 1,
            command: Command::SendNativeToken {
                token_id,
                destination: Destination::ForeignAccountId32 {
                    para_id: container_para_id,
                    id: AccountId::from(ALICE).into(),
                    fee: 1000,
                },
                amount: 100,
                fee: 0,
            },
        }).encode();

        let envelope = Envelope {
            channel_id,
            gateway: EthereumGatewayAddress::get(),
            payload,
            nonce: 1,
            message_id: H256::zero(),
        };

        assert!(
            !<crate::bridge_to_ethereum_config::NativeContainerProcessor as MessageProcessor>::can_process_message(
                &channel, &envelope
            )
        );
    });
}

#[cfg(not(feature = "runtime-benchmarks"))]
#[test]
fn native_container_can_process_message_returns_false_for_wrong_gateway() {
    ExtBuilder::default().build().execute_with(|| {
        let channel_id = ChannelId::new([1; 32]);
        let agent_id = AgentId::from_low_u64_be(10);
        let para_id: ParaId = 2000u32.into();
        let container_para_id = 3000u32;

        // Register a container token in EthereumSystem
        let token_location = Location::new(0, [Parachain(container_para_id), PalletInstance(3)]);
        assert_ok!(EthereumSystem::register_token(
            root_origin(),
            Box::new(token_location.clone().into()),
            snowbridge_core::AssetMetadata {
                name: "para".as_bytes().to_vec().try_into().unwrap(),
                symbol: "para".as_bytes().to_vec().try_into().unwrap(),
                decimals: 12,
            }
        ));

        let token_location_reanchored = token_location
            .reanchored(&EthereumLocation::get(), &crate::xcm_config::UniversalLocation::get())
            .expect("unable to reanchor token");
        let token_id = EthereumSystem::convert_back(&token_location_reanchored).unwrap();

        // Register the channel with EthereumTokenTransfers
        assert_ok!(EthereumTokenTransfers::set_token_transfer_channel(
            root_origin(),
            channel_id,
            agent_id,
            para_id
        ));

        let channel = Channel { para_id, agent_id };

        // Create container token payload inline
        let payload = VersionedXcmMessage::V1(MessageV1 {
            chain_id: 1,
            command: Command::SendNativeToken {
                token_id,
                destination: Destination::ForeignAccountId32 {
                    para_id: container_para_id,
                    id: AccountId::from(ALICE).into(),
                    fee: 1000,
                },
                amount: 100,
                fee: 0,
            },
        }).encode();

        let envelope = Envelope {
            channel_id,
            gateway: H160::random(), // Wrong gateway address
            payload,
            nonce: 1,
            message_id: H256::zero(),
        };

        assert!(
            !<crate::bridge_to_ethereum_config::NativeContainerProcessor as MessageProcessor>::can_process_message(
                &channel, &envelope
            )
        );
    });
}

#[cfg(not(feature = "runtime-benchmarks"))]
#[test]
fn native_container_can_process_message_returns_true_for_valid_message() {
    ExtBuilder::default().build().execute_with(|| {
        let channel_id = ChannelId::new([1; 32]);
        let agent_id = AgentId::from_low_u64_be(10);
        let para_id: ParaId = 2000u32.into();
        let container_para_id = 3000u32;

        // Register a container token in EthereumSystem
        let token_location = Location::new(0, [Parachain(container_para_id), PalletInstance(3)]);
        assert_ok!(EthereumSystem::register_token(
            root_origin(),
            Box::new(token_location.clone().into()),
            snowbridge_core::AssetMetadata {
                name: "para".as_bytes().to_vec().try_into().unwrap(),
                symbol: "para".as_bytes().to_vec().try_into().unwrap(),
                decimals: 12,
            }
        ));

        let token_location_reanchored = token_location
            .reanchored(&EthereumLocation::get(), &crate::xcm_config::UniversalLocation::get())
            .expect("unable to reanchor token");
        let token_id = EthereumSystem::convert_back(&token_location_reanchored).unwrap();

        // Register the channel with EthereumTokenTransfers
        assert_ok!(EthereumTokenTransfers::set_token_transfer_channel(
            root_origin(),
            channel_id,
            agent_id,
            para_id
        ));

        let channel = Channel { para_id, agent_id };

        // Create container token payload with destination that owns the token (same para_id)
        let payload = VersionedXcmMessage::V1(MessageV1 {
            chain_id: 1,
            command: Command::SendNativeToken {
                token_id,
                destination: Destination::ForeignAccountId32 {
                    para_id: container_para_id, // Same para_id as token owner
                    id: AccountId::from(ALICE).into(),
                    fee: 1000,
                },
                amount: 100,
                fee: 0,
            },
        }).encode();

        let envelope = Envelope {
            channel_id,
            gateway: EthereumGatewayAddress::get(), // Correct gateway address
            payload,
            nonce: 1,
            message_id: H256::zero(),
        };

        // This should return true since all validations pass
        assert!(
            <crate::bridge_to_ethereum_config::NativeContainerProcessor as MessageProcessor>::can_process_message(
                &channel, &envelope
            )
        );
    });
}

#[test]
fn receive_container_foreign_tokens_from_eth_works_for_foreign_account_id_20() {
    ExtBuilder::default()
        .with_balances(vec![
            (AccountId::from(ALICE), 100_000 * UNIT),
            (
                SnowbridgeFeesAccount::get(),
                100_000_000_000_000_000_000 * UNIT,
            ),
        ])
        .build()
        .execute_with(|| {
            sp_tracing::try_init_simple();

            let relayer = RuntimeOrigin::signed(AccountId::from(ALICE));
            let para_id: ParaId = 2000u32.into();
            let container_para_id = 2001u32;

            assert_ok!(XcmPallet::force_default_xcm_version(
                root_origin(),
                Some(5u32)
            ));

            assert_ok!(Paras::force_set_current_head(
                root_origin(),
                container_para_id.into(),
                HeadData::from(vec![1u8, 2u8, 3u8])
            ));

            let channel_id = ChannelId::new([1u8; 32]);
            let agent_id = AgentId::from_low_u64_be(42);

            assert_ok!(EthereumTokenTransfers::set_token_transfer_channel(
                root_origin(),
                channel_id,
                agent_id,
                para_id
            ));

            let token_addr = H160::repeat_byte(0x11);
            let token_location = Location {
                parents: 1,
                interior: X2([
                    GlobalConsensus(EthereumNetwork::get()),
                    AccountKey20 {
                        network: Some(EthereumNetwork::get()),
                        key: token_addr.into(),
                    },
                ]
                .into()),
            };

            let asset_id = 42u16;

            assert_ok!(
                pallet_foreign_asset_creator::Pallet::<Runtime>::create_foreign_asset(
                    root_origin(),
                    token_location.clone(),
                    asset_id,
                    AccountId::from(ALICE),
                    true,
                    1
                )
            );

            let beneficiary = [5u8; 20];

            let amount_to_transfer = 100_000_000;
            let fee = 1_500_000_000_000_000;
            let container_fee = 500_000_000_000_000;

            let command = Command::SendToken {
                token: token_addr,
                destination: Destination::ForeignAccountId20 {
                    para_id: container_para_id,
                    id: beneficiary,
                    fee: container_fee,
                },
                amount: amount_to_transfer,
                fee,
            };

            let payload = VersionedXcmMessage::V1(MessageV1 {
                chain_id: 1,
                command: command.clone(),
            })
            .encode();

            let event = OutboundMessageAccepted {
                channel_id: <[u8; 32]>::from(channel_id).into(),
                nonce: 1,
                message_id: Default::default(),
                payload,
            };

            let message = EventProof {
                event_log: Log {
                    address:
                        <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                    topics: event
                        .encode_topics()
                        .into_iter()
                        .map(|w| H256::from(w.0 .0))
                        .collect(),
                    data: event.encode_data(),
                },
                proof: mock_snowbridge_message_proof(),
            };

            assert_ok!(EthereumInboundQueue::submit(relayer, message));

            let xcm_sent_event = System::events()
                .iter()
                .filter(|rec| {
                    is_expected_xcm_message_for_foreign_tokens(&rec.event, command.clone())
                })
                .count();
            assert_eq!(xcm_sent_event, 1, "XCM Sent event should be emitted!");
        });
}

#[test]
fn receive_container_foreign_tokens_from_eth_works_for_foreign_account_id_32() {
    ExtBuilder::default()
        .with_balances(vec![
            (AccountId::from(ALICE), 100_000 * UNIT),
            (
                SnowbridgeFeesAccount::get(),
                100_000_000_000_000_000_000 * UNIT,
            ),
        ])
        .build()
        .execute_with(|| {
            sp_tracing::try_init_simple();

            let relayer = RuntimeOrigin::signed(AccountId::from(ALICE));
            let para_id: ParaId = 2000u32.into();
            let container_para_id = 2001u32;

            assert_ok!(XcmPallet::force_default_xcm_version(
                root_origin(),
                Some(5u32)
            ));

            assert_ok!(Paras::force_set_current_head(
                root_origin(),
                container_para_id.into(),
                HeadData::from(vec![1u8, 2u8, 3u8])
            ));

            let channel_id = ChannelId::new([1u8; 32]);
            let agent_id = AgentId::from_low_u64_be(42);

            assert_ok!(EthereumTokenTransfers::set_token_transfer_channel(
                root_origin(),
                channel_id,
                agent_id,
                para_id
            ));

            let token_addr = H160::repeat_byte(0x11);
            let token_location = Location {
                parents: 1,
                interior: X2([
                    GlobalConsensus(EthereumNetwork::get()),
                    AccountKey20 {
                        network: Some(EthereumNetwork::get()),
                        key: token_addr.into(),
                    },
                ]
                .into()),
            };

            let asset_id = 42u16;

            assert_ok!(
                pallet_foreign_asset_creator::Pallet::<Runtime>::create_foreign_asset(
                    root_origin(),
                    token_location.clone(),
                    asset_id,
                    AccountId::from(ALICE),
                    true,
                    1
                )
            );

            let beneficiary = [5u8; 32];

            let amount_to_transfer = 100_000_000;
            let fee = 1_500_000_000_000_000;
            let container_fee = 2_000_000_000_000_000;

            let command = Command::SendToken {
                token: token_addr,
                destination: Destination::ForeignAccountId32 {
                    para_id: container_para_id,
                    id: beneficiary,
                    fee: container_fee,
                },
                amount: amount_to_transfer,
                fee,
            };

            let payload = VersionedXcmMessage::V1(MessageV1 {
                chain_id: 1,
                command: command.clone(),
            })
            .encode();

            let event = OutboundMessageAccepted {
                channel_id: <[u8; 32]>::from(channel_id).into(),
                nonce: 1,
                message_id: Default::default(),
                payload,
            };

            let message = EventProof {
                event_log: Log {
                    address:
                        <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                    topics: event
                        .encode_topics()
                        .into_iter()
                        .map(|w| H256::from(w.0 .0))
                        .collect(),
                    data: event.encode_data(),
                },
                proof: mock_snowbridge_message_proof(),
            };

            assert_ok!(EthereumInboundQueue::submit(relayer, message));

            let xcm_sent_event = System::events()
                .iter()
                .filter(|rec| {
                    is_expected_xcm_message_for_foreign_tokens(&rec.event, command.clone())
                })
                .count();
            assert_eq!(xcm_sent_event, 1, "XCM Sent event should be emitted!");
        });
}

#[test]
fn receive_container_foreign_tokens_from_eth_without_para_head_set_doesnt_error() {
    ExtBuilder::default().build().execute_with(|| {
        let relayer = RuntimeOrigin::signed(AccountId::from(ALICE));
        let para_id: ParaId = 2000u32.into();
        let container_para_id = 2001u32;

        assert_ok!(XcmPallet::force_default_xcm_version(
            root_origin(),
            Some(5u32)
        ));

        let channel_id = ChannelId::new([2u8; 32]);
        let agent_id = AgentId::from_low_u64_be(43);
        assert_ok!(EthereumTokenTransfers::set_token_transfer_channel(
            root_origin(),
            channel_id,
            agent_id,
            para_id
        ));

        let token_addr = H160::repeat_byte(0x22);
        let token_location = Location {
            parents: 1,
            interior: X2([
                GlobalConsensus(EthereumNetwork::get()),
                AccountKey20 {
                    network: Some(EthereumNetwork::get()),
                    key: token_addr.into(),
                },
            ]
            .into()),
        };

        let asset_id = 42u16;

        assert_ok!(
            pallet_foreign_asset_creator::Pallet::<Runtime>::create_foreign_asset(
                root_origin(),
                token_location.clone(),
                asset_id,
                AccountId::from(ALICE),
                true,
                1
            )
        );

        let beneficiary = [5u8; 20];
        let amount_to_transfer = 100_000_000u128;
        let fee = 1_500_000_000_000_000u128;
        let container_fee = 500_000_000_000_000u128;

        let payload = VersionedXcmMessage::V1(MessageV1 {
            chain_id: 1,
            command: Command::SendToken {
                token: token_addr,
                destination: Destination::ForeignAccountId20 {
                    para_id: container_para_id,
                    id: beneficiary,
                    fee: container_fee,
                },
                amount: amount_to_transfer,
                fee,
            },
        })
        .encode();

        let event = OutboundMessageAccepted {
            channel_id: <[u8; 32]>::from(channel_id).into(),
            nonce: 1,
            message_id: Default::default(),
            payload,
        };

        let message = EventProof {
            event_log: Log {
                address: <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(
                ),
                topics: event
                    .encode_topics()
                    .into_iter()
                    .map(|w| H256::from(w.0 .0))
                    .collect(),
                data: event.encode_data(),
            },
            proof: mock_snowbridge_message_proof(),
        };

        assert_ok!(EthereumInboundQueue::submit(relayer, message));

        let sent = System::events().iter().any(|r| {
            matches!(
                r.event,
                RuntimeEvent::XcmPallet(pallet_xcm::Event::Sent { .. })
            )
        });
        assert!(!sent, "XCM Sent event should NOT be emitted!");
    });
}

fn is_expected_xcm_message_for_foreign_tokens(event: &RuntimeEvent, command: Command) -> bool {
    let Command::SendToken {
        token,
        destination,
        amount,
        ..
    } = command
    else {
        log::error!(
            "is_expected_xcm_message: unsupported command type: {:?}",
            command
        );
        return false;
    };

    let (container_para_id, beneficiary, container_fee) = match destination {
        Destination::ForeignAccountId32 { para_id, id, fee } => (
            para_id,
            Location::new(0, [AccountId32 { network: None, id }]),
            fee,
        ),
        Destination::ForeignAccountId20 { para_id, id, fee } => (
            para_id,
            Location::new(
                0,
                [AccountKey20 {
                    network: None,
                    key: id,
                }],
            ),
            fee,
        ),
        _ => {
            log::error!(
                "is_expected_message: unsupported destination type: {:?}",
                destination
            );
            return false;
        }
    };

    let tanssi_location = Location::here();
    let container_location = Location::new(0, [Parachain(container_para_id)]);

    let asset_fee_container: Asset = (Location::parent(), container_fee).into();
    let erc20_asset_location: Location = Location {
        parents: 2,
        interior: X2([
            GlobalConsensus(EthereumNetwork::get()),
            AccountKey20 {
                network: Some(EthereumNetwork::get()),
                key: token.into(),
            },
        ]
        .into()),
    };
    let amount_to_transfer = amount;
    let asset_to_deposit: Asset = (erc20_asset_location, amount_to_transfer).into();

    let remote_xcm = Xcm::<()>(vec![
        ReserveAssetDeposited(vec![asset_fee_container.clone(), asset_to_deposit.clone()].into()),
        BuyExecution {
            fees: asset_fee_container.clone(),
            weight_limit: Unlimited,
        },
        DepositAsset {
            assets: Definite(vec![asset_to_deposit].into()),
            beneficiary,
        },
    ]);

    match event {
        RuntimeEvent::XcmPallet(pallet_xcm::Event::Sent {
            origin,
            destination,
            message,
            message_id: _,
        }) => {
            return *origin == tanssi_location
                && *destination == container_location
                && *message == remote_xcm
        }
        _ => return false,
    };
}
