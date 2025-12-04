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

use crate::processors::v2::{
    execute_xcm, reanchor_location_to_tanssi, ExtractedXcmConstructionInfo,
    FallbackMessageProcessor,
};
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

        let eth_location_reanchored_to_tanssi = reanchor_location_to_tanssi(
            &EthereumUniversalLocation::get(),
            &TanssiUniversalLocation::get(),
            ().into(),
        )
        .map_err(|asset_derivation_error| {
            MessageProcessorError::ProcessMessage(DispatchError::Other(
                format!(
                    "Unable to reanchor eth location to tanssi: {:?}",
                    asset_derivation_error
                )
                .leak(),
            ))
        })?;

        // Depending upon the content of raw xcm, it might be the case that it is not fully revertible
        // (i.e xcm that sends a message in another container chain and then return an error).
        // Another reason we are not returning error here as otherwise the tx will be reverted and assets will be in limbo in ethereum.
        // By returning success here, the assets will be trapped here and claimable by the claimer.
        if let Err(instruction_error) = execute_xcm::<T, XcmProcessor, XcmWeigher>(
            eth_location_reanchored_to_tanssi,
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

/// This fallback message processor will only attempt to trap assets if there are any, otherwise it will return
/// an error.
/// In simple term this is a wrapper around `AssetTrapMessageProcessor`
pub struct SymbioticFallbackProcessor<
    T,
    AssetTrapFallbackProcessor,
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
        AssetTrapFallbackProcessor,
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
        AssetTrapFallbackProcessor,
        AccountId,
        GatewayAddress,
        DefaultClaimer,
        EthereumNetwork,
        EthereumUniversalLocation,
        TanssiUniversalLocation,
        XcmProcessor,
        XcmWeigher,
    > FallbackMessageProcessor<AccountId>
    for SymbioticFallbackProcessor<
        T,
        AssetTrapFallbackProcessor,
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
    AssetTrapFallbackProcessor: FallbackMessageProcessor<AccountId>,
    [u8; 32]: From<<T as frame_system::Config>::AccountId>,
    GatewayAddress: Get<H160>,
    DefaultClaimer: Get<<T as frame_system::Config>::AccountId>,
    EthereumNetwork: Get<NetworkId>,
    EthereumUniversalLocation: Get<InteriorLocation>,
    TanssiUniversalLocation: Get<InteriorLocation>,
    XcmProcessor: ExecuteXcm<<T as pallet_xcm::Config>::RuntimeCall>,
    XcmWeigher: WeightBounds<<T as pallet_xcm::Config>::RuntimeCall>,
{
    fn handle_message(who: AccountId, message: Message) -> Result<[u8; 32], MessageProcessorError> {
        // It is highly likey that:
        // If any assets are associated with the message, a user mistakenly or maliciously sent Symbiotic message
        // If no assets are associated with the message, the symbiotic middleware sent the message with wrong semantics
        // Based on above assumption we do conditional fallback
        if !message.assets.is_empty() || message.value > 0 || message.execution_fee > 0 {
            AssetTrapFallbackProcessor::handle_message(who, message)
        } else {
            Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "Invalid symbiotic message payload",
            )))
        }
    }
}
