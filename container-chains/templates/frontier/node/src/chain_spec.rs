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
    container_chain_template_frontier_runtime::{
        AccountId, EVMChainIdConfig, EVMConfig, MaintenanceModeConfig, MigrationsConfig,
        PolkadotXcmConfig, Precompiles,
    },
    cumulus_primitives_core::ParaId,
    fp_evm::GenesisAccount,
    hex_literal::hex,
    sc_chain_spec::{ChainSpecExtension, ChainSpecGroup},
    sc_network::config::MultiaddrWithPeerId,
    sc_service::ChainType,
    serde::{Deserialize, Serialize},
};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<
    container_chain_template_frontier_runtime::RuntimeGenesisConfig,
    Extensions,
>;

/// Orcherstrator's parachain id
pub const ORCHESTRATOR: ParaId = ParaId::new(1000);

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
    /// The relay chain of the Parachain.
    pub relay_chain: String,
    /// The id of the Parachain.
    pub para_id: u32,
}

impl Extensions {
    /// Try to get the extension from the given `ChainSpec`.
    pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
        sc_chain_spec::get_extension(chain_spec.extensions())
    }
}

pub fn development_config(para_id: ParaId, boot_nodes: Vec<String>) -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "UNIT".into());
    properties.insert("tokenDecimals".into(), 18.into());
    properties.insert("ss58Format".into(), 42.into());
    properties.insert("isEthereum".into(), true.into());

    let mut default_funded_accounts = pre_funded_accounts();
    default_funded_accounts.sort();
    default_funded_accounts.dedup();
    let boot_nodes: Vec<MultiaddrWithPeerId> = boot_nodes
        .into_iter()
        .map(|x| {
            x.parse::<MultiaddrWithPeerId>()
                .unwrap_or_else(|e| panic!("invalid bootnode address format {:?}: {:?}", x, e))
        })
        .collect();

    ChainSpec::builder(
        container_chain_template_frontier_runtime::WASM_BINARY
            .expect("WASM binary was not built, please build it!"),
        Extensions {
            relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
            para_id: para_id.into(),
        },
    )
    .with_name("Development")
    .with_id("dev")
    .with_chain_type(ChainType::Development)
    .with_genesis_config(testnet_genesis(
        default_funded_accounts.clone(),
        para_id,
        AccountId::from(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")), // Alith
    ))
    .with_properties(properties)
    .with_boot_nodes(boot_nodes)
    .build()
}

pub fn local_testnet_config(para_id: ParaId, boot_nodes: Vec<String>) -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "UNIT".into());
    properties.insert("tokenDecimals".into(), 18.into());
    properties.insert("ss58Format".into(), 42.into());
    properties.insert("isEthereum".into(), true.into());
    let protocol_id = format!("container-chain-{}", para_id);

    let mut default_funded_accounts = pre_funded_accounts();
    default_funded_accounts.sort();
    default_funded_accounts.dedup();
    let boot_nodes: Vec<MultiaddrWithPeerId> = boot_nodes
        .into_iter()
        .map(|x| {
            x.parse::<MultiaddrWithPeerId>()
                .unwrap_or_else(|e| panic!("invalid bootnode address format {:?}: {:?}", x, e))
        })
        .collect();

    ChainSpec::builder(
        container_chain_template_frontier_runtime::WASM_BINARY
            .expect("WASM binary was not built, please build it!"),
        Extensions {
            relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
            para_id: para_id.into(),
        },
    )
    .with_name(&format!("Frontier Container {}", para_id))
    .with_id(&format!("frontier_container_{}", para_id))
    .with_chain_type(ChainType::Local)
    .with_genesis_config(testnet_genesis(
        default_funded_accounts.clone(),
        para_id,
        AccountId::from(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")), // Alith
    ))
    .with_properties(properties)
    .with_protocol_id(&protocol_id)
    .with_boot_nodes(boot_nodes)
    .build()
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
