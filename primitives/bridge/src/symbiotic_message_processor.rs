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
    frame_support::pallet_prelude::*,
    parity_scale_codec::DecodeAll,
    snowbridge_core::{Channel, PRIMARY_GOVERNANCE_CHANNEL},
    snowbridge_inbound_queue_primitives::v1::{Envelope, MessageProcessor},
    sp_runtime::DispatchError,
    sp_std::vec::Vec,
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

impl<T> MessageProcessor for SymbioticMessageProcessor<T>
where
    T: pallet_external_validators::Config,
{
    fn can_process_message(_channel: &Channel, envelope: &Envelope) -> bool {
        let decode_result = Payload::<T>::decode_all(&mut envelope.payload.as_slice());
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

    fn process_message(_channel: Channel, envelope: Envelope) -> Result<(), DispatchError> {
        let decode_result = Payload::<T>::decode_all(&mut envelope.payload.as_slice());
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
                if envelope.channel_id != PRIMARY_GOVERNANCE_CHANNEL {
                    return Err(DispatchError::Other(
                        "Received governance message from invalid channel id",
                    ));
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
