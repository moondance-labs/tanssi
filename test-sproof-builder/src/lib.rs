// Copyright 2019-2022 Moondance Labs Ltd.
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
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>.

use {
    cumulus_primitives_core::{relay_chain::HeadData, ParaId},
    frame_support::Hashable,
    parity_scale_codec::Encode,
    sp_runtime::traits::{BlakeTwo256, HashFor},
    sp_state_machine::Backend,
    sp_trie::{MemoryDB, StorageProof},
    tp_collator_assignment::AssignedCollators,
    tp_core::well_known_keys::{COLLATOR_ASSIGNMENT_INDEX, PARAS_HEADS_INDEX},
};

/// Enum representing how we want to insert the Header
#[derive(Clone)]
pub enum HeaderAs {
    AlreadyEncoded(Vec<u8>),
    NonEncoded(sp_runtime::generic::Header<u32, BlakeTwo256>),
}

/// Builds a sproof (portmanteau of 'spoof' and 'proof') of the relay chain state.
#[derive(Clone)]
pub struct ParaHeaderSproofBuilderItem {
    /// The para id of the current parachain.
    pub para_id: ParaId,

    /// The author_id, which represents a Header with a Aura Digest
    pub author_id: HeaderAs,
}

impl Default for ParaHeaderSproofBuilderItem {
    fn default() -> Self {
        Self {
            para_id: ParaId::from(200),
            author_id: HeaderAs::AlreadyEncoded(vec![]),
        }
    }
}

/// Builds a sproof (portmanteau of 'spoof' and 'proof') of the relay chain state.
/// Receives a vec of individual ParaHeaderSproofBuilderItem items of which
/// we need to insert the header
#[derive(Clone, Default)]
pub struct ParaHeaderSproofBuilder {
    pub items: Vec<ParaHeaderSproofBuilderItem>,
}

impl ParaHeaderSproofBuilder {
    pub fn into_state_root_and_proof(
        self,
    ) -> (
        cumulus_primitives_core::relay_chain::Hash,
        sp_state_machine::StorageProof,
    ) {
        let (db, root) =
            MemoryDB::<HashFor<cumulus_primitives_core::relay_chain::Block>>::default_with_root();
        let state_version = Default::default(); // for test using default.
        let mut backend = sp_state_machine::TrieBackendBuilder::new(db, root).build();

        let mut relevant_keys = Vec::new();
        {
            use parity_scale_codec::Encode as _;

            let mut insert = |key: Vec<u8>, value: Vec<u8>| {
                relevant_keys.push(key.clone());
                backend.insert(vec![(None, vec![(key, Some(value))])], state_version);
            };

            for item in self.items {
                let para_key = item.para_id.twox_64_concat();
                let key = [PARAS_HEADS_INDEX, para_key.as_slice()].concat();

                let encoded = match item.author_id {
                    HeaderAs::AlreadyEncoded(encoded) => encoded,
                    HeaderAs::NonEncoded(header) => header.encode(),
                };

                let head_data: HeadData = encoded.into();
                insert(key, head_data.encode());
            }
        }

        let root = backend.root().clone();
        let proof = sp_state_machine::prove_read(backend, relevant_keys).expect("prove read");

        (root, proof)
    }

    pub fn relevant_keys(self) -> Vec<Vec<u8>> {
        let mut relevant_keys = Vec::new();
        {
            for item in self.items {
                let para_key = item.para_id.twox_64_concat();
                let key = [PARAS_HEADS_INDEX, para_key.as_slice()].concat();

                relevant_keys.push(key.clone());
            }
        }
        relevant_keys
    }

    // Construct the proof from an existing state and proof
    pub fn from_existing_state(
        self,
        root: cumulus_primitives_core::relay_chain::Hash,
        state: StorageProof,
    ) -> (
        cumulus_primitives_core::relay_chain::Hash,
        sp_state_machine::StorageProof,
    ) {
        // Recover the db
        let db = state
            .clone()
            .into_memory_db::<HashFor<cumulus_primitives_core::relay_chain::Block>>();
        let state_version = Default::default(); // for test using default.
                                                // Construct the backend
        let mut backend = sp_state_machine::TrieBackendBuilder::new(db, root).build();
        // Fetch all existing keys
        let mut relevant_keys = backend
            .keys(Default::default())
            .expect("we should have keys if entering this func")
            .map(|result| result.unwrap())
            .collect::<Vec<_>>();

        // Insert new keys and add them to relevant keys
        {
            use parity_scale_codec::Encode as _;

            let mut insert = |key: Vec<u8>, value: Vec<u8>| {
                relevant_keys.push(key.clone());
                backend.insert(vec![(None, vec![(key, Some(value))])], state_version);
            };

            for item in self.items {
                let para_key = item.para_id.twox_64_concat();
                let key = [PARAS_HEADS_INDEX, para_key.as_slice()].concat();

                let encoded = match item.author_id {
                    HeaderAs::AlreadyEncoded(encoded) => encoded,
                    HeaderAs::NonEncoded(header) => header.encode(),
                };

                let head_data: HeadData = encoded.into();
                insert(key, head_data.encode());
            }
        }

        // Construct proof again
        let root = backend.root().clone();
        let proof = sp_state_machine::prove_read(backend, relevant_keys).expect("prove read");

        (root, proof)
    }
}

/// Builds a sproof (portmanteau of 'spoof' and 'proof') of the orchestrator chain state.
#[derive(Clone, Encode, Default)]
pub struct CollatorAssignmentSproofBuilder<AccountId> {
    /// The para id of the current parachain.
    pub collator_assignment: AssignedCollators<AccountId>,
}

impl<AccountId: Encode> CollatorAssignmentSproofBuilder<AccountId> {
    pub fn into_state_root_and_proof(
        self,
    ) -> (
        cumulus_primitives_core::relay_chain::Hash,
        sp_state_machine::StorageProof,
    ) {
        let (db, root) =
            MemoryDB::<HashFor<cumulus_primitives_core::relay_chain::Block>>::default_with_root();
        let state_version = Default::default();
        let mut backend = sp_state_machine::TrieBackendBuilder::new(db, root).build();
        let mut relevant_keys = Vec::new();

        let mut insert = |key: Vec<u8>, value: Vec<u8>| {
            relevant_keys.push(key.clone());
            backend.insert(vec![(None, vec![(key, Some(value))])], state_version);
        };

        insert(COLLATOR_ASSIGNMENT_INDEX.to_vec(), self.encode());

        let root = backend.root().clone();
        let proof = sp_state_machine::prove_read(backend, relevant_keys).expect("prove read");

        (root, proof)
    }
}
