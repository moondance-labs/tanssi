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
    crate::{Call, Config, Pallet},
    frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite},
    frame_system::RawOrigin,
    sp_std::vec,
};

benchmarks! {
	set_config_with_u32 {}: set_max_collators(RawOrigin::Root, 100)

	impl_benchmark_test_suite!(
		Pallet,
		crate::mock::new_test_ext(Default::default()),
		crate::mock::Test
	);
}

#[cfg(test)]
mod tests {
    use {super::*, crate::mock::Test, frame_support::assert_ok, sp_io::TestExternalities};

    pub fn new_test_ext() -> TestExternalities {
        let t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();
        TestExternalities::new(t)
    }

    #[test]
    fn bench_register() {
        new_test_ext().execute_with(|| {
            assert_ok!(Pallet::<Test>::test_benchmark_register());
        });
    }
}

impl_benchmark_test_suite!(
    Pallet,
    crate::benchmarks::tests::new_test_ext(),
    crate::mock::Test
);