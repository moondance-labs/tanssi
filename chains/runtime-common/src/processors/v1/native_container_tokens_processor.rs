extern crate alloc;

use crate::processors::v1::{GatewayAndChannelValidator, NativeTokenTransferData};
use alloc::vec;
use core::marker::PhantomData;
use snowbridge_core::Channel;
use snowbridge_inbound_queue_primitives::v1::{Destination, Envelope, MessageProcessor};
use sp_core::Get;
use sp_runtime::{traits::MaybeEquivalence, DispatchError, DispatchResult};
use xcm::prelude::*;
use xcm_executor::traits::TransactAsset;

/// `NativeContainerTokensProcessor` is responsible for receiving and processing native container
/// chain tokens coming from Ethereum and forwarding them to the container chain via Tanssi through XCM.
pub struct NativeContainerTokensProcessor<
    T,
    AssetTransactor,
    EthereumLocation,
    EthereumNetwork,
    InboundQueuePalletInstance,
    TanssiLocationReanchored,
>(
    PhantomData<(
        T,
        AssetTransactor,
        EthereumLocation,
        EthereumNetwork,
        InboundQueuePalletInstance,
        TanssiLocationReanchored,
    )>,
);

impl<
        T,
        AssetTransactor,
        EthereumLocation,
        EthereumNetwork,
        InboundQueuePalletInstance,
        TanssiLocationReanchored,
    > MessageProcessor
    for NativeContainerTokensProcessor<
        T,
        AssetTransactor,
        EthereumLocation,
        EthereumNetwork,
        InboundQueuePalletInstance,
        TanssiLocationReanchored,
    >
where
    T: snowbridge_pallet_inbound_queue::Config
        + pallet_ethereum_token_transfers::Config
        + snowbridge_pallet_system::Config
        + pallet_xcm::Config,
    <T as frame_system::Config>::RuntimeEvent: From<pallet_xcm::Event<T>>,
    <T as frame_system::Config>::AccountId: Into<Location>,
    AssetTransactor: TransactAsset,
    EthereumLocation: Get<Location>,
    EthereumNetwork: Get<NetworkId>,
    InboundQueuePalletInstance: Get<u8>,
    TanssiLocationReanchored: Get<Location>,
{
    fn can_process_message(channel: &Channel, envelope: &Envelope) -> bool {
        // Validate channel and gateway
        if !GatewayAndChannelValidator::<T>::validate_gateway_and_channel(channel, envelope) {
            log::warn!(
                "NativeContainerTokensProcessor::can_process_message: invalid gateway or channel"
            );
            return false;
        }

        // Try decoding the message and check if the destination owns the token being transferred
        match Self::get_token_data_and_location(&envelope.payload) {
            TokenDataResult::Success(token_data, token_location) => {
                Self::validate_destination_owns_token(&token_location, &token_data.destination)
            }
            TokenDataResult::DecodeFailure => {
                log::error!(
                    "NativeContainerTokensProcessor::can_process_message: failed to decode token data"
                );
                false
            }
            TokenDataResult::LocationNotFound(token_data) => {
                log::error!(
                    "NativeContainerTokensProcessor::can_process_message: token location not found for token_id: {:?}",
                    token_data.token_id
                );
                false
            }
            TokenDataResult::UnsupportedToken => {
                log::error!(
                    "NativeContainerTokensProcessor::can_process_message: unsupported token"
                );
                false
            }
        }
    }

    fn process_message(_channel: Channel, envelope: Envelope) -> DispatchResult {
        match Self::get_token_data_and_location(&envelope.payload) {
            TokenDataResult::Success(token_data, token_location) => {
                Self::process_native_container_token_transfer(token_data, token_location);
                Ok(())
            }
            TokenDataResult::DecodeFailure => Err(DispatchError::Other(
                "NativeContainerTokensProcessor: unexpected message",
            )),
            TokenDataResult::LocationNotFound(token_data) => {
                log::warn!(
                    "NativeContainerTokensProcessor::process_message: token location not found for token_id: {:?}",
                    token_data.token_id
                );
                Ok(())
            }
            TokenDataResult::UnsupportedToken => {
                log::error!("NativeContainerTokensProcessor::process_message: unsupported token");
                Ok(())
            }
        }
    }
}

/// Result of token data and location extraction
enum TokenDataResult {
    /// Successfully extracted both token data and location
    Success(NativeTokenTransferData, Location),
    /// Failed to decode token data from payload
    DecodeFailure,
    /// Token data decoded but location not found
    LocationNotFound(NativeTokenTransferData),
    /// Token data decoded but the token is not supported
    UnsupportedToken,
}

impl<
        T,
        AssetTransactor,
        EthereumLocation,
        EthereumNetwork,
        InboundQueuePalletInstance,
        TanssiLocationReanchored,
    >
    NativeContainerTokensProcessor<
        T,
        AssetTransactor,
        EthereumLocation,
        EthereumNetwork,
        InboundQueuePalletInstance,
        TanssiLocationReanchored,
    >
where
    T: snowbridge_pallet_inbound_queue::Config
        + pallet_ethereum_token_transfers::Config
        + snowbridge_pallet_system::Config
        + pallet_xcm::Config,
    <T as frame_system::Config>::RuntimeEvent: From<pallet_xcm::Event<T>>,
    <T as frame_system::Config>::AccountId: Into<Location>,
    AssetTransactor: TransactAsset,
    EthereumLocation: Get<Location>,
    EthereumNetwork: Get<NetworkId>,
    InboundQueuePalletInstance: Get<u8>,
    TanssiLocationReanchored: Get<Location>,
{
    /// Decodes token data from payload and gets the corresponding token location.
    /// Returns different outcomes based on what succeeded or failed.
    fn get_token_data_and_location(payload: &[u8]) -> TokenDataResult {
        if let Some(token_data) = NativeTokenTransferData::decode_native_token_message(payload) {
            if let Some(token_location) =
                snowbridge_pallet_system::Pallet::<T>::convert(&token_data.token_id)
            {
                if token_location == TanssiLocationReanchored::get() {
                    // Extra safety check to forbid native Tanssi token for this processor
                    TokenDataResult::UnsupportedToken
                } else {
                    TokenDataResult::Success(token_data, token_location)
                }
            } else {
                TokenDataResult::LocationNotFound(token_data)
            }
        } else {
            TokenDataResult::DecodeFailure
        }
    }

    /// Validates that the destination para_id owns the token being transferred.
    fn validate_destination_owns_token(
        token_location: &Location,
        destination: &Destination,
    ) -> bool {
        // Extract para_id from destination - only foreign destinations are supported
        let expected_para_id = match destination {
            Destination::ForeignAccountId32 { para_id, .. } => *para_id,
            Destination::ForeignAccountId20 { para_id, .. } => *para_id,
            _ => {
                log::error!(
                    "NativeContainerTokensProcessor: unsupported destination type: {:?}",
                    destination
                );
                return false;
            }
        };

        let chain_part = token_location.interior().clone().split_global().ok();

        match chain_part {
            Some((_, interior)) => {
                if let Some(Parachain(id)) = interior.first() {
                    expected_para_id == *id
                } else {
                    log::error!(
                        "NativeContainerTokensProcessor: destination doesn't own the token!"
                    );
                    false
                }
            }
            _ => {
                log::error!("NativeContainerTokensProcessor: invalid chain part");
                false
            }
        }
    }

    /// Process a native container token transfer by creating and sending an XCM message to the destination parachain.
    fn process_native_container_token_transfer(
        token_data: NativeTokenTransferData,
        token_location: Location,
    ) {
        let interior = match token_location.interior().clone().split_global().ok() {
            Some((_, interior)) => interior,
            None => {
                log::error!(
                    "NativeContainerTokensProcessor: failed to split global on token location"
                );
                return;
            }
        };

        let (beneficiary, container_fee, container_para_id) = match token_data.destination {
            Destination::ForeignAccountId32 { para_id, id, fee } => {
                let beneficiary = Location::new(0, [AccountId32 { network: None, id }]);
                (beneficiary, fee, para_id)
            }
            Destination::ForeignAccountId20 { para_id, id, fee } => {
                let beneficiary = Location::new(
                    0,
                    [AccountKey20 {
                        network: None,
                        key: id,
                    }],
                );
                (beneficiary, fee, para_id)
            }
            _ => {
                log::error!("NativeContainerTokensProcessor::process_native_token_transfer: invalid destination");
                return;
            }
        };

        let container_location = Location::new(0, [Parachain(container_para_id)]);

        let container_token_from_tanssi = Location::new(0, interior);
        let token_location_reanchored = match container_token_from_tanssi.reanchored(
            &container_location,
            &<T as pallet_xcm::Config>::UniversalLocation::get(),
        ) {
            Ok(loc) => loc,
            Err(e) => {
                log::error!(
                        "NativeContainerTokensProcessor: failed to reanchor container token location: {:?}",
                        e
                    );
                return;
            }
        };

        // Fees are going to be paid with the native relay token on the container chain
        let asset_fee_relay: Asset = (Location::here(), container_fee).into();

        // Reanchor the asset fee to the container chain location
        let relay_asset_fee_container_context = match asset_fee_relay.clone().reanchored(
            &container_location,
            &<T as pallet_xcm::Config>::UniversalLocation::get(),
        ) {
            Ok(loc) => loc,
            Err(e) => {
                log::error!(
                    "NativeContainerTokensProcessor: failed to reanchor relay token location: {:?}",
                    e
                );
                return;
            }
        };

        let dummy_context = XcmContext {
            origin: None,
            message_id: Default::default(),
            topic: None,
        };

        // Transfer fee from FeesAccount to container sovereign account
        if let Err(e) = AssetTransactor::transfer_asset(
            &asset_fee_relay,
            &T::FeesAccount::get().into(),
            &container_location,
            &dummy_context,
        ) {
            log::error!(
                "NativeContainerTokensProcessor: failed to transfer fee from FeesAccount to container sovereign account: {:?}",
                e
            );
            return;
        }

        // Reanchor Ethereum location to the container chain's point of view
        let bridge_location = match EthereumLocation::get().clone().reanchored(
            &container_location,
            &<T as pallet_xcm::Config>::UniversalLocation::get(),
        ) {
            Ok(loc) => loc,
            Err(e) => {
                log::error!(
                    "NativeContainerTokensProcessor: failed to reanchor bridge location: {:?}",
                    e
                );
                return;
            }
        };

        let container_asset: Asset = (token_location_reanchored.clone(), token_data.amount).into();
        let inbound_queue_pallet_index = InboundQueuePalletInstance::get();

        let remote_xcm = Xcm::<()>(vec![
            ReserveAssetDeposited(vec![relay_asset_fee_container_context.clone()].into()),
            BuyExecution {
                fees: relay_asset_fee_container_context,
                weight_limit: Unlimited,
            },
            DescendOrigin(PalletInstance(inbound_queue_pallet_index).into()),
            UniversalOrigin(GlobalConsensus(EthereumNetwork::get())),
            WithdrawAsset(vec![container_asset.clone()].into()),
            DepositAsset {
                assets: Definite(container_asset.into()),
                beneficiary,
            },
            // When the execution finishes deposit any leftover fees to the ETH
            // sovereign account on destination.
            SetAppendix(Xcm(vec![DepositAsset {
                assets: Wild(AllOf {
                    id: Location::parent().into(),
                    fun: WildFungibility::Fungible,
                }),
                beneficiary: bridge_location,
            }])),
        ]);

        send_xcm::<<T as pallet_xcm::Config>::XcmRouter>(
            container_location.clone(),
            remote_xcm.clone(),
        )
        .map(|(message_id, _price)| {
            let xcm_event: pallet_xcm::Event<T> = pallet_xcm::Event::Sent {
                origin: Here.into_location(),
                destination: container_location,
                message: remote_xcm,
                message_id,
            };
            frame_system::Pallet::<T>::deposit_event(
                <T as frame_system::Config>::RuntimeEvent::from(xcm_event),
            );
        })
        .map_err(|e| {
            log::error!(
                "NativeContainerTokensProcessor: XCM send failed with error: {:?}",
                e
            );
        })
        .ok();
    }
}
