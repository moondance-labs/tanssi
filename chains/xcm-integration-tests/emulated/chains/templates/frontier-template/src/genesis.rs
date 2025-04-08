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
    container_chain_template_frontier_runtime::AccountId,
    emulated_integration_tests_common::build_genesis_storage, hex_literal::hex,
};

pub const PARA_ID: u32 = 2001;
pub const ORCHESTRATOR: u32 = 2000;

pub fn genesis() -> sp_core::storage::Storage {
    let genesis_config = container_chain_template_frontier_runtime::RuntimeGenesisConfig {
        system: Default::default(),
        balances: container_chain_template_frontier_runtime::BalancesConfig {
            balances: pre_funded_accounts()
                .iter()
                .cloned()
                .map(|k| (k, 1 << 80))
                .collect(),
        },
        parachain_info: container_chain_template_frontier_runtime::ParachainInfoConfig {
            parachain_id: PARA_ID.into(),
            ..Default::default()
        },
        // EVM compatibility
        // We should change this to something different than Moonbeam
        // For now moonwall is very tailored for moonbeam so we need it for tests
        evm_chain_id: container_chain_template_frontier_runtime::EVMChainIdConfig {
            chain_id: 1281,
            ..Default::default()
        },
        sudo: container_chain_template_frontier_runtime::SudoConfig {
            key: Some(pre_funded_accounts()[0]),
        },
        authorities_noting: container_chain_template_frontier_runtime::AuthoritiesNotingConfig {
            orchestrator_para_id: ORCHESTRATOR.into(),
            ..Default::default()
        },
        ..Default::default()
    };

    build_genesis_storage(
        &genesis_config,
        container_chain_template_frontier_runtime::WASM_BINARY.unwrap(),
    )
}
/// Get pre-funded accounts
pub fn pre_funded_accounts() -> Vec<AccountId> {
    // These addresses are derived from Substrate's canonical mnemonic:
    // bottom drive obey lake curtain smoke basket hold race lonely fit walk
    vec![
        AccountId::from(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")), // Alith
        AccountId::from(hex!("3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0")), // Baltathar
        AccountId::from(hex!("798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc")), // Charleth
        AccountId::from(hex!("773539d4Ac0e786233D90A233654ccEE26a613D9")), // Dorothy
    ]
}
