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
    crate as container_chain_template_frontier_runtime,
    crate::{
        AccountId, EVMChainIdConfig, EVMConfig, EthereumNetwork, MaintenanceModeConfig,
        MigrationsConfig, PolkadotXcmConfig, Precompiles,
    },
    alloc::{vec, vec::Vec},
    cumulus_primitives_core::{GlobalConsensus, Junctions::X1, Location, ParaId},
    fp_evm::GenesisAccount,
    hex_literal::hex,
};

/// Orcherstrator's parachain id
pub const ORCHESTRATOR: ParaId = ParaId::new(1000);

pub fn local(
    endowed_accounts: Vec<AccountId>,
    id: ParaId,
    root_key: AccountId,
) -> serde_json::Value {
    testnet_genesis(endowed_accounts, id, root_key)
}

pub fn development(
    endowed_accounts: Vec<AccountId>,
    id: ParaId,
    root_key: AccountId,
) -> serde_json::Value {
    testnet_genesis(endowed_accounts, id, root_key)
}

fn testnet_genesis(
    endowed_accounts: Vec<AccountId>,
    id: ParaId,
    root_key: AccountId,
) -> serde_json::Value {
    // This is the simplest bytecode to revert without returning any data.
    // We will pre-deploy it under all of our precompiles to ensure they can be called from
    // within contracts.
    // (PUSH1 0x00 PUSH1 0x00 REVERT)
    let revert_bytecode = vec![0x60, 0x00, 0x60, 0x00, 0xFD];

    let g = container_chain_template_frontier_runtime::RuntimeGenesisConfig {
        system: Default::default(),
        balances: container_chain_template_frontier_runtime::BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1 << 80))
                .collect(),
            ..Default::default()
        },
        parachain_info: container_chain_template_frontier_runtime::ParachainInfoConfig {
            parachain_id: id,
            ..Default::default()
        },
        parachain_system: Default::default(),
        // EVM compatibility
        // We should change this to something different than Moonbeam
        // For now moonwall is very tailored for moonbeam so we need it for tests
        evm_chain_id: EVMChainIdConfig {
            chain_id: 1281,
            ..Default::default()
        },
        evm: EVMConfig {
            // We need _some_ code inserted at the precompile address so that
            // the evm will actually call the address.
            accounts: Precompiles::used_addresses()
                .map(|addr| {
                    (
                        addr.into(),
                        GenesisAccount {
                            nonce: Default::default(),
                            balance: Default::default(),
                            storage: Default::default(),
                            code: revert_bytecode.clone(),
                        },
                    )
                })
                .collect(),
            ..Default::default()
        },
        ethereum: Default::default(),
        base_fee: Default::default(),
        transaction_payment: Default::default(),
        sudo: container_chain_template_frontier_runtime::SudoConfig {
            key: Some(root_key),
        },
        authorities_noting: container_chain_template_frontier_runtime::AuthoritiesNotingConfig {
            orchestrator_para_id: ORCHESTRATOR,
            ..Default::default()
        },
        migrations: MigrationsConfig {
            ..Default::default()
        },
        maintenance_mode: MaintenanceModeConfig {
            start_in_maintenance_mode: false,
            ..Default::default()
        },
        foreign_assets_creator: pallet_foreign_asset_creator::GenesisConfig {
            // foreign_asset, asset_id, admin, is_sufficient, min_balance
            assets: vec![
                // ETH
                (
                    Location {
                        parents: 2,
                        interior: X1([GlobalConsensus(EthereumNetwork::get())].into()),
                    },
                    1, // ETH local asset id
                    root_key,
                    true,
                    1,
                ),
                // TANSSI
                (
                    Location::parent(), // native token of parent chain (orchestrator)
                    2,                  // TANSSI local asset id
                    root_key,
                    true,
                    1,
                ),
            ],
        },
        // This should initialize it to whatever we have set in the pallet
        polkadot_xcm: PolkadotXcmConfig::default(),
        tx_pause: Default::default(),
    };

    serde_json::to_value(g).unwrap()
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
