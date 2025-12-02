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

use crate::{MinimumSelfDelegation, PooledStaking, RewardsCollatorCommission};
use frame_support::assert_ok;
use pallet_pooled_staking::{ActivePoolKind, PendingOperationKey, PendingOperationQuery};
use {
    crate::{tests::common::*, AuthorNoting, RewardsPortion},
    alloc::vec,
    cumulus_primitives_core::ParaId,
    parity_scale_codec::Encode,
    sp_consensus_aura::AURA_ENGINE_ID,
    sp_runtime::{generic::DigestItem, traits::BlakeTwo256},
    test_relay_sproof_builder::{HeaderAs, ParaHeaderSproofBuilder, ParaHeaderSproofBuilderItem},
    tp_traits::ContainerChainBlockInfo,
};

#[test]
fn test_reward_to_staking_candidate() {
    // Alice, Bob, Charlie are invulnerables
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000_000_000 * UNIT),
            (AccountId::from(BOB), 100_000_000_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000_000_000 * UNIT),
            (AccountId::from(DAVE), 100_000_000_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
        ])
        .with_empty_parachains(vec![1001, 1002])
        .build()
        .execute_with(|| {
            run_to_block(2);

            // Set DAVE session keys
            let dave_keys = get_authority_keys_from_seed(&AccountId::from(DAVE).to_string());

            assert_ok!(Session::set_keys(
                origin_of(DAVE.into()),
                crate::SessionKeys {
                    babe: dave_keys.babe.clone(),
                    grandpa: dave_keys.grandpa.clone(),
                    para_validator: dave_keys.para_validator.clone(),
                    para_assignment: dave_keys.para_assignment.clone(),
                    authority_discovery: dave_keys.authority_discovery.clone(),
                    beefy: dave_keys.beefy.clone(),
                    nimbus: dave_keys.nimbus.clone(),
                },
                vec![]
            ));

            // We make delegations to DAVE so that she is an elligible candidate.

            let stake = 10 * MinimumSelfDelegation::get();

            assert_ok!(PooledStaking::request_delegate(
                origin_of(DAVE.into()),
                DAVE.into(),
                ActivePoolKind::ManualRewards,
                stake,
            ));
            assert_ok!(PooledStaking::request_delegate(
                origin_of(BOB.into()),
                DAVE.into(),
                ActivePoolKind::AutoCompounding,
                stake,
            ));

            // wait few sessions for the request to be executable
            run_to_session(3u32);
            run_block();
            assert_ok!(PooledStaking::execute_pending_operations(
                origin_of(ALICE.into()),
                vec![
                    PendingOperationQuery {
                        delegator: DAVE.into(),
                        operation: PendingOperationKey::JoiningManualRewards {
                            candidate: DAVE.into(),
                            at: 0
                        }
                    },
                    PendingOperationQuery {
                        delegator: BOB.into(),
                        operation: PendingOperationKey::JoiningAutoCompounding {
                            candidate: DAVE.into(),
                            at: 0
                        }
                    }
                ]
            ));

            // wait for next session so that DAVE is elected
            run_to_session(4u32);
            run_block();

            let account: AccountId = DAVE.into();
            let balance_before = System::account(account.clone()).data.free;
            let summary = run_block();

            // Verify that all chains have collators
            let collator_assignment = TanssiCollatorAssignment::collator_container_chain();
            // 2 container chains total
            assert_eq!(collator_assignment.container_chains.len(), 2);
            // All container chains have collators
            assert!(collator_assignment
                .container_chains
                .values()
                .all(|cs| cs.len() == 2));

            let mut sproof = ParaHeaderSproofBuilder::default();
            let slot: u64 = 5;
            let other_para: ParaId = 1002u32.into();

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

            // We need to set the AuthorNoting's inherent for it to also run
            // InflationRewards::on_container_authors_noted and reward the collator.
            set_author_noting_inherent_data(sproof);

            // Check that DAVE authored the container chain block. If this assert fails, change slot number above.
            let container_block_author = AuthorNoting::latest_author(ParaId::from(1002u32))
                .unwrap()
                .author;
            assert_eq!(container_block_author, AccountId::from(DAVE));
            let balance_after = System::account(account).data.free;

            let all_rewards = RewardsPortion::get() * summary.inflation;
            // rewards are shared between the 2 paras
            let orchestrator_rewards = all_rewards / 2;
            let candidate_rewards = RewardsCollatorCommission::get() * orchestrator_rewards;

            assert_eq!(
                candidate_rewards,
                balance_after - balance_before,
                "dave should get the correct reward portion"
            );
        });
}

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
            (AccountId::from(DAVE), 100 * UNIT),
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

            // In dancelight there is no orchestrator chain, so instead of Charlie and Dave
            // we assign Alice and Bob.
            let assignment = TanssiCollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![ALICE.into(), BOB.into()]
            );
            // All container chains have collators
            assert!(assignment.container_chains.values().all(|cs| cs.len() == 2));

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
            // InflationRewards::on_container_authors_noted and reward the collator.
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

#[test]
fn test_reward_to_invulnerable_with_key_change() {
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
            run_to_block(2);

            run_to_session(2u32);
            run_block();

            // change key, this should be reflected 2 sessions afterward
            let alice_new_key = get_authority_keys_from_seed(&AccountId::from(EVE).to_string());

            assert_ok!(Session::set_keys(
                origin_of(ALICE.into()),
                crate::SessionKeys {
                    babe: alice_new_key.babe.clone(),
                    grandpa: alice_new_key.grandpa.clone(),
                    para_validator: alice_new_key.para_validator.clone(),
                    para_assignment: alice_new_key.para_assignment.clone(),
                    authority_discovery: alice_new_key.authority_discovery.clone(),
                    beefy: alice_new_key.beefy.clone(),
                    nimbus: alice_new_key.nimbus.clone(),
                },
                vec![]
            ));

            run_to_session(4u32);
            run_block();

            let account: AccountId = ALICE.into();
            let balance_before = System::account(account.clone()).data.free;

            let summary = run_block();

            // Verify that all chains have collators
            let collator_assignment = TanssiCollatorAssignment::collator_container_chain();
            // 2 container chains total
            assert_eq!(collator_assignment.container_chains.len(), 2);
            // All container chains have collators
            assert!(collator_assignment
                .container_chains
                .values()
                .all(|cs| cs.len() == 2));

            let mut sproof = ParaHeaderSproofBuilder::default();
            let slot: u64 = 6;
            let other_para: ParaId = 1001u32.into();

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

            // We need to set the AuthorNoting's inherent for it to also run
            // InflationRewards::on_container_authors_noted and reward the collator.
            set_author_noting_inherent_data(sproof);

            // Check that ALICE authored the container chain block. If this assert fails, change slot number above.
            let container_block_author = AuthorNoting::latest_author(ParaId::from(1001u32))
                .unwrap()
                .author;
            assert_eq!(container_block_author, AccountId::from(ALICE));

            let balance_after = System::account(account).data.free;

            let all_rewards = RewardsPortion::get() * summary.inflation;
            // rewards are shared between the 2 paras
            let orchestrator_rewards = all_rewards / 2;
            assert_eq!(
                orchestrator_rewards,
                balance_after - balance_before,
                "alice should get the correct reward portion"
            );
        });
}
