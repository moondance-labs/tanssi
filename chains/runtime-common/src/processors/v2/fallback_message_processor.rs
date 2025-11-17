extern crate alloc;

use crate::processors::v2::{execute_xcm, ExtractedXcmConstructionInfo, FallbackMessageProcessor};
use alloc::format;
use core::marker::PhantomData;
use frame_support::__private::Get;
use snowbridge_inbound_queue_primitives::v2::{Message, MessageProcessorError};
use sp_core::H160;
use sp_runtime::DispatchError;
use xcm::latest::{ExecuteXcm, InteriorLocation, NetworkId, Xcm};
use xcm_executor::traits::WeightBounds;

pub struct AssetTrapFallbackProcessor<
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
    > FallbackMessageProcessor<AccountId>
    for AssetTrapFallbackProcessor<
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
    fn handle_message(
        _who: AccountId,
        message: Message,
    ) -> Result<[u8; 32], MessageProcessorError> {
        let extracted_message: ExtractedXcmConstructionInfo<
            <T as pallet_xcm::Config>::RuntimeCall,
        > = ExtractedXcmConstructionInfo {
            origin: message.origin.clone(),
            maybe_claimer: message.claimer.clone(),
            assets: message.assets.clone(),
            eth_value: message.value,
            execution_fee_in_eth: message.execution_fee,
            nonce: message.nonce,
            user_xcm: Xcm::new(),
        };

        let prepared_xcm = crate::processors::v2::prepare_raw_message_xcm_instructions::<T>(
            EthereumNetwork::get(),
            &EthereumUniversalLocation::get(),
            &TanssiUniversalLocation::get(),
            GatewayAddress::get(),
            DefaultClaimer::get(),
            crate::processors::v2::RAW_MESSAGE_PROCESSOR_TOPIC_PREFIX,
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

        // Depending upon the content of raw xcm, it might be the case that it is not fully revertible
        // (i.e xcm that sends a message in another container chain and then return an error).
        // Another reason we are not returning error here as otherwise the tx will be reverted and assets will be in limbo in ethereum.
        // By returning success here, the assets will be trapped here and claimable by the claimer.
        if let Err(instruction_error) = execute_xcm::<T, XcmProcessor, XcmWeigher>(
            EthereumUniversalLocation::get(),
            prepared_xcm,
        ) {
            log::error!(
                "Error while executing xcm in fallback message processor: {:?}",
                instruction_error
            );
        }

        Ok([0; 32])
    }
}
