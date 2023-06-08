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

#![cfg(feature = "runtime-benchmarks")]

//! Benchmarking
use tp_traits::GetContainerChainAuthor;
use tp_traits::GetCurrentContainerChains;
use crate::MockableRelaychainStateProvider;
use {
    crate::{Call, Config, Pallet},
    frame_benchmarking::{account, benchmarks},
    frame_system::RawOrigin,
    sp_std::vec,
};

mod test_sproof {
    mod well_known_keys {
        // They key to retrieve the para heads
        pub const PARAS_HEADS_INDEX: &[u8] =
    //&hex_literal::hex!["cd710b30bd2eab0352ddcc26417aa1941b3c252fcb29d88eff4f3de5de4476c3"];
    b"\xcd\x71\x0b\x30\xbd\x2e\xab\x03\x52\xdd\xcc\x26\x41\x7a\xa1\x94\x1b\x3c\x25\x2f\xcb\x29\xd8\x8e\xff\x4f\x3d\xe5\xde\x44\x76\xc3";
    }
    use crate::BlakeTwo256;
    use crate::HeadData;
    use crate::ParaId;
    use crate::Vec;
    use frame_support::Hashable;
    use sp_runtime::traits::HashFor;
    use sp_state_machine::Backend;
    use sp_std::vec;
    use sp_trie::MemoryDB;
    use sp_trie::StorageProof;

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
        ) -> (cumulus_primitives_core::relay_chain::Hash, StorageProof) {
            /*
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
                    let key = [well_known_keys::PARAS_HEADS_INDEX, para_key.as_slice()].concat();

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
            */
            let encoded = crate::mock_proof::ENCODED_PROOFS[self.items.len()];

            let root = hex::decode(encoded.1).unwrap();
            let proof = StorageProof::new(encoded.2.iter().map(|s| hex::decode(s).unwrap()));

            (<[u8; 32]>::try_from(root).unwrap().into(), proof)
        }
    }
}

benchmarks! {
    set_latest_author_data {
        // Depend on the number of parachains registered
        let x in 0..100;

        let mut sproof_builder = test_sproof::ParaHeaderSproofBuilder::default();
        let mut container_chains = vec![];

        for para_id in 0..x {
            use crate::benchmarks::test_sproof::HeaderAs;
            use crate::benchmarks::test_sproof::ParaHeaderSproofBuilderItem;
            let mut s = ParaHeaderSproofBuilderItem::default();
            s.para_id = para_id.into();
            container_chains.push(s.para_id);
            // Mock assigned authors for this para id
            let author: T::AccountId = account("account id", 0u32, 0u32);
            // Use the max allowed value for num_each_container_chain
            let num_each_container_chain = 2;
            T::ContainerChainAuthor::set_authors_for_para_id(s.para_id, vec![author; num_each_container_chain]);
            // TODO: this header can be arbitrarily large, because "digest.logs" is an unbounded vec
            let header = HeaderAs::NonEncoded(tp_core::Header {
                parent_hash: Default::default(),
                number: Default::default(),
                state_root: Default::default(),
                extrinsics_root: Default::default(),
                digest: sp_runtime::generic::Digest { logs: vec![] },
            });
            s.author_id = header;
            sproof_builder.items.push(s);
        }

        let (root, proof) = sproof_builder.into_state_root_and_proof();

        let mut data = tp_author_noting_inherent::OwnParachainInherentData {
            relay_storage_proof: proof,
        };

        T::ContainerChains::set_current_container_chains(&container_chains);
        T::RelayChainStateProvider::set_current_relay_chain_state(cumulus_pallet_parachain_system::RelayChainState {
            state_root: root,
            number: 0,
        });

    }: _(RawOrigin::None, data)

    set_author {
        let para_id = 1000.into();
        let author: T::AccountId = account("account id", 0u32, 0u32);
    }: _(RawOrigin::Root, para_id, author)

    impl_benchmark_test_suite!(
        Pallet,
        crate::mock::new_test_ext(),
        crate::mock::Test
    );
}
