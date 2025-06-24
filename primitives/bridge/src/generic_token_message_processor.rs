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
    sp_std::marker::PhantomData,
    snowbridge_router_primitives::inbound::{envelope::Envelope, MessageProcessor, MessageV1, Command as SnowbridgeCommand},
    sp_runtime::DispatchError,
    parity_scale_codec::DecodeAll,
};

pub struct GenericTokenMessageProcessor<T, NativeTokenProcessor, ForeignTokenProcessor>(PhantomData<(T, NativeTokenProcessor, ForeignTokenProcessor)>);

impl<T, NativeTokenProcessor, ForeignTokenProcessor> MessageProcessor for GenericTokenMessageProcessor<T, NativeTokenProcessor, ForeignTokenProcessor>
where
    T: frame_system::Config,
    NativeTokenProcessor: MessageProcessor,
    ForeignTokenProcessor: MessageProcessor,
{
    fn can_process_message(channel: &Channel, envelope: &Envelope) -> bool {
        match VersionedXcmMessage::decode_all(&mut envelope.payload.as_slice()) {
            Ok(VersionedXcmMessage::V1(MessageV1 {
                command: SnowbridgeCommand::SendNativeToken { .. },
                ..
            })) => {
                NativeTokenProcessor::can_process_message(channel, envelope)
            }
            Ok(VersionedXcmMessage::V1(MessageV1 {
                command: SnowbridgeCommand::SendToken { .. },
                ..
            })) => {
                ForeignTokenProcessor::can_process_message(channel, envelope)
            }
            Ok(VersionedXcmMessage::V1(MessageV1 {
                command: SnowbridgeCommand::RegisterToken { .. },
                ..
            })) => {
                true
            }
            Err(e) => {
                log::trace!("GenericMessageProcessor: failed to decode message. Error: {:?}", e);
                false
            }
        }
    }

    fn process_message(channel: Channel, envelope: Envelope) -> Result<(), DispatchError> {
        match VersionedXcmMessage::decode_all(&mut envelope.payload.as_slice()) {
            Ok(VersionedXcmMessage::V1(MessageV1 {
                command: SnowbridgeCommand::SendNativeToken { .. },
                ..
            })) => {
                NativeTokenProcessor::process_message(channel, envelope)
            }
            Ok(VersionedXcmMessage::V1(MessageV1 {
                command: SnowbridgeCommand::SendToken { .. },
                ..
            })) => {
                ForeignTokenProcessor::process_message(channel, envelope)
            }
            Ok(VersionedXcmMessage::V1(MessageV1 {
                command: SnowbridgeCommand::RegisterToken { .. },
                ..
            })) => {
                Ok(())
            }
            Err(e) => {
                log::trace!("GenericMessageProcessor: failed to process message. Error: {:?}", e);
                Ok(())
            }
        }
    }
}