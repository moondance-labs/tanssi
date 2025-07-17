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
    frame_support::BoundedBTreeSet,
    pallet_inactivity_tracking::pallet::{
        ActiveCollatorsForCurrentSession, ActiveContainerChainsForCurrentSession, InactiveCollators,
    },
    parity_scale_codec::Encode,
    sp_consensus_aura::AURA_ENGINE_ID,
    sp_runtime::{traits::BlakeTwo256, DigestItem},
    test_relay_sproof_builder::{HeaderAs, ParaHeaderSproofBuilder, ParaHeaderSproofBuilderItem},
    tp_traits::{NodeActivityTrackingHelper, ParaId},
};

fn note_blocks_for_container_chain(
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
fn inactivity_tracking_correctly_updates_storages() {
    ExtBuilder::default()
        .with_empty_parachains(vec![3000, 3001])
        .with_collators(vec![
            (AccountId::from(ALICE), 100_000),
            (AccountId::from(BOB), 100_000),
            (AccountId::from(CHARLIE), 100_000),
            (AccountId::from(DAVE), 100_000),
        ])
        .build()
        .execute_with(|| {
            let assignment = TanssiCollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&3000.into()],
                vec![ALICE.into(), BOB.into()]
            );
            assert_eq!(
                assignment.container_chains[&3001.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );
            note_blocks_for_container_chain(3000.into(), 3001.into(), 1, session_to_block(1));
            run_block();
            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get(),
                get_collators_set(vec![ALICE.into(), DAVE.into()])
            );
            assert_eq!(
                <ActiveContainerChainsForCurrentSession<Runtime>>::get(),
                get_chains_set(vec![3000.into(), 3001.into()])
            );

            run_block();
            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get(),
                get_collators_set(vec![ALICE.into(), DAVE.into()])
            );
            assert_eq!(
                <ActiveContainerChainsForCurrentSession<Runtime>>::get(),
                get_chains_set(vec![3000.into(), 3001.into()])
            );

            run_to_session(1);
            run_block();
            assert_eq!(
                <InactiveCollators<Runtime>>::get(0),
                get_collators_set(vec![BOB.into(), CHARLIE.into()])
            );
            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get(),
                get_collators_set(vec![])
            );
            assert_eq!(
                <ActiveContainerChainsForCurrentSession<Runtime>>::get(),
                get_chains_set(vec![])
            );

            run_block();
            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get(),
                get_collators_set(vec![])
            );
            assert_eq!(
                <ActiveContainerChainsForCurrentSession<Runtime>>::get(),
                get_chains_set(vec![])
            );
            run_to_session(2);
            run_block();

            // Since chains 3000 and 3001 are inactive, all collators should be marked as active
            assert_eq!(
                <InactiveCollators<Runtime>>::get(1),
                get_collators_set(vec![])
            );
            assert_eq!(<ActiveCollatorsForCurrentSession<Runtime>>::get().len(), 0);
            assert_eq!(
                <ActiveContainerChainsForCurrentSession<Runtime>>::get(),
                get_chains_set(vec![])
            );

            let max_inactive_sessions =
                <Runtime as pallet_inactivity_tracking::Config>::MaxInactiveSessions::get();
            run_to_session(max_inactive_sessions - 1);
            run_block();
            assert!(!InactivityTracking::is_node_inactive(&AccountId::from(
                ALICE
            )));
            assert!(!InactivityTracking::is_node_inactive(&AccountId::from(BOB)));
            assert!(!InactivityTracking::is_node_inactive(&AccountId::from(
                CHARLIE
            )));
            assert!(!InactivityTracking::is_node_inactive(&AccountId::from(
                DAVE
            )));

            run_to_session(max_inactive_sessions);
            assert!(!InactivityTracking::is_node_inactive(&AccountId::from(
                ALICE
            )));
            assert!(!InactivityTracking::is_node_inactive(&AccountId::from(BOB)));
            assert!(!InactivityTracking::is_node_inactive(&AccountId::from(
                CHARLIE
            )));
            assert!(!InactivityTracking::is_node_inactive(&AccountId::from(
                DAVE
            )));
            assert!(!<InactiveCollators<Runtime>>::get(0).is_empty());

            run_to_session(max_inactive_sessions + 1);
            run_block();

            assert!(<InactiveCollators<Runtime>>::get(0).is_empty());
        });
}
