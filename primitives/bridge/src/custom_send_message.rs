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
    GetAggregateMessageOrigin:
        Convert<ChannelId, <T as snowbridge_pallet_outbound_queue::Config>::AggregateMessageOrigin>,
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
