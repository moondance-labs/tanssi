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
use crate::Config;
use frame_support::pallet_prelude::MaxEncodedLen;
use frame_support::{
    CloneNoBound, DebugNoBound, Deserialize, EqNoBound, PartialEqNoBound, Serialize,
};
use sp_runtime::BoundedVec;
use tp_bridge::layerzero_message::{LayerZeroAddress, LayerZeroEndpoint};
use tp_traits::__reexports::{Decode, DecodeWithMemTracking, Encode, RuntimeDebug, TypeInfo};

pub type ChainId = u32;
pub type PalletIndex = u8;
pub type CallIndex = u8;

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
    /// List of whitelisted (LayerZeroEndpoint, LayerZeroAddress) tuples allowed to forward messages to this chain
    pub whitelisted_senders:
        BoundedVec<(LayerZeroEndpoint, LayerZeroAddress), T::MaxWhitelistedSenders>,
    /// Pallet index and call index to be used when forwarding messages to this chain
    pub notification_destination: (PalletIndex, CallIndex),
}
