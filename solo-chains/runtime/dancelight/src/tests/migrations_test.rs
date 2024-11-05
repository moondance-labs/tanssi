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

use crate::tests::common::ExtBuilder;
use crate::{BeefyMmrLeaf, ExternalValidators, PalletInfo, Runtime, Session};
use beefy_primitives::mmr::BeefyAuthoritySet;
use frame_support::migration::clear_storage_prefix;
use frame_support::storage::unhashed;
use frame_support::traits::PalletInfo as _;
use pallet_migrations::Migration;
use tanssi_runtime_common::migrations::{ExternalValidatorsInitialMigration, MigrateMMRLeafPallet};
use xcm::v3::Weight;

#[test]
fn test_migration_mmr_leaf_pallet_renaming() {
    ExtBuilder::default().build().execute_with(|| {
        let migrate_mmr_leaf_pallet = MigrateMMRLeafPallet::<Runtime>(Default::default());
        let old_pallet_name = MigrateMMRLeafPallet::<Runtime>::old_pallet_name();
        let old_storage_1 = frame_support::storage::storage_prefix(
            old_pallet_name.as_bytes(),
            b"example_storage_1",
        );
        let new_storage_1 = frame_support::storage::storage_prefix(
            PalletInfo::name::<BeefyMmrLeaf>()
                .expect("BeefyMMRLeaf pallet must be part of the runtime")
                .as_bytes(),
            b"example_storage_1",
        );
        unhashed::put(&old_storage_1, &1u64);

        let beefy_authority_set: BeefyAuthoritySet<()> = BeefyAuthoritySet {
            len: 5,
            ..Default::default()
        };
        let old_storage_2 = frame_support::storage::storage_prefix(
            old_pallet_name.as_bytes(),
            b"example_storage_2",
        );
        let new_storage_2 = frame_support::storage::storage_prefix(
            PalletInfo::name::<BeefyMmrLeaf>()
                .expect("BeefyMMRLeaf pallet must be part of the runtime")
                .as_bytes(),
            b"example_storage_2",
        );
        unhashed::put(&old_storage_2, &beefy_authority_set);

        let used_weight = migrate_mmr_leaf_pallet.migrate(Weight::MAX);
        assert_eq!(used_weight, Weight::MAX);

        assert_eq!(unhashed::get::<u64>(&old_storage_1), None);
        assert_eq!(unhashed::get::<BeefyAuthoritySet<()>>(&old_storage_2), None);

        assert_eq!(unhashed::get::<u64>(&new_storage_1), Some(1u64));
        assert_eq!(
            unhashed::get::<BeefyAuthoritySet<()>>(&new_storage_2),
            Some(BeefyAuthoritySet {
                len: 5,
                ..Default::default()
            })
        );
    });
}

#[test]
fn test_migration_external_validators_pallet() {
    ExtBuilder::default().build().execute_with(|| {
        let migrate_external_validators =
            ExternalValidatorsInitialMigration::<Runtime>(Default::default());
        let old_pallet_name = b"ValidatorManager";

        // Kill storage of ExternalValidators pallet, because this migration will initialize this pallet
        let _ = clear_storage_prefix(b"ExternalValidators", b"", b"", None, None);

        // Simulate adding data to the old pallet storage
        // The value is not used for anything, we only care that it is removed by the migration.
        let old_storage_key =
            frame_support::storage::storage_prefix(old_pallet_name, b"ValidatorsToAdd");
        let expected_validators: Vec<u64> = vec![5, 6];
        unhashed::put(&old_storage_key, &expected_validators);

        // Run migration
        let _used_weight = migrate_external_validators.migrate(Weight::MAX);

        // Assert that ValidatorManager pallet prefix is empty after migration
        let old_pallet_key = frame_support::storage::storage_prefix(old_pallet_name, b"");
        let old_storage_exists = unhashed::contains_prefixed_key(&old_pallet_key);
        assert!(
            !old_storage_exists,
            "Old pallet storage should be cleared after migration"
        );

        // Assert that ExternalValidators has the validators from ValidatorManager
        let migrated_validators = ExternalValidators::validators();
        let empty = vec![];
        assert_ne!(
            migrated_validators, empty,
            "ExternalValidators should not be empty after migration"
        );

        // ExternalValidators should be equal to validators from Session::queued_keys
        let expected_validators: Vec<_> = Session::queued_keys().into_iter().map(|x| x.0).collect();
        assert_eq!(migrated_validators, expected_validators);
    });
}
