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

//! Crate containing various traits used by moondance crates allowing to connect pallet
//! with each other or with mocks.

#![cfg_attr(not(feature = "std"), no_std)]

pub mod symbiotic_message_processor;

use {
    core::marker::PhantomData,
    cumulus_primitives_core::{
        relay_chain::{AccountId, Balance},
        Assets, Location, SendResult, SendXcm, Xcm, XcmHash,
    },
    ethabi::{Token, U256},
    frame_support::{
        ensure,
        pallet_prelude::{Decode, Encode, Get},
        traits::Contains,
    },
    frame_system::unique,
    scale_info::TypeInfo,
    snowbridge_core::{
        outbound::{Fee, SendError},
        ChannelId,
    },
    snowbridge_pallet_outbound_queue::send_message_impl::Ticket,
    snowbridge_router_primitives::inbound::{
        ConvertMessage, ConvertMessageError, VersionedXcmMessage,
    },
    sp_core::hashing,
    sp_core::H256,
    sp_runtime::{app_crypto::sp_core, traits::Convert, RuntimeDebug},
    sp_std::vec::Vec,
};

// Separate import as rustfmt wrongly change it to `sp_std::vec::self`, which is the module instead
// of the macro.
use sp_std::vec;

pub use {
    custom_do_process_message::{ConstantGasMeter, CustomProcessSnowbridgeMessage},
    custom_send_message::CustomSendMessage,
};

mod custom_do_process_message;
mod custom_send_message;

/// A command which is executable by the Gateway contract on Ethereum
#[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(PartialEq))]
pub enum Command {
    // TODO: add real commands here
    Test(Vec<u8>),
    ReportRewards {
        // block timestamp
        timestamp: u64,
        // index of the era we are sending info of
        era_index: u32,
        // total_points for the era
        total_points: u128,
        // new tokens inflated during the era
        tokens_inflated: u128,
        // merkle root of vec![(validatorId, rewardPoints)]
        rewards_merkle_root: H256,
    },
    ReportSlashes {
        // index of the era we are sending info of
        era_index: u32,
        // vec of tuples: (validatorId, slash_fraction)
        slashes: Vec<(Vec<u8>, u32)>,
    },
}

impl Command {
    /// Compute the enum variant index
    pub fn index(&self) -> u8 {
        match self {
            // Starting from 32 to keep compatibility with Snowbridge Command enum
            Command::Test { .. } => 32,
            Command::ReportRewards { .. } => 33,
            Command::ReportSlashes { .. } => 34,
        }
    }

    /// ABI-encode the Command.
    pub fn abi_encode(&self) -> Vec<u8> {
        match self {
            Command::Test(payload) => {
                ethabi::encode(&[Token::Tuple(vec![Token::Bytes(payload.clone())])])
            }
            Command::ReportRewards {
                timestamp,
                era_index,
                total_points,
                tokens_inflated,
                rewards_merkle_root,
            } => {
                let timestamp_token = Token::Uint(U256::from(*timestamp));
                let era_index_token = Token::Uint(U256::from(*era_index));
                let total_points_token = Token::Uint(U256::from(*total_points));
                let tokens_inflated_token = Token::Uint(U256::from(*tokens_inflated));
                let rewards_mr_token = Token::FixedBytes(rewards_merkle_root.0.to_vec());
                ethabi::encode(&[Token::Tuple(vec![
                    timestamp_token,
                    era_index_token,
                    total_points_token,
                    tokens_inflated_token,
                    rewards_mr_token,
                ])])
            }
            Command::ReportSlashes { era_index, slashes } => {
                let era_index_token = Token::Uint(U256::from(*era_index));
                let mut slashes_tokens_vec: Vec<Token> = vec![];

                for slash in slashes.into_iter() {
                    // TODO: we could probably do some conversion here to ensure the account
                    // has 32 bytes.
                    let account_token = Token::Bytes(slash.0.clone());
                    let slash_fraction_token = Token::Uint(U256::from(slash.1));
                    let tuple_token = Token::Tuple(vec![account_token, slash_fraction_token]);

                    slashes_tokens_vec.push(tuple_token);
                }

                let slashes_tokens_tuple = Token::Tuple(slashes_tokens_vec);
                ethabi::encode(&[Token::Tuple(vec![era_index_token, slashes_tokens_tuple])])
            }
        }
    }
}

// A message which can be accepted by implementations of `/[`SendMessage`\]`
#[derive(Encode, Decode, TypeInfo, Clone, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(PartialEq))]
pub struct Message {
    /// ID for this message. One will be automatically generated if not provided.
    ///
    /// When this message is created from an XCM message, the ID should be extracted
    /// from the `SetTopic` instruction.
    ///
    /// The ID plays no role in bridge consensus, and is purely meant for message tracing.
    pub id: Option<H256>,
    /// The message channel ID
    pub channel_id: ChannelId,
    /// The stable ID for a receiving gateway contract
    pub command: Command,
}

pub struct MessageValidator<T: snowbridge_pallet_outbound_queue::Config>(PhantomData<T>);

pub trait ValidateMessage {
    type Ticket;

    fn validate(message: &Message) -> Result<(Self::Ticket, Fee<u64>), SendError>;
}

impl<T: snowbridge_pallet_outbound_queue::Config> ValidateMessage for MessageValidator<T> {
    type Ticket = Ticket<T>;

    fn validate(message: &Message) -> Result<(Self::Ticket, Fee<u64>), SendError> {
        // The inner payload should not be too large
        let payload = message.command.abi_encode();
        ensure!(
            payload.len() < T::MaxMessagePayloadSize::get() as usize,
            SendError::MessageTooLarge
        );

        // Ensure there is a registered channel we can transmit this message on
        ensure!(
            T::Channels::contains(&message.channel_id),
            SendError::InvalidChannel
        );

        // Generate a unique message id unless one is provided
        let message_id: H256 = message
            .id
            .unwrap_or_else(|| unique((message.channel_id, &message.command)).into());

        // Fee not used
        /*
        let gas_used_at_most = T::GasMeter::maximum_gas_used_at_most(&message.command);
        let fee = Self::calculate_fee(gas_used_at_most, T::PricingParameters::get());
         */

        let queued_message: VersionedQueuedMessage = QueuedMessage {
            id: message_id,
            channel_id: message.channel_id,
            command: message.command.clone(),
        }
        .into();
        // The whole message should not be too large
        let encoded = queued_message
            .encode()
            .try_into()
            .map_err(|_| SendError::MessageTooLarge)?;

        let ticket = Ticket {
            message_id,
            channel_id: message.channel_id,
            message: encoded,
        };
        let fee = Fee {
            local: Default::default(),
            remote: Default::default(),
        };

        Ok((ticket, fee))
    }
}

impl ValidateMessage for () {
    type Ticket = ();

    fn validate(_message: &Message) -> Result<(Self::Ticket, Fee<u64>), SendError> {
        Ok((
            (),
            Fee {
                local: 1,
                remote: 1,
            },
        ))
    }
}

/// Message which is awaiting processing in the MessageQueue pallet
#[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(PartialEq))]
pub struct QueuedMessage {
    /// Message ID
    pub id: H256,
    /// Channel ID
    pub channel_id: ChannelId,
    /// Command to execute in the Gateway contract
    pub command: Command,
}

/// Enqueued outbound messages need to be versioned to prevent data corruption
/// or loss after forkless runtime upgrades
#[derive(Encode, Decode, TypeInfo, Clone, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(PartialEq))]
pub enum VersionedQueuedMessage {
    V1(QueuedMessage),
}

impl From<QueuedMessage> for VersionedQueuedMessage {
    fn from(x: QueuedMessage) -> Self {
        VersionedQueuedMessage::V1(x)
    }
}

impl From<VersionedQueuedMessage> for QueuedMessage {
    fn from(x: VersionedQueuedMessage) -> Self {
        match x {
            VersionedQueuedMessage::V1(x) => x,
        }
    }
}

pub trait DeliverMessage {
    type Ticket;

    fn deliver(ticket: Self::Ticket) -> Result<H256, SendError>;
}

/// Dummy router for xcm messages coming from ethereum
pub struct DoNothingRouter;
impl SendXcm for DoNothingRouter {
    type Ticket = Xcm<()>;

    fn validate(
        _dest: &mut Option<Location>,
        xcm: &mut Option<Xcm<()>>,
    ) -> SendResult<Self::Ticket> {
        Ok((xcm.clone().unwrap(), Assets::new()))
    }
    fn deliver(xcm: Xcm<()>) -> Result<XcmHash, cumulus_primitives_core::SendError> {
        let hash = xcm.using_encoded(hashing::blake2_256);
        Ok(hash)
    }
}

/// Dummy message converter to convert message to Xcm
pub struct DoNothingConvertMessage;

impl ConvertMessage for DoNothingConvertMessage {
    type Balance = Balance;
    type AccountId = AccountId;

    fn convert(
        _: H256,
        _message: VersionedXcmMessage,
    ) -> Result<(Xcm<()>, Self::Balance), ConvertMessageError> {
        Err(ConvertMessageError::UnsupportedVersion)
    }
}
