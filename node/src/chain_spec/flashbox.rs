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
    crate::chain_spec::{
        account_ids, get_account_id_from_seed, invulnerables_from_seeds, Extensions,
    },
    cumulus_primitives_core::ParaId,
    flashbox_runtime::{
        prod_or_fast, AccountId, DataPreserversConfig, MaintenanceModeConfig, MigrationsConfig,
        RegistrarConfig, ServicesPaymentConfig, SudoConfig,
    },
    nimbus_primitives::NimbusId,
    pallet_configuration::HostConfiguration,
    sc_service::ChainType,
    sp_core::sr25519,
    sp_runtime::traits::Get,
    tp_container_chain_genesis_data::{
        json::container_chain_genesis_data_from_path, ContainerChainGenesisData,
    },
};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec =
    sc_service::GenericChainSpec<flashbox_runtime::RuntimeGenesisConfig, Extensions>;

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn template_session_keys(keys: NimbusId) -> flashbox_runtime::SessionKeys {
    flashbox_runtime::SessionKeys { nimbus: keys }
}

pub fn development_config(
    para_id: ParaId,
    container_chains: Vec<String>,
    mock_container_chains: Vec<ParaId>,
    invulnerables: Vec<String>,
) -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "FLASH".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());
    properties.insert("isEthereum".into(), false.into());

    ChainSpec::from_genesis(
        // Name
        "Flashbox Development Testnet",
        // ID
        "flashbox_dev",
        ChainType::Development,
        move || {
            testnet_genesis(
                // initial collators.
                invulnerables_from_seeds(invulnerables.iter()),
                account_ids(&[
                    "Alice",
                    "Bob",
                    "Charlie",
                    "Dave",
                    "Eve",
                    "Ferdie",
                    "Alice//stash",
                    "Bob//stash",
                    "Charlie//stash",
                    "Dave//stash",
                    "Eve//stash",
                    "Ferdie//stash",
                ]),
                para_id,
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                &container_chains,
                &mock_container_chains,
                pallet_configuration::GenesisConfig {
                    config: HostConfiguration {
                        max_collators: 100u32,
                        min_orchestrator_collators: 1u32,
                        max_orchestrator_collators: 1u32,
                        collators_per_container: 2u32,
                        full_rotation_period: prod_or_fast!(24u32, 5u32),
                    },
                    ..Default::default()
                },
            )
        },
        vec![],
        None,
        None,
        None,
        Some(properties),
        Extensions {
            relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
            para_id: para_id.into(),
        },
    )
}

pub fn local_flashbox_config(
    para_id: ParaId,
    container_chains: Vec<String>,
    mock_container_chains: Vec<ParaId>,
    invulnerables: Vec<String>,
) -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "FLASH".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());
    properties.insert("isEthereum".into(), false.into());

    ChainSpec::from_genesis(
        // Name
        "Flashbox Local Testnet",
        // ID
        "flashbox_local",
        ChainType::Local,
        move || {
            testnet_genesis(
                // initial collators.
                invulnerables_from_seeds(invulnerables.iter()),
                account_ids(&[
                    "Alice",
                    "Bob",
                    "Charlie",
                    "Dave",
                    "Eve",
                    "Ferdie",
                    "Alice//stash",
                    "Bob//stash",
                    "Charlie//stash",
                    "Dave//stash",
                    "Eve//stash",
                    "Ferdie//stash",
                ]),
                para_id,
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                &container_chains,
                &mock_container_chains,
                pallet_configuration::GenesisConfig {
                    config: HostConfiguration {
                        max_collators: 100u32,
                        min_orchestrator_collators: 2u32,
                        max_orchestrator_collators: 5u32,
                        collators_per_container: 2u32,
                        full_rotation_period: prod_or_fast!(24u32, 5u32),
                    },
                    ..Default::default()
                },
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        Some("orchestrator"),
        // Fork ID
        None,
        // Properties
        Some(properties),
        // Extensions
        Extensions {
            relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
            para_id: para_id.into(),
        },
    )
}

fn testnet_genesis(
    invulnerables: Vec<(AccountId, NimbusId)>,
    endowed_accounts: Vec<AccountId>,
    id: ParaId,
    root_key: AccountId,
    container_chains: &[String],
    mock_container_chains: &[ParaId],
    configuration: pallet_configuration::GenesisConfig<flashbox_runtime::Runtime>,
) -> flashbox_runtime::RuntimeGenesisConfig {
    let para_ids: Vec<_> = container_chains
        .iter()
        .map(|x| {
            container_chain_genesis_data_from_path(x).unwrap_or_else(|e| {
                panic!(
                    "Failed to build genesis data for container chain {:?}: {}",
                    x, e
                )
            })
        })
        .chain(
            mock_container_chains
                .iter()
                .map(|x| (*x, mock_container_chain_genesis_data(*x), vec![])),
        )
        .collect();
    // Assign 1000 block credits to all container chains registered in genesis
    let para_id_credits: Vec<_> = para_ids
        .iter()
        .map(|(para_id, _genesis_data, _boot_nodes)| (*para_id, 1000))
        .collect();
    let para_id_boot_nodes: Vec<_> = para_ids
        .iter()
        .map(|(para_id, _genesis_data, boot_nodes)| (*para_id, boot_nodes.clone()))
        .collect();
    let para_ids: Vec<_> = para_ids
        .into_iter()
        .map(|(para_id, genesis_data, _boot_nodes)| (para_id, genesis_data))
        .collect();

    let accounts_with_ed = vec![
        flashbox_runtime::StakingAccount::get(),
        flashbox_runtime::ParachainBondAccount::get(),
        flashbox_runtime::PendingRewardsAccount::get(),
    ];
    flashbox_runtime::RuntimeGenesisConfig {
        system: flashbox_runtime::SystemConfig {
            code: flashbox_runtime::WASM_BINARY
                .expect("WASM binary was not build, please build it!")
                .to_vec(),
            ..Default::default()
        },
        balances: flashbox_runtime::BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1 << 60))
                .chain(
                    accounts_with_ed
                        .iter()
                        .cloned()
                        .map(|k| (k, flashbox_runtime::EXISTENTIAL_DEPOSIT)),
                )
                .collect(),
        },
        parachain_info: flashbox_runtime::ParachainInfoConfig {
            parachain_id: id,
            ..Default::default()
        },
        invulnerables: flashbox_runtime::InvulnerablesConfig {
            invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
        },
        session: flashbox_runtime::SessionConfig {
            keys: invulnerables
                .into_iter()
                .map(|(acc, aura)| {
                    (
                        acc.clone(),                 // account id
                        acc,                         // validator id
                        template_session_keys(aura), // session keys
                    )
                })
                .collect(),
        },
        parachain_system: Default::default(),
        configuration,
        data_preservers: DataPreserversConfig {
            para_id_boot_nodes,
            ..Default::default()
        },
        registrar: RegistrarConfig { para_ids },
        services_payment: ServicesPaymentConfig { para_id_credits },
        sudo: SudoConfig {
            key: Some(root_key),
        },
        migrations: MigrationsConfig {
            ..Default::default()
        },
        maintenance_mode: MaintenanceModeConfig {
            start_in_maintenance_mode: false,
            ..Default::default()
        },
        transaction_payment: Default::default(),
        tx_pause: Default::default(),
    }
}

fn mock_container_chain_genesis_data<MaxLengthTokenSymbol: Get<u32>>(
    para_id: ParaId,
) -> ContainerChainGenesisData<MaxLengthTokenSymbol> {
    ContainerChainGenesisData {
        storage: vec![],
        name: format!("Container Chain {}", para_id).into(),
        id: format!("container-chain-{}", para_id).into(),
        fork_id: None,
        extensions: vec![],
        properties: Default::default(),
    }
}

/// Can be called for a `Configuration` to check if it is a configuration for
/// the `Tanssi` network.
pub trait IdentifyVariant {
    /// Returns `true` if this is a configuration for the `Flashbox` network.
    fn is_flashbox(&self) -> bool;
    /// Returns `true` if this is a configuration for a dev network.
    fn is_dev(&self) -> bool;
}

impl IdentifyVariant for Box<dyn sc_service::ChainSpec> {
    fn is_flashbox(&self) -> bool {
        self.id().starts_with("flashbox")
    }

    fn is_dev(&self) -> bool {
        self.chain_type() == sc_chain_spec::ChainType::Development
    }
}
