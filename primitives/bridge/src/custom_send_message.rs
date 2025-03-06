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
    GetAggregateMessageOrigin:
        Convert<ChannelId, <T as snowbridge_pallet_outbound_queue::Config>::AggregateMessageOrigin>,
{
    type Ticket = Ticket<T>;

    fn deliver(ticket: Self::Ticket) -> Result<sp_core::H256, SendError> {
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
