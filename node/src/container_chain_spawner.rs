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
use std::time::Duration;
use tc_orchestrator_chain_interface::OrchestratorChainInterface;
use tokio::sync::mpsc::UnboundedSender;

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

// TODO: this is for testing, remove when not needed and remove lazy_static dependency
lazy_static::lazy_static! {
    pub static ref CCSPAWN: Mutex<Option<UnboundedSender<CcSpawnMsg>>> = Mutex::new(None);
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

            // The database path is something like
            // /tmp/zombie-6368e3e070dcee9722a19fb5fa479f21_-4023642-W9VgHHslcbzB/Collator2001-01/data/polkadot/chains/local_testnet/db/full
            // We want to change the last "db/full" into "db/full-container-{}"
            let mut db_path = container_chain_cli_config
                .database
                .path()
                .unwrap()
                .to_owned();
            db_path.set_file_name(format!("full-container-{}", container_chain_para_id));
            log::info!("DB PATH IS {:?}", db_path);
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

            {
                spawned_para_ids
                    .lock()
                    .expect("poison error")
                    .insert(container_chain_para_id.into(), StopContainerChain(signal));
            }

            // Emulate task_manager.add_child by making the parent task manager stop if the
            // container chain task manager stops.
            // The reverse is also true, if the parent task manager stops, the container chain
            // task manager will also stop.
            // But add an additional on_exit future to allow graceful shutdown of container chains.
            // TODO: not sure what is the difference between spawn and spawn_essential, because
            // when a "spawn" task panics it stops the node
            spawn_handle.spawn("container-chain-task-manager", None, async move {
                let mut t1 = container_chain_task_manager.future().fuse();
                let mut t2 = on_exit.fuse();

                futures::select! {
                    res1 = t1 => {
                        match res1 {
                            Ok(()) => panic!("container_chain_task_manager has stopped unexpectedly"),
                            Err(e) => panic!("container_chain_task_manager failed: {}", e),
                        }
                    }
                    _ = t2 => {}
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

    fn stop(&self, container_chain_para_id: ParaId) {
        let kill_handle = self
            .spawned_para_ids
            .lock()
            .expect("poison error")
            .remove(&container_chain_para_id);

        match kill_handle {
            Some(_signal) => {
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

    pub async fn rx_loop(self, mut rx: tokio::sync::mpsc::UnboundedReceiver<CcSpawnMsg>) {
        while let Some(msg) = rx.recv().await {
            match msg {
                CcSpawnMsg::Spawn(para_id) => {
                    // For testing

                    if para_id == 2001.into() {
                        tokio::time::sleep(Duration::from_secs(120)).await;
                        let tx3 = CCSPAWN
                            .lock()
                            .expect("poison error")
                            .as_ref()
                            .cloned()
                            .unwrap();
                        tx3.send(CcSpawnMsg::Stop(2000.into())).unwrap();
                    }

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
