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

use sp_core::crypto::get_public_from_string_or_panic;
use {
    beefy_primitives::ecdsa_crypto::AuthorityId as BeefyId,
    cumulus_primitives_core::ParaId,
    dancelight_runtime::genesis_config_presets::{
        dancelight_development_config_genesis, dancelight_local_testnet_genesis,
    },
    dp_container_chain_genesis_data::{
        json::container_chain_genesis_data_from_path, ContainerChainGenesisData,
    },
    grandpa::AuthorityId as GrandpaId,
    polkadot_primitives::{AccountId, AssignmentId, ValidatorId},
    sp_authority_discovery::AuthorityId as AuthorityDiscoveryId,
    sp_consensus_babe::AuthorityId as BabeId,
};

#[cfg(feature = "dancelight-native")]
use dancelight_runtime as dancelight;
#[cfg(feature = "dancelight-native")]
use sc_chain_spec::ChainType;
#[cfg(feature = "dancelight-native")]
use telemetry::TelemetryEndpoints;
use {
    sc_chain_spec::ChainSpecExtension,
    serde::{Deserialize, Serialize},
    sp_core::{sr25519, storage::well_known_keys as StorageWellKnownKeys},
};

#[cfg(feature = "dancelight-native")]
const DANCELIGHT_STAGING_TELEMETRY_URL: &str = "wss://telemetry.tanssi.network/submit/";
#[cfg(feature = "dancelight-native")]
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
pub type GenericChainSpec = service::GenericChainSpec<Extensions>;

/// The `ChainSpec` parameterized for the dancelight runtime.
#[cfg(feature = "dancelight-native")]
pub type DancelightChainSpec = service::GenericChainSpec<Extensions>;

/// The `ChainSpec` parameterized for the dancelight runtime.
// Dummy chain spec, but that is fine when we don't have the native runtime.
#[cfg(not(feature = "dancelight-native"))]
pub type DancelightChainSpec = GenericChainSpec;

pub fn dancelight_config() -> Result<DancelightChainSpec, String> {
    DancelightChainSpec::from_json_bytes(&include_bytes!("../chain-specs/rococo.json")[..])
    // FIXME: Update this to Dancelight.json once it is available
}

/// Dancelight staging testnet config.
#[cfg(feature = "dancelight-native")]
pub fn dancelight_staging_testnet_config() -> Result<DancelightChainSpec, String> {
    Ok(DancelightChainSpec::builder(
        dancelight::WASM_BINARY.ok_or("Dancelight development wasm not available")?,
        Default::default(),
    )
    .with_name("Dancelight Staging Testnet")
    .with_id("dancelight_staging_testnet")
    .with_chain_type(ChainType::Live)
    .with_genesis_config_preset_name("staging_testnet")
    .with_telemetry_endpoints(
        TelemetryEndpoints::new(vec![(DANCELIGHT_STAGING_TELEMETRY_URL.to_string(), 0)])
            .expect("Dancelight Staging telemetry url is valid; qed"),
    )
    .with_protocol_id(DEFAULT_PROTOCOL_ID)
    .build())
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
        get_public_from_string_or_panic::<BeefyId>(seed),
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
        get_public_from_string_or_panic::<sr25519::Public>(&format!("{}//stash", seed)).into(),
        get_public_from_string_or_panic::<sr25519::Public>(seed).into(),
        get_public_from_string_or_panic::<BabeId>(seed),
        get_public_from_string_or_panic::<GrandpaId>(seed),
        get_public_from_string_or_panic::<ValidatorId>(seed),
        get_public_from_string_or_panic::<AssignmentId>(seed),
        get_public_from_string_or_panic::<AuthorityDiscoveryId>(seed),
    )
}

/// Dancelight development config (single validator Alice)
#[cfg(feature = "dancelight-native")]
pub fn dancelight_development_config(
    container_chains: Vec<String>,
    mock_container_chains: Vec<ParaId>,
    invulnerables: Vec<String>,
) -> Result<DancelightChainSpec, String> {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "STAR".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());
    properties.insert("isEthereum".into(), false.into());

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

    Ok(DancelightChainSpec::builder(
        dancelight::WASM_BINARY.ok_or("Dancelight development wasm not available")?,
        Default::default(),
    )
    .with_name("Development")
    .with_id("dancelight_dev")
    .with_chain_type(ChainType::Development)
    .with_genesis_config_patch(dancelight_development_config_genesis(
        container_chains,
        invulnerables,
    ))
    .with_protocol_id(DEFAULT_PROTOCOL_ID)
    .with_properties(properties)
    .build())
}

/// Dancelight local testnet config (multivalidator Alice + Bob)
#[cfg(feature = "dancelight-native")]
pub fn dancelight_local_testnet_config(
    container_chains: Vec<String>,
    mock_container_chains: Vec<ParaId>,
    invulnerables: Vec<String>,
) -> Result<DancelightChainSpec, String> {
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

    Ok(DancelightChainSpec::builder(
        dancelight::WASM_BINARY.ok_or("Dancelight development wasm not available")?,
        Default::default(),
    )
    .with_name("Dancelight Local Testnet")
    .with_id("dancelight_local_testnet")
    .with_chain_type(ChainType::Local)
    .with_genesis_config_patch(dancelight_local_testnet_genesis(
        container_chains,
        invulnerables,
    ))
    .with_protocol_id(DEFAULT_PROTOCOL_ID)
    .build())
}

fn mock_container_chain_genesis_data(para_id: ParaId) -> ContainerChainGenesisData {
    ContainerChainGenesisData {
        storage: vec![
            dp_container_chain_genesis_data::ContainerChainGenesisDataItem {
                key: StorageWellKnownKeys::CODE.to_vec(),
                value: dummy_validation_code().0,
            },
        ],
        name: format!("Container Chain {}", para_id).into(),
        id: format!("container-chain-{}", para_id).into(),
        fork_id: None,
        extensions: vec![],
        properties: Default::default(),
    }
}

/// Create meaningless validation code.
pub fn dummy_validation_code() -> cumulus_primitives_core::relay_chain::ValidationCode {
    cumulus_primitives_core::relay_chain::ValidationCode(vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
}
