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

#![allow(clippy::await_holding_lock)]

// This tests have been greatly influenced by
// https://github.com/paritytech/substrate/blob/master/client/consensus/aura/src/lib.rs#L832
// Most of the items hereby added are intended to make it work with our current consensus mechanism
use {
    crate::{
        collators::{tanssi_claim_slot, ClaimMode, Collator, Params as CollatorParams},
        mocks::*,
        OrchestratorAuraWorkerAuxData, SlotFrequency,
    },
    cumulus_client_collator::service::CollatorService,
    cumulus_client_consensus_proposer::Proposer as ConsensusProposer,
    nimbus_primitives::{NimbusId, NimbusPair, NIMBUS_KEY_ID},
    parity_scale_codec::Encode,
    parking_lot::Mutex,
    sc_client_api::HeaderBackend,
    sc_keystore::LocalKeystore,
    sc_network_test::{Block as TestBlock, Header as TestHeader, *},
    sp_consensus_aura::{inherents::InherentDataProvider, SlotDuration, AURA_ENGINE_ID},
    sp_consensus_slots::Slot,
    sp_keyring::sr25519::Keyring,
    sp_keystore::{Keystore, KeystorePtr},
    sp_runtime::{Digest, DigestItem},
    sp_timestamp::Timestamp,
    std::{sync::Arc, time::Duration},
};

// Checks node slot claim. Again for different slots, different authorities
// should be able to claim
#[tokio::test]
async fn current_node_authority_should_claim_slot() {
    let mut authorities: Vec<NimbusId> = vec![
        Keyring::Alice.public().into(),
        Keyring::Bob.public().into(),
        Keyring::Charlie.public().into(),
    ];

    let keystore_path = tempfile::tempdir().expect("Creates keystore path");
    let keystore = LocalKeystore::open(keystore_path.path(), None).expect("Creates keystore.");

    let public = keystore
        .sr25519_generate_new(NIMBUS_KEY_ID, None)
        .expect("Key should be created");
    authorities.push(public.into());

    let keystore_ptr: KeystorePtr = keystore.into();
    let mut claimed_slots = vec![];

    for slot in 0..8 {
        let dummy_head = TestHeader {
            parent_hash: Default::default(),
            number: Default::default(),
            state_root: Default::default(),
            extrinsics_root: Default::default(),
            digest: Default::default(),
        };
        let aux_data = OrchestratorAuraWorkerAuxData {
            authorities: authorities.clone(),
            slot_freq: None,
        };
        let claim = tanssi_claim_slot::<NimbusPair, TestBlock>(
            aux_data,
            &dummy_head,
            slot.into(),
            ClaimMode::NormalAuthoring,
            &keystore_ptr,
        );

        if claim.is_some() {
            claimed_slots.push(slot);
        }
    }

    assert_eq!(claimed_slots, vec![3, 7]);
}

#[tokio::test]
async fn claim_slot_respects_min_slot_freq() {
    // There is only 1 authority, but it can only claim every 4 slots
    let mut authorities: Vec<NimbusId> = vec![];
    let min_slot_freq = 4u32;

    let keystore_path = tempfile::tempdir().expect("Creates keystore path");
    let keystore = LocalKeystore::open(keystore_path.path(), None).expect("Creates keystore.");

    let public = keystore
        .sr25519_generate_new(NIMBUS_KEY_ID, None)
        .expect("Key should be created");
    authorities.push(public.into());

    let keystore_ptr: KeystorePtr = keystore.into();

    let mut claimed_slots = vec![];

    for slot in 0..10 {
        let parent_slot: u64 = claimed_slots.last().copied().unwrap_or_default();
        let parent_slot: Slot = parent_slot.into();
        let pre_digest = Digest {
            logs: vec![
                DigestItem::PreRuntime(AURA_ENGINE_ID, parent_slot.encode()),
                //DigestItem::PreRuntime(NIMBUS_ENGINE_ID, authority.encode()),
            ],
        };
        let head = TestHeader {
            parent_hash: Default::default(),
            // If we use number=0 aura ignores the digest
            number: claimed_slots.len() as u64,
            state_root: Default::default(),
            extrinsics_root: Default::default(),
            digest: pre_digest,
        };
        let aux_data = OrchestratorAuraWorkerAuxData {
            authorities: authorities.clone(),
            slot_freq: Some(SlotFrequency {
                min: min_slot_freq,
                max: 0u32,
            }),
        };
        let claim = tanssi_claim_slot::<NimbusPair, TestBlock>(
            aux_data,
            &head,
            slot.into(),
            ClaimMode::NormalAuthoring,
            &keystore_ptr,
        );

        if claim.is_some() {
            claimed_slots.push(slot);
        }
    }

    assert_eq!(claimed_slots, vec![0, 4, 8]);
}

#[tokio::test]
async fn collate_returns_correct_block() {
    let net = AuraTestNet::new(4);
    sp_tracing::try_init_simple();

    let keystore_path = tempfile::tempdir().expect("Creates keystore path");
    let keystore = LocalKeystore::open(keystore_path.path(), None).expect("Creates keystore.");
    let alice_public = keystore
        .sr25519_generate_new(NIMBUS_KEY_ID, Some(&Keyring::Alice.to_seed()))
        .expect("Key should be created");

    // Copy of the keystore needed for tanssi_claim_slot()
    let keystore_copy = LocalKeystore::open(keystore_path.path(), None).expect("Copies keystore.");
    keystore_copy
        .sr25519_generate_new(NIMBUS_KEY_ID, Some(&Keyring::Alice.to_seed()))
        .expect("Key should be copied");

    let net = Arc::new(Mutex::new(net));

    let mut net = net.lock();
    let peer = net.peer(3);
    let client = peer.client().as_client();
    let environ = DummyFactory(client.clone());
    let spawner = DummySpawner;
    let relay_client = RelayChain {
        client: client.clone(),
        block_import_iterations: 1u32,
    };

    // Build the collator
    let mut collator = {
        let params = CollatorParams {
            create_inherent_data_providers: |_, _| async {
                // We will always trigger an is_allowed_to_buy_Core
                let slot = InherentDataProvider::from_timestamp_and_slot_duration(
                    Timestamp::current(),
                    SlotDuration::from_millis(SLOT_DURATION_MS),
                );

                Ok((slot,))
            },
            block_import: client.clone(),
            relay_client: relay_client.clone(),
            keystore: keystore.into(),
            para_id: 1000.into(),
            proposer: ConsensusProposer::new(environ.clone()),
            collator_service: CollatorService::new(
                client.clone(),
                Arc::new(spawner),
                Arc::new(move |_, _| {}),
                Arc::new(environ),
            ),
        };

        Collator::<Block, NimbusPair, _, _, _, _, _>::new(params)
    };

    let head = client.expect_header(client.info().genesis_hash).unwrap();

    // First we create inherent data
    let (parachain_inherent_data, other_inherent_data) = collator
        .create_inherent_data(
            Default::default(),
            &Default::default(),
            head.clone().hash(),
            None,
        )
        .await
        .unwrap();

    // Params for tanssi_claim_slot()
    let slot = InherentDataProvider::from_timestamp_and_slot_duration(
        Timestamp::current(),
        SlotDuration::from_millis(SLOT_DURATION_MS),
    );
    let keystore_ptr: KeystorePtr = keystore_copy.into();

    let mut claim = tanssi_claim_slot::<NimbusPair, TestBlock>(
        OrchestratorAuraWorkerAuxData {
            authorities: vec![alice_public.into()],
            slot_freq: None,
        },
        &head,
        *slot,
        ClaimMode::NormalAuthoring,
        &keystore_ptr,
    )
    .unwrap();

    // At the end we call collate() function
    let res = collator
        .collate(
            &head,
            &mut claim,
            None,
            (parachain_inherent_data, other_inherent_data),
            Duration::from_millis(500),
            3_500_000usize,
        )
        .await
        .unwrap()
        .unwrap()
        .1;

    // The returned block should be imported and we should be able to get its header by now.
    assert!(client.header(res.header().hash()).unwrap().is_some());
}

// Tests authorities are correctly returned and eligibility is correctly calculated
// thanks to the mocked runtime-apis
#[tokio::test]
async fn authorities_runtime_api_tests() {
    let net = AuraTestNet::new(4);
    let net = Arc::new(Mutex::new(net));

    let mut net = net.lock();
    let peer = net.peer(3);
    let client = peer.client().as_client();
    let environ = DummyFactory(client);

    let default_hash = Default::default();

    let authorities = crate::authorities::<_, _, nimbus_primitives::NimbusPair>(
        &environ,
        &default_hash,
        1000u32.into(),
    );

    assert_eq!(authorities, Some(vec![Keyring::Alice.public().into()]));
}

#[tokio::test]
async fn collate_lookahead_returns_correct_block() {
    let (fut, _exit_notification_receiver, client, _orchestrator_tx_pool, _cancellation_token) =
        CollatorLookaheadTestBuilder::default()
            .with_block_import_iterations(1)
            .with_para_id(1000u32.into())
            .with_core_scheduled_for_para(1000u32.into())
            .build_and_spawn();

    fut.await;

    // We only had one notification import, but n_built goes from 0..2. Since we are not mocking the async backing params, then
    // this is going to create 2 blocks on to of the latest
    assert_eq!(client.chain_info().best_number, 2);
}

#[tokio::test]
async fn collate_lookahead_for_2_relay_imports_goes_till_num_4() {
    let (fut, _exit_notification_receiver, client, _orchestrator_tx_pool, _cancellation_token) =
        CollatorLookaheadTestBuilder::default()
            .with_block_import_iterations(2)
            .with_para_id(1000u32.into())
            .with_core_scheduled_for_para(1000u32.into())
            .build_and_spawn();

    fut.await;

    // We only had one notification import, but n_built goes from 0..2. Since we are not mocking the async backing params, then
    // this is going to create 2 blocks on to of the latest
    assert_eq!(client.chain_info().best_number, 4);
}

#[tokio::test]
async fn collate_lookahead_returns_correct_block_for_parathreads() {
    let (fut, _exit_notification_receiver, client, _orchestrator_tx_pool, _cancellation_token) =
        CollatorLookaheadTestBuilder::default()
            .with_block_import_iterations(1)
            .with_para_id(1000u32.into())
            .with_min_slot_freq(4)
            .with_core_scheduled_for_para(1000u32.into())
            .build_and_spawn();
    fut.await;
    // This time we have the slot frequency limit, so we should only create one
    assert_eq!(client.chain_info().best_number, 1);
}

#[tokio::test]
async fn collate_lookahead_should_buy_core_for_parathread() {
    let (fut, _exit_notification_receiver, client, orchestrator_tx_pool, _cancellation_token) =
        CollatorLookaheadTestBuilder::default()
            .with_block_import_iterations(1)
            .with_para_id(1000u32.into())
            .with_min_slot_freq(4)
            .build_and_spawn();
    fut.await;
    // the block should not have been created, but the tx should be in the pool
    assert_eq!(client.chain_info().best_number, 0);
    assert_eq!(orchestrator_tx_pool.status().ready, 1);
}

#[tokio::test]
async fn collate_lookahead_for_1000_relay_imports_but_with_cancellation_token_stops_before() {
    let (fut, _exit_notification_receiver, client, _orchestrator_tx_pool, cancellation_token) =
        CollatorLookaheadTestBuilder::default()
            .with_block_import_iterations(1000)
            .with_para_id(1000u32.into())
            .build_and_spawn();

    cancellation_token.cancel();
    fut.await;
    // We have 1000 import noficiations but the cancellation token should kick in (hopefully) before creating block 50
    // But this is not deterministic
    // The probability of this assert failing  is 0.5^25
    assert!(client.chain_info().best_number < 50);
}
