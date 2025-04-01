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

use sp_runtime::BoundedBTreeSet;
use {
    crate::tests::common::*,
    frame_support::traits::Get,
    pallet_inactivity_tracking::pallet::{
        ActiveCollators, ActiveCollatorsForCurrentSession, ActiveContainerChainsForCurrentSession,
    },
    tp_traits::{
        AuthorNotingHook, AuthorNotingInfo, MaybeSelfChainBlockAuthor, NodeActivityTrackingHelper,
        ParaId,
    },
};

fn get_author_noting_info(
    author: &AccountId,
    container_chain: &ParaId,
) -> AuthorNotingInfo<AccountId> {
    AuthorNotingInfo {
        block_number: System::block_number(),
        author: author.clone(),
        para_id: *container_chain,
    }
}

fn note_block_authors(authors: Vec<(AccountId, ParaId)>) {
    let mut authors_info: Vec<AuthorNotingInfo<AccountId>> = Vec::new();
    authors.iter().for_each(|block_info| {
        authors_info.push(get_author_noting_info(&block_info.0, &block_info.1))
    });
    let _ = InactivityTracking::on_container_authors_noted(&authors_info.as_slice());
}

fn get_active_collators_for_current_session(
    collators: Vec<AccountId>,
) -> BoundedBTreeSet<
    AccountId,
    <Runtime as pallet_inactivity_tracking::Config>::MaxCollatorsPerSession,
> {
    let mut collator_set: BoundedBTreeSet<
        AccountId,
        <Runtime as pallet_inactivity_tracking::Config>::MaxCollatorsPerSession,
    > = BoundedBTreeSet::new();
    collators.iter().for_each(|collator| {
        collator_set.try_insert(collator.clone()).ok();
    });
    collator_set
}

fn get_active_chains_for_current_session(
    chains: Vec<ParaId>,
) -> BoundedBTreeSet<ParaId, <Runtime as pallet_inactivity_tracking::Config>::MaxContainerChains> {
    let mut chains_set: BoundedBTreeSet<
        ParaId,
        <Runtime as pallet_inactivity_tracking::Config>::MaxContainerChains,
    > = BoundedBTreeSet::new();
    chains.iter().for_each(|collator| {
        chains_set.try_insert(collator.clone()).ok();
    });
    chains_set
}

#[test]
fn inactivity_tracking_correctly_updates_active_chain_and_collator_storages() {
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
            run_block();
            note_block_authors(vec![(CHARLIE.into(), 3001.into())]);
            assert_eq!(<Runtime as pallet_inactivity_tracking::Config>::GetSelfChainBlockAuthor::get_block_author(), Some(BOB.into()));
            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get(),
                get_active_collators_for_current_session(vec![ALICE.into(),BOB.into(),CHARLIE.into()])
            );
            assert_eq!(
                <ActiveContainerChainsForCurrentSession<Runtime>>::get(),
                get_active_chains_for_current_session(vec![ParaId::from(3001)])
            );
            run_block();
            note_block_authors(vec![(ALICE.into(), 3000.into())]);
            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get(),
                get_active_collators_for_current_session(vec![ALICE.into(),BOB.into(),CHARLIE.into()])
            );
            assert_eq!(
                <ActiveContainerChainsForCurrentSession<Runtime>>::get(),
                get_active_chains_for_current_session(vec![ParaId::from(3000), ParaId::from(3001)])
            );
            assert_eq!(<ActiveCollatorsForCurrentSession<Runtime>>::get().len(), 3);
            run_to_session(1);
            run_block();

            assert_eq!(
                <ActiveCollators<Runtime>>::get(0).contains(&ALICE.into()),
                true
            );
            assert_eq!(
                <ActiveCollators<Runtime>>::get(0).contains(&BOB.into()),
                true
            );
            assert_eq!(
                <ActiveCollators<Runtime>>::get(0).contains(&CHARLIE.into()),
                true
            );
            assert_eq!(
                <ActiveCollators<Runtime>>::get(0).contains(&DAVE.into()),
                false
            );
            assert_eq!(<ActiveCollatorsForCurrentSession<Runtime>>::get().len(), 2);
            assert_eq!(<ActiveContainerChainsForCurrentSession<Runtime>>::get().len(), 0);

            note_block_authors(vec![(CHARLIE.into(), 3000.into())]);
            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get(),
                get_active_collators_for_current_session(vec![ALICE.into(),BOB.into(),CHARLIE.into()])
            );
            assert_eq!(
                <ActiveContainerChainsForCurrentSession<Runtime>>::get(),
                get_active_chains_for_current_session(vec![ParaId::from(3000)])
            );

            run_to_session(2);
            //TO DO: Check if process_inactive_chains_for_session is called
            run_block();

            assert_eq!(
                <ActiveCollators<Runtime>>::get(1).contains(&CHARLIE.into()),
                true
            );
            assert_eq!(
                <ActiveCollators<Runtime>>::get(1).contains(&BOB.into()),
                true
            );
            assert_eq!(
                <ActiveCollators<Runtime>>::get(1).contains(&DAVE.into()),
                false
            );
            assert_eq!(
                <ActiveCollators<Runtime>>::get(1).contains(&ALICE.into()),
                true
            );
            assert_eq!(<ActiveCollatorsForCurrentSession<Runtime>>::get().len(), 2);
            assert_eq!(<ActiveContainerChainsForCurrentSession<Runtime>>::get().len(), 0);
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
                false
            );
            assert_eq!(
                InactivityTracking::is_node_inactive(
                    &cumulus_primitives_core::relay_chain::AccountId::from(DAVE)
                ),
                true
            );
            assert_eq!(<ActiveCollators<Runtime>>::get(0).is_empty(), false);

            run_to_session(max_inactive_sessions + 1);
            run_block();
            assert_eq!(<ActiveCollators<Runtime>>::get(0).is_empty(), true);
        });
}
