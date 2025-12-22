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

//! ExternalValidators pallet.
//!
//! A pallet to manage external validators for a solochain.
//!
//! ## Terminology
//!
//! - WhitelistedValidators: Fixed validators set by root/governance. Have priority over the external validators.
//!      Are not rewarded.
//! - ExternalValidators: Validators set using storage proofs from another blockchain. Can be disabled by setting
//!     `SkipExternalValidators` to true.
//!
//! Validators only change once per era. By default the era changes after a fixed number of sessions, but new eras
//! can be forced or disabled using a root extrinsic.
//!
//! The structure of this pallet and the concept of eras is inspired by `pallet_staking` from Polkadot.

#![cfg_attr(not(feature = "std"), no_std)]

mod types;

extern crate alloc;

use crate::types::ChainId;
pub use pallet::*;
use xcm::latest::Location;
use xcm::prelude::Parachain;
use {parity_scale_codec::Encode, sp_runtime::traits::Get, tp_bridge::layerzero_message::Message};

fn extract_container_chain_id(location: &Location) -> Option<ChainId> {
    match location.unpack() {
        (0, [Parachain(id)]) => Some(id.clone()),
        _ => None,
    }
}

#[frame_support::pallet]
pub mod pallet {
    use crate::types::{ChainId, MessageForwardingConfig};
    use alloc::vec;
    use snowbridge_inbound_queue_primitives::v2::MessageProcessorError;
    use tp_bridge::ConvertLocation;
    use xcm::latest::{send_xcm, Location, Xcm};
    use xcm::prelude::{OriginKind, Transact};
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
        /// Origin locations allowed to update message forwarding configurations
        type ContainerChainOrigin: EnsureOrigin<
            <Self as frame_system::Config>::RuntimeOrigin,
            Success = Location,
        >;

        /// Used to obtain container chain sovereign account.
        type ConvertLocation: ConvertLocation<Self::AccountId>;
    }

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// Configuration per container chain for message forwarding
    #[pallet::storage]
    pub type MessageForwardingConfigs<T: Config> =
        StorageMap<_, Twox64Concat, ChainId, MessageForwardingConfig<T>, OptionQuery>;

    #[pallet::error]
    pub enum Error<T> {
        /// The provided origin location is not a container chain
        LocationIsNotAContainerChain,
        /// No forwarding configuration found for the destination chain
        NoForwardingConfig,
        /// The sender (address+endpoint) is not whitelisted to forward messages to the destination chain
        NotWhitelistedSender,
        /// Setting the same configuration that already exists
        SameConfigAlreadyExists,
        /// Failed to convert container chain location to account ID
        ChainAccountConversionFailed,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(crate) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Message forwarding configuration updated for a container chain
        MessageForwardingConfigUpdated {
            chain_id: ChainId,
            new_config: MessageForwardingConfig<T>,
            old_config: Option<MessageForwardingConfig<T>>,
        },
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1,1))]
        pub fn update_message_forwarding_config(
            origin: OriginFor<T>,
            new_config: MessageForwardingConfig<T>,
        ) -> DispatchResult {
            let origin_location = T::ContainerChainOrigin::ensure_origin(origin)?;
            let chain_id = extract_container_chain_id(&origin_location.into())
                .ok_or(Error::<T>::LocationIsNotAContainerChain)?;

            let old_config = MessageForwardingConfigs::<T>::get(chain_id);
            ensure!(
                old_config != Some(new_config.clone()),
                Error::<T>::SameConfigAlreadyExists
            );

            MessageForwardingConfigs::<T>::insert(chain_id, new_config.clone());

            Self::deposit_event(Event::MessageForwardingConfigUpdated {
                chain_id,
                new_config,
                old_config,
            });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn forward_message_to_chain(message: Message) -> Result<(), MessageProcessorError> {
            let dest_chain_id: ChainId = message.destination_chain;

            let config = MessageForwardingConfigs::<T>::get(dest_chain_id).ok_or(
                MessageProcessorError::ProcessMessage(Error::<T>::NoForwardingConfig.into()),
            )?;

            config
                .whitelisted_senders
                .iter()
                .any(|(lz_endpoint, lz_address)| {
                    &message.lz_source_endpoint == lz_endpoint
                        && &message.lz_source_address == lz_address
                })
                .then_some(())
                .ok_or(MessageProcessorError::ProcessMessage(
                    Error::<T>::NotWhitelistedSender.into(),
                ))?;

            let container_chain_location = Location::new(0, [Parachain(dest_chain_id)]);

            // Craft a Transact XCM to send the message to the destination chain
            let pallet_index = config.notification_destination.0;
            let call_index = config.notification_destination.1;

            let remote_xcm = Xcm::<()>(vec![Transact {
                origin_kind: OriginKind::Xcm,
                fallback_max_weight: None,
                call: (pallet_index, call_index, message).encode().into(),
            }]);

            send_xcm::<<T as pallet_xcm::Config>::XcmRouter>(
                container_chain_location.clone(),
                remote_xcm.clone(),
            )
            .map_err(|err| MessageProcessorError::SendMessage(err))?;

            Ok(())
        }
    }
}
