extern crate alloc;

use crate::processors::v2::{
    fallback_message_processor::AssetTrapFallbackProcessor, CodecError, FallbackMessageProcessor,
    MessageExtractionError, MessageProcessorWithFallback,
};
use alloc::boxed::Box;
use alloc::format;
use alloc::string::ToString;
use core::marker::PhantomData;
use parity_scale_codec::Decode;
use snowbridge_inbound_queue_primitives::v2::{Message, MessageProcessorError, Payload};
use sp_core::{Get, H160};
use tp_bridge::symbiotic_message_processor::{
    InboundCommand, Message as SymbioticMessage, Payload as SymbioticPayload, MAGIC_BYTES,
};
use v2_processor_proc_macro::MessageProcessor;
use xcm::latest::{ExecuteXcm, InteriorLocation, NetworkId};
use xcm_executor::traits::WeightBounds;

pub fn try_extract_message<T: pallet_external_validators::Config>(
    message: &Message,
    gateway_proxy_address: H160,
) -> Result<SymbioticMessage<T>, MessageExtractionError> {
    match message.payload {
        Payload::Raw(ref payload) => {
            let raw_payload = crate::processors::v2::RawPayload::decode(&mut payload.as_slice())
                .map_err(|error| MessageExtractionError::InvalidMessage {
                    context: "Unable to decode RawMessage".to_string(),
                    source: Some(Box::new(CodecError(error))),
                })?;
            match raw_payload {
                crate::processors::v2::RawPayload::Xcm(_payload) => {
                    Err(MessageExtractionError::UnsupportedMessage {
                        context: "Unsupported Message".to_string(),
                        source: None,
                    })
                }
                crate::processors::v2::RawPayload::Symbiotic(payload) => {
                    if message.origin != gateway_proxy_address {
                        return Err(MessageExtractionError::InvalidMessage {
                            context: format!(
                                "Symbiotic  message origin is {:?} expected {:?}",
                                message.origin, gateway_proxy_address
                            ),
                            source: None,
                        });
                    }

                    if message.value > 0 || !message.assets.is_empty() {
                        return Err(MessageExtractionError::InvalidMessage {
                            context: "Symbiotic message cannot have assets".to_string(),
                            source: None,
                        });
                    }

                    let symbiotic_payload = SymbioticPayload::decode(&mut payload.as_slice())
                        .map_err(|error| MessageExtractionError::InvalidMessage {
                            context: "Unable to decode Symbiotic Payload".to_string(),
                            source: Some(Box::new(CodecError(error))),
                        })?;
                    if symbiotic_payload.magic_bytes != MAGIC_BYTES {
                        return Err(MessageExtractionError::InvalidMessage {
                            context: format!(
                                "Symbiotic magic bytes expected: {:?} got: {:?}",
                                MAGIC_BYTES, symbiotic_payload.magic_bytes
                            ),
                            source: None,
                        });
                    }

                    return Ok(symbiotic_payload.message);
                }
            }
        }
        _ => Err(MessageExtractionError::UnsupportedMessage {
            context: "Unsupported Message".to_string(),
            source: None,
        }),
    }
}

pub fn process_message<T: pallet_external_validators::Config>(
    symbiotic_message: SymbioticMessage<T>,
) -> Result<(), MessageProcessorError> {
    match symbiotic_message {
        tp_bridge::symbiotic_message_processor::Message::V1(
            InboundCommand::ReceiveValidators {
                validators,
                external_index,
            },
        ) => {
            pallet_external_validators::Pallet::<T>::set_external_validators_inner(
                validators,
                external_index,
            )
            .map_err(|error| MessageProcessorError::ProcessMessage(error))?;
            Ok(())
        }
    }
}

#[derive(MessageProcessor)]
pub struct SymbioticMessageProcessor<
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
    for SymbioticMessageProcessor<
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
    type Fallback = AssetTrapFallbackProcessor<
        T,
        GatewayAddress,
        DefaultClaimer,
        EthereumNetwork,
        EthereumUniversalLocation,
        TanssiUniversalLocation,
        XcmProcessor,
        XcmWeigher,
    >;
    type ExtractedMessage = SymbioticMessage<T>;

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
    ) -> Result<[u8; 32], MessageProcessorError> {
        process_message(extracted_message).map(|_| [0; 32])
    }
}
