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
#[allow(unused)]
use crate::Pallet as InactivityTracking;

use {
    super::*,
    frame_benchmarking::{account, v2::*},
    frame_support::dispatch::RawOrigin,
};

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn set_inactivity_tracking_status() -> Result<(), BenchmarkError> {
        <T as crate::pallet::Config>::CurrentSessionIndex::skip_to_session(1);
        #[extrinsic_call]
        _(RawOrigin::Root, false);

        Ok(())
    }

    #[benchmark]
    fn enable_offline_marking() -> Result<(), BenchmarkError> {
        #[extrinsic_call]
        _(RawOrigin::Root, true);

        Ok(())
    }
    #[benchmark]
    fn set_offline() -> Result<(), BenchmarkError> {
        const USER_SEED: u32 = 1;
        let caller: T::AccountId = account("caller", 2, USER_SEED);
        T::CollatorStakeHelper::make_collator_eligible_candidate(&caller);
        InactivityTracking::<T>::enable_offline_marking(RawOrigin::Root.into(), true)?;

        #[extrinsic_call]
        _(RawOrigin::Signed(caller));

        Ok(())
    }

    #[benchmark]
    fn set_online() -> Result<(), BenchmarkError> {
        const USER_SEED: u32 = 1;
        let caller: T::AccountId = account("caller", 2, USER_SEED);
        T::CollatorStakeHelper::make_collator_eligible_candidate(&caller);
        InactivityTracking::<T>::enable_offline_marking(RawOrigin::Root.into(), true)?;
        InactivityTracking::<T>::set_offline(RawOrigin::Signed(caller.clone()).into())?;

        #[extrinsic_call]
        _(RawOrigin::Signed(caller));

        Ok(())
    }

    #[benchmark]
    fn notify_inactive_collator() -> Result<(), BenchmarkError> {
        const USER_SEED: u32 = 1;
        let caller: T::AccountId = account("caller", 2, USER_SEED);
        let collator: T::AccountId = account("collator", 3, USER_SEED);
        T::CollatorStakeHelper::make_collator_eligible_candidate(&collator);
        InactivityTracking::<T>::enable_offline_marking(RawOrigin::Root.into(), true)?;
        InactivityTracking::<T>::make_node_inactive(&collator);

        #[extrinsic_call]
        _(RawOrigin::Signed(caller), collator);

        Ok(())
    }

    impl_benchmark_test_suite!(
        InactivityTracking,
        crate::mock::ExtBuilder.build(),
        crate::mock::Test,
    );
}
