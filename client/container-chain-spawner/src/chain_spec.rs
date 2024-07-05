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
    cumulus_client_collator::service::CollatorService,
    cumulus_client_consensus_common::{
        ParachainBlockImport as TParachainBlockImport, ParachainBlockImportMarker,
    },
    cumulus_client_consensus_proposer::Proposer,
    cumulus_client_service::{
        prepare_node_config, start_relay_chain_tasks, DARecoveryProfile, ParachainHostFunctions,
        StartRelayChainTasksParams,
    },
    cumulus_primitives_core::{relay_chain::CollatorPair, ParaId},
    cumulus_relay_chain_interface::{OverseerHandle, RelayChainInterface},
    dancebox_runtime::{opaque::Hash, RuntimeApi},
    dc_orchestrator_chain_interface::OrchestratorChainInterface,
    dp_core::Block,
    dp_slot_duration_runtime_api::TanssiSlotDurationApi,
    nimbus_primitives::{NimbusId, NimbusPair},
    node_common::service::{NodeBuilder, NodeBuilderConfig},
    sc_chain_spec::{ChainSpecExtension, ChainSpecGroup},
    sc_consensus::BasicQueue,
    sc_executor::{NativeElseWasmExecutor, WasmExecutor},
    sc_network::NetworkBlock,
    sc_network_sync::SyncingService,
    sc_service::{
        Configuration, ImportQueue, SpawnTaskHandle, TFullBackend, TFullClient, TaskManager,
    },
    sc_telemetry::TelemetryHandle,
    serde::{Deserialize, Serialize},
    sp_api::ProvideRuntimeApi,
    sp_consensus_slots::{Slot, SlotDuration},
    sp_keystore::KeystorePtr,
    std::{collections::BTreeMap, sync::Arc, time::Duration},
    substrate_prometheus_endpoint::Registry,
    tc_consensus::{
        collators::{
            lookahead as lookahead_tanssi_aura, lookahead::Params as LookaheadTanssiAuraParams,
        },
        OrchestratorAuraWorkerAuxData,
    },
    tokio_util::sync::CancellationToken,
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