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

use crate::{ContainerRegistrar, ParasSudoWrapper};
use primitives::CoreIndex;
use runtime_parachains::paras::{ParaGenesisArgs, ParaKind};
use starlight_runtime_constants::time::EpochDurationInBlocks;
use {
    crate::tests::common::*,
    crate::{OnDemandAssignmentProvider, Paras, Session},
    cumulus_primitives_core::relay_chain::{vstaging::SchedulerParams, AsyncBackingParams},
    frame_support::assert_ok,
    frame_system::pallet_prelude::BlockNumberFor,
    primitives::runtime_api::runtime_decl_for_parachain_host::ParachainHostV11,
    sp_core::{Decode, Encode},
    sp_keystore::testing::MemoryKeystore,
    sp_std::{collections::btree_map::BTreeMap, vec},
    std::sync::Arc,
    tp_traits::SlotFrequency,
};

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
#[should_panic(expected = "CandidatesFilteredDuringExecution")]
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
            ..Default::default()
        })
        .with_keystore(Arc::new(MemoryKeystore::new()))
        .build()
        .execute_with(|| {
            run_to_block(2);
            // Here para-id is not registered, but we can indeed buy a on-demand core for a non-existing para
            // however we should not be able to produce for it
            assert_ok!(OnDemandAssignmentProvider::place_order_allow_death(
                origin_of(ALICE.into()),
                100 * UNIT,
                1000u32.into()
            ));
            run_block();
            // Now we try to create the block
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
#[should_panic(expected = "CandidatesFilteredDuringExecution")]
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
            // Now the parathread should be there
            assert!(Paras::is_parathread(1000u32.into()));
            let alice_keys =
                get_authority_keys_from_seed(&AccountId::from(ALICE).to_string(), None);

            // Parathread should have collators
            assert!(
                authorities_for_container(1000u32.into()) == Some(vec![alice_keys.nimbus.clone()])
            );

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
            // Now the parathread should be there
            assert!(Paras::is_parathread(1000u32.into()));
            let alice_keys =
                get_authority_keys_from_seed(&AccountId::from(ALICE).to_string(), None);

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
        .with_relay_config(runtime_parachains::configuration::HostConfiguration::<
            BlockNumberFor<Runtime>,
        > {
            scheduler_params: SchedulerParams {
                num_cores: 2,
                // A very high number to avoid group rotation in tests
                // Otherwise we get a 1 by default, which changes groups every block
                group_rotation_frequency: 10000000,
                ttl: 2,
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
            // Now the parathread should be there
            assert!(Paras::is_parathread(1000u32.into()));
            let alice_keys =
                get_authority_keys_from_seed(&AccountId::from(ALICE).to_string(), None);

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

            // We try producing having an on-demand core
            let cores_with_backed: BTreeMap<_, _> =
                vec![(1000u32, Session::validators().len() as u32)]
                    .into_iter()
                    .collect();

            let inherent_data = ParasInherentTestBuilder::<Runtime>::new()
                .set_backed_and_concluding_paras(cores_with_backed)
                .build();
            set_new_inherent_data(inherent_data);
            run_block();

            run_to_session(1);

            // When we do run_to_session, we only do on_initialize(block in which session changed)
            // Since we still did not do on_finalize, the parathread is still bounded to core 0
            let key = storage_map_final_key::<frame_support::Twox64Concat>(
                "OnDemandAssignmentProvider",
                "ParaIdAffinity",
                &cumulus_primitives_core::ParaId::from(1000u32).encode(),
            );
            let value_before_session: Option<CoreAffinityCount> =
                frame_support::storage::unhashed::get(key.as_ref());

            assert_eq!(
                value_before_session,
                Some(CoreAffinityCount {
                    core_index: CoreIndex(0),
                    count: 1
                })
            );

            // However as soon as we do on_finalize of the current block (the session boundary change block) the affinity
            // will be removed, allowing parathread 1000 to use any core available that is not 0
            // The latter is demonstrated better in the following test
            end_block();
            let value_after_session: Option<CoreAffinityCount> =
                frame_support::storage::unhashed::get(key.as_ref());
            assert_eq!(value_after_session, None);
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
            para_id: 1001,
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
                ttl: 2,
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
            // IMPORTANT: we use parachain 1000 and parathread 1001 because when both cores
            // need to be used, they will get inserted in the cores_with_backed map ordered
            // this is, 1000 will go first, then 1001. Since we want 1000 to use core 0,
            // the only way to achieve this is by assigning the parathread a higher para-id
            run_to_block(2);
            // Now the parathread should be there
            assert!(Paras::is_parathread(1001u32.into()));
            let alice_keys =
                get_authority_keys_from_seed(&AccountId::from(ALICE).to_string(), None);

            // Parathread should have collators
            assert!(
                authorities_for_container(1001u32.into()) == Some(vec![alice_keys.nimbus.clone()])
            );

            // Register parachain
            assert_ok!(ContainerRegistrar::register(
                origin_of(ALICE.into()),
                1000.into(),
                empty_genesis_data()
            ));
            assert_ok!(ContainerRegistrar::mark_valid_for_collating(
                root_origin(),
                1000.into()
            ));
            assert_ok!(ParasSudoWrapper::sudo_schedule_para_initialize(
                root_origin(),
                1000.into(),
                ParaGenesisArgs {
                    genesis_head: ParasInherentTestBuilder::<Runtime>::mock_head_data(),
                    validation_code: mock_validation_code(),
                    para_kind: ParaKind::Parachain,
                },
            ));

            assert_ok!(Paras::add_trusted_validation_code(
                root_origin(),
                mock_validation_code()
            ));

            // The parathread now uses core 0 but once the parachain is onboarded (and gets collators)
            // it should use core 1.
            // let's just go to the block right before edge of session 2.
            let epoch_duration = EpochDurationInBlocks::get();

            run_to_block(2 * epoch_duration - 1);
            // we are not a parachain yet
            assert!(!Paras::is_parachain(1000u32.into()));
            // we dont have authorities
            assert_eq!(authorities_for_container(1000u32.into()), None);

            // let's buy core for 1001
            assert_ok!(OnDemandAssignmentProvider::place_order_allow_death(
                origin_of(ALICE.into()),
                100 * UNIT,
                1001u32.into()
            ));

            // We try producing having an on-demand core
            let cores_with_backed: BTreeMap<_, _> =
                vec![(1001u32, Session::validators().len() as u32)]
                    .into_iter()
                    .collect();

            let inherent_data = ParasInherentTestBuilder::<Runtime>::new()
                .set_backed_and_concluding_paras(cores_with_backed)
                .build();
            set_new_inherent_data(inherent_data.clone());
            run_block();

            let key = storage_map_final_key::<frame_support::Twox64Concat>(
                "OnDemandAssignmentProvider",
                "ParaIdAffinity",
                &cumulus_primitives_core::ParaId::from(1001u32).encode(),
            );
            let value_before_session: Option<CoreAffinityCount> =
                frame_support::storage::unhashed::get(key.as_ref());

            // 1001 is bounded to core 0!
            assert_eq!(
                value_before_session,
                Some(CoreAffinityCount {
                    core_index: CoreIndex(0),
                    count: 1
                })
            );

            // let's run to right after the edge
            // We need one more run block to trigger the on_finalize
            run_to_session(2);
            run_block();
            // Now the parachain should be there
            assert!(Paras::is_parachain(1000u32.into()));

            let bob_keys = get_authority_keys_from_seed(&AccountId::from(BOB).to_string(), None);
            // we should have authorities now: two sessions elapsed and para is parachain already
            assert_eq!(
                authorities_for_container(1000u32.into()),
                Some(vec![bob_keys.nimbus.clone()])
            );

            // 1000 should occupy core 0 now, as it is a parachains. which means if we try to buy a core (and use it)
            // for parathread 1001 then it should assign core 1 to the parathread
            // let's buy core for 1001
            assert_ok!(OnDemandAssignmentProvider::place_order_allow_death(
                origin_of(ALICE.into()),
                100 * UNIT,
                1001u32.into()
            ));

            // We try producing having an on-demand core
            let cores_with_backed: BTreeMap<_, _> = vec![
                (1000u32, Session::validators().len() as u32),
                (1001u32, Session::validators().len() as u32),
            ]
            .into_iter()
            .collect();

            let inherent_data = ParasInherentTestBuilder::<Runtime>::new()
                .set_backed_and_concluding_paras(cores_with_backed)
                .build();

            set_new_inherent_data(inherent_data);
            run_block();

            let value_after_session: Option<CoreAffinityCount> =
                frame_support::storage::unhashed::get(key.as_ref());

            // 1001 is bounded to core 1!
            assert_eq!(
                value_after_session,
                Some(CoreAffinityCount {
                    core_index: CoreIndex(1),
                    count: 1
                })
            );
        })
}

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

// we dont have access to the type so this is the only thing we can do
#[derive(Encode, Decode, Debug, Default, Clone, Copy, PartialEq, scale_info::TypeInfo)]
pub struct CoreAffinityCount {
    pub core_index: cumulus_primitives_core::relay_chain::CoreIndex,
    pub count: u32,
}
