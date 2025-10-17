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

use {super::*, core::marker::PhantomData, snowbridge_outbound_queue_primitives::SendError};

/// Custom Impl
pub struct CustomMessageValidatorV1<T: snowbridge_pallet_outbound_queue::Config>(PhantomData<T>);
impl<T: snowbridge_pallet_outbound_queue::Config> ValidateMessage for MessageValidator<T> {
    type Ticket = Ticket<T>;

    fn validate(message: &Message) -> Result<(Self::Ticket, Fee<u64>), SendError> {
        log::trace!("CustomMessageValidatorV1: {:?}", message);
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
