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
    container_chain_template_simple_runtime::{AccountId, Signature},
    cumulus_primitives_core::ParaId,
    nimbus_primitives::NimbusId,
    sc_chain_spec::{ChainSpecExtension, ChainSpecGroup},
    sc_service::ChainType,
    serde::{Deserialize, Serialize},
    sp_core::{sr25519, Pair, Public},
    sp_runtime::traits::{IdentifyAccount, Verify},
};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<
    container_chain_template_simple_runtime::GenesisConfig,
    Extensions,
>;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Orcherstrator's parachain id
const ORCHESTRATOR: ParaId = ParaId::new(1000);

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

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> NimbusId {
    get_from_seed::<NimbusId>(seed)
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn template_session_keys(
    keys: NimbusId,
) -> container_chain_template_simple_runtime::SessionKeys {
    container_chain_template_simple_runtime::SessionKeys { aura: keys }
}

pub fn development_config(para_id: ParaId, seeds: Option<Vec<String>>) -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "UNIT".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());
    properties.insert("isEthereum".into(), false.into());

    let initial_collator_seeds = seeds.unwrap_or(vec!["Alice".to_string(), "Bob".to_string()]);
    let collator_accounts: Vec<AccountId> = initial_collator_seeds
        .iter()
        .map(|seed| get_account_id_from_seed::<sr25519::Public>(seed))
        .collect();
    let collator_keys: Vec<NimbusId> = initial_collator_seeds
        .iter()
        .map(|seed| get_collator_keys_from_seed(seed))
        .collect();
    let mut default_funded_accounts = pre_funded_accounts();
    default_funded_accounts.extend(collator_accounts.clone());
    default_funded_accounts.sort();
    default_funded_accounts.dedup();

    ChainSpec::from_genesis(
        // Name
        "Development",
        // ID
        "dev",
        ChainType::Development,
        move || {
            testnet_genesis(
                collator_accounts
                    .iter()
                    .zip(collator_keys.iter())
                    .map(|(x, y)| (x.clone(), y.clone()))
                    .collect(),
                default_funded_accounts.clone(),
                para_id.into(),
                get_account_id_from_seed::<sr25519::Public>("Alice"),
            )
        },
        Vec::new(),
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

pub fn local_testnet_config(para_id: ParaId, seeds: Option<Vec<String>>) -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "UNIT".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());
    properties.insert("isEthereum".into(), false.into());
    let protocol_id = Some(format!("container-chain-{}", para_id));

    let initial_collator_seeds = seeds.unwrap_or(vec!["Alice".to_string(), "Bob".to_string()]);
    let collator_accounts: Vec<AccountId> = initial_collator_seeds
        .iter()
        .map(|seed| get_account_id_from_seed::<sr25519::Public>(seed))
        .collect();
    let collator_keys: Vec<NimbusId> = initial_collator_seeds
        .iter()
        .map(|seed| get_collator_keys_from_seed(seed))
        .collect();
    let mut default_funded_accounts = pre_funded_accounts();
    default_funded_accounts.extend(collator_accounts.clone());
    default_funded_accounts.sort();
    default_funded_accounts.dedup();

    ChainSpec::from_genesis(
        // Name
        "Local Testnet",
        // ID
        "local_testnet",
        ChainType::Local,
        move || {
            testnet_genesis(
                collator_accounts
                    .iter()
                    .zip(collator_keys.iter())
                    .map(|(x, y)| (x.clone(), y.clone()))
                    .collect(),
                default_funded_accounts.clone(),
                para_id.into(),
                get_account_id_from_seed::<sr25519::Public>("Alice"),
            )
        },
        // Bootnodes
        Vec::new(),
        // Telemetry
        None,
        // Protocol ID
        protocol_id.as_deref(),
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
) -> container_chain_template_simple_runtime::GenesisConfig {
    container_chain_template_simple_runtime::GenesisConfig {
        system: container_chain_template_simple_runtime::SystemConfig {
            code: container_chain_template_simple_runtime::WASM_BINARY
                .expect("WASM binary was not build, please build it!")
                .to_vec(),
        },
        balances: container_chain_template_simple_runtime::BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1 << 60))
                .collect(),
        },
        parachain_info: container_chain_template_simple_runtime::ParachainInfoConfig {
            parachain_id: id,
        },
        session: container_chain_template_simple_runtime::SessionConfig {
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
        // no need to pass anything to aura, in fact it will panic if we do. Session will take care
        // of this.
        aura: Default::default(),
        aura_ext: Default::default(),
        parachain_system: Default::default(),
        sudo: container_chain_template_simple_runtime::SudoConfig {
            key: Some(root_key),
        },
        authorities_noting: container_chain_template_simple_runtime::AuthoritiesNotingConfig {
            orchestrator_para_id: ORCHESTRATOR,
        },
    }
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
