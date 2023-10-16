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

//! # Chain Snapshot Primitives
//!
//! This crate defines those primitives to retrieve keys from a defined Backend

#![cfg_attr(not(feature = "std"), no_std)]

use {
    parity_scale_codec::Decode,
    sp_runtime::traits::HashingFor,
    sp_state_machine::{Backend, TrieBackend, TrieBackendBuilder},
    sp_trie::{HashDBT, MemoryDB, StorageProof, EMPTY_PREFIX},
};

#[derive(Debug)]
pub enum ReadEntryErr {
    /// The value cannot be extracted from the proof.
    Proof,
    /// The value cannot be decoded.
    Decode,
    /// The value is expected to be present on the relay chain, but it doesn't exist.
    Absent,
    /// The proof provided does not match the root provided
    RootMismatch,
}

/// Read an entry given by the key and try to decode it. If the value specified by the key according
/// to the proof is empty, the `fallback` value will be returned.
///
/// Returns `Err` in case the backend can't return the value under the specific key (likely due to
/// a malformed proof), in case the decoding fails, or in case where the value is empty in the relay
/// chain state and no fallback was provided.
fn read_entry<T, B, Block>(backend: &B, key: &[u8], fallback: Option<T>) -> Result<T, ReadEntryErr>
where
    T: Decode,
    B: Backend<HashingFor<Block>>,
    Block: sp_runtime::traits::Block,
{
    backend
        .storage(key)
        .map_err(|_| ReadEntryErr::Proof)?
        .map(|raw_entry| T::decode(&mut &raw_entry[..]).map_err(|_| ReadEntryErr::Decode))
        .transpose()?
        .or(fallback)
        .ok_or(ReadEntryErr::Absent)
}

/// Read an optional entry given by the key and try to decode it.
/// Returns `None` if the value specified by the key according to the proof is empty.
///
/// Returns `Err` in case the backend can't return the value under the specific key (likely due to
/// a malformed proof) or if the value couldn't be decoded.
fn read_optional_entry<T, B, Block>(backend: &B, key: &[u8]) -> Result<Option<T>, ReadEntryErr>
where
    T: Decode,
    B: Backend<HashingFor<Block>>,
    Block: sp_runtime::traits::Block,
{
    match read_entry::<T, B, Block>(backend, key, None) {
        Ok(v) => Ok(Some(v)),
        Err(ReadEntryErr::Absent) => Ok(None),
        Err(err) => Err(err),
    }
}

/// A state proof extracted from the relay chain.
///
/// This state proof is extracted from the relay chain block we are building on top of.
pub struct GenericStateProof<Block: sp_runtime::traits::Block> {
    trie_backend: TrieBackend<MemoryDB<HashingFor<Block>>, HashingFor<Block>>,
}

impl<Block: sp_runtime::traits::Block> GenericStateProof<Block> {
    /// Create a new instance of `Self`.
    ///
    /// Returns an error if the given `relay_parent_storage_root` is not the root of the given
    /// `proof`.
    pub fn new(
        relay_parent_storage_root: Block::Hash,
        proof: StorageProof,
    ) -> Result<Self, ReadEntryErr> {
        // Retrieve whether the proof is empty
        let proof_empty = proof.is_empty();

        let db = proof.into_memory_db::<HashingFor<Block>>();
        // If the proof is empty we should not compare against any root, but rather, expect that the pallet
        // will dot he job when looking for certain keys
        if !db.contains(&relay_parent_storage_root, EMPTY_PREFIX) && !proof_empty {
            return Err(ReadEntryErr::RootMismatch);
        }
        let trie_backend = TrieBackendBuilder::new(db, relay_parent_storage_root).build();

        Ok(Self { trie_backend })
    }

    /// Read an entry given by the key and try to decode it. If the value specified by the key according
    /// to the proof is empty, the `fallback` value will be returned.
    ///
    /// Returns `Err` in case the backend can't return the value under the specific key (likely due to
    /// a malformed proof), in case the decoding fails, or in case where the value is empty in the relay
    /// chain state and no fallback was provided.
    pub fn read_entry<T>(&self, key: &[u8], fallback: Option<T>) -> Result<T, ReadEntryErr>
    where
        T: Decode,
    {
        read_entry::<T, TrieBackend<MemoryDB<HashingFor<Block>>, HashingFor<Block>>, Block>(
            &self.trie_backend,
            key,
            fallback,
        )
    }

    /// Read an optional entry given by the key and try to decode it.
    ///
    /// Returns `Err` in case the backend can't return the value under the specific key (likely due to
    /// a malformed proof) or if the value couldn't be decoded.
    pub fn read_optional_entry<T>(&self, key: &[u8]) -> Result<Option<T>, ReadEntryErr>
    where
        T: Decode,
    {
        read_optional_entry::<T, TrieBackend<MemoryDB<HashingFor<Block>>, HashingFor<Block>>, Block>(
            &self.trie_backend,
            key,
        )
    }
}
