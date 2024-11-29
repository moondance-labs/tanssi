// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot. If not, see <http://www.gnu.org/licenses/>.

//! Tests for the Dancelight Runtime Configuration

use pallet_migrations::Migration;
use {crate::*, std::collections::HashSet};

use crate::tests::common::ExtBuilder;
use tanssi_runtime_common::migrations::{
    HostConfigurationV3, MigrateConfigurationAddFullRotationMode,
};
use {frame_support::traits::WhitelistedStorageKeys, sp_core::hexdisplay::HexDisplay};

mod author_noting_tests;
mod beefy;
mod collator_assignment_tests;
mod common;
mod core_scheduling_tests;
mod ethereum_client;
mod external_validators_tests;
mod inbound_queue_tests;
mod inflation_rewards;
mod integration_test;
mod migrations_test;
mod relay_configuration;
mod relay_registrar;
mod services_payment;
mod session_keys;
mod slashes;
mod sudo;

#[test]
fn check_whitelist() {
    let whitelist: HashSet<String> = AllPalletsWithSystem::whitelisted_storage_keys()
        .iter()
        .map(|e| HexDisplay::from(&e.key).to_string())
        .collect();

    // Block number
    assert!(whitelist.contains("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac"));
    // Total issuance
    assert!(whitelist.contains("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80"));
    // Execution phase
    assert!(whitelist.contains("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a"));
    // Event count
    assert!(whitelist.contains("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850"));
    // System events
    assert!(whitelist.contains("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7"));
    // XcmPallet VersionDiscoveryQueue
    assert!(whitelist.contains("1405f2411d0af5a7ff397e7c9dc68d194a222ba0333561192e474c59ed8e30e1"));
    // XcmPallet SafeXcmVersion
    assert!(whitelist.contains("1405f2411d0af5a7ff397e7c9dc68d196323ae84c43568be0d1394d5d0d522c4"));
}

#[test]
fn check_treasury_pallet_id() {
    assert_eq!(
        <Treasury as frame_support::traits::PalletInfoAccess>::index() as u8,
        dancelight_runtime_constants::TREASURY_PALLET_ID
    );
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
