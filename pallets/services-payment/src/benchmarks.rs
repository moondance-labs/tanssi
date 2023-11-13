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
    crate::{BalanceOf, BlockNumberFor, Call, Config, Pallet},
    frame_benchmarking::{account, v2::*},
    frame_support::{assert_ok, traits::Currency},
    frame_system::RawOrigin,
    sp_std::prelude::*,
};

const SEED: u32 = 0;

fn create_funded_user<T: Config>(
    string: &'static str,
    n: u32,
    balance_factor: u32,
) -> T::AccountId {
    let user = account(string, n, SEED);
    let balance = <T::Currency>::minimum_balance() * balance_factor.into();
    let _ = <T::Currency>::make_free_balance_be(&user, balance);
    user
}

#[benchmarks(where BalanceOf<T>: From<BlockNumberFor<T>>)]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn purchase_credits() {
        let caller = create_funded_user::<T>("caller", 1, 100);
        let para_id = 1001u32.into();
        let credits = 1000u32.into();

        // Before call: 0 credits
        assert_eq!(
            crate::BlockProductionCredits::<T>::get(&para_id).unwrap_or_default(),
            0u32.into()
        );

        #[extrinsic_call]
        Pallet::<T>::purchase_credits(
            RawOrigin::Signed(caller),
            para_id,
            credits,
            Some(u32::MAX.into()),
        );

        // verification code
        assert_eq!(
            crate::BlockProductionCredits::<T>::get(&para_id).unwrap_or_default(),
            credits
        );
    }

    #[benchmark]
    fn set_credits() {
        let caller = create_funded_user::<T>("caller", 1, 100);
        let para_id = 1001u32.into();
        let credits = 1000u32.into();

        assert_ok!(Pallet::<T>::purchase_credits(
            RawOrigin::Signed(caller).into(),
            para_id,
            credits,
            Some(u32::MAX.into()),
        ));

        // Before call: 1000 credits
        assert_eq!(
            crate::BlockProductionCredits::<T>::get(&para_id).unwrap_or_default(),
            1000u32.into()
        );

        #[extrinsic_call]
        Pallet::<T>::set_credits(RawOrigin::Root, para_id, 1u32.into());

        // After call: 1 credit
        assert_eq!(
            crate::BlockProductionCredits::<T>::get(&para_id).unwrap_or_default(),
            1u32.into()
        );
    }

    impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}
