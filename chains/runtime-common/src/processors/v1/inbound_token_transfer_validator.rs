use core::marker::PhantomData;
use snowbridge_core::Channel;
use snowbridge_inbound_queue_primitives::v1::Envelope;
use snowbridge_inbound_queue_primitives::v2;
use sp_core::Get;

/// Validates the gateway and channel of an inbound envelope
pub struct InboundTokenTransferValidator<T>(PhantomData<T>);
impl<T> InboundTokenTransferValidator<T>
where
    T: snowbridge_pallet_inbound_queue::Config + pallet_ethereum_token_transfers::Config,
{
    pub fn validate_v1(channel: &Channel, envelope: &Envelope) -> bool {
        // Ensure that the message is intended for the current channel, para_id and agent_id
        if let Some(channel_info) = pallet_ethereum_token_transfers::CurrentChannelInfo::<T>::get()
        {
            if envelope.channel_id != channel_info.channel_id
                || channel.para_id != channel_info.para_id
                || channel.agent_id != channel_info.agent_id
            {
                log::debug!(
                    "Unexpected channel id: {:?} != {:?}",
                    (envelope.channel_id, channel.para_id, channel.agent_id),
                    (
                        channel_info.channel_id,
                        channel_info.para_id,
                        channel_info.agent_id
                    )
                );
                return false;
            }
        } else {
            log::warn!("CurrentChannelInfo not set in storage");
            return false;
        }

        // Check it is from the right gateway
        if envelope.gateway != T::GatewayAddress::get() {
            log::warn!("Wrong gateway address: {:?}", envelope.gateway);
            return false;
        }
        true
    }

    pub fn validate_v2(
        _relayer_address: &<T as frame_system::Config>::AccountId,
        message: &v2::Message,
    ) -> bool {
        // Check it is from the right gateway
        if message.gateway != T::GatewayAddress::get() {
            log::warn!("Wrong gateway address: {:?}", message.gateway);
            return false;
        }
        true
    }
}
