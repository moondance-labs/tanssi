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
    frame_support::{assert_ok, pallet_prelude::Weight, BoundedVec},
    frame_system::RawOrigin,
    sp_std::vec,
    tp_traits::{ParaId, ParathreadParams, SlotFrequency},
};

pub const BUY_EXECUTION_COST: u128 = 50_000_000;
pub const PLACE_ORDER_WEIGHT_AT_MOST: Weight = Weight::from_parts(1_000_000_000, 100_000);

#[benchmarks(where <T as frame_system::Config>::RuntimeOrigin: From<pallet_xcm::Origin>)]
mod benchmarks {
    use super::*;
    use crate::{InFlightCoreBuyingOrder, PendingBlocks, QueryIdToParaId};
    use frame_system::pallet_prelude::BlockNumberFor;
    use staging_xcm::latest::{MaybeErrorCode, QueryId};
    use staging_xcm::v3::{MultiLocation, Response};

    #[benchmark]
    fn force_buy_core() {
        assert_ok!(Pallet::<T>::set_relay_xcm_weight_config(
            RawOrigin::Root.into(),
            Some(RelayXcmWeightConfigInner {
                buy_execution_cost: BUY_EXECUTION_COST,
                weight_at_most: PLACE_ORDER_WEIGHT_AT_MOST,
                _phantom: PhantomData,
            }),
        ));

        let x = 1000u32;

        let para_id = ParaId::from(x + 1);
        for i in 0..=x {
            InFlightOrders::<T>::set(
                ParaId::from(i),
                Some(InFlightCoreBuyingOrder {
                    para_id: ParaId::from(i),
                    query_id: QueryId::from(i),
                    ttl: <frame_system::Pallet<T>>::block_number()
                        + BlockNumberFor::<T>::from(100u32),
                }),
            );

            QueryIdToParaId::<T>::set(QueryId::from(i), Some(ParaId::from(i)));
        }

        assert!(InFlightOrders::<T>::get(para_id).is_none());

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

        assert!(InFlightOrders::<T>::get(para_id).is_some());
    }

    #[benchmark]
    fn query_response() {
        let x = 1000u32;

        let para_id = ParaId::from(x);
        for i in 0..=x {
            InFlightOrders::<T>::set(
                ParaId::from(i),
                Some(InFlightCoreBuyingOrder {
                    para_id: ParaId::from(i),
                    query_id: QueryId::from(i),
                    ttl: <frame_system::Pallet<T>>::block_number()
                        + BlockNumberFor::<T>::from(100u32),
                }),
            );

            QueryIdToParaId::<T>::set(QueryId::from(i), Some(ParaId::from(i)));
        }

        assert!(InFlightOrders::<T>::get(para_id).is_some());

        let response = if x % 2 == 0 {
            Response::DispatchResult(MaybeErrorCode::Success)
        } else {
            Response::DispatchResult(MaybeErrorCode::Error(BoundedVec::default()))
        };
        let xcm_origin = pallet_xcm::Origin::Response(MultiLocation::here());

        #[extrinsic_call]
        Pallet::<T>::query_response(xcm_origin, QueryId::from(x), response);

        assert!(InFlightOrders::<T>::get(para_id).is_none());
        assert!(QueryIdToParaId::<T>::get(QueryId::from(x)).is_none());

        if x % 2 == 0 {
            assert!(PendingBlocks::<T>::get(para_id).is_some());
        } else {
            assert!(PendingBlocks::<T>::get(para_id).is_none());
        }
    }

    #[benchmark]
    fn clean_up_expired_in_flight_orders(x: Linear<1, 1000>) {
        let caller: T::AccountId = whitelisted_caller();
        for i in 0..=x {
            InFlightOrders::<T>::set(
                ParaId::from(i),
                Some(InFlightCoreBuyingOrder {
                    para_id: ParaId::from(i),
                    query_id: QueryId::from(i),
                    ttl: BlockNumberFor::<T>::from(0u32),
                }),
            );

            QueryIdToParaId::<T>::set(QueryId::from(i), Some(ParaId::from(i)));
        }

        let para_ids_to_clean_up = (0..=x).map(ParaId::from).collect();

        #[extrinsic_call]
        Pallet::<T>::clean_up_expired_in_flight_orders(
            RawOrigin::Signed(caller),
            para_ids_to_clean_up,
        );

        for i in 0..=x {
            assert!(InFlightOrders::<T>::get(ParaId::from(i)).is_none());
            assert!(QueryIdToParaId::<T>::get(QueryId::from(i)).is_none());
        }
    }

    #[benchmark]
    fn clean_up_expired_pending_blocks(x: Linear<1, 1000>) {
        let caller: T::AccountId = whitelisted_caller();
        for i in 0..=x {
            PendingBlocks::<T>::set(ParaId::from(i), Some(BlockNumberFor::<T>::from(0u32)));
        }

        let para_ids_to_clean_up = (0..=x).map(ParaId::from).collect();

        #[extrinsic_call]
        Pallet::<T>::clean_up_expired_pending_blocks(
            RawOrigin::Signed(caller),
            para_ids_to_clean_up,
        );

        for i in 0..=x {
            assert!(PendingBlocks::<T>::get(ParaId::from(i)).is_none());
        }
    }

    #[benchmark]
    fn set_relay_xcm_weight_config() {
        let xcm_weights = RelayXcmWeightConfigInner {
            buy_execution_cost: BUY_EXECUTION_COST,
            weight_at_most: PLACE_ORDER_WEIGHT_AT_MOST,
            _phantom: PhantomData,
        };

        #[extrinsic_call]
        Pallet::<T>::set_relay_xcm_weight_config(RawOrigin::Root, Some(xcm_weights.clone()));

        assert_eq!(RelayXcmWeightConfig::<T>::get(), Some(xcm_weights));
    }

    #[benchmark]
    fn set_relay_chain() {
        #[extrinsic_call]
        Pallet::<T>::set_relay_chain(RawOrigin::Root, Some(T::RelayChain::default()));
    }

    impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}
