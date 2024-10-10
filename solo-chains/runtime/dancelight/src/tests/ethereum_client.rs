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
    frame_support::{assert_noop, assert_ok},
    snowbridge_pallet_ethereum_client::functions::*,
    snowbridge_pallet_ethereum_client::mock::*,
    sp_core::H256,
    sp_std::vec,
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
            // This tests submits the initial checkpoint that contains the initial sync committee
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
            // This tests submits a new header signed by the sync committee members within the same
            // period BUT without injecting the next sync committee
            let initial_checkpoint =
                Box::new(snowbridge_pallet_ethereum_client::mock::load_checkpoint_update_fixture());
            let update_header = Box::new(
                snowbridge_pallet_ethereum_client::mock::load_finalized_header_update_fixture(),
            );

            let initial_period = compute_period(initial_checkpoint.header.slot);
            let update_period = compute_period(update_header.finalized_header.slot);
            assert_eq!(initial_period, update_period);
            assert_ok!(EthereumBeaconClient::force_checkpoint(
                root_origin(),
                initial_checkpoint.clone()
            ));
            assert_ok!(EthereumBeaconClient::submit(
                origin_of(ALICE.into()),
                update_header.clone()
            ));
            let block_root: H256 = update_header.finalized_header.hash_tree_root().unwrap();
            assert!(snowbridge_pallet_ethereum_client::FinalizedBeaconState::<
                Runtime,
            >::contains_key(block_root));
        });
}

#[test]
fn test_submit_update_with_next_sync_committee_in_current_period() {
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
            // This tests submits a new header signed by the sync committee members within the same
            // period AND injecting the next sync committee
            let initial_checkpoint = Box::new(load_checkpoint_update_fixture());
            let update_header = Box::new(load_sync_committee_update_fixture());
            let initial_period = compute_period(initial_checkpoint.header.slot);
            let update_period = compute_period(update_header.finalized_header.slot);
            assert_eq!(initial_period, update_period);
            assert_ok!(EthereumBeaconClient::force_checkpoint(
                root_origin(),
                initial_checkpoint.clone()
            ));
            assert!(!snowbridge_pallet_ethereum_client::NextSyncCommittee::<
                Runtime,
            >::exists());
            assert_ok!(EthereumBeaconClient::submit(
                origin_of(ALICE.into()),
                update_header.clone()
            ));
            assert!(snowbridge_pallet_ethereum_client::NextSyncCommittee::<
                Runtime,
            >::exists());
        });
}

#[test]
fn test_submit_update_with_next_sync_committee_in_current_period_without_majority() {
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
                // This tests submits a new header signed by the sync committee members within the same
                // period BUT putting all signed bits to 0
	            let initial_checkpoint = Box::new(load_checkpoint_update_fixture());
	            let mut update_header = Box::new(load_sync_committee_update_fixture());
                update_header.sync_aggregate.sync_committee_bits = [0u8; snowbridge_pallet_ethereum_client::config::SYNC_COMMITTEE_BITS_SIZE];
                let initial_period = compute_period(initial_checkpoint.header.slot);
                let update_period = compute_period(update_header.finalized_header.slot);
	            assert_eq!(initial_period, update_period);
                assert_ok!(EthereumBeaconClient::force_checkpoint(
                    root_origin(),
                    initial_checkpoint.clone()
                ));
                assert_noop!(EthereumBeaconClient::submit(origin_of(ALICE.into()), update_header.clone()), snowbridge_pallet_ethereum_client::Error::<Runtime>::SyncCommitteeParticipantsNotSupermajority);
	        });
}

#[test]
fn test_submit_update_in_next_period() {
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
            // This test submits, assuming current period is n:
            // 1. A header in the next period n+1 but without having set the next sync committee, which fails
            // 2. Then submits a proper sync committee update for this period (n), indicating who the next sync committee will be
            // 3. Then submits an update on the next period (n+1), but without indicating who the next committee is going to be again (in period n+2), which fails
            // 4. Then submits a header on the next period(n+1), this time indicating who the next sync committee is going to be
            // 4. Then submits an update on the next period(n+1)
            let initial_checkpoint = Box::new(load_checkpoint_update_fixture());
            let sync_committee_update = Box::new(load_sync_committee_update_fixture());
            let next_sync_committee_update = Box::new(load_next_sync_committee_update_fixture());
            let next_update = Box::new(load_next_finalized_header_update_fixture());
            let initial_period = compute_period(initial_checkpoint.header.slot);

            assert_ok!(EthereumBeaconClient::force_checkpoint(
                root_origin(),
                initial_checkpoint.clone()
            ));

            // we need an update about the sync committee before we proceed
            assert_noop!(
                EthereumBeaconClient::submit(origin_of(ALICE.into()), next_update.clone()),
                snowbridge_pallet_ethereum_client::Error::<Runtime>::SkippedSyncCommitteePeriod
            );

            assert_ok!(EthereumBeaconClient::submit(
                origin_of(ALICE.into()),
                sync_committee_update.clone()
            ));

            // we need an update about the next sync committee
            assert_noop!(
                EthereumBeaconClient::submit(origin_of(ALICE.into()), next_update.clone()),
                snowbridge_pallet_ethereum_client::Error::<Runtime>::SyncCommitteeUpdateRequired
            );

            assert_ok!(EthereumBeaconClient::submit(
                origin_of(ALICE.into()),
                next_sync_committee_update.clone()
            ));

            // check we are now in period +1
            let latest_finalized_block_root =
                snowbridge_pallet_ethereum_client::LatestFinalizedBlockRoot::<Runtime>::get();
            let last_finalized_state = snowbridge_pallet_ethereum_client::FinalizedBeaconState::<
                Runtime,
            >::get(latest_finalized_block_root)
            .unwrap();
            let last_synced_period = compute_period(last_finalized_state.slot);
            assert_eq!(last_synced_period, initial_period + 1);

            assert_ok!(EthereumBeaconClient::submit(
                origin_of(ALICE.into()),
                next_update.clone()
            ));
        });
}
