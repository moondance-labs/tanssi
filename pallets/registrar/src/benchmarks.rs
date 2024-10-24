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
        benchmark_blob::benchmark_blob, Call, Config, DepositBalanceOf, EnsureSignedByManager,
        Pallet, RegistrarHooks,
    },
    dp_container_chain_genesis_data::{ContainerChainGenesisData, ContainerChainGenesisDataItem},
    frame_benchmarking::{account, v2::*},
    frame_support::{
        assert_ok,
        traits::{
            fungible::{Inspect, Mutate},
            EnsureOrigin, EnsureOriginWithArg,
        },
    },
    frame_system::RawOrigin,
    sp_core::Get,
    sp_std::{vec, vec::Vec},
    tp_traits::{ParaId, RegistrarHandler, RelayStorageRootProvider, SlotFrequency},
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
    let min_reserve_amount = T::Currency::minimum_balance() * 10_000_000u32.into();
    let total = min_reserve_amount + extra;
    assert_ok!(T::Currency::mint_into(&user, total));
    (user, total)
}

#[benchmarks]
mod benchmarks {
    use {
        super::*, cumulus_primitives_core::relay_chain::MIN_CODE_SIZE, parity_scale_codec::Encode,
    };

    fn new_genesis_data(storage: Vec<ContainerChainGenesisDataItem>) -> ContainerChainGenesisData {
        ContainerChainGenesisData {
            storage,
            name: Default::default(),
            id: Default::default(),
            fork_id: Default::default(),
            extensions: Default::default(),
            properties: Default::default(),
        }
    }

    /// Creates a `ContainerChainGenesisData` with encoded size very near to `max_encoded_size`, and
    /// with the provided number of keys.
    fn max_size_genesis_data(num_keys: u32, max_encoded_size: u32) -> ContainerChainGenesisData {
        let mut storage = vec![];
        // Create one big storage item
        storage.push(
            (
                b"dummy".to_vec(),
                vec![1; max_encoded_size.saturating_sub(MIN_CODE_SIZE) as usize],
            )
                .into(),
        );
        storage.push((b":code".to_vec(), vec![1; MIN_CODE_SIZE as usize]).into());
        // Fill rest of keys with empty values
        for _i in 1..num_keys {
            storage.push((b"".to_vec(), b"".to_vec()).into());
        }
        // Calculate resulting encoded size
        let size = new_genesis_data(storage.clone()).encoded_size();
        // Should be bigger than max
        assert!(
            size >= max_encoded_size as usize,
            "{:?}",
            (size, ">=", max_encoded_size)
        );
        // Remove size diff from first item in storage vec
        let size_diff = size - max_encoded_size as usize;
        let first_value = &mut storage[0].value;
        assert!(
            first_value.len() >= size_diff,
            "{:?}",
            (first_value.len(), ">=", size_diff)
        );
        first_value.truncate(first_value.len() - size_diff);

        let genesis_data = new_genesis_data(storage);

        // Verify new size matches max exactly
        let size = genesis_data.encoded_size();
        // Should be almost exact, but in some cases it is 1 byte smaller because of encoding
        assert!(
            size <= max_encoded_size as usize,
            "{:?}",
            (size, "<=", max_encoded_size)
        );

        genesis_data
    }

    fn get_code(storage: &ContainerChainGenesisData) -> Vec<u8> {
        storage
            .storage
            .iter()
            .find_map(|kv| {
                if kv.key == b":code" {
                    Some(kv.value.clone())
                } else {
                    None
                }
            })
            .unwrap()
    }

    // Returns number of para ids in pending verification (registered but not marked as valid)
    fn pending_verification_len<T: Config>() -> usize {
        crate::PendingVerification::<T>::iter_keys().count()
    }

    #[benchmark]
    fn register(x: Linear<100, 3_000_000>, z: Linear<1, 10>) {
        let storage = max_size_genesis_data(z, x);

        let (caller, _deposit_amount) =
            create_funded_user::<T>("caller", 0, T::DepositAmount::get());

        #[extrinsic_call]
        Pallet::<T>::register(
            RawOrigin::Signed(caller),
            Default::default(),
            storage,
            T::InnerRegistrar::bench_head_data(),
        );

        // verification code
        assert_eq!(pending_verification_len::<T>(), 1usize);
        assert!(Pallet::<T>::registrar_deposit(ParaId::default()).is_some());
    }

    #[benchmark]
    fn register_with_relay_proof(
        x: Linear<100, 3_000_000>,
        z: Linear<1, 10>,
    ) -> Result<(), BenchmarkError> {
        // This extrinsic is disabled in flashbox runtime, return 0 weight there
        let _origin = T::RegisterWithRelayProofOrigin::try_successful_origin()
            .map_err(|_| BenchmarkError::Weightless)?;
        let storage = max_size_genesis_data(z, x);

        let (caller, _deposit_amount) =
            create_funded_user::<T>("caller", 0, T::DepositAmount::get());

        // Uncomment to update blob
        //panic!("caller: {:?}, is_u64? {}", caller.encode(), core::any::TypeId::of::<T::AccountId>() == core::any::TypeId::of::<u64>());

        let blob = benchmark_blob();
        let (relay_parent_storage_root, proof) = blob.sproof_0;

        T::RelayStorageRootProvider::set_relay_storage_root(1, Some(relay_parent_storage_root));

        // In tests we need a signature for a u64 accountid, in runtime we need a different signature
        // for a 32 byte account id.
        let signature = if core::any::TypeId::of::<T::AccountId>() == core::any::TypeId::of::<u64>()
        {
            blob.signature_account_u64
        } else {
            blob.signature_account_32_bytes
        };

        #[extrinsic_call]
        Pallet::<T>::register_with_relay_proof(
            RawOrigin::Signed(caller),
            Default::default(),
            None,
            1,
            proof,
            signature,
            storage,
            None,
        );

        // verification code
        assert_eq!(pending_verification_len::<T>(), 1usize);
        assert!(Pallet::<T>::registrar_deposit(ParaId::default()).is_some());

        Ok(())
    }

    #[benchmark]
    fn deregister_immediate() {
        let x = T::MaxGenesisDataSize::get();
        let y = T::MaxLengthParaIds::get();
        let storage = max_size_genesis_data(1, x);

        for i in 0..y {
            // Twice the deposit just in case
            let (caller, _deposit_amount) =
                create_funded_user::<T>("caller", i, T::DepositAmount::get());
            Pallet::<T>::register(
                RawOrigin::Signed(caller.clone()).into(),
                i.into(),
                storage.clone(),
                T::InnerRegistrar::bench_head_data(),
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
    fn deregister_scheduled() {
        let x = T::MaxGenesisDataSize::get();
        let y = T::MaxLengthParaIds::get();
        let storage = max_size_genesis_data(1, x);
        let code = get_code(&storage);

        // Deregister all the existing chains to avoid conflicts with the new ones
        for para_id in Pallet::<T>::registered_para_ids() {
            Pallet::<T>::deregister(RawOrigin::Root.into(), para_id).unwrap();
        }

        for i in 0..y {
            // Twice the deposit just in case
            let (caller, _deposit_amount) =
                create_funded_user::<T>("caller", i, T::DepositAmount::get());
            Pallet::<T>::register(
                RawOrigin::Signed(caller.clone()).into(),
                i.into(),
                storage.clone(),
                T::InnerRegistrar::bench_head_data(),
            )
            .unwrap();
            T::InnerRegistrar::add_trusted_validation_code(code.clone());
            T::RegistrarHooks::benchmarks_ensure_valid_for_collating(i.into());
        }

        T::InnerRegistrar::registrar_new_session(1);
        T::InnerRegistrar::registrar_new_session(2);
        T::InnerRegistrar::registrar_new_session(3);

        for i in 0..y {
            // Call mark_valid_for_collating to ensure that the deregister call
            // does not execute the cleanup hooks immediately
            Pallet::<T>::mark_valid_for_collating(RawOrigin::Root.into(), i.into()).unwrap();
        }

        // Start a new session
        Pallet::<T>::initializer_on_new_session(&(T::SessionDelay::get() + 3u32.into()));
        // We should have registered y
        assert_eq!(Pallet::<T>::registered_para_ids().len(), y as usize);
        assert!(Pallet::<T>::registrar_deposit(ParaId::from(y - 1)).is_some());

        #[extrinsic_call]
        Pallet::<T>::deregister(RawOrigin::Root, (y - 1).into());

        // We now have y - 1 but the deposit has not been removed yet
        assert_eq!(
            Pallet::<T>::pending_registered_para_ids()[0].1.len(),
            (y - 1) as usize
        );
        assert!(Pallet::<T>::registrar_deposit(ParaId::from(y - 1)).is_some());

        // Start a new session
        Pallet::<T>::initializer_on_new_session(
            &(T::SessionDelay::get() * 2u32.into() + 3u32.into()),
        );

        // Now it has been removed
        assert_eq!(Pallet::<T>::registered_para_ids().len(), (y - 1) as usize);
        assert!(Pallet::<T>::registrar_deposit(ParaId::from(y - 1)).is_none());
    }

    #[benchmark]
    fn deregister_with_relay_proof_immediate() -> Result<(), BenchmarkError> {
        // This extrinsic is disabled in flashbox runtime, return 0 weight there
        let _origin = T::RegisterWithRelayProofOrigin::try_successful_origin()
            .map_err(|_| BenchmarkError::Weightless)?;
        let x = T::MaxGenesisDataSize::get();
        let y = T::MaxLengthParaIds::get();
        let storage = max_size_genesis_data(1, x);

        for i in 0..y {
            // Twice the deposit just in case
            let (caller, _deposit_amount) =
                create_funded_user::<T>("caller", i, T::DepositAmount::get());
            Pallet::<T>::register(
                RawOrigin::Signed(caller.clone()).into(),
                i.into(),
                storage.clone(),
                T::InnerRegistrar::bench_head_data(),
            )
            .unwrap();
            // Do not call mark_valid_for_collating, to ensure that the deregister call also executes the cleanup hooks
        }

        // We should have registered y
        assert_eq!(pending_verification_len::<T>(), y as usize);
        assert!(Pallet::<T>::registrar_deposit(ParaId::from(y - 1)).is_some());

        let (caller, _deposit_amount) =
            create_funded_user::<T>("caller", 0, T::DepositAmount::get());

        let blob = benchmark_blob();
        let (relay_parent_storage_root, proof) = blob.sproof_empty;

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

        Ok(())
    }

    #[benchmark]
    fn deregister_with_relay_proof_scheduled() -> Result<(), BenchmarkError> {
        // This extrinsic is disabled in flashbox runtime, return 0 weight there
        let _origin = T::RegisterWithRelayProofOrigin::try_successful_origin()
            .map_err(|_| BenchmarkError::Weightless)?;
        let x = T::MaxGenesisDataSize::get();
        let y = T::MaxLengthParaIds::get();
        let storage = max_size_genesis_data(1, x);
        let code = get_code(&storage);
        // Deregister all the existing chains to avoid conflicts with the new ones
        for para_id in Pallet::<T>::registered_para_ids() {
            Pallet::<T>::deregister(RawOrigin::Root.into(), para_id).unwrap();
        }

        for i in 0..y {
            // Twice the deposit just in case
            let (caller, _deposit_amount) =
                create_funded_user::<T>("caller", i, T::DepositAmount::get());
            Pallet::<T>::register(
                RawOrigin::Signed(caller.clone()).into(),
                i.into(),
                storage.clone(),
                T::InnerRegistrar::bench_head_data(),
            )
            .unwrap();
            T::InnerRegistrar::add_trusted_validation_code(code.clone());
            T::RegistrarHooks::benchmarks_ensure_valid_for_collating(i.into());
        }

        T::InnerRegistrar::registrar_new_session(1);
        T::InnerRegistrar::registrar_new_session(2);
        T::InnerRegistrar::registrar_new_session(3);

        for i in 0..y {
            // Call mark_valid_for_collating to ensure that the deregister call
            // does not execute the cleanup hooks immediately
            Pallet::<T>::mark_valid_for_collating(RawOrigin::Root.into(), i.into()).unwrap();
        }

        // Start a new session
        Pallet::<T>::initializer_on_new_session(&(T::SessionDelay::get() + 3u32.into()));
        // We should have registered y
        assert_eq!(Pallet::<T>::registered_para_ids().len(), y as usize);
        assert!(Pallet::<T>::registrar_deposit(ParaId::from(y - 1)).is_some());

        let (caller, _deposit_amount) =
            create_funded_user::<T>("caller", 0, T::DepositAmount::get());

        let blob = benchmark_blob();
        let (relay_parent_storage_root, proof) = blob.sproof_empty;

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
            (y - 1) as usize
        );
        assert!(Pallet::<T>::registrar_deposit(ParaId::from(y - 1)).is_none());

        // Start a new session
        Pallet::<T>::initializer_on_new_session(
            &(T::SessionDelay::get() * 2u32.into() + 3u32.into()),
        );

        // Now it has been removed
        assert_eq!(Pallet::<T>::registered_para_ids().len(), (y - 1) as usize);

        Ok(())
    }

    #[benchmark]
    fn mark_valid_for_collating() {
        let x = T::MaxGenesisDataSize::get();
        let y = T::MaxLengthParaIds::get();
        let storage = max_size_genesis_data(1, x);
        let code = get_code(&storage);

        // Deregister all the existing chains to avoid conflicts with the new ones
        for para_id in Pallet::<T>::registered_para_ids() {
            Pallet::<T>::deregister(RawOrigin::Root.into(), para_id).unwrap();
        }

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
                T::InnerRegistrar::bench_head_data(),
            )
            .unwrap();
        }

        // Second loop to fill RegisteredParaIds to its maximum, minus 1 space for the benchmark call
        for k in 1000..(1000 + y - 1) {
            // Twice the deposit just in case
            let (caller, _deposit_amount) =
                create_funded_user::<T>("caller", k, T::DepositAmount::get());
            Pallet::<T>::register(
                RawOrigin::Signed(caller.clone()).into(),
                k.into(),
                storage.clone(),
                T::InnerRegistrar::bench_head_data(),
            )
            .unwrap();
            T::InnerRegistrar::add_trusted_validation_code(code.clone());
            T::RegistrarHooks::benchmarks_ensure_valid_for_collating(k.into());
        }

        T::InnerRegistrar::registrar_new_session(1);
        T::InnerRegistrar::registrar_new_session(2);
        T::InnerRegistrar::registrar_new_session(3);

        for k in 1000..(1000 + y - 1) {
            // Call mark_valid_for_collating to ensure that the deregister call
            // does not execute the cleanup hooks immediately
            Pallet::<T>::mark_valid_for_collating(RawOrigin::Root.into(), k.into()).unwrap();
        }

        // Start a new session
        Pallet::<T>::initializer_on_new_session(&(T::SessionDelay::get() + 3u32.into()));

        // We should have registered y
        assert_eq!(pending_verification_len::<T>(), y as usize);
        T::RegistrarHooks::benchmarks_ensure_valid_for_collating((y - 1).into());

        #[extrinsic_call]
        Pallet::<T>::mark_valid_for_collating(RawOrigin::Root, (y - 1).into());

        // We should have y-1
        assert_eq!(pending_verification_len::<T>(), (y - 1) as usize);
    }

    #[benchmark]
    fn pause_container_chain() {
        let x = T::MaxGenesisDataSize::get();
        let y = T::MaxLengthParaIds::get();
        let storage = max_size_genesis_data(1, x);
        let code = get_code(&storage);
        // Deregister all the existing chains to avoid conflicts with the new ones
        for para_id in Pallet::<T>::registered_para_ids() {
            Pallet::<T>::deregister(RawOrigin::Root.into(), para_id).unwrap();
        }

        // Worst case: when RegisteredParaIds and Paused are both full
        // Second loop to fill Paused to its maximum, minus 1 space for the benchmark call
        for k in 1000..(1000 + y - 1) {
            let (caller, _deposit_amount) =
                create_funded_user::<T>("caller", k, T::DepositAmount::get());
            Pallet::<T>::register(
                RawOrigin::Signed(caller.clone()).into(),
                k.into(),
                storage.clone(),
                T::InnerRegistrar::bench_head_data(),
            )
            .unwrap();
            T::InnerRegistrar::add_trusted_validation_code(code.clone());
            T::RegistrarHooks::benchmarks_ensure_valid_for_collating(k.into());
        }

        // First loop to fill RegisteredParaIds to its maximum
        for i in 0..y {
            // Twice the deposit just in case
            let (caller, _deposit_amount) =
                create_funded_user::<T>("caller", i, T::DepositAmount::get());
            Pallet::<T>::register(
                RawOrigin::Signed(caller.clone()).into(),
                i.into(),
                storage.clone(),
                T::InnerRegistrar::bench_head_data(),
            )
            .unwrap();
            T::InnerRegistrar::add_trusted_validation_code(code.clone());
            T::RegistrarHooks::benchmarks_ensure_valid_for_collating(i.into());
        }

        T::InnerRegistrar::registrar_new_session(1);
        T::InnerRegistrar::registrar_new_session(2);
        T::InnerRegistrar::registrar_new_session(3);

        for k in 1000..(1000 + y - 1) {
            Pallet::<T>::mark_valid_for_collating(RawOrigin::Root.into(), k.into()).unwrap();
            Pallet::<T>::pause_container_chain(RawOrigin::Root.into(), k.into()).unwrap();
        }
        for i in 0..y {
            Pallet::<T>::mark_valid_for_collating(RawOrigin::Root.into(), i.into()).unwrap();
        }

        // Check PendingPaused has a length of y - 1
        assert_eq!(Pallet::<T>::pending_paused()[0].1.len(), y as usize - 1);
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
        Pallet::<T>::initializer_on_new_session(&(T::SessionDelay::get() + 3u32.into()));

        // Check y-1 is in Paused
        assert!(Pallet::<T>::paused().contains(&ParaId::from(y - 1)));
        // Check y-1 is not in registered_para_ids
        assert!(!Pallet::<T>::registered_para_ids().contains(&ParaId::from(y - 1)));
    }

    #[benchmark]
    fn unpause_container_chain() {
        let x = T::MaxGenesisDataSize::get();
        let y = T::MaxLengthParaIds::get();
        let storage = max_size_genesis_data(1, x);
        let code = get_code(&storage);
        // Deregister all the existing chains to avoid conflicts with the new ones
        for para_id in Pallet::<T>::registered_para_ids() {
            Pallet::<T>::deregister(RawOrigin::Root.into(), para_id).unwrap();
        }

        // Worst case: when RegisteredParaIds and Paused are both full
        // Second loop to fill Paused to its maximum
        for k in 1000..(1000 + y) {
            let (caller, _deposit_amount) =
                create_funded_user::<T>("caller", k, T::DepositAmount::get());
            Pallet::<T>::register(
                RawOrigin::Signed(caller.clone()).into(),
                k.into(),
                storage.clone(),
                T::InnerRegistrar::bench_head_data(),
            )
            .unwrap();
            T::InnerRegistrar::add_trusted_validation_code(code.clone());
            T::RegistrarHooks::benchmarks_ensure_valid_for_collating(k.into());
        }

        // First loop to fill RegisteredParaIds to its maximum, minus 1 space for the benchmark call
        for i in 0..(y - 1) {
            // Twice the deposit just in case
            let (caller, _deposit_amount) =
                create_funded_user::<T>("caller", i, T::DepositAmount::get());
            Pallet::<T>::register(
                RawOrigin::Signed(caller.clone()).into(),
                i.into(),
                storage.clone(),
                T::InnerRegistrar::bench_head_data(),
            )
            .unwrap();
            T::InnerRegistrar::add_trusted_validation_code(code.clone());
            T::RegistrarHooks::benchmarks_ensure_valid_for_collating(i.into());
        }

        T::InnerRegistrar::registrar_new_session(1);
        T::InnerRegistrar::registrar_new_session(2);
        T::InnerRegistrar::registrar_new_session(3);

        for k in 1000..(1000 + y) {
            Pallet::<T>::mark_valid_for_collating(RawOrigin::Root.into(), k.into()).unwrap();
            Pallet::<T>::pause_container_chain(RawOrigin::Root.into(), k.into()).unwrap();
        }
        for i in 0..(y - 1) {
            Pallet::<T>::mark_valid_for_collating(RawOrigin::Root.into(), i.into()).unwrap();
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
        Pallet::<T>::initializer_on_new_session(&(T::SessionDelay::get() + 3u32.into()));

        // Check 1000 is not in Paused
        assert!(!Pallet::<T>::paused().contains(&ParaId::from(1000)));
        // Check 1000 is in registered_para_ids
        assert!(Pallet::<T>::registered_para_ids().contains(&ParaId::from(1000)));
    }

    #[benchmark]
    fn register_parathread(x: Linear<100, 3_000_000>, z: Linear<1, 10>) {
        let storage = max_size_genesis_data(z, x);
        let slot_frequency = SlotFrequency::default();

        let (caller, _deposit_amount) =
            create_funded_user::<T>("caller", 0, T::DepositAmount::get());

        #[extrinsic_call]
        Pallet::<T>::register_parathread(
            RawOrigin::Signed(caller),
            Default::default(),
            slot_frequency,
            storage,
            T::InnerRegistrar::bench_head_data(),
        );

        // verification code
        assert_eq!(pending_verification_len::<T>(), 1usize);
        assert!(Pallet::<T>::registrar_deposit(ParaId::default()).is_some());
    }

    #[benchmark]
    fn set_parathread_params() {
        let x = T::MaxGenesisDataSize::get();
        let y = T::MaxLengthParaIds::get();
        let storage = max_size_genesis_data(1, x);
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
                T::InnerRegistrar::bench_head_data(),
            )
            .unwrap();
            T::RegistrarHooks::benchmarks_ensure_valid_for_collating(i.into());
        }

        T::InnerRegistrar::registrar_new_session(1);
        T::InnerRegistrar::registrar_new_session(2);
        T::InnerRegistrar::registrar_new_session(3);

        for i in 0..y {
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
        Pallet::<T>::initializer_on_new_session(&(T::SessionDelay::get() + 3u32.into()));

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
