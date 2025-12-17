use alloy_core::sol;
use frame_support::DebugNoBound;
use frame_support::pallet_prelude::{Decode, Encode};
use xcm::latest::Location;

/// Magic bytes are added in every payload intended for this processor to make sure
/// that we are the intended recipient of the message. Reason being scale encoding is not type aware.
/// So a same set of bytes can be decoded for two different data structures if their
/// total size is same. Magic bytes can be checked after decoding to make sure that the sender
/// indeed send a message intended for this processor.
pub const MAGIC_BYTES: &[u8; 4] = b"lzb1";


sol! {
    struct Payload {
        bytes4  magicBytes;
        Message message;
    }
    struct Message {
        bytes32 lzSourceAddress;
        uint32  lzSourceEndpoint;
        uint32  destinationChain;
        bytes   message;
    }
}
