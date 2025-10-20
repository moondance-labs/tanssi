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

use frame_support::assert_noop;
use snowbridge_core::PRIMARY_GOVERNANCE_CHANNEL;
use snowbridge_outbound_queue_primitives::v1::AgentExecuteCommand;
use {super::*, hex_literal::hex};

#[test]
fn test_command_encoding() {
    let command = Command::Test(b"Hello, world!".to_vec());

    let expected = hex!(
        "0000000000000000000000000000000000000000000000000000000000000020" // tuple offset
        "0000000000000000000000000000000000000000000000000000000000000020" // bytes offset in tuple
        "000000000000000000000000000000000000000000000000000000000000000d" // bytes size
        "48656C6C6F2C20776F726C642100000000000000000000000000000000000000" // bytes
    );

    assert_eq!(command.abi_encode(), expected);
}

#[test]
fn test_report_rewards_encoding() {
    let command = Command::ReportRewards {
        external_idx: 123_456_789,
        era_index: 42,
        total_points: 123_456_789_012_345,
        tokens_inflated: 987_654_321_098,
        rewards_merkle_root: H256::from(hex!(
            "b6e16d27ac5ab427a7f68900ac5559ce272dc6c37c82b3e052246c82244c50e4"
        )),
        token_id: H256::repeat_byte(0x01),
    };

    let expected = hex!(
        // no tuple offset since all fields have static size
        "00000000000000000000000000000000000000000000000000000000075BCD15" // timestamp
        "000000000000000000000000000000000000000000000000000000000000002A" // era index
        "00000000000000000000000000000000000000000000000000007048860DDF79" // total points
        "000000000000000000000000000000000000000000000000000000E5F4C8F3CA" // total inflated
        "b6e16d27ac5ab427a7f68900ac5559ce272dc6c37c82b3e052246c82244c50e4" // root
        "0101010101010101010101010101010101010101010101010101010101010101" // token_id
    );

    assert_eq!(command.abi_encode(), expected);
}

#[test]
fn test_report_slashes_encoding() {
    pub const ALICE: [u8; 32] = [4u8; 32];
    pub const BOB: [u8; 32] = [5u8; 32];
    pub const CHARLIE: [u8; 32] = [6u8; 32];
    let command = Command::ReportSlashes {
        era_index: 42,
        slashes: vec![
            SlashData {
                encoded_validator_id: sp_runtime::AccountId32::from(ALICE).encode(),
                slash_fraction: 5_000u32,
                external_idx: 500u64,
            },
            SlashData {
                encoded_validator_id: sp_runtime::AccountId32::from(BOB).encode(),
                slash_fraction: 4_000u32,
                external_idx: 400u64,
            },
            SlashData {
                encoded_validator_id: sp_runtime::AccountId32::from(CHARLIE).encode(),
                slash_fraction: 3_000u32,
                external_idx: 300u64,
            },
        ],
    };

    let expected = hex!(
        "0000000000000000000000000000000000000000000000000000000000000020" // offset of era_index
        "000000000000000000000000000000000000000000000000000000000000002A" // era index
        "0000000000000000000000000000000000000000000000000000000000000040" // offset of slashes
        "0000000000000000000000000000000000000000000000000000000000000003" // length of slashes
        "0404040404040404040404040404040404040404040404040404040404040404" // ALICE
        "0000000000000000000000000000000000000000000000000000000000001388" // 5_000u32
        "00000000000000000000000000000000000000000000000000000000000001F4" // 500u64
        "0505050505050505050505050505050505050505050505050505050505050505" // BOB
        "0000000000000000000000000000000000000000000000000000000000000FA0" // 4_000u32
        "0000000000000000000000000000000000000000000000000000000000000190" // 400u64
        "0606060606060606060606060606060606060606060606060606060606060606" // CHARLIE
        "0000000000000000000000000000000000000000000000000000000000000BB8" // 3_000u32
        "000000000000000000000000000000000000000000000000000000000000012C"  // 300u64
    );

    assert_eq!(command.abi_encode(), expected);
}

mod xcm_converter {
    use super::*;
    use crate::snowbridge_outbound_token_transfer::XcmConverter;
    use cumulus_primitives_core::WeightLimit;
    use snowbridge_outbound_queue_primitives::v1::Command;
    use xcm::opaque::latest::{
        AssetFilter, AssetId, Fungibility, Instruction, Junction, NetworkId, WildAsset,
    };

    #[test]
    fn works_with_complete_message() {
        let agent_id = AgentId::repeat_byte(0x77);
        let token_address = hex!("0123456789abcdef0123456789abcdef01234567");
        let beneficiary_address = hex!("0101010101010101010101010101010101010101");
        let amount = 1000;
        let topic = hex!("deadbeafdeadbeafdeadbeafdeadbeafdeadbeafdeadbeafdeadbeafdeadbeaf");

        let ethereum_network = NetworkId::Ethereum { chain_id: 42 };

        let asset_location = Location::new(
            0,
            [Junction::AccountKey20 {
                network: Some(ethereum_network),
                key: token_address,
            }],
        );
        let asset = AssetId(asset_location).into_asset(Fungibility::Fungible(amount));
        let reserve_assets = Assets::from(vec![asset.clone()]);

        let beneficiary = Location::new(
            0,
            [Junction::AccountKey20 {
                network: Some(ethereum_network),
                key: beneficiary_address,
            }],
        );

        let xcm_message = Xcm(vec![
            Instruction::WithdrawAsset(reserve_assets),
            Instruction::ClearOrigin,
            Instruction::BuyExecution {
                fees: asset,
                weight_limit: WeightLimit::Unlimited,
            },
            Instruction::DepositAsset {
                assets: AssetFilter::Wild(WildAsset::All),
                beneficiary,
            },
            Instruction::SetTopic(topic),
        ]);

        let mut xcm_converter =
            XcmConverter::<(), ()>::new(&xcm_message, ethereum_network, agent_id);

        let command = xcm_converter
            .make_unlock_native_token_command()
            .expect("should be valid xcm message");

        assert_eq!(
            command,
            (
                Command::AgentExecute {
                    agent_id,
                    command: AgentExecuteCommand::TransferToken {
                        token: token_address.into(),
                        recipient: beneficiary_address.into(),
                        amount,
                    },
                },
                topic
            )
        )
    }
}

mod custom_message_validator {
    use super::*;
    use crate::{
        mock::{new_test_ext, MaxMessagePayloadSize, OwnLocation, Test},
        SendError,
    };

    #[test]
    fn test_message_validator_v1_successful() {
        // Correct channels and such do work
        new_test_ext().execute_with(|| {
            let tanssi_message = TanssiMessage {
                id: None,
                channel_id: PRIMARY_GOVERNANCE_CHANNEL,
                command: Command::ReportRewards {
                    external_idx: 0u64,
                    era_index: 0u32,
                    total_points: 0u128,
                    tokens_inflated: 0u128,
                    rewards_merkle_root: H256::default(),
                    token_id: H256::default(),
                },
            };

            let result = CustomMessageValidatorV1::<Test>::validate(&tanssi_message);
            assert!(result.is_ok());
        });
    }

    #[test]
    fn test_message_validator_v1_wrong_channel() {
        new_test_ext().execute_with(|| {
            let tanssi_message = TanssiMessage {
                id: None,
                channel_id: H256::default().into(),
                command: Command::ReportRewards {
                    external_idx: 0u64,
                    era_index: 0u32,
                    total_points: 0u128,
                    tokens_inflated: 0u128,
                    rewards_merkle_root: H256::default(),
                    token_id: H256::default(),
                },
            };

            assert_noop!(
                CustomMessageValidatorV1::<Test>::validate(&tanssi_message),
                SendError::InvalidChannel
            );
        });
    }

    #[test]
    fn test_assert_error_max_payload() {
        new_test_ext().execute_with(|| {
            let tanssi_message = TanssiMessage {
                id: None,
                channel_id: H256::default().into(),
                command: Command::Test(vec![0u8; (MaxMessagePayloadSize::get() + 1) as usize]),
            };

            assert_noop!(
                CustomMessageValidatorV1::<Test>::validate(&tanssi_message),
                SendError::MessageTooLarge
            );
        });
    }

    #[test]
    fn test_assert_succesful_v2() {
        new_test_ext().execute_with(|| {
            let tanssi_message = TanssiMessage {
                id: None,
                channel_id: PRIMARY_GOVERNANCE_CHANNEL,
                command: Command::ReportRewards {
                    external_idx: 0u64,
                    era_index: 0u32,
                    total_points: 0u128,
                    tokens_inflated: 0u128,
                    rewards_merkle_root: H256::default(),
                    token_id: H256::default(),
                },
            };

            let result = CustomMessageValidatorV2::<Test, OwnLocation>::validate(&tanssi_message);
            assert!(result.is_ok());
        });
    }

    #[test]
    fn test_assert_error_max_payload_v2() {
        new_test_ext().execute_with(|| {
            let tanssi_message = TanssiMessage {
                id: None,
                channel_id: H256::default().into(),
                command: Command::Test(vec![0u8; (MaxMessagePayloadSize::get() + 1) as usize]),
            };

            assert_noop!(
                CustomMessageValidatorV2::<Test, OwnLocation>::validate(&tanssi_message),
                SendError::MessageTooLarge
            );
        });
    }

    #[test]
    fn test_assert_succesful_v2_in_versioned() {
        new_test_ext().execute_with(|| {
            let tanssi_message = TanssiMessage {
                id: None,
                channel_id: PRIMARY_GOVERNANCE_CHANNEL,
                command: Command::ReportRewards {
                    external_idx: 0u64,
                    era_index: 0u32,
                    total_points: 0u128,
                    tokens_inflated: 0u128,
                    rewards_merkle_root: H256::default(),
                    token_id: H256::default(),
                },
            };

            let (versioned_ticket, fee) = VersionedCustomMessageValidator::<
                Test,
                OwnLocation,
                frame_support::traits::ConstBool<true>,
            >::validate(&tanssi_message)
            .unwrap();
            assert!(matches!(versioned_ticket, VersionedTanssiTicket::V2(_)));
        });
    }

    #[test]
    fn test_assert_succesful_v1_in_versioned() {
        new_test_ext().execute_with(|| {
            let tanssi_message = TanssiMessage {
                id: None,
                channel_id: PRIMARY_GOVERNANCE_CHANNEL,
                command: Command::ReportRewards {
                    external_idx: 0u64,
                    era_index: 0u32,
                    total_points: 0u128,
                    tokens_inflated: 0u128,
                    rewards_merkle_root: H256::default(),
                    token_id: H256::default(),
                },
            };

            let (versioned_ticket, fee) = VersionedCustomMessageValidator::<
                Test,
                OwnLocation,
                frame_support::traits::ConstBool<false>,
            >::validate(&tanssi_message)
            .unwrap();
            assert!(matches!(versioned_ticket, VersionedTanssiTicket::V1(_)));
        });
    }
}

mod custom_message_delivers {
    use super::*;
    use crate::{
        mock::{
            last_delivered_message_queue, new_test_ext, run_to_block, MaxMessagePayloadSize,
            MockGetAggregateMessageOrigin, OwnLocation, RuntimeEvent, System, Test,
        },
        SendError,
    };
    use assert_matches::assert_matches;
    use snowbridge_outbound_queue_primitives::v2::Message as MessageV2;

    #[test]
    fn test_message_deliver_v1_successful() {
        // Correct channels and such do work
        new_test_ext().execute_with(|| {
            // otherwise no events
            run_to_block(1);
            let tanssi_message = TanssiMessage {
                id: None,
                channel_id: PRIMARY_GOVERNANCE_CHANNEL,
                command: Command::ReportRewards {
                    external_idx: 0u64,
                    era_index: 0u32,
                    total_points: 0u128,
                    tokens_inflated: 0u128,
                    rewards_merkle_root: H256::default(),
                    token_id: H256::default(),
                },
            };

            let (ticket, fee) =
                CustomMessageValidatorV1::<Test>::validate(&tanssi_message).unwrap();

            let result =
                CustomSendMessageV1::<Test, MockGetAggregateMessageOrigin>::deliver(ticket.clone());

            assert!(result.is_ok());
            assert_eq!(
                last_delivered_message_queue(),
                ticket.message.as_bounded_slice().to_vec()
            );
            // assert event has been emited
            System::assert_last_event(RuntimeEvent::EthereumOutboundQueue(
                snowbridge_pallet_outbound_queue::Event::MessageQueued {
                    id: ticket.message_id,
                },
            ));
        });
    }

    #[test]
    fn test_message_deliver_v2_successful() {
        new_test_ext().execute_with(|| {
            // otherwise no events
            run_to_block(1);

            let tanssi_message = TanssiMessage {
                id: None,
                channel_id: PRIMARY_GOVERNANCE_CHANNEL,
                command: Command::ReportRewards {
                    external_idx: 0u64,
                    era_index: 0u32,
                    total_points: 0u128,
                    tokens_inflated: 0u128,
                    rewards_merkle_root: H256::default(),
                    token_id: H256::default(),
                },
            };

            let (ticket, fee) =
                CustomMessageValidatorV2::<Test, OwnLocation>::validate(&tanssi_message).unwrap();
            let result =
                CustomSendMessageV2::<Test, MockGetAggregateMessageOrigin>::deliver(ticket.clone());

            assert!(result.is_ok());

            assert_eq!(
                last_delivered_message_queue(),
                ticket.message.as_bounded_slice().to_vec()
            );

            // assert event has been emited
            System::assert_last_event(RuntimeEvent::EthereumOutboundQueueV2(
                snowbridge_pallet_outbound_queue_v2::Event::MessageQueued {
                    message: MessageV2 {
                        origin: ticket.origin,
                        fee: ticket.fee,
                        id: ticket.id,
                        commands: vec![].try_into().unwrap(),
                    },
                },
            ));
        });
    }

    #[test]
    fn test_message_versioned_deliver_is_succesful_v1() {
        // Correct channels and such do work
        new_test_ext().execute_with(|| {
            // otherwise no events
            run_to_block(1);
            let tanssi_message = TanssiMessage {
                id: None,
                channel_id: PRIMARY_GOVERNANCE_CHANNEL,
                command: Command::ReportRewards {
                    external_idx: 0u64,
                    era_index: 0u32,
                    total_points: 0u128,
                    tokens_inflated: 0u128,
                    rewards_merkle_root: H256::default(),
                    token_id: H256::default(),
                },
            };

            let (versioned_ticket, fee) = VersionedCustomMessageValidator::<
                Test,
                OwnLocation,
                frame_support::traits::ConstBool<false>,
            >::validate(&tanssi_message)
            .unwrap();

            let result =
                VersionedCustomMessageSender::<Test, MockGetAggregateMessageOrigin>::deliver(
                    versioned_ticket.clone(),
                );

            assert!(result.is_ok());
            assert_matches!(versioned_ticket, VersionedTanssiTicket::V1(_));

            // assert event has been emited
            System::assert_last_event(RuntimeEvent::EthereumOutboundQueue(
                snowbridge_pallet_outbound_queue::Event::MessageQueued {
                    id: versioned_ticket.message_id(),
                },
            ));
        });
    }

    #[test]
    fn test_message_versioned_deliver_is_succesful_v2() {
        // Correct channels and such do work
        new_test_ext().execute_with(|| {
            // otherwise no events
            run_to_block(1);
            let tanssi_message = TanssiMessage {
                id: None,
                channel_id: PRIMARY_GOVERNANCE_CHANNEL,
                command: Command::ReportRewards {
                    external_idx: 0u64,
                    era_index: 0u32,
                    total_points: 0u128,
                    tokens_inflated: 0u128,
                    rewards_merkle_root: H256::default(),
                    token_id: H256::default(),
                },
            };

            let (versioned_ticket, fee) = VersionedCustomMessageValidator::<
                Test,
                OwnLocation,
                frame_support::traits::ConstBool<true>,
            >::validate(&tanssi_message)
            .unwrap();
            assert_matches!(versioned_ticket, VersionedTanssiTicket::V2(_));

            let result =
                VersionedCustomMessageSender::<Test, MockGetAggregateMessageOrigin>::deliver(
                    versioned_ticket.clone(),
                );

            assert!(result.is_ok());
            let event = System::events().pop().expect("Expected event").event;

            assert_matches!(
                event,
                RuntimeEvent::EthereumOutboundQueueV2(
                    snowbridge_pallet_outbound_queue_v2::Event::MessageQueued { ref message }
                ) if message.id == versioned_ticket.message_id()
            );
        });
    }
}
