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
        tests::common::*, EthereumInboundQueue, EthereumTokenTransfers, Paras, RuntimeEvent,
        XcmPallet,
    },
    alloc::vec,
    alloy_sol_types::SolEvent,
    frame_support::assert_ok,
    parity_scale_codec::Encode,
    polkadot_parachain_primitives::primitives::HeadData,
    snowbridge_core::{AgentId, ChannelId, ParaId},
    snowbridge_inbound_queue_primitives::v1::{
        Command, Destination, MessageV1, OutboundMessageAccepted, VersionedXcmMessage,
    },
    snowbridge_inbound_queue_primitives::{EventProof, Log},
    sp_core::{H160, H256},
    starlight_runtime_constants::snowbridge::EthereumNetwork,
    xcm::latest::{prelude::*, Junctions::*, Location},
};

#[test]
fn receive_container_foreign_tokens_from_eth_is_disabled_so_no_error_and_no_xcm_is_sent() {
    ExtBuilder::default()
        .with_balances(vec![(AccountId::from(ALICE), 100_000 * UNIT)])
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

            let sent = System::events().iter().any(|r| {
                matches!(
                    r.event,
                    RuntimeEvent::XcmPallet(pallet_xcm::Event::Sent { .. })
                )
            });
            assert!(!sent, "XCM Sent event should be emitted!");
        });
}
