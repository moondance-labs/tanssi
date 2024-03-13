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
use crate::InFlightOrders;
use sp_std::collections::btree_set::BTreeSet;
use {
    crate::{Call, Config, Pallet},
    frame_benchmarking::v2::*,
    frame_system::RawOrigin,
    tp_traits::ParaId,
};

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn force_buy_core(x: Linear<1, 200>, y: Linear<1, 10>) {
        let para_id = ParaId::from(3333);
        assert_eq!(InFlightOrders::<T>::get(), BTreeSet::new());

        // TODO: need to add benchmark methods to config traits, to ensure that:
        // * the para_id is a parathread
        // * and to assign collators to that para_id

        #[extrinsic_call]
        Pallet::<T>::force_buy_core(RawOrigin::Root, para_id);

        assert_eq!(InFlightOrders::<T>::get(), BTreeSet::from_iter([para_id]));
    }

    impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}
