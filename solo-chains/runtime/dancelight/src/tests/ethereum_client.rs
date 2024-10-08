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
    crate::EthereumBeaconClient,
    frame_support::{assert_noop, assert_ok, error::BadOrigin},
    pallet_author_noting_runtime_api::runtime_decl_for_author_noting_api::AuthorNotingApi,
    parity_scale_codec::Encode,
    sp_consensus_aura::AURA_ENGINE_ID,
    sp_core::H256,
    sp_runtime::{generic::DigestItem, traits::BlakeTwo256},
    sp_std::vec,
    test_relay_sproof_builder::{HeaderAs, ParaHeaderSproofBuilder, ParaHeaderSproofBuilderItem},
    tp_traits::{ContainerChainBlockInfo, ParaId},
    snowbridge_pallet_ethereum_client::mock::*,
    snowbridge_pallet_ethereum_client::functions::*
};
#[test]
fn test_ethereum_force_checkpoint() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            let checkpoint =
                Box::new(snowbridge_pallet_ethereum_client::mock::load_checkpoint_update_fixture());
            assert_ok!(EthereumBeaconClient::force_checkpoint(
                root_origin(),
                checkpoint.clone()
            ));
            // assert checkpoint has updated storages
            assert_eq!(
                EthereumBeaconClient::initial_checkpoint_root(),
                checkpoint.header.hash_tree_root().unwrap()
            );
            // sync committee is correct
            let unwrap_keys: Vec<snowbridge_beacon_primitives::PublicKey> =
                snowbridge_pallet_ethereum_client::CurrentSyncCommittee::<Runtime>::get()
                    .pubkeys
                    .iter()
                    .map(|key| {
                        let unwrapped = key.as_bytes();
                        let pubkey: snowbridge_beacon_primitives::PublicKey = unwrapped.into();
                        pubkey
                    })
                    .collect();
            assert_eq!(
                unwrap_keys,
                checkpoint.current_sync_committee.pubkeys.to_vec()
            );
        })
}

#[test]
fn test_invalid_initial_checkpoint() {
    ExtBuilder::default()
            .with_balances(vec![
                // Alice gets 10k extra tokens for her mapping deposit
                (AccountId::from(ALICE), 210_000 * UNIT),
                (AccountId::from(BOB), 100_000 * UNIT),
                (AccountId::from(CHARLIE), 100_000 * UNIT),
                (AccountId::from(DAVE), 100_000 * UNIT),
            ])
            .build()
            .execute_with(|| {
                let mut checkpoint_invalid_sync_committee_proof = Box::new(snowbridge_pallet_ethereum_client::mock::load_checkpoint_update_fixture());

                let mut checkpoint_invalid_blocks_root_proof = checkpoint_invalid_sync_committee_proof.clone();

                let mut check_invalid_sync_committee = checkpoint_invalid_sync_committee_proof.clone();

	            checkpoint_invalid_sync_committee_proof.current_sync_committee_branch[0] = H256::default();
	            checkpoint_invalid_blocks_root_proof.block_roots_branch[0] = H256::default();
                let new_random_keys: Vec<snowbridge_beacon_primitives::PublicKey> = generate_ethereum_pub_keys(snowbridge_pallet_ethereum_client::config::SYNC_COMMITTEE_SIZE as u32).iter().map(|key| {
                    let public: snowbridge_beacon_primitives::PublicKey =   key.pk.as_bytes().into();
                    public
                }).collect();
                check_invalid_sync_committee.current_sync_committee.pubkeys = new_random_keys.try_into().expect("cannot convert keys");
                assert_noop!(
                    EthereumBeaconClient::force_checkpoint(root_origin(), checkpoint_invalid_sync_committee_proof),
                    snowbridge_pallet_ethereum_client::Error::<Runtime>::InvalidSyncCommitteeMerkleProof
                );

                assert_noop!(
                    EthereumBeaconClient::force_checkpoint(root_origin(), checkpoint_invalid_blocks_root_proof),
                    snowbridge_pallet_ethereum_client::Error::<Runtime>::InvalidBlockRootsRootMerkleProof
                );

                assert_noop!(
                    EthereumBeaconClient::force_checkpoint(root_origin(), check_invalid_sync_committee),
                    snowbridge_pallet_ethereum_client::Error::<Runtime>::InvalidSyncCommitteeMerkleProof
                );
	});
}

#[test]
fn test_submit_update_using_same_committee_same_checkpoint() {
    ExtBuilder::default()
            .with_balances(vec![
                // Alice gets 10k extra tokens for her mapping deposit
                (AccountId::from(ALICE), 210_000 * UNIT),
                (AccountId::from(BOB), 100_000 * UNIT),
                (AccountId::from(CHARLIE), 100_000 * UNIT),
                (AccountId::from(DAVE), 100_000 * UNIT),
            ])
            .build()
            .execute_with(|| {
	            let initial_checkpoint = Box::new(snowbridge_pallet_ethereum_client::mock::load_checkpoint_update_fixture());
                println!("INITIAL {:?}", initial_checkpoint);
	            let update_header = Box::new(snowbridge_pallet_ethereum_client::mock::load_finalized_header_update_fixture());
                println!("UPDATE {:?}", initial_checkpoint);

                let initial_period = compute_period(initial_checkpoint.header.slot);
                let update_period = compute_period(update_header.finalized_header.slot);
	            assert_eq!(initial_period, update_period);
                assert_ok!(EthereumBeaconClient::force_checkpoint(
                    root_origin(),
                    initial_checkpoint.clone()
                ));
                assert_ok!(EthereumBeaconClient::submit(origin_of(ALICE.into()), update_header.clone()));
                let block_root: H256 = update_header.finalized_header.hash_tree_root().unwrap();
                assert!(snowbridge_pallet_ethereum_client::FinalizedBeaconState::<Runtime>::contains_key(block_root));
	        });
}

#[test]
fn test_submit_update_with_sync_committee_in_current_period() {
    ExtBuilder::default()
            .with_balances(vec![
                // Alice gets 10k extra tokens for her mapping deposit
                (AccountId::from(ALICE), 210_000 * UNIT),
                (AccountId::from(BOB), 100_000 * UNIT),
                (AccountId::from(CHARLIE), 100_000 * UNIT),
                (AccountId::from(DAVE), 100_000 * UNIT),
            ])
            .build()
            .execute_with(|| {
	            let initial_checkpoint = Box::new(load_checkpoint_update_fixture());
	            let update_header = Box::new(load_sync_committee_update_fixture());
                let initial_period = compute_period(initial_checkpoint.header.slot);
                let update_period = compute_period(update_header.finalized_header.slot);
	            assert_eq!(initial_period, update_period);
                assert_ok!(EthereumBeaconClient::force_checkpoint(
                    root_origin(),
                    initial_checkpoint.clone()
                ));
                assert_ok!(EthereumBeaconClient::submit(origin_of(ALICE.into()), update_header.clone()));
                assert!(snowbridge_pallet_ethereum_client::NextSyncCommittee::<Runtime>::exists());
	        });
}