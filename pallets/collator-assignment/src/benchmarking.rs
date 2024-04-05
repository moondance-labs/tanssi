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

//! Benchmarking setup for pallet-invulnerables

#![cfg(feature = "runtime-benchmarks")]

use super::*;

#[allow(unused)]
use {
    crate::Pallet,
    frame_benchmarking::{account, impl_benchmark_test_suite, v2::*, BenchmarkError},
    frame_support::{
        pallet_prelude::*,
        traits::{Currency, EnsureOrigin, Get},
    },
    frame_system::{EventRecord, RawOrigin},
    sp_std::collections::btree_map::BTreeMap,
    sp_std::prelude::*,
};

const SEED: u32 = 0;

fn invulnerable<T: Config>(c: u32, seed: u32) -> T::AccountId {
    account::<T::AccountId>("candidate", c, seed)
}

fn invulnerables<T: Config + frame_system::Config>(count: u32, seed: u32) -> Vec<T::AccountId> {
    (0..count)
        .map(|c| invulnerable::<T>(c, seed))
        .collect::<Vec<_>>()
}

fn assert_event_is_present<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
    let events = frame_system::Pallet::<T>::events();
    let system_event: <T as frame_system::Config>::RuntimeEvent = generic_event.into();
    // compare to the last event record
    let event_records: Vec<<T as frame_system::Config>::RuntimeEvent> =
        events.iter().map(|i| i.event.clone()).collect();
    assert!(event_records.contains(&system_event));
}

#[benchmarks]
mod benchmarks {
    use super::*;

    // worst case for new session.
    // TODO: this should be parametric over the config values:
    // * min_collators_for_orchestrator
    // * max_collators_for_orchestrator
    // * collators_per_container
    #[benchmark]
    fn new_session(x: Linear<1, 200>, y: Linear<1, 20>) -> Result<(), BenchmarkError> {
        frame_system::Pallet::<T>::set_block_number(0u32.into());

        let collators = invulnerables::<T>(x, SEED);
        let container_chains: Vec<_> = (0..y).map(ParaId::from).collect();
        let session_index = 0u32.into();
        T::ContainerChains::set_session_container_chains(session_index, &container_chains);
        T::RemoveParaIdsWithNoCredits::make_valid_para_ids(&container_chains);
        T::HostConfiguration::set_host_configuration(session_index);

        // Assign random collators to test worst case: when collators need to be checked against existing collators
        // In this case all of the old collators don't exist anymore
        let old_container_chains: Vec<(ParaId, _)> = (0..y)
            .map(|para_id| (para_id.into(), invulnerables::<T>(10, SEED + 2 + para_id)))
            .collect();

        let old_assigned = AssignedCollators {
            orchestrator_chain: invulnerables::<T>(100, SEED + 1),
            container_chains: BTreeMap::from_iter(old_container_chains),
        };
        <CollatorContainerChain<T>>::put(&old_assigned);
        // Do not use [0; 32] because that seed will not shuffle the list of collators
        // We use a different random seed every time to make sure that the event is included
        let random_seed = [x as u8; 32];
        <Randomness<T>>::put(random_seed);

        #[block]
        {
            <Pallet<T>>::initializer_on_new_session(&session_index, collators);
        }

        // Assignment changed
        assert_ne!(<CollatorContainerChain::<T>>::get(), old_assigned);
        // New assignment is not empty
        // If more than one, at least one chain should have gotten collators
        if x > 1 {
            assert_ne!(
                <CollatorContainerChain::<T>>::get().container_chains.len(),
                0
            );
        }

        // Worst case is `full_rotation: false` because it needs to check the previous assignment
        assert_event_is_present::<T>(
            Event::NewPendingAssignment {
                random_seed,
                full_rotation: false,
                target_session: T::SessionIndex::from(1u32),
            }
            .into(),
        );

        Ok(())
    }

    impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test,);
}
