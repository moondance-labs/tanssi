mod eth_token_local_processor;
mod inbound_token_transfer_validator;
mod native_container_tokens_processor;
mod native_token_transfer_data;
mod native_token_transfer_processor;

use core::marker::PhantomData;
pub use eth_token_local_processor::EthTokensLocalProcessor;
pub use inbound_token_transfer_validator::InboundTokenTransferValidator;
pub use native_container_tokens_processor::NativeContainerTokensProcessor;
pub use native_token_transfer_data::NativeTokenTransferData;
pub use native_token_transfer_processor::NativeTokenTransferMessageProcessor;
use snowbridge_core::Channel;
use snowbridge_inbound_queue_primitives::v1::Envelope;
use sp_core::Get;

/// Validates the gateway and channel of an inbound envelope
pub struct GatewayAndChannelValidator<T>(PhantomData<T>);
impl<T> GatewayAndChannelValidator<T>
where
    T: snowbridge_pallet_inbound_queue::Config + pallet_ethereum_token_transfers::Config,
{
    pub fn validate_gateway_and_channel(channel: &Channel, envelope: &Envelope) -> bool {
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
}
