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
    frame_support::{assert_ok, traits::Get},
    pallet_inactivity_tracking::pallet::{ActiveCollators, ActiveCollatorsForCurrentSession},
    tp_traits::{AuthorNotingHook, AuthorNotingInfo, NodeActivityTrackingHelper, ParaId},
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
            assert_ok!(InactivityTracking::set_inactivity_tracking_status(
                root_origin(),
                true
            ));

            run_block();
            note_block_authors(vec![(ALICE.into(), 3000.into()), (BOB.into(), 3001.into())]);
            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get().contains(&ALICE.into()),
                true
            );
            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get().contains(&BOB.into()),
                true
            );
            assert_eq!(<ActiveCollatorsForCurrentSession<Runtime>>::get().len(), 2);

            run_block();
            note_block_authors(vec![(ALICE.into(), 3000.into()), (BOB.into(), 3001.into())]);
            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get().contains(&ALICE.into()),
                true
            );
            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get().contains(&BOB.into()),
                true
            );
            assert_eq!(<ActiveCollatorsForCurrentSession<Runtime>>::get().len(), 2);

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
                false
            );
            assert_eq!(
                <ActiveCollators<Runtime>>::get(0).contains(&DAVE.into()),
                false
            );
            assert_eq!(<ActiveCollatorsForCurrentSession<Runtime>>::get().len(), 0);

            note_block_authors(vec![(ALICE.into(), 3000.into()), (BOB.into(), 3001.into())]);
            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get().contains(&ALICE.into()),
                true
            );
            assert_eq!(
                <ActiveCollatorsForCurrentSession<Runtime>>::get().contains(&BOB.into()),
                true
            );
            assert_eq!(<ActiveCollatorsForCurrentSession<Runtime>>::get().len(), 2);

            run_block();
            // TO DO: Emulate orchestrator chain block author noting for CHARLIE

            run_to_session(2);
            run_block();

            assert_eq!(
                <ActiveCollators<Runtime>>::get(1).contains(&ALICE.into()),
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
            assert_eq!(<ActiveCollatorsForCurrentSession<Runtime>>::get().len(), 0);
            run_to_session(3);
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

            let max_inactive_sessions =
                <Runtime as pallet_inactivity_tracking::Config>::MaxInactiveSessions::get();

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

            run_block();

            assert_eq!(<ActiveCollators<Runtime>>::get(0).is_empty(), true);
        });
}
