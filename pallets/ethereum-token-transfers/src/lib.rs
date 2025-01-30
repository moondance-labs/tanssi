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

//! EthereumTokenTransfers pallet.
//!
//! This pallet takes care of sending the native Starlight token from Starlight to Ethereum.
//!
//! It does this by sending a MintForeignToken command to Ethereum through a
//! specific channel_id (which is also stored in this pallet).
//!
//! ## Extrinsics:
//!
//! ### set_token_transfer_channel:
//!
//! Only callable by root. Used to specify which channel_id
//! will be used to send the tokens through. It also receives the para_id and
//! agent_id params corresponding to the channel specified.
//!
//! ### transfer_native_token:
//!
//! Used to perform the actual sending of the tokens, it requires to specify an amount and a recipient.
//!
//! Inside it, the message is built using the MintForeignToken command. Once the message is validated,
//! the amount is transferred from the caller to the EthereumSovereignAccount. This allows to prevent
//! double-spending and to track how much of the native token is sent to Ethereum.
//!
//! After that, the message is delivered to Ethereum through the T::OutboundQueue implementation.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;

use {
    frame_support::{
        pallet_prelude::*,
        traits::{
            fungible::{self, Inspect, Mutate},
            tokens::Preservation,
            Get,
        },
    },
    frame_system::pallet_prelude::*,
    snowbridge_core::{
        outbound::{
            Command as SnowbridgeCommand, Message as SnowbridgeMessage, SendError, SendMessage,
        },
        AgentId, ChannelId, ParaId, TokenId,
    },
    sp_core::{H160, H256},
    sp_runtime::{traits::MaybeEquivalence, DispatchResult},
    sp_std::vec,
    tp_bridge::TicketInfo,
    tp_traits::EthereumSystemChannelManager,
    xcm::prelude::*,
};

#[cfg(feature = "runtime-benchmarks")]
use tp_bridge::TokenChannelSetterBenchmarkHelperTrait;

pub use pallet::*;

/// Information of the token-sending channel stored in this pallet.
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, Clone, PartialEq, MaxEncodedLen)]
pub struct ChannelInfo {
    pub channel_id: ChannelId,
    pub para_id: ParaId,
    pub agent_id: AgentId,
}

pub type BalanceOf<T> =
    <<T as pallet::Config>::Currency as Inspect<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    pub use crate::weights::WeightInfo;

    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

    pub type RewardPoints = u32;
    pub type EraIndex = u32;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Currency to handle fees and internal native transfers.
        type Currency: fungible::Inspect<Self::AccountId, Balance: From<u128>>
            + fungible::Mutate<Self::AccountId>;

        /// Validate and send a message to Ethereum.
        type OutboundQueue: SendMessage<Balance = BalanceOf<Self>, Ticket: TicketInfo>;

        /// Handler for EthereumSystem pallet. Commonly used to manage channel creation.
        type EthereumSystemHandler: EthereumSystemChannelManager;

        /// Ethereum sovereign account, where native transfers will go to.
        type EthereumSovereignAccount: Get<Self::AccountId>;

        /// Account in which fees will be minted.
        type FeesAccount: Get<Self::AccountId>;

        /// Token Location from the external chain's point of view.
        type TokenLocationReanchored: Get<Location>;

        /// How to convert from a given Location to a specific TokenId.
        type TokenIdFromLocation: MaybeEquivalence<TokenId, Location>;

        // The weight information of this pallet.
        type WeightInfo: WeightInfo;

        #[cfg(feature = "runtime-benchmarks")]
        type BenchmarkHelper: TokenChannelSetterBenchmarkHelperTrait;
    }

    // Events
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Information for the channel was set properly.
        ChannelInfoSet { channel_info: ChannelInfo },
        /// Some native token was successfully transferred to Ethereum.
        NativeTokenTransferred {
            message_id: H256,
            channel_id: ChannelId,
            source: T::AccountId,
            recipient: H160,
            token_id: H256,
            amount: u128,
            fee: BalanceOf<T>,
        },
    }

    // Errors
    #[pallet::error]
    pub enum Error<T> {
        /// The requested ChannelId is already present in this pallet.
        ChannelIdAlreadyExists,
        /// The channel's information has not been set on this pallet yet.
        ChannelInfoNotSet,
        /// The requested ParaId is already present in this pallet.
        ParaIdAlreadyExists,
        /// The requested AgentId is already present in this pallet.
        AgentIdAlreadyExists,
        /// Conversion from Location to TokenId failed.
        UnknownLocationForToken,
        /// The outbound message is invalid prior to send.
        InvalidMessage(SendError),
        /// The outbound message could not be sent.
        TransferMessageNotSent(SendError),
    }

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    // Storage
    #[pallet::storage]
    #[pallet::getter(fn current_channel_info)]
    pub type CurrentChannelInfo<T: Config> = StorageValue<_, ChannelInfo, OptionQuery>;

    // Calls
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::set_token_transfer_channel())]
        pub fn set_token_transfer_channel(
            origin: OriginFor<T>,
            channel_id: ChannelId,
            agent_id: AgentId,
            para_id: ParaId,
        ) -> DispatchResult {
            ensure_root(origin)?;

            if let Some(channel_info) = CurrentChannelInfo::<T>::get() {
                if channel_info.channel_id == channel_id {
                    return Err(Error::<T>::ChannelIdAlreadyExists.into());
                }

                if channel_info.para_id == para_id {
                    return Err(Error::<T>::ParaIdAlreadyExists.into());
                }

                if channel_info.agent_id == agent_id {
                    return Err(Error::<T>::AgentIdAlreadyExists.into());
                }
            }

            let channel_info = ChannelInfo {
                channel_id,
                para_id,
                agent_id,
            };

            CurrentChannelInfo::<T>::put(channel_info.clone());

            T::EthereumSystemHandler::create_channel(
                channel_info.channel_id,
                channel_info.agent_id,
                channel_info.para_id,
            )?;

            Self::deposit_event(Event::<T>::ChannelInfoSet { channel_info });

            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::transfer_native_token())]
        pub fn transfer_native_token(
            origin: OriginFor<T>,
            amount: u128,
            recipient: H160,
        ) -> DispatchResult {
            let source = ensure_signed(origin)?;

            if let Some(channel_info) = CurrentChannelInfo::<T>::get() {
                let token_location = T::TokenLocationReanchored::get();
                let token_id = T::TokenIdFromLocation::convert_back(&token_location);

                if let Some(token_id) = token_id {
                    let command = SnowbridgeCommand::MintForeignToken {
                        token_id,
                        recipient,
                        amount,
                    };

                    let message = SnowbridgeMessage {
                        id: None,
                        channel_id: channel_info.channel_id,
                        command,
                    };

                    let (ticket, fee) = T::OutboundQueue::validate(&message)
                        .map_err(|err| Error::<T>::InvalidMessage(err))?;

                    // Transfer fees to FeesAccount.
                    T::Currency::transfer(
                        &source,
                        &T::FeesAccount::get(),
                        fee.total(),
                        Preservation::Preserve,
                    )?;

                    // Transfer amount to Ethereum's sovereign account.
                    T::Currency::transfer(
                        &source,
                        &T::EthereumSovereignAccount::get(),
                        amount.into(),
                        Preservation::Preserve,
                    )?;

                    let message_id = ticket.message_id();

                    T::OutboundQueue::deliver(ticket)
                        .map_err(|err| Error::<T>::TransferMessageNotSent(err))?;

                    Self::deposit_event(Event::<T>::NativeTokenTransferred {
                        message_id,
                        channel_id: channel_info.channel_id,
                        source,
                        recipient,
                        token_id,
                        amount,
                        fee: fee.total(),
                    });

                    return Ok(());
                } else {
                    return Err(Error::<T>::UnknownLocationForToken.into());
                }
            } else {
                return Err(Error::<T>::ChannelInfoNotSet.into());
            }
        }
    }
}
