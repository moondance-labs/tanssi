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
    Message as LayerZeroMessage, Payload as LayerZeroPayload, MAGIC_BYTES as LZ_MAGIC_BYTES,
};
use v2_processor_proc_macro::MessageProcessor;
use xcm::latest::{ExecuteXcm, InteriorLocation, NetworkId};
use xcm_executor::traits::WeightBounds;

pub fn try_extract_message(
    message: &Message,
    gateway_proxy_address: H160,
) -> Result<LayerZeroMessage, MessageExtractionError> {
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

                    let layer_zero_payload =
                        LayerZeroPayload::abi_decode_validate(&mut payload.as_slice()).map_err(
                            |error| MessageExtractionError::InvalidMessage {
                                context: "Unable to decode LayerZero Payload".to_string(),
                                source: Some(Box::new(error)),
                            },
                        )?;
                    if &layer_zero_payload.magicBytes.0 != LZ_MAGIC_BYTES {
                        return Err(MessageExtractionError::InvalidMessage {
                            context: format!(
                                "LayerZero magic bytes expected: {:?} got: {:?}",
                                LZ_MAGIC_BYTES, layer_zero_payload.magicBytes.0,
                            ),
                            source: None,
                        });
                    }

                    Ok(layer_zero_payload.message)
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

pub fn process_message(layer_zero_message: LayerZeroMessage) -> Result<(), MessageProcessorError> {
    /* TODO: Implement the actual message processing logic here
     *
     * In a `LayerZeroMessageForwarder` pallet,
     * We first check that the source eid/address is whitelisted by the container chain,
     * then we send a XCM message to query `LayerZeroHandler` pallet on the container chain,
     * when we get notified in `LayerZeroMessageForwarder` about the pallet index of `LayerZeroHandler`,
     * we can then send the message to `LayerZeroHandler` using XCM `Transact` instruction sending the bytes received.
     */
    unimplemented!()
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
        + pallet_external_validators::Config,
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
    type ExtractedMessage = LayerZeroMessage;

    fn try_extract_message(
        _sender: &AccountId,
        message: &Message,
    ) -> Result<Self::ExtractedMessage, MessageExtractionError> {
        let gateway_proxy_address = T::GatewayAddress::get();
        try_extract_message(message, gateway_proxy_address)
    }

    fn process_extracted_message(
        _sender: AccountId,
        extracted_message: Self::ExtractedMessage,
    ) -> Result<(), MessageProcessorError> {
        process_message(extracted_message)
    }
}
