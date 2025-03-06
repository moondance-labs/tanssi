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

use crate::xcm_config::UniversalLocation;
use crate::EthereumLocation;
use snowbridge_core::TokenIdOf;
use xcm::latest::Junction::GlobalConsensus;
use xcm::latest::Junctions::Here;
use xcm::latest::Junctions::X1;
use xcm::latest::NetworkId;
use xcm::latest::{Location, Reanchorable, ROCOCO_GENESIS_HASH};
use xcm_executor::traits::ConvertLocation;
use {
    crate::{
        tests::common::ExtBuilder, BeefyMmrLeaf, CollatorConfiguration, ExternalValidators,
        PalletInfo, Runtime, Session,
    },
    beefy_primitives::mmr::BeefyAuthoritySet,
    frame_support::{migration::clear_storage_prefix, storage::unhashed, traits::PalletInfo as _},
    pallet_migrations::Migration,
    parity_scale_codec::Encode,
    sp_arithmetic::Perbill,
    tanssi_runtime_common::migrations::{
        BondedErasTimestampMigration, ExternalValidatorsInitialMigration, HostConfigurationV3,
        MigrateConfigurationAddFullRotationMode, MigrateMMRLeafPallet,
        SnowbridgeEthereumSystemXcmV5,
    },
    xcm::v3::Weight,
};

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

#[test]
fn test_migration_config_add_full_rotation_mode() {
    ExtBuilder::default().build().execute_with(|| {
        const CONFIGURATION_ACTIVE_CONFIG_KEY: &[u8] =
            &hex_literal::hex!("86e86c1d728ee2b18f76dd0e04d96cdbb4b49d95320d9021994c850f25b8e385");
        const CONFIGURATION_PENDING_CONFIGS_KEY: &[u8] =
            &hex_literal::hex!("86e86c1d728ee2b18f76dd0e04d96cdb53b4123b2e186e07fb7bad5dda5f55c0");

        // Modify active config
        frame_support::storage::unhashed::put_raw(
            CONFIGURATION_ACTIVE_CONFIG_KEY,
            &HostConfigurationV3 {
                max_collators: 5,
                min_orchestrator_collators: 2,
                max_orchestrator_collators: 1,
                collators_per_container: 3,
                full_rotation_period: 4,
                collators_per_parathread: 2,
                parathreads_per_collator: 1,
                target_container_chain_fullness: Perbill::from_percent(45),
                max_parachain_cores_percentage: Some(Perbill::from_percent(75)),
            }
            .encode(),
        );
        // Modify pending configs
        frame_support::storage::unhashed::put_raw(
            CONFIGURATION_PENDING_CONFIGS_KEY,
            &vec![
                (
                    1234u32,
                    HostConfigurationV3 {
                        max_collators: 1,
                        min_orchestrator_collators: 4,
                        max_orchestrator_collators: 45,
                        collators_per_container: 5,
                        full_rotation_period: 1,
                        collators_per_parathread: 1,
                        parathreads_per_collator: 1,
                        target_container_chain_fullness: Perbill::from_percent(65),
                        max_parachain_cores_percentage: Some(Perbill::from_percent(75)),
                    },
                ),
                (
                    5678u32,
                    HostConfigurationV3 {
                        max_collators: 1,
                        min_orchestrator_collators: 4,
                        max_orchestrator_collators: 45,
                        collators_per_container: 5,
                        full_rotation_period: 1,
                        collators_per_parathread: 1,
                        parathreads_per_collator: 1,
                        target_container_chain_fullness: Perbill::from_percent(65),
                        max_parachain_cores_percentage: Some(Perbill::from_percent(75)),
                    },
                ),
            ]
            .encode(),
        );

        let migration = MigrateConfigurationAddFullRotationMode::<Runtime>(Default::default());
        migration.migrate(Default::default());

        let expected_active = pallet_configuration::HostConfiguration {
            max_collators: 5,
            min_orchestrator_collators: 2,
            max_orchestrator_collators: 1,
            collators_per_container: 3,
            full_rotation_period: 4,
            collators_per_parathread: 2,
            parathreads_per_collator: 1,
            target_container_chain_fullness: Perbill::from_percent(45),
            max_parachain_cores_percentage: Some(Perbill::from_percent(75)),
            ..Default::default()
        };
        assert_eq!(CollatorConfiguration::config(), expected_active);

        let expected_pending = vec![
            (
                1234u32,
                pallet_configuration::HostConfiguration {
                    max_collators: 1,
                    min_orchestrator_collators: 4,
                    max_orchestrator_collators: 45,
                    collators_per_container: 5,
                    full_rotation_period: 1,
                    collators_per_parathread: 1,
                    parathreads_per_collator: 1,
                    target_container_chain_fullness: Perbill::from_percent(65),
                    max_parachain_cores_percentage: Some(Perbill::from_percent(75)),
                    ..Default::default()
                },
            ),
            (
                5678u32,
                pallet_configuration::HostConfiguration {
                    max_collators: 1,
                    min_orchestrator_collators: 4,
                    max_orchestrator_collators: 45,
                    collators_per_container: 5,
                    full_rotation_period: 1,
                    collators_per_parathread: 1,
                    parathreads_per_collator: 1,
                    target_container_chain_fullness: Perbill::from_percent(65),
                    max_parachain_cores_percentage: Some(Perbill::from_percent(75)),
                    ..Default::default()
                },
            ),
        ];
        assert_eq!(CollatorConfiguration::pending_configs(), expected_pending);
    });
}

#[test]
fn test_add_timestamp_to_bonded_eras_migration() {
    ExtBuilder::default().build().execute_with(|| {
        let bonded_eras_key =
            pallet_external_validator_slashes::BondedEras::<Runtime>::hashed_key();

        let previous_value: Vec<(sp_staking::EraIndex, sp_staking::SessionIndex)> =
            vec![(1, 1), (2, 2), (3, 3)];

        // Modify storage to pevious value
        frame_support::storage::unhashed::put_raw(&bonded_eras_key, &previous_value.encode());

        let migration = BondedErasTimestampMigration::<Runtime>(Default::default());
        migration.migrate(Default::default());

        let expected_bonded_eras_after: Vec<(sp_staking::EraIndex, sp_staking::SessionIndex, u64)> =
            previous_value
                .iter()
                .map(|(era, session)| (*era, *session, 0u64))
                .collect();
        assert_eq!(
            pallet_external_validator_slashes::BondedEras::<Runtime>::get(),
            expected_bonded_eras_after
        );
    });
}

#[test]
fn test_snowbridge_ethereum_system_xcm_v5_migration() {
    ExtBuilder::default().build().execute_with(|| {
        // Raw values copied from stagelight runtime 1100
        const FOREIGN_TO_NATIVE_ID_KEY: &[u8] =
            &hex_literal::hex!("ccee781f0b9380204db9882d1b1c771d53e99bc228247291bd1d0d34fa7f53993faff06e7c800c84bd5d1f5ea566d14962e8f33b7fb0e7e2d2276564061a2f3c7bcb612e733b8bf5733ea16cee0ecba6");
        const FOREIGN_TO_NATIVE_ID_VALUE: &[u8] =
            &hex_literal::hex!("01010905");
        const NATIVE_TO_FOREIGN_ID_KEY: &[u8] =
            &hex_literal::hex!("ccee781f0b9380204db9882d1b1c771ddec2be471806c468b349224cf542e742627f68650cbf12ff5b6ab3d5751bc1ea01010905");
        const NATIVE_TO_FOREIGN_ID_VALUE: &[u8] =
            &hex_literal::hex!("62e8f33b7fb0e7e2d2276564061a2f3c7bcb612e733b8bf5733ea16cee0ecba6");

        // Write foreign assets to pallet storage
        frame_support::storage::unhashed::put_raw(
            FOREIGN_TO_NATIVE_ID_KEY,
            FOREIGN_TO_NATIVE_ID_VALUE,
        );
        frame_support::storage::unhashed::put_raw(
            NATIVE_TO_FOREIGN_ID_KEY,
            NATIVE_TO_FOREIGN_ID_VALUE,
        );

        let migration = SnowbridgeEthereumSystemXcmV5::<Runtime>(Default::default());
        migration.migrate(Default::default());

        let f_n = snowbridge_pallet_system::ForeignToNativeId::<Runtime>::iter().collect::<Vec<_>>();
        let n_f = snowbridge_pallet_system::NativeToForeignId::<Runtime>::iter().collect::<Vec<_>>();

        assert_eq!(f_n, [(
            hex_literal::hex!("62e8f33b7fb0e7e2d2276564061a2f3c7bcb612e733b8bf5733ea16cee0ecba6").into(),
            Location {
                parents: 1,
                interior: X1([GlobalConsensus(NetworkId::ByGenesis(ROCOCO_GENESIS_HASH))].into()),
            },
        )]);
        assert_eq!(n_f, [(
            Location {
                parents: 1,
                interior: X1([GlobalConsensus(NetworkId::ByGenesis(ROCOCO_GENESIS_HASH))].into()),
            },
            hex_literal::hex!("62e8f33b7fb0e7e2d2276564061a2f3c7bcb612e733b8bf5733ea16cee0ecba6").into(),
        )]);
    });
}

#[test]
fn snowbridge_ethereum_system_token_id_does_not_change() {
    // If a new XCM version is released, we need to check that this location still encodes to the same
    // token id.
    // If this test fails, we need a migration in snowbridge_pallet_system to migrate the mappings
    // ForeignToNativeId and NativeToForeignId. Use migration SnowbridgeEthereumSystemXcmV5 as base.

    // Location of native starlight token.
    // When we support other token locations, maybe add them to this test as well.
    let location = Location {
        parents: 0,
        interior: Here,
    };

    let ethereum_location = EthereumLocation::get();
    // reanchor to Ethereum context
    // This means to add this junction: GlobalConsensus(NetworkId::ByGenesis(ROCOCO_GENESIS_HASH))
    let location = location
        .clone()
        .reanchored(&ethereum_location, &UniversalLocation::get())
        .unwrap();

    let token_id = TokenIdOf::convert_location(&location).unwrap();

    // The token id from stagelight has been derived using xcm v4, but we are in xcm v5.
    // The derived token id from xcm v4 (the one you will find on chain) is:
    // 0x62e8f33b7fb0e7e2d2276564061a2f3c7bcb612e733b8bf5733ea16cee0ecba6
    // So the exact token id from below is not important, as long as it does not change:
    assert_eq!(
        token_id,
        hex_literal::hex!("bcd4282ca0c30cbd9c578b5c790e88c803d80cd9cc91f28686f24ac25a61e06e")
            .into()
    );
}
