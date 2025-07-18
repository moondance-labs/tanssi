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

#[cfg(feature = "runtime-benchmarks")]
mod benchmarks;
#[cfg(test)]
mod tests;

pub mod generic_token_message_processor;
pub mod snowbridge_outbound_token_transfer;
pub mod symbiotic_message_processor;

use {
    core::marker::PhantomData,
    cumulus_primitives_core::{
        relay_chain::{AccountId, Balance},
        AccountKey20, Assets, Ethereum, GlobalConsensus, Location, SendResult, SendXcm, Xcm,
        XcmHash,
    },
    ethabi::{Token, U256},
    frame_support::{
        ensure,
        pallet_prelude::{Decode, Encode, Get},
        traits::Contains,
    },
    frame_system::unique,
    parity_scale_codec::{DecodeWithMemTracking, MaxEncodedLen},
    scale_info::TypeInfo,
    snowbridge_core::{AgentId, Channel, ChannelId, ParaId},
    snowbridge_inbound_queue_primitives::v1::{
        ConvertMessage, ConvertMessageError, VersionedXcmMessage,
    },
    snowbridge_outbound_queue_primitives::{v1::Fee, SendError},
    snowbridge_pallet_outbound_queue::send_message_impl::Ticket,
    sp_core::{blake2_256, hashing, H256},
    sp_runtime::{app_crypto::sp_core, traits::Convert, RuntimeDebug},
    sp_std::vec::Vec,
};

// Separate import as rustfmt wrongly change it to `sp_std::vec::self`, which is the module instead
// of the macro.
use sp_std::vec;

pub use {
    custom_do_process_message::{ConstantGasMeter, CustomProcessSnowbridgeMessage},
    custom_send_message::CustomSendMessage,
    xcm_executor::traits::ConvertLocation,
};

#[cfg(feature = "runtime-benchmarks")]
pub use benchmarks::*;

mod custom_do_process_message;
mod custom_send_message;

#[derive(Clone, Encode, Decode, DecodeWithMemTracking, RuntimeDebug, TypeInfo, PartialEq)]
pub struct SlashData {
    pub encoded_validator_id: Vec<u8>,
    pub slash_fraction: u32,
    pub external_idx: u64,
}

/// A command which is executable by the Gateway contract on Ethereum
#[derive(Clone, Encode, Decode, DecodeWithMemTracking, RuntimeDebug, TypeInfo, PartialEq)]
pub enum Command {
    // TODO: add real commands here
    Test(Vec<u8>),
    ReportRewards {
        // external identifier for validators
        external_idx: u64,
        // index of the era we are sending info of
        era_index: u32,
        // total_points for the era
        total_points: u128,
        // new tokens inflated during the era
        tokens_inflated: u128,
        // merkle root of vec![(validatorId, rewardPoints)]
        rewards_merkle_root: H256,
        // the token id in which we need to mint
        token_id: H256,
    },
    ReportSlashes {
        // index of the era we are sending info of
        era_index: u32,
        // vec of `SlashData`
        slashes: Vec<SlashData>,
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
                external_idx,
                era_index,
                total_points,
                tokens_inflated,
                rewards_merkle_root,
                token_id,
            } => {
                let external_idx_token = Token::Uint(U256::from(*external_idx));
                let era_index_token = Token::Uint(U256::from(*era_index));
                let total_points_token = Token::Uint(U256::from(*total_points));
                let tokens_inflated_token = Token::Uint(U256::from(*tokens_inflated));
                let rewards_mr_token = Token::FixedBytes(rewards_merkle_root.0.to_vec());
                let token_id_token = Token::FixedBytes(token_id.0.to_vec());

                ethabi::encode(&[Token::Tuple(vec![
                    external_idx_token,
                    era_index_token,
                    total_points_token,
                    tokens_inflated_token,
                    rewards_mr_token,
                    token_id_token,
                ])])
            }
            Command::ReportSlashes { era_index, slashes } => {
                let era_index_token = Token::Uint(U256::from(*era_index));
                let mut slashes_tokens_vec: Vec<Token> = vec![];

                for slash in slashes.iter() {
                    let account_token = Token::FixedBytes(slash.encoded_validator_id.clone());
                    let slash_fraction_token = Token::Uint(U256::from(slash.slash_fraction));
                    let external_idx = Token::Uint(U256::from(slash.external_idx));
                    let tuple_token =
                        Token::Tuple(vec![account_token, slash_fraction_token, external_idx]);

                    slashes_tokens_vec.push(tuple_token);
                }

                let slashes_tokens_array = Token::Array(slashes_tokens_vec);
                ethabi::encode(&[Token::Tuple(vec![era_index_token, slashes_tokens_array])])
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

pub trait TicketInfo {
    fn message_id(&self) -> H256;
}

impl TicketInfo for () {
    fn message_id(&self) -> H256 {
        H256::zero()
    }
}

#[cfg(not(feature = "runtime-benchmarks"))]
impl<T: snowbridge_pallet_outbound_queue::Config> TicketInfo for Ticket<T> {
    fn message_id(&self) -> H256 {
        self.message_id
    }
}

// Benchmarks check message_id so it must be deterministic.
#[cfg(feature = "runtime-benchmarks")]
impl<T: snowbridge_pallet_outbound_queue::Config> TicketInfo for Ticket<T> {
    fn message_id(&self) -> H256 {
        H256::default()
    }
}

pub struct MessageValidator<T: snowbridge_pallet_outbound_queue::Config>(PhantomData<T>);

pub trait ValidateMessage {
    type Ticket: TicketInfo;

    fn validate(message: &Message) -> Result<(Self::Ticket, Fee<u64>), SendError>;
}

impl<T: snowbridge_pallet_outbound_queue::Config> ValidateMessage for MessageValidator<T> {
    type Ticket = Ticket<T>;

    fn validate(message: &Message) -> Result<(Self::Ticket, Fee<u64>), SendError> {
        log::trace!("MessageValidator: {:?}", message);
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

// This is a variation of the converter found here:
// https://github.com/paritytech/polkadot-sdk/blob/711e6ff33373bc08b026446ce19b73920bfe068c/bridges/snowbridge/primitives/router/src/inbound/mod.rs#L467
//
// Upstream converter only works for parachains (parents 2) while we to use it in tanssi solo-chain
// (parents 1).
pub struct EthereumLocationsConverterFor<AccountId>(PhantomData<AccountId>);
impl<AccountId> ConvertLocation<AccountId> for EthereumLocationsConverterFor<AccountId>
where
    AccountId: From<[u8; 32]> + Clone,
{
    fn convert_location(location: &Location) -> Option<AccountId> {
        match location.unpack() {
            (1, [GlobalConsensus(Ethereum { chain_id })]) => {
                Some(Self::from_chain_id(chain_id).into())
            }
            (1, [GlobalConsensus(Ethereum { chain_id }), AccountKey20 { network: _, key }]) => {
                Some(Self::from_chain_id_with_key(chain_id, *key).into())
            }
            _ => None,
        }
    }
}

impl<AccountId> EthereumLocationsConverterFor<AccountId> {
    pub fn from_chain_id(chain_id: &u64) -> [u8; 32] {
        (b"ethereum-chain", chain_id).using_encoded(blake2_256)
    }
    pub fn from_chain_id_with_key(chain_id: &u64, key: [u8; 20]) -> [u8; 32] {
        (b"ethereum-chain", chain_id, key).using_encoded(blake2_256)
    }
}

/// Information of a recently created channel.
#[derive(
    Encode, Decode, DecodeWithMemTracking, RuntimeDebug, TypeInfo, Clone, PartialEq, MaxEncodedLen,
)]
pub struct ChannelInfo {
    pub channel_id: ChannelId,
    pub para_id: ParaId,
    pub agent_id: AgentId,
}

/// Trait to manage channel creation inside EthereumSystem pallet.
pub trait EthereumSystemChannelManager {
    fn create_channel(channel_id: ChannelId, agent_id: AgentId, para_id: ParaId) -> ChannelInfo;
}

/// Implementation struct for EthereumSystemChannelManager trait.
pub struct EthereumSystemHandler<Runtime>(PhantomData<Runtime>);
impl<Runtime> EthereumSystemChannelManager for EthereumSystemHandler<Runtime>
where
    Runtime: snowbridge_pallet_system::Config,
{
    fn create_channel(channel_id: ChannelId, agent_id: AgentId, para_id: ParaId) -> ChannelInfo {
        if let Some(channel) = snowbridge_pallet_system::Channels::<Runtime>::get(channel_id) {
            ChannelInfo {
                channel_id,
                para_id: channel.para_id,
                agent_id: channel.agent_id,
            }
        } else {
            if !snowbridge_pallet_system::Agents::<Runtime>::contains_key(agent_id) {
                snowbridge_pallet_system::Agents::<Runtime>::insert(agent_id, ());
            }

            let channel = Channel { agent_id, para_id };
            snowbridge_pallet_system::Channels::<Runtime>::insert(channel_id, channel);

            ChannelInfo {
                channel_id,
                para_id,
                agent_id,
            }
        }
    }
}

/// Helper struct to set up token and channel characteristics needed for EthereumTokenTransfers
/// pallet benchmarks.
#[cfg(feature = "runtime-benchmarks")]
pub struct EthereumTokenTransfersBenchHelper<Runtime>(PhantomData<Runtime>);

#[cfg(feature = "runtime-benchmarks")]
impl<Runtime> crate::TokenChannelSetterBenchmarkHelperTrait
    for EthereumTokenTransfersBenchHelper<Runtime>
where
    Runtime: snowbridge_pallet_system::Config,
{
    fn set_up_token(location: Location, token_id: snowbridge_core::TokenId) {
        snowbridge_pallet_system::ForeignToNativeId::<Runtime>::insert(token_id, &location);
        snowbridge_pallet_system::NativeToForeignId::<Runtime>::insert(&location, token_id);
    }

    fn set_up_channel(channel_id: ChannelId, para_id: ParaId, agent_id: AgentId) {
        let channel = Channel { agent_id, para_id };
        snowbridge_pallet_system::Agents::<Runtime>::insert(agent_id, ());
        snowbridge_pallet_system::Channels::<Runtime>::insert(channel_id, channel);
    }
}
