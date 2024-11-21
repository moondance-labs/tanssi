use frame_support::pallet_prelude::*;
use parity_scale_codec::DecodeAll;
use snowbridge_core::Channel;
use snowbridge_router_primitives::inbound::envelope::Envelope;
use snowbridge_router_primitives::inbound::MessageProcessor;
use sp_runtime::DispatchError;
use sp_std::vec::Vec;

/// Magic bytes are added in every payload intended for this processor to make sure
/// that we are the intended recipient of the message. Reason being scale encoding is not type aware.
/// So a same set of bytes can be decoded for two different data structures if their
/// total size is same. Magic bytes can be checked after decoding to make sure that the sender
/// indeed send a message intended for this processor.
pub const MAGIC_BYTES: [u8; 4] = [112, 21, 0, 56];

#[derive(Encode, Decode)]
pub struct Payload<T>
where
    T: pallet_external_validators::Config,
{
    pub magic_bytes: [u8; 4],
    pub message: Message<T>,
}

#[derive(Encode, Decode)]
pub enum Message<T>
where
    T: pallet_external_validators::Config,
{
    V1(Command<T>),
}

#[derive(Encode, Decode)]
pub enum Command<T>
where
    T: pallet_external_validators::Config,
{
    ReceiveValidators {
        validators: Vec<<T as pallet_external_validators::Config>::ValidatorId>,
    },
}

pub struct SymbioticMessageProcessor<T>(PhantomData<T>);

impl<T> MessageProcessor for SymbioticMessageProcessor<T>
where
    T: pallet_external_validators::Config,
{
    fn can_process_message(_channel: &Channel, envelope: &Envelope) -> bool {
        let decode_result = Payload::<T>::decode_all(&mut envelope.payload.as_slice());
        if let Ok(payload) = decode_result {
            payload.magic_bytes == MAGIC_BYTES
        } else {
            false
        }
    }

    fn process_message(_channel: Channel, envelope: Envelope) -> Result<(), DispatchError> {
        let decode_result = Payload::<T>::decode_all(&mut envelope.payload.as_slice());
        let message = if let Ok(payload) = decode_result {
            payload.message
        } else {
            return Err(DispatchError::Other("unable to parse the payload"));
        };

        match message {
            Message::V1(Command::ReceiveValidators { validators }) => {
                pallet_external_validators::Pallet::<T>::set_external_validators(validators)?;
                Ok(())
            }
        }
    }
}
