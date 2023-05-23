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

//! Data structures used to store a ContainerChain ChainSpec in the registrar pallet

#![cfg_attr(not(feature = "std"), no_std)]

use {
    frame_support::BoundedVec,
    parity_scale_codec::{Decode, Encode},
    sp_core::Get,
    sp_std::vec::Vec,
};

#[cfg(feature = "json")]
pub mod json;

// TODO: improve serialization of storage field
// Currently it looks like this:
/*
"storage": [
    {
      "key": "0x0d715f2646c8f85767b5d2764bb2782604a74d81251e398fd8a0a4d55023bb3f"
      "value": "0xd1070000"
    },
    {
      "key": "0x0d715f2646c8f85767b5d2764bb278264e7b9012096b41c4eb3aaf947f6ea429"
      "value": "0x0000"
    }
]
 */
// Ideally it would be:
/*
"storage": {
    "0x0d715f2646c8f85767b5d2764bb2782604a74d81251e398fd8a0a4d55023bb3f": "0xd1070000",
    "0x0d715f2646c8f85767b5d2764bb278264e7b9012096b41c4eb3aaf947f6ea429": "0x0000"
}
 */
// This is just so it looks nicer on polkadot.js, the functionality is the same
// The original approach of using `storage: BTreeMap<Vec<u8>, Vec<u8>>` looks very bad
// in polkadot.js, because `Vec<u8>` is serialized as `[12, 51, 124]` instead of hex.
// That's why we use `serde(with = "sp_core::bytes")` everywhere, to convert it to hex.
#[cfg_attr(feature = "std", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, scale_info::TypeInfo)]
pub struct ContainerChainGenesisData {
    pub storage: Vec<ContainerChainGenesisDataItem>,
    // TODO: make all these Vec<u8> bounded
    #[cfg_attr(feature = "std", serde(with = "sp_core::bytes"))]
    pub name: Vec<u8>,
    #[cfg_attr(feature = "std", serde(with = "sp_core::bytes"))]
    pub id: Vec<u8>,
    pub fork_id: Option<Vec<u8>>,
    #[cfg_attr(feature = "std", serde(with = "sp_core::bytes"))]
    pub extensions: Vec<u8>,
    pub properties: TokenMetadata,
}

// TODO: turn this into a Config type parameter
// The problem with that is that it forces ContainerChainGenesisData to be generic,
// and the automatically derived traits force the generic parameter to implement those traits.
// The errors are like "MaxLengthTokenSymbol does not implement Debug".
// The solution is to either implement all the traits manually, or use a helper crate like
// derivative, although that does not seem to support deriving the substrate traits.
pub struct MaxLengthTokenSymbol;

impl Get<u32> for MaxLengthTokenSymbol {
    fn get() -> u32 {
        255
    }
}

#[cfg_attr(feature = "std", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, scale_info::TypeInfo)]
pub struct TokenMetadata {
    pub token_symbol: BoundedVec<u8, MaxLengthTokenSymbol>,
    pub ss58_format: u32,
    pub token_decimals: u32,
}

impl Default for TokenMetadata {
    fn default() -> Self {
        // Default values from polkadot.js
        Self {
            token_symbol: BoundedVec::truncate_from(b"UNIT".to_vec()),
            ss58_format: 42,
            token_decimals: 12,
        }
    }
}

#[cfg_attr(feature = "std", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, scale_info::TypeInfo)]
pub struct ContainerChainGenesisDataItem {
    #[cfg_attr(feature = "std", serde(with = "sp_core::bytes"))]
    pub key: Vec<u8>,
    #[cfg_attr(feature = "std", serde(with = "sp_core::bytes"))]
    pub value: Vec<u8>,
}

impl From<(Vec<u8>, Vec<u8>)> for ContainerChainGenesisDataItem {
    fn from(x: (Vec<u8>, Vec<u8>)) -> Self {
        Self {
            key: x.0,
            value: x.1,
        }
    }
}

impl From<ContainerChainGenesisDataItem> for (Vec<u8>, Vec<u8>) {
    fn from(x: ContainerChainGenesisDataItem) -> Self {
        (x.key, x.value)
    }
}
