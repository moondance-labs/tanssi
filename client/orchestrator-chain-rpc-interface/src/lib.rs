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

mod ws_client;

use {
    async_trait::async_trait,
    core::pin::Pin,
    dc_orchestrator_chain_interface::{
        BlockNumber, ContainerChainGenesisData, DataPreserverAssignment, DataPreserverProfileId,
        OrchestratorChainError, OrchestratorChainInterface, OrchestratorChainResult, PHash,
        PHeader,
    },
    dp_core::ParaId,
    futures::{Stream, StreamExt},
    jsonrpsee::{core::params::ArrayParams, rpc_params},
    sc_client_api::{StorageData, StorageProof},
    sc_rpc_api::state::ReadProof,
    sc_service::TaskManager,
    serde::de::DeserializeOwned,
    sp_core::{Decode, Encode},
    sp_state_machine::StorageValue,
    sp_storage::StorageKey,
    tokio::sync::{mpsc, oneshot},
    url::Url,
    ws_client::{JsonRpcRequest, WsClientRequest},
};

const LOG_TARGET: &str = "orchestrator-rpc-client";
const NOTIFICATION_CHANNEL_SIZE_LIMIT: usize = 20;

/// Format url and force addition of a port
fn url_to_string_with_port(url: Url) -> Option<String> {
    // This is already validated on CLI side, just defensive here
    if (url.scheme() != "ws" && url.scheme() != "wss") || url.host_str().is_none() {
        tracing::warn!(target: LOG_TARGET, ?url, "Non-WebSocket URL or missing host.");
        return None;
    }

    // Either we have a user-supplied port or use the default for 'ws' or 'wss' here
    Some(format!(
        "{}://{}:{}{}{}",
        url.scheme(),
        url.host_str()?,
        url.port_or_known_default()?,
        url.path(),
        url.query()
            .map(|query| format!("?{}", query))
            .unwrap_or_default()
    ))
}

pub async fn create_client_and_start_worker(
    urls: Vec<Url>,
    task_manager: &mut TaskManager,
    overseer_handle: Option<polkadot_overseer::Handle>,
) -> OrchestratorChainResult<OrchestratorChainRpcClient> {
    let urls: Vec<_> = urls
        .into_iter()
        .filter_map(url_to_string_with_port)
        .collect();
    let (worker, request_sender) = ws_client::ReconnectingWsClientWorker::new(urls)
        .await
        .map_err(|_| {
            OrchestratorChainError::GenericError(
                "Failed to connect to all provided Orchestrator chain RPC endpoints".to_string(),
            )
        })?;

    task_manager
        .spawn_essential_handle()
        .spawn("orchestrator-rpc-worker", None, worker.run());

    let client = OrchestratorChainRpcClient {
        request_sender,
        overseer_handle,
    };

    Ok(client)
}

#[derive(Clone)]
pub struct OrchestratorChainRpcClient {
    request_sender: mpsc::Sender<WsClientRequest>,
    overseer_handle: Option<polkadot_overseer::Handle>,
}

impl OrchestratorChainRpcClient {
    /// Call a call to `state_call` rpc method.
    pub async fn call_remote_runtime_function<R: Decode>(
        &self,
        method_name: &str,
        hash: PHash,
        payload: Option<impl Encode>,
    ) -> OrchestratorChainResult<R> {
        let payload_bytes =
            payload.map_or(sp_core::Bytes(Vec::new()), |v| sp_core::Bytes(v.encode()));
        let params = rpc_params! {
            method_name,
            payload_bytes,
            hash
        };
        let res = self
            .request_tracing::<sp_core::Bytes, _>("state_call", params, |err| {
                tracing::debug!(
                    target: LOG_TARGET,
                    %method_name,
                    %hash,
                    error = %err,
                    "Error during call to 'state_call'.",
                );
            })
            .await?;
        Decode::decode(&mut &*res.0).map_err(Into::into)
    }

    async fn request<'a, R>(
        &self,
        method: &'a str,
        params: ArrayParams,
    ) -> OrchestratorChainResult<R>
    where
        R: DeserializeOwned + std::fmt::Debug,
    {
        self.request_tracing(
            method,
            params,
            |e| tracing::trace!(target:LOG_TARGET, error = %e, %method, "Unable to complete RPC request"),
        ).await
    }

    fn send_register_message(
        &self,
        message_builder: impl FnOnce(mpsc::Sender<dp_core::Header>) -> WsClientRequest,
    ) -> OrchestratorChainResult<mpsc::Receiver<dp_core::Header>> {
        let (tx, rx) = mpsc::channel(NOTIFICATION_CHANNEL_SIZE_LIMIT);
        self.request_sender
            .try_send(message_builder(tx))
            .map_err(|e| OrchestratorChainError::WorkerCommunicationError(e.to_string()))?;
        Ok(rx)
    }

    /// Send a request to the RPC worker and awaits for a response. The worker is responsible
    /// for retrying requests if connection dies.
    async fn request_tracing<'a, R, OR>(
        &self,
        method: &'a str,
        params: ArrayParams,
        trace_error: OR,
    ) -> OrchestratorChainResult<R>
    where
        R: DeserializeOwned + std::fmt::Debug,
        OR: Fn(&OrchestratorChainError),
    {
        let (response_sender, response_receiver) = oneshot::channel();

        let request = WsClientRequest::JsonRpcRequest(JsonRpcRequest {
            method: method.into(),
            params,
            response_sender,
        });
        self.request_sender.send(request).await.map_err(|err| {
            OrchestratorChainError::WorkerCommunicationError(format!(
                "Unable to send message to RPC worker: {}",
                err
            ))
        })?;

        let response = response_receiver.await.map_err(|err| {
			OrchestratorChainError::WorkerCommunicationError(format!(
				"RPC worker channel closed. This can hint and connectivity issues with the supplied RPC endpoints. Message: {}",
				err
			))
		})??;

        serde_json::from_value(response).map_err(|_| {
            trace_error(&OrchestratorChainError::GenericError(
                "Unable to deserialize value".to_string(),
            ));
            OrchestratorChainError::RpcCallError(
                method.to_string(),
                "failed to decode returned value".to_string(),
            )
        })
    }

    /// Retrieve storage item at `storage_key`
    pub async fn state_get_storage(
        &self,
        storage_key: StorageKey,
        at: Option<PHash>,
    ) -> OrchestratorChainResult<Option<StorageData>> {
        let params = rpc_params![storage_key, at];
        self.request("state_getStorage", params).await
    }

    /// Get read proof for `storage_keys`
    pub async fn state_get_read_proof(
        &self,
        storage_keys: Vec<StorageKey>,
        at: Option<PHash>,
    ) -> OrchestratorChainResult<ReadProof<PHash>> {
        let params = rpc_params![storage_keys, at];
        self.request("state_getReadProof", params).await
    }
}

#[async_trait]
impl OrchestratorChainInterface for OrchestratorChainRpcClient {
    /// Fetch a storage item by key.
    async fn get_storage_by_key(
        &self,
        orchestrator_parent: PHash,
        key: &[u8],
    ) -> OrchestratorChainResult<Option<StorageValue>> {
        let storage_key = StorageKey(key.to_vec());
        self.state_get_storage(storage_key, Some(orchestrator_parent))
            .await
            .map(|storage_data| storage_data.map(|sv| sv.0))
    }

    /// Get a handle to the overseer.
    fn overseer_handle(&self) -> OrchestratorChainResult<polkadot_overseer::Handle> {
        self.overseer_handle
            .clone()
            .ok_or(OrchestratorChainError::GenericError(
                "OrchestratorChainRpcClient doesn't contain an Overseer Handle".to_string(),
            ))
    }

    /// Generate a storage read proof.
    async fn prove_read(
        &self,
        orchestrator_parent: PHash,
        relevant_keys: &[Vec<u8>],
    ) -> OrchestratorChainResult<StorageProof> {
        let mut cloned = Vec::new();
        cloned.extend_from_slice(relevant_keys);
        let storage_keys: Vec<StorageKey> = cloned.into_iter().map(StorageKey).collect();

        self.state_get_read_proof(storage_keys, Some(orchestrator_parent))
            .await
            .map(|read_proof| {
                StorageProof::new(read_proof.proof.into_iter().map(|bytes| bytes.to_vec()))
            })
    }

    /// Get a stream of import block notifications.
    async fn import_notification_stream(
        &self,
    ) -> OrchestratorChainResult<Pin<Box<dyn Stream<Item = PHeader> + Send>>> {
        let rx = self.send_register_message(WsClientRequest::RegisterImportListener)?;
        let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
        Ok(stream.boxed())
    }

    /// Get a stream of new best block notifications.
    async fn new_best_notification_stream(
        &self,
    ) -> OrchestratorChainResult<Pin<Box<dyn Stream<Item = PHeader> + Send>>> {
        let rx = self.send_register_message(WsClientRequest::RegisterBestHeadListener)?;
        let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
        Ok(stream.boxed())
    }

    /// Get a stream of finality notifications.
    async fn finality_notification_stream(
        &self,
    ) -> OrchestratorChainResult<Pin<Box<dyn Stream<Item = PHeader> + Send>>> {
        let rx = self.send_register_message(WsClientRequest::RegisterFinalizationListener)?;
        let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
        Ok(stream.boxed())
    }

    async fn genesis_data(
        &self,
        orchestrator_parent: PHash,
        para_id: ParaId,
    ) -> OrchestratorChainResult<Option<ContainerChainGenesisData>> {
        self.call_remote_runtime_function("genesis_data", orchestrator_parent, Some(para_id))
            .await
    }

    async fn boot_nodes(
        &self,
        orchestrator_parent: PHash,
        para_id: ParaId,
    ) -> OrchestratorChainResult<Vec<Vec<u8>>> {
        self.call_remote_runtime_function("boot_nodes", orchestrator_parent, Some(para_id))
            .await
    }

    async fn latest_block_number(
        &self,
        orchestrator_parent: PHash,
        para_id: ParaId,
    ) -> OrchestratorChainResult<Option<BlockNumber>> {
        self.call_remote_runtime_function("latest_block_number", orchestrator_parent, Some(para_id))
            .await
    }

    async fn best_block_hash(&self) -> OrchestratorChainResult<PHash> {
        self.request("chain_getHead", rpc_params![]).await
    }

    async fn finalized_block_hash(&self) -> OrchestratorChainResult<PHash> {
        self.request("chain_getFinalizedHead", rpc_params![]).await
    }

    async fn get_active_assignment(
        &self,
        orchestrator_parent: PHash,
        profile_id: DataPreserverProfileId,
    ) -> OrchestratorChainResult<DataPreserverAssignment<ParaId>> {
        self.call_remote_runtime_function(
            "DataPreserversApi_get_active_assignment",
            orchestrator_parent,
            Some(profile_id),
        )
        .await
    }
}
