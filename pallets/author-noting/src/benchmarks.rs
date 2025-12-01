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
//!
//! ## Note about inherents
//!
//! `set_latest_author_data` is an inherent that must be included in every block, or else the block
//! is invalid.
//!
//! Usually the purpose of an extrinsic weight is for the block author to be able to decide whether
//! to include it in the block or skip it. Taking into account the fee, and the block space that
//! will be used by that extrinsic.
//!
//! But in the case of inherents, there is no fee, and there is no possibility of not including it,
//! so the weight is useless.

use {
    crate::{AuthorNotingInfo, Call, Config, HeadData, Pallet, ParaId, RelayOrPara},
    alloc::{boxed::Box, vec},
    core::any::{Any, TypeId},
    frame_benchmarking::{account, v2::*},
    frame_support::{assert_ok, Hashable},
    frame_system::RawOrigin,
    parity_scale_codec::Encode,
    tp_traits::{
        AuthorNotingHook, ForSession, GetContainerChainAuthor, GetContainerChainsWithCollators,
    },
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

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn set_latest_author_data(x: Linear<1, 100>) -> Result<(), BenchmarkError> {
        // This benchmarks is `set_latest_author_data` with hooks
        let mut container_chains = vec![];

        // Register collators in staking pallet and initialize `ChainsToReward`.
        // TODO: we probably need to initialize `ChainsToReward` again after advancing blocks
        // Para ids always start at 1000
        for para_id in 0..x {
            let para_id = 1000u32 + para_id;
            let para_id: ParaId = para_id.into();
            let author: T::AccountId = account("account id", u32::from(para_id), 0u32);
            T::AuthorNotingHook::prepare_worst_case_for_bench(&author, 1, para_id);
        }

        // Advance a few blocks and execute the pending staking operations
        T::AuthorNotingHook::bench_advance_block();
        T::AuthorNotingHook::bench_execute_pending();

        let data = if TypeId::of::<<<T as Config>::RelayOrPara as RelayOrPara>::InherentArg>()
            == TypeId::of::<tp_author_noting_inherent::OwnParachainInherentData>()
        {
            // PARA MODE
            let mut sproof_builder = test_sproof::ParaHeaderSproofBuilder::default();

            // Must start at 1000 in Para mode (why?)
            for para_id in 0..x {
                let para_id = (1000u32 + para_id).into();

                let author: T::AccountId = account("account id", u32::from(para_id), 0u32);
                container_chains.push((para_id, vec![author.clone()]));

                // Mock assigned authors for this para id
                // Use the max allowed value for num_each_container_chain
                let num_each_container_chain = 2;
                T::ContainerChainAuthor::set_authors_for_para_id(
                    para_id,
                    vec![author; num_each_container_chain],
                );
                sproof_builder.num_items += 1;
            }
            let (root, proof) = sproof_builder.into_state_root_and_proof();
            T::RelayOrPara::set_current_relay_chain_state(
                cumulus_pallet_parachain_system::RelayChainState {
                    state_root: root,
                    number: 0,
                },
            );

            let arg = tp_author_noting_inherent::OwnParachainInherentData {
                relay_storage_proof: proof,
            };

            *(Box::new(arg) as Box<dyn Any>).downcast().unwrap()
        } else if TypeId::of::<<<T as Config>::RelayOrPara as RelayOrPara>::InherentArg>()
            == TypeId::of::<()>()
        {
            // RELAY MODE

            // Must start at 1000 in Relay mode (why?)
            for para_id in 0..x {
                let para_id = 1000u32 + para_id;
                let slot: crate::InherentType = 13u64.into();
                let header = sp_runtime::generic::Header::<crate::BlockNumber, crate::BlakeTwo256> {
                    parent_hash: Default::default(),
                    number: 1,
                    state_root: Default::default(),
                    extrinsics_root: Default::default(),
                    digest: sp_runtime::generic::Digest {
                        logs: vec![crate::DigestItem::PreRuntime(
                            crate::AURA_ENGINE_ID,
                            slot.encode(),
                        )],
                    },
                };
                let para_id: ParaId = para_id.into();
                let bytes = para_id.twox_64_concat();

                // Mock assigned authors for this para id
                let author: T::AccountId = account("account id", u32::from(para_id), 0u32);

                container_chains.push((para_id, vec![author.clone()]));

                // Use the max allowed value for num_each_container_chain
                let num_each_container_chain = 2;
                T::ContainerChainAuthor::set_authors_for_para_id(
                    para_id,
                    vec![author.clone(); num_each_container_chain],
                );
                // CONCAT
                let key = [crate::PARAS_HEADS_INDEX, bytes.as_slice()].concat();

                let head_data = HeadData(header.encode());
                frame_support::storage::unhashed::put(&key, &head_data);
            }
            let arg = ();
            *(Box::new(arg) as Box<dyn Any>).downcast().unwrap()
        } else {
            unreachable!("Unknown InherentArg")
        };

        T::ContainerChains::set_container_chains_with_collators(
            ForSession::Current,
            &container_chains,
        );

        #[extrinsic_call]
        _(RawOrigin::None, data);

        Ok(())
    }

    #[benchmark]
    fn set_author() -> Result<(), BenchmarkError> {
        let para_id = 1000.into();
        let block_number = 1;
        let author: T::AccountId = account("account id", 0u32, 0u32);

        #[extrinsic_call]
        _(
            RawOrigin::Root,
            para_id,
            block_number,
            author,
            u64::from(block_number).into(),
        );

        Ok(())
    }

    #[benchmark]
    fn kill_author_data() -> Result<(), BenchmarkError> {
        let para_id = 1000.into();
        let block_number = 1;
        let author: T::AccountId = account("account id", 0u32, 0u32);
        assert_ok!(Pallet::<T>::set_author(
            RawOrigin::Root.into(),
            para_id,
            block_number,
            author,
            u64::from(block_number).into()
        ));

        #[extrinsic_call]
        _(RawOrigin::Root, para_id);

        Ok(())
    }

    #[benchmark]
    fn on_container_authors_noted(x: Linear<1, 50>) -> Result<(), BenchmarkError> {
        let mut infos = vec![];
        for i in 0..x {
            let para_id = (1000 + i).into();
            let block_number = 1;
            let author: T::AccountId = account("account id", u32::from(para_id), 0u32);
            T::AuthorNotingHook::prepare_worst_case_for_bench(&author, block_number, para_id);
            infos.push(AuthorNotingInfo {
                author,
                block_number,
                para_id,
            });
        }

        T::AuthorNotingHook::bench_advance_block();
        T::AuthorNotingHook::bench_execute_pending();

        #[block]
        {
            T::AuthorNotingHook::on_container_authors_noted(&infos);
        }

        Ok(())
    }

    impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}
