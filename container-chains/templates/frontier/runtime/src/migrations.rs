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
    crate::{Precompiles, LOG_TARGET},
    frame_support::weights::Weight,
    pallet_migrations::{GetMigrations, Migration},
    sp_core::Get,
    sp_std::{marker::PhantomData, prelude::*},
};

pub struct MigratePrecompileDummyCode<T>(pub PhantomData<T>);
impl<T> Migration for MigratePrecompileDummyCode<T>
where
    T: pallet_evm::Config,
    T: frame_system::Config,
{
    fn friendly_name(&self) -> &str {
        "TM_MigratePrecompileCode"
    }

    fn migrate(&self, _available_weight: Weight) -> Weight {
        log::info!(target: LOG_TARGET, "migrate");
        let revert_bytecode = vec![0x60, 0x00, 0x60, 0x00, 0xFD];

        let db_weights = T::DbWeight::get();

        let mut count = 0u64;

        for address in Precompiles::used_addresses() {
            pallet_evm::Pallet::<T>::create_account(address.into(), revert_bytecode.clone());
            count += 1;
        }
        db_weights.reads_writes(count, count * 2)
    }

    /// Run a standard pre-runtime test. This works the same way as in a normal runtime upgrade.
    #[cfg(feature = "try-runtime")]
    fn pre_upgrade(&self) -> Result<Vec<u8>, sp_runtime::DispatchError> {
        log::info!(target: LOG_TARGET, "pre_upgrade");

        for address in Precompiles::used_addresses() {
            let account: sp_core::H160 = address.into();
            assert!(pallet_evm::AccountCodes::<T>::get(account).is_empty());
        }
        Ok(vec![])
    }

    /// Run a standard post-runtime test. This works the same way as in a normal runtime upgrade.
    #[cfg(feature = "try-runtime")]
    fn post_upgrade(&self, _: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
        log::info!(target: LOG_TARGET, "post_upgrade");
        let revert_bytecode = vec![0x60, 0x00, 0x60, 0x00, 0xFD];
        for address in Precompiles::used_addresses() {
            let account: sp_core::H160 = address.into();
            assert_eq!(pallet_evm::AccountCodes::<T>::get(account), revert_bytecode);
        }

        Ok(())
    }
}

pub struct TemplateMigrations<Runtime>(PhantomData<Runtime>);

impl<Runtime> GetMigrations for TemplateMigrations<Runtime>
where
    Runtime: pallet_evm::Config,
    Runtime: frame_system::Config,
{
    fn get_migrations() -> Vec<Box<dyn Migration>> {
        let migrate_precompiles = MigratePrecompileDummyCode::<Runtime>(Default::default());

        vec![
            // Comment after runtime 300
            Box::new(migrate_precompiles),
        ]
    }
}
