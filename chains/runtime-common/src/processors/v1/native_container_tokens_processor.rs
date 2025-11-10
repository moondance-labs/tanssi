extern crate alloc;

use alloc::vec;
use core::marker::PhantomData;
use snowbridge_core::Channel;
use sp_core::Get;
use sp_runtime::{traits::MaybeEquivalence, DispatchError, DispatchResult};
use xcm::prelude::*;

use crate::processors::v1::{GatewayAndChannelValidator, NativeTokenTransferData};
use snowbridge_inbound_queue_primitives::v1::{Destination, Envelope, MessageProcessor};

/// `NativeContainerTokensProcessor` is responsible for receiving and processing native container
/// chain tokens coming from Ethereum and forwarding them to the container chain via Tanssi through XCM.
pub struct NativeContainerTokensProcessor<
    T,
    EthereumLocation,
    EthereumNetwork,
    InboundQueuePalletInstance,
    TanssiLocationReanchored,
>(
    PhantomData<(
        T,
        EthereumLocation,
        EthereumNetwork,
        InboundQueuePalletInstance,
        TanssiLocationReanchored,
    )>,
);

impl<
        T,
        EthereumLocation,
        EthereumNetwork,
        InboundQueuePalletInstance,
        TanssiLocationReanchored,
    > MessageProcessor
    for NativeContainerTokensProcessor<
        T,
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
        EthereumLocation,
        EthereumNetwork,
        InboundQueuePalletInstance,
        TanssiLocationReanchored,
    >
    NativeContainerTokensProcessor<
        T,
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
        let token_split = token_location.interior().clone().split_global().ok();
        if let Some((_, interior)) = token_split {
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
            let reanchor_result = container_token_from_tanssi.reanchored(
                &container_location,
                &<T as pallet_xcm::Config>::UniversalLocation::get(),
            );

            if let Ok(token_location_reanchored) = reanchor_result {
                let network = EthereumNetwork::get();

                let total_container_asset = token_data.amount.saturating_add(container_fee);
                let container_asset_to_withdraw: Asset =
                    (token_location_reanchored.clone(), total_container_asset).into();
                let container_asset_fee: Asset =
                    (token_location_reanchored.clone(), container_fee).into();
                let container_asset_to_deposit: Asset =
                    (token_location_reanchored.clone(), token_data.amount).into();

                let inbound_queue_pallet_index = InboundQueuePalletInstance::get();

                let remote_xcm = Xcm::<()>(vec![
                    DescendOrigin(PalletInstance(inbound_queue_pallet_index).into()),
                    UniversalOrigin(GlobalConsensus(network)),
                    WithdrawAsset(vec![container_asset_to_withdraw.clone()].into()),
                    BuyExecution {
                        fees: container_asset_fee,
                        weight_limit: Unlimited,
                    },
                    DepositAsset {
                        assets: Definite(container_asset_to_deposit.into()),
                        beneficiary,
                    },
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
            } else {
                log::error!("NativeContainerTokensProcessor: failed to reanchor token location");
            }
        } else {
            log::error!("NativeContainerTokensProcessor: failed to reanchor token location");
        }
    }
}
