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

//! # Migrations
//!
//! This module acts as a registry where each migration is defined. Each migration should implement
//! the "Migration" trait declared in the pallet-migrations crate.

use {
    frame_support::{
        pallet_prelude::GetStorageVersion,
        traits::{OnRuntimeUpgrade, PalletInfoAccess},
        weights::Weight,
    },
    pallet_migrations::{GetMigrations, Migration},
    sp_core::{Get, H160},
    sp_std::{marker::PhantomData, prelude::*},
};

pub struct MigratePrecompileXcmDummyCode<T>(pub PhantomData<T>);
impl<T> Migration for MigratePrecompileXcmDummyCode<T>
where
    T: pallet_evm::Config,
    T: frame_system::Config,
{
    fn friendly_name(&self) -> &str {
        "TM_MigratePrecompileXcmCode"
    }

    fn migrate(&self, _available_weight: Weight) -> Weight {
        log::info!("Performing migration: TM_MigratePrecompileXcmCode");
        let revert_bytecode = vec![0x60, 0x00, 0x60, 0x00, 0xFD];

        let db_weights = T::DbWeight::get();

        // Pallet-xcm precompile address
        let address = H160::from_low_u64_be(2052);
        let _ = pallet_evm::Pallet::<T>::create_account(address, revert_bytecode.clone(), None);

        // reads: <Suicided<T>> and <AccountCodes<T>>
        // writes: <AccountCodesMetadata<T>> and <AccountCodes<T>>
        db_weights.reads_writes(2, 2)
    }

    /// Run a standard pre-runtime test. This works the same way as in a normal runtime upgrade.
    #[cfg(feature = "try-runtime")]
    fn pre_upgrade(&self) -> Result<Vec<u8>, sp_runtime::DispatchError> {
        log::info!("Performing TM_MigratePrecompileXcmCode - pre_upgrade");

        let address = H160::from_low_u64_be(2052);
        assert!(pallet_evm::AccountCodes::<T>::get(address).is_empty());
        Ok(vec![])
    }

    /// Run a standard post-runtime test. This works the same way as in a normal runtime upgrade.
    #[cfg(feature = "try-runtime")]
    fn post_upgrade(&self, _: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
        log::info!("Performing TM_MigratePrecompileXcmCode - post_upgrade");

        let revert_bytecode = vec![0x60, 0x00, 0x60, 0x00, 0xFD];
        let address = H160::from_low_u64_be(2052);
        assert_eq!(pallet_evm::AccountCodes::<T>::get(address), revert_bytecode);
        Ok(())
    }
}

pub struct MigratePrecompileProxyDummyCode<T>(pub PhantomData<T>);
impl<T> Migration for MigratePrecompileProxyDummyCode<T>
where
    T: pallet_evm::Config,
    T: frame_system::Config,
{
    fn friendly_name(&self) -> &str {
        "TM_MigratePrecompileProxyCode"
    }

    fn migrate(&self, _available_weight: Weight) -> Weight {
        log::info!("Performing migration: TM_MigratePrecompileProxyCode");
        let revert_bytecode = vec![0x60, 0x00, 0x60, 0x00, 0xFD];

        let db_weights = T::DbWeight::get();

        // Pallet-xcm precompile address
        let address = H160::from_low_u64_be(2053);
        let _ = pallet_evm::Pallet::<T>::create_account(address, revert_bytecode.clone(), None);

        // reads: <Suicided<T>> and <AccountCodes<T>>
        // writes: <AccountCodesMetadata<T>> and <AccountCodes<T>>
        db_weights.reads_writes(2, 2)
    }

    /// Run a standard pre-runtime test. This works the same way as in a normal runtime upgrade.
    #[cfg(feature = "try-runtime")]
    fn pre_upgrade(&self) -> Result<Vec<u8>, sp_runtime::DispatchError> {
        log::info!("Performing TM_MigratePrecompileProxyCode - pre_upgrade");

        let address = H160::from_low_u64_be(2053);
        assert!(pallet_evm::AccountCodes::<T>::get(address).is_empty());
        Ok(vec![])
    }

    /// Run a standard post-runtime test. This works the same way as in a normal runtime upgrade.
    #[cfg(feature = "try-runtime")]
    fn post_upgrade(&self, _: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
        log::info!("Performing TM_MigratePrecompileProxyCode - post_upgrade");

        let revert_bytecode = vec![0x60, 0x00, 0x60, 0x00, 0xFD];
        let address = H160::from_low_u64_be(2053);
        assert_eq!(pallet_evm::AccountCodes::<T>::get(address), revert_bytecode);
        Ok(())
    }
}

pub struct TemplateMigrations<Runtime, XcmpQueue, PolkadotXcm>(
    PhantomData<(Runtime, XcmpQueue, PolkadotXcm)>,
);

pub struct MigrateToLatestXcmVersion<Runtime>(PhantomData<Runtime>);
impl<Runtime> Migration for MigrateToLatestXcmVersion<Runtime>
where
    pallet_xcm::migration::MigrateToLatestXcmVersion<Runtime>:
        frame_support::traits::OnRuntimeUpgrade,
{
    fn friendly_name(&self) -> &str {
        "MM_MigrateToLatestXcmVersion5"
    }

    fn migrate(&self, _available_weight: Weight) -> Weight {
        pallet_xcm::migration::MigrateToLatestXcmVersion::<Runtime>::on_runtime_upgrade()
    }

    #[cfg(feature = "try-runtime")]
    fn pre_upgrade(&self) -> Result<Vec<u8>, sp_runtime::DispatchError> {
        pallet_xcm::migration::MigrateToLatestXcmVersion::<Runtime>::pre_upgrade()
    }

    #[cfg(feature = "try-runtime")]
    fn post_upgrade(&self, state: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
        pallet_xcm::migration::MigrateToLatestXcmVersion::<Runtime>::post_upgrade(state)
    }
}

impl<Runtime, XcmpQueue, PolkadotXcm> GetMigrations
    for TemplateMigrations<Runtime, XcmpQueue, PolkadotXcm>
where
    PolkadotXcm: GetStorageVersion + PalletInfoAccess + 'static,
    XcmpQueue: GetStorageVersion + PalletInfoAccess + 'static,
    Runtime: pallet_evm::Config,
    Runtime: frame_system::Config,
    Runtime: cumulus_pallet_xcmp_queue::Config,
    Runtime: pallet_xcm_executor_utils::Config,
    Runtime: pallet_xcm::Config,
{
    fn get_migrations() -> Vec<Box<dyn Migration>> {
        // let migrate_precompiles = MigratePrecompileDummyCode::<Runtime>(Default::default());
        //let migrate_polkadot_xcm_v1 =
        //    PolkadotXcmMigrationFixVersion::<Runtime, PolkadotXcm>(Default::default());
        //let migrate_xcmp_queue_v2 =
        //    XcmpQueueMigrationFixVersion::<Runtime, XcmpQueue>(Default::default());
        //let migrate_xcmp_queue_v3 = XcmpQueueMigrationV3::<Runtime>(Default::default());
        //let migrate_xcmp_queue_v4 = XcmpQueueMigrationV4::<Runtime>(Default::default());
        //let migrate_xcm_executor_utils_v4 =
        //    pallet_xcm_executor_utils::migrations::MigrateToV1::<Runtime>(Default::default());
        // let migrate_pallet_xcm_v4 = MigrateToLatestXcmVersion::<Runtime>(Default::default());
        //let migrate_precompile_proxy_code =
        //    MigratePrecompileProxyDummyCode::<Runtime>(Default::default());
        //let migrate_precompile_xcm_code =
        //    MigratePrecompileXcmDummyCode::<Runtime>(Default::default());

        //let migrate_pallet_xcm_v5 = MigrateToLatestXcmVersion::<Runtime>(Default::default());
        vec![
            // Applied in runtime 400
            // Box::new(migrate_precompiles),
            // Box::new(migrate_polkadot_xcm_v1),
            // Box::new(migrate_xcmp_queue_v2),
            // Box::new(migrate_xcmp_queue_v3),
            // Box::new(migrate_xcmp_queue_v4),
            // Box::new(migrate_xcm_executor_utils_v4),
            // Box::new(migrate_pallet_xcm_v4),
            // Box::new(migrate_precompile_proxy_code),
            // Box::new(migrate_precompile_xcm_code),
            // Applied in runtime 1200
            //Box::new(migrate_pallet_xcm_v5),
        ]
    }
}
