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
    crate::{
        tests::common::*, ContainerRegistrar, OnDemandAssignmentProvider, Paras, Registrar, Session,
    },
    cumulus_primitives_core::relay_chain::{
        AsyncBackingParams, CoreIndex, HeadData, SchedulerParams,
    },
    dancelight_runtime_constants::time::EpochDurationInBlocks,
    frame_support::assert_ok,
    frame_system::pallet_prelude::BlockNumberFor,
    primitives::{
        node_features::FeatureIndex, runtime_api::runtime_decl_for_parachain_host::ParachainHostV11,
    },
    runtime_parachains::scheduler::common::Assignment,
    sp_core::{Decode, Encode},
    sp_keystore::testing::MemoryKeystore,
    sp_std::{collections::btree_map::BTreeMap, vec},
    std::sync::Arc,
    tp_traits::SlotFrequency,
};

#[test]
#[should_panic(expected = "InherentDataFilteredDuringExecution")]
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
        .with_inherent_data_enabled()
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
            node_features: bitvec::vec::BitVec::from_element(
                (1u8 << (FeatureIndex::ElasticScalingMVP as usize))
                    | (1u8 << (FeatureIndex::CandidateReceiptV2 as usize)),
            ),
            ..Default::default()
        })
        .with_keystore(Arc::new(MemoryKeystore::new()))
        .build()
        .execute_with(|| {
            run_to_block(2);
            // The claim queue should be always empty, as there is no para
            let assignments: BTreeMap<_, _> = claim_queue_assignments().collect();
            assert!(assignments.is_empty());

            let cores_with_backed: BTreeMap<_, _> =
                vec![(1000u32, Session::validators().len() as u32)]
                    .into_iter()
                    .collect();

            let inherent_data = ParasInherentTestBuilder::<Runtime>::new()
                .set_backed_and_concluding_paras(cores_with_backed)
                .build();
            set_new_inherent_data(inherent_data);
            run_block();
        })
}

#[test]
#[should_panic(expected = "InherentDataFilteredDuringExecution")]
// This test does not panic when producing the candidate, but when injecting it as backed
// the inclusion pallet will filter it as it does not have a core assigned
fn test_cannot_produce_block_even_if_buying_on_demand_if_no_collators() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 2,
            min_orchestrator_collators: 0,
            max_orchestrator_collators: 0,
            collators_per_container: 2,
            ..Default::default()
        })
        .with_para_ids(vec![ParaRegistrationParams {
            para_id: 1000,
            genesis_data: empty_genesis_data(),
            block_production_credits: u32::MAX,
            collator_assignment_credits: u32::MAX,
            parathread_params: None,
        }])
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
            node_features: bitvec::vec::BitVec::from_element(
                (1u8 << (FeatureIndex::ElasticScalingMVP as usize))
                    | (1u8 << (FeatureIndex::CandidateReceiptV2 as usize)),
            ),
            ..Default::default()
        })
        .with_keystore(Arc::new(MemoryKeystore::new()))
        .with_inherent_data_enabled()
        .build()
        .execute_with(|| {
            sp_tracing::try_init_simple();
            run_to_block(2);

            // Here para-id is registered but does not have collators, but we can indeed buy a on-demand core
            // however we should not be able to produce for it
            assert_ok!(OnDemandAssignmentProvider::place_order_allow_death(
                origin_of(ALICE.into()),
                100 * UNIT,
                1000u32.into()
            ));
            run_block();

            // We even have affinity with respect to what on-demand thinks
            let key = affinity_key_for_parathread(1000u32);
            let affinity: Option<CoreAffinityCount> =
                frame_support::storage::unhashed::get(key.as_ref());

            assert_eq!(
                affinity,
                Some(CoreAffinityCount {
                    core_index: CoreIndex(0),
                    count: 1
                })
            );

            // However the claim queue will never give say we have availability
            // This can be shown by trying to produce a block
            let assignments: BTreeMap<_, _> = claim_queue_assignments().collect();
            assert!(assignments.is_empty());

            // Now we try to create the block
            // the previous empty call should have advanced the claim queue
            // if there was availability, the claim queue should have indicated it
            // in this case no collators, so does not adavance
            let cores_with_backed: BTreeMap<_, _> =
                vec![(1000u32, Session::validators().len() as u32)]
                    .into_iter()
                    .collect();

            let inherent_data = ParasInherentTestBuilder::<Runtime>::new()
                .set_backed_and_concluding_paras(cores_with_backed)
                .build();
            set_new_inherent_data(inherent_data);
            // This should filter out, as we dont have any collators assigned to it
            run_block();
        })
}

#[test]
#[should_panic(expected = "InherentDataFilteredDuringExecution")]
// This test does not panic when producing the candidate, but when injecting it as backed
// the inclusion pallet will filter it as it does not have a core assigned
fn test_cannot_use_elastic_scaling_if_not_enabled() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 2,
            min_orchestrator_collators: 0,
            max_orchestrator_collators: 0,
            collators_per_container: 2,
            ..Default::default()
        })
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .with_para_ids(vec![ParaRegistrationParams {
            para_id: 1000,
            genesis_data: empty_genesis_data(),
            block_production_credits: u32::MAX,
            collator_assignment_credits: u32::MAX,
            parathread_params: None,
        }])
        .with_inherent_data_enabled()
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
            sp_tracing::try_init_simple();
            run_to_block(2);
            // Here para-id is registered and has collators, but we can indeed buy a on-demand core additional
            // however we should not be able to produce for it without elastic scaling enabled
            assert_ok!(OnDemandAssignmentProvider::place_order_allow_death(
                origin_of(ALICE.into()),
                100 * UNIT,
                1000u32.into()
            ));
            run_block();

            // The claim queue this time allows us to produce blocks
            // We have a parachain core and a bought core through on demand
            let assignments: BTreeMap<_, _> = claim_queue_assignments().collect();
            assert_eq!(
                assignments.get(&CoreIndex(0)).unwrap(),
                &Assignment::Bulk(1000u32.into())
            );
            assert_eq!(
                assignments.get(&CoreIndex(1)).unwrap(),
                &Assignment::Pool {
                    para_id: 1000u32.into(),
                    core_index: CoreIndex(1)
                }
            );

            // Now we try to create the block
            // Since we have 2 cores (the one we bought, and the one assigned for being parachain)
            // This is assumed to be elastic scaling, and since it is not enabled in the node features,
            // it will fail
            let cores_with_backed: BTreeMap<_, _> =
                vec![(1000u32, Session::validators().len() as u32)]
                    .into_iter()
                    .collect();

            let inherent_data = ParasInherentTestBuilder::<Runtime>::new()
                .set_backed_and_concluding_paras(cores_with_backed)
                .build();
            set_new_inherent_data(inherent_data);
            // This should filter out, because we are trying to use elastic scaling when not enabled
            run_block();
        })
}

#[test]
#[should_panic(expected = "InherentDataFilteredDuringExecution")]
// This test does not panic when producing the candidate, but when injecting it as backed
// the inclusion pallet will filter it as it does not have a core assigned
fn test_parathread_that_does_not_buy_core_does_not_have_affinity() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 2,
            min_orchestrator_collators: 0,
            max_orchestrator_collators: 0,
            collators_per_container: 2,
            ..Default::default()
        })
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .with_para_ids(vec![ParaRegistrationParams {
            para_id: 1000,
            genesis_data: empty_genesis_data(),
            block_production_credits: u32::MAX,
            collator_assignment_credits: u32::MAX,
            parathread_params: Some(tp_traits::ParathreadParams {
                slot_frequency: SlotFrequency { min: 1, max: 1 },
            }),
        }])
        .with_inherent_data_enabled()
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
            node_features: bitvec::vec::BitVec::from_element(
                (1u8 << (FeatureIndex::ElasticScalingMVP as usize))
                    | (1u8 << (FeatureIndex::CandidateReceiptV2 as usize)),
            ),
            ..Default::default()
        })
        .with_keystore(Arc::new(MemoryKeystore::new()))
        .build()
        .execute_with(|| {
            run_to_block(2);
            // Now the parathread should be there
            assert!(Paras::is_parathread(1000u32.into()));
            let alice_keys = get_authority_keys_from_seed(&AccountId::from(ALICE).to_string());

            // Parathread should have collators
            assert!(
                authorities_for_container(1000u32.into()) == Some(vec![alice_keys.nimbus.clone()])
            );

            // The claim queue should be empty for the parathread, as it did not buy any core
            let assignments: BTreeMap<_, _> = claim_queue_assignments().collect();
            assert!(assignments.is_empty());

            // We try producing without having an on-demand core, this should panic
            let cores_with_backed: BTreeMap<_, _> =
                vec![(1000u32, Session::validators().len() as u32)]
                    .into_iter()
                    .collect();

            let inherent_data = ParasInherentTestBuilder::<Runtime>::new()
                .set_backed_and_concluding_paras(cores_with_backed)
                .build();
            set_new_inherent_data(inherent_data);
            // This should filter out, as we dont have an on-demand core bought
            run_block();
        })
}

#[test]
// This test does not panic when producing the candidate, but when injecting it as backed
// the inclusion pallet will filter it as it does not have a core assigned
fn test_parathread_that_buys_core_has_affinity_and_can_produce() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 2,
            min_orchestrator_collators: 0,
            max_orchestrator_collators: 0,
            collators_per_container: 2,
            ..Default::default()
        })
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .with_para_ids(vec![ParaRegistrationParams {
            para_id: 1000,
            genesis_data: empty_genesis_data(),
            block_production_credits: u32::MAX,
            collator_assignment_credits: u32::MAX,
            parathread_params: Some(tp_traits::ParathreadParams {
                slot_frequency: SlotFrequency { min: 1, max: 1 },
            }),
        }])
        .with_inherent_data_enabled()
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
            node_features: bitvec::vec::BitVec::from_element(
                (1u8 << (FeatureIndex::ElasticScalingMVP as usize))
                    | (1u8 << (FeatureIndex::CandidateReceiptV2 as usize)),
            ),
            ..Default::default()
        })
        .with_keystore(Arc::new(MemoryKeystore::new()))
        .build()
        .execute_with(|| {
            run_to_block(2);
            // Now the parathread should be there
            assert!(Paras::is_parathread(1000u32.into()));
            let alice_keys = get_authority_keys_from_seed(&AccountId::from(ALICE).to_string());

            // Parathread should have collators
            assert!(
                authorities_for_container(1000u32.into()) == Some(vec![alice_keys.nimbus.clone()])
            );

            // let's buy core
            assert_ok!(OnDemandAssignmentProvider::place_order_allow_death(
                origin_of(ALICE.into()),
                100 * UNIT,
                1000u32.into()
            ));
            run_block();

            // The claim queue this time allows us to produce blocks
            let assignments: BTreeMap<_, _> = claim_queue_assignments().collect();
            assert_eq!(
                assignments.get(&CoreIndex(0)).unwrap(),
                &Assignment::Pool {
                    para_id: 1000u32.into(),
                    core_index: CoreIndex(0)
                }
            );

            // We try producing having an on-demand core
            let cores_with_backed: BTreeMap<_, _> =
                vec![(1000u32, Session::validators().len() as u32)]
                    .into_iter()
                    .collect();

            let inherent_data = ParasInherentTestBuilder::<Runtime>::new()
                .set_backed_and_concluding_paras(cores_with_backed)
                .build();
            set_new_inherent_data(inherent_data);

            let availability_before = Runtime::candidates_pending_availability(1000u32.into());
            // Before there is no availability as we never injected a candidate
            assert_eq!(availability_before.len(), 0);
            // This should work
            run_block();
            let availability_after = Runtime::candidates_pending_availability(1000u32.into());
            // After the availability length is 1 as we have one candidate succesfully backed
            assert_eq!(availability_after.len(), 1);
        })
}

#[test]
fn test_on_demand_core_affinity_bound_to_core_gets_expired_at_session_boundaries() {
    sp_tracing::try_init_simple();

    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 2,
            min_orchestrator_collators: 0,
            max_orchestrator_collators: 0,
            collators_per_container: 2,
            ..Default::default()
        })
        .with_inherent_data_enabled()
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .with_para_ids(vec![ParaRegistrationParams {
            para_id: 1000,
            genesis_data: empty_genesis_data(),
            block_production_credits: u32::MAX,
            collator_assignment_credits: u32::MAX,
            parathread_params: Some(tp_traits::ParathreadParams {
                slot_frequency: SlotFrequency { min: 1, max: 1 },
            }),
        }])
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
            node_features: bitvec::vec::BitVec::from_element(
                (1u8 << (FeatureIndex::ElasticScalingMVP as usize))
                    | (1u8 << (FeatureIndex::CandidateReceiptV2 as usize)),
            ),
            ..Default::default()
        })
        .with_keystore(Arc::new(MemoryKeystore::new()))
        .build()
        .execute_with(|| {
            run_to_block(2);
            // Now the parathread should be there
            assert!(Paras::is_parathread(1000u32.into()));
            let alice_keys = get_authority_keys_from_seed(&AccountId::from(ALICE).to_string());

            // Parathread should have collators
            assert!(
                authorities_for_container(1000u32.into()) == Some(vec![alice_keys.nimbus.clone()])
            );

            // let's buy core
            assert_ok!(OnDemandAssignmentProvider::place_order_allow_death(
                origin_of(ALICE.into()),
                100 * UNIT,
                1000u32.into()
            ));
            run_block();

            // The claim queue this time allows us to produce blocks
            let assignments: BTreeMap<_, _> = claim_queue_assignments().collect();
            assert_eq!(
                assignments.get(&CoreIndex(0)).unwrap(),
                &Assignment::Pool {
                    para_id: 1000u32.into(),
                    core_index: CoreIndex(0)
                }
            );

            // When we do run_to_session, we only do on_initialize(block in which session changed)
            // Since we still did not do on_finalize, the parathread is still bounded to core 0
            let key = affinity_key_for_parathread(1000u32);
            let value_before_session: Option<CoreAffinityCount> =
                frame_support::storage::unhashed::get(key.as_ref());

            assert_eq!(
                value_before_session,
                Some(CoreAffinityCount {
                    core_index: CoreIndex(0),
                    count: 1
                })
            );

            // We dont produce a block this time, so that our affinity does not get decreased before the session
            run_to_session(1);

            // However as soon as we do on_finalize of the current block (the session boundary change block) the affinity
            // will be removed, allowing parathread 1000 to use any core available that is not 0
            // The latter is demonstrated better in the following test
            let value_after_session: Option<CoreAffinityCount> =
                frame_support::storage::unhashed::get(key.as_ref());
            assert_eq!(value_after_session, None);

            // The claim queue is empty
            let assignments: BTreeMap<_, _> = claim_queue_assignments().collect();
            assert!(assignments.is_empty());
        })
}
#[test]
fn test_parathread_uses_0_and_then_1_after_parachain_onboarded() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 2,
            min_orchestrator_collators: 0,
            max_orchestrator_collators: 0,
            collators_per_container: 1,
            collators_per_parathread: 1,
            ..Default::default()
        })
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .with_para_ids(vec![ParaRegistrationParams {
            para_id: 2001,
            genesis_data: empty_genesis_data(),
            block_production_credits: u32::MAX,
            collator_assignment_credits: u32::MAX,
            parathread_params: Some(tp_traits::ParathreadParams {
                slot_frequency: SlotFrequency { min: 1, max: 1 },
            }),
        }])
        .with_inherent_data_enabled()
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
            node_features: bitvec::vec::BitVec::from_element(
                (1u8 << (FeatureIndex::ElasticScalingMVP as usize))
                    | (1u8 << (FeatureIndex::CandidateReceiptV2 as usize)),
            ),
            ..Default::default()
        })
        .with_keystore(Arc::new(MemoryKeystore::new()))
        .build()
        .execute_with(|| {
            // IMPORTANT: we use parachain 1000 and parathread 1001 because when both cores
            // need to be used, they will get inserted in the cores_with_backed map ordered
            // this is, 1000 will go first, then 1001. Since we want 1000 to use core 0,
            // the only way to achieve this is by assigning the parathread a higher para-id
            run_to_block(2);

            // Now the parathread should be there
            assert!(Paras::is_parathread(2001u32.into()));
            let alice_keys = get_authority_keys_from_seed(&AccountId::from(ALICE).to_string());

            // Parathread should have collators
            assert!(
                authorities_for_container(2001u32.into()) == Some(vec![alice_keys.nimbus.clone()])
            );

            // Register parachain
            assert_ok!(Registrar::reserve(origin_of(ALICE.into())));
            assert_ok!(ContainerRegistrar::register(
                origin_of(ALICE.into()),
                2000.into(),
                get_genesis_data_with_validation_code().0,
                Some(HeadData(vec![1u8, 1u8, 1u8]))
            ));

            run_to_session(2);

            // We call run_block() after each run_to_session() for on_finalize() to
            // be properly executed, and thus to coordinate all the necessary storages,
            // specially ParasShared and Session CurrentIndex storages.
            //
            // If we don't do this, ParasShared's CurrentIndex is configured to
            // one session before, and we want the to be the same for later checking in
            // session 6.
            run_block();

            // We need to accept the validation code, so that the para is onboarded after 2 sessions.
            assert_ok!(Paras::add_trusted_validation_code(
                root_origin(),
                get_genesis_data_with_validation_code().1.into()
            ));

            run_to_session(4);
            run_block();

            set_dummy_boot_node(origin_of(ALICE.into()), 2000.into());
            assert_ok!(ContainerRegistrar::mark_valid_for_collating(
                root_origin(),
                2000.into()
            ));

            // The parathread now uses core 0 but once the parachain is onboarded (and gets collators)
            // it should use core 1.
            // let's just go to the block right before edge of session 6.
            let epoch_duration = EpochDurationInBlocks::get();

            run_to_block(6 * epoch_duration - 2);
            // we are not a parachain yet
            assert!(!Paras::is_parachain(2000u32.into()));
            // we dont have authorities
            assert_eq!(authorities_for_container(2000u32.into()), None);

            // let's buy core for 2001
            assert_ok!(OnDemandAssignmentProvider::place_order_allow_death(
                origin_of(ALICE.into()),
                100 * UNIT,
                2001u32.into()
            ));

            // We need to run one block for the place order to have effect in the claim queue
            run_block();

            let key = affinity_key_for_parathread(2001u32);

            let value_before_use_core_0: Option<CoreAffinityCount> =
                frame_support::storage::unhashed::get(key.as_ref());

            // Affinity should have assigned to core 0 for parathread
            assert_eq!(
                value_before_use_core_0,
                Some(CoreAffinityCount {
                    core_index: CoreIndex(0),
                    count: 1
                })
            );

            // The claim queue allows us to produce blocks at core 0
            let assignments: BTreeMap<_, _> = claim_queue_assignments().collect();
            assert_eq!(
                assignments.get(&CoreIndex(0)).unwrap(),
                &Assignment::Pool {
                    para_id: 2001u32.into(),
                    core_index: CoreIndex(0)
                }
            );

            // We try producing having an on-demand core
            let cores_with_backed: BTreeMap<_, _> =
                vec![(2001u32, Session::validators().len() as u32)]
                    .into_iter()
                    .collect();

            let inherent_data = ParasInherentTestBuilder::<Runtime>::new()
                .set_backed_and_concluding_paras(cores_with_backed)
                .build();
            set_new_inherent_data(inherent_data.clone());
            run_block();

            // Value after use
            let value_after_use_core_0: Option<CoreAffinityCount> =
                frame_support::storage::unhashed::get(key.as_ref());

            // Affinity gets cleared after use!
            assert_eq!(value_after_use_core_0, None);

            // let's run to right after the edge
            // We need one more run block to trigger the on_finalize
            run_to_session(6);
            run_block();
            // Now the parachain should be there
            assert!(Paras::is_parachain(2000u32.into()));

            let bob_keys = get_authority_keys_from_seed(&AccountId::from(BOB).to_string());
            // we should have authorities now: two sessions elapsed and para is parachain already
            assert_eq!(
                authorities_for_container(2000u32.into()),
                Some(vec![bob_keys.nimbus.clone()])
            );

            // 2000 should occupy core 0 now, as it is a parachains. which means if we try to buy a core (and use it)
            // for parathread 2001 then it should assign core 1 to the parathread
            // let's buy core for 2001
            assert_ok!(OnDemandAssignmentProvider::place_order_allow_death(
                origin_of(ALICE.into()),
                100 * UNIT,
                2001u32.into()
            ));
            run_block();

            // The claim queue allows us to produce blocks at core 1 for 2001
            // and in core 0 for 2000
            let assignments: BTreeMap<_, _> = claim_queue_assignments().collect();
            assert_eq!(
                assignments.get(&CoreIndex(0)).unwrap(),
                &Assignment::Bulk(2000u32.into())
            );
            assert_eq!(
                assignments.get(&CoreIndex(1)).unwrap(),
                &Assignment::Pool {
                    para_id: 2001u32.into(),
                    core_index: CoreIndex(1)
                }
            );

            // We now have a parachain at core 0. Therefore the affinity should be bounded to core 1
            let value_after_session_core_1: Option<CoreAffinityCount> =
                frame_support::storage::unhashed::get(key.as_ref());

            // 2001 is bounded to core 1!
            assert_eq!(
                value_after_session_core_1,
                Some(CoreAffinityCount {
                    core_index: CoreIndex(1),
                    count: 1
                })
            );

            // We try producing having an on-demand core
            let cores_with_backed: BTreeMap<_, _> = vec![
                (2000u32, Session::validators().len() as u32),
                (2001u32, Session::validators().len() as u32),
            ]
            .into_iter()
            .collect();

            let inherent_data = ParasInherentTestBuilder::<Runtime>::new()
                .set_backed_and_concluding_paras(cores_with_backed)
                .build();

            set_new_inherent_data(inherent_data);
            run_block();

            let value_after_session_after_use: Option<CoreAffinityCount> =
                frame_support::storage::unhashed::get(key.as_ref());

            // The affinity has been used, should be cleared
            assert_eq!(value_after_session_after_use, None);
        })
}

#[test]
fn test_should_have_availability_for_registered_parachain() {
    sp_tracing::try_init_simple();

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
        .with_inherent_data_enabled()
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
            node_features: bitvec::vec::BitVec::from_element(
                (1u8 << (FeatureIndex::ElasticScalingMVP as usize))
                    | (1u8 << (FeatureIndex::CandidateReceiptV2 as usize)),
            ),
            ..Default::default()
        })
        .with_para_ids(vec![ParaRegistrationParams {
            para_id: 1000,
            genesis_data: empty_genesis_data(),
            block_production_credits: u32::MAX,
            collator_assignment_credits: u32::MAX,
            parathread_params: None,
        }])
        .with_keystore(Arc::new(MemoryKeystore::new()))
        .build()
        .execute_with(|| {
            run_to_block(2);
            // Parachain 1000 should have affinity and claim queue should reflect it
            let assignments: BTreeMap<_, _> = claim_queue_assignments().collect();
            assert_eq!(
                assignments.get(&CoreIndex(0)).unwrap(),
                &Assignment::Bulk(1000u32.into())
            );

            let cores_with_backed: BTreeMap<_, _> =
                vec![(1000u32, Session::validators().len() as u32)]
                    .into_iter()
                    .collect();
            let alice_keys = get_authority_keys_from_seed(&AccountId::from(ALICE).to_string());
            let bob_keys = get_authority_keys_from_seed(&AccountId::from(BOB).to_string());

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

// we dont have access to the type so this is the only thing we can do
#[derive(Encode, Decode, Debug, Default, Clone, Copy, PartialEq, scale_info::TypeInfo)]
pub struct CoreAffinityCount {
    pub core_index: cumulus_primitives_core::relay_chain::CoreIndex,
    pub count: u32,
}

/// Get affinity for a given parathread
pub fn affinity_key_for_parathread(para_id: u32) -> Vec<u8> {
    let key = storage_map_final_key::<frame_support::Twox64Concat>(
        "OnDemandAssignmentProvider",
        "ParaIdAffinity",
        &cumulus_primitives_core::ParaId::from(para_id).encode(),
    );
    key
}

/// Get claim queue assignments
fn claim_queue_assignments() -> impl Iterator<Item = (CoreIndex, Assignment)> {
    let claim_queue = runtime_parachains::scheduler::ClaimQueue::<Runtime>::get();
    claim_queue
        .into_iter()
        .filter_map(|(core_idx, v)| v.front().map(|a| (core_idx, a.clone())))
}
