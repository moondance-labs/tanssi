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
    frame_support::{assert_ok, traits::Get, BoundedBTreeSet},
    pallet_inactivity_tracking::pallet::{
        ActiveCollatorsForCurrentSession, ActiveContainerChainsForCurrentSession, InactiveCollators,
    },
    parity_scale_codec::Encode,
    sp_consensus_aura::AURA_ENGINE_ID,
    sp_runtime::{traits::BlakeTwo256, DigestItem},
    test_relay_sproof_builder::{HeaderAs, ParaHeaderSproofBuilder, ParaHeaderSproofBuilderItem},
    tp_traits::{MaybeSelfChainBlockAuthor, NodeActivityTrackingHelper, ParaId},
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
    chains.iter().for_each(|collator| {
        chains_set.try_insert(*collator).ok();
    });
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
            run_block();
            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get(),
                get_collators_set(vec![ALICE.into(), BOB.into()])
            );

            run_to_session(1);
            run_block();

            assert_eq!(
                <InactiveCollators<Runtime>>::get(0),
                get_collators_set(vec![CHARLIE.into(), DAVE.into()])
            );
            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get(),
                get_collators_set(vec![ALICE.into(), BOB.into()])
            );

            run_to_session(2);
            run_block();

            assert_eq!(
                <InactiveCollators<Runtime>>::get(1),
                get_collators_set(vec![CHARLIE.into(), DAVE.into()])
            );

            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get(),
                get_collators_set(vec![ALICE.into(), BOB.into()])
            );
            let max_inactive_sessions =
                <Runtime as pallet_inactivity_tracking::Config>::MaxInactiveSessions::get();
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
                true
            );
            assert_eq!(
                InactivityTracking::is_node_inactive(
                    &cumulus_primitives_core::relay_chain::AccountId::from(DAVE)
                ),
                true
            );
            assert_eq!(<InactiveCollators<Runtime>>::get(0).is_empty(), false);
            run_to_session(max_inactive_sessions + 1);
            run_block();
            assert_eq!(<InactiveCollators<Runtime>>::get(0).is_empty(), true);
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
            note_blocks_for_container_chain(3000.into(), 1, session_to_block(1), 1);
            run_block();
            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get(),
                get_collators_set(vec![ALICE.into(), BOB.into(), DAVE.into()])
            );
            run_block();
            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get(),
                get_collators_set(vec![ALICE.into(), BOB.into(), DAVE.into()])
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

            run_to_session(2);
            run_block();

            assert_eq!(
                <InactiveCollators<Runtime>>::get(1),
                get_collators_set(vec![CHARLIE.into(), DAVE.into()])
            );
            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get(),
                get_collators_set(vec![ALICE.into()])
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
                true
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
fn inactivity_tracking_correctly_updates_storages_with_all_chain_being_inactive() {
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

            // Since chain 3000 is inactive, all collators should be marked as active
            assert_eq!(
                <InactiveCollators<Runtime>>::get(0),
                get_collators_set(vec![ALICE.into(), BOB.into(), CHARLIE.into(), DAVE.into()])
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
                get_collators_set(vec![ALICE.into(), BOB.into(), CHARLIE.into(), DAVE.into()])
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
