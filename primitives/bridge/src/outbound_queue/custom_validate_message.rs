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

use frame_support::ensure;
use {crate::*, core::marker::PhantomData, snowbridge_outbound_queue_primitives::SendError};

/// Custom Impl
pub struct TanssiEthMessageValidatorV1<T: snowbridge_pallet_outbound_queue::Config>(PhantomData<T>);
impl<T: snowbridge_pallet_outbound_queue::Config> ValidateMessage
    for TanssiEthMessageValidatorV1<T>
{
    type Ticket = TanssiTicketV1<T>;

    fn validate(message: &TanssiMessage) -> Result<(Self::Ticket, Fee<u64>), SendError> {
        log::trace!("TanssiEthMessageValidatorV1: {:?}", message);
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

        let queued_message: VersionedQueuedTanssiMessage = QueuedTanssiMessage {
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

pub struct TanssiEthMessageValidatorV2<
    T: snowbridge_pallet_outbound_queue_v2::Config,
    OwnOrigin: Get<Location>,
>(PhantomData<(T, OwnOrigin)>);

impl<T: snowbridge_pallet_outbound_queue_v2::Config, OwnOrigin: Get<Location>> ValidateMessage
    for TanssiEthMessageValidatorV2<T, OwnOrigin>
{
    type Ticket = TanssiTicketV2<T>;

    fn validate(message: &TanssiMessage) -> Result<(Self::Ticket, Fee<u64>), SendError> {
        log::trace!("MessageValidatorV2: {:?}", message);
        // The inner payload should not be too large
        let payload = message.command.abi_encode();

        // make it dependent on pallet-v2
        ensure!(
            payload.len() < T::MaxMessagePayloadSize::get() as usize,
            SendError::MessageTooLarge
        );

        // This is only called by system level pallets
        // so we can put the origin to system
        let origin = crate::TanssiAgentIdOf::convert_location(&OwnOrigin::get())
            .ok_or(SendError::InvalidOrigin)?;

        // Generate a unique message id unless one is provided
        let message_id: H256 = message
            .id
            .unwrap_or_else(|| unique((message.channel_id, &message.command)).into());

        let queued_message: VersionedQueuedTanssiMessage = QueuedTanssiMessage {
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

        let ticket = TanssiTicketV2::<T> {
            origin,
            id: message_id,
            fee: 0,
            message: encoded,
        };

        // Fee not used for now
        let fee = Fee {
            local: Default::default(),
            remote: Default::default(),
        };

        Ok((ticket, fee))
    }
}

pub struct VersionedTanssiEthMessageValidator<
    T: snowbridge_pallet_outbound_queue::Config + snowbridge_pallet_outbound_queue_v2::Config,
    OwnOrigin: Get<Location>,
    UseV2: Get<bool>,
>(PhantomData<(T, OwnOrigin, UseV2)>);
impl<
        T: snowbridge_pallet_outbound_queue::Config + snowbridge_pallet_outbound_queue_v2::Config,
        OwnOrigin: Get<Location>,
        UseV2: Get<bool>,
    > ValidateMessage for VersionedTanssiEthMessageValidator<T, OwnOrigin, UseV2>
{
    type Ticket = VersionedTanssiTicket<T>;
    fn validate(message: &TanssiMessage) -> Result<(Self::Ticket, Fee<u64>), SendError> {
        if UseV2::get() {
            TanssiEthMessageValidatorV2::<T, OwnOrigin>::validate(message)
                .map(|(ticket, fee)| (ticket.into(), fee))
        } else {
            TanssiEthMessageValidatorV1::<T>::validate(message)
                .map(|(ticket, fee)| (ticket.into(), fee))
        }
    }
}
