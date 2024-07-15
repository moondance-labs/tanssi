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
    sc_chain_spec::{ChainSpecExtension, ChainSpecGroup},
    serde::{Deserialize, Serialize},
    std::collections::BTreeMap,
};

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
