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
    crate::{Call, Config, DepositBalanceOf, EnsureSignedByManager, Pallet, RegistrarHooks},
    frame_benchmarking::{account, v2::*},
    frame_support::traits::{Currency, EnsureOriginWithArg},
    frame_system::RawOrigin,
    sp_core::{Decode, Encode, Get, H256},
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

#[derive(Encode, Decode)]
pub struct BenchmarkBlob {
    pub signature_account_u64: cumulus_primitives_core::relay_chain::Signature,
    pub signature_account_32_bytes: cumulus_primitives_core::relay_chain::Signature,
    pub sproof_0: (H256, sp_trie::StorageProof),
    pub sproof_empty: (H256, sp_trie::StorageProof),
}

fn benchmark_blob() -> BenchmarkBlob {
    const ENCODED: &[u8] = &[
        0, 9, 177, 241, 214, 77, 65, 110, 230, 156, 74, 146, 155, 254, 39, 188, 91, 117, 252, 8,
        90, 167, 163, 117, 158, 193, 82, 40, 92, 159, 126, 167, 127, 117, 168, 49, 35, 21, 117,
        102, 82, 252, 221, 89, 243, 5, 169, 208, 200, 206, 28, 124, 211, 224, 185, 242, 66, 250,
        47, 27, 200, 31, 48, 141, 6, 0, 118, 30, 162, 34, 163, 78, 91, 130, 87, 18, 148, 219, 145,
        6, 214, 15, 236, 131, 51, 167, 137, 239, 95, 207, 21, 167, 140, 167, 87, 106, 104, 155,
        189, 123, 155, 7, 173, 16, 8, 252, 54, 108, 185, 113, 139, 218, 215, 254, 68, 37, 173, 30,
        51, 245, 163, 205, 241, 202, 134, 131, 97, 41, 182, 15, 144, 103, 146, 229, 52, 207, 5, 82,
        255, 36, 219, 55, 41, 38, 112, 208, 127, 150, 239, 131, 227, 6, 154, 95, 28, 205, 66, 32,
        209, 89, 69, 228, 28, 176, 0, 0, 32, 0, 0, 0, 16, 0, 8, 0, 0, 0, 0, 4, 0, 0, 0, 1, 0, 0, 5,
        0, 0, 0, 5, 0, 0, 0, 6, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 196, 47, 140, 97, 41,
        216, 22, 207, 81, 195, 116, 188, 127, 8, 195, 230, 62, 209, 86, 207, 120, 174, 251, 74,
        101, 80, 217, 123, 135, 153, 121, 119, 238, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 9, 1, 63, 32, 6, 222, 61, 138, 84, 210, 126, 68, 169, 213, 206, 24, 150, 24, 242, 45,
        180, 180, 157, 149, 50, 13, 144, 33, 153, 76, 133, 15, 37, 184, 227, 133, 236, 219, 232,
        203, 100, 143, 101, 214, 239, 232, 53, 250, 27, 50, 253, 149, 140, 97, 244, 92, 63, 91,
        253, 137, 187, 12, 221, 174, 199, 161, 103, 210, 57, 1, 63, 56, 15, 186, 152, 104, 158,
        190, 209, 19, 135, 53, 224, 231, 165, 167, 144, 171, 205, 113, 11, 48, 189, 46, 171, 3, 82,
        221, 204, 38, 65, 122, 161, 148, 180, 222, 242, 92, 253, 166, 239, 58, 0, 0, 0, 0, 69, 171,
        129, 240, 74, 150, 152, 45, 218, 197, 226, 11, 83, 38, 121, 204, 15, 12, 197, 115, 153, 57,
        69, 212, 166, 142, 68, 212, 239, 160, 6, 119, 200, 95, 12, 230, 120, 121, 157, 62, 255, 2,
        66, 83, 185, 14, 132, 146, 124, 198, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 1, 128, 11, 0, 128, 21, 69, 87, 39,
        80, 189, 63, 244, 210, 244, 71, 86, 224, 172, 98, 5, 171, 30, 40, 194, 211, 222, 117, 47,
        81, 150, 51, 191, 236, 129, 237, 184, 128, 223, 91, 79, 3, 86, 79, 50, 130, 149, 40, 193,
        33, 8, 158, 35, 82, 137, 134, 195, 10, 60, 248, 193, 39, 236, 182, 225, 143, 110, 206, 62,
        80, 128, 77, 174, 114, 63, 107, 0, 39, 241, 147, 41, 150, 41, 130, 189, 148, 216, 5, 24,
        47, 167, 29, 117, 8, 120, 210, 51, 35, 145, 27, 94, 189, 250, 169, 1, 159, 12, 182, 243,
        110, 2, 122, 187, 32, 145, 207, 181, 17, 10, 181, 8, 127, 137, 0, 104, 95, 6, 21, 91, 60,
        217, 168, 201, 229, 233, 162, 63, 213, 220, 19, 165, 237, 32, 0, 0, 0, 0, 0, 0, 0, 0, 104,
        95, 8, 49, 108, 191, 143, 160, 218, 130, 42, 32, 172, 28, 85, 191, 27, 227, 32, 0, 0, 0, 0,
        0, 0, 0, 0, 128, 254, 108, 203, 37, 75, 132, 240, 210, 32, 46, 181, 5, 189, 223, 159, 84,
        203, 158, 189, 15, 178, 113, 144, 114, 233, 46, 229, 124, 29, 161, 216, 9, 190, 247, 74,
        173, 54, 61, 117, 185, 65, 198, 150, 80, 9, 116, 166, 1, 185, 33, 23, 38, 14, 102, 24, 248,
        23, 224, 15, 137, 66, 208, 101, 122, 20, 176, 0, 0, 32, 0, 0, 0, 16, 0, 8, 0, 0, 0, 0, 4,
        0, 0, 0, 1, 0, 0, 5, 0, 0, 0, 5, 0, 0, 0, 6, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        9, 1, 63, 32, 6, 222, 61, 138, 84, 210, 126, 68, 169, 213, 206, 24, 150, 24, 242, 45, 180,
        180, 157, 149, 50, 13, 144, 33, 153, 76, 133, 15, 37, 184, 227, 133, 236, 219, 232, 203,
        100, 143, 101, 214, 239, 232, 53, 250, 27, 50, 253, 149, 140, 97, 244, 92, 63, 91, 253,
        137, 187, 12, 221, 174, 199, 161, 103, 210, 200, 95, 12, 230, 120, 121, 157, 62, 255, 2,
        66, 83, 185, 14, 132, 146, 124, 198, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 1, 128, 3, 0, 128, 21, 69, 87, 39, 80,
        189, 63, 244, 210, 244, 71, 86, 224, 172, 98, 5, 171, 30, 40, 194, 211, 222, 117, 47, 81,
        150, 51, 191, 236, 129, 237, 184, 128, 223, 91, 79, 3, 86, 79, 50, 130, 149, 40, 193, 33,
        8, 158, 35, 82, 137, 134, 195, 10, 60, 248, 193, 39, 236, 182, 225, 143, 110, 206, 62, 80,
        169, 1, 159, 12, 182, 243, 110, 2, 122, 187, 32, 145, 207, 181, 17, 10, 181, 8, 127, 137,
        0, 104, 95, 6, 21, 91, 60, 217, 168, 201, 229, 233, 162, 63, 213, 220, 19, 165, 237, 32, 0,
        0, 0, 0, 0, 0, 0, 0, 104, 95, 8, 49, 108, 191, 143, 160, 218, 130, 42, 32, 172, 28, 85,
        191, 27, 227, 32, 0, 0, 0, 0, 0, 0, 0, 0, 128, 254, 108, 203, 37, 75, 132, 240, 210, 32,
        46, 181, 5, 189, 223, 159, 84, 203, 158, 189, 15, 178, 113, 144, 114, 233, 46, 229, 124,
        29, 161, 216, 9,
    ];
    #[allow(const_item_mutation)]
    BenchmarkBlob::decode(&mut ENCODED).unwrap()
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
