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
    dancelight_runtime::{xcm_config, AccountId, Runtime},
    frame_support::parameter_types,
    parity_scale_codec::Encode,
    snowbridge_core::{AgentId, ChannelId},
    snowbridge_inbound_queue_primitives::v2::{message::Message, Payload},
    sp_core::{H160, H256},
    tanssi_runtime_common::processors::v2::{
        MessageExtractionError, MessageProcessorWithFallback, RawPayload, SymbioticMessageProcessor,
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

#[test]
fn symbiotic_try_extract_message_fails_with_invalid_symbiotic_payload() {
    ExtBuilder::default().build().execute_with(|| {
        let origin = GatewayAddress::get();
        let sender: AccountId = AccountId::from(ALICE);

        let raw_payload = RawPayload::Symbiotic(vec![0xAA, 0xBB, 0xCC].encode());

        let message = Message {
            gateway: origin,
            nonce: 1,
            origin,
            assets: vec![],
            payload: Payload::Raw(raw_payload.encode()),
            claimer: None,
            value: 0,
            execution_fee: 0,
            relayer_fee: 0,
        };

        type Processor = SymbioticMessageProcessor<
            Runtime,
            GatewayAddress,
            DefaultClaimer,
            EthereumNetwork,
            EthereumUniversalLocation,
            TanssiUniversalLocation,
            xcm_executor::XcmExecutor<xcm_config::XcmConfig>,
            <xcm_config::XcmConfig as xcm_executor::Config>::Weigher,
        >;

        let result = <Processor as MessageProcessorWithFallback<AccountId>>::try_extract_message(
            &sender, &message,
        );

        assert!(
            matches!(
                result,
                Err(MessageExtractionError::InvalidMessage { .. })
            ),
            "Invalid Symbiotic payload should result in InvalidMessage error, got: {:?}",
            result
        );
    });
}
