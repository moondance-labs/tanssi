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
use {
    crate::{AuthorNotingInfo, Call, Config, HeadData, Pallet, ParaId, RelayOrPara},
    core::any::{Any, TypeId},
    frame_benchmarking::{account, benchmarks},
    frame_support::{assert_ok, Hashable},
    frame_system::RawOrigin,
    parity_scale_codec::Encode,
    sp_std::{boxed::Box, vec},
    tp_traits::{AuthorNotingHook, GetContainerChainAuthor, GetCurrentContainerChains},
};

mod test_sproof {
    use sp_trie::StorageProof;

    /// Mocked proof because we cannot build proofs in a no-std environment.
    /// Only stores the number of parachains, and reads a previously encoded proof for that number
    /// of items from `crate::mock_proof`.
    #[derive(Clone, Default)]
    pub struct ParaHeaderSproofBuilder {
        pub num_items: usize,
    }

    impl ParaHeaderSproofBuilder {
        pub fn into_state_root_and_proof(
            self,
        ) -> (cumulus_primitives_core::relay_chain::Hash, StorageProof) {
            let encoded = crate::mock_proof::ENCODED_PROOFS[self.num_items];

            let root = hex::decode(encoded.1).unwrap();
            let proof = StorageProof::new(encoded.2.iter().map(|s| hex::decode(s).unwrap()));

            (<[u8; 32]>::try_from(root).unwrap().into(), proof)
        }
    }
}

benchmarks! {
    set_latest_author_data {
        // Depend on the number of parachains registered
        let x in 1..100;
        let mut container_chains = vec![];

        let data = if TypeId::of::<<<T as Config>::RelayOrPara as RelayOrPara>::InherentArg>() == TypeId::of::<tp_author_noting_inherent::OwnParachainInherentData>() {
            // RELAY MODE
            let mut sproof_builder = test_sproof::ParaHeaderSproofBuilder::default();

            // Must start at 0 in Relay mode (why?)
            for para_id in 0..x {
                let para_id = para_id.into();
                container_chains.push(para_id);
                // Mock assigned authors for this para id
                let author: T::AccountId = account("account id", 0u32, 0u32);
                // Use the max allowed value for num_each_container_chain
                let num_each_container_chain = 2;
                T::ContainerChainAuthor::set_authors_for_para_id(para_id, vec![author; num_each_container_chain]);
                sproof_builder.num_items += 1;

            }
            let (root, proof) = sproof_builder.into_state_root_and_proof();
            T::RelayOrPara::set_current_relay_chain_state(cumulus_pallet_parachain_system::RelayChainState {
                state_root: root,
                number: 0,
            });

            let arg = tp_author_noting_inherent::OwnParachainInherentData {
                relay_storage_proof: proof,
            };

            for para_id in 0..x {
                let para_id = para_id.into();
                let author: T::AccountId = account("account id", 0u32, 0u32);

                T::AuthorNotingHook::prepare_worst_case_for_bench(&author, 1, para_id);
            }

            *(Box::new(arg) as Box<dyn Any>).downcast().unwrap()
        } else if TypeId::of::<<<T as Config>::RelayOrPara as RelayOrPara>::InherentArg>() == TypeId::of::<()>() {
            // PARA MODE

            // Must start at 1 in Para mode (why?)
            for para_id in 1..x {
                let slot: crate::InherentType = 13u64.into();
                let header = sp_runtime::generic::Header::<crate::BlockNumber, crate::BlakeTwo256> {
                    parent_hash: Default::default(),
                    number: Default::default(),
                    state_root: Default::default(),
                    extrinsics_root: Default::default(),
                    digest: sp_runtime::generic::Digest {
                        logs: vec![crate::DigestItem::PreRuntime(crate::AURA_ENGINE_ID, slot.encode())],
                    },
                };
                let para_id: ParaId = para_id.into();
                let bytes = para_id.twox_64_concat();
                container_chains.push(para_id);

                // Mock assigned authors for this para id
                let author: T::AccountId = account("account id", 0u32, 0u32);
                // Use the max allowed value for num_each_container_chain
                let num_each_container_chain = 2;
                T::ContainerChainAuthor::set_authors_for_para_id(para_id, vec![author.clone(); num_each_container_chain]);
                // CONCAT
                let key = [crate::PARAS_HEADS_INDEX, bytes.as_slice()].concat();

                let head_data = HeadData(header.encode());
                frame_support::storage::unhashed::put(&key, &head_data);

                T::AuthorNotingHook::prepare_worst_case_for_bench(&author, 1, para_id);
            }
            let arg = ();
            *(Box::new(arg) as Box<dyn Any>).downcast().unwrap()
        } else {
            unreachable!("Unknown InherentArg")
        };

        T::ContainerChains::set_current_container_chains(&container_chains);

    }: _(RawOrigin::None, data)

    set_author {
        let para_id = 1000.into();
        let block_number = 1;
        let author: T::AccountId = account("account id", 0u32, 0u32);
    }: _(RawOrigin::Root, para_id, block_number, author, u64::from(block_number).into())

    kill_author_data {
        let para_id = 1000.into();
        let block_number = 1;
        let author: T::AccountId = account("account id", 0u32, 0u32);
        assert_ok!(Pallet::<T>::set_author(RawOrigin::Root.into(), para_id, block_number, author, u64::from(block_number).into()));
    }: _(RawOrigin::Root, para_id)

    on_container_authors_noted {
        // Depend on the number of parachains registered
        let x in 1..50;

        let mut infos = vec![];
        for i in 0..x {
            let para_id = (1000 + x).into();
            let block_number = 1;
            let author: T::AccountId = account("account id", 0u32, 0u32);
            T::AuthorNotingHook::prepare_worst_case_for_bench(&author, block_number, para_id);
            infos.push(AuthorNotingInfo { author, block_number, para_id });
        }
    }: { T::AuthorNotingHook::on_container_authors_noted(&infos)}

    impl_benchmark_test_suite!(
        Pallet,
        crate::mock::new_test_ext(),
        crate::mock::Test
    );
}
