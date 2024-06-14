// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Polkadot chain configurations.

use {
    beefy_primitives::ecdsa_crypto::AuthorityId as BeefyId,
    grandpa::AuthorityId as GrandpaId,
    polkadot_primitives::{AccountId, AccountPublic, AssignmentId, ValidatorId},
    sp_authority_discovery::AuthorityId as AuthorityDiscoveryId,
    sp_consensus_babe::AuthorityId as BabeId,
};

#[cfg(feature = "mozart-native")]
use mozart_runtime as mozart;
#[cfg(any(feature = "mozart-native"))]
use sc_chain_spec::ChainType;
#[cfg(any(feature = "mozart-native"))]
use telemetry::TelemetryEndpoints;
use {
    sc_chain_spec::ChainSpecExtension,
    serde::{Deserialize, Serialize},
    sp_core::{sr25519, Pair, Public},
    sp_runtime::traits::IdentifyAccount,
};

#[cfg(feature = "mozart-native")]
const MOZART_STAGING_TELEMETRY_URL: &str = "wss://telemetry.tanssi.network/submit/";
#[cfg(any(feature = "mozart-native"))]
const DEFAULT_PROTOCOL_ID: &str = "moz";

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

/// The `ChainSpec` parameterized for the mozart runtime.
#[cfg(feature = "mozart-native")]
pub type MozartChainSpec = service::GenericChainSpec<(), Extensions>;

/// The `ChainSpec` parameterized for the mozart runtime.
// Dummy chain spec, but that is fine when we don't have the native runtime.
#[cfg(not(feature = "mozart-native"))]
pub type MozartChainSpec = GenericChainSpec;

pub fn mozart_config() -> Result<MozartChainSpec, String> {
    MozartChainSpec::from_json_bytes(&include_bytes!("../chain-specs/rococo.json")[..])
    // FIXME: Update this to Mozart.json once it is available
}

/// Mozart staging testnet config.
#[cfg(feature = "mozart-native")]
pub fn mozart_staging_testnet_config() -> Result<MozartChainSpec, String> {
    Ok(MozartChainSpec::builder(
        mozart::WASM_BINARY.ok_or("Mozart development wasm not available")?,
        Default::default(),
    )
    .with_name("Mozart Staging Testnet")
    .with_id("mozart_staging_testnet")
    .with_chain_type(ChainType::Live)
    .with_genesis_config_preset_name("staging_testnet")
    .with_telemetry_endpoints(
        TelemetryEndpoints::new(vec![(MOZART_STAGING_TELEMETRY_URL.to_string(), 0)])
            .expect("Mozart Staging telemetry url is valid; qed"),
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

/// Mozart development config (single validator Alice)
#[cfg(feature = "mozart-native")]
pub fn mozart_development_config() -> Result<MozartChainSpec, String> {
    Ok(MozartChainSpec::builder(
        mozart::WASM_BINARY.ok_or("Mozart development wasm not available")?,
        Default::default(),
    )
    .with_name("Development")
    .with_id("mozart_dev")
    .with_chain_type(ChainType::Development)
    .with_genesis_config_preset_name("development")
    .with_protocol_id(DEFAULT_PROTOCOL_ID)
    .build())
}

/// Mozart local testnet config (multivalidator Alice + Bob)
#[cfg(feature = "mozart-native")]
pub fn mozart_local_testnet_config() -> Result<MozartChainSpec, String> {
    Ok(MozartChainSpec::builder(
        mozart::fast_runtime_binary::WASM_BINARY.ok_or("Mozart development wasm not available")?,
        Default::default(),
    )
    .with_name("Mozart Local Testnet")
    .with_id("mozart_local_testnet")
    .with_chain_type(ChainType::Local)
    .with_genesis_config_preset_name("local_testnet")
    .with_protocol_id(DEFAULT_PROTOCOL_ID)
    .build())
}
