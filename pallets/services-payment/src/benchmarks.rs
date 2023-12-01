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
    frame_support::{
        assert_ok,
        traits::{Currency, Get},
    },
    frame_system::RawOrigin,
    sp_std::prelude::*,
};

// Build genesis storage according to the mock runtime.
#[cfg(test)]
pub fn new_test_ext() -> sp_io::TestExternalities {
    const ALICE: u64 = 1;

    crate::mock::ExtBuilder::default()
        .with_balances(vec![(ALICE, 1_000)])
        .build()
}

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
        let caller = create_funded_user::<T>("caller", 1, 1000);
        let para_id = 1001u32.into();
        let credits = T::MaxCreditsStored::get();

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
        let caller = create_funded_user::<T>("caller", 1, 1000);
        let para_id = 1001u32.into();
        let credits = T::MaxCreditsStored::get();

        assert_ok!(Pallet::<T>::purchase_credits(
            RawOrigin::Signed(caller).into(),
            para_id,
            credits,
            Some(u32::MAX.into()),
        ));

        // Before call: 1000 credits
        assert_eq!(
            crate::BlockProductionCredits::<T>::get(&para_id).unwrap_or_default(),
            T::MaxCreditsStored::get()
        );

        #[extrinsic_call]
        Pallet::<T>::set_credits(RawOrigin::Root, para_id, 1u32.into());

        // After call: 1 credit
        assert_eq!(
            crate::BlockProductionCredits::<T>::get(&para_id).unwrap_or_default(),
            1u32.into()
        );
    }

    #[benchmark]
    fn set_given_free_credits() {
        let para_id = 1001u32.into();

        // Before call: no given free credits
        assert!(crate::GivenFreeCredits::<T>::get(&para_id).is_none());

        #[extrinsic_call]
        Pallet::<T>::set_given_free_credits(RawOrigin::Root, para_id, true);

        // After call: given free credits
        assert!(crate::GivenFreeCredits::<T>::get(&para_id).is_some());
    }

    impl_benchmark_test_suite!(Pallet, crate::benchmarks::new_test_ext(), crate::mock::Test);
}
