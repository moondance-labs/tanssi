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

use cumulus_primitives_core::relay_chain::AccountId;
use snowbridge_core::ChannelId;
use snowbridge_inbound_queue_primitives::v2;
use {
    alloc::vec::Vec,
    frame_support::pallet_prelude::*,
    parity_scale_codec::DecodeAll,
    snowbridge_core::{Channel, PRIMARY_GOVERNANCE_CHANNEL},
    snowbridge_inbound_queue_primitives::{
        v1::{Envelope, MessageProcessor},
        v2::MessageProcessorError,
    },
    sp_runtime::DispatchError,
};

/// Magic bytes are added in every payload intended for this processor to make sure
/// that we are the intended recipient of the message. Reason being scale encoding is not type aware.
/// So a same set of bytes can be decoded for two different data structures if their
/// total size is same. Magic bytes can be checked after decoding to make sure that the sender
/// indeed send a message intended for this processor.
pub const MAGIC_BYTES: [u8; 4] = [112, 21, 0, 56];

/// Payload is the whole data we expect to receive from the relayer
#[derive(Encode, Decode, DebugNoBound)]
pub struct Payload<T>
where
    T: pallet_external_validators::Config,
{
    pub magic_bytes: [u8; 4],
    pub message: Message<T>,
}

/// Actual message inside the payload
#[derive(Encode, Decode, DebugNoBound)]
pub enum Message<T>
where
    T: pallet_external_validators::Config,
{
    V1(InboundCommand<T>),
}

/// Command to be executed by this message processor
#[derive(Encode, Decode, DebugNoBound)]
pub enum InboundCommand<T>
where
    T: pallet_external_validators::Config,
{
    ReceiveValidators {
        validators: Vec<<T as pallet_external_validators::Config>::ValidatorId>,
        external_index: u64,
    },
}

pub struct SymbioticMessageProcessor<T>(PhantomData<T>);

impl<T> SymbioticMessageProcessor<T>
where
    T: pallet_external_validators::Config,
{
    fn can_process_message(mut payload: &[u8]) -> bool {
        let decode_result = Payload::<T>::decode_all(&mut payload);
        match decode_result {
            Ok(payload) => {
                if payload.magic_bytes == MAGIC_BYTES {
                    true
                } else {
                    log::debug!("SymbioticMessageProcessor: magic number mismatch, will try next processor: {:?}", payload.magic_bytes);
                    false
                }
            }
            Err(e) => {
                // Message cannot be decoded as `Payload`.
                // This is expected if the message is intended for a different processor.
                log::trace!("SymbioticMessageProcessor: failed to decode payload. This is expected if the message is not for this processor. Error: {:?}", e);
                false
            }
        }
    }

    fn process_message(
        channel_id: Option<ChannelId>,
        mut payload: &[u8],
    ) -> Result<(), DispatchError> {
        let decode_result = Payload::<T>::decode_all(&mut payload);
        let message = if let Ok(payload) = decode_result {
            payload.message
        } else {
            return Err(DispatchError::Other("unable to parse the envelope payload"));
        };

        log::trace!("SymbioticMessageProcessor: {:?}", message);

        match message {
            Message::V1(InboundCommand::ReceiveValidators {
                validators,
                external_index,
            }) => {
                if let Some(channel_id) = channel_id {
                    if channel_id != PRIMARY_GOVERNANCE_CHANNEL {
                        return Err(DispatchError::Other(
                            "Received governance message from invalid channel id",
                        ));
                    }
                }
                pallet_external_validators::Pallet::<T>::set_external_validators_inner(
                    validators,
                    external_index,
                )?;
                Ok(())
            }
        }
    }
}

impl<T> v2::MessageProcessor<AccountId> for SymbioticMessageProcessor<T>
where
    T: pallet_external_validators::Config,
{
    fn can_process_message(_who: &AccountId, message: &v2::Message) -> bool {
        match &message.payload {
            v2::message::Payload::Raw(data) => Self::can_process_message(&data),
            v2::message::Payload::CreateAsset { .. } => false,
        }
    }

    fn process_message(
        _who: AccountId,
        message: v2::Message,
    ) -> Result<[u8; 32], MessageProcessorError> {
        match &message.payload {
            v2::message::Payload::Raw(data) => Self::process_message(None, &data)
                .map(|_| [0; 32])
                .map_err(|e| MessageProcessorError::ProcessMessage(e)),
            v2::message::Payload::CreateAsset { .. } => Err(MessageProcessorError::ProcessMessage(
                DispatchError::Other("Create asset is not supported"),
            )),
        }
    }
}

impl<T> MessageProcessor for SymbioticMessageProcessor<T>
where
    T: pallet_external_validators::Config,
{
    fn can_process_message(_channel: &Channel, envelope: &Envelope) -> bool {
        Self::can_process_message(&envelope.payload)
    }

    fn process_message(_channel: Channel, envelope: Envelope) -> Result<(), DispatchError> {
        Self::process_message(Some(envelope.channel_id), &envelope.payload)
    }
}
