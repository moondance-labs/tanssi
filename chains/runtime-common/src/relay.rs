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

//! Shared code between relay runtimes.

extern crate alloc;

use alloc::{vec, vec::Vec};
use core::marker::PhantomData;
use frame_support::{
    pallet_prelude::Zero,
    traits::{
        fungible::{Inspect, Mutate},
        tokens::{Fortitude, Preservation},
    },
};
use frame_system::pallet_prelude::BlockNumberFor;
use parity_scale_codec::{DecodeAll, Encode, EncodeLike};
use snowbridge_core::Channel;
use snowbridge_pallet_inbound_queue::RewardProcessor;
use sp_core::{Get, H160, H256};
use sp_runtime::{
    traits::{Hash as _, MaybeEquivalence},
    DispatchError, DispatchResult,
};
use xcm::latest::{
    prelude::*, Asset as XcmAsset, AssetId as XcmAssetId, Assets as XcmAssets, ExecuteXcm,
    Fungibility, Junctions::*,
};
use xcm_executor::traits::WeightBounds;
use {
    snowbridge_inbound_queue_primitives::v1::{
        Command, Destination, Envelope, MessageProcessor, MessageV1, VersionedXcmMessage,
    },
    snowbridge_inbound_queue_primitives::EventProof as Message,
};

/// Validates the gateway and channel of an inbound envelope
pub struct GatewayAndChannelValidator<T>(PhantomData<T>);
impl<T> GatewayAndChannelValidator<T>
where
    T: snowbridge_pallet_inbound_queue::Config + pallet_ethereum_token_transfers::Config,
{
    pub fn validate_gateway_and_channel(channel: &Channel, envelope: &Envelope) -> bool {
        // Ensure that the message is intended for the current channel, para_id and agent_id
        if let Some(channel_info) = pallet_ethereum_token_transfers::CurrentChannelInfo::<T>::get()
        {
            if envelope.channel_id != channel_info.channel_id
                || channel.para_id != channel_info.para_id
                || channel.agent_id != channel_info.agent_id
            {
                log::debug!(
                    "Unexpected channel id: {:?} != {:?}",
                    (envelope.channel_id, channel.para_id, channel.agent_id),
                    (
                        channel_info.channel_id,
                        channel_info.para_id,
                        channel_info.agent_id
                    )
                );
                return false;
            }
        } else {
            log::warn!("CurrentChannelInfo not set in storage");
            return false;
        }

        // Check it is from the right gateway
        if envelope.gateway != T::GatewayAddress::get() {
            log::warn!("Wrong gateway address: {:?}", envelope.gateway);
            return false;
        }
        true
    }
}

/// Information needed to process a native token transfer message from ethereum.
pub struct NativeTokenTransferData {
    pub token_id: H256,
    pub destination: Destination,
    pub amount: u128,
    pub fee: u128,
}

impl NativeTokenTransferData {
    pub fn decode_native_token_message(mut payload: &[u8]) -> Option<Self> {
        match VersionedXcmMessage::decode_all(&mut payload) {
            Ok(VersionedXcmMessage::V1(MessageV1 {
                command:
                    Command::SendNativeToken {
                        token_id,
                        destination,
                        amount,
                        fee,
                    },
                ..
            })) => Some(NativeTokenTransferData {
                token_id,
                destination,
                amount,
                fee,
            }),
            Ok(msg) => {
                log::trace!("NativeTokenTransferData: unexpected message: {:?}", msg);
                None
            }
            Err(e) => {
                log::trace!("NativeTokenTransferData: failed to decode message. This is expected if the message is not related to a SendNativeToken command. Error: {:?}", e);
                None
            }
        }
    }
}

/// `NativeTokenTransferMessageProcessor` is responsible for receiving and processing the Tanssi
/// native token sent from Ethereum. If the message is valid, it performs the token transfer
/// from the Ethereum sovereign account to the specified destination account.
pub struct NativeTokenTransferMessageProcessor<T>(PhantomData<T>);
impl<T> MessageProcessor for NativeTokenTransferMessageProcessor<T>
where
    T: snowbridge_pallet_inbound_queue::Config
        + pallet_ethereum_token_transfers::Config
        + snowbridge_pallet_system::Config,
    T::AccountId: From<[u8; 32]>,
{
    fn can_process_message(channel: &Channel, envelope: &Envelope) -> bool {
        if !GatewayAndChannelValidator::<T>::validate_gateway_and_channel(channel, envelope) {
            log::warn!("NativeTokenTransferMessageProcessor: invalid gateway or channel");
            return false;
        }

        // Try decode the message and check the token id is the expected one
        if let Some(token_data) =
            NativeTokenTransferData::decode_native_token_message(&envelope.payload)
        {
            let token_location = T::TokenLocationReanchored::get();

            if let Some(expected_token_id) =
                snowbridge_pallet_system::Pallet::<T>::convert_back(&token_location)
            {
                if token_data.token_id == expected_token_id {
                    true
                } else {
                    // TODO: ensure this does not warn on container token transfers or other message types, if yes change to debug
                    log::warn!(
                        "NativeTokenTransferMessageProcessor: unexpected token_id: {:?}",
                        token_data.token_id
                    );
                    false
                }
            } else {
                log::warn!(
                    "NativeTokenTransferMessageProcessor: token id not found for location: {:?}",
                    token_location
                );
                false
            }
        } else {
            false
        }
    }

    fn process_message(_channel: Channel, envelope: Envelope) -> DispatchResult {
        // Decode payload as SendNativeToken using the helper function
        if let Some(token_data) =
            NativeTokenTransferData::decode_native_token_message(&envelope.payload)
        {
            log::trace!("NativeTokenTransferMessageProcessor: processing token transfer: token_id={:?}, amount={}, destination={:?}", 
                token_data.token_id, token_data.amount, token_data.destination);

            match token_data.destination {
                Destination::AccountId32 {
                    id: destination_account,
                } => {
                    // Transfer the amounts of tokens from Ethereum sov account to the destination
                    let sovereign_account = T::EthereumSovereignAccount::get();

                    if let Err(e) = T::Currency::transfer(
                        &sovereign_account,
                        &destination_account.into(),
                        token_data.amount.into(),
                        Preservation::Preserve,
                    ) {
                        log::warn!(
                            "NativeTokenTransferMessageProcessor: Error transferring tokens: {:?}",
                            e
                        );
                    }

                    Ok(())
                }
                _ => {
                    log::warn!(
                        "NativeTokenTransferMessageProcessor: unsupported destination type: {:?}",
                        token_data.destination
                    );
                    Ok(())
                }
            }
        } else {
            log::trace!("NativeTokenTransferMessageProcessor: failed to decode message. This is expected if the message is not for this processor.");
            Err(DispatchError::Other("unable to parse the envelope payload"))
        }
    }
}

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

/// Rewards the relayer that processed a native token transfer message
/// using the FeesAccount configured in pallet_ethereum_token_transfers
pub struct RewardThroughFeesAccount<T>(PhantomData<T>);

impl<T> RewardProcessor<T> for RewardThroughFeesAccount<T>
where
    T: snowbridge_pallet_inbound_queue::Config + pallet_ethereum_token_transfers::Config,
    T::AccountId: From<sp_runtime::AccountId32>,
    <T::Token as Inspect<T::AccountId>>::Balance: core::fmt::Debug,
{
    fn process_reward(who: T::AccountId, _channel: Channel, message: Message) -> DispatchResult {
        let reward_amount = snowbridge_pallet_inbound_queue::Pallet::<T>::calculate_delivery_cost(
            message.encode().len() as u32,
        );

        let fees_account: T::AccountId = T::FeesAccount::get();

        let amount =
            T::Token::reducible_balance(&fees_account, Preservation::Preserve, Fortitude::Polite)
                .min(reward_amount);

        if amount != reward_amount {
            log::warn!(
                "RewardThroughFeesAccount: fees account running low on funds {:?}: {:?}",
                fees_account,
                amount
            );
        }

        if !amount.is_zero() {
            T::Token::transfer(&fees_account, &who, amount, Preservation::Preserve)?;
        }

        Ok(())
    }
}

pub struct BabeSlotBeacon<T>(PhantomData<T>);
impl<T: pallet_babe::Config> sp_runtime::traits::BlockNumberProvider for BabeSlotBeacon<T> {
    type BlockNumber = u32;

    fn current_block_number() -> Self::BlockNumber {
        // TODO: nimbus_primitives::SlotBeacon requires u32, but this is a u64 in pallet_babe, and
        // also it gets converted to u64 in pallet_author_noting, so let's do something to remove
        // this intermediate u32 conversion, such as using a different trait
        u64::from(pallet_babe::CurrentSlot::<T>::get()) as u32
    }
}

/// Combines the vrf output of the previous block with the provided subject.
/// This ensures that the randomness will be different on different pallets, as long as the subject is different.
pub fn mix_randomness<T: frame_system::Config>(vrf_output: [u8; 32], subject: &[u8]) -> T::Hash {
    let mut digest = Vec::new();
    digest.extend_from_slice(vrf_output.as_ref());
    digest.extend_from_slice(subject);

    T::Hashing::hash(digest.as_slice())
}

pub struct BabeAuthorVrfBlockRandomness<T>(PhantomData<T>);
impl<T: pallet_babe::Config + frame_system::Config> BabeAuthorVrfBlockRandomness<T> {
    pub fn get_block_randomness() -> Option<[u8; 32]> {
        // In a relay context we get block randomness from Babe's AuthorVrfRandomness
        pallet_babe::Pallet::<T>::author_vrf_randomness()
    }

    pub fn get_block_randomness_mixed(subject: &[u8]) -> Option<T::Hash> {
        Self::get_block_randomness().map(|random_hash| mix_randomness::<T>(random_hash, subject))
    }
}

impl<T: pallet_babe::Config + frame_system::Config>
    frame_support::traits::Randomness<T::Hash, BlockNumberFor<T>>
    for BabeAuthorVrfBlockRandomness<T>
{
    fn random(subject: &[u8]) -> (T::Hash, BlockNumberFor<T>) {
        let block_number = frame_system::Pallet::<T>::block_number();
        let randomness = Self::get_block_randomness_mixed(subject).unwrap_or_default();

        (randomness, block_number)
    }
}

pub struct BabeGetCollatorAssignmentRandomness<T>(PhantomData<T>);
impl<T: pallet_babe::Config + frame_system::Config> Get<[u8; 32]>
    for BabeGetCollatorAssignmentRandomness<T>
{
    fn get() -> [u8; 32] {
        let block_number = frame_system::Pallet::<T>::block_number();
        let random_seed = if !block_number.is_zero() {
            if let Some(random_hash) = {
                BabeAuthorVrfBlockRandomness::<T>::get_block_randomness_mixed(b"CollatorAssignment")
            } {
                // Return random_hash as a [u8; 32] instead of a Hash
                let mut buf = [0u8; 32];
                let len = core::cmp::min(32, random_hash.as_ref().len());
                buf[..len].copy_from_slice(&random_hash.as_ref()[..len]);

                buf
            } else {
                // If there is no randomness return [0; 32]
                [0; 32]
            }
        } else {
            // In block 0 (genesis) there is no randomness
            [0; 32]
        };

        random_seed
    }
}

/// `EthTokensLocalProcessor` is responsible for receiving and processing the ETH native
/// token and ERC20s coming from Ethereum with Tanssi chain or container-chains as final destinations.
/// TODO: add support for container transfers
pub struct EthTokensLocalProcessor<
    T,
    XcmProcessor,
    XcmWeigher,
    EthereumLocation,
    EthereumNetwork,
    InboundQueuePalletInstance,
    ContainerTransfersEnabled, // TODO: remove this when both runtimes support container transfers :) 
>(
    PhantomData<(
        T,
        XcmProcessor,
        XcmWeigher,
        EthereumLocation,
        EthereumNetwork,
        InboundQueuePalletInstance,
        ContainerTransfersEnabled,
    )>,
);

impl<
        T,
        XcmProcessor,
        XcmWeigher,
        EthereumLocation,
        EthereumNetwork,
        InboundQueuePalletInstance,
        ContainerTransfersEnabled,
    > MessageProcessor
    for EthTokensLocalProcessor<
        T,
        XcmProcessor,
        XcmWeigher,
        EthereumLocation,
        EthereumNetwork,
        InboundQueuePalletInstance,
        ContainerTransfersEnabled,
    >
where
    T: snowbridge_pallet_inbound_queue::Config
        + pallet_ethereum_token_transfers::Config
        + pallet_foreign_asset_creator::Config
        + pallet_xcm::Config,
    <T as frame_system::Config>::RuntimeEvent: From<pallet_xcm::Event<T>>,
    XcmProcessor: ExecuteXcm<<T as pallet_xcm::Config>::RuntimeCall>,
    XcmWeigher: WeightBounds<<T as pallet_xcm::Config>::RuntimeCall>,
    EthereumLocation: Get<Location>,
    EthereumNetwork: Get<NetworkId>,
    InboundQueuePalletInstance: Get<u8>,
    ContainerTransfersEnabled: Get<bool>,
    cumulus_primitives_core::Location:
        EncodeLike<<T as pallet_foreign_asset_creator::Config>::ForeignAsset>,
{
    fn can_process_message(channel: &Channel, envelope: &Envelope) -> bool {
        if let Some(channel_info) = pallet_ethereum_token_transfers::CurrentChannelInfo::<T>::get()
        {
            if envelope.channel_id != channel_info.channel_id
                || channel.para_id != channel_info.para_id
                || channel.agent_id != channel_info.agent_id
            {
                return false;
            }
        } else {
            return false;
        }

        if envelope.gateway != T::GatewayAddress::get() {
            return false;
        }

        if let Some(eth_transfer_data) =
            Self::decode_message_for_eth_transfer(envelope.payload.as_slice())
        {
            return pallet_foreign_asset_creator::ForeignAssetToAssetId::<T>::get(
                eth_transfer_data.token_location,
            )
            .is_some();
        }

        false
    }

    fn process_message(_channel: Channel, envelope: Envelope) -> DispatchResult {
        let eth_transfer_data = Self::decode_message_for_eth_transfer(envelope.payload.as_slice())
            .ok_or(DispatchError::Other("unexpected message"))?;

        match eth_transfer_data.destination {
            Destination::AccountId32 { id: _ } => {
                Self::process_xcm_local_native_eth_transfer(eth_transfer_data)
            }
            Destination::ForeignAccountId32 { .. } | Destination::ForeignAccountId20 { .. } => {
                if ContainerTransfersEnabled::get() {
                    Self::process_xcm_local_container_eth_transfer(eth_transfer_data)
                } else {
                    log::error!("EthTokensLocalProcessor: container transfers not supported yet");
                    return Ok(());
                }
            }
        }
    }
}

pub struct EthTransferData {
    pub token_location: Location,
    pub destination: Destination,
    pub amount: u128,
}

impl<
        T,
        XcmProcessor,
        XcmWeigher,
        EthereumLocation,
        EthereumNetwork,
        InboundQueuePalletInstance,
        ContainerTransfersEnabled,
    >
    EthTokensLocalProcessor<
        T,
        XcmProcessor,
        XcmWeigher,
        EthereumLocation,
        EthereumNetwork,
        InboundQueuePalletInstance,
        ContainerTransfersEnabled,
    >
where
    T: frame_system::Config + pallet_xcm::Config,
    <T as frame_system::Config>::RuntimeEvent: From<pallet_xcm::Event<T>>,
    XcmProcessor: ExecuteXcm<<T as pallet_xcm::Config>::RuntimeCall>,
    XcmWeigher: WeightBounds<<T as pallet_xcm::Config>::RuntimeCall>,
    EthereumLocation: Get<Location>,
    EthereumNetwork: Get<NetworkId>,
    InboundQueuePalletInstance: Get<u8>,
    ContainerTransfersEnabled: Get<bool>,
{
    pub fn decode_message_for_eth_transfer(mut payload: &[u8]) -> Option<EthTransferData> {
        match VersionedXcmMessage::decode_all(&mut payload) {
            Ok(VersionedXcmMessage::V1(MessageV1 {
                command:
                    Command::SendToken {
                        token: token_address,
                        destination,
                        amount,
                        fee: _,
                    },
                ..
            })) => {
                let token_location = if token_address == H160::zero() {
                    Location {
                        parents: 1,
                        interior: X1([GlobalConsensus(EthereumNetwork::get())].into()),
                    }
                } else {
                    Location {
                        parents: 1,
                        interior: X2([
                            GlobalConsensus(EthereumNetwork::get()),
                            AccountKey20 {
                                network: Some(EthereumNetwork::get()),
                                key: token_address.into(),
                            },
                        ]
                        .into()),
                    }
                };

                Some(EthTransferData {
                    token_location,
                    destination,
                    amount,
                })
            }
            _ => None,
        }
    }

    fn process_xcm_local_native_eth_transfer(eth_transfer_data: EthTransferData) -> DispatchResult {
        let assets_to_holding: XcmAssets = vec![XcmAsset {
            id: XcmAssetId::from(eth_transfer_data.token_location),
            fun: Fungibility::Fungible(eth_transfer_data.amount),
        }]
        .into();

        let destination_account = match eth_transfer_data.destination {
            Destination::AccountId32 { id } => id,
            _ => {
                log::error!("EthTokensLocalProcessor: invalid destination");
                return Ok(());
            }
        };

        let mut xcm = Xcm::<<T as pallet_xcm::Config>::RuntimeCall>(vec![
            ReserveAssetDeposited(assets_to_holding),
            DepositAsset {
                assets: AllCounted(1).into(),
                beneficiary: Location::new(
                    0,
                    [AccountId32 {
                        network: None,
                        id: destination_account,
                    }],
                ),
            },
        ]);

        let ethereum_location = EthereumLocation::get();

        if let Ok(weight) = XcmWeigher::weight(&mut xcm) {
            let mut message_id = xcm.using_encoded(sp_io::hashing::blake2_256);

            let outcome = XcmProcessor::prepare_and_execute(
                ethereum_location,
                xcm,
                &mut message_id,
                weight,
                weight,
            );

            if let Err(error) = outcome.ensure_complete() {
                log::error!(
                    "EthTokensLocalProcessor: XCM execution failed with error {:?}",
                    error
                );
            }
        } else {
            log::error!("EthTokensLocalProcessor: unweighable message");
        }

        Ok(())
    }

    fn process_xcm_local_container_eth_transfer(
        eth_transfer_data: EthTransferData,
    ) -> DispatchResult {
        let (para_id, beneficiary, fee) = match eth_transfer_data.destination {
            Destination::ForeignAccountId32 { para_id, id, fee } => (
                para_id,
                Location::new(0, [AccountId32 { network: None, id }]),
                fee,
            ),
            Destination::ForeignAccountId20 { para_id, id, fee } => (
                para_id,
                Location::new(
                    0,
                    [AccountKey20 {
                        network: None,
                        key: id,
                    }],
                ),
                fee,
            ),
            _ => {
                log::error!(
                    "EthTokensLocalProcessor: unsupported destination for container transfer: {:?}",
                    eth_transfer_data.destination
                );
                return Ok(());
            }
        };

        let container_location = Location::new(0, [Parachain(para_id)]);

        let token_reanchored = match eth_transfer_data.token_location.reanchored(
            &container_location,
            &<T as pallet_xcm::Config>::UniversalLocation::get(),
        ) {
            Ok(loc) => loc,
            Err(e) => {
                log::error!(
                    "EthTokensLocalProcessor: failed to reanchor token location: {:?}",
                    e
                );
                return Ok(());
            }
        };

        let asset_fee: Asset = (container_location.clone(), fee).into();
        let asset_to_deposit: Asset = (token_reanchored.clone(), eth_transfer_data.amount).into();

        let inbound_queue_pallet_index = InboundQueuePalletInstance::get();
        let network = EthereumNetwork::get();

        let remote_xcm = Xcm::<()>(vec![
            DescendOrigin(PalletInstance(inbound_queue_pallet_index).into()),
            UniversalOrigin(GlobalConsensus(network)),
            WithdrawAsset(vec![asset_to_deposit.clone(), container_location.clone()].into()),
            BuyExecution {
                fees: asset_fee,
                weight_limit: Unlimited,
            },
            DepositAsset {
                assets: Definite(vec![asset_to_deposit].into()),
                beneficiary,
            },
        ]);

        match send_xcm::<<T as pallet_xcm::Config>::XcmRouter>(
            container_location.clone(),
            remote_xcm.clone(),
        ) {
            Ok((message_id, _price)) => {
                let evt: pallet_xcm::Event<T> = pallet_xcm::Event::Sent {
                    origin: Here.into_location(),
                    destination: container_location,
                    message: remote_xcm,
                    message_id,
                };
                frame_system::Pallet::<T>::deposit_event(
                    <T as frame_system::Config>::RuntimeEvent::from(evt),
                );
            }
            Err(e) => {
                log::error!(
                    "EthTokensLocalProcessor: XCM send failed to para_id {} with error: {:?}",
                    para_id,
                    e
                );
            }
        };

        Ok(())
    }
}