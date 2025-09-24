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
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>.

use {
    dancelight_runtime_constants::currency::UNITS as DANCE,
    emulated_integration_tests_common::build_genesis_storage,
    polkadot_parachain_primitives::primitives::ValidationCode,
    runtime_parachains::paras::{ParaGenesisArgs, ParaKind},
    sp_core::storage::Storage,
};
pub const INITIAL_BALANCE: u128 = 1_000_000 * DANCE;

pub fn genesis() -> Storage {
    let genesis_config = dancelight_runtime::RuntimeGenesisConfig {
        balances: dancelight_runtime::BalancesConfig {
            balances: tanssi_emulated_integration_tests_common::accounts::init_balances()
                .iter()
                .chain(std::iter::once(
                    &dancelight_runtime::SnowbridgeFeesAccount::get(),
                ))
                .cloned()
                .map(|k| (k, INITIAL_BALANCE))
                .collect(),
            ..Default::default()
        },
        babe: dancelight_runtime::BabeConfig {
            authorities: Default::default(),
            epoch_config: dancelight_runtime::BABE_GENESIS_EPOCH_CONFIG,
            ..Default::default()
        },
        configuration: dancelight_runtime::ConfigurationConfig {
            config:
                dancelight_runtime::genesis_config_presets::default_parachains_host_configuration(),
        },
        paras: dancelight_runtime::ParasConfig {
            _config: Default::default(),
            paras: vec![
                (
                    2001.into(),
                    ParaGenesisArgs {
                        genesis_head: Default::default(),
                        validation_code: ValidationCode(vec![1, 1, 2, 3, 4]),
                        para_kind: ParaKind::Parachain,
                    },
                ),
                (
                    2002.into(),
                    ParaGenesisArgs {
                        genesis_head: Default::default(),
                        validation_code: ValidationCode(vec![1, 1, 2, 3, 4]),
                        para_kind: ParaKind::Parachain,
                    },
                ),
            ],
        },
        ..Default::default()
    };
    build_genesis_storage(&genesis_config, dancelight_runtime::WASM_BINARY.unwrap())
}
