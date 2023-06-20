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
        AccountId, EVMChainIdConfig, EVMConfig, Signature,
    },
    cumulus_primitives_core::ParaId,
    hex_literal::hex,
    nimbus_primitives::NimbusId,
    sc_chain_spec::{ChainSpecExtension, ChainSpecGroup},
    sc_service::ChainType,
    serde::{Deserialize, Serialize},
    sp_core::{ecdsa, Pair, Public, H160, U256},
    sp_runtime::traits::{IdentifyAccount, Verify},
    std::{collections::BTreeMap, str::FromStr},
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
) -> container_chain_template_frontier_runtime::SessionKeys {
    container_chain_template_frontier_runtime::SessionKeys { aura: keys }
}

pub fn development_config(para_id: ParaId, seeds: Option<Vec<String>>) -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "UNIT".into());
    properties.insert("tokenDecimals".into(), 18.into());
    properties.insert("ss58Format".into(), 42.into());
    properties.insert("isEthereum".into(), true.into());

    let initial_collator_seeds = seeds.unwrap_or(vec!["Alice".to_string(), "Bob".to_string()]);
    let collator_accounts: Vec<AccountId> = initial_collator_seeds
        .iter()
        .map(|seed| get_account_id_from_seed::<ecdsa::Public>(seed))
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
                collator_accounts[0].clone(),
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
    properties.insert("tokenDecimals".into(), 18.into());
    properties.insert("ss58Format".into(), 42.into());
    properties.insert("isEthereum".into(), true.into());
    let protocol_id = Some(format!("container-chain-{}", para_id));

    let initial_collator_seeds = seeds.unwrap_or(vec!["Alice".to_string(), "Bob".to_string()]);
    let collator_accounts: Vec<AccountId> = initial_collator_seeds
        .iter()
        .map(|seed| get_account_id_from_seed::<ecdsa::Public>(seed))
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
                collator_accounts[0].clone(),
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
) -> container_chain_template_frontier_runtime::GenesisConfig {
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
                .map(|k| (k, 1 << 80))
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
        parachain_system: Default::default(),
        // EVM compatibility
        // We should change this to something different than Moonbeam
        // For now moonwall is very tailored for moonbeam so we need it for tests
        evm_chain_id: EVMChainIdConfig {
            chain_id: 1281u32 as u64,
        },
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
        sudo: container_chain_template_frontier_runtime::SudoConfig {
            key: Some(root_key),
        },
    }
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
