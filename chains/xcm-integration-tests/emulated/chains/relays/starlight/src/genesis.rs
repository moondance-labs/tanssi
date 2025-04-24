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
    emulated_integration_tests_common::build_genesis_storage, sp_core::storage::Storage,
    starlight_runtime_constants::currency::UNITS as DANCE,
};
const INITIAL_BALANCE: u128 = 1_000_000 * DANCE;

pub fn genesis() -> Storage {
    let genesis_config = starlight_runtime::RuntimeGenesisConfig {
        balances: starlight_runtime::BalancesConfig {
            balances: tanssi_emulated_integration_tests_common::accounts::init_balances()
                .iter()
                .cloned()
                .map(|k| (k, INITIAL_BALANCE))
                .collect(),
        },
        babe: starlight_runtime::BabeConfig {
            authorities: Default::default(),
            epoch_config: starlight_runtime::BABE_GENESIS_EPOCH_CONFIG,
            ..Default::default()
        },
        configuration: starlight_runtime::ConfigurationConfig {
            config:
                starlight_runtime::genesis_config_presets::default_parachains_host_configuration(),
        },
        ..Default::default()
    };
    build_genesis_storage(&genesis_config, starlight_runtime::WASM_BINARY.unwrap())
}
