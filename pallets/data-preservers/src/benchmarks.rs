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
    frame_benchmarking::{account, v2::*},
    frame_support::{traits::Currency, BoundedVec},
    frame_system::RawOrigin,
    sp_core::Get,
    sp_std::{vec, vec::Vec},
    tp_traits::ParaId,
};

/*
/// Create a funded user.
/// Used for generating the necessary amount for registering
fn create_funded_user<T: Config>(
    string: &'static str,
    n: u32,
    extra: DepositBalanceOf<T>,
) -> (T::AccountId, DepositBalanceOf<T>) {
    const SEED: u32 = 0;
    let user = account(string, n, SEED);
    let min_reserve_amount = T::DepositAmount::get();
    let total = min_reserve_amount + extra;
    T::Currency::make_free_balance_be(&user, total);
    T::Currency::issue(total);
    (user, total)
}
*/

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn set_boot_nodes(x: Linear<1, 200>, y: Linear<1, 10>) {
        /*
        let storage = vec![(b"code".to_vec(), vec![1; x as usize]).into()];
        let storage = new_genesis_data(storage);

        let (caller, _deposit_amount) =
            create_funded_user::<T>("caller", 0, T::DepositAmount::get());

        Pallet::<T>::register(
            RawOrigin::Signed(caller.clone()).into(),
            Default::default(),
            storage,
        )
        .expect("Failed to register chain");

        // x: url len, y: num boot_nodes
        let boot_nodes = BoundedVec::try_from(vec![
            BoundedVec::try_from(vec![b'A'; x as usize])
                .unwrap();
            y as usize
        ])
        .unwrap();

        // Worst case is when caller is not root, need some way to register a container chain from here
        #[extrinsic_call]
        Pallet::<T>::set_boot_nodes(RawOrigin::Signed(caller), Default::default(), boot_nodes);
        */
        // x: url len, y: num boot_nodes
        let boot_nodes = BoundedVec::try_from(vec![
            BoundedVec::try_from(vec![b'A'; x as usize])
                .unwrap();
            y as usize
        ])
        .unwrap();

        #[extrinsic_call]
        Pallet::<T>::set_boot_nodes(RawOrigin::Root, Default::default(), boot_nodes);
    }

    impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}
