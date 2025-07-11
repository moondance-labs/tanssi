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
    frame_support::traits::{fungible::Mutate, tokens::Preservation, Get},
    parity_scale_codec::DecodeAll,
    snowbridge_core::Channel,
    snowbridge_inbound_queue_primitives::v1::{
        Command, Destination, Envelope, MessageProcessor, MessageV1, VersionedXcmMessage,
    },
    sp_runtime::{traits::MaybeEquivalence, DispatchError, DispatchResult},
};

/// `NativeTokenTransferMessageProcessor` is responsible for receiving and processing the Tanssi
/// native token sent from Ethereum. If the message is valid, it performs the token transfer
/// from the Ethereum sovereign account to the specified destination account.
pub struct NativeTokenTransferMessageProcessor<T>(sp_std::marker::PhantomData<T>);
impl<T> MessageProcessor for NativeTokenTransferMessageProcessor<T>
where
    T: snowbridge_pallet_inbound_queue::Config
        + pallet_ethereum_token_transfers::Config
        + snowbridge_pallet_system::Config,
    T::AccountId: From<[u8; 32]>,
{
    fn can_process_message(channel: &Channel, envelope: &Envelope) -> bool {
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

        // Try decode the message and check the token id is the expected one
        match VersionedXcmMessage::decode_all(&mut envelope.payload.as_slice()) {
            Ok(VersionedXcmMessage::V1(MessageV1 {
                command: Command::SendNativeToken { token_id, .. },
                ..
            })) => {
                let token_location = T::TokenLocationReanchored::get();

                if let Some(expected_token_id) =
                    snowbridge_pallet_system::Pallet::<T>::convert_back(&token_location)
                {
                    if token_id == expected_token_id {
                        true
                    } else {
                        // TODO: ensure this does not warn on container token transfers or other message types, if yes change to debug
                        log::warn!(
                            "NativeTokenTransferMessageProcessor: unexpected token_id: {:?}",
                            token_id
                        );
                        false
                    }
                } else {
                    log::warn!("NativeTokenTransferMessageProcessor: token id not found for location: {:?}", token_location);

                    false
                }
            }
            Ok(msg) => {
                log::trace!(
                    "NativeTokenTransferMessageProcessor: unexpected message: {:?}",
                    msg
                );
                false
            }
            Err(e) => {
                log::trace!("NativeTokenTransferMessageProcessor: failed to decode message. This is expected if the message is not for this processor. Error: {:?}", e);
                false
            }
        }
    }

    fn process_message(_channel: Channel, envelope: Envelope) -> DispatchResult {
        // - Decode payload as SendNativeToken
        let message = VersionedXcmMessage::decode_all(&mut envelope.payload.as_slice())
        .map_err(|e| {
            log::trace!("NativeTokenTransferMessageProcessor: failed to decode message. This is expected if the message is not for this processor. Error: {:?}", e);

            DispatchError::Other("unable to parse the envelope payload")
        })?;

        log::trace!("NativeTokenTransferMessageProcessor: {:?}", message);

        match message {
            VersionedXcmMessage::V1(MessageV1 {
                chain_id: _,
                command:
                    Command::SendNativeToken {
                        destination:
                            Destination::AccountId32 {
                                id: destination_account,
                            },
                        amount,
                        ..
                    },
            }) => {
                // - Transfer the amounts of tokens from Ethereum sov account to the destination
                let sovereign_account = T::EthereumSovereignAccount::get();

                if let Err(e) = T::Currency::transfer(
                    &sovereign_account,
                    &destination_account.into(),
                    amount.into(),
                    Preservation::Preserve,
                ) {
                    log::warn!(
                        "NativeTokenTransferMessageProcessor: Error transferring tokens: {:?}",
                        e
                    );
                }

                Ok(())
            }
            msg => {
                log::warn!(
                    "NativeTokenTransferMessageProcessor: unexpected message: {:?}",
                    msg
                );
                Ok(())
            }
        }
    }
}
