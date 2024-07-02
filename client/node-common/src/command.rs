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
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>.

use {
    parity_scale_codec::Encode,
    sc_chain_spec::{construct_genesis_block, ChainSpec},
    sp_runtime::{
        traits::{Block as BlockT, Hash as HashT, Header as HeaderT},
        StateVersion,
    },
};

/// Generate the genesis block from a given ChainSpec.
pub fn generate_genesis_block<Block: BlockT>(
    chain_spec: &dyn ChainSpec,
    genesis_state_version: StateVersion,
) -> Result<Block, String> {
    let storage = chain_spec.build_storage()?;

    let child_roots = storage.children_default.iter().map(|(sk, child_content)| {
        let state_root = <<<Block as BlockT>::Header as HeaderT>::Hashing as HashT>::trie_root(
            child_content.data.clone().into_iter().collect(),
            genesis_state_version,
        );
        (sk.clone(), state_root.encode())
    });
    let state_root = <<<Block as BlockT>::Header as HeaderT>::Hashing as HashT>::trie_root(
        storage.top.clone().into_iter().chain(child_roots).collect(),
        genesis_state_version,
    );

    Ok(construct_genesis_block(state_root, genesis_state_version))
}
