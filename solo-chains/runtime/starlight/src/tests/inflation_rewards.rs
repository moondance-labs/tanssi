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
    crate::{AuthorNoting, RewardsPortion},
    cumulus_primitives_core::ParaId,
    parity_scale_codec::Encode,
    sp_consensus_aura::AURA_ENGINE_ID,
    sp_runtime::generic::DigestItem,
    sp_runtime::traits::BlakeTwo256,
    sp_std::vec,
    test_relay_sproof_builder::{HeaderAs, ParaHeaderSproofBuilder, ParaHeaderSproofBuilderItem},
    tp_traits::ContainerChainBlockInfo,
};

#[test]
fn test_reward_to_invulnerable() {
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
        ])
        .with_empty_parachains(vec![1001, 1002])
        .build()
        .execute_with(|| {
            // Let's get the inflation of the block.
            let summary = run_block();

            // Calculate Bob's rewards.
            let all_rewards = RewardsPortion::get() * summary.inflation;
            let bob_rewards = all_rewards / 2;

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

            // Build the proof needed to call AuthorNoting's inherent.
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

            let account: AccountId = BOB.into();
            let balance_before = System::account(account.clone()).data.free;

            // We need to set the AuthorNoting's inherent for it to also run
            // InflationRewards::on_container_author_noted and reward the collator.
            set_author_noting_inherent_data(sproof);

            assert_eq!(
                AuthorNoting::latest_author(other_para),
                Some(ContainerChainBlockInfo {
                    block_number: 1,
                    author: AccountId::from(BOB),
                    latest_slot_number: 2.into(),
                })
            );

            let balance_after = System::account(account).data.free;

            assert_eq!(
                bob_rewards,
                balance_after - balance_before,
                "bob should get the correct reward portion"
            );
        });
}
