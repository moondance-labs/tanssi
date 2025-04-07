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
    container_chain_template_simple_runtime::UNIT as DEV,
    emulated_integration_tests_common::build_genesis_storage,
    tanssi_emulated_integration_tests_common::accounts,
};

pub const PARA_ID: u32 = 2001;
pub const ORCHESTRATOR: u32 = 2000;
const ENDOWMENT: u128 = 1_000_000 * DEV;

pub fn genesis() -> sp_core::storage::Storage {
    let genesis_config = container_chain_template_simple_runtime::RuntimeGenesisConfig {
        balances: container_chain_template_simple_runtime::BalancesConfig {
            balances: accounts::init_balances()
                .iter()
                .cloned()
                .map(|k| (k, ENDOWMENT))
                .collect(),
        },
        parachain_info: container_chain_template_simple_runtime::ParachainInfoConfig {
            parachain_id: PARA_ID.into(),
            ..Default::default()
        },
        sudo: container_chain_template_simple_runtime::SudoConfig {
            key: Some(accounts::init_balances()[0].clone()),
        },
        authorities_noting: container_chain_template_simple_runtime::AuthoritiesNotingConfig {
            orchestrator_para_id: ORCHESTRATOR.into(),
            ..Default::default()
        },
        ..Default::default()
    };
    build_genesis_storage(
        &genesis_config,
        container_chain_template_simple_runtime::WASM_BINARY.unwrap(),
    )
}
