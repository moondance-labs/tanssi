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

#![cfg(feature = "runtime-benchmarks")]

//! Benchmarking
use {
    crate::{Config, Pallet},
    frame_benchmarking::v2::*,
    frame_system::RawOrigin,
};

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn set_config_with_u32() -> Result<(), BenchmarkError> {
        #[block]
        {
            Pallet::<T>::set_max_collators(RawOrigin::Root.into(), 100).expect("to return Ok");
        }

        Ok(())
    }

    impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}
