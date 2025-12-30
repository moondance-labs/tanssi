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

//! Types used by the LzRouter pallet.

use crate::Config;
use frame_support::pallet_prelude::MaxEncodedLen;
use frame_support::{
    CloneNoBound, DebugNoBound, Deserialize, EqNoBound, PartialEqNoBound, Serialize,
};
use sp_runtime::BoundedVec;
use tp_bridge::layerzero_message::{LayerZeroAddress, LayerZeroEndpoint};
use tp_traits::__reexports::{Decode, DecodeWithMemTracking, Encode, RuntimeDebug, TypeInfo};

/// Parachain/container chain identifier (matches the para_id)
pub type ChainId = u32;

/// Index of a pallet in the runtime
pub type PalletIndex = u8;

/// Index of an extrinsic call within a pallet
pub type CallIndex = u8;

/// Configuration for forwarding LayerZero messages to a container chain.
#[derive(
    DebugNoBound,
    PartialEqNoBound,
    EqNoBound,
    Encode,
    Decode,
    CloneNoBound,
    TypeInfo,
    MaxEncodedLen,
    DecodeWithMemTracking,
)]
#[scale_info(skip_type_params(T))]
pub struct MessageForwardingConfig<T: Config> {
    /// Whitelisted (endpoint, address) tuples allowed to send messages.
    pub whitelisted_senders:
        BoundedVec<(LayerZeroEndpoint, LayerZeroAddress), T::MaxWhitelistedSenders>,

    /// Target (pallet_index, call_index) on the container chain.
    pub notification_destination: (PalletIndex, CallIndex),
}
