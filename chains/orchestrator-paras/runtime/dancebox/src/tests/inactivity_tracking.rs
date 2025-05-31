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
    frame_support::{assert_ok, BoundedBTreeSet},
    pallet_inactivity_tracking::pallet::{
        ActiveCollatorsForCurrentSession, ActiveContainerChainsForCurrentSession, InactiveCollators,
    },
    parity_scale_codec::Encode,
    sp_consensus_aura::AURA_ENGINE_ID,
    sp_runtime::{traits::BlakeTwo256, DigestItem},
    sp_std::collections::btree_set::BTreeSet,
    test_relay_sproof_builder::{HeaderAs, ParaHeaderSproofBuilder, ParaHeaderSproofBuilderItem},
    tp_traits::{
        ForSession, GetContainerChainsWithCollators, MaybeSelfChainBlockAuthor,
        NodeActivityTrackingHelper, ParaId, ParathreadHelper, ParathreadParams, SlotFrequency,
    },
};

fn note_blocks_for_container_chain(para_id: ParaId, start_block: u32, end_block: u32, slot: u64) {
    // Simulate the inclusion of a block for a container chain
    let mut sproof = ParaHeaderSproofBuilder::default();

    for block_number in start_block..=end_block {
        let s = ParaHeaderSproofBuilderItem {
            para_id,
            author_id: HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                parent_hash: Default::default(),
                number: block_number,
                state_root: Default::default(),
                extrinsics_root: Default::default(),
                digest: sp_runtime::generic::Digest {
                    logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                },
            }),
        };
        sproof.items.push(s);
    }
    set_author_noting_inherent_data(sproof.clone())
}

fn note_blocks_for_2_container_chain(
    para_id_1: ParaId,
    para_id_2: ParaId,
    start_block: u32,
    end_block: u32,
) {
    // Simulate the inclusion of a block for a container chain
    let mut sproof = ParaHeaderSproofBuilder::default();

    for block_number in start_block..=end_block {
        let s1 = ParaHeaderSproofBuilderItem {
            para_id: para_id_1,
            author_id: HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                parent_hash: Default::default(),
                number: block_number,
                state_root: Default::default(),
                extrinsics_root: Default::default(),
                digest: sp_runtime::generic::Digest {
                    logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, 2u64.encode())],
                },
            }),
        };
        let s2 = ParaHeaderSproofBuilderItem {
            para_id: para_id_2,
            author_id: HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                parent_hash: Default::default(),
                number: block_number,
                state_root: Default::default(),
                extrinsics_root: Default::default(),
                digest: sp_runtime::generic::Digest {
                    logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, 3u64.encode())],
                },
            }),
        };
        sproof.items.push(s1);
        sproof.items.push(s2);
    }
    set_author_noting_inherent_data(sproof.clone())
}

fn get_collators_set(
    collators: Vec<cumulus_primitives_core::relay_chain::AccountId>,
) -> BoundedBTreeSet<
    cumulus_primitives_core::relay_chain::AccountId,
    <Runtime as pallet_inactivity_tracking::Config>::MaxCollatorsPerSession,
> {
    let mut collator_set: BoundedBTreeSet<
        cumulus_primitives_core::relay_chain::AccountId,
        <Runtime as pallet_inactivity_tracking::Config>::MaxCollatorsPerSession,
    > = BoundedBTreeSet::new();
    collators.iter().for_each(|collator| {
        collator_set.try_insert(collator.clone()).ok();
    });
    collator_set
}

fn get_chains_set(
    chains: Vec<ParaId>,
) -> BoundedBTreeSet<ParaId, <Runtime as pallet_inactivity_tracking::Config>::MaxContainerChains> {
    let mut chains_set: BoundedBTreeSet<
        ParaId,
        <Runtime as pallet_inactivity_tracking::Config>::MaxContainerChains,
    > = BoundedBTreeSet::new();
    for para_id in chains {
        chains_set.try_insert(para_id).unwrap();
    }
    chains_set
}

#[test]
fn inactivity_tracking_correctly_updates_storages_on_orchestrator_chains_author_noting() {
    ExtBuilder::default()
        .with_empty_parachains(vec![3000])
        .with_collators(vec![
            (AccountId::from(ALICE), 100_000),
            (AccountId::from(BOB), 100_000),
            (AccountId::from(CHARLIE), 100_000),
            (AccountId::from(DAVE), 100_000),
        ])
        .build()
        .execute_with(|| {
            assert_eq!(<Runtime as pallet_inactivity_tracking::Config>::GetSelfChainBlockAuthor::get_block_author(), Some(ALICE.into()));
            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get(),
                get_collators_set(vec![ALICE.into()])
            );
            run_block();
            assert_eq!(<Runtime as pallet_inactivity_tracking::Config>::GetSelfChainBlockAuthor::get_block_author(), Some(BOB.into()));
            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get(),
                get_collators_set(vec![ALICE.into(), BOB.into()])
            );
            assert_eq!(
                <ActiveContainerChainsForCurrentSession<Runtime>>::get(),
                get_chains_set(vec![])
            );
            run_block();
            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get(),
                get_collators_set(vec![ALICE.into(), BOB.into()])
            );
            assert_eq!(
                <ActiveContainerChainsForCurrentSession<Runtime>>::get(),
                get_chains_set(vec![])
            );

            run_to_session(1);
            run_block();

            // Since chain 3000 is inactive, all collators should be marked as active
            assert_eq!(
                <InactiveCollators<Runtime>>::get(0),
                get_collators_set(vec![])
            );
            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get(),
                get_collators_set(vec![ALICE.into(), BOB.into()])
            );
            assert_eq!(
                <ActiveContainerChainsForCurrentSession<Runtime>>::get(),
                get_chains_set(vec![])
            );
        });
}

#[test]
fn inactivity_tracking_correctly_updates_storages_on_container_chains_author_noting() {
    ExtBuilder::default()
        .with_empty_parachains(vec![3000])
        .with_collators(vec![
            (AccountId::from(ALICE), 100_000),
            (AccountId::from(BOB), 100_000),
            (AccountId::from(CHARLIE), 100_000),
            (AccountId::from(DAVE), 100_000),
        ])
        .build()
        .execute_with(|| {
            let max_inactive_sessions =
                <Runtime as pallet_inactivity_tracking::Config>::MaxInactiveSessions::get();
            assert_ok!(Configuration::set_max_orchestrator_collators(
                root_origin(),
                1
            ));
            assert_eq!(
                <ActiveContainerChainsForCurrentSession<Runtime>>::get(),
                get_chains_set(vec![])
            );
            note_blocks_for_container_chain(3000.into(), 1, session_to_block(1), 1);
            run_block();
            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get(),
                get_collators_set(vec![ALICE.into(), BOB.into(), DAVE.into()])
            );
            assert_eq!(
                <ActiveContainerChainsForCurrentSession<Runtime>>::get(),
                get_chains_set(vec![3000.into()])
            );
            run_block();
            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get(),
                get_collators_set(vec![ALICE.into(), BOB.into(), DAVE.into()])
            );
            assert_eq!(
                <ActiveContainerChainsForCurrentSession<Runtime>>::get(),
                get_chains_set(vec![3000.into()])
            );

            run_to_session(1);
            run_block();
            assert_eq!(
                <InactiveCollators<Runtime>>::get(0),
                get_collators_set(vec![CHARLIE.into()])
            );
            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get(),
                get_collators_set(vec![ALICE.into(), BOB.into()])
            );
            assert_eq!(
                <ActiveContainerChainsForCurrentSession<Runtime>>::get(),
                get_chains_set(vec![])
            );

            run_to_session(2);
            run_block();

            assert_eq!(
                <InactiveCollators<Runtime>>::get(1),
                get_collators_set(vec![])
            );
            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get(),
                get_collators_set(vec![ALICE.into()])
            );
            assert_eq!(
                <ActiveContainerChainsForCurrentSession<Runtime>>::get(),
                get_chains_set(vec![])
            );

            run_to_session(max_inactive_sessions - 1);
            run_block();
            assert_eq!(
                InactivityTracking::is_node_inactive(
                    &cumulus_primitives_core::relay_chain::AccountId::from(ALICE)
                ),
                false
            );
            assert_eq!(
                InactivityTracking::is_node_inactive(
                    &cumulus_primitives_core::relay_chain::AccountId::from(BOB)
                ),
                false
            );
            assert_eq!(
                InactivityTracking::is_node_inactive(
                    &cumulus_primitives_core::relay_chain::AccountId::from(CHARLIE)
                ),
                false
            );
            assert_eq!(
                InactivityTracking::is_node_inactive(
                    &cumulus_primitives_core::relay_chain::AccountId::from(DAVE)
                ),
                false
            );
            run_to_session(max_inactive_sessions);
            assert_eq!(
                InactivityTracking::is_node_inactive(
                    &cumulus_primitives_core::relay_chain::AccountId::from(ALICE)
                ),
                false
            );
            assert_eq!(
                InactivityTracking::is_node_inactive(
                    &cumulus_primitives_core::relay_chain::AccountId::from(BOB)
                ),
                false
            );
            assert_eq!(
                InactivityTracking::is_node_inactive(
                    &cumulus_primitives_core::relay_chain::AccountId::from(CHARLIE)
                ),
                false
            );
            assert_eq!(
                InactivityTracking::is_node_inactive(
                    &cumulus_primitives_core::relay_chain::AccountId::from(DAVE)
                ),
                false
            );
            assert_eq!(<InactiveCollators<Runtime>>::get(0).is_empty(), false);

            run_to_session(max_inactive_sessions + 1);
            run_block();
            assert_eq!(<InactiveCollators<Runtime>>::get(0).is_empty(), true);
        });
}

#[test]
fn inactivity_tracking_correctly_updates_storages_with_all_chains_being_inactive() {
    ExtBuilder::default()
        .with_empty_parachains(vec![3000, 3001])
        .with_collators(vec![
            (
                cumulus_primitives_core::relay_chain::AccountId::from(ALICE),
                100_000,
            ),
            (
                cumulus_primitives_core::relay_chain::AccountId::from(BOB),
                100_000,
            ),
            (
                cumulus_primitives_core::relay_chain::AccountId::from(CHARLIE),
                100_000,
            ),
            (
                cumulus_primitives_core::relay_chain::AccountId::from(DAVE),
                100_000,
            ),
        ])
        .build()
        .execute_with(|| {
            run_block();
            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get(),
                get_collators_set(vec![ALICE.into(), BOB.into()])
            );
            assert_eq!(
                <ActiveContainerChainsForCurrentSession<Runtime>>::get().len(),
                0
            );
            run_to_session(1);
            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get(),
                get_collators_set(vec![ALICE.into(), BOB.into()])
            );
            assert_eq!(
                <ActiveContainerChainsForCurrentSession<Runtime>>::get().len(),
                0
            );
            run_block();

            // Since chains 3000 and 3001 are inactive, all collators should be marked as active
            assert_eq!(
                <InactiveCollators<Runtime>>::get(0),
                get_collators_set(vec![])
            );
            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get(),
                get_collators_set(vec![ALICE.into(), BOB.into()])
            );
            assert_eq!(
                <ActiveContainerChainsForCurrentSession<Runtime>>::get().len(),
                0
            );

            run_to_session(2);
            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get(),
                get_collators_set(vec![ALICE.into(), BOB.into()])
            );
            assert_eq!(
                <ActiveContainerChainsForCurrentSession<Runtime>>::get().len(),
                0
            );
            run_block();

            assert_eq!(
                <InactiveCollators<Runtime>>::get(1),
                get_collators_set(vec![])
            );
            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get(),
                get_collators_set(vec![ALICE.into(), BOB.into()])
            );
            assert_eq!(
                <ActiveContainerChainsForCurrentSession<Runtime>>::get().len(),
                0
            );
        });
}

#[test]
fn inactivity_tracking_edge_case_one_block_per_collator() {
    // Check that all block authors are marked as active, even on edge cases such as the first or
    // last block of each session.

    // Skip test if not compiled with fast-runtime
    let session_period = crate::Period::get();
    if session_period > 10 {
        println!(
            "Skipping test because session period must be 10, is {:?}",
            session_period
        );
        return;
    }

    ExtBuilder::default()
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            min_orchestrator_collators: 2,
            max_orchestrator_collators: 100,
            ..Default::default()
        })
        .with_collators(vec![
            (AccountId::from(ALICE), 100_000),
            (AccountId::from(BOB), 100_000),
            (AccountId::from(CHARLIE), 100_000),
            (AccountId::from(DAVE), 100_000),
            (AccountId::from([8; 32]), 100_000),
            (AccountId::from([9; 32]), 100_000),
            (AccountId::from([10; 32]), 100_000),
            (AccountId::from([11; 32]), 100_000),
            (AccountId::from([12; 32]), 100_000),
            (AccountId::from([13; 32]), 100_000),
            (AccountId::from([14; 32]), 100_000),
            (AccountId::from([15; 32]), 100_000),
            (AccountId::from([16; 32]), 100_000),
            (AccountId::from([17; 32]), 100_000),
        ])
        .build()
        .execute_with(|| {
            run_to_session(2);

            // Confirm that all 14 collators have been assigned to orchestrator chain
            let collators =
                pallet_collator_assignment::Pallet::<Runtime>::collator_container_chain();
            let num_collators = collators.orchestrator_chain.len();
            assert_eq!(num_collators, 14);

            // Since 1 session = 10 blocks, at most 10 block authors will be marked as active per session.
            // In tests we use a simple round robin collator selection, so no collator will produce more
            // than one block per sessoin. Thus we can assert that we will see exactly 4 inactive collators
            // per session.
            let inactive_collators = InactiveCollators::<Runtime>::get(1);
            assert_eq!(
                inactive_collators.len(),
                num_collators - session_period as usize,
                "{:3}: {} inactive: {:?}",
                1,
                inactive_collators.len(),
                inactive_collators
            );
        });
}

#[test]
fn inactivity_tracking_edge_case_inactive_at_session_start() {
    // Check that an inactive collator can always be marked as inactive, even on edge cases such as the first or
    // last block of each session.

    // Skip test if not compiled with fast-runtime
    let session_period = crate::Period::get();
    if session_period > 10 {
        println!(
            "Skipping test because session period must be 10, is {:?}",
            session_period
        );
        return;
    }

    ExtBuilder::default()
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            min_orchestrator_collators: 2,
            max_orchestrator_collators: 100,
            ..Default::default()
        })
        .with_collators(vec![
            (AccountId::from(ALICE), 100_000),
            (AccountId::from(BOB), 100_000),
            (AccountId::from(CHARLIE), 100_000),
            (AccountId::from(DAVE), 100_000),
        ])
        .build()
        .execute_with(|| {
            let inactivity_period: u32 =
                <Runtime as pallet_inactivity_tracking::Config>::MaxInactiveSessions::get();
            run_to_session(2 + inactivity_period);

            // Mock inactive collator by writing directly into storage
            for s in 2..(2 + inactivity_period) {
                InactiveCollators::<Runtime>::insert(
                    s,
                    BoundedBTreeSet::try_from(BTreeSet::from([AccountId::from(BOB)])).unwrap(),
                );
            }

            let expected_session_index = Session::current_index();
            // Since run_block ends a block then starts one the last block of the session
            // we need to run the loop until session_period - 1
            for _ in 0..(session_period - 1) {
                assert_eq!(expected_session_index, Session::current_index());
                let bob_inactive = pallet_inactivity_tracking::Pallet::<Runtime>::is_node_inactive(
                    &AccountId::from(BOB),
                );
                assert!(bob_inactive);
                run_block();
            }

            // The next block is a new session
            run_block();
            let session_index = Session::current_index();
            assert_eq!(session_index, 8);
        });
}

#[test]
fn inactivity_tracking_edge_case_parachain_and_parathread_with_one_active_collator() {
    ExtBuilder::default()
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            min_orchestrator_collators: 1,
            max_orchestrator_collators: 1,
            collators_per_container: 2,
            collators_per_parathread: 2,
            ..Default::default()
        })
        .with_para_ids(vec![
            ParaRegistrationParams {
                para_id: 3001,
                parathread_params: Some(ParathreadParams {
                    slot_frequency: SlotFrequency::default(),
                }),
                genesis_data: empty_genesis_data(),
                block_production_credits: u32::MAX,
                collator_assignment_credits: u32::MAX,
            },
            ParaRegistrationParams {
                para_id: 3000,
                parathread_params: None,
                genesis_data: empty_genesis_data(),
                block_production_credits: u32::MAX,
                collator_assignment_credits: u32::MAX,
            },
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 100_000),
            (AccountId::from(BOB), 100_000),
            (AccountId::from(CHARLIE), 100_000),
            (AccountId::from(DAVE), 100_000),
            (AccountId::from([8; 32]), 100_000),
        ])
        .build()
        .execute_with(|| {
            assert_eq!(
                Session::current_index(),
                0
            );
            assert_eq!(
                <Runtime as pallet_inactivity_tracking::Config>::ParaFilter::get_parathreads_for_session(),
                get_chains_set(vec![3001.into()]).into_inner()
            );
            assert_eq!(
                <ActiveContainerChainsForCurrentSession<Runtime>>::get(),
                get_chains_set(vec![])
            );
            let container_chains_with_collators =
                <Runtime as pallet_inactivity_tracking::Config>::CurrentCollatorsFetcher::container_chains_with_collators(
                    ForSession::Current
                );
            let collators_for_parathread= container_chains_with_collators
                .iter().find(|(para_id, _)| para_id == &3001.into()).unwrap().clone().1;
            let collators_for_parachain= container_chains_with_collators
                .iter().find(|(para_id, _)| para_id == &3000.into()).unwrap().clone().1;

            assert_eq!(collators_for_parathread, vec![DAVE.into(), [8; 32].into()]);
            assert_eq!(collators_for_parachain, vec![BOB.into(), CHARLIE.into()]);
            note_blocks_for_2_container_chain(3000.into(), 3001.into(), 1, 2);
            run_block();
            // Verify that only one non-orchestrator collator is marked as active
            while Session::current_index() == 0 {
                assert_eq!(
                    <ActiveCollatorsForCurrentSession<Runtime>>::get(),
                    get_collators_set(vec![ALICE.into(), BOB.into(), [8; 32].into()])
                );
                assert_eq!(
                    <ActiveContainerChainsForCurrentSession<Runtime>>::get(),
                    get_chains_set(vec![3000.into(), 3001.into()])
                );
                run_block();
            }
            // Container chain 3001 is an active parathread - so should be handled an inactive chain
            // Container chain 3000 is an active parachain.
            // So only the inactive collator of the parachain should appear in the inactive collators set
            assert_eq!(
                <InactiveCollators<Runtime>>::get(0),
                get_collators_set(vec![CHARLIE.into()])
            );
            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get(),
                get_collators_set(vec![ALICE.into()])
            );
            assert_eq!(
                <ActiveContainerChainsForCurrentSession<Runtime>>::get().len(),
                0
            );
        });
}

#[test]
fn inactivity_tracking_edge_case_active_parachain_with_one_active_collator_and_inactive_parathread()
{
    ExtBuilder::default()
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            min_orchestrator_collators: 1,
            max_orchestrator_collators: 1,
            collators_per_container: 2,
            collators_per_parathread: 2,
            ..Default::default()
        })
        .with_para_ids(vec![
            ParaRegistrationParams {
                para_id: 3001,
                parathread_params: Some(ParathreadParams {
                    slot_frequency: SlotFrequency::default(),
                }),
                genesis_data: empty_genesis_data(),
                block_production_credits: u32::MAX,
                collator_assignment_credits: u32::MAX,
            },
            ParaRegistrationParams {
                para_id: 3000,
                parathread_params: None,
                genesis_data: empty_genesis_data(),
                block_production_credits: u32::MAX,
                collator_assignment_credits: u32::MAX,
            },
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 100_000),
            (AccountId::from(BOB), 100_000),
            (AccountId::from(CHARLIE), 100_000),
            (AccountId::from(DAVE), 100_000),
            (AccountId::from([8; 32]), 100_000),
        ])
        .build()
        .execute_with(|| {
            assert_eq!(
                Session::current_index(),
                0
            );
            assert_eq!(
                <Runtime as pallet_inactivity_tracking::Config>::ParaFilter::get_parathreads_for_session(),
                get_chains_set(vec![3001.into()]).into_inner()
            );
            assert_eq!(
                <ActiveContainerChainsForCurrentSession<Runtime>>::get(),
                get_chains_set(vec![])
            );
            let container_chains_with_collators =
                <Runtime as pallet_inactivity_tracking::Config>::CurrentCollatorsFetcher::container_chains_with_collators(
                    ForSession::Current
                );
            let collators_for_parathread= container_chains_with_collators
                .iter().find(|(para_id, _)| para_id == &3001.into()).unwrap().clone().1;
            let collators_for_parachain= container_chains_with_collators
                .iter().find(|(para_id, _)| para_id == &3000.into()).unwrap().clone().1;
            assert_eq!(collators_for_parathread, vec![DAVE.into(), [8; 32].into()]);
            assert_eq!(collators_for_parachain, vec![BOB.into(), CHARLIE.into()]);
            note_blocks_for_container_chain(3000.into(), 1, 2, 1);
            run_block();
            // Verify that only one non-orchestrator collator is marked as active
            while Session::current_index() == 0 {
                assert_eq!(
                    <ActiveCollatorsForCurrentSession<Runtime>>::get(),
                    get_collators_set(vec![ALICE.into(), CHARLIE.into()])
                );
                assert_eq!(
                    <ActiveContainerChainsForCurrentSession<Runtime>>::get(),
                    get_chains_set(vec![3000.into()])
                );
                run_block();
            }
            // Container chain 3001 is an inactive parathread
            // Container chain 3000 is an active parachain.
            // So only the inactive collator of the parachain should appear in the inactive collators set
            assert_eq!(
                <InactiveCollators<Runtime>>::get(0),
                get_collators_set(vec![BOB.into()])
            );
            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get(),
                get_collators_set(vec![ALICE.into()])
            );
            assert_eq!(
                <ActiveContainerChainsForCurrentSession<Runtime>>::get().len(),
                0
            );
        });
}
