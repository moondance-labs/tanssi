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
    super::*, core::marker::PhantomData, frame_support::traits::EnqueueMessage,
    snowbridge_core::PRIMARY_GOVERNANCE_CHANNEL,
    snowbridge_outbound_queue_primitives::v2::Message as MessageV2,
    snowbridge_outbound_queue_primitives::SendError, sp_runtime::BoundedVec,
};

/// Alternative to [snowbridge_pallet_outbound_queue::Pallet::deliver] using a different
/// origin.
pub struct CustomSendMessageV2<T, GetAggregateMessageOrigin>(
    PhantomData<(T, GetAggregateMessageOrigin)>,
);

impl<T, GetAggregateMessageOrigin> DeliverMessage
    for CustomSendMessageV2<T, GetAggregateMessageOrigin>
where
    T: snowbridge_pallet_outbound_queue_v2::Config,
    GetAggregateMessageOrigin: Convert<
        ChannelId,
        <T as snowbridge_pallet_outbound_queue_v2::Config>::AggregateMessageOrigin,
    >,
{
    type Ticket = TanssiMessageV2<T>;

    fn deliver(ticket: Self::Ticket) -> Result<sp_core::H256, SendError> {
        let origin = GetAggregateMessageOrigin::convert(ticket.origin.into());

        let message = ticket.message.as_bounded_slice();
        <T as snowbridge_pallet_outbound_queue_v2::Config>::MessageQueue::enqueue_message(
            message, origin,
        );
        snowbridge_pallet_outbound_queue_v2::Pallet::<T>::deposit_event(
            snowbridge_pallet_outbound_queue_v2::Event::MessageQueued {
                message: MessageV2 {
                    origin: ticket.origin,
                    fee: ticket.fee,
                    id: ticket.id,
                    commands: vec![].try_into().unwrap(),
                },
            },
        );

        Ok(ticket.id)
    }
}
