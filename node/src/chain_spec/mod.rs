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
    dancebox_runtime::{AccountId, Signature},
    nimbus_primitives::NimbusId,
    sc_chain_spec::{ChainSpecExtension, ChainSpecGroup},
    serde::{Deserialize, Serialize},
    sp_core::{sr25519, Pair, Public},
    sp_runtime::traits::{IdentifyAccount, Verify},
    std::collections::BTreeMap,
};

pub mod dancebox;
pub mod flashbox;

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

/// Helper function to turn a list of names into a list of `(AccountId, NimbusId)`
pub fn invulnerables_from_seeds<S: AsRef<str>, I: Iterator<Item = S>>(
    names: I,
) -> Vec<(AccountId, NimbusId)> {
    names
        .map(|name| {
            let name = name.as_ref();
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
