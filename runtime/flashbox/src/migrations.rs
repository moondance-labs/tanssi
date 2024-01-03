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
    crate::{ParaId, Runtime, ServicesPayment},
    frame_support::{
        pallet_prelude::ValueQuery, storage::types::StorageMap, weights::Weight, Blake2_128Concat,
    },
    pallet_migrations::{GetMigrations, Migration},
    sp_core::Get,
    sp_runtime::BoundedVec,
    sp_std::{collections::btree_set::BTreeSet, marker::PhantomData, prelude::*},
};

pub struct MigrateServicesPaymentAddCredits<T>(pub PhantomData<T>);
impl<T> Migration for MigrateServicesPaymentAddCredits<T>
where
    T: pallet_configuration::Config,
{
    fn friendly_name(&self) -> &str {
        "TM_MigrateServicesPaymentAddCredits"
    }

    fn migrate(&self, _available_weight: Weight) -> Weight {
        // For each parachain in pallet_registrar (active, pending or pending_verification),
        // insert `MaxCreditsStored` to pallet_services_payment,
        // and mark that parachain as "given_free_credits".
        let mut para_ids = BTreeSet::new();
        let active = pallet_registrar::RegisteredParaIds::<Runtime>::get();
        let pending = pallet_registrar::PendingParaIds::<Runtime>::get();
        let pending_verification = pallet_registrar::PendingVerification::<Runtime>::get();
        // This migration ignores Paused and PendingPaused because they do not exist yet in flashbox

        para_ids.extend(active);
        para_ids.extend(pending.into_iter().flat_map(|(_session, active)| active));
        para_ids.extend(pending_verification);

        let reads = 3 + 2 * para_ids.len() as u64;
        let writes = 2 * para_ids.len() as u64;

        for para_id in para_ids {
            // 2 reads 2 writes
            ServicesPayment::give_free_credits(&para_id);
        }

        let db_weights = T::DbWeight::get();
        db_weights.reads_writes(reads, writes)
    }
}

pub struct RegistrarBootNodesStorageValuePrefix<T>(PhantomData<T>);
impl<T> frame_support::traits::StorageInstance for RegistrarBootNodesStorageValuePrefix<T> {
    const STORAGE_PREFIX: &'static str = "BootNodes";
    fn pallet_prefix() -> &'static str {
        "Registrar"
    }
}
pub type RegistrarBootNodesStorageMap<T> = StorageMap<
    RegistrarBootNodesStorageValuePrefix<T>,
    Blake2_128Concat,
    ParaId,
    //BoundedVec<BoundedVec<u8, T::MaxBootNodeUrlLen>, T::MaxBootNodes>,
    Vec<Vec<u8>>,
    ValueQuery,
>;

pub struct MigrateBootNodes<T>(pub PhantomData<T>);
impl<T> Migration for MigrateBootNodes<T>
where
    T: pallet_configuration::Config,
{
    fn friendly_name(&self) -> &str {
        "TM_MigrateBootNodes"
    }

    fn migrate(&self, _available_weight: Weight) -> Weight {
        let mut len = 0;
        for (para_id, bootnodes) in RegistrarBootNodesStorageMap::<Runtime>::drain() {
            len += 1;
            // Convert Vec<Vec<u8>> into BoundedVec<BoundedVec<u8>>
            // Cannot fail because the old storage was actually a BoundedVec with the same limit as the new one
            let bootnodes: Vec<_> = bootnodes
                .into_iter()
                .map(|bootnode| bootnode.try_into().unwrap())
                .collect();
            let bootnodes: BoundedVec<_, _> = bootnodes.try_into().unwrap();
            pallet_data_preservers::BootNodes::<Runtime>::insert(para_id, bootnodes);
        }

        let db_weights = T::DbWeight::get();
        let reads = len;
        let writes = len;
        db_weights.reads_writes(reads, writes)
    }
}

pub struct FlashboxMigrations<Runtime>(PhantomData<Runtime>);

impl<Runtime> GetMigrations for FlashboxMigrations<Runtime>
where
    Runtime: pallet_balances::Config,
    Runtime: pallet_configuration::Config,
{
    fn get_migrations() -> Vec<Box<dyn Migration>> {
        let migrate_services_payment =
            MigrateServicesPaymentAddCredits::<Runtime>(Default::default());
        let migrate_boot_nodes = MigrateBootNodes::<Runtime>(Default::default());

        vec![
            Box::new(migrate_services_payment),
            Box::new(migrate_boot_nodes),
        ]
    }
}
