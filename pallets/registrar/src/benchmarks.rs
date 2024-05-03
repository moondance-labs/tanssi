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
    crate::{Call, Config, DepositBalanceOf, Pallet, RegistrarHooks},
    frame_benchmarking::{account, v2::*},
    frame_support::traits::Currency,
    frame_system::RawOrigin,
    sp_core::Get,
    sp_std::{vec, vec::Vec},
    tp_container_chain_genesis_data::{ContainerChainGenesisData, ContainerChainGenesisDataItem},
    tp_traits::{ParaId, SlotFrequency},
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
    let _ = T::Currency::issue(total);
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

    // Returns number of para ids in pending verification (registered but not marked as valid)
    fn pending_verification_len<T: Config>() -> usize {
        crate::PendingVerification::<T>::iter_keys().count()
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
        assert_eq!(pending_verification_len::<T>(), (y - 1) as usize);

        let (caller, _deposit_amount) =
            create_funded_user::<T>("caller", 0, T::DepositAmount::get());

        #[extrinsic_call]
        Pallet::<T>::register(RawOrigin::Signed(caller), Default::default(), storage);

        // verification code
        assert_eq!(pending_verification_len::<T>(), y as usize);
        assert!(Pallet::<T>::registrar_deposit(ParaId::default()).is_some());
    }

    #[benchmark]
    fn deregister_immediate(x: Linear<5, 3_000_000>, y: Linear<1, 50>) {
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
            // Do not call mark_valid_for_collating, to ensure that the deregister call also executes the cleanup hooks
        }

        // We should have registered y
        assert_eq!(pending_verification_len::<T>(), y as usize);
        assert!(Pallet::<T>::registrar_deposit(ParaId::from(y - 1)).is_some());

        #[extrinsic_call]
        Pallet::<T>::deregister(RawOrigin::Root, (y - 1).into());

        // We should have y-1
        assert_eq!(pending_verification_len::<T>(), (y - 1) as usize);
        assert!(Pallet::<T>::registrar_deposit(ParaId::from(y - 1)).is_none());
    }

    #[benchmark]
    fn deregister_scheduled(x: Linear<5, 3_000_000>, y: Linear<1, 50>) {
        let storage = vec![(b"code".to_vec(), vec![1; x as usize]).into()];
        let storage = new_genesis_data(storage);
        let genesis_para_id_len = Pallet::<T>::registered_para_ids().len();

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
            // Call mark_valid_for_collating to ensure that the deregister call
            // does not execute the cleanup hooks immediately
            T::RegistrarHooks::benchmarks_ensure_valid_for_collating(i.into());
            Pallet::<T>::mark_valid_for_collating(RawOrigin::Root.into(), i.into()).unwrap();
        }

        // Start a new session
        Pallet::<T>::initializer_on_new_session(&T::SessionDelay::get());
        // We should have registered y
        assert_eq!(
            Pallet::<T>::registered_para_ids().len(),
            genesis_para_id_len + y as usize
        );
        assert!(Pallet::<T>::registrar_deposit(ParaId::from(y - 1)).is_some());

        #[extrinsic_call]
        Pallet::<T>::deregister(RawOrigin::Root, (y - 1).into());

        // We now have y - 1 but the deposit has not been removed yet
        assert_eq!(
            Pallet::<T>::pending_registered_para_ids()[0].1.len(),
            genesis_para_id_len + (y - 1) as usize
        );
        assert!(Pallet::<T>::registrar_deposit(ParaId::from(y - 1)).is_some());

        // Start a new session
        Pallet::<T>::initializer_on_new_session(&T::SessionDelay::get());

        // Now it has been removed
        assert_eq!(
            Pallet::<T>::registered_para_ids().len(),
            genesis_para_id_len + (y - 1) as usize
        );
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
            T::RegistrarHooks::benchmarks_ensure_valid_for_collating(k.into());
            Pallet::<T>::mark_valid_for_collating(RawOrigin::Root.into(), k.into()).unwrap();
        }

        // Start a new session
        Pallet::<T>::initializer_on_new_session(&T::SessionDelay::get());

        // We should have registered y
        assert_eq!(pending_verification_len::<T>(), y as usize);
        T::RegistrarHooks::benchmarks_ensure_valid_for_collating((y - 1).into());

        #[extrinsic_call]
        Pallet::<T>::mark_valid_for_collating(RawOrigin::Root, (y - 1).into());

        // We should have y-1
        assert_eq!(pending_verification_len::<T>(), (y - 1) as usize);
    }

    #[benchmark]
    fn pause_container_chain(y: Linear<1, 50>) {
        let storage = vec![(vec![1; 4], vec![1; 3_000_000usize]).into()];
        let storage = new_genesis_data(storage);

        // Deregister all the existing chains to avoid conflicts with the new ones
        for para_id in Pallet::<T>::registered_para_ids() {
            Pallet::<T>::deregister(RawOrigin::Root.into(), para_id).unwrap();
        }

        // Worst case: when RegisteredParaIds and Paused are both full
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
            T::RegistrarHooks::benchmarks_ensure_valid_for_collating(i.into());
            Pallet::<T>::mark_valid_for_collating(RawOrigin::Root.into(), i.into()).unwrap();
        }

        // Second loop to fill Paused to its maximum
        for k in 1000..(1000 + y) {
            let (caller, _deposit_amount) =
                create_funded_user::<T>("caller", k, T::DepositAmount::get());
            Pallet::<T>::register(
                RawOrigin::Signed(caller.clone()).into(),
                k.into(),
                storage.clone(),
            )
            .unwrap();
            T::RegistrarHooks::benchmarks_ensure_valid_for_collating(k.into());
            Pallet::<T>::mark_valid_for_collating(RawOrigin::Root.into(), k.into()).unwrap();
            Pallet::<T>::pause_container_chain(RawOrigin::Root.into(), k.into()).unwrap();
        }

        // Check PendingPaused has a length of y
        assert_eq!(Pallet::<T>::pending_paused()[0].1.len(), y as usize);
        // Check y-1 is not in PendingPaused
        assert!(!Pallet::<T>::pending_paused()[0]
            .1
            .contains(&ParaId::from(y - 1)));
        // Check y-1 is in pending_registered_para_ids
        assert!(Pallet::<T>::pending_registered_para_ids()[0]
            .1
            .contains(&ParaId::from(y - 1)));

        #[extrinsic_call]
        Pallet::<T>::pause_container_chain(RawOrigin::Root, (y - 1).into());

        // Start a new session
        Pallet::<T>::initializer_on_new_session(&T::SessionDelay::get());

        // Check y-1 is in Paused
        assert!(Pallet::<T>::paused().contains(&ParaId::from(y - 1)));
        // Check y-1 is not in registered_para_ids
        assert!(!Pallet::<T>::registered_para_ids().contains(&ParaId::from(y - 1)));
    }

    #[benchmark]
    fn unpause_container_chain(y: Linear<1, 50>) {
        let storage = vec![(vec![1; 4], vec![1; 3_000_000usize]).into()];
        let storage = new_genesis_data(storage);

        // Deregister all the existing chains to avoid conflicts with the new ones
        for para_id in Pallet::<T>::registered_para_ids() {
            Pallet::<T>::deregister(RawOrigin::Root.into(), para_id).unwrap();
        }

        // Worst case: when RegisteredParaIds and Paused are both full
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
            T::RegistrarHooks::benchmarks_ensure_valid_for_collating(i.into());
            Pallet::<T>::mark_valid_for_collating(RawOrigin::Root.into(), i.into()).unwrap();
        }

        // Second loop to fill Paused to its maximum
        for k in 1000..(1000 + y) {
            let (caller, _deposit_amount) =
                create_funded_user::<T>("caller", k, T::DepositAmount::get());
            Pallet::<T>::register(
                RawOrigin::Signed(caller.clone()).into(),
                k.into(),
                storage.clone(),
            )
            .unwrap();
            T::RegistrarHooks::benchmarks_ensure_valid_for_collating(k.into());
            Pallet::<T>::mark_valid_for_collating(RawOrigin::Root.into(), k.into()).unwrap();
            Pallet::<T>::pause_container_chain(RawOrigin::Root.into(), k.into()).unwrap();
        }

        // Check PendingPaused has a length of y
        assert_eq!(Pallet::<T>::pending_paused()[0].1.len(), y as usize);
        // Check 1000 is in PendingPaused
        assert!(Pallet::<T>::pending_paused()[0]
            .1
            .contains(&ParaId::from(1000)));
        // Check 1000 is not in pending_registered_para_ids
        assert!(!Pallet::<T>::pending_registered_para_ids()[0]
            .1
            .contains(&ParaId::from(1000)));

        #[extrinsic_call]
        Pallet::<T>::unpause_container_chain(RawOrigin::Root, 1000u32.into());

        // Start a new session
        Pallet::<T>::initializer_on_new_session(&T::SessionDelay::get());

        // Check 1000 is not in Paused
        assert!(!Pallet::<T>::paused().contains(&ParaId::from(1000)));
        // Check 1000 is in registered_para_ids
        assert!(Pallet::<T>::registered_para_ids().contains(&ParaId::from(1000)));
    }

    #[benchmark]
    fn register_parathread(x: Linear<5, 3_000_000>, y: Linear<1, 50>, z: Linear<1, 10>) {
        let mut data = vec![];
        // Number of keys
        for _i in 1..z {
            data.push((b"code".to_vec(), vec![1; (x / z) as usize]).into())
        }

        let slot_frequency = SlotFrequency::default();
        let storage = new_genesis_data(data);

        for i in 1..y {
            // Twice the deposit just in case
            let (caller, _deposit_amount) =
                create_funded_user::<T>("caller", i, T::DepositAmount::get());
            Pallet::<T>::register_parathread(
                RawOrigin::Signed(caller.clone()).into(),
                i.into(),
                slot_frequency.clone(),
                storage.clone(),
            )
            .unwrap();
        }

        // We should have registered y-1
        assert_eq!(pending_verification_len::<T>(), (y - 1) as usize);

        let (caller, _deposit_amount) =
            create_funded_user::<T>("caller", 0, T::DepositAmount::get());

        #[extrinsic_call]
        Pallet::<T>::register_parathread(
            RawOrigin::Signed(caller),
            Default::default(),
            slot_frequency,
            storage,
        );

        // verification code
        assert_eq!(pending_verification_len::<T>(), y as usize);
        assert!(Pallet::<T>::registrar_deposit(ParaId::default()).is_some());
    }

    #[benchmark]
    fn set_parathread_params(y: Linear<1, 50>) {
        let storage = vec![(vec![1; 4], vec![1; 3_000_000usize]).into()];
        let storage = new_genesis_data(storage);
        let slot_frequency = SlotFrequency::default();

        // Deregister all the existing chains to avoid conflicts with the new ones
        for para_id in Pallet::<T>::registered_para_ids() {
            Pallet::<T>::deregister(RawOrigin::Root.into(), para_id).unwrap();
        }

        for i in 0..y {
            // Twice the deposit just in case
            let (caller, _deposit_amount) =
                create_funded_user::<T>("caller", i, T::DepositAmount::get());
            Pallet::<T>::register_parathread(
                RawOrigin::Signed(caller.clone()).into(),
                i.into(),
                slot_frequency.clone(),
                storage.clone(),
            )
            .unwrap();
            T::RegistrarHooks::benchmarks_ensure_valid_for_collating(i.into());
            Pallet::<T>::mark_valid_for_collating(RawOrigin::Root.into(), i.into()).unwrap();
        }

        let new_slot_frequency = SlotFrequency { min: 2, max: 2 };

        #[extrinsic_call]
        Pallet::<T>::set_parathread_params(
            RawOrigin::Root,
            (y - 1).into(),
            new_slot_frequency.clone(),
        );

        // Start a new session
        Pallet::<T>::initializer_on_new_session(&T::SessionDelay::get());

        // Check y-1 has new slot frequency
        assert_eq!(
            Pallet::<T>::parathread_params(ParaId::from(y - 1)).map(|x| x.slot_frequency),
            Some(new_slot_frequency)
        );
    }

    impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}
