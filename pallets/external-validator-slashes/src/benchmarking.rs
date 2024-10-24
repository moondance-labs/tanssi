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

//! Benchmarking setup for pallet-external-validator-slashes

use super::*;

#[allow(unused)]
use crate::Pallet as ExternalValidatorSlashes;
use {
    frame_benchmarking::{account, v2::*, BenchmarkError},
    frame_support::{
        pallet_prelude::*,
        traits::{tokens::fungible::Balanced, Currency, EnsureOrigin, Get},
    },
    frame_system::{EventRecord, RawOrigin},
    pallet_session::{self as session, SessionManager},
    sp_runtime::traits::{AtLeast32BitUnsigned, Convert, TrailingZeroInput},
    sp_std::prelude::*,
};

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
    let events = frame_system::Pallet::<T>::events();
    let system_event: <T as frame_system::Config>::RuntimeEvent = generic_event.into();
    // compare to the last event record
    let EventRecord { event, .. } = &events[events.len() - 1];
    assert_eq!(event, &system_event);
}

const MAX_SLASHES: u32 = 1000;
const SEED: u32 = 0;

#[allow(clippy::multiple_bound_locations)]
#[benchmarks(where T: session::Config)]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn cancel_deferred_slash(s: Linear<1, MAX_SLASHES>) -> Result<(), BenchmarkError> {
        let mut existing_slashes = Vec::new();
        let era = EraIndex::one();
        let dummy = || T::AccountId::decode(&mut TrailingZeroInput::zeroes()).unwrap();
        for _ in 0..MAX_SLASHES {
            existing_slashes.push(Slash::<T::AccountId, T::SlashId>::default_from(dummy()));
        }
        Slashes::<T>::insert(era, &existing_slashes);
        let slash_indices: Vec<u32> = (0..s).collect();

        #[extrinsic_call]
        _(RawOrigin::Root, era, slash_indices);

        assert_eq!(Slashes::<T>::get(&era).len(), (MAX_SLASHES - s) as usize);
        Ok(())
    }

    #[benchmark]
    fn force_inject_slash(s: Linear<1, MAX_SLASHES>) -> Result<(), BenchmarkError> {
        let mut existing_slashes = Vec::new();
        let era = T::EraIndexProvider::active_era().index;
        let dummy = || T::AccountId::decode(&mut TrailingZeroInput::zeroes()).unwrap();
        for _ in 0..s - 1 {
            existing_slashes.push(Slash::<T::AccountId, T::SlashId>::default_from(dummy()));
        }
        Slashes::<T>::insert(era, &existing_slashes);

        #[extrinsic_call]
        _(RawOrigin::Root, era, dummy(), Perbill::from_percent(50));

        assert_eq!(Slashes::<T>::get(&era).len(), s as usize);
        Ok(())
    }

    impl_benchmark_test_suite!(
        ExternalValidatorSlashes,
        crate::mock::new_test_ext(),
        crate::mock::Test,
    );
}
