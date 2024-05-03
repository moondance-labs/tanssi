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
    dancebox_runtime::{
        prod_or_fast, AccountId, DataPreserversConfig, MaintenanceModeConfig, MigrationsConfig,
        PolkadotXcmConfig, RegistrarConfig, ServicesPaymentConfig, SudoConfig,
    },
    nimbus_primitives::NimbusId,
    pallet_configuration::HostConfiguration,
    sc_service::ChainType,
    sp_core::sr25519,
    sp_runtime::{traits::Get, Perbill},
    tp_container_chain_genesis_data::{
        json::container_chain_genesis_data_from_path, ContainerChainGenesisData,
    },
};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec =
    sc_service::GenericChainSpec<dancebox_runtime::RuntimeGenesisConfig, Extensions>;

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn template_session_keys(keys: NimbusId) -> dancebox_runtime::SessionKeys {
    dancebox_runtime::SessionKeys { nimbus: keys }
}

pub fn development_config(
    para_id: ParaId,
    container_chains: Vec<String>,
    mock_container_chains: Vec<ParaId>,
    invulnerables: Vec<String>,
) -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "DANCE".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());
    properties.insert("isEthereum".into(), false.into());

    ChainSpec::builder(
        dancebox_runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
        Extensions {
            relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
            para_id: para_id.into(),
        },
    )
    .with_name("Dancebox Development Testnet")
    .with_id("dancebox_dev")
    .with_chain_type(ChainType::Development)
    .with_genesis_config(testnet_genesis(
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
                collators_per_parathread: 1,
                parathreads_per_collator: 1,
                target_container_chain_fullness: Perbill::from_percent(80),
            },
            ..Default::default()
        },
    ))
    .with_properties(properties)
    .build()
}

pub fn local_dancebox_config(
    para_id: ParaId,
    container_chains: Vec<String>,
    mock_container_chains: Vec<ParaId>,
    invulnerables: Vec<String>,
) -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "DANCE".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());
    properties.insert("isEthereum".into(), false.into());

    ChainSpec::builder(
        dancebox_runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
        Extensions {
            relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
            para_id: para_id.into(),
        },
    )
    .with_name("Dancebox Local Testnet")
    .with_id("dancebox_local")
    .with_chain_type(ChainType::Local)
    .with_genesis_config(testnet_genesis(
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
                collators_per_parathread: 1,
                parathreads_per_collator: 1,
                target_container_chain_fullness: Perbill::from_percent(80),
            },
            ..Default::default()
        },
    ))
    .with_properties(properties)
    .with_protocol_id("orchestrator")
    .build()
}

fn testnet_genesis(
    invulnerables: Vec<(AccountId, NimbusId)>,
    endowed_accounts: Vec<AccountId>,
    id: ParaId,
    root_key: AccountId,
    container_chains: &[String],
    mock_container_chains: &[ParaId],
    configuration: pallet_configuration::GenesisConfig<dancebox_runtime::Runtime>,
) -> serde_json::Value {
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
    // Assign 100 collator assignment credits to all container chains registered in genesis
    let para_id_credits: Vec<_> = para_ids
        .iter()
        .map(|(para_id, _genesis_data, _boot_nodes)| (*para_id, 1000, 100).into())
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
        dancebox_runtime::StakingAccount::get(),
        dancebox_runtime::ParachainBondAccount::get(),
        dancebox_runtime::PendingRewardsAccount::get(),
    ];
    let g = dancebox_runtime::RuntimeGenesisConfig {
        system: Default::default(),
        balances: dancebox_runtime::BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1 << 60))
                .chain(
                    accounts_with_ed
                        .iter()
                        .cloned()
                        .map(|k| (k, dancebox_runtime::EXISTENTIAL_DEPOSIT)),
                )
                .collect(),
        },
        parachain_info: dancebox_runtime::ParachainInfoConfig {
            parachain_id: id,
            ..Default::default()
        },
        invulnerables: dancebox_runtime::InvulnerablesConfig {
            invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
        },
        session: dancebox_runtime::SessionConfig {
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
        // This should initialize it to whatever we have set in the pallet
        polkadot_xcm: PolkadotXcmConfig::default(),
        transaction_payment: Default::default(),
        tx_pause: Default::default(),
        treasury: Default::default(),
    };

    serde_json::to_value(g).unwrap()
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
