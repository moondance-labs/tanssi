use std::collections::BTreeMap;

use container_chain_template_frontier_runtime::EVMConfig;

use {
    container_chain_template_frontier_runtime::{AccountId, AuraId, EVMChainIdConfig, Signature},
    cumulus_primitives_core::ParaId,
    sc_chain_spec::{ChainSpecExtension, ChainSpecGroup},
    sc_service::ChainType,
    serde::{Deserialize, Serialize},
    sp_core::{Pair, Public, H160, U256},
    sp_runtime::traits::{IdentifyAccount, Verify},
    std::str::FromStr,
};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<
    container_chain_template_frontier_runtime::GenesisConfig,
    Extensions,
>;

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
pub fn template_session_keys(
    keys: AuraId,
) -> container_chain_template_frontier_runtime::SessionKeys {
    container_chain_template_frontier_runtime::SessionKeys { aura: keys }
}

pub fn development_config(para_id: ParaId, seeds: Option<Vec<String>>) -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "UNIT".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());

    let initial_collator_seeds = seeds.unwrap_or(vec!["Alice".to_string(), "Bob".to_string()]);
    let collator_accounts: Vec<AccountId> = initial_collator_seeds
        .iter()
        .map(|seed| get_account_id_from_seed::<sp_core::ecdsa::Public>(seed))
        .collect();
    let collator_keys: Vec<AuraId> = initial_collator_seeds
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

pub fn local_testnet_config(para_id: ParaId, seeds: Option<Vec<String>>) -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "UNIT".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());

    let initial_collator_seeds = seeds.unwrap_or(vec!["Alice".to_string(), "Bob".to_string()]);
    let collator_accounts: Vec<AccountId> = initial_collator_seeds
        .iter()
        .map(|seed| get_account_id_from_seed::<sp_core::ecdsa::Public>(seed))
        .collect();
    let collator_keys: Vec<AuraId> = initial_collator_seeds
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
            )
        },
        // Bootnodes
        Vec::new(),
        // Telemetry
        None,
        // Protocol ID
        Some("template-local"),
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
) -> container_chain_template_frontier_runtime::GenesisConfig {
    let chain_id: u32 = id.into();
    let chain_id = chain_id as u64;
    container_chain_template_frontier_runtime::GenesisConfig {
        system: container_chain_template_frontier_runtime::SystemConfig {
            code: container_chain_template_frontier_runtime::WASM_BINARY
                .expect("WASM binary was not build, please build it!")
                .to_vec(),
        },
        balances: container_chain_template_frontier_runtime::BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1 << 60))
                .collect(),
        },
        parachain_info: container_chain_template_frontier_runtime::ParachainInfoConfig {
            parachain_id: id,
        },
        session: container_chain_template_frontier_runtime::SessionConfig {
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
        evm_chain_id: EVMChainIdConfig { chain_id },
        evm: EVMConfig {
            accounts: {
                let mut map = BTreeMap::new();
                map.insert(
                    // H160 address of Alice dev account
                    // Derived from SS58 (42 prefix) address
                    // SS58: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
                    // hex: 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
                    // Using the full hex key, truncating to the first 20 bytes (the first 40 hex chars)
                    H160::from_str("d43593c715fdd31c61141abd04a99fd6822c8558")
                        .expect("internal H160 is valid; qed"),
                    fp_evm::GenesisAccount {
                        balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
                            .expect("internal U256 is valid; qed"),
                        code: Default::default(),
                        nonce: Default::default(),
                        storage: Default::default(),
                    },
                );
                map.insert(
                    // H160 address of CI test runner account
                    H160::from_str("6be02d1d3665660d22ff9624b7be0551ee1ac91b")
                        .expect("internal H160 is valid; qed"),
                    fp_evm::GenesisAccount {
                        balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
                            .expect("internal U256 is valid; qed"),
                        code: Default::default(),
                        nonce: Default::default(),
                        storage: Default::default(),
                    },
                );
                map.insert(
                    // H160 address for benchmark usage
                    H160::from_str("1000000000000000000000000000000000000001")
                        .expect("internal H160 is valid; qed"),
                    fp_evm::GenesisAccount {
                        nonce: U256::from(1),
                        balance: U256::from(1_000_000_000_000_000_000_000_000u128),
                        storage: Default::default(),
                        code: vec![0x00],
                    },
                );
                map
            },
        },
        ethereum: Default::default(),
        dynamic_fee: Default::default(),
        base_fee: Default::default(),
        transaction_payment: Default::default(),
    }
}

/// Get pre-funded accounts
pub fn pre_funded_accounts() -> Vec<AccountId> {
    vec![
        get_account_id_from_seed::<sp_core::ecdsa::Public>("Alice"),
        get_account_id_from_seed::<sp_core::ecdsa::Public>("Bob"),
        get_account_id_from_seed::<sp_core::ecdsa::Public>("Charlie"),
        get_account_id_from_seed::<sp_core::ecdsa::Public>("Dave"),
        get_account_id_from_seed::<sp_core::ecdsa::Public>("Eve"),
        get_account_id_from_seed::<sp_core::ecdsa::Public>("Ferdie"),
        get_account_id_from_seed::<sp_core::ecdsa::Public>("Alice//stash"),
        get_account_id_from_seed::<sp_core::ecdsa::Public>("Bob//stash"),
        get_account_id_from_seed::<sp_core::ecdsa::Public>("Charlie//stash"),
        get_account_id_from_seed::<sp_core::ecdsa::Public>("Dave//stash"),
        get_account_id_from_seed::<sp_core::ecdsa::Public>("Eve//stash"),
        get_account_id_from_seed::<sp_core::ecdsa::Public>("Ferdie//stash"),
    ]
}
