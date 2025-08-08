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

//! # Author Noting Inherent Core Primitives
//!
//! This crate defines the runtime primitives for the author-noting pallet inherent.
//!
//! Client-side methods should be implemented in `tp-author-noting-inherent` crate instead.
//!
//! In particular, this crate contains:
//! - The Inherent identifier
//! - The `OwnParachainInherentData` struct
//! - The `InherentDataProvider` trait impl

#![cfg_attr(not(feature = "std"), no_std)]

use {
    parity_scale_codec::{Decode, DecodeWithMemTracking, Encode},
    scale_info::TypeInfo,
    sp_inherents::InherentIdentifier,
};

#[cfg(feature = "std")]
mod client_side;

#[derive(
    Encode, Decode, DecodeWithMemTracking, sp_core::RuntimeDebug, Clone, PartialEq, TypeInfo,
)]
pub struct OwnParachainInherentData {
    pub relay_storage_proof: sp_trie::StorageProof,
}

// Identifier of the author-noting inherent
pub const INHERENT_IDENTIFIER: InherentIdentifier = *b"auno1337";
