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
use sc_network::config::MultiaddrWithPeerId;

use {
    cumulus_primitives_core::ParaId,
    dancebox_runtime::{AccountId, RegistrarConfig, Signature, SudoConfig, EXISTENTIAL_DEPOSIT},
    nimbus_primitives::NimbusId,
    pallet_configuration::HostConfiguration,
    sc_chain_spec::{ChainSpecExtension, ChainSpecGroup},
    sc_service::ChainType,
    serde::{Deserialize, Serialize},
    sp_core::{sr25519, Pair, Public},
    sp_runtime::traits::{IdentifyAccount, Verify},
    std::collections::BTreeMap,
    tp_container_chain_genesis_data::{
        json::container_chain_genesis_data_from_path, ContainerChainGenesisData,
    },
};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<dancebox_runtime::GenesisConfig, Extensions>;

/// Specialized `ChainSpec` for container chains that only allows raw genesis format.
pub type RawChainSpec = sc_service::GenericChainSpec<RawGenesisConfig, Extensions>;

/// Helper type that implements the traits needed to be used as a "GenesisConfig",
/// but whose implementation panics because we only expect it to be used with raw ChainSpecs,
/// so it will never be serialized or deserialized.
/// This is because container chains must use raw chain spec files where the "genesis"
/// field only has one field: "raw".
pub struct RawGenesisConfig {
    pub storage_raw: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl Serialize for RawGenesisConfig {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        panic!("RawGenesisConfigDummy should never be serialized")
    }
}

impl<'de> Deserialize<'de> for RawGenesisConfig {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        panic!("Attempted to read a non-raw ContainerChain ChainSpec.\nHelp: add `--raw` flag to `build-spec` command to generate a raw chain spec")
    }
}

impl sp_runtime::BuildStorage for RawGenesisConfig {
    fn assimilate_storage(&self, storage: &mut sp_core::storage::Storage) -> Result<(), String> {
        storage
            .top
            .extend(self.storage_raw.iter().map(|(k, v)| (k.clone(), v.clone())));

        Ok(())
    }
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

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
pub fn template_session_keys(keys: NimbusId) -> dancebox_runtime::SessionKeys {
    dancebox_runtime::SessionKeys { aura: keys }
}

/// Helper function to turn a list of names into a list of `(AccountId, AuraId)`
pub fn invulnerables(names: &[&str]) -> Vec<(AccountId, NimbusId)> {
    names
        .iter()
        .map(|name| {
            (
                get_account_id_from_seed::<sr25519::Public>(name),
                get_collator_keys_from_seed(name),
            )
        })
        .collect()
}

/// Helper function to turn a list of names into a list of `AccountId`
pub fn account_ids(names: &[&str]) -> Vec<AccountId> {
    names
        .iter()
        .map(|name| get_account_id_from_seed::<sr25519::Public>(name))
        .collect()
}

pub fn development_config(
    para_id: ParaId,
    container_chains: Vec<String>,
    mock_container_chains: Vec<ParaId>,
    boot_nodes: Vec<String>,
) -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "UNIT".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());
    properties.insert("isEthereum".into(), false.into());
    let boot_nodes: Vec<MultiaddrWithPeerId> = boot_nodes
        .into_iter()
        .map(|x| {
            x.parse::<MultiaddrWithPeerId>()
                .unwrap_or_else(|e| panic!("invalid bootnode address format {:?}: {:?}", x, e))
        })
        .collect();

    ChainSpec::from_genesis(
        // Name
        "Dancebox Development Testnet",
        // ID
        "dancebox_dev",
        ChainType::Development,
        move || {
            testnet_genesis(
                // initial collators.
                invulnerables(&["Alice", "Bob", "Charlie", "Dave"]),
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
                    },
                },
            )
        },
        boot_nodes,
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

pub fn local_dancebox_config(
    para_id: ParaId,
    container_chains: Vec<String>,
    mock_container_chains: Vec<ParaId>,
    boot_nodes: Vec<String>,
) -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "UNIT".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());
    properties.insert("isEthereum".into(), false.into());
    let boot_nodes: Vec<MultiaddrWithPeerId> = boot_nodes
        .into_iter()
        .map(|x| {
            x.parse::<MultiaddrWithPeerId>()
                .unwrap_or_else(|e| panic!("invalid bootnode address format {:?}: {:?}", x, e))
        })
        .collect();

    ChainSpec::from_genesis(
        // Name
        "Dancebox Local Testnet",
        // ID
        "dancebox_local",
        ChainType::Local,
        move || {
            testnet_genesis(
                // initial collators.
                invulnerables(&["Alice", "Bob", "Charlie", "Dave"]),
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
                    },
                },
            )
        },
        // Bootnodes
        boot_nodes,
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
    configuration: pallet_configuration::GenesisConfig,
) -> dancebox_runtime::GenesisConfig {
    dancebox_runtime::GenesisConfig {
        system: dancebox_runtime::SystemConfig {
            code: dancebox_runtime::WASM_BINARY
                .expect("WASM binary was not build, please build it!")
                .to_vec(),
        },
        balances: dancebox_runtime::BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1 << 60))
                .collect(),
        },
        parachain_info: dancebox_runtime::ParachainInfoConfig { parachain_id: id },
        collator_selection: dancebox_runtime::CollatorSelectionConfig {
            invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
            candidacy_bond: EXISTENTIAL_DEPOSIT * 16,
            ..Default::default()
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
        // no need to pass anything to aura, in fact it will panic if we do. Session will take care
        // of this.
        aura: Default::default(),
        aura_ext: Default::default(),
        parachain_system: Default::default(),
        configuration,
        registrar: RegistrarConfig {
            para_ids: container_chains
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
                .collect(),
        },
        sudo: SudoConfig {
            key: Some(root_key),
        },
    }
}

fn mock_container_chain_genesis_data(para_id: ParaId) -> ContainerChainGenesisData {
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
    /// Returns `true` if this is a configuration for the `Dancebox` network.
    fn is_dancebox(&self) -> bool;
    /// Returns `true` if this is a configuration for a dev network.
    fn is_dev(&self) -> bool;
}

impl IdentifyVariant for Box<dyn sc_service::ChainSpec> {
    fn is_dancebox(&self) -> bool {
        self.id().starts_with("dancebox")
    }

    fn is_dev(&self) -> bool {
        self.chain_type() == sc_chain_spec::ChainType::Development
    }
}
