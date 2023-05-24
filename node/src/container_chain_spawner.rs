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

use crate::cli::ContainerChainCli;
use crate::service::start_node_impl_container;
use crate::service::ParachainClient;
use cumulus_primitives_core::ParaId;
use cumulus_relay_chain_interface::RelayChainInterface;
use futures::FutureExt;
use pallet_registrar_runtime_api::RegistrarApi;
use polkadot_primitives::CollatorPair;
use sc_service::SpawnTaskHandle;
use sp_api::ProvideRuntimeApi;
use sp_keystore::SyncCryptoStorePtr;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use tc_orchestrator_chain_interface::OrchestratorChainInterface;
use tokio::sync::mpsc::UnboundedReceiver;

/// Struct with all the params needed to start a container chain node given the CLI arguments,
/// and creating the ChainSpec from on-chain data from the orchestrator chain.
#[derive(Clone)]
pub struct ContainerChainSpawner {
    // Start container chain params
    pub orchestrator_chain_interface: Arc<dyn OrchestratorChainInterface>,
    pub orchestrator_client: Arc<ParachainClient>,
    pub container_chain_cli: ContainerChainCli,
    pub tokio_handle: tokio::runtime::Handle,
    pub chain_type: sc_chain_spec::ChainType,
    pub relay_chain: String,
    pub relay_chain_interface: Arc<dyn RelayChainInterface>,
    pub collator_key: Option<CollatorPair>,
    pub sync_keystore: SyncCryptoStorePtr,
    pub orchestrator_para_id: ParaId,
    pub validator: bool,
    pub spawn_handle: SpawnTaskHandle,

    // State
    pub spawned_para_ids: Arc<Mutex<HashMap<ParaId, StopContainerChain>>>,

    // Async callback that enables collation on the orchestrator chain
    pub collate_on_tanssi: Arc<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>,
}

/// Stops a container chain when dropped
pub struct StopContainerChain(exit_future::Signal);

/// Messages used to control the `ContainerChainSpawner`. This is needed because one of the fields
/// of `ContainerChainSpawner` is not `Sync`, so we cannot simply pass an
/// `Arc<ContainerChainSpawner>` to other threads.
#[derive(Debug)]
pub enum CcSpawnMsg {
    /// Start a container chain client for this ParaId. If the ParaId is the orchestrator chain id,
    /// start collating there.
    Spawn(ParaId),
    /// Stop the container chain client previously started for this ParaId. If the ParaId is the
    /// orchestrator chain id, ignore this message.
    Stop(ParaId),
}

impl ContainerChainSpawner {
    /// Try to start a new container chain. In case of error, this panics and stops the node.
    fn spawn(&self, container_chain_para_id: ParaId) -> impl Future<Output = ()> {
        let ContainerChainSpawner {
            orchestrator_chain_interface,
            orchestrator_client,
            container_chain_cli,
            tokio_handle,
            chain_type,
            relay_chain,
            relay_chain_interface,
            collator_key,
            sync_keystore,
            orchestrator_para_id,
            validator,
            spawn_handle,
            spawned_para_ids,
            collate_on_tanssi: _,
        } = self.clone();
        let mut container_chain_cli: ContainerChainCli = container_chain_cli.clone();

        // This closure is used to emulate a try block, it enables using the `?` operator inside
        let try_closure = move || async move {
            // Preload genesis data from orchestrator chain storage.
            // The preload must finish before calling create_configuration, so any async operations
            // need to be awaited.

            // TODO: the orchestrator chain node may not be fully synced yet,
            // in that case we will be reading an old state.
            let orchestrator_chain_info = orchestrator_client.chain_info();
            log::info!(
                "Reading container chain genesis data from orchestrator chain at block #{} {}",
                orchestrator_chain_info.best_number,
                orchestrator_chain_info.best_hash,
            );
            let orchestrator_runtime_api = orchestrator_client.runtime_api();

            log::info!(
                "Detected assignment for container chain {}",
                container_chain_para_id
            );

            let genesis_data = orchestrator_runtime_api
                .genesis_data(
                    orchestrator_chain_info.best_hash,
                    container_chain_para_id.into(),
                )
                .expect("error")
                .ok_or_else(|| {
                    format!(
                        "No genesis data registered for container chain id {}",
                        container_chain_para_id
                    )
                })?;

            container_chain_cli
                .preload_chain_spec_from_genesis_data(
                    container_chain_para_id.into(),
                    genesis_data,
                    chain_type.clone(),
                    relay_chain.clone(),
                )
                .map_err(|e| format!("failed to create container chain chain spec from on chain genesis data: {}", e))?;

            log::info!(
                "Loaded chain spec for container chain {}",
                container_chain_para_id
            );

            // Update CLI params
            container_chain_cli.base.para_id = Some(container_chain_para_id.into());

            let mut container_chain_cli_config = sc_cli::SubstrateCli::create_configuration(
                &container_chain_cli,
                &container_chain_cli,
                tokio_handle.clone(),
            )
            .map_err(|err| format!("Container chain argument error: {}", err))?;

            // Change database path to make it depend on container chain para id
            // So instead of the usual "db/full" we have "db/full-container-2000"
            let mut db_path = container_chain_cli_config
                .database
                .path()
                .unwrap()
                .to_owned();
            db_path.set_file_name(format!("full-container-{}", container_chain_para_id));
            container_chain_cli_config.database.set_path(&db_path);

            // Start container chain node
            let (mut container_chain_task_manager, _container_chain_client) =
                start_node_impl_container(
                    container_chain_cli_config,
                    relay_chain_interface.clone(),
                    orchestrator_chain_interface.clone(),
                    collator_key.clone(),
                    sync_keystore.clone(),
                    container_chain_para_id.into(),
                    orchestrator_para_id,
                    validator,
                )
                .await?;

            // Signal that allows to gracefully stop a container chain
            let (signal, on_exit) = exit_future::signal();

            spawned_para_ids
                .lock()
                .expect("poison error")
                .insert(container_chain_para_id.into(), StopContainerChain(signal));

            // Add the container chain task manager as a child task to the parent task manager.
            // We want to stop the node if this task manager stops, but we also want to allow a
            // graceful shutdown using the `on_exit` future.
            let name = "container-chain-task-manager";
            spawn_handle.spawn(name, None, async move {
                let mut container_chain_task_manager_future =
                    container_chain_task_manager.future().fuse();
                let mut on_exit_future = on_exit.fuse();

                futures::select! {
                    res1 = container_chain_task_manager_future => {
                        log::error!("Essential task `{}` failed. Shutting down service.", name);

                        // This should do `essential_failed_tx.close()` but we can't get that from
                        // the parent task manager (it is private), so just panic
                        match res1 {
                            Ok(()) => panic!("{} has stopped unexpectedly", name),
                            Err(e) => panic!("{} failed: {}", name, e),
                        }
                    }
                    _ = on_exit_future => {
                        // Graceful shutdown
                    }
                }
            });

            sc_service::error::Result::Ok(())
        };

        async {
            match try_closure().await {
                Ok(()) => {}
                Err(e) => {
                    panic!("Failed to start container chain node: {}", e);
                }
            }
        }
    }

    /// Stop a container chain. Prints a warning if the container chain was not running.
    fn stop(&self, container_chain_para_id: ParaId) {
        let stop_handle = self
            .spawned_para_ids
            .lock()
            .expect("poison error")
            .remove(&container_chain_para_id);

        match stop_handle {
            Some(_stop_handle) => {
                log::info!("Stopping container chain {}", container_chain_para_id);
            }
            None => {
                log::warn!(
                    "Tried to stop a container chain that is not running: {}",
                    container_chain_para_id
                );
            }
        }
    }

    /// Receive and process `CcSpawnMsg`s indefinitely
    pub async fn rx_loop(self, mut rx: UnboundedReceiver<CcSpawnMsg>) {
        while let Some(msg) = rx.recv().await {
            match msg {
                CcSpawnMsg::Spawn(para_id) => {
                    if para_id == self.orchestrator_para_id {
                        // Restart collation on orchestrator chain
                        let f = (self.collate_on_tanssi)();
                        f.await;
                    } else {
                        // Spawn new container chain node
                        self.spawn_handle.spawn(
                            "container-chain-spawner",
                            None,
                            self.spawn(para_id),
                        );
                    }
                }
                CcSpawnMsg::Stop(para_id) => {
                    if para_id == self.orchestrator_para_id {
                        // Do nothing, because currently the only way to stop collation
                        // on the orchestrator chain is to start a new container chain.
                    } else {
                        self.stop(para_id);
                    }
                }
            }
        }
    }
}
