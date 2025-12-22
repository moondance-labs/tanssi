use alloc::vec::Vec;
use alloy_core::sol;
use frame_support::pallet_prelude::{Decode, Encode};
use frame_support::DebugNoBound;
use sp_core::ConstU32;
use sp_runtime::BoundedVec;

/// Magic bytes are added in every payload intended for this processor to make sure
/// that we are the intended recipient of the message. Reason being scale encoding is not type aware.
/// So a same set of bytes can be decoded for two different data structures if their
/// total size is same. Magic bytes can be checked after decoding to make sure that the sender
/// indeed send a message intended for this processor.
pub const MAGIC_BYTES: &[u8; 4] = b"lzb1";

sol! {
    struct SolPayload {
        bytes4  magicBytes;
        SolMessage message;
    }
    struct SolMessage {
        bytes32 lzSourceAddress;
        uint32  lzSourceEndpoint;
        uint32  destinationChain;
        bytes   message;
    }
}

pub type LayerZeroAddress = BoundedVec<u8, ConstU32<32>>;
pub type LayerZeroEndpoint = u32;

#[derive(Encode, Decode, Clone, DebugNoBound)]
pub struct Message {
    pub lz_source_address: LayerZeroAddress,
    pub lz_source_endpoint: LayerZeroEndpoint,
    pub destination_chain: u32,
    pub message: Vec<u8>,
}

// from SolMessage to Message
impl From<SolMessage> for Message {
    fn from(sol_message: SolMessage) -> Self {
        Self {
            lz_source_address: sol_message
                .lzSourceAddress
                .to_vec()
                .try_into()
                .expect("lzSourceAddress is always 32 bytes; qed"),
            lz_source_endpoint: sol_message.lzSourceEndpoint,
            destination_chain: sol_message.destinationChain,
            message: sol_message.message.into(),
        }
    }
}
