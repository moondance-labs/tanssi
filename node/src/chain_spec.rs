use tp_container_chain_genesis_data::json::container_chain_genesis_data_from_path;
use tp_container_chain_genesis_data::ContainerChainGenesisData;

use {
    cumulus_primitives_core::ParaId,
    sc_chain_spec::{ChainSpecExtension, ChainSpecGroup},
    sc_service::ChainType,
    serde::{Deserialize, Serialize},
    sp_core::{sr25519, Pair, Public},
    sp_runtime::traits::{IdentifyAccount, Verify},
    std::collections::BTreeMap,
    test_runtime::{
        AccountId, AuraId, RegistrarConfig, Signature, SudoConfig, EXISTENTIAL_DEPOSIT,
    },
};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<test_runtime::GenesisConfig, Extensions>;

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
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
    get_from_seed::<AuraId>(seed)
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
pub fn template_session_keys(keys: AuraId) -> test_runtime::SessionKeys {
    test_runtime::SessionKeys { aura: keys.clone() }
}

/// Helper function to turn a list of names into a list of `(AccountId, AuraId)`
pub fn invulnerables(names: &[&str]) -> Vec<(AccountId, AuraId)> {
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
) -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "UNIT".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());

    ChainSpec::from_genesis(
        // Name
        "Development",
        // ID
        "dev",
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
            )
        },
        Vec::new(),
        None,
        None,
        None,
        None,
        Extensions {
            relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
            para_id: para_id.into(),
        },
    )
}

pub fn local_testnet_config(
    para_id: ParaId,
    container_chains: Vec<String>,
    mock_container_chains: Vec<ParaId>,
) -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "UNIT".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());

    ChainSpec::from_genesis(
        // Name
        "Local Testnet",
        // ID
        "local_testnet",
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
            )
        },
        // Bootnodes
        Vec::new(),
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
    invulnerables: Vec<(AccountId, AuraId)>,
    endowed_accounts: Vec<AccountId>,
    id: ParaId,
    root_key: AccountId,
    container_chains: &[String],
    mock_container_chains: &[ParaId],
) -> test_runtime::GenesisConfig {
    test_runtime::GenesisConfig {
        system: test_runtime::SystemConfig {
            code: test_runtime::WASM_BINARY
                .expect("WASM binary was not build, please build it!")
                .to_vec(),
        },
        balances: test_runtime::BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1 << 60))
                .collect(),
        },
        parachain_info: test_runtime::ParachainInfoConfig { parachain_id: id },
        collator_selection: test_runtime::CollatorSelectionConfig {
            invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
            candidacy_bond: EXISTENTIAL_DEPOSIT * 16,
            ..Default::default()
        },
        session: test_runtime::SessionConfig {
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
        configuration: Default::default(),
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
                        .map(|x| (*x, mock_container_chain_genesis_data(*x))),
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
