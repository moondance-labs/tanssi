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

//! Code used to run a solochain orchestrator node.

/// Provides an implementation of the [`OrchestratorChainInterface`] from a `RelayChainInterface`,
/// which makes sense in case the orchestrator chain is also the relay chain.
use {
    cumulus_primitives_core::ParaId,
    cumulus_relay_chain_interface::call_runtime_api,
    dc_orchestrator_chain_interface::{
        BlockNumber, ContainerChainGenesisData, DataPreserverAssignment, DataPreserverProfileId,
        OrchestratorChainError, OrchestratorChainResult, PHash, PHeader,
    },
    futures::Stream,
    nimbus_primitives::NimbusId,
    polkadot_service::Handle,
    sp_api::StorageProof,
    sp_state_machine::StorageValue,
    std::{pin::Pin, sync::Arc},
    tc_consensus::{OrchestratorChainInterface, RelayChainInterface},
};

/// Builder for a concrete relay chain interface, created from a full node. Builds
/// a [`RelayAsOrchestratorChainInterface`] to access relay chain data necessary for parachain operation.
///
/// The builder takes a [`polkadot_client::Client`]
/// that wraps a concrete instance. By using [`polkadot_client::ExecuteWithClient`]
/// the builder gets access to this concrete instance and instantiates a [`RelayAsOrchestratorChainInterface`] with it.
pub struct RelayAsOrchestratorChainInterfaceBuilder {
    pub overseer_handle: Handle,
    pub relay_chain_interface: Arc<dyn RelayChainInterface>,
}

impl RelayAsOrchestratorChainInterfaceBuilder {
    pub fn build(self) -> Arc<dyn OrchestratorChainInterface> {
        Arc::new(RelayAsOrchestratorChainInterface::new(
            self.overseer_handle,
            self.relay_chain_interface,
        ))
    }
}

pub struct RelayAsOrchestratorChainInterface {
    pub overseer_handle: Handle,
    pub relay_chain_interface: Arc<dyn RelayChainInterface>,
}

impl RelayAsOrchestratorChainInterface {
    /// Create a new instance of [`RelayAsOrchestratorChainInterface`]
    pub fn new(
        overseer_handle: Handle,
        relay_chain_interface: Arc<dyn RelayChainInterface>,
    ) -> Self {
        Self {
            overseer_handle,
            relay_chain_interface,
        }
    }
}

#[async_trait::async_trait]
impl OrchestratorChainInterface for RelayAsOrchestratorChainInterface {
    async fn get_storage_by_key(
        &self,
        relay_parent: PHash,
        key: &[u8],
    ) -> OrchestratorChainResult<Option<StorageValue>> {
        self.relay_chain_interface
            .get_storage_by_key(relay_parent, key)
            .await
            .map_err(|e| OrchestratorChainError::Application(Box::new(e)))
    }

    async fn prove_read(
        &self,
        relay_parent: PHash,
        relevant_keys: &Vec<Vec<u8>>,
    ) -> OrchestratorChainResult<StorageProof> {
        self.relay_chain_interface
            .prove_read(relay_parent, relevant_keys)
            .await
            .map_err(|e| OrchestratorChainError::Application(Box::new(e)))
    }

    fn overseer_handle(&self) -> OrchestratorChainResult<Handle> {
        Ok(self.overseer_handle.clone())
    }

    /// Get a stream of import block notifications.
    async fn import_notification_stream(
        &self,
    ) -> OrchestratorChainResult<Pin<Box<dyn Stream<Item = PHeader> + Send>>> {
        self.relay_chain_interface
            .import_notification_stream()
            .await
            .map_err(|e| OrchestratorChainError::Application(Box::new(e)))
    }

    /// Get a stream of new best block notifications.
    async fn new_best_notification_stream(
        &self,
    ) -> OrchestratorChainResult<Pin<Box<dyn Stream<Item = PHeader> + Send>>> {
        self.relay_chain_interface
            .new_best_notification_stream()
            .await
            .map_err(|e| OrchestratorChainError::Application(Box::new(e)))
    }

    /// Get a stream of finality notifications.
    async fn finality_notification_stream(
        &self,
    ) -> OrchestratorChainResult<Pin<Box<dyn Stream<Item = PHeader> + Send>>> {
        self.relay_chain_interface
            .finality_notification_stream()
            .await
            .map_err(|e| OrchestratorChainError::Application(Box::new(e)))
    }

    async fn genesis_data(
        &self,
        relay_parent: PHash,
        para_id: ParaId,
    ) -> OrchestratorChainResult<Option<ContainerChainGenesisData>> {
        let res: Option<ContainerChainGenesisData> = call_runtime_api(
            &self.relay_chain_interface,
            "RegistrarApi_genesis_data",
            relay_parent,
            &para_id,
        )
        .await
        .map_err(|e| OrchestratorChainError::Application(Box::new(e)))?;

        Ok(res)
    }

    async fn boot_nodes(
        &self,
        relay_parent: PHash,
        para_id: ParaId,
    ) -> OrchestratorChainResult<Vec<Vec<u8>>> {
        let res: Vec<Vec<u8>> = call_runtime_api(
            &self.relay_chain_interface,
            "RegistrarApi_boot_nodes",
            relay_parent,
            &para_id,
        )
        .await
        .map_err(|e| OrchestratorChainError::Application(Box::new(e)))?;

        Ok(res)
    }

    async fn latest_block_number(
        &self,
        relay_parent: PHash,
        para_id: ParaId,
    ) -> OrchestratorChainResult<Option<BlockNumber>> {
        let res: Option<BlockNumber> = call_runtime_api(
            &self.relay_chain_interface,
            "AuthorNotingApi_latest_block_number",
            relay_parent,
            &para_id,
        )
        .await
        .map_err(|e| OrchestratorChainError::Application(Box::new(e)))?;

        Ok(res)
    }

    async fn best_block_hash(&self) -> OrchestratorChainResult<PHash> {
        self.relay_chain_interface
            .best_block_hash()
            .await
            .map_err(|e| OrchestratorChainError::Application(Box::new(e)))
    }

    async fn finalized_block_hash(&self) -> OrchestratorChainResult<PHash> {
        self.relay_chain_interface
            .finalized_block_hash()
            .await
            .map_err(|e| OrchestratorChainError::Application(Box::new(e)))
    }

    async fn data_preserver_active_assignment(
        &self,
        orchestrator_parent: PHash,
        profile_id: DataPreserverProfileId,
    ) -> OrchestratorChainResult<DataPreserverAssignment<ParaId>> {
        call_runtime_api(
            &self.relay_chain_interface,
            "DataPreserversApi_get_active_assignment",
            orchestrator_parent,
            Some(profile_id),
        )
        .await
        .map_err(|e| OrchestratorChainError::Application(Box::new(e)))
    }

    async fn check_para_id_assignment(
        &self,
        relay_parent: PHash,
        authority: NimbusId,
    ) -> OrchestratorChainResult<Option<ParaId>> {
        let res: Option<ParaId> = call_runtime_api(
            &self.relay_chain_interface,
            "TanssiAuthorityAssignmentApi_check_para_id_assignment",
            relay_parent,
            &authority,
        )
        .await
        .map_err(|e| OrchestratorChainError::Application(Box::new(e)))?;

        Ok(res)
    }

    async fn check_para_id_assignment_next_session(
        &self,
        relay_parent: PHash,
        authority: NimbusId,
    ) -> OrchestratorChainResult<Option<ParaId>> {
        let res: Option<ParaId> = call_runtime_api(
            &self.relay_chain_interface,
            "TanssiAuthorityAssignmentApi_check_para_id_assignment_next_session",
            relay_parent,
            &authority,
        )
        .await
        .map_err(|e| OrchestratorChainError::Application(Box::new(e)))?;

        Ok(res)
    }
}
