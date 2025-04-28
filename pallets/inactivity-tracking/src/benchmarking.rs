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

use {super::*, frame_benchmarking::v2::*, frame_support::dispatch::RawOrigin};
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

    impl_benchmark_test_suite!(
        InactivityTracking,
        crate::mock::ExtBuilder.build(),
        crate::mock::Test,
    );
}
