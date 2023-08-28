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
use crate::Pallet as CollatorSelection;
use frame_benchmarking::{account, impl_benchmark_test_suite, v2::*, BenchmarkError};
use frame_support::{
    codec::Decode,
    traits::{Currency, EnsureOrigin, Get},
};
use frame_system::{EventRecord, RawOrigin};
use pallet_session::{self as session, SessionManager};
use sp_std::prelude::*;

const SEED: u32 = 0;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
    let events = frame_system::Pallet::<T>::events();
    let system_event: <T as frame_system::Config>::RuntimeEvent = generic_event.into();
    // compare to the last event record
    let EventRecord { event, .. } = &events[events.len() - 1];
    assert_eq!(event, &system_event);
}

fn create_user<T: Config>(string: &'static str, n: u32) -> T::AccountId {
    account(string, n, SEED)
}

fn invulnerable<T: Config + frame_system::Config>(c: u32) -> T::AccountId {
    create_user::<T>("candidate", c)
}

fn invulnerables<T: Config + frame_system::Config>(count: u32) -> Vec<T::AccountId> {
    (0..count).map(|c| invulnerable::<T>(c)).collect::<Vec<_>>()
}

fn create_funded_user<T: Config>(
    string: &'static str,
    n: u32,
    balance_factor: u32,
) -> T::AccountId {
    let user = account(string, n, SEED);
    let balance = T::Currency::minimum_balance() * balance_factor.into();
    let _ = T::Currency::make_free_balance_be(&user, balance);
    user
}

fn keys<T: Config + session::Config>(c: u32) -> <T as session::Config>::Keys {
    use rand::{RngCore, SeedableRng};

    let keys = {
        let mut keys = [0u8; 128];

        if c > 0 {
            let mut rng = rand::rngs::StdRng::seed_from_u64(c as u64);
            rng.fill_bytes(&mut keys);
        }

        keys
    };

    Decode::decode(&mut &keys[..]).unwrap()
}

fn validator<T: Config + session::Config>(c: u32) -> (T::AccountId, <T as session::Config>::Keys) {
    (create_funded_user::<T>("candidate", c, 1000), keys::<T>(c))
}

fn register_validators<T: Config + session::Config>(count: u32) -> Vec<T::AccountId> {
    let validators = (0..count).map(|c| validator::<T>(c)).collect::<Vec<_>>();

    for (who, keys) in validators.clone() {
        <session::Pallet<T>>::set_keys(RawOrigin::Signed(who).into(), keys, Vec::new()).unwrap();
    }

    validators.into_iter().map(|(who, _)| who).collect()
}

#[benchmarks(where T: session::Config)]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn set_invulnerables(
        b: Linear<1, { T::MaxInvulnerables::get() }>,
    ) -> Result<(), BenchmarkError> {
        let origin =
            T::UpdateOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?;

        let new_invulnerables = invulnerables::<T>(b);
        let mut sorted_new_invulnerables = new_invulnerables.clone();
        sorted_new_invulnerables.sort();

        #[extrinsic_call]
        _(origin as T::RuntimeOrigin, new_invulnerables);

        // assert that it comes out sorted
        assert_last_event::<T>(
            Event::NewInvulnerables {
                invulnerables: sorted_new_invulnerables,
            }
            .into(),
        );
        Ok(())
    }

    #[benchmark]
    fn add_invulnerable(
        b: Linear<1, { T::MaxInvulnerables::get() - 1 }>,
    ) -> Result<(), BenchmarkError> {
        let origin =
            T::UpdateOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?;

        // now we need to fill up invulnerables
        let mut invulnerables = invulnerables::<T>(b);
        invulnerables.sort();
        let invulnerables: frame_support::BoundedVec<_, T::MaxInvulnerables> =
            frame_support::BoundedVec::try_from(invulnerables).unwrap();
        <Invulnerables<T>>::put(invulnerables);

        let new_invulnerable = invulnerable::<T>(b + 1);

        #[extrinsic_call]
        _(origin as T::RuntimeOrigin, new_invulnerable.clone());

        assert_last_event::<T>(
            Event::InvulnerableAdded {
                account_id: new_invulnerable,
            }
            .into(),
        );
        Ok(())
    }

    #[benchmark]
    fn remove_invulnerable(
        b: Linear<{ 1 }, { T::MaxInvulnerables::get() }>,
    ) -> Result<(), BenchmarkError> {
        let origin =
            T::UpdateOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?;
        let mut invulnerables = invulnerables::<T>(b);
        invulnerables.sort();
        let invulnerables: frame_support::BoundedVec<_, T::MaxInvulnerables> =
            frame_support::BoundedVec::try_from(invulnerables).unwrap();
        <Invulnerables<T>>::put(invulnerables);
        let to_remove = <Invulnerables<T>>::get().first().unwrap().clone();

        #[extrinsic_call]
        _(origin as T::RuntimeOrigin, to_remove.clone());

        assert_last_event::<T>(
            Event::InvulnerableRemoved {
                account_id: to_remove,
            }
            .into(),
        );
        Ok(())
    }

    // worst case for new session.
    #[benchmark]
    fn new_session(r: Linear<1, { T::MaxInvulnerables::get() }>) {
        frame_system::Pallet::<T>::set_block_number(0u32.into());
        register_validators::<T>(r);
        // now we need to fill up invulnerables
        let mut invulnerables = invulnerables::<T>(r);
        invulnerables.sort();
        let invulnerables: frame_support::BoundedVec<_, T::MaxInvulnerables> =
            frame_support::BoundedVec::try_from(invulnerables).unwrap();
        <Invulnerables<T>>::put(invulnerables);

        #[block]
        {
            <CollatorSelection<T> as SessionManager<_>>::new_session(0);
        }
    }

    impl_benchmark_test_suite!(
        CollatorSelection,
        crate::mock::new_test_ext(),
        crate::mock::Test,
    );
}
