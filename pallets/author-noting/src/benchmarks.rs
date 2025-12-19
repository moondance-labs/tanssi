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
//!
//! So in our case we use this weight hint to ensure that we can support the number of chains that
//! we aim to support. This ensures that when we add a new expensive hook, we get an error ahead of
//! time, instead of getting degraded block production as the number of registered chains grows.
//!
//! This benchmark is a bit complex because we allow each runtime to configure a set of "hooks"
//! that will run for every container chain that produces a block. We want to benchmark the hooks
//! separately, because we allow hooks to return its own weight.
//!
//! So the `set_latest_author_data` benchmark disables hooks using the
//! `set_should_run_author_noting_hooks` function. This means that the weight hint for the
//! `set_latest_author_data` inherent must be `set_latest_author_data + on_container_authors_noted`.
//! This is verified in test `inherent_weight_is_base_plus_hooks`.

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

/// Storage key used as a helper for benchmarking, to be able to skip `set_latest_author_data` hooks.
/// We set this to false in `set_latest_author_data` benchmark because that benchmarks only the base
/// weight, excluding the hooks.
const STORAGE_KEY_ENABLE_HOOKS: &[u8] = b"__mock_bench_run_author_noting_hooks";

pub fn should_run_author_noting_hooks() -> bool {
    // default: true
    frame_support::storage::unhashed::get(STORAGE_KEY_ENABLE_HOOKS).unwrap_or(true)
}
pub fn set_should_run_author_noting_hooks(x: bool) {
    frame_support::storage::unhashed::put(STORAGE_KEY_ENABLE_HOOKS, &x);
}

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn set_latest_author_data(x: Linear<1, 100>) -> Result<(), BenchmarkError> {
        // This benchmarks is `set_latest_author_data` with empty hooks
        let mut container_chains = vec![];

        // Register collators in staking pallet and initialize `ChainsToReward`.
        // TODO: we probably need to initialize `ChainsToReward` again after advancing blocks
        // Start at index 2000 because para_id < 2000 are system parachains
        for para_id in 0..x {
            let para_id = 2000u32 + para_id;
            let para_id: ParaId = para_id.into();
            let author: T::AccountId = account("account id", u32::from(para_id), 0u32);
            T::AuthorNotingHook::prepare_worst_case_for_bench(&author, 1, para_id);
        }

        // TODO: maybe we can remove all hooks code? Since we don't test hooks in this benchmark
        // Advance a few blocks and execute the pending staking operations
        T::AuthorNotingHook::bench_advance_block();
        T::AuthorNotingHook::bench_execute_pending();

        let data = if TypeId::of::<<<T as Config>::RelayOrPara as RelayOrPara>::InherentArg>()
            == TypeId::of::<tp_author_noting_inherent::OwnParachainInherentData>()
        {
            // PARA MODE
            let mut sproof_builder = test_sproof::ParaHeaderSproofBuilder::default();

            // Start at index 2000 because para_id < 2000 are system parachains
            for para_id in 0..x {
                let para_id = (2000u32 + para_id).into();

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

            // Start at index 2000 because para_id < 2000 are system parachains
            for para_id in 0..x {
                let para_id = 2000u32 + para_id;
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

        // This benchmark is without hooks
        set_should_run_author_noting_hooks(false);

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
            // Start at index 2000 because para_id < 2000 are system parachains
            let para_id = (2000 + i).into();
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
