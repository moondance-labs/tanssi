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
        Call, Config, DepositBalanceOf, EnsureSignedByManager, Pallet, ParaInfo, RegistrarHooks,
        REGISTRAR_PARAS_INDEX,
    },
    cumulus_test_relay_sproof_builder::RelayStateSproofBuilder,
    frame_benchmarking::{account, v2::*},
    frame_support::{
        traits::{Currency, EnsureOriginWithArg},
        Hashable,
    },
    frame_system::RawOrigin,
    sp_core::{ed25519, Encode, Get, Pair},
    sp_std::{vec, vec::Vec},
    tp_container_chain_genesis_data::{ContainerChainGenesisData, ContainerChainGenesisDataItem},
    tp_traits::{ParaId, RelayStorageRootProvider, SlotFrequency},
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

fn get_ed25519_pairs(num: u32) -> Vec<ed25519::Pair> {
    let seed: u128 = 12345678901234567890123456789012;
    let mut pairs = Vec::new();
    for i in 0..num {
        pairs.push(ed25519::Pair::from_seed(
            (seed + u128::from(i))
                .to_string()
                .as_bytes()
                .try_into()
                .unwrap(),
        ))
    }
    pairs
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
    fn register_with_proof(x: Linear<5, 3_000_000>, y: Linear<1, 50>, z: Linear<1, 10>) {
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

        let pairs = get_ed25519_pairs(1);
        let mut sproof = RelayStateSproofBuilder::default();
        let para_id: ParaId = 42.into();
        let bytes = para_id.twox_64_concat();
        let key = [REGISTRAR_PARAS_INDEX, bytes.as_slice()].concat();
        let para_info: ParaInfo<
            cumulus_primitives_core::relay_chain::AccountId,
            cumulus_primitives_core::relay_chain::Balance,
        > = ParaInfo {
            manager: pairs[0].public().into(),
            deposit: Default::default(),
            locked: None,
        };
        sproof.additional_key_values = vec![(key, para_info.encode())];
        let (relay_parent_storage_root, proof) = sproof.into_state_root_and_proof();

        T::RelayStorageRootProvider::set_relay_storage_root(1, Some(relay_parent_storage_root));

        let signature_msg = Pallet::<T>::relay_signature_msg(
            Default::default(),
            &caller,
            relay_parent_storage_root,
        );
        let signature: cumulus_primitives_core::relay_chain::Signature =
            pairs[0].sign(&signature_msg).into();

        #[extrinsic_call]
        Pallet::<T>::register_with_relay_proof(
            RawOrigin::Signed(caller),
            Default::default(),
            None,
            1,
            proof,
            signature,
            storage,
        );

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
    fn deregister_with_relay_proof_immediate(x: Linear<5, 3_000_000>, y: Linear<1, 50>) {
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

        let (caller, _deposit_amount) =
            create_funded_user::<T>("caller", 0, T::DepositAmount::get());

        let (relay_parent_storage_root, proof) =
            RelayStateSproofBuilder::default().into_state_root_and_proof();

        T::RelayStorageRootProvider::set_relay_storage_root(1, Some(relay_parent_storage_root));

        #[extrinsic_call]
        Pallet::<T>::deregister_with_relay_proof(
            RawOrigin::Signed(caller),
            (y - 1).into(),
            1,
            proof,
        );

        // We should have y-1
        assert_eq!(pending_verification_len::<T>(), (y - 1) as usize);
        assert!(Pallet::<T>::registrar_deposit(ParaId::from(y - 1)).is_none());
    }

    #[benchmark]
    fn deregister_with_relay_proof_scheduled(x: Linear<5, 3_000_000>, y: Linear<1, 50>) {
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

        let (caller, _deposit_amount) =
            create_funded_user::<T>("caller", 0, T::DepositAmount::get());

        let (relay_parent_storage_root, proof) =
            RelayStateSproofBuilder::default().into_state_root_and_proof();

        T::RelayStorageRootProvider::set_relay_storage_root(1, Some(relay_parent_storage_root));

        #[extrinsic_call]
        Pallet::<T>::deregister_with_relay_proof(
            RawOrigin::Signed(caller),
            (y - 1).into(),
            1,
            proof,
        );

        // We now have y - 1 and the deposit has been removed
        assert_eq!(
            Pallet::<T>::pending_registered_para_ids()[0].1.len(),
            genesis_para_id_len + (y - 1) as usize
        );
        assert!(Pallet::<T>::registrar_deposit(ParaId::from(y - 1)).is_none());

        // Start a new session
        Pallet::<T>::initializer_on_new_session(&T::SessionDelay::get());

        // Now it has been removed
        assert_eq!(
            Pallet::<T>::registered_para_ids().len(),
            genesis_para_id_len + (y - 1) as usize
        );
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

    #[benchmark]
    fn set_para_manager() {
        let para_id = 1001u32.into();

        let origin = EnsureSignedByManager::<T>::try_successful_origin(&para_id)
            .expect("failed to create ManagerOrigin");

        let manager_address = account("sufficient", 0, 1000);

        // Before call: not set as manager
        assert_ne!(
            crate::ParaManager::<T>::get(para_id).as_ref(),
            Some(&manager_address)
        );

        #[extrinsic_call]
        Pallet::<T>::set_para_manager(origin as T::RuntimeOrigin, para_id, manager_address.clone());

        // After call: para manager
        assert_eq!(
            crate::ParaManager::<T>::get(para_id).as_ref(),
            Some(&manager_address)
        );
    }

    impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}
