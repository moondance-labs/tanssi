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
    crate::{Call, Config, DepositBalanceOf, Pallet},
    frame_benchmarking::{account, v2::*},
    frame_support::traits::Currency,
    frame_system::RawOrigin,
    sp_core::Get,
    sp_std::vec,
    tp_container_chain_genesis_data::ContainerChainGenesisData,
    tp_traits::ParaId,
};

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

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn register(x: Linear<5, 3_000_000>, y: Linear<1, 50>) {
        let storage = ContainerChainGenesisData {
            // Runtime would go under "code" key, so we mimic
            // with 4 byte key
            storage: vec![(vec![1; 4], vec![1; x as usize]).into()],
            name: Default::default(),
            id: Default::default(),
            fork_id: Default::default(),
            extensions: Default::default(),
            properties: Default::default(),
        };

        for i in 1..y {
            // Twice the deposit just in case
            let (caller, _deposit_amount) =
                create_funded_user::<T>("caller", i, T::DepositAmount::get());
            Pallet::<T>::register(
                RawOrigin::Signed(caller.clone()).into(),
                i.into(),
                storage.clone(),
            )
            .unwrap();
        }

        // We should have registered y-1
        assert_eq!(Pallet::<T>::pending_verification().len(), (y - 1) as usize);

        let (caller, _deposit_amount) =
            create_funded_user::<T>("caller", 0, T::DepositAmount::get());

        #[extrinsic_call]
        Pallet::<T>::register(
            RawOrigin::Signed(caller.clone()),
            Default::default(),
            storage.clone(),
        );

        // verification code
        assert_eq!(Pallet::<T>::pending_verification().len(), y as usize);
        assert!(Pallet::<T>::registrar_deposit(ParaId::default()).is_some());
    }

    #[benchmark]
    fn deregister(x: Linear<5, 3_000_000>, y: Linear<1, 50>) {
        let storage = ContainerChainGenesisData {
            // Runtime would go under "code" key, so we mimic
            // with 4 byte key
            storage: vec![(b"code".to_vec(), vec![1; x as usize]).into()],
            name: Default::default(),
            id: Default::default(),
            fork_id: Default::default(),
            extensions: Default::default(),
            properties: Default::default(),
        };

        for i in 0..y {
            // Twice the deposit just in case
            let (caller, _deposit_amount) =
                create_funded_user::<T>("caller", i, T::DepositAmount::get());
            Pallet::<T>::register(
                RawOrigin::Signed(caller.clone()).into(),
                i.into(),
                storage.clone(),
            )
            .unwrap();
        }

        // We should have registered y
        assert_eq!(Pallet::<T>::pending_verification().len(), y as usize);

        #[extrinsic_call]
        Pallet::<T>::deregister(RawOrigin::Root, (y - 1).into());

        // We should have y-1
        assert_eq!(Pallet::<T>::pending_verification().len(), (y - 1) as usize);
        assert!(Pallet::<T>::registrar_deposit(ParaId::from(y - 1)).is_none());
    }

    #[benchmark]
    fn mark_valid_for_collatings(x: Linear<5, 3_000_000>, y: Linear<1, 50>) {
        let storage = ContainerChainGenesisData {
            // Runtime would go under "code" key, so we mimic
            // with 4 byte key
            storage: vec![(vec![1; 4], vec![1; x as usize]).into()],
            name: Default::default(),
            id: Default::default(),
            fork_id: Default::default(),
            extensions: Default::default(),
            properties: Default::default(),
        };

        for i in 0..y {
            // Twice the deposit just in case
            let (caller, _deposit_amount) =
                create_funded_user::<T>("caller", i, T::DepositAmount::get());
            Pallet::<T>::register(
                RawOrigin::Signed(caller.clone()).into(),
                i.into(),
                storage.clone(),
            )
            .unwrap();
        }

        // We should have registered y
        assert_eq!(Pallet::<T>::pending_verification().len(), y as usize);

        #[extrinsic_call]
        Pallet::<T>::mark_valid_for_collating(RawOrigin::Root, (y - 1).into());

        // We should have y-1
        assert_eq!(Pallet::<T>::pending_verification().len(), (y - 1) as usize);
    }

    impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}