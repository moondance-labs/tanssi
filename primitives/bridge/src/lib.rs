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

use {
    core::marker::PhantomData,
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
    sp_core::H256,
    sp_runtime::{app_crypto::sp_core, traits::Convert, RuntimeDebug, Perbill},
    sp_std::vec::Vec,
};
pub use {
    custom_do_process_message::{ConstantGasMeter, CustomProcessSnowbridgeMessage},
    custom_send_message::CustomSendMessage,
};

// Separate import as rustfmt wrongly change it to `sp_std::vec::self`, which is the module instead
// of the macro.
use sp_std::vec;

/// A command which is executable by the Gateway contract on Ethereum
#[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(PartialEq))]
pub enum Command {
    // TODO: add real commands here
    Test(Vec<u8>),
    ReportSlashes {
        era_index: u32,
        // merkle root of vec![(validatorId, slash)]
        slashes_merkle_root: H256
    }
}

impl Command {
    /// Compute the enum variant index
    pub fn index(&self) -> u8 {
        match self {
            // Starting from 32 to keep compatibility with Snowbridge Command enum
            Command::Test { .. } => 32,
            Command::ReportSlashes { .. } => 33,
        }
    }

    /// ABI-encode the Command.
    pub fn abi_encode(&self) -> Vec<u8> {
        match self {
            Command::Test(payload) => {
                ethabi::encode(&[Token::Tuple(vec![Token::Bytes(payload.clone())])])
            },
            Command::ReportSlashes { era_index, slashes_merkle_root } => {
                let era_index_token = Token::Uint(U256::from(*era_index));
                let slashes_mr_token = Token::FixedBytes(slashes_merkle_root.0.to_vec());
                ethabi::encode(&[Token::Tuple(vec![era_index_token, slashes_mr_token])])
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

mod custom_do_process_message {
    use {
        super::*,
        frame_support::{
            ensure,
            traits::{Defensive, ProcessMessage, ProcessMessageError},
            weights::WeightMeter,
        },
        snowbridge_pallet_outbound_queue::{
            CommittedMessage, MessageLeaves, Messages, Nonce, ProcessMessageOriginOf, WeightInfo,
        },
        sp_runtime::traits::Hash,
        sp_std::boxed::Box,
    };

    /// Alternative to [snowbridge_pallet_outbound_queue::Pallet::process_message] using a different
    /// [Command] enum.
    pub struct CustomProcessSnowbridgeMessage<T>(PhantomData<T>);

    impl<T> CustomProcessSnowbridgeMessage<T>
    where
        T: snowbridge_pallet_outbound_queue::Config,
    {
        /// Process a message delivered by the MessageQueue pallet
        pub(crate) fn do_process_message(
            _: ProcessMessageOriginOf<T>,
            mut message: &[u8],
        ) -> Result<bool, ProcessMessageError> {
            use ProcessMessageError::*;

            // Yield if the maximum number of messages has been processed this block.
            // This ensures that the weight of `on_finalize` has a known maximum bound.
            ensure!(
                MessageLeaves::<T>::decode_len().unwrap_or(0)
                    < T::MaxMessagesPerBlock::get() as usize,
                Yield
            );

            // Decode bytes into versioned message
            let versioned_queued_message: VersionedQueuedMessage =
                VersionedQueuedMessage::decode(&mut message).map_err(|_| Corrupt)?;

            // Convert versioned message into latest supported message version
            let queued_message: QueuedMessage = versioned_queued_message
                .try_into()
                .map_err(|_| Unsupported)?;

            // Obtain next nonce
            let nonce = <Nonce<T>>::try_mutate(
                queued_message.channel_id,
                |nonce| -> Result<u64, ProcessMessageError> {
                    *nonce = nonce.checked_add(1).ok_or(Unsupported)?;
                    Ok(*nonce)
                },
            )?;

            let pricing_params = T::PricingParameters::get();
            let command = queued_message.command.index();
            let params = queued_message.command.abi_encode();
            let max_dispatch_gas =
                ConstantGasMeter::maximum_dispatch_gas_used_at_most(&queued_message.command);
            let reward = pricing_params.rewards.remote;

            // Construct the final committed message
            let message = CommittedMessage {
                channel_id: queued_message.channel_id,
                nonce,
                command,
                params,
                max_dispatch_gas,
                max_fee_per_gas: pricing_params
                    .fee_per_gas
                    .try_into()
                    .defensive_unwrap_or(u128::MAX),
                reward: reward.try_into().defensive_unwrap_or(u128::MAX),
                id: queued_message.id,
            };

            // ABI-encode and hash the prepared message
            let message_abi_encoded = ethabi::encode(&[message.clone().into()]);
            let message_abi_encoded_hash =
                <T as snowbridge_pallet_outbound_queue::Config>::Hashing::hash(
                    &message_abi_encoded,
                );

            Messages::<T>::append(Box::new(message));
            MessageLeaves::<T>::append(message_abi_encoded_hash);

            snowbridge_pallet_outbound_queue::Pallet::<T>::deposit_event(
                snowbridge_pallet_outbound_queue::Event::MessageAccepted {
                    id: queued_message.id,
                    nonce,
                },
            );

            Ok(true)
        }
    }

    impl<T> ProcessMessage for CustomProcessSnowbridgeMessage<T>
    where
        T: snowbridge_pallet_outbound_queue::Config,
    {
        type Origin = T::AggregateMessageOrigin;

        fn process_message(
            message: &[u8],
            origin: Self::Origin,
            meter: &mut WeightMeter,
            _id: &mut [u8; 32],
        ) -> Result<bool, ProcessMessageError> {
            // TODO: this weight is from the pallet, should be very similar to the weight of
            // Self::do_process_message, but ideally we should benchmark this separately
            let weight = T::WeightInfo::do_process_message();
            if meter.try_consume(weight).is_err() {
                return Err(ProcessMessageError::Overweight(weight));
            }

            Self::do_process_message(origin.clone(), message)
        }
    }

    /// A meter that assigns a constant amount of gas for the execution of a command
    ///
    /// The gas figures are extracted from this report:
    /// > forge test --match-path test/Gateway.t.sol --gas-report
    ///
    /// A healthy buffer is added on top of these figures to account for:
    /// * The EIP-150 63/64 rule
    /// * Future EVM upgrades that may increase gas cost
    pub struct ConstantGasMeter;

    impl ConstantGasMeter {
        // The base transaction cost, which includes:
        // 21_000 transaction cost, roughly worst case 64_000 for calldata, and 100_000
        // for message verification
        pub const MAXIMUM_BASE_GAS: u64 = 185_000;

        fn maximum_dispatch_gas_used_at_most(command: &Command) -> u64 {
            match command {
                Command::Test { .. } => 60_000,
                Command::ReportSlashes { .. } => 60_000,
            }
        }
    }
}

mod custom_send_message {
    use {
        super::*,
        ethabi::H256,
        frame_support::traits::EnqueueMessage,
        snowbridge_core::{outbound::SendError, PRIMARY_GOVERNANCE_CHANNEL},
        sp_std::marker::PhantomData,
    };

    /// Alternative to [snowbridge_pallet_outbound_queue::Pallet::deliver] using a different
    /// origin.
    pub struct CustomSendMessage<T, GetAggregateMessageOrigin>(
        PhantomData<(T, GetAggregateMessageOrigin)>,
    );

    impl<T, GetAggregateMessageOrigin> DeliverMessage
        for CustomSendMessage<T, GetAggregateMessageOrigin>
    where
        T: snowbridge_pallet_outbound_queue::Config,
        GetAggregateMessageOrigin: Convert<
            ChannelId,
            <T as snowbridge_pallet_outbound_queue::Config>::AggregateMessageOrigin,
        >,
    {
        type Ticket = Ticket<T>;

        fn deliver(ticket: Self::Ticket) -> Result<H256, SendError> {
            let origin = GetAggregateMessageOrigin::convert(ticket.channel_id);

            if ticket.channel_id != PRIMARY_GOVERNANCE_CHANNEL {
                ensure!(
                    !<snowbridge_pallet_outbound_queue::Pallet<T>>::operating_mode().is_halted(),
                    SendError::Halted
                );
            }

            let message = ticket.message.as_bounded_slice();

            <T as snowbridge_pallet_outbound_queue::Config>::MessageQueue::enqueue_message(
                message, origin,
            );
            snowbridge_pallet_outbound_queue::Pallet::<T>::deposit_event(
                snowbridge_pallet_outbound_queue::Event::MessageQueued {
                    id: ticket.message_id,
                },
            );

            Ok(ticket.message_id)
        }
    }
}
