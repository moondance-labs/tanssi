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
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>

//! Polkadot chain configurations.

use {
    beefy_primitives::ecdsa_crypto::AuthorityId as BeefyId,
    cumulus_primitives_core::ParaId,
    dp_container_chain_genesis_data::{
        json::container_chain_genesis_data_from_path, ContainerChainGenesisData,
    },
    grandpa::AuthorityId as GrandpaId,
    polkadot_primitives::{AccountId, AccountPublic, AssignmentId, ValidatorId},
    sp_authority_discovery::AuthorityId as AuthorityDiscoveryId,
    sp_consensus_babe::AuthorityId as BabeId,
    starlight_runtime::genesis_config_presets::starlight_development_config_genesis,
    starlight_runtime::genesis_config_presets::starlight_local_testnet_genesis,
};

#[cfg(any(feature = "starlight-native"))]
use sc_chain_spec::ChainType;
#[cfg(feature = "starlight-native")]
use starlight_runtime as starlight;
#[cfg(any(feature = "starlight-native"))]
use telemetry::TelemetryEndpoints;
use {
    sc_chain_spec::ChainSpecExtension,
    serde::{Deserialize, Serialize},
    sp_core::{sr25519, Pair, Public},
    sp_runtime::traits::IdentifyAccount,
};

#[cfg(feature = "starlight-native")]
const STARLIGHT_STAGING_TELEMETRY_URL: &str = "wss://telemetry.tanssi.network/submit/";
#[cfg(any(feature = "starlight-native"))]
const DEFAULT_PROTOCOL_ID: &str = "star";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    /// Block numbers with known hashes.
    pub fork_blocks: sc_client_api::ForkBlocks<polkadot_primitives::Block>,
    /// Known bad block hashes.
    pub bad_blocks: sc_client_api::BadBlocks<polkadot_primitives::Block>,
    /// The light sync state.
    ///
    /// This value will be set by the `sync-state rpc` implementation.
    pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

// Generic chain spec, in case when we don't have the native runtime.
pub type GenericChainSpec = service::GenericChainSpec<(), Extensions>;

/// The `ChainSpec` parameterized for the starlight runtime.
#[cfg(feature = "starlight-native")]
pub type StarlightChainSpec = service::GenericChainSpec<(), Extensions>;

/// The `ChainSpec` parameterized for the starlight runtime.
// Dummy chain spec, but that is fine when we don't have the native runtime.
#[cfg(not(feature = "starlight-native"))]
pub type StarlightChainSpec = GenericChainSpec;

pub fn starlight_config() -> Result<StarlightChainSpec, String> {
    StarlightChainSpec::from_json_bytes(&include_bytes!("../chain-specs/rococo.json")[..])
    // FIXME: Update this to Starlight.json once it is available
}

/// Starlight staging testnet config.
#[cfg(feature = "starlight-native")]
pub fn starlight_staging_testnet_config() -> Result<StarlightChainSpec, String> {
    Ok(StarlightChainSpec::builder(
        starlight::WASM_BINARY.ok_or("Starlight development wasm not available")?,
        Default::default(),
    )
    .with_name("Starlight Staging Testnet")
    .with_id("starlight_staging_testnet")
    .with_chain_type(ChainType::Live)
    .with_genesis_config_preset_name("staging_testnet")
    .with_telemetry_endpoints(
        TelemetryEndpoints::new(vec![(STARLIGHT_STAGING_TELEMETRY_URL.to_string(), 0)])
            .expect("Starlight Staging telemetry url is valid; qed"),
    )
    .with_protocol_id(DEFAULT_PROTOCOL_ID)
    .build())
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed(
    seed: &str,
) -> (
    AccountId,
    AccountId,
    BabeId,
    GrandpaId,
    ValidatorId,
    AssignmentId,
    AuthorityDiscoveryId,
    BeefyId,
) {
    let keys = get_authority_keys_from_seed_no_beefy(seed);
    (
        keys.0,
        keys.1,
        keys.2,
        keys.3,
        keys.4,
        keys.5,
        keys.6,
        get_from_seed::<BeefyId>(seed),
    )
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed_no_beefy(
    seed: &str,
) -> (
    AccountId,
    AccountId,
    BabeId,
    GrandpaId,
    ValidatorId,
    AssignmentId,
    AuthorityDiscoveryId,
) {
    (
        get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
        get_account_id_from_seed::<sr25519::Public>(seed),
        get_from_seed::<BabeId>(seed),
        get_from_seed::<GrandpaId>(seed),
        get_from_seed::<ValidatorId>(seed),
        get_from_seed::<AssignmentId>(seed),
        get_from_seed::<AuthorityDiscoveryId>(seed),
    )
}

/// Starlight development config (single validator Alice)
#[cfg(feature = "starlight-native")]
pub fn starlight_development_config(
    container_chains: Vec<String>,
    mock_container_chains: Vec<ParaId>,
    invulnerables: Vec<String>,
) -> Result<StarlightChainSpec, String> {
    let container_chains: Vec<_> = container_chains
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

    Ok(StarlightChainSpec::builder(
        starlight::WASM_BINARY.ok_or("Starlight development wasm not available")?,
        Default::default(),
    )
    .with_name("Development")
    .with_id("starlight_dev")
    .with_chain_type(ChainType::Development)
    .with_genesis_config_patch(starlight_development_config_genesis(
        container_chains,
        invulnerables,
    ))
    .with_protocol_id(DEFAULT_PROTOCOL_ID)
    .build())
}

/// Starlight local testnet config (multivalidator Alice + Bob)
#[cfg(feature = "starlight-native")]
pub fn starlight_local_testnet_config(
    container_chains: Vec<String>,
    mock_container_chains: Vec<ParaId>,
    invulnerables: Vec<String>,
) -> Result<StarlightChainSpec, String> {
    let container_chains: Vec<_> = container_chains
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

    Ok(StarlightChainSpec::builder(
        starlight::fast_runtime_binary::WASM_BINARY
            .ok_or("Starlight development wasm not available")?,
        Default::default(),
    )
    .with_name("Starlight Local Testnet")
    .with_id("starlight_local_testnet")
    .with_chain_type(ChainType::Local)
    .with_genesis_config_patch(starlight_local_testnet_genesis(
        container_chains,
        invulnerables,
    ))
    .with_protocol_id(DEFAULT_PROTOCOL_ID)
    .build())
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
