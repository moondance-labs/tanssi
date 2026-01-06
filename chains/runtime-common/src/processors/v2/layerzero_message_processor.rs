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

extern crate alloc;

use crate::processors::v2::fallback_message_processor::{
    AssetTrapFallbackProcessor, PrivilegedFallbackProcessor,
};
use crate::processors::v2::{
    CodecError, FallbackMessageProcessor, MessageExtractionError, MessageProcessorWithFallback,
};
use alloc::boxed::Box;
use alloc::format;
use alloc::string::ToString;
use alloy_core::sol_types::SolType;
use core::marker::PhantomData;
use parity_scale_codec::Decode;
use snowbridge_inbound_queue_primitives::v2::{Message, MessageProcessorError, Payload};
use sp_core::{Get, H160};
use tp_bridge::layerzero_message::{
    InboundMessage as LayerZeroInboundMessage, InboundSolPayload as LayerZeroInboundSolPayload,
    MAGIC_BYTES as LZ_MAGIC_BYTES,
};
use v2_processor_proc_macro::MessageProcessor;
use xcm::latest::{ExecuteXcm, InteriorLocation, NetworkId};
use xcm_executor::traits::WeightBounds;

pub fn try_extract_message<T: pallet_lz_router::Config>(
    message: &Message,
    gateway_proxy_address: H160,
) -> Result<LayerZeroInboundMessage, MessageExtractionError> {
    match message.payload {
        Payload::Raw(ref payload) => {
            let raw_payload = crate::processors::v2::RawPayload::decode(&mut payload.as_slice())
                .map_err(|error| MessageExtractionError::InvalidMessage {
                    context: "Unable to decode RawMessage".to_string(),
                    source: Some(Box::new(CodecError(error))),
                })?;
            match raw_payload {
                crate::processors::v2::RawPayload::LayerZero(payload) => {
                    if message.origin != gateway_proxy_address {
                        return Err(MessageExtractionError::InvalidMessage {
                            context: format!(
                                "LayerZero message origin is {:?} expected {:?}",
                                message.origin, gateway_proxy_address
                            ),
                            source: None,
                        });
                    }

                    if message.value > 0 || !message.assets.is_empty() {
                        return Err(MessageExtractionError::InvalidMessage {
                            context: "LayerZero message cannot have assets".to_string(),
                            source: None,
                        });
                    }

                    let sol_payload =
                        LayerZeroInboundSolPayload::abi_decode_validate(&mut payload.as_slice())
                            .map_err(|error| MessageExtractionError::InvalidMessage {
                                context: "Unable to decode LayerZero Payload".to_string(),
                                source: Some(Box::new(error)),
                            })?;
                    if &sol_payload.magicBytes.0 != LZ_MAGIC_BYTES {
                        return Err(MessageExtractionError::InvalidMessage {
                            context: format!(
                                "LayerZero magic bytes expected: {:?} got: {:?}",
                                LZ_MAGIC_BYTES, sol_payload.magicBytes.0,
                            ),
                            source: None,
                        });
                    }

                    sol_payload.message.try_into().map_err(|e| {
                        MessageExtractionError::InvalidMessage {
                            context: format!("LayerZero message conversion failed: {}", e),
                            source: None,
                        }
                    })
                }
                _ => Err(MessageExtractionError::UnsupportedMessage {
                    context: "Unsupported Message".to_string(),
                    source: None,
                }),
            }
        }
        _ => Err(MessageExtractionError::UnsupportedMessage {
            context: "Unsupported Message".to_string(),
            source: None,
        }),
    }
}

pub fn process_message<T: pallet_lz_router::Config>(
    message: LayerZeroInboundMessage,
) -> Result<(), MessageProcessorError> {
    pallet_lz_router::Pallet::<T>::handle_inbound_message(message)
}

#[derive(MessageProcessor)]
pub struct LayerZeroMessageProcessor<
    T,
    GatewayAddress,
    DefaultClaimer,
    EthereumNetwork,
    EthereumUniversalLocation,
    TanssiUniversalLocation,
    XcmProcessor,
    XcmWeigher,
>(
    PhantomData<(
        T,
        GatewayAddress,
        DefaultClaimer,
        EthereumNetwork,
        EthereumUniversalLocation,
        TanssiUniversalLocation,
        XcmProcessor,
        XcmWeigher,
    )>,
);

impl<
        T,
        AccountId,
        GatewayAddress,
        DefaultClaimer,
        EthereumNetwork,
        EthereumUniversalLocation,
        TanssiUniversalLocation,
        XcmProcessor,
        XcmWeigher,
    > MessageProcessorWithFallback<AccountId>
    for LayerZeroMessageProcessor<
        T,
        GatewayAddress,
        DefaultClaimer,
        EthereumNetwork,
        EthereumUniversalLocation,
        TanssiUniversalLocation,
        XcmProcessor,
        XcmWeigher,
    >
where
    T: snowbridge_pallet_inbound_queue::Config
        + pallet_xcm::Config
        + snowbridge_pallet_system::Config
        + pallet_lz_router::Config,
    [u8; 32]: From<<T as frame_system::Config>::AccountId>,
    GatewayAddress: Get<H160>,
    DefaultClaimer: Get<<T as frame_system::Config>::AccountId>,
    EthereumNetwork: Get<NetworkId>,
    EthereumUniversalLocation: Get<InteriorLocation>,
    TanssiUniversalLocation: Get<InteriorLocation>,
    XcmProcessor: ExecuteXcm<<T as pallet_xcm::Config>::RuntimeCall>,
    XcmWeigher: WeightBounds<<T as pallet_xcm::Config>::RuntimeCall>,
{
    type Fallback = PrivilegedFallbackProcessor<
        T,
        AssetTrapFallbackProcessor<
            T,
            GatewayAddress,
            DefaultClaimer,
            EthereumNetwork,
            EthereumUniversalLocation,
            TanssiUniversalLocation,
            XcmProcessor,
            XcmWeigher,
        >,
        GatewayAddress,
        DefaultClaimer,
        EthereumNetwork,
        EthereumUniversalLocation,
        TanssiUniversalLocation,
        XcmProcessor,
        XcmWeigher,
    >;
    type ExtractedMessage = LayerZeroInboundMessage;

    fn try_extract_message(
        _sender: &AccountId,
        message: &Message,
    ) -> Result<Self::ExtractedMessage, MessageExtractionError> {
        let gateway_proxy_address = T::GatewayAddress::get();
        try_extract_message::<T>(message, gateway_proxy_address)
    }

    fn process_extracted_message(
        _sender: AccountId,
        extracted_message: Self::ExtractedMessage,
    ) -> Result<(), MessageProcessorError> {
        process_message::<T>(extracted_message)
    }
}
