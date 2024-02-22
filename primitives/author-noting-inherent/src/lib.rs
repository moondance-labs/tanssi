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

//! # Author Noting Inherent Primitives
//!
//! This crate defines those primitives that should be taken into account when building
//! the author-noting pallet inherent
//!
//! In particular this crate contains:
//! - The Inherent identifier
//! - The client side trait implementations to introduce the inherent
//! - The mock version that gets used both in test files and manual seal
//! - The sproof builder that generates a fake proof that mimics the relay chain sproof

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
mod client_side;

#[cfg(feature = "std")]
mod mock;
#[cfg(feature = "std")]
pub use mock::*;

#[cfg(test)]
mod tests;

use {
    parity_scale_codec::{Decode, Encode},
    scale_info::TypeInfo,
    sp_inherents::InherentIdentifier,
};

#[derive(Encode, Decode, sp_core::RuntimeDebug, Clone, PartialEq, TypeInfo)]
pub struct OwnParachainInherentData {
    pub relay_storage_proof: sp_trie::StorageProof,
}

// Identifier of the author-noting inherent
pub const INHERENT_IDENTIFIER: InherentIdentifier = *b"auno1337";
