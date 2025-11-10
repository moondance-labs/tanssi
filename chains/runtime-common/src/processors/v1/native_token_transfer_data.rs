use parity_scale_codec::DecodeAll;
use snowbridge_inbound_queue_primitives::v1::{
    Command, Destination, MessageV1, VersionedXcmMessage,
};
use sp_core::H256;

/// Information needed to process a native token transfer message from ethereum.
pub struct NativeTokenTransferData {
    pub token_id: H256,
    pub destination: Destination,
    pub amount: u128,
    pub fee: u128,
}

impl NativeTokenTransferData {
    pub fn decode_native_token_message(mut payload: &[u8]) -> Option<Self> {
        match VersionedXcmMessage::decode_all(&mut payload) {
            Ok(VersionedXcmMessage::V1(MessageV1 {
                command:
                    Command::SendNativeToken {
                        token_id,
                        destination,
                        amount,
                        fee,
                    },
                ..
            })) => Some(NativeTokenTransferData {
                token_id,
                destination,
                amount,
                fee,
            }),
            Ok(msg) => {
                log::trace!("NativeTokenTransferData: unexpected message: {:?}", msg);
                None
            }
            Err(e) => {
                log::trace!("NativeTokenTransferData: failed to decode message. This is expected if the message is not related to a SendNativeToken command. Error: {:?}", e);
                None
            }
        }
    }
}
