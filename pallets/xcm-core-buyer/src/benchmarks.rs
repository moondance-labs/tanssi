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
    crate::{
        Call, Config, GetParathreadCollators, GetParathreadParams, InFlightOrders, Pallet,
        RelayXcmWeightConfig, RelayXcmWeightConfigInner,
    },
    core::marker::PhantomData,
    frame_benchmarking::{account, v2::*},
    frame_support::{assert_ok, pallet_prelude::Weight, BoundedBTreeSet},
    frame_system::RawOrigin,
    sp_std::{collections::btree_set::BTreeSet, vec},
    tp_traits::{ParaId, ParathreadParams, SlotFrequency},
};

pub const BUY_EXECUTION_COST: u128 = 50_000_000;
pub const PLACE_ORDER_WEIGHT_AT_MOST: Weight = Weight::from_parts(1_000_000_000, 100_000);

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn force_buy_core(x: Linear<1, 99>) {
        assert_ok!(Pallet::<T>::set_xcm_weights(
            RawOrigin::Root.into(),
            Some(RelayXcmWeightConfigInner {
                buy_execution_cost: BUY_EXECUTION_COST,
                weight_at_most: PLACE_ORDER_WEIGHT_AT_MOST,
                _phantom: PhantomData,
            }),
        ));

        let para_id = ParaId::from(x + 1);
        assert_eq!(InFlightOrders::<T>::get(), BTreeSet::new());

        // Mock `x` xcm messages already sent in this block
        let bbs: BoundedBTreeSet<ParaId, _> = BTreeSet::from_iter((0..x).map(ParaId::from))
            .try_into()
            .expect("x is greater than MaxParathreads");
        InFlightOrders::<T>::put(bbs);
        assert!(!InFlightOrders::<T>::get().contains(&para_id));

        // For the extrinsic to succeed, we need to ensure that:
        // * the para_id is a parathread
        // * it has assigned collators
        T::GetParathreadParams::set_parathread_params(
            para_id,
            Some(ParathreadParams {
                slot_frequency: SlotFrequency { min: 10, max: 10 },
            }),
        );
        let author: T::AccountId = account("account id", 0u32, 0u32);
        T::GetAssignedCollators::set_parathread_collators(para_id, vec![author]);

        #[extrinsic_call]
        Pallet::<T>::force_buy_core(RawOrigin::Root, para_id);

        assert!(InFlightOrders::<T>::get().contains(&para_id));
    }

    #[benchmark]
    fn set_xcm_weights() {
        let xcm_weights = RelayXcmWeightConfigInner {
            buy_execution_cost: BUY_EXECUTION_COST,
            weight_at_most: PLACE_ORDER_WEIGHT_AT_MOST,
            _phantom: PhantomData,
        };

        #[extrinsic_call]
        Pallet::<T>::set_xcm_weights(RawOrigin::Root, Some(xcm_weights.clone()));

        assert_eq!(RelayXcmWeightConfig::<T>::get(), Some(xcm_weights));
    }

    impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}
