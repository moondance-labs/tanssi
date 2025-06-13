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
        tests::common::*, CollatorAssignment, InactivityTracking, Invulnerables,
        MinimumSelfDelegation, PooledStaking,
    },
    frame_support::assert_ok,
    pallet_pooled_staking::{ActivePoolKind, SortedEligibleCandidates},
    tp_traits::NodeActivityTrackingHelper,
};

fn init_test_setup() {
    // Ensure that BOB is not an invulnerable collator and is part of the sorted eligible candidates list.
    assert_ok!(Invulnerables::remove_invulnerable(
        root_origin(),
        BOB.into()
    ));
    let stake = MinimumSelfDelegation::get() * 10;
    assert_ok!(PooledStaking::request_delegate(
        origin_of(BOB.into()),
        BOB.into(),
        ActivePoolKind::AutoCompounding,
        stake
    ));
    // Verify that BOB is
    //- a non-invulnerable collator
    //- assigned to a container chain
    //- part of the sorted eligible candidates list.
    assert_eq!(Invulnerables::invulnerables().contains(&BOB.into()), false);
    assert_eq!(
        CollatorAssignment::collator_container_chain()
            .container_chains
            .iter()
            .any(|(_, collators)| collators.contains(&BOB.into())),
        true
    );
    assert_eq!(
        <SortedEligibleCandidates<Runtime>>::get()
            .iter()
            .any(|c| c.candidate == BOB.into()),
        true
    );
    // Enable offline marking.
    assert_ok!(PooledStaking::enable_offline_marking(root_origin(), true));
}

#[test]
fn set_collator_offline_using_set_offline_removes_it_from_assigned_collators_and_sorted_eligible_candidates(
) {
    ExtBuilder::default()
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            min_orchestrator_collators: 1,
            max_orchestrator_collators: 1,
            collators_per_container: 2,
            ..Default::default()
        })
        .with_balances(vec![
            // BOB gets 10k extra tokens for his mapping deposit
            (AccountId::from(ALICE), 100_000 * UNIT),
            (AccountId::from(BOB), 210_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
            (AccountId::from(FERDIE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 100 * UNIT),
            (AccountId::from(BOB), 210 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
            (AccountId::from(FERDIE), 100 * UNIT),
        ])
        .with_empty_parachains(vec![3001u32, 3002u32])
        .build()
        .execute_with(|| {
            init_test_setup();
            assert_eq!(InactivityTracking::is_node_offline(&BOB.into()), false);
            assert_ok!(PooledStaking::set_offline(origin_of(BOB.into())));
            assert_eq!(InactivityTracking::is_node_offline(&BOB.into()), true);
            run_to_session(2);
            run_block();
            // Verify that after being set offline, BOB is no longer:
            // - assigned to any container chain
            // - in the sorted eligible candidates list
            assert_eq!(
                CollatorAssignment::collator_container_chain()
                    .container_chains
                    .iter()
                    .any(|(_, collators)| collators.contains(&BOB.into())),
                false
            );
            assert_eq!(
                <SortedEligibleCandidates<Runtime>>::get()
                    .iter()
                    .any(|c| c.candidate == BOB.into()),
                false
            );
        });
}

#[test]
fn set_collator_online_using_adds_it_to_assigned_collators_and_sorted_eligible_candidates() {
    ExtBuilder::default()
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            min_orchestrator_collators: 1,
            max_orchestrator_collators: 1,
            collators_per_container: 2,
            ..Default::default()
        })
        .with_balances(vec![
            // BOB gets 10k extra tokens for his mapping deposit
            (AccountId::from(ALICE), 100_000 * UNIT),
            (AccountId::from(BOB), 210_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
            (AccountId::from(FERDIE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 100 * UNIT),
            (AccountId::from(BOB), 210 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
            (AccountId::from(FERDIE), 100 * UNIT),
        ])
        .with_empty_parachains(vec![3001u32, 3002u32])
        .build()
        .execute_with(|| {
            init_test_setup();
            assert_eq!(InactivityTracking::is_node_offline(&BOB.into()), false);
            assert_ok!(PooledStaking::set_offline(origin_of(BOB.into())));
            run_to_session(2);
            run_block();
            assert_eq!(InactivityTracking::is_node_offline(&BOB.into()), true);
            assert_ok!(PooledStaking::set_online(origin_of(BOB.into())));
            assert_eq!(InactivityTracking::is_node_offline(&BOB.into()), false);
            run_to_session(4);
            run_block();
            // Verify that after being set online, BOB is:
            // - assigned to any container chain
            // - in the sorted eligible candidates list
            assert_eq!(
                CollatorAssignment::collator_container_chain()
                    .container_chains
                    .iter()
                    .any(|(_, collators)| collators.contains(&BOB.into())),
                true
            );
            assert_eq!(
                <SortedEligibleCandidates<Runtime>>::get()
                    .iter()
                    .any(|c| c.candidate == BOB.into()),
                true
            );
        });
}
