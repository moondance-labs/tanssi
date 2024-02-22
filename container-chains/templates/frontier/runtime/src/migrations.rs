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
    frame_support::{pallet_prelude::GetStorageVersion, traits::PalletInfoAccess},
    pallet_migrations::{GetMigrations, Migration},
    runtime_common::migrations::{
        PolkadotXcmMigrationFixVersion, XcmpQueueMigrationFixVersion, XcmpQueueMigrationV3,
        XcmpQueueMigrationV4,
    },
    sp_std::{marker::PhantomData, prelude::*},
};

pub struct TemplateMigrations<Runtime, XcmpQueue, PolkadotXcm>(
    PhantomData<(Runtime, XcmpQueue, PolkadotXcm)>,
);

impl<Runtime, XcmpQueue, PolkadotXcm> GetMigrations
    for TemplateMigrations<Runtime, XcmpQueue, PolkadotXcm>
where
    PolkadotXcm: GetStorageVersion + PalletInfoAccess + 'static,
    XcmpQueue: GetStorageVersion + PalletInfoAccess + 'static,
    Runtime: pallet_evm::Config,
    Runtime: frame_system::Config,
    Runtime: cumulus_pallet_xcmp_queue::Config,
{
    fn get_migrations() -> Vec<Box<dyn Migration>> {
        // let migrate_precompiles = MigratePrecompileDummyCode::<Runtime>(Default::default());
        let migrate_polkadot_xcm_v1 =
            PolkadotXcmMigrationFixVersion::<Runtime, PolkadotXcm>(Default::default());
        let migrate_xcmp_queue_v2 =
            XcmpQueueMigrationFixVersion::<Runtime, XcmpQueue>(Default::default());
        let migrate_xcmp_queue_v3 = XcmpQueueMigrationV3::<Runtime>(Default::default());
        let migrate_xcmp_queue_v4 = XcmpQueueMigrationV4::<Runtime>(Default::default());
        vec![
            // Applied in runtime 400
            // Box::new(migrate_precompiles),
            Box::new(migrate_polkadot_xcm_v1),
            Box::new(migrate_xcmp_queue_v2),
            Box::new(migrate_xcmp_queue_v3),
            Box::new(migrate_xcmp_queue_v4),
        ]
    }
}
