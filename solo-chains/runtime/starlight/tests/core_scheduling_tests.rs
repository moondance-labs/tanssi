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

use frame_system::pallet_prelude::BlockNumberFor;
use sp_keystore::testing::MemoryKeystore;
use sp_std::collections::btree_map::BTreeMap;
use {
    crate::common::*,
    cumulus_primitives_core::relay_chain::vstaging::SchedulerParams,
    frame_support::assert_ok,
    sp_std::vec,
    starlight_runtime::{
        CollatorConfiguration, ContainerRegistrar, TanssiAuthorityMapping, TanssiInvulnerables,
    },
};
mod common;

const UNIT: Balance = 1_000_000_000_000_000_000;

#[test]
#[should_panic(expected = "CandidatesFilteredDuringExecution")]
// This test does not panic when producing the candidate, but when injecting it as backed
// the inclusion pallet will filter it as it does not have a core assigned
fn test_cannot_propose_a_block_without_availability() {
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
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 2,
            min_orchestrator_collators: 0,
            max_orchestrator_collators: 0,
            collators_per_container: 2,
            ..Default::default()
        })
        .with_relay_config(runtime_parachains::configuration::HostConfiguration::<
            BlockNumberFor<Runtime>,
        > {
            scheduler_params: SchedulerParams {
                num_cores: 2,
                // A very high number to avoid group rotation in tests
                // Otherwise we get a 1 by default, which changes groups every block
                group_rotation_frequency: 10000000,
                ..Default::default()
            },
            async_backing_params: AsyncBackingParams {
                allowed_ancestry_len: 1,
                max_candidate_depth: 0,
            },
            minimum_backing_votes: 1,
            max_head_data_size: 5,
            ..Default::default()
        })
        .with_keystore(Arc::new(MemoryKeystore::new()))
        .build()
        .execute_with(|| {
            run_to_block(2);
            let cores_with_backed: BTreeMap<_, _> =
                vec![(0u32, Session::validators().len() as u32)]
                    .into_iter()
                    .collect();

            let inherent_data = ParasInherentTestBuilder::<Runtime>::new()
                .set_backed_and_concluding_paras(cores_with_backed)
                .build();
            set_new_inherent_data(inherent_data);
            run_block();
        })
}

use cumulus_primitives_core::relay_chain::ValidatorId;
use sc_keystore::LocalKeystore;
use sp_keystore::{Keystore, KeystorePtr};
use std::sync::Arc;
fn validator_pubkeys(val_ids: &[Sr25519Keyring]) -> Vec<ValidatorId> {
    val_ids.iter().map(|v| v.public().into()).collect()
}
use cumulus_primitives_core::relay_chain::AsyncBackingParams;
use keyring::Sr25519Keyring;
use primitives::runtime_api::runtime_decl_for_parachain_host::ParachainHostV11;
use starlight_runtime::RuntimeEvent;

#[test]
fn test_should_have_availability_for_registered_parachain() {
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
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 2,
            min_orchestrator_collators: 0,
            max_orchestrator_collators: 0,
            collators_per_container: 2,
            ..Default::default()
        })
        .with_relay_config(runtime_parachains::configuration::HostConfiguration::<
            BlockNumberFor<Runtime>,
        > {
            scheduler_params: SchedulerParams {
                num_cores: 2,
                // A very high number to avoid group rotation in tests
                // Otherwise we get a 1 by default, which changes groups every block
                group_rotation_frequency: 10000000,
                ..Default::default()
            },
            async_backing_params: AsyncBackingParams {
                allowed_ancestry_len: 1,
                max_candidate_depth: 0,
            },
            minimum_backing_votes: 1,
            max_head_data_size: 5,
            ..Default::default()
        })
        .with_para_ids(vec![(1000, empty_genesis_data(), u32::MAX, u32::MAX).into()])
        .with_keystore(Arc::new(MemoryKeystore::new()))
        .build()
        .execute_with(|| {
            run_to_block(2);
            let validators_shuffled = runtime_parachains::session_info::Sessions::<Runtime>::get(0)
                .unwrap()
                .validators
                .clone();

            let cores_with_backed: BTreeMap<_, _> =
                vec![(1000u32, Session::validators().len() as u32)]
                    .into_iter()
                    .collect();
            let alice_keys =
                get_authority_keys_from_seed(&AccountId::from(ALICE).to_string(), None);
            let bob_keys = get_authority_keys_from_seed(&AccountId::from(BOB).to_string(), None);

            // This should make sure we have a core-assigned
            assert!(
                authorities_for_container(1000u32.into())
                    == Some(vec![alice_keys.nimbus.clone(), bob_keys.nimbus.clone()])
            );
            let inherent_data = ParasInherentTestBuilder::<Runtime>::new()
                .set_backed_and_concluding_paras(cores_with_backed)
                .build();
            set_new_inherent_data(inherent_data);
            let availability_before = Runtime::candidates_pending_availability(1000u32.into());
            // Before there is no availability as we never injected a candidate
            assert_eq!(availability_before.len(), 0);
            run_block();
            let availability_after = Runtime::candidates_pending_availability(1000u32.into());
            // After the availability length is 1 as we have one candidate succesfully backed
            assert_eq!(availability_after.len(), 1);
        })
}
