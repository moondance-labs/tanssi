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
//! via the relay chain.
//!
//! ## Message Flow
//!
//! **Inbound**: Ethereum → Relay (this pallet) → Container Chain
//! **Outbound**: Container Chain → Relay (this pallet) → Ethereum
//!
//! ## Usage
//!
//! Container chains configure routing via `update_routing_config`, specifying:
//! - Whitelisted senders: `(LayerZeroEndpoint, LayerZeroAddress)` tuples
//! - Notification destination: `(pallet_index, call_index)` to receive messages
//!
//! See `pallet-lz-receiver-example` for a reference implementation.

#![cfg_attr(not(feature = "std"), no_std)]

mod types;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

extern crate alloc;

use crate::types::ChainId;
pub use pallet::*;
use xcm::latest::Location;
use xcm::prelude::{Parachain, Unlimited};
use {
    parity_scale_codec::Encode,
    sp_runtime::traits::Get,
    tp_bridge::layerzero_message::{
        InboundMessage, LayerZeroAddress, LayerZeroEndpoint, LayerZeroOutboundPayload,
        OutboundMessage,
    },
};

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
    use xcm::latest::{send_xcm, Location, Xcm};
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
    }

    #[pallet::pallet]
    #[pallet::without_storage_info]
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
        /// Outbound message queued for Ethereum/LayerZero
        OutboundMessageQueued { message: OutboundMessage },
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Update routing configuration for a container chain.
        ///
        /// Must be called via XCM from the container chain itself.
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
        /// Called via XCM from a container chain to send a message to an external chain.
        /// The `source_chain` is automatically set from the calling container chain's origin.
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn send_message_to_ethereum(
            origin: OriginFor<T>,
            lz_destination_address: LayerZeroAddress,
            lz_destination_endpoint: LayerZeroEndpoint,
            payload: LayerZeroOutboundPayload,
        ) -> DispatchResult {
            let origin_location = T::ContainerChainOrigin::ensure_origin(origin)?;
            let source_chain = extract_container_chain_id(&origin_location)
                .ok_or(Error::<T>::LocationIsNotAContainerChain)?;

            let message = OutboundMessage {
                source_chain,
                lz_destination_address,
                lz_destination_endpoint,
                payload,
            };

            // TODO: Queue message for Ethereum outbound queue
            // This will integrate with snowbridge outbound queue

            Self::deposit_event(Event::OutboundMessageQueued { message });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Handle an inbound LayerZero message by forwarding it to its destination container chain via XCM.
        ///
        /// Called by `LayerZeroInboundMessageProcessorV2` when messages arrive from Ethereum.
        /// Validates the sender is whitelisted and sends an XCM Transact to the destination.
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
    }
}
