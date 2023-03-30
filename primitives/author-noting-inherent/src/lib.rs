#![cfg_attr(not(feature = "std"), no_std)]

use cumulus_primitives_core::PersistedValidationData;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_inherents::InherentIdentifier;

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

#[derive(Encode, Decode, sp_core::RuntimeDebug, Clone, PartialEq, TypeInfo)]
pub struct OwnParachainInherentData {
    pub validation_data: PersistedValidationData,
    pub relay_chain_state: sp_trie::StorageProof,
}

pub const INHERENT_IDENTIFIER: InherentIdentifier = *b"auno1337";

pub const PARAS_HEADS_INDEX: &[u8] =
    &hex_literal::hex!["cd710b30bd2eab0352ddcc26417aa1941b3c252fcb29d88eff4f3de5de4476c3"];
