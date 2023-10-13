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
    frame_support::{traits::Currency, BoundedVec},
    frame_system::RawOrigin,
    sp_core::Get,
    sp_std::{vec, vec::Vec},
    tp_container_chain_genesis_data::{ContainerChainGenesisData, ContainerChainGenesisDataItem},
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

    fn new_genesis_data<T: Get<u32>>(
        storage: Vec<ContainerChainGenesisDataItem>,
    ) -> ContainerChainGenesisData<T> {
        ContainerChainGenesisData {
            storage,
            name: Default::default(),
            id: Default::default(),
            fork_id: Default::default(),
            extensions: Default::default(),
            properties: Default::default(),
        }
    }

    #[benchmark]
    fn register(x: Linear<5, 3_000_000>, y: Linear<1, 50>, z: Linear<1, 10>) {
        let mut data = vec![];
        // Number of keys
        for _i in 1..z {
            data.push((b"code".to_vec(), vec![1; (x / z) as usize]).into())
        }

        let storage = new_genesis_data(data);

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
        Pallet::<T>::register(RawOrigin::Signed(caller), Default::default(), storage);

        // verification code
        assert_eq!(Pallet::<T>::pending_verification().len(), y as usize);
        assert!(Pallet::<T>::registrar_deposit(ParaId::default()).is_some());
    }

    #[benchmark]
    fn deregister(x: Linear<5, 3_000_000>, y: Linear<1, 50>) {
        let storage = vec![(b"code".to_vec(), vec![1; x as usize]).into()];
        let storage = new_genesis_data(storage);

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
    fn mark_valid_for_collating(y: Linear<1, 50>) {
        let storage = vec![(vec![1; 4], vec![1; 3_000_000usize]).into()];
        let storage = new_genesis_data(storage);

        // Worst case: when RegisteredParaIds and PendingVerification are both full
        // First loop to fill PendingVerification to its maximum
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

        // Second loop to fill RegisteredParaIds to its maximum
        for k in 1000..(1000 + y) {
            // Twice the deposit just in case
            let (caller, _deposit_amount) =
                create_funded_user::<T>("caller", k, T::DepositAmount::get());
            Pallet::<T>::register(
                RawOrigin::Signed(caller.clone()).into(),
                k.into(),
                storage.clone(),
            )
            .unwrap();
            Pallet::<T>::mark_valid_for_collating(RawOrigin::Root.into(), k.into()).unwrap();
        }

        // Start a new session
        Pallet::<T>::initializer_on_new_session(&T::SessionDelay::get());

        // We should have registered y
        assert_eq!(Pallet::<T>::pending_verification().len(), y as usize);

        #[extrinsic_call]
        Pallet::<T>::mark_valid_for_collating(RawOrigin::Root, (y - 1).into());

        // We should have y-1
        assert_eq!(Pallet::<T>::pending_verification().len(), (y - 1) as usize);
    }

    #[benchmark]
    fn set_boot_nodes(x: Linear<1, 200>, y: Linear<1, 10>) {
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

        #[extrinsic_call]
        Pallet::<T>::set_boot_nodes(RawOrigin::Signed(caller), Default::default(), boot_nodes);
    }

    #[benchmark]
    fn pause_container_chain(y: Linear<1, 50>) {
        let storage = vec![(vec![1; 4], vec![1; 3_000_000usize]).into()];
        let storage = new_genesis_data(storage);

        // Deregister all the existing chains to avoid conflicts with the new ones
        for para_id in Pallet::<T>::registered_para_ids() {
            Pallet::<T>::deregister(RawOrigin::Root.into(), para_id).unwrap();
        }

        // Worst case: when RegisteredParaIds and PendingVerification are both full
        // First loop to fill RegisteredParaIds to its maximum
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
            Pallet::<T>::mark_valid_for_collating(RawOrigin::Root.into(), i.into()).unwrap();
        }

        // Second loop to fill PendingVerification to its maximum
        for k in 1000..(1000 + y) {
            let (caller, _deposit_amount) =
                create_funded_user::<T>("caller", k, T::DepositAmount::get());
            Pallet::<T>::register(
                RawOrigin::Signed(caller.clone()).into(),
                k.into(),
                storage.clone(),
            )
            .unwrap();
        }

        // Check PendingParaIds has a length of y
        assert_eq!(
            Pallet::<T>::pending_registered_para_ids()[0].1.len(),
            y as usize
        );

        // Start a new session
        Pallet::<T>::initializer_on_new_session(&T::SessionDelay::get());

        // Check y-1 is not in PendingVerification
        assert!(!Pallet::<T>::pending_verification().contains(&ParaId::from(y - 1)));

        #[extrinsic_call]
        Pallet::<T>::pause_container_chain(RawOrigin::Root, (y - 1).into());

        // y-1 should be included again in PendingVerification
        assert!(Pallet::<T>::pending_verification().contains(&ParaId::from(y - 1)));

        // y-1 should not be in PendingParaIds
        assert_eq!(
            Pallet::<T>::pending_registered_para_ids()[0].1.len(),
            (y - 1) as usize
        );
    }

    impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}
