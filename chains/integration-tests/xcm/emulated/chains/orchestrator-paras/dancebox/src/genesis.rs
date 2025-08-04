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

use {
    cumulus_primitives_core::Junctions::X1,
    emulated_integration_tests_common::build_genesis_storage,
    pallet_configuration::HostConfiguration,
    sp_core::storage::Storage,
    tanssi_emulated_integration_tests_common::accounts::{get_aura_id_from_seed, ALICE, BOB},
    xcm::prelude::*,
    xcm_builder::{ParentIsPreset, SiblingParachainConvertsVia},
    xcm_executor::traits::ConvertLocation,
};

pub fn genesis() -> Storage {
    let genesis_config = dancebox_runtime::RuntimeGenesisConfig {
        balances: dancebox_runtime::BalancesConfig {
            balances: vec![
                // Alice gets 10k extra tokens for her mapping deposit
                (
                    dancebox_runtime::AccountId::from(ALICE),
                    210_000 * dancebox_runtime::UNIT,
                ),
                (
                    dancebox_runtime::AccountId::from(BOB),
                    100_000 * dancebox_runtime::UNIT,
                ),
                // Give some balance to the relay chain account
                (
                    ParentIsPreset::<dancebox_runtime::AccountId>::convert_location(
                        &Location::parent(),
                    )
                    .unwrap(),
                    100_000 * dancebox_runtime::UNIT,
                ),
                // And to sovereigns
                (
                    SiblingParachainConvertsVia::<
                        polkadot_parachain_primitives::primitives::Sibling,
                        dancebox_runtime::AccountId,
                    >::convert_location(&Location {
                        parents: 1,
                        interior: X1([Parachain(2001u32)].into()),
                    })
                    .unwrap(),
                    100_000 * dancebox_runtime::UNIT,
                ),
                (
                    SiblingParachainConvertsVia::<
                        polkadot_parachain_primitives::primitives::Sibling,
                        dancebox_runtime::AccountId,
                    >::convert_location(&Location {
                        parents: 1,
                        interior: X1([Parachain(2002u32)].into()),
                    })
                    .unwrap(),
                    100_000 * dancebox_runtime::UNIT,
                ),
            ],
            ..Default::default()
        },
        configuration: dancebox_runtime::ConfigurationConfig {
            config: HostConfiguration {
                max_collators: 100,
                min_orchestrator_collators: 1,
                max_orchestrator_collators: 1,
                collators_per_container: 1,
                collators_per_parathread: 1,
                full_rotation_period: 0,
                ..Default::default()
            },
            ..Default::default()
        },
        invulnerables: dancebox_runtime::InvulnerablesConfig {
            invulnerables: vec![
                (
                    dancebox_runtime::AccountId::from(ALICE),
                    210 * dancebox_runtime::UNIT,
                ),
                (
                    dancebox_runtime::AccountId::from(BOB),
                    100 * dancebox_runtime::UNIT,
                ),
            ]
            .clone()
            .into_iter()
            .map(|(account, _balance)| account)
            .collect(),
        },
        parachain_info: dancebox_runtime::ParachainInfoConfig {
            parachain_id: 2000u32.into(),
            ..Default::default()
        },
        polkadot_xcm: dancebox_runtime::PolkadotXcmConfig {
            safe_xcm_version: 3.into(),
            ..Default::default()
        },
        session: dancebox_runtime::SessionConfig {
            keys: vec![
                (
                    dancebox_runtime::AccountId::from(ALICE),
                    210 * dancebox_runtime::UNIT,
                ),
                (
                    dancebox_runtime::AccountId::from(BOB),
                    100 * dancebox_runtime::UNIT,
                ),
            ]
            .into_iter()
            .map(|(account, _balance)| {
                let nimbus_id = get_aura_id_from_seed(&account.to_string());
                (
                    account.clone(),
                    account,
                    dancebox_runtime::SessionKeys { nimbus: nimbus_id },
                )
            })
            .collect(),
            ..Default::default()
        },
        ..Default::default()
    };

    let mut storage =
        build_genesis_storage(&genesis_config, dancebox_runtime::WASM_BINARY.unwrap());

    storage
        .top
        .insert(b"__mock_is_xcm_test".to_vec(), b"1".to_vec());

    storage
}
