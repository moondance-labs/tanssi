extern crate alloc;

use crate::processors::v2::RAW_MESSAGE_PROCESSOR_TOPIC_PREFIX;
use crate::processors::v2::{
    execute_xcm, fallback_message_processor::AssetTrapFallbackProcessor,
    prepare_raw_message_xcm_instructions, CodecError, ExtractedXcmConstructionInfo,
    FallbackMessageProcessor, MessageExtractionError, MessageProcessorWithFallback, RawPayload,
};
use alloc::boxed::Box;
use alloc::format;
use alloc::string::ToString;
use core::marker::PhantomData;
use parity_scale_codec::{Decode, DecodeLimit};
use snowbridge_inbound_queue_primitives::v2::{message::Message, MessageProcessorError, Payload};
use sp_core::{Get, H160};
use sp_runtime::DispatchError;
use thiserror::Error;
use v2_processor_proc_macro::MessageProcessor;
use xcm::latest::ExecuteXcm;
use xcm::prelude::{InteriorLocation, NetworkId, VersionedXcm, Xcm};
use xcm::{IdentifyVersion, Version, MAX_XCM_DECODE_DEPTH};
use xcm_executor::traits::WeightBounds;

#[derive(Error, Debug)]
pub enum XcmDecodeError {
    #[error("Failed to decode versioned xcm message: {0}")]
    VersionedXcmDecodeError(#[from] CodecError),
    #[error("Xcm version {version} is not supported")]
    UnsupportedXcmVersion { version: Version },
}

/// Parse and strictly decode `raw` XCM bytes into a `Xcm<()>`.
fn decode_raw_xcm<T>(
    mut data: &[u8],
) -> Result<Xcm<<T as pallet_xcm::Config>::RuntimeCall>, XcmDecodeError>
where
    T: pallet_xcm::Config,
{
    VersionedXcm::<<T as pallet_xcm::Config>::RuntimeCall>::decode_with_depth_limit(
        MAX_XCM_DECODE_DEPTH,
        &mut data,
    )
    .map_err(|e| XcmDecodeError::VersionedXcmDecodeError(e.into()))
    .and_then(|xcm| {
        let version = xcm.identify_version();
        xcm.try_into()
            .map_err(|_| XcmDecodeError::UnsupportedXcmVersion { version })
    })
}

pub fn try_extract_message<T>(
    message: &Message,
) -> Result<
    ExtractedXcmConstructionInfo<<T as pallet_xcm::Config>::RuntimeCall>,
    MessageExtractionError,
>
where
    T: pallet_xcm::Config,
{
    match message.payload {
        Payload::Raw(ref payload) => {
            let raw_payload =
                RawPayload::decode(&mut payload.as_slice()).map_err(|decode_error| {
                    MessageExtractionError::InvalidMessage {
                        context: "Unable to decode RawPayload".to_string(),
                        source: Some(Box::new(CodecError(decode_error))),
                    }
                })?;
            match raw_payload {
                RawPayload::Xcm(payload) => Ok(decode_raw_xcm::<T>(&payload)
                    .map(|xcm| ExtractedXcmConstructionInfo {
                        origin: message.origin.clone(),
                        maybe_claimer: message.claimer.clone(),
                        assets: message.assets.clone(),
                        eth_value: message.value,
                        execution_fee_in_eth: message.execution_fee,
                        nonce: message.nonce,
                        user_xcm: xcm,
                    })
                    .map_err(|error| MessageExtractionError::InvalidMessage {
                        context: "Unable to decode Xcm".to_string(),
                        source: Some(Box::new(error)),
                    })?),
                RawPayload::Symbiotic(_) => Err(MessageExtractionError::UnsupportedMessage {
                    context: "Message is unsupported".to_string(),
                    source: None,
                }),
            }
        }
        _ => Err(MessageExtractionError::UnsupportedMessage {
            context: "Message is unsupported".to_string(),
            source: None,
        }),
    }
}

#[derive(MessageProcessor)]
pub struct RawMessageProcessor<
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
        GatewayAddress,
        DefaultClaimer,
        EthereumNetwork,
        EthereumUniversalLocation,
        TanssiUniversalLocation,
        XcmProcessor,
        XcmWeigher,
    > MessageProcessorWithFallback<<T as frame_system::Config>::AccountId>
    for RawMessageProcessor<
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
        + snowbridge_pallet_system::Config,
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
    type ExtractedMessage = ExtractedXcmConstructionInfo<<T as pallet_xcm::Config>::RuntimeCall>;

    fn try_extract_message(
        _sender: &<T as frame_system::Config>::AccountId,
        message: &Message,
    ) -> Result<Self::ExtractedMessage, MessageExtractionError> {
        try_extract_message::<T>(message)
    }

    fn process_extracted_message(
        _sender: <T as frame_system::Config>::AccountId,
        extracted_message: Self::ExtractedMessage,
    ) -> Result<[u8; 32], MessageProcessorError> {
        let prepared_xcm = prepare_raw_message_xcm_instructions::<T>(
            EthereumNetwork::get(),
            &EthereumUniversalLocation::get(),
            &TanssiUniversalLocation::get(),
            GatewayAddress::get(),
            DefaultClaimer::get(),
            RAW_MESSAGE_PROCESSOR_TOPIC_PREFIX,
            extracted_message,
        )
        .map_err(|asset_derivation_error| {
            MessageProcessorError::ProcessMessage(DispatchError::Other(
                format!(
                    "Error while preparing xcm instructions: {:?}",
                    asset_derivation_error
                )
                .leak(),
            ))
        })?
        .into();

        if let Err(instruction_error) = execute_xcm::<T, XcmProcessor, XcmWeigher>(
            EthereumUniversalLocation::get(),
            prepared_xcm,
        ) {
            // TODO: Print an error
        }

        Ok([0; 32])
    }
}
