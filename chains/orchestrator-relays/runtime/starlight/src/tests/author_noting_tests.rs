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

#![cfg(test)]

use {
    crate::tests::common::*,
    crate::{weights, BlockWeights},
    alloc::vec,
    frame_support::dispatch::{CallableCallFor, GetDispatchInfo},
    frame_support::{assert_noop, assert_ok, error::BadOrigin},
    pallet_author_noting::WeightInfo as _,
    pallet_author_noting_runtime_api::runtime_decl_for_author_noting_api::AuthorNotingApi,
    parity_scale_codec::Encode,
    sp_arithmetic::Perbill,
    sp_consensus_aura::AURA_ENGINE_ID,
    sp_runtime::{generic::DigestItem, traits::BlakeTwo256},
    test_relay_sproof_builder::{HeaderAs, ParaHeaderSproofBuilder, ParaHeaderSproofBuilderItem},
    tp_traits::{ContainerChainBlockInfo, ParaId},
};

#[test]
fn test_author_noting_not_self_para() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .with_empty_parachains(vec![1001, 1002])
        .build()
        .execute_with(|| {
            let mut sproof = ParaHeaderSproofBuilder::default();
            let slot: u64 = 5;
            let other_para: ParaId = 1001u32.into();

            // In starlight there is no orchestrator chain, so instead of Charlie and Dave
            // we assign Alice and Bob.
            let assignment = TanssiCollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![ALICE.into(), BOB.into()]
            );

            let s = ParaHeaderSproofBuilderItem {
                para_id: other_para,
                author_id: HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                    parent_hash: Default::default(),
                    number: 1,
                    state_root: Default::default(),
                    extrinsics_root: Default::default(),
                    digest: sp_runtime::generic::Digest {
                        logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                    },
                }),
            };
            sproof.items.push(s);

            set_author_noting_inherent_data(sproof);

            assert_eq!(
                AuthorNoting::latest_author(other_para),
                Some(ContainerChainBlockInfo {
                    block_number: 1,
                    author: AccountId::from(BOB),
                    latest_slot_number: 1.into(),
                })
            );
        });
}

#[test]
fn test_author_noting_set_author_and_kill_author_data() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .with_empty_parachains(vec![1001, 1002])
        .build()
        .execute_with(|| {
            let other_para: ParaId = 1001u32.into();

            assert_ok!(AuthorNoting::set_author(
                root_origin(),
                other_para,
                1,
                AccountId::from(BOB),
                1.into()
            ));

            assert_eq!(
                AuthorNoting::latest_author(other_para),
                Some(ContainerChainBlockInfo {
                    block_number: 1,
                    author: AccountId::from(BOB),
                    latest_slot_number: 1.into(),
                })
            );

            assert_ok!(AuthorNoting::kill_author_data(root_origin(), other_para));

            assert_eq!(AuthorNoting::latest_author(other_para), None);
        });
}

#[test]
fn test_author_noting_set_author_and_kill_author_data_bad_origin() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .with_empty_parachains(vec![1001, 1002])
        .build()
        .execute_with(|| {
            let other_para: ParaId = 1001u32.into();

            assert_noop!(
                AuthorNoting::set_author(
                    origin_of(ALICE.into()),
                    other_para,
                    1,
                    AccountId::from(BOB),
                    1.into()
                ),
                BadOrigin
            );

            assert_noop!(
                AuthorNoting::kill_author_data(origin_of(ALICE.into()), other_para),
                BadOrigin
            );
        });
}

#[test]
fn max_num_chains_for_author_noting_inherent() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .with_empty_parachains(vec![1001, 1002])
        .build()
        .execute_with(|| {
            // TODO: where to find weight reserved for inherents? I'm assuming 20% here
            let inherents_weight_limit = Perbill::from_percent(20) * BlockWeights::get().max_block;
            // We have enough weight to support MaxContainerChains
            let author_noting_weight = |x| {
                <weights::pallet_author_noting::SubstrateWeight<Runtime>>::set_latest_author_data(x)
                    .saturating_add(<weights::pallet_author_noting::SubstrateWeight<Runtime>>::on_container_authors_noted(x))
            };

            let worst_case_author_noting_weight = author_noting_weight(
                <Runtime as pallet_author_noting::Config>::MaxContainerChains::get(),
            );
            // First, assert that real weight hint is calculated as we expect (base + hooks)
            let call = CallableCallFor::<AuthorNoting, Runtime>::set_latest_author_data { data: () };
            assert_eq!(call.get_dispatch_info().call_weight, worst_case_author_noting_weight);
            // Assert that the weight with current max number of chains is lower than inherents_weight_limit
            assert!(worst_case_author_noting_weight.all_lte(inherents_weight_limit));

            // We also support double of that, as a safety margin
            // If this assert fails we can just remove it, or we can decrease MaxLengthParaIds a bit
            let worst_case_author_noting_weight = author_noting_weight(
                2 * <Runtime as pallet_author_noting::Config>::MaxContainerChains::get(),
            );
            assert!(worst_case_author_noting_weight.all_lte(inherents_weight_limit));
        });
}

#[test]
fn test_author_noting_runtime_api() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .with_empty_parachains(vec![1001, 1002])
        .build()
        .execute_with(|| {
            let mut sproof = ParaHeaderSproofBuilder::default();
            let slot: u64 = 5;
            let other_para: ParaId = 1001u32.into();

            // In starlight there is no orchestrator chain, so instead of Charlie and Dave
            // we assign Alice and Bob.
            let assignment = TanssiCollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![ALICE.into(), BOB.into()]
            );

            let s = ParaHeaderSproofBuilderItem {
                para_id: other_para,
                author_id: HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                    parent_hash: Default::default(),
                    number: 1,
                    state_root: Default::default(),
                    extrinsics_root: Default::default(),
                    digest: sp_runtime::generic::Digest {
                        logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                    },
                }),
            };
            sproof.items.push(s);

            set_author_noting_inherent_data(sproof);

            assert_eq!(
                AuthorNoting::latest_author(other_para),
                Some(ContainerChainBlockInfo {
                    block_number: 1,
                    author: AccountId::from(BOB),
                    latest_slot_number: 1.into(),
                })
            );

            assert_eq!(
                Runtime::latest_author(other_para),
                Some(AccountId::from(BOB))
            );
            assert_eq!(Runtime::latest_block_number(other_para), Some(1));
        });
}
