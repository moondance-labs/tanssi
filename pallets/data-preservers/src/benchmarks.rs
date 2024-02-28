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
    frame_benchmarking::v2::*,
    frame_support::{
        traits::{EnsureOriginWithArg, OriginTrait},
        BoundedVec,
    },
    frame_system::RawOrigin,
    sp_std::vec,
    tp_traits::ParaId,
};

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn set_boot_nodes(x: Linear<1, 200>, y: Linear<1, 10>) {
        // x: url len, y: num boot_nodes
        let boot_nodes = BoundedVec::try_from(vec![
            BoundedVec::try_from(vec![b'A'; x as usize])
                .unwrap();
            y as usize
        ])
        .unwrap();
        let para_id = ParaId::from(2);
        let origin = T::SetBootNodesOrigin::try_successful_origin(&para_id)
            .expect("failed to create SetBootNodesOrigin");
        // Worst case is when caller is not root
        let raw_origin = origin.as_system_ref();
        assert!(matches!(raw_origin, Some(RawOrigin::Signed(..))));

        #[extrinsic_call]
        Pallet::<T>::set_boot_nodes(origin as T::RuntimeOrigin, para_id, boot_nodes.clone());

        assert_eq!(Pallet::<T>::boot_nodes(para_id), boot_nodes);
    }

    impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}
