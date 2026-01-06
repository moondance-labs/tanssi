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
use alloy_core::sol;
use frame_support::pallet_prelude::{Decode, Encode};
use frame_support::DebugNoBound;
use scale_info::TypeInfo;
use sp_core::{ConstU32, DecodeWithMemTracking};
use sp_runtime::BoundedVec;

/// Magic bytes are added in every payload intended for this processor to make sure
/// that we are the intended recipient of the message. Reason being scale encoding is not type aware.
/// So a same set of bytes can be decoded for two different data structures if their
/// total size is same. Magic bytes can be checked after decoding to make sure that the sender
/// indeed send a message intended for this processor.
pub const MAGIC_BYTES: &[u8; 4] = b"lzb1";

/// Maximum size of a LayerZero message payload in bytes (8 KB).
/// This limit prevents memory exhaustion from arbitrarily large payloads.
pub const MAX_LAYERZERO_MESSAGE_SIZE: u32 = 8 * 1024;

/// Bounded message payload type for LayerZero messages.
pub type LayerZeroMessage = BoundedVec<u8, ConstU32<MAX_LAYERZERO_MESSAGE_SIZE>>;

sol! {
    struct InboundSolPayload {
        bytes4  magicBytes;
        InboundSolMessage message;
    }
    struct InboundSolMessage {
        bytes32 lzSourceAddress;
        uint32  lzSourceEndpoint;
        uint32  destinationChain;
        bytes   message;
    }
}

pub type LayerZeroAddress = BoundedVec<u8, ConstU32<32>>;
pub type LayerZeroEndpoint = u32;

#[derive(Encode, Decode, DecodeWithMemTracking, Clone, DebugNoBound, PartialEq, Eq, TypeInfo)]
pub struct InboundMessage {
    pub lz_source_address: LayerZeroAddress,
    pub lz_source_endpoint: LayerZeroEndpoint,
    pub destination_chain: u32,
    pub message: LayerZeroMessage,
}

/// Error when converting from InboundSolMessage to InboundMessage
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InboundMessageConversionError {
    /// The message payload exceeds the maximum allowed size
    MessageTooLarge { size: usize, max: u32 },
}

impl core::fmt::Display for InboundMessageConversionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MessageTooLarge { size, max } => {
                write!(f, "message size {} exceeds maximum {}", size, max)
            }
        }
    }
}

// from InboundSolMessage to InboundMessage
impl TryFrom<InboundSolMessage> for InboundMessage {
    type Error = InboundMessageConversionError;

    fn try_from(sol_message: InboundSolMessage) -> Result<Self, Self::Error> {
        let message_bytes: alloc::vec::Vec<u8> = sol_message.message.into();
        let message_len = message_bytes.len();
        let message: LayerZeroMessage =
            message_bytes
                .try_into()
                .map_err(|_| InboundMessageConversionError::MessageTooLarge {
                    size: message_len,
                    max: MAX_LAYERZERO_MESSAGE_SIZE,
                })?;

        Ok(Self {
            lz_source_address: sol_message
                .lzSourceAddress
                .to_vec()
                .try_into()
                .expect("lzSourceAddress is always 32 bytes; qed"),
            lz_source_endpoint: sol_message.lzSourceEndpoint,
            destination_chain: sol_message.destinationChain,
            message,
        })
    }
}
