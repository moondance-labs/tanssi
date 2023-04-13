//! # Author Noting Inherent Primitives
//!
//! This crate defines those primitives that should be taken into account when building
//! the author-noting pallet inherent
//!
//! In particular this crate contains:
//! - The hardcoded relay key that needs to be read
//! - The Inherent identifier
//! - The client side trait implementations to introduce the inherent
//! - The mock version that gets used both in test files and manual seal
//! - The sproof builder that generates a fake proof that mimics the relay chain sproof
#![cfg_attr(not(feature = "std"), no_std)]

use cumulus_primitives_core::ParaId;
use cumulus_primitives_core::PersistedValidationData;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_inherents::InherentIdentifier;
use sp_io::hashing::twox_64;
use sp_std::vec::Vec;

#[cfg(feature = "std")]
mod client_side;
#[cfg(feature = "std")]
pub use client_side::*;
#[cfg(feature = "std")]
mod mock;
#[cfg(feature = "std")]
pub use mock::*;
#[cfg(feature = "std")]
mod sproof_builder;
#[cfg(feature = "std")]
pub use sproof_builder::*;
#[cfg(test)]
mod tests;

#[derive(Encode, Decode, sp_core::RuntimeDebug, Clone, PartialEq, TypeInfo)]
pub struct OwnParachainInherentData {
    pub validation_data: PersistedValidationData,
    pub relay_chain_state: sp_trie::StorageProof,
}

// Identifier of the author-noting inherent
pub const INHERENT_IDENTIFIER: InherentIdentifier = *b"auno1337";