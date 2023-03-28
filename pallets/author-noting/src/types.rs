use cumulus_primitives_core::PersistedValidationData;
use scale_info::TypeInfo;
use parity_scale_codec::{Encode, Decode};
#[derive(Encode, Decode, sp_core::RuntimeDebug, Clone, PartialEq, TypeInfo)]
pub struct OwnParachainInherentData {
    pub validation_data: PersistedValidationData,
    pub relay_chain_state: sp_trie::StorageProof,
}