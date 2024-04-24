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
    container_chain_template_simple_runtime::{
        AccountId, MaintenanceModeConfig, MigrationsConfig, PolkadotXcmConfig, Signature,
    },
    cumulus_primitives_core::ParaId,
    sc_chain_spec::{ChainSpecExtension, ChainSpecGroup},
    sc_network::config::MultiaddrWithPeerId,
    sc_service::ChainType,
    serde::{Deserialize, Serialize},
    sp_core::{sr25519, Pair, Public},
    sp_runtime::traits::{IdentifyAccount, Verify},
};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<
    container_chain_template_simple_runtime::RuntimeGenesisConfig,
    Extensions,
>;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

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

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

pub fn development_config(para_id: ParaId, boot_nodes: Vec<String>) -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "UNIT".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());
    properties.insert("isEthereum".into(), false.into());

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
        container_chain_template_simple_runtime::WASM_BINARY
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
        get_account_id_from_seed::<sr25519::Public>("Alice"),
    ))
    .with_properties(properties)
    .with_boot_nodes(boot_nodes)
    .build()
}

pub fn local_testnet_config(para_id: ParaId, boot_nodes: Vec<String>) -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "UNIT".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());
    properties.insert("isEthereum".into(), false.into());
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
        container_chain_template_simple_runtime::WASM_BINARY
            .expect("WASM binary was not built, please build it!"),
        Extensions {
            relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
            para_id: para_id.into(),
        },
    )
    .with_name(&format!("Simple Container {}", para_id))
    .with_id(&format!("simple_container_{}", para_id))
    .with_chain_type(ChainType::Local)
    .with_genesis_config(testnet_genesis(
        default_funded_accounts.clone(),
        para_id,
        get_account_id_from_seed::<sr25519::Public>("Alice"),
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
    let g = container_chain_template_simple_runtime::RuntimeGenesisConfig {
        balances: container_chain_template_simple_runtime::BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1 << 60))
                .collect(),
        },
        parachain_info: container_chain_template_simple_runtime::ParachainInfoConfig {
            parachain_id: id,
            ..Default::default()
        },
        parachain_system: Default::default(),
        sudo: container_chain_template_simple_runtime::SudoConfig {
            key: Some(root_key),
        },
        authorities_noting: container_chain_template_simple_runtime::AuthoritiesNotingConfig {
            orchestrator_para_id: ORCHESTRATOR,
            ..Default::default()
        },
        migrations: MigrationsConfig::default(),
        maintenance_mode: MaintenanceModeConfig {
            start_in_maintenance_mode: false,
            ..Default::default()
        },
        // This should initialize it to whatever we have set in the pallet
        polkadot_xcm: PolkadotXcmConfig::default(),
        transaction_payment: Default::default(),
        tx_pause: Default::default(),
        system: Default::default(),
    };

    serde_json::to_value(g).unwrap()
}

/// Get pre-funded accounts
pub fn pre_funded_accounts() -> Vec<AccountId> {
    vec![
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        get_account_id_from_seed::<sr25519::Public>("Bob"),
        get_account_id_from_seed::<sr25519::Public>("Charlie"),
        get_account_id_from_seed::<sr25519::Public>("Dave"),
        get_account_id_from_seed::<sr25519::Public>("Eve"),
        get_account_id_from_seed::<sr25519::Public>("Ferdie"),
        get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
        get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
        get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
        get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
        get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
        get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
    ]
}
