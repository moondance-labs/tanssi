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
            MessageLeaves::<T>::decode_len().unwrap_or(0) < T::MaxMessagesPerBlock::get() as usize,
            Yield
        );

        // Decode bytes into versioned message
        let versioned_queued_message: VersionedQueuedMessage =
            VersionedQueuedMessage::decode(&mut message).map_err(|_| Corrupt)?;

        log::trace!(
            "CustomProcessSnowbridgeMessage: {:?}",
            versioned_queued_message
        );

        // Convert versioned message into latest supported message version
        let queued_message: QueuedMessage = versioned_queued_message.into();

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
            <T as snowbridge_pallet_outbound_queue::Config>::Hashing::hash(&message_abi_encoded);

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
            // TODO: revisit gas cost
            Command::ReportRewards { .. } => 1_000_000,
            Command::ReportSlashes { .. } => 1_000_000,
        }
    }
}
