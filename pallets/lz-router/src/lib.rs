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

//! # LayerZero Router Pallet
//!
//! Routes LayerZero messages between container chains and external chains (Ethereum, etc.)
//! via the relay chain, using Snowbridge as the underlying bridge infrastructure.
//!
//! ## Overview
//!
//! This pallet acts as a router on the relay chain, enabling container chains (parachains)
//! to send and receive LayerZero messages to/from external chains. It integrates with
//! Snowbridge to communicate with Ethereum, which serves as a hub for LayerZero messaging.
//!
//! ## Message Flows
//!
//! ### Inbound: External Chain → Ethereum → Relay (Router) → Container Chain
//!
//! 1. External chain sends LayerZero message to Ethereum
//! 2. Snowbridge relays the message to the relay chain
//! 3. `LayerZeroInboundMessageProcessorV2` calls `handle_inbound_message`
//! 4. Router validates sender is whitelisted for the destination chain
//! 5. Router sends XCM `Transact` to the destination container chain
//! 6. Container chain's configured pallet receives the message
//!
//! ### Outbound: Container Chain → Relay (Router) → Ethereum → External Chain
//!
//! 1. Container chain calls `send_message_to_ethereum` via XCM
//! 2. Router validates origin, minimum reward, and transfers fee from sovereign account
//! 3. Router creates LayerZero message envelope (with magic bytes)
//! 4. Router ABI-encodes the envelope and creates `CallContract` command
//! 5. Router sends to Snowbridge V2 outbound queue targeting LayerZero hub on Ethereum
//! 6. Snowbridge relays to Ethereum, which forwards via LayerZero to the destination
//!
//! ## Configuration
//!
//! ### For Container Chains (Inbound)
//!
//! Call `update_routing_config` via XCM to configure:
//! - **Whitelisted senders**: `(LayerZeroEndpoint, LayerZeroAddress)` tuples allowed to send
//! - **Notification destination**: `(pallet_index, call_index)` to receive messages
//!
//! Example: A container chain can whitelist an Ethereum contract at LayerZero endpoint 30101
//! and specify that messages should be delivered to pallet 79, call 0.
//!
//! ### For Relay Chain (Outbound)
//!
//! The relay chain runtime must configure:
//! - `LayerZeroHubAddress`: Address of the LayerZero hub contract on Ethereum
//! - `MinOutboundReward`: Minimum fee for sending messages
//! - `FeesAccount`: Account where routing fees are deposited
//!
//! ## Security
//!
//! - Container chains control their own routing configuration (via XCM)
//! - Whitelist enforcement prevents unauthorized message delivery
//! - Sovereign account funds are used for outbound fees (not user accounts)
//! - Minimum reward requirement prevents spam/DoS
//!
//! ## See Also
//!
//! - `pallet-lz-receiver-example`: Reference implementation for receiving messages
//! - `tp-bridge::layerzero_message`: Message type definitions and encoding
//! - Snowbridge pallets: Underlying bridge infrastructure

#![cfg_attr(not(feature = "std"), no_std)]

mod types;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

extern crate alloc;

use crate::types::ChainId;
use alloy_core::sol_types::SolValue;
pub use pallet::*;
use xcm::latest::Location;
use xcm::prelude::{Parachain, Unlimited};
use {
    alloc::vec::Vec,
    frame_support::{
        pallet_prelude::BoundedVec,
        traits::{
            fungible::{Inspect, Mutate},
            tokens::Preservation,
        },
    },
    frame_system::unique,
    parity_scale_codec::Encode,
    snowbridge_outbound_queue_primitives::v2::{
        Command as SnowbridgeCommandV2, Message as SnowbridgeMessageV2,
        SendMessage as SendMessageV2,
    },
    snowbridge_outbound_queue_primitives::SendError,
    sp_core::{H160, H256},
    sp_runtime::traits::Get,
    tp_bridge::{
        layerzero_message::{
            InboundMessage, LayerZeroAddress, LayerZeroEndpoint, LayerZeroOutboundPayload,
            OutboundMessage, OutboundSolMessageEnvelope,
        },
        ConvertLocation, TicketInfo,
    },
    xcm::prelude::InteriorLocation,
    xcm_executor::traits::ConvertLocation as XcmConvertLocation,
};

/// Extract the container chain (parachain) ID from an XCM location.
///
/// Validates that the location represents a direct parachain (same consensus, no hops)
/// and extracts its para ID.
///
/// ## Expected Format:
/// - Parents: 0 (same consensus)
/// - Interior: Single `Parachain(id)` junction
///
/// ## Parameters:
/// - `location`: The XCM location to extract from
///
/// ## Returns:
/// - `Some(ChainId)`: The parachain ID if the location is valid
/// - `None`: If the location doesn't represent a container chain
fn extract_container_chain_id(location: &Location) -> Option<ChainId> {
    match location.unpack() {
        (0, [Parachain(id)]) => Some(*id),
        _ => None,
    }
}

#[frame_support::pallet]
pub mod pallet {
    use crate::types::{ChainId, RoutingConfig};
    use alloc::vec;
    use snowbridge_inbound_queue_primitives::v2::MessageProcessorError;
    use xcm::latest::{send_xcm, Location, Reanchorable, Xcm};
    use xcm::prelude::{OriginKind, Transact, UnpaidExecution};
    use {
        super::*,
        frame_support::{pallet_prelude::*, traits::EnsureOrigin},
        frame_system::pallet_prelude::*,
    };

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_xcm::Config {
        #[pallet::constant]
        type MaxWhitelistedSenders: Get<u32> + Clone;
        /// Origin locations allowed to update routing configurations
        type ContainerChainOrigin: EnsureOrigin<
            <Self as frame_system::Config>::RuntimeOrigin,
            Success = Location,
        >;

        /// Validate and send a message to Ethereum V2.
        type OutboundQueueV2: SendMessageV2<Ticket: TicketInfo>;

        /// The address of the LayerZero hub contract on Ethereum.
        #[pallet::constant]
        type LayerZeroHubAddress: Get<H160>;

        /// Minimum reward for outbound messages.
        #[pallet::constant]
        type MinOutboundReward: Get<u128>;

        /// Converts Location to H256 for message origin.
        type LocationHashOf: ConvertLocation<H256>;

        /// The bridge's configured Ethereum location.
        type EthereumLocation: Get<Location>;

        /// This chain's Universal Location.
        type UniversalLocation: Get<InteriorLocation>;

        /// Currency for handling fee transfers.
        type Currency: Inspect<Self::AccountId, Balance = u128> + Mutate<Self::AccountId>;

        /// Account where fees are deposited.
        type FeesAccount: Get<Self::AccountId>;

        /// Converts a Location to an AccountId (for sovereign accounts).
        type LocationToAccountId: XcmConvertLocation<Self::AccountId>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Routing configuration per container chain
    #[pallet::storage]
    pub type RoutingConfigs<T: Config> =
        StorageMap<_, Twox64Concat, ChainId, RoutingConfig<T>, OptionQuery>;

    #[pallet::error]
    pub enum Error<T> {
        /// The provided origin location is not a container chain
        LocationIsNotAContainerChain,
        /// No routing configuration found for the destination chain
        NoRoutingConfig,
        /// The sender (address+endpoint) is not whitelisted to forward messages to the destination chain
        NotWhitelistedSender,
        /// Setting the same configuration that already exists
        SameConfigAlreadyExists,
        /// The outbound message is invalid prior to send.
        InvalidMessage(SendError),
        /// The outbound message could not be sent.
        MessageNotSent(SendError),
        /// Too many commands in the message.
        TooManyCommands,
        /// The reward provided is below the minimum required.
        MinRewardNotAchieved,
        /// Failed to convert location to origin hash.
        LocationToOriginConversionFailed,
        /// Failed to reanchor location.
        LocationReanchorFailed,
        /// Failed to convert location to account ID.
        LocationToAccountConversionFailed,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(crate) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Routing configuration updated for a container chain
        RoutingConfigUpdated {
            chain_id: ChainId,
            new_config: RoutingConfig<T>,
            old_config: Option<RoutingConfig<T>>,
        },
        /// Inbound message routed to a container chain
        InboundMessageRouted {
            chain_id: ChainId,
            message: InboundMessage,
        },
        /// Outbound message sent to Ethereum/LayerZero
        OutboundMessageSent {
            message_id: H256,
            message: OutboundMessage,
            reward: u128,
            gas: u64,
        },
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Update routing configuration for a container chain.
        ///
        /// Configures how the container chain receives inbound LayerZero messages.
        /// Must be called via XCM from the container chain itself.
        ///
        /// The configuration specifies:
        /// - **Whitelisted Senders**: `(LayerZeroEndpoint, LayerZeroAddress)` tuples allowed to send
        /// - **Notification Destination**: `(pallet_index, call_index)` to handle incoming messages
        ///
        /// Emits `RoutingConfigUpdated` event.
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn update_routing_config(
            origin: OriginFor<T>,
            new_config: RoutingConfig<T>,
        ) -> DispatchResult {
            let origin_location = T::ContainerChainOrigin::ensure_origin(origin)?;
            let chain_id = extract_container_chain_id(&origin_location)
                .ok_or(Error::<T>::LocationIsNotAContainerChain)?;

            let old_config = RoutingConfigs::<T>::get(chain_id);
            ensure!(
                old_config != Some(new_config.clone()),
                Error::<T>::SameConfigAlreadyExists
            );

            RoutingConfigs::<T>::insert(chain_id, new_config.clone());

            Self::deposit_event(Event::RoutingConfigUpdated {
                chain_id,
                new_config,
                old_config,
            });

            Ok(())
        }

        /// Send an outbound message to Ethereum/LayerZero.
        ///
        /// Called via XCM from a container chain to send a LayerZero message to an external chain.
        /// The message is ABI-encoded and sent as a `CallContract` to the LayerZero hub on Ethereum.
        ///
        /// The reward is transferred from the container chain's sovereign account to the fees account.
        ///
        /// Emits `OutboundMessageSent` event.
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn send_message_to_ethereum(
            origin: OriginFor<T>,
            lz_destination_address: LayerZeroAddress,
            lz_destination_endpoint: LayerZeroEndpoint,
            payload: LayerZeroOutboundPayload,
            reward: u128,
            gas: u64,
        ) -> DispatchResult {
            let origin_location = T::ContainerChainOrigin::ensure_origin(origin)?;
            let source_chain = extract_container_chain_id(&origin_location)
                .ok_or(Error::<T>::LocationIsNotAContainerChain)?;

            // Check for minimum reward
            ensure!(
                reward >= T::MinOutboundReward::get(),
                Error::<T>::MinRewardNotAchieved
            );

            // Get the sovereign account of the container chain
            let sovereign_account = T::LocationToAccountId::convert_location(&origin_location)
                .ok_or(Error::<T>::LocationToAccountConversionFailed)?;

            // Transfer fee from container chain's sovereign account to fees account
            <T as Config>::Currency::transfer(
                &sovereign_account,
                &T::FeesAccount::get(),
                reward,
                Preservation::Preserve,
            )?;

            // Build the outbound message
            let message = OutboundMessage {
                source_chain,
                lz_destination_address: lz_destination_address.clone(),
                lz_destination_endpoint,
                payload,
            };

            // Convert to ABI-encodable envelope (includes magic bytes)
            let envelope: OutboundSolMessageEnvelope = message.clone().into();
            // TODO: incode the function selctor also
            // | 4 bytes  |   N × 32 bytes |
            // | selector | ABI-encoded arguments |
            let calldata = envelope.abi_encode();

            // Create CallContract command targeting the LayerZero hub on Ethereum
            let command = SnowbridgeCommandV2::CallContract {
                target: T::LayerZeroHubAddress::get(),
                calldata,
                gas,
                value: 0, // No ETH value sent with the call
            };

            // Convert location to message origin (reanchored relative to Ethereum)
            let origin = Self::location_to_message_origin(origin_location)?;
            let id = unique((origin, &command)).into();

            let commands: Vec<SnowbridgeCommandV2> = vec![command];

            let snowbridge_message = SnowbridgeMessageV2 {
                id,
                commands: BoundedVec::try_from(commands)
                    .map_err(|_| Error::<T>::TooManyCommands)?,
                fee: reward,
                origin,
            };

            // Validate and deliver the message
            let ticket = T::OutboundQueueV2::validate(&snowbridge_message)
                .map_err(|err| Error::<T>::InvalidMessage(err))?;
            let message_id = ticket.message_id();

            T::OutboundQueueV2::deliver(ticket).map_err(|err| Error::<T>::MessageNotSent(err))?;

            Self::deposit_event(Event::OutboundMessageSent {
                message_id,
                message,
                reward,
                gas,
            });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Handle an inbound LayerZero message by forwarding it to the destination container chain.
        ///
        /// Called by `LayerZeroMessageProcessor` when messages arrive from Ethereum.
        /// Validates the sender is whitelisted and sends an XCM `Transact` to the configured
        /// destination pallet on the container chain.
        ///
        /// Returns `Ok(())` if routed successfully, `Err(MessageProcessorError)` otherwise.
        pub fn handle_inbound_message(
            message: InboundMessage,
        ) -> Result<(), MessageProcessorError> {
            let dest_chain_id: ChainId = message.destination_chain;

            let config = RoutingConfigs::<T>::get(dest_chain_id).ok_or(
                MessageProcessorError::ProcessMessage(Error::<T>::NoRoutingConfig.into()),
            )?;

            let sender = (
                message.lz_source_endpoint,
                message.lz_source_address.clone(),
            );
            if !config.whitelisted_senders.contains(&sender) {
                return Err(MessageProcessorError::ProcessMessage(
                    Error::<T>::NotWhitelistedSender.into(),
                ));
            }

            let container_chain_location = Location::new(0, [Parachain(dest_chain_id)]);

            // Craft a Transact XCM to send the message to the destination chain
            let pallet_index = config.notification_destination.0;
            let call_index = config.notification_destination.1;

            let remote_xcm = Xcm::<()>(vec![
                UnpaidExecution {
                    weight_limit: Unlimited,
                    check_origin: None,
                },
                Transact {
                    origin_kind: OriginKind::Xcm,
                    fallback_max_weight: None,
                    call: (pallet_index, call_index, message.encode()).encode().into(),
                },
            ]);

            send_xcm::<<T as pallet_xcm::Config>::XcmRouter>(
                container_chain_location.clone(),
                remote_xcm.clone(),
            )
            .map_err(MessageProcessorError::SendMessage)?;

            Self::deposit_event(Event::InboundMessageRouted {
                chain_id: dest_chain_id,
                message,
            });

            Ok(())
        }

        /// Convert a location to a message origin hash by reanchoring relative to Ethereum.
        ///
        /// This is used to create a consistent origin identifier that Ethereum can understand.
        /// The process involves:
        /// 1. Reanchoring the location from the relay chain's perspective to Ethereum's perspective
        /// 2. Hashing the reanchored location to produce a unique H256 identifier
        ///
        /// ## Parameters:
        /// - `location`: The XCM location to convert (typically a container chain location)
        ///
        /// ## Returns:
        /// - `Ok(H256)`: The unique origin hash for the location
        /// - `Err(Error<T>)`: Conversion failed
        ///
        /// ## Errors:
        /// - `LocationReanchorFailed`: The location couldn't be reanchored to Ethereum's context
        /// - `LocationToOriginConversionFailed`: The location hash conversion failed
        pub fn location_to_message_origin(location: Location) -> Result<H256, Error<T>> {
            let reanchored_location = Self::reanchor(location)?;
            T::LocationHashOf::convert_location(&reanchored_location)
                .ok_or(Error::<T>::LocationToOriginConversionFailed)
        }

        /// Reanchor a location from the relay chain's perspective to Ethereum's perspective.
        ///
        /// XCM locations are relative, so the same location appears differently depending on
        /// the observer. This function converts a location from "how the relay chain sees it"
        /// to "how Ethereum sees it" by using the universal location system.
        ///
        /// ## Example:
        /// A container chain at `Parachain(2000)` on the relay chain would be reanchored to
        /// include the full path from Ethereum's perspective (e.g., including the relay chain
        /// identifier).
        ///
        /// ## Parameters:
        /// - `location`: The location to reanchor
        ///
        /// ## Returns:
        /// - `Ok(Location)`: The reanchored location from Ethereum's perspective
        /// - `Err(Error<T>)`: Reanchoring failed
        ///
        /// ## Errors:
        /// - `LocationReanchorFailed`: The reanchoring operation failed (e.g., incompatible locations)
        pub fn reanchor(location: Location) -> Result<Location, Error<T>> {
            location
                .reanchored(
                    &T::EthereumLocation::get(),
                    &<T as Config>::UniversalLocation::get(),
                )
                .map_err(|_| Error::<T>::LocationReanchorFailed)
        }
    }
}
