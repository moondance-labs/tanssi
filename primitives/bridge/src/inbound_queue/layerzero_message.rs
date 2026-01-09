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

/// Maximum size of a LayerZero message payload in bytes (8 KB).
/// This limit prevents memory exhaustion from arbitrarily large payloads.
pub const MAX_LAYERZERO_PAYLOAD_SIZE: u32 = 8 * 1024;

/// Bounded payload type for inbound LayerZero messages.
pub type LayerZeroInboundPayload = BoundedVec<u8, ConstU32<MAX_LAYERZERO_PAYLOAD_SIZE>>;

/// Bounded payload type for outbound LayerZero messages.
pub type LayerZeroOutboundPayload = BoundedVec<u8, ConstU32<MAX_LAYERZERO_PAYLOAD_SIZE>>;

pub type LayerZeroAddress = BoundedVec<u8, ConstU32<32>>;
pub type LayerZeroEndpoint = u32;

sol! {
    struct InboundSolMessage {
        bytes32 lzSourceAddress;
        uint32  lzSourceEndpoint;
        uint32  destinationChain;
        bytes   payload;
    }
}

#[derive(Encode, Decode, DecodeWithMemTracking, Clone, DebugNoBound, PartialEq, Eq, TypeInfo)]
pub struct InboundMessage {
    pub lz_source_address: LayerZeroAddress,
    pub lz_source_endpoint: LayerZeroEndpoint,
    pub destination_chain: u32,
    pub payload: LayerZeroInboundPayload,
}

/// Error when converting from InboundSolMessage to InboundMessage
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InboundMessageConversionError {
    /// The message payload exceeds the maximum allowed size
    PayloadTooLarge { size: usize, max: u32 },
}

impl core::fmt::Display for InboundMessageConversionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::PayloadTooLarge { size, max } => {
                write!(f, "payload size {} exceeds maximum {}", size, max)
            }
        }
    }
}

// from InboundSolMessage to InboundMessage
impl TryFrom<InboundSolMessage> for InboundMessage {
    type Error = InboundMessageConversionError;

    fn try_from(sol_message: InboundSolMessage) -> Result<Self, Self::Error> {
        let payload_bytes: alloc::vec::Vec<u8> = sol_message.payload.into();
        let payload_len = payload_bytes.len();
        let payload: LayerZeroInboundPayload = payload_bytes.try_into().map_err(|_| {
            InboundMessageConversionError::PayloadTooLarge {
                size: payload_len,
                max: MAX_LAYERZERO_PAYLOAD_SIZE,
            }
        })?;

        Ok(Self {
            lz_source_address: sol_message
                .lzSourceAddress
                .to_vec()
                .try_into()
                .expect("lzSourceAddress is always 32 bytes; qed"),
            lz_source_endpoint: sol_message.lzSourceEndpoint,
            destination_chain: sol_message.destinationChain,
            payload,
        })
    }
}

sol! {
    struct OutboundSolMessage {
        uint32  sourceChain;
        bytes32 lzDestinationAddress;
        uint32  lzDestinationEndpoint;
        bytes   payload;
    }
}

#[derive(Encode, Decode, DecodeWithMemTracking, Clone, DebugNoBound, PartialEq, Eq, TypeInfo)]
pub struct OutboundMessage {
    pub source_chain: u32,
    pub lz_destination_address: LayerZeroAddress,
    pub lz_destination_endpoint: LayerZeroEndpoint,
    pub payload: LayerZeroOutboundPayload,
}

// from OutboundMessage to OutboundSolMessage
impl From<OutboundMessage> for OutboundSolMessage {
    fn from(message: OutboundMessage) -> Self {
        let mut destination_address = [0u8; 32];
        let addr_slice = message.lz_destination_address.as_slice();
        let len = addr_slice.len().min(32);
        destination_address[..len].copy_from_slice(&addr_slice[..len]);

        Self {
            sourceChain: message.source_chain,
            lzDestinationAddress: alloy_core::primitives::FixedBytes(destination_address),
            lzDestinationEndpoint: message.lz_destination_endpoint,
            payload: message.payload.to_vec().into(),
        }
    }
}
