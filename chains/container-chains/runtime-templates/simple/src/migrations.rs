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
    sp_std::{marker::PhantomData, prelude::*},
};

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
    Runtime: frame_system::Config,
    Runtime: cumulus_pallet_xcmp_queue::Config,
    Runtime: pallet_xcm_executor_utils::Config,
    Runtime: pallet_xcm::Config,
    Runtime: pallet_foreign_asset_creator::Config,
{
    fn get_migrations() -> Vec<Box<dyn Migration>> {
        //let migrate_polkadot_xcm_v1 =
        //    PolkadotXcmMigrationFixVersion::<Runtime, PolkadotXcm>(Default::default());
        //let migrate_xcmp_queue_v2 =
        //    XcmpQueueMigrationFixVersion::<Runtime, XcmpQueue>(Default::default());
        //let migrate_xcmp_queue_v3 = XcmpQueueMigrationV3::<Runtime>(Default::default());
        //let migrate_xcmp_queue_v4 = XcmpQueueMigrationV4::<Runtime>(Default::default());
        //let migrate_xcm_executor_utils_v4 =
        //    pallet_xcm_executor_utils::migrations::MigrateToV1::<Runtime>(Default::default());
        // let migrate_pallet_xcm_v4 = MigrateToLatestXcmVersion::<Runtime>(Default::default());
        //let foreign_asset_creator_migration =
        //    ForeignAssetCreatorMigration::<Runtime>(Default::default());
        //let migrate_pallet_xcm_v5 = MigrateToLatestXcmVersion::<Runtime>(Default::default());
        vec![
            // Box::new(migrate_polkadot_xcm_v1),
            // Box::new(migrate_xcmp_queue_v2),
            // Box::new(migrate_xcmp_queue_v3),
            // Box::new(migrate_xcmp_queue_v4),
            //Box::new(migrate_xcm_executor_utils_v4),
            // Box::new(migrate_pallet_xcm_v4),
            //Box::new(foreign_asset_creator_migration),
            // Applied in runtime 1200
            //Box::new(migrate_pallet_xcm_v5),
        ]
    }
}
