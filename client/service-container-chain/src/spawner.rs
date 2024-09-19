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

//! Container Chain Spawner
//!
//! Controls the starting and stopping of container chains.
//!
//! For more information about when the database is deleted, check the
//! [Keep db flowchart](https://raw.githubusercontent.com/moondance-labs/tanssi/master/docs/keep_db_flowchart.png)

use {
    crate::{
        cli::ContainerChainCli,
        monitor::{SpawnedContainer, SpawnedContainersMonitor},
        service::{start_node_impl_container, ContainerChainClient, ParachainClient},
    },
    cumulus_primitives_core::ParaId,
    cumulus_relay_chain_interface::RelayChainInterface,
    dancebox_runtime::{opaque::Block as OpaqueBlock, Block},
    dc_orchestrator_chain_interface::{OrchestratorChainInterface, PHash},
    fs2::FileExt,
    futures::FutureExt,
    node_common::command::generate_genesis_block,
    pallet_author_noting_runtime_api::AuthorNotingApi,
    polkadot_primitives::CollatorPair,
    sc_cli::{Database, SyncMode},
    sc_network::config::MultiaddrWithPeerId,
    sc_service::SpawnTaskHandle,
    sc_transaction_pool::FullPool,
    sp_api::ProvideRuntimeApi,
    sp_core::H256,
    sp_keystore::KeystorePtr,
    sp_runtime::traits::Block as BlockT,
    std::{
        collections::{HashMap, HashSet},
        path::{Path, PathBuf},
        sync::{Arc, Mutex},
        time::Instant,
    },
    tokio::{
        sync::{mpsc, oneshot},
        time::{sleep, Duration},
    },
    tokio_util::sync::CancellationToken,
};

/// Timeout to wait for the database to close before starting it again, used in `wait_for_paritydb_lock`.
/// This is the max timeout, if the db is closed in 1 second then that function will only wait 1 second.
const MAX_DB_RESTART_TIMEOUT: Duration = Duration::from_secs(60);

/// Block diff threshold above which we decide it will be faster to delete the database and
/// use warp sync, rather than using full sync to download a large number of blocks.
/// This is only needed because warp sync does not support syncing from a state that is not
/// genesis, it falls back to full sync in that case.
/// 30_000 blocks = 50 hours at 6s/block.
/// Assuming a syncing speed of 100 blocks per second, this will take 5 minutes to sync.
const MAX_BLOCK_DIFF_FOR_FULL_SYNC: u32 = 30_000;

pub trait TSelectSyncMode:
    Send + Sync + Clone + 'static + (Fn(bool, ParaId) -> sc_service::error::Result<SyncMode>)
{
}
impl<
        T: Send + Sync + Clone + 'static + (Fn(bool, ParaId) -> sc_service::error::Result<SyncMode>),
    > TSelectSyncMode for T
{
}

/// Task that handles spawning a stopping container chains based on assignment.
/// The main loop is [rx_loop](ContainerChainSpawner::rx_loop).
pub struct ContainerChainSpawner<SelectSyncMode> {
    /// Start container chain params
    pub params: ContainerChainSpawnParams<SelectSyncMode>,

    /// State
    pub state: Arc<Mutex<ContainerChainSpawnerState>>,

    /// Async callback that enables collation on the orchestrator chain
    pub collate_on_tanssi:
        Arc<dyn Fn() -> (CancellationToken, futures::channel::oneshot::Receiver<()>) + Send + Sync>,
    /// Stores the cancellation token used to stop the orchestrator chain collator process.
    /// When this is None, the orchestrator collator is not running.
    pub collation_cancellation_constructs:
        Option<(CancellationToken, futures::channel::oneshot::Receiver<()>)>,
}

/// Struct with all the params needed to start a container chain node given the CLI arguments,
/// and creating the ChainSpec from on-chain data from the orchestrator chain.
/// These params must be the same for all container chains, params that change such as the
/// `container_chain_para_id` should be passed as separate arguments to the [try_spawn] function.
///
/// This struct MUST NOT contain types (outside of `Option<CollationParams>`) obtained through
/// running an embeded orchestrator node, as this will prevent spawning a container chain in a node
/// connected to an orchestrator node through WebSocket.
#[derive(Clone)]
pub struct ContainerChainSpawnParams<SelectSyncMode> {
    pub orchestrator_chain_interface: Arc<dyn OrchestratorChainInterface>,
    pub container_chain_cli: ContainerChainCli,
    pub tokio_handle: tokio::runtime::Handle,
    pub chain_type: sc_chain_spec::ChainType,
    pub relay_chain: String,
    pub relay_chain_interface: Arc<dyn RelayChainInterface>,
    pub sync_keystore: KeystorePtr,
    pub orchestrator_para_id: ParaId,
    pub spawn_handle: SpawnTaskHandle,
    pub collation_params: Option<CollationParams>,
    pub sync_mode: SelectSyncMode,
    pub data_preserver: bool,
}

/// Params specific to collation. This struct can contain types obtained through running an
/// embeded orchestrator node.
#[derive(Clone)]
pub struct CollationParams {
    pub collator_key: CollatorPair,
    pub orchestrator_tx_pool: Arc<FullPool<OpaqueBlock, ParachainClient>>,
    pub orchestrator_client: Arc<ParachainClient>,
    pub orchestrator_para_id: ParaId,
    pub solochain: bool,
}

/// Mutable state for container chain spawner. Keeps track of running chains.
#[derive(Default)]
pub struct ContainerChainSpawnerState {
    spawned_container_chains: HashMap<ParaId, ContainerChainState>,
    assigned_para_id: Option<ParaId>,
    next_assigned_para_id: Option<ParaId>,
    failed_para_ids: HashSet<ParaId>,
    // For debugging and detecting errors
    pub spawned_containers_monitor: SpawnedContainersMonitor,
}

pub struct ContainerChainState {
    /// Handle that can be used to stop the container chain
    stop_handle: StopContainerChain,
    /// Database path
    db_path: PathBuf,
}

/// Stops a container chain when signal is sent. The bool means `keep_db`, whether to keep the
/// container chain database (true) or remove it (false).
pub struct StopContainerChain {
    signal: oneshot::Sender<bool>,
    id: usize,
}

/// Messages used to control the `ContainerChainSpawner`. This is needed because one of the fields
/// of `ContainerChainSpawner` is not `Sync`, so we cannot simply pass an
/// `Arc<ContainerChainSpawner>` to other threads.
#[derive(Debug)]
pub enum CcSpawnMsg {
    /// Update container chain assignment
    UpdateAssignment {
        current: Option<ParaId>,
        next: Option<ParaId>,
    },
}

// Separate function to allow using `?` to return a result, and also to avoid using `self` in an
// async function. Mutable state should be written by locking `state`.
// TODO: `state` should be an async mutex
async fn try_spawn<SelectSyncMode: TSelectSyncMode>(
    try_spawn_params: ContainerChainSpawnParams<SelectSyncMode>,
    state: Arc<Mutex<ContainerChainSpawnerState>>,
    container_chain_para_id: ParaId,
    start_collation: bool,
) -> sc_service::error::Result<()> {
    let ContainerChainSpawnParams {
        orchestrator_chain_interface,
        mut container_chain_cli,
        tokio_handle,
        chain_type,
        relay_chain,
        relay_chain_interface,
        sync_keystore,
        spawn_handle,
        mut collation_params,
        sync_mode,
        data_preserver,
        ..
    } = try_spawn_params;
    // Preload genesis data from orchestrator chain storage.

    // TODO: the orchestrator chain node may not be fully synced yet,
    // in that case we will be reading an old state.
    let orchestrator_block_hash = orchestrator_chain_interface
        .finalized_block_hash()
        .await
        .map_err(|e| format!("Failed to get latest block hash: {e}"))?;

    log::info!(
        "Detected assignment for container chain {}",
        container_chain_para_id
    );

    let genesis_data = orchestrator_chain_interface
        .genesis_data(orchestrator_block_hash, container_chain_para_id)
        .await
        .map_err(|e| format!("Failed to call genesis_data runtime api: {}", e))?
        .ok_or_else(|| {
            format!(
                "No genesis data registered for container chain id {}",
                container_chain_para_id
            )
        })?;

    let boot_nodes_raw = orchestrator_chain_interface
        .boot_nodes(orchestrator_block_hash, container_chain_para_id)
        .await
        .map_err(|e| format!("Failed to call boot_nodes runtime api: {}", e))?;

    if boot_nodes_raw.is_empty() {
        log::warn!(
            "No boot nodes registered on-chain for container chain {}",
            container_chain_para_id
        );
    }
    let boot_nodes = parse_boot_nodes_ignore_invalid(boot_nodes_raw, container_chain_para_id);
    if boot_nodes.is_empty() {
        log::warn!(
            "No valid boot nodes for container chain {}",
            container_chain_para_id
        );
    }

    container_chain_cli
        .preload_chain_spec_from_genesis_data(
            container_chain_para_id.into(),
            genesis_data,
            chain_type.clone(),
            relay_chain.clone(),
            boot_nodes,
        )
        .map_err(|e| {
            format!(
                "failed to create container chain chain spec from on chain genesis data: {}",
                e
            )
        })?;

    log::info!(
        "Loaded chain spec for container chain {}",
        container_chain_para_id
    );

    if !data_preserver && !start_collation {
        log::info!("This is a syncing container chain, using random ports");

        collation_params = None;

        // Use random ports to avoid conflicts with the other running container chain
        let random_ports = [23456, 23457, 23458];

        container_chain_cli
            .base
            .base
            .prometheus_params
            .prometheus_port = Some(random_ports[0]);
        container_chain_cli.base.base.network_params.port = Some(random_ports[1]);
        container_chain_cli.base.base.rpc_port = Some(random_ports[2]);
    }

    let validator = collation_params.is_some();

    // Update CLI params
    container_chain_cli.base.para_id = Some(container_chain_para_id.into());
    container_chain_cli
        .base
        .base
        .import_params
        .database_params
        .database = Some(Database::ParityDb);

    let keep_db = container_chain_cli.base.keep_db;

    // Get a closure that checks if db_path exists.Need this to know when to use full sync instead of warp sync.
    let check_db_exists = {
        // Get db_path from config
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
            .ok_or_else(|| "Failed to get database path".to_string())?
            .to_owned();
        db_path.set_file_name(format!("full-container-{}", container_chain_para_id));
        container_chain_cli_config.database.set_path(&db_path);

        // Return a closure because we may need to check if the db exists multiple times
        move || db_path.exists()
    };

    // Start container chain node. After starting, check if the database is good or needs to
    // be removed. If the db needs to be removed, this function will handle the node restart, and
    // return the components of a running container chain node.
    // This should be a separate function, but it has so many arguments that I prefer to have it as a closure for now
    let start_node_impl_container_with_restart = || async move {
        // Loop will run at most 2 times: 1 time if the db is good and 2 times if the db needs to be removed
        for _ in 0..2 {
            let db_existed_before = check_db_exists();
            container_chain_cli.base.base.network_params.sync =
                sync_mode(db_existed_before, container_chain_para_id)?;
            log::info!(
                "Container chain sync mode: {:?}",
                container_chain_cli.base.base.network_params.sync
            );

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
                .ok_or_else(|| "Failed to get database path".to_string())?
                .to_owned();
            db_path.set_file_name(format!("full-container-{}", container_chain_para_id));
            container_chain_cli_config.database.set_path(&db_path);

            let (container_chain_task_manager, container_chain_client, container_chain_db) =
                start_node_impl_container(
                    container_chain_cli_config,
                    relay_chain_interface.clone(),
                    orchestrator_chain_interface.clone(),
                    sync_keystore.clone(),
                    container_chain_para_id,
                    collation_params.clone(),
                )
                .await?;

            // Keep all node parts in one variable to make them easier to drop
            let node_parts = (
                container_chain_task_manager,
                container_chain_client,
                container_chain_db,
                db_path,
            );

            if db_existed_before {
                // If the database already existed before, check if it can be used or it needs to be removed.
                // To remove the database, we restart the node, wait for the db to close to avoid a
                // "shutdown error" log, and then remove it.
                if let Some(db_removal_reason) = db_needs_removal(
                    &node_parts.1,
                    &orchestrator_chain_interface,
                    orchestrator_block_hash,
                    container_chain_para_id,
                    &container_chain_cli,
                    container_chain_cli.base.keep_db,
                )
                .await?
                {
                    let db_path = node_parts.3.clone();
                    // Important, drop `node_parts` before trying to `wait_for_paritydb_lock`
                    drop(node_parts);
                    // Wait here to for the database created in the previous loop iteration to close.
                    // Dropping is not enough because there is some background process that keeps the database open,
                    // so we check the paritydb lock file directly.
                    log::info!(
                        "Restarting container chain {} after db deletion. Reason: {:?}",
                        container_chain_para_id,
                        db_removal_reason,
                    );
                    wait_for_paritydb_lock(&db_path, MAX_DB_RESTART_TIMEOUT)
                        .await
                        .map_err(|e| {
                            log::warn!(
                                "Error waiting for chain {} to release db lock: {:?}",
                                container_chain_para_id,
                                e
                            );

                            e
                        })?;
                    delete_container_chain_db(&db_path);

                    // Recursion, will only happen once because `db_existed_before` will be false after
                    // removing the db. Apparently closures cannot be recursive so fake recursion by
                    // using a loop + continue
                    continue;
                }
            }

            // If using full sync, print a warning if the local db is at block 0 and the chain has thousands of blocks
            if container_chain_cli.base.base.network_params.sync == SyncMode::Full {
                let last_container_block_temp = node_parts.1.chain_info().best_number;
                let cc_block_num = get_latest_container_block_number_from_orchestrator(
                    &orchestrator_chain_interface,
                    orchestrator_block_hash,
                    container_chain_para_id,
                )
                .await
                .unwrap_or(0);
                if last_container_block_temp == 0 && cc_block_num > MAX_BLOCK_DIFF_FOR_FULL_SYNC {
                    let db_folder = format!("full-container-{}", container_chain_para_id);
                    log::error!("\
                        Existing database for container chain {} is at block 0, assuming that warp sync failed.\n\
                        The node will now use full sync, which has to download {} blocks.\n\
                        If running as collator, it may not finish syncing on time and miss block rewards.\n\
                        To force using warp sync, stop tanssi-node and manually remove the db folder: {:?}\n\
                        ", container_chain_para_id, cc_block_num, db_folder)
                }
            }

            return sc_service::error::Result::Ok(node_parts);
        }

        unreachable!("Above loop can run at most 2 times, and in the second iteration it is guaranteed to return")
    };

    let (mut container_chain_task_manager, container_chain_client, container_chain_db, db_path) =
        start_node_impl_container_with_restart().await?;

    // Signal that allows to gracefully stop a container chain
    let (signal, on_exit) = oneshot::channel::<bool>();

    let monitor_id;
    {
        let mut state = state.lock().expect("poison error");

        monitor_id = state.spawned_containers_monitor.push(SpawnedContainer {
            id: 0,
            para_id: container_chain_para_id,
            start_time: Instant::now(),
            stop_signal_time: None,
            stop_task_manager_time: None,
            stop_refcount_time: Default::default(),
            backend: Arc::downgrade(&container_chain_db),
            client: Arc::downgrade(&container_chain_client),
        });

        if state
            .spawned_container_chains
            .contains_key(&container_chain_para_id)
        {
            return Err(format!("Tried to spawn a container chain when another container chain with the same para id was already running: {:?}", container_chain_para_id).into());
        }
        state.spawned_container_chains.insert(
            container_chain_para_id,
            ContainerChainState {
                stop_handle: StopContainerChain {
                    signal,
                    id: monitor_id,
                },
                db_path: db_path.clone(),
            },
        );
    }

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
                // An essential task failed or the task manager was stopped unexpectedly
                // using `.terminate()`. This should stop the container chain but not the node.
                if res1.is_err() {
                    log::error!("Essential task failed in container chain {} task manager. Shutting down container chain service", container_chain_para_id);
                } else {
                    log::error!("Unexpected shutdown in container chain {} task manager. Shutting down container chain service", container_chain_para_id);
                }
                // Mark this container chain as "failed to stop" to avoid warning in `self.stop()`
                let mut state = state.lock().expect("poison error");
                state.failed_para_ids.insert(container_chain_para_id);
                // Never delete db in this case because it is not a graceful shutdown
            }
            stop_unassigned = on_exit_future => {
                // Graceful shutdown.
                // `stop_unassigned` will be `Ok(keep_db)` if `.stop()` has been called, which means that the
                // container chain has been unassigned, and will be `Err` if the handle has been dropped,
                // which means that the node is stopping.
                // Delete existing database if running as collator
                if validator && stop_unassigned == Ok(false) && !keep_db {
                    // If this breaks after a code change, make sure that all the variables that
                    // may keep the chain alive are dropped before the call to `wait_for_paritydb_lock`.
                    drop(container_chain_task_manager_future);
                    drop(container_chain_task_manager);
                    let db_closed = wait_for_paritydb_lock(&db_path, MAX_DB_RESTART_TIMEOUT)
                        .await
                        .map_err(|e| {
                            log::warn!(
                                "Error waiting for chain {} to release db lock: {:?}",
                                container_chain_para_id,
                                e
                            );
                        }).is_ok();
                    // If db has not closed in 60 seconds we do not delete it.
                    if db_closed {
                        delete_container_chain_db(&db_path);
                    }
                }
            }
        }

        let mut state = state.lock().expect("poison error");
        state
            .spawned_containers_monitor
            .set_stop_task_manager_time(monitor_id, Instant::now());
    });

    Ok(())
}

/// Interface for spawning and stopping container chain embeded nodes.
pub trait Spawner {
    /// Access to the Orchestrator Chain Interface
    fn orchestrator_chain_interface(&self) -> Arc<dyn OrchestratorChainInterface>;

    /// Try to start a new container chain. In case of an error, this does not stop the node, and
    /// the container chain will be attempted to spawn again when the collator is reassigned to it.
    ///
    /// It is possible that we try to spawn-stop-spawn the same chain, and the second spawn fails
    /// because the chain has not stopped yet, because `stop` does not wait for the chain to stop,
    /// so before calling `spawn` make sure to call `wait_for_paritydb_lock` before, like we do in
    /// `handle_update_assignment`.
    fn spawn(
        &self,
        container_chain_para_id: ParaId,
        start_collation: bool,
    ) -> impl std::future::Future<Output = ()> + Send;

    /// Stop a container chain. Prints a warning if the container chain was not running.
    /// Returns the database path for the container chain, can be used with `wait_for_paritydb_lock`
    /// to ensure that the container chain has fully stopped. The database path can be `None` if the
    /// chain was not running.
    fn stop(&self, container_chain_para_id: ParaId, keep_db: bool) -> Option<PathBuf>;
}

impl<SelectSyncMode: TSelectSyncMode> Spawner for ContainerChainSpawner<SelectSyncMode> {
    /// Access to the Orchestrator Chain Interface
    fn orchestrator_chain_interface(&self) -> Arc<dyn OrchestratorChainInterface> {
        self.params.orchestrator_chain_interface.clone()
    }

    /// Try to start a new container chain. In case of an error, this does not stop the node, and
    /// the container chain will be attempted to spawn again when the collator is reassigned to it.
    ///
    /// It is possible that we try to spawn-stop-spawn the same chain, and the second spawn fails
    /// because the chain has not stopped yet, because `stop` does not wait for the chain to stop,
    /// so before calling `spawn` make sure to call `wait_for_paritydb_lock` before, like we do in
    /// `handle_update_assignment`.
    async fn spawn(&self, container_chain_para_id: ParaId, start_collation: bool) {
        let try_spawn_params = self.params.clone();
        let state = self.state.clone();
        let state2 = state.clone();

        match try_spawn(
            try_spawn_params,
            state,
            container_chain_para_id,
            start_collation,
        )
        .await
        {
            Ok(()) => {}
            Err(e) => {
                log::error!(
                    "Failed to start container chain {}: {}",
                    container_chain_para_id,
                    e
                );
                // Mark this container chain as "failed to start"
                let mut state = state2.lock().expect("poison error");
                state.failed_para_ids.insert(container_chain_para_id);
            }
        }
    }

    /// Stop a container chain. Prints a warning if the container chain was not running.
    /// Returns the database path for the container chain, can be used with `wait_for_paritydb_lock`
    /// to ensure that the container chain has fully stopped. The database path can be `None` if the
    /// chain was not running.
    fn stop(&self, container_chain_para_id: ParaId, keep_db: bool) -> Option<PathBuf> {
        let mut state = self.state.lock().expect("poison error");
        let stop_handle = state
            .spawned_container_chains
            .remove(&container_chain_para_id);

        match stop_handle {
            Some(stop_handle) => {
                log::info!("Stopping container chain {}", container_chain_para_id);

                let id = stop_handle.stop_handle.id;
                state
                    .spawned_containers_monitor
                    .set_stop_signal_time(id, Instant::now());

                // Send signal to perform graceful shutdown, which will delete the db if needed
                let _ = stop_handle.stop_handle.signal.send(keep_db);

                Some(stop_handle.db_path)
            }
            None => {
                // Do not print the warning message if this is a container chain that has failed to
                // start, because in that case it will not be running
                if !state.failed_para_ids.remove(&container_chain_para_id) {
                    log::warn!(
                        "Tried to stop a container chain that is not running: {}",
                        container_chain_para_id
                    );
                }

                None
            }
        }
    }
}

impl<SelectSyncMode: TSelectSyncMode> ContainerChainSpawner<SelectSyncMode> {
    /// Receive and process `CcSpawnMsg`s indefinitely
    pub async fn rx_loop(
        mut self,
        mut rx: mpsc::UnboundedReceiver<CcSpawnMsg>,
        validator: bool,
        solochain: bool,
    ) {
        // The node always starts as an orchestrator chain collator.
        // This is because the assignment is detected after importing a new block, so if all
        // collators stop at the same time, when they start again nobody will produce the new block.
        // So all nodes start as orchestrator chain collators, until the first block is imported,
        // then the real assignment is used.
        // Except in solochain mode, then the initial assignment is None.
        if validator && !solochain {
            self.handle_update_assignment(Some(self.params.orchestrator_para_id), None)
                .await;
        }

        while let Some(msg) = rx.recv().await {
            match msg {
                CcSpawnMsg::UpdateAssignment { current, next } => {
                    self.handle_update_assignment(current, next).await;
                }
            }
        }

        // The while loop can end if all the senders get dropped, but since this is an
        // essential task we don't want it to stop. So await a future that never completes.
        // This should only happen when starting a full node.
        if !validator {
            let () = std::future::pending().await;
        }
    }

    /// Handle `CcSpawnMsg::UpdateAssignment`
    async fn handle_update_assignment(&mut self, current: Option<ParaId>, next: Option<ParaId>) {
        let HandleUpdateAssignmentResult {
            chains_to_stop,
            chains_to_start,
            need_to_restart: _,
        } = handle_update_assignment_state_change(
            &mut self.state.lock().expect("poison error"),
            self.params.orchestrator_para_id,
            current,
            next,
        );

        if current != Some(self.params.orchestrator_para_id) {
            // If not assigned to orchestrator chain anymore, we need to stop the collator process
            let maybe_exit_notification_receiver = self
                .collation_cancellation_constructs
                .take()
                .map(|(cancellation_token, exit_notification_receiver)| {
                    cancellation_token.cancel();
                    exit_notification_receiver
                });

            if let Some(exit_notification_receiver) = maybe_exit_notification_receiver {
                let _ = exit_notification_receiver.await;
            }
        } else if self.collation_cancellation_constructs.is_none() {
            // If assigned to orchestrator chain but the collator process is not running, start it
            self.collation_cancellation_constructs = Some((self.collate_on_tanssi)());
        }

        // Stop all container chains that are no longer needed
        let mut db_paths_restart = vec![];
        for para_id in chains_to_stop {
            // Keep db if we are currently assigned to this chain
            let keep_db = Some(para_id) == current;
            let maybe_db_path = self.stop(para_id, keep_db);
            // If we are restarting this chain, save its db_path to check when it actually stopped
            if let Some(db_path) = maybe_db_path {
                if chains_to_start.contains(&para_id) {
                    db_paths_restart.push((para_id, db_path));
                }
            }
        }

        if !db_paths_restart.is_empty() {
            // Ensure the chains we stopped actually stopped by checking if their database is unlocked.
            // Using `join_all` because in one edge case we may be restarting 2 chains,
            // but almost always this will be only one future.
            let futs = db_paths_restart
                .into_iter()
                .map(|(para_id, db_path)| async move {
                    wait_for_paritydb_lock(&db_path, MAX_DB_RESTART_TIMEOUT)
                        .await
                        .map_err(|e| {
                            log::warn!(
                                "Error waiting for chain {} to release db lock: {:?}",
                                para_id,
                                e
                            );
                        })
                });
            futures::future::join_all(futs).await;
        }

        // Start all new container chains (usually 1)
        for para_id in chains_to_start {
            // Edge case: when starting the node it may be assigned to a container chain, so we need to
            // start a container chain already collating.
            // TODO: another edge case: if current == None, and running_chains == 0,
            // and chains_to_start == 1, we can start this chain as collating, and we won't need
            // to restart it on the next session. We need to add some extra state somewhere to
            // implement this properly.
            let start_collation = Some(para_id) == current;
            self.spawn(para_id, start_collation).await;
        }
    }
}

struct HandleUpdateAssignmentResult {
    chains_to_stop: Vec<ParaId>,
    chains_to_start: Vec<ParaId>,
    #[allow(dead_code)] // no longer used except in tests
    need_to_restart: bool,
}

// This is a separate function to allow testing
fn handle_update_assignment_state_change(
    state: &mut ContainerChainSpawnerState,
    orchestrator_para_id: ParaId,
    current: Option<ParaId>,
    next: Option<ParaId>,
) -> HandleUpdateAssignmentResult {
    if (state.assigned_para_id, state.next_assigned_para_id) == (current, next) {
        // If nothing changed there is nothing to update
        return HandleUpdateAssignmentResult {
            chains_to_stop: Default::default(),
            chains_to_start: Default::default(),
            need_to_restart: false,
        };
    }

    // Create a set with the container chains that were running before, and the container
    // chains that should be running after the updated assignment. This is used to calculate
    // the difference, and stop and start the required container chains.
    let mut running_chains_before = HashSet::new();
    let mut running_chains_after = HashSet::new();

    running_chains_before.extend(state.assigned_para_id);
    running_chains_before.extend(state.next_assigned_para_id);
    // Ignore orchestrator_para_id because it is handled in a special way, as it does not need to
    // start one session before in order to sync.
    running_chains_before.remove(&orchestrator_para_id);

    running_chains_after.extend(current);
    running_chains_after.extend(next);
    running_chains_after.remove(&orchestrator_para_id);
    let mut need_to_restart_current = false;
    let mut need_to_restart_next = false;

    if state.assigned_para_id != current {
        if let Some(para_id) = current {
            // If the assigned container chain has changed, we may need to
            // restart it in collation mode, unless it is the orchestrator chain.
            if para_id != orchestrator_para_id {
                need_to_restart_current = true;
            }
        }

        if let Some(para_id) = state.assigned_para_id {
            if para_id != orchestrator_para_id && Some(para_id) == next {
                need_to_restart_next = true;
            }
        }
    }

    state.assigned_para_id = current;
    state.next_assigned_para_id = next;

    let mut chains_to_stop: Vec<_> = running_chains_before
        .difference(&running_chains_after)
        .copied()
        .collect();
    let mut chains_to_start: Vec<_> = running_chains_after
        .difference(&running_chains_before)
        .copied()
        .collect();

    if need_to_restart_current {
        // Force restart of new assigned container chain: if it was running before it was in "syncing mode",
        // which doesn't use the correct ports, so start it in "collation mode".
        let id = current.unwrap();
        if running_chains_before.contains(&id) && !chains_to_stop.contains(&id) {
            chains_to_stop.push(id);
        }
        if !chains_to_start.contains(&id) {
            chains_to_start.push(id);
        }
    }

    if need_to_restart_next {
        // Handle edge case of going from (2000, 2001) to (2001, 2000). In that case we must restart both chains,
        // because previously 2000 was collating and now 2000 will only be syncing.
        let id = next.unwrap();
        if running_chains_before.contains(&id) && !chains_to_stop.contains(&id) {
            chains_to_stop.push(id);
        }
        if !chains_to_start.contains(&id) {
            chains_to_start.push(id);
        }
    }

    HandleUpdateAssignmentResult {
        chains_to_stop,
        chains_to_start,
        need_to_restart: need_to_restart_current || need_to_restart_next,
    }
}

/// Select [SyncMode] to use for a container chain.
/// We want to use warp sync unless the db still exists, or the container chain is
/// still at genesis block (because of a warp sync bug in that case).
///
/// Remember that warp sync doesn't work if a partially synced database already exists, it falls
/// back to full sync instead. The only exception is if the previous instance of the database was
/// interrupted before it finished downloading the state, in that case the node will use warp sync.
/// If it was interrupted during the block history download, the node will use full sync but also
/// finish the block history download in the background, even if sync mode is set to full sync.
pub fn select_sync_mode_using_client(
    db_exists: bool,
    orchestrator_client: &Arc<ParachainClient>,
    container_chain_para_id: ParaId,
) -> sc_service::error::Result<SyncMode> {
    if db_exists {
        // If the user wants to use warp sync, they should have already removed the database
        return Ok(SyncMode::Full);
    }

    // The following check is only needed because of this bug:
    // https://github.com/paritytech/polkadot-sdk/issues/1930

    let orchestrator_runtime_api = orchestrator_client.runtime_api();
    let orchestrator_chain_info = orchestrator_client.chain_info();

    // If the container chain is still at genesis block, use full sync because warp sync is broken
    let full_sync_needed = orchestrator_runtime_api
        .latest_author(orchestrator_chain_info.best_hash, container_chain_para_id)
        .map_err(|e| format!("Failed to read latest author: {}", e))?
        .is_none();

    if full_sync_needed {
        Ok(SyncMode::Full)
    } else {
        Ok(SyncMode::Warp)
    }
}

async fn get_latest_container_block_number_from_orchestrator(
    orchestrator_chain_interface: &Arc<dyn OrchestratorChainInterface>,
    orchestrator_block_hash: PHash,
    container_chain_para_id: ParaId,
) -> Option<u32> {
    // Get the container chain's latest block from orchestrator chain and compare with client's one
    let last_container_block_from_orchestrator = orchestrator_chain_interface
        .latest_block_number(orchestrator_block_hash, container_chain_para_id)
        .await
        .unwrap_or_default();

    last_container_block_from_orchestrator
}

#[derive(Debug)]
#[allow(dead_code)]
enum DbRemovalReason {
    HighBlockDiff {
        best_block_number_db: u32,
        best_block_number_onchain: u32,
    },
    GenesisHashMismatch {
        container_client_genesis_hash: H256,
        chain_spec_genesis_hash_v0: H256,
        chain_spec_genesis_hash_v1: H256,
    },
}

/// Given a container chain client, check if the database is valid. If not, returns `Some` with the
/// reason for db removal.
/// Reasons may be:
/// * High block diff: when the local db is outdated and it would take a long time to sync using full sync, we remove it to be able to use warp sync.
/// * Genesis hash mismatch, when the chain was deregistered and a different chain with the same para id was registered.
async fn db_needs_removal(
    container_chain_client: &Arc<ContainerChainClient>,
    orchestrator_chain_interface: &Arc<dyn OrchestratorChainInterface>,
    orchestrator_block_hash: PHash,
    container_chain_para_id: ParaId,
    container_chain_cli: &ContainerChainCli,
    keep_db: bool,
) -> sc_service::error::Result<Option<DbRemovalReason>> {
    // Check block diff, only needed if keep-db is false
    if !keep_db {
        // Get latest block number from the container chain client
        let last_container_block_temp = container_chain_client.chain_info().best_number;
        if last_container_block_temp == 0 {
            // Don't remove an empty database, as it may be in the process of a warp sync
        } else {
            if get_latest_container_block_number_from_orchestrator(
                orchestrator_chain_interface,
                orchestrator_block_hash,
                container_chain_para_id,
            )
            .await
            .unwrap_or(0)
            .abs_diff(last_container_block_temp)
                > MAX_BLOCK_DIFF_FOR_FULL_SYNC
            {
                // if the diff is big, delete db and restart using warp sync
                return Ok(Some(DbRemovalReason::HighBlockDiff {
                    best_block_number_db: last_container_block_temp,
                    best_block_number_onchain: last_container_block_temp,
                }));
            }
        }
    }

    // Generate genesis hash to compare against container client's genesis hash
    let container_preloaded_genesis = container_chain_cli.preloaded_chain_spec.as_ref().unwrap();

    // Check with both state versions, but first v1 which is the latest
    let block_v1: Block =
        generate_genesis_block(&**container_preloaded_genesis, sp_runtime::StateVersion::V1)
            .map_err(|e| format!("{:?}", e))?;
    let chain_spec_genesis_hash_v1 = block_v1.header().hash();

    let container_client_genesis_hash = container_chain_client.chain_info().genesis_hash;

    if container_client_genesis_hash != chain_spec_genesis_hash_v1 {
        let block_v0: Block =
            generate_genesis_block(&**container_preloaded_genesis, sp_runtime::StateVersion::V0)
                .map_err(|e| format!("{:?}", e))?;
        let chain_spec_genesis_hash_v0 = block_v0.header().hash();

        if container_client_genesis_hash != chain_spec_genesis_hash_v0 {
            log::info!("Container genesis V0: {:?}", chain_spec_genesis_hash_v0);
            log::info!("Container genesis V1: {:?}", chain_spec_genesis_hash_v1);
            log::info!(
                "Chain spec genesis {:?} did not match with any container genesis - Restarting...",
                container_client_genesis_hash
            );
            return Ok(Some(DbRemovalReason::GenesisHashMismatch {
                container_client_genesis_hash,
                chain_spec_genesis_hash_v0,
                chain_spec_genesis_hash_v1,
            }));
        }
    }

    Ok(None)
}

/// Remove the container chain database folder. This is called with db_path:
///     `Collator2002-01/data/containers/chains/simple_container_2002/paritydb/full-container-2002`
/// but we want to delete everything under
///     `Collator2002-01/data/containers/chains/simple_container_2002`
/// So we use `delete_empty_folders_recursive` to try to remove the parent folders as well, but only
/// if they are empty. This is to avoid removing any secret keys or other important data.
fn delete_container_chain_db(db_path: &Path) {
    // Remove folder `full-container-2002`
    let _ = std::fs::remove_dir_all(db_path);
    // Remove all the empty folders inside `simple_container_2002`, including self
    if let Some(parent) = db_path.ancestors().nth(2) {
        delete_empty_folders_recursive(parent);
    }
}

/// Removes all empty folders in `path`, recursively. Then, if `path` is empty, it removes it as well.
/// Ignores any IO errors.
fn delete_empty_folders_recursive(path: &Path) {
    let entry_iter = std::fs::read_dir(path);
    let entry_iter = match entry_iter {
        Ok(x) => x,
        Err(_e) => return,
    };

    for entry in entry_iter {
        let entry = match entry {
            Ok(x) => x,
            Err(_e) => continue,
        };

        let path = entry.path();
        if path.is_dir() {
            delete_empty_folders_recursive(&path);
        }
    }

    // Try to remove dir. Returns an error if the directory is not empty, but we ignore it.
    let _ = std::fs::remove_dir(path);
}

/// Parse a list of boot nodes in `Vec<u8>` format. Invalid boot nodes are filtered out.
fn parse_boot_nodes_ignore_invalid(
    boot_nodes_raw: Vec<Vec<u8>>,
    container_chain_para_id: ParaId,
) -> Vec<MultiaddrWithPeerId> {
    boot_nodes_raw
        .into_iter()
        .filter_map(|x| {
            let x = String::from_utf8(x)
                .map_err(|e| {
                    log::debug!(
                        "Invalid boot node in container chain {}: {}",
                        container_chain_para_id,
                        e
                    );
                })
                .ok()?;

            x.parse::<MultiaddrWithPeerId>()
                .map_err(|e| {
                    log::debug!(
                        "Invalid boot node in container chain {}: {}",
                        container_chain_para_id,
                        e
                    )
                })
                .ok()
        })
        .collect()
}

pub async fn wait_for_paritydb_lock(db_path: &Path, max_timeout: Duration) -> Result<(), String> {
    let now = Instant::now();

    while now.elapsed() < max_timeout {
        let lock_held = check_paritydb_lock_held(db_path)
            .map_err(|e| format!("Failed to check if lock file is held: {}", e))?;
        if !lock_held {
            return Ok(());
        }
        sleep(Duration::from_secs(1)).await;
    }

    Err("Timeout when waiting for paritydb lock".to_string())
}

/// Given a path to a paritydb database, check if its lock file is held. This indicates that a
/// background process is still using the database, so we should wait before trying to open it.
///
/// This should be kept up to date with the way paritydb handles the lock file:
/// <https://github.com/paritytech/parity-db/blob/2b6820e310a08678d4540c044f41a93d87343ac8/src/db.rs#L215>
fn check_paritydb_lock_held(db_path: &Path) -> Result<bool, std::io::Error> {
    if !db_path.is_dir() {
        // Lock file does not exist, so it is not held
        return Ok(false);
    }

    let mut lock_path: std::path::PathBuf = db_path.to_owned();
    lock_path.push("lock");
    let lock_file = std::fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(true)
        .open(lock_path.as_path())?;
    // Check if the lock file is busy by trying to lock it.
    // Returns err if failed to adquire the lock.
    let lock_held = lock_file.try_lock_exclusive().is_err();

    Ok(lock_held)
}

#[cfg(test)]
mod tests {
    use {super::*, std::path::PathBuf};

    // Copy of ContainerChainSpawner with extra assertions for tests, and mocked spawn function.
    struct MockContainerChainSpawner {
        state: Arc<Mutex<ContainerChainSpawnerState>>,
        orchestrator_para_id: ParaId,
        collate_on_tanssi: Arc<
            dyn Fn() -> (CancellationToken, futures::channel::oneshot::Receiver<()>) + Send + Sync,
        >,
        collation_cancellation_constructs: Option<()>,
        // Keep track of the last CollateOn message, for tests
        currently_collating_on: Arc<Mutex<Option<ParaId>>>,
    }

    impl MockContainerChainSpawner {
        fn new() -> Self {
            let orchestrator_para_id = 1000.into();
            // The node always starts as an orchestrator chain collator
            let currently_collating_on = Arc::new(Mutex::new(Some(orchestrator_para_id)));
            let currently_collating_on2 = currently_collating_on.clone();
            let collate_closure = move || {
                let mut cco = currently_collating_on2.lock().unwrap();
                assert_ne!(
                    *cco,
                    Some(orchestrator_para_id),
                    "Received CollateOn message when we were already collating on this chain: {}",
                    orchestrator_para_id
                );
                *cco = Some(orchestrator_para_id);
                let (_, receiver) = futures::channel::oneshot::channel();
                (CancellationToken::new(), receiver)
            };
            let collate_on_tanssi: Arc<
                dyn Fn() -> (CancellationToken, futures::channel::oneshot::Receiver<()>)
                    + Send
                    + Sync,
            > = Arc::new(collate_closure);

            Self {
                state: Arc::new(Mutex::new(ContainerChainSpawnerState {
                    spawned_container_chains: Default::default(),
                    assigned_para_id: Some(orchestrator_para_id),
                    next_assigned_para_id: None,
                    failed_para_ids: Default::default(),
                    spawned_containers_monitor: Default::default(),
                })),
                orchestrator_para_id,
                collate_on_tanssi,
                // Some if collator starts on orchestrator chain
                collation_cancellation_constructs: Some(()),
                currently_collating_on,
            }
        }

        fn spawn(&self, container_chain_para_id: ParaId, start_collation: bool) {
            let (signal, _on_exit) = oneshot::channel();
            let currently_collating_on2 = self.currently_collating_on.clone();
            let collate_closure = move || {
                let mut cco = currently_collating_on2.lock().unwrap();
                assert_ne!(
                    *cco,
                    Some(container_chain_para_id),
                    "Received CollateOn message when we were already collating on this chain: {}",
                    container_chain_para_id
                );
                *cco = Some(container_chain_para_id);
                let (_, receiver) = futures::channel::oneshot::channel();
                (CancellationToken::new(), receiver)
            };
            let collate_on: Arc<
                dyn Fn() -> (CancellationToken, futures::channel::oneshot::Receiver<()>)
                    + Send
                    + Sync,
            > = Arc::new(collate_closure);
            // Dummy db_path for tests, is not actually used
            let db_path = PathBuf::from(format!("/tmp/container-{}/db", container_chain_para_id));

            let old = self
                .state
                .lock()
                .expect("poison error")
                .spawned_container_chains
                .insert(
                    container_chain_para_id,
                    ContainerChainState {
                        stop_handle: StopContainerChain { signal, id: 0 },
                        db_path,
                    },
                );

            assert!(
                old.is_none(),
                "tried to spawn a container chain that was already running: {}",
                container_chain_para_id
            );

            if start_collation {
                let (_cancellation_token, _exit_receiver) = collate_on();
            }
        }

        fn stop(&self, container_chain_para_id: ParaId) {
            let stop_handle = self
                .state
                .lock()
                .expect("poison error")
                .spawned_container_chains
                .remove(&container_chain_para_id);

            match stop_handle {
                Some(_stop_handle) => {
                    log::info!("Stopping container chain {}", container_chain_para_id);
                }
                None => {
                    panic!(
                        "Tried to stop a container chain that is not running: {}",
                        container_chain_para_id
                    );
                }
            }

            // Update currently_collating_on, if we stopped the chain we are no longer collating there
            let mut lco = self.currently_collating_on.lock().unwrap();
            if *lco == Some(container_chain_para_id) {
                *lco = None;
            }
        }

        fn handle_update_assignment(&mut self, current: Option<ParaId>, next: Option<ParaId>) {
            let HandleUpdateAssignmentResult {
                chains_to_stop,
                chains_to_start,
                need_to_restart,
            } = handle_update_assignment_state_change(
                &mut self.state.lock().unwrap(),
                self.orchestrator_para_id,
                current,
                next,
            );

            if current != Some(self.orchestrator_para_id) {
                // If not assigned to orchestrator chain anymore, we need to stop the collator process
                let mut cco = self.currently_collating_on.lock().unwrap();
                if *cco == Some(self.orchestrator_para_id) {
                    *cco = None;
                }
                self.collation_cancellation_constructs = None;
            } else if self.collation_cancellation_constructs.is_none() {
                let (_cancellation_token, _exit_notification_receiver) = (self.collate_on_tanssi)();
                self.collation_cancellation_constructs = Some(());
            }

            // Assert we never start and stop the same container chain
            for para_id in &chains_to_start {
                if !need_to_restart {
                    assert!(
                        !chains_to_stop.contains(para_id),
                        "Tried to start and stop same container chain: {}",
                        para_id
                    );
                } else {
                    // Will try to start and stop container chain with id "current" or "next", so ignore that
                    if Some(*para_id) != current && Some(*para_id) != next {
                        assert!(
                            !chains_to_stop.contains(para_id),
                            "Tried to start and stop same container chain: {}",
                            para_id
                        );
                    }
                }
            }
            // Assert we never start or stop the orchestrator chain
            assert!(!chains_to_start.contains(&self.orchestrator_para_id));
            assert!(!chains_to_stop.contains(&self.orchestrator_para_id));

            // Stop all container chains that are no longer needed
            for para_id in chains_to_stop {
                self.stop(para_id);
            }

            // Start all new container chains (usually 1)
            for para_id in chains_to_start {
                // Edge case: when starting the node it may be assigned to a container chain, so we need to
                // start a container chain already collating.
                let start_collation = Some(para_id) == current;
                self.spawn(para_id, start_collation);
            }

            // Assert that if we are currently assigned to a container chain, we are collating there
            if let Some(para_id) = current {
                self.assert_collating_on(Some(para_id));
            } else {
                self.assert_collating_on(None);
            }
        }

        #[track_caller]
        fn assert_collating_on(&self, para_id: Option<ParaId>) {
            let currently_collating_on = *self.currently_collating_on.lock().unwrap();
            assert_eq!(currently_collating_on, para_id);
        }

        #[track_caller]
        fn assert_running_chains(&self, para_ids: &[ParaId]) {
            let mut actually_running: Vec<ParaId> = self
                .state
                .lock()
                .unwrap()
                .spawned_container_chains
                .keys()
                .cloned()
                .collect();
            actually_running.sort();
            let mut should_be_running = para_ids.to_vec();
            should_be_running.sort();
            assert_eq!(actually_running, should_be_running);
        }
    }

    #[test]
    fn starts_collating_on_tanssi() {
        let mut m = MockContainerChainSpawner::new();
        m.assert_collating_on(Some(1000.into()));
        m.assert_running_chains(&[]);

        m.handle_update_assignment(None, None);
        m.assert_collating_on(None);
        m.assert_running_chains(&[]);
    }

    #[test]
    fn assigned_to_orchestrator_chain() {
        let mut m = MockContainerChainSpawner::new();

        m.handle_update_assignment(Some(1000.into()), Some(1000.into()));
        m.assert_collating_on(Some(1000.into()));
        m.assert_running_chains(&[]);

        m.handle_update_assignment(Some(1000.into()), None);
        m.assert_collating_on(Some(1000.into()));
        m.assert_running_chains(&[]);

        m.handle_update_assignment(None, None);
        m.assert_collating_on(None);
        m.assert_running_chains(&[]);

        m.handle_update_assignment(None, Some(1000.into()));
        m.assert_collating_on(None);
        m.assert_running_chains(&[]);

        m.handle_update_assignment(Some(1000.into()), Some(1000.into()));
        m.assert_collating_on(Some(1000.into()));
        m.assert_running_chains(&[]);
    }

    #[test]
    fn assigned_to_container_chain() {
        let mut m = MockContainerChainSpawner::new();

        m.handle_update_assignment(Some(2000.into()), Some(2000.into()));
        m.assert_collating_on(Some(2000.into()));
        m.assert_running_chains(&[2000.into()]);

        m.handle_update_assignment(Some(2000.into()), None);
        m.assert_collating_on(Some(2000.into()));
        m.assert_running_chains(&[2000.into()]);

        m.handle_update_assignment(None, None);
        m.assert_collating_on(None);
        m.assert_running_chains(&[]);

        m.handle_update_assignment(None, Some(2000.into()));
        m.assert_collating_on(None);
        m.assert_running_chains(&[2000.into()]);

        m.handle_update_assignment(Some(2000.into()), Some(2000.into()));
        m.assert_collating_on(Some(2000.into()));
        m.assert_running_chains(&[2000.into()]);
    }

    #[test]
    fn spawn_container_chains() {
        let mut m = MockContainerChainSpawner::new();

        m.handle_update_assignment(Some(1000.into()), Some(2000.into()));
        m.assert_collating_on(Some(1000.into()));
        m.assert_running_chains(&[2000.into()]);

        m.handle_update_assignment(Some(2000.into()), Some(2000.into()));
        m.assert_collating_on(Some(2000.into()));
        m.assert_running_chains(&[2000.into()]);

        m.handle_update_assignment(Some(2000.into()), Some(2001.into()));
        m.assert_collating_on(Some(2000.into()));
        m.assert_running_chains(&[2000.into(), 2001.into()]);

        m.handle_update_assignment(Some(2001.into()), Some(2001.into()));
        m.assert_collating_on(Some(2001.into()));
        m.assert_running_chains(&[2001.into()]);

        m.handle_update_assignment(Some(2001.into()), Some(1000.into()));
        m.assert_collating_on(Some(2001.into()));
        m.assert_running_chains(&[2001.into()]);

        m.handle_update_assignment(Some(1000.into()), Some(1000.into()));
        m.assert_collating_on(Some(1000.into()));
        m.assert_running_chains(&[]);
    }

    #[test]
    fn swap_current_next() {
        // Going from (2000, 2001) to (2001, 2000) shouldn't start or stop any container chains
        let mut m: MockContainerChainSpawner = MockContainerChainSpawner::new();

        m.handle_update_assignment(Some(2000.into()), Some(2001.into()));
        m.assert_collating_on(Some(2000.into()));
        m.assert_running_chains(&[2000.into(), 2001.into()]);

        m.handle_update_assignment(Some(2001.into()), Some(2000.into()));
        m.assert_collating_on(Some(2001.into()));
        m.assert_running_chains(&[2000.into(), 2001.into()]);
    }

    #[test]
    fn stop_collating_orchestrator() {
        let mut m: MockContainerChainSpawner = MockContainerChainSpawner::new();

        m.handle_update_assignment(Some(1000.into()), Some(1000.into()));
        m.assert_collating_on(Some(1000.into()));
        m.assert_running_chains(&[]);

        m.handle_update_assignment(Some(1000.into()), None);
        m.assert_collating_on(Some(1000.into()));
        m.assert_running_chains(&[]);

        m.handle_update_assignment(None, None);
        m.assert_collating_on(None);
        m.assert_running_chains(&[]);

        m.handle_update_assignment(Some(1000.into()), None);
        m.assert_collating_on(Some(1000.into()));
        m.assert_running_chains(&[]);
    }

    #[test]
    fn stop_collating_container() {
        let mut m: MockContainerChainSpawner = MockContainerChainSpawner::new();

        m.handle_update_assignment(Some(2000.into()), None);
        m.assert_collating_on(Some(2000.into()));
        m.assert_running_chains(&[2000.into()]);

        m.handle_update_assignment(None, None);
        m.assert_collating_on(None);
        m.assert_running_chains(&[]);

        m.handle_update_assignment(None, Some(2000.into()));
        m.assert_collating_on(None);
        m.assert_running_chains(&[2000.into()]);

        // This will send a CollateOn message to the same chain as the last CollateOn,
        // but this is needed because that chain has been stopped
        m.handle_update_assignment(Some(2000.into()), Some(2000.into()));
        m.assert_collating_on(Some(2000.into()));
        m.assert_running_chains(&[2000.into()]);
    }

    #[test]
    fn stop_collating_container_start_immediately() {
        let mut m: MockContainerChainSpawner = MockContainerChainSpawner::new();

        m.handle_update_assignment(Some(2000.into()), None);
        m.assert_collating_on(Some(2000.into()));
        m.assert_running_chains(&[2000.into()]);

        m.handle_update_assignment(None, None);
        m.assert_collating_on(None);
        m.assert_running_chains(&[]);

        // This will start the chain already collating
        m.handle_update_assignment(Some(2000.into()), Some(2000.into()));
        m.assert_collating_on(Some(2000.into()));
        m.assert_running_chains(&[2000.into()]);
    }

    #[test]
    fn stop_all_chains() {
        let mut m: MockContainerChainSpawner = MockContainerChainSpawner::new();

        m.handle_update_assignment(Some(2000.into()), Some(2001.into()));
        m.assert_collating_on(Some(2000.into()));
        m.assert_running_chains(&[2000.into(), 2001.into()]);

        m.handle_update_assignment(None, None);
        m.assert_collating_on(None);
        m.assert_running_chains(&[]);
    }

    #[test]
    fn keep_collating_on_container() {
        let mut m: MockContainerChainSpawner = MockContainerChainSpawner::new();

        m.handle_update_assignment(Some(2000.into()), None);
        m.assert_collating_on(Some(2000.into()));
        m.assert_running_chains(&[2000.into()]);

        m.handle_update_assignment(None, Some(2000.into()));
        m.assert_collating_on(None);
        m.assert_running_chains(&[2000.into()]);

        m.handle_update_assignment(Some(2000.into()), Some(2000.into()));
        m.assert_collating_on(Some(2000.into()));
        m.assert_running_chains(&[2000.into()]);
    }

    #[test]
    fn invalid_boot_nodes_are_ignored() {
        let para_id = 100.into();
        let bootnode1 =
            b"/ip4/127.0.0.1/tcp/33049/ws/p2p/12D3KooWHVMhQDHBpj9vQmssgyfspYecgV6e3hH1dQVDUkUbCYC9"
                .to_vec();
        assert_eq!(
            parse_boot_nodes_ignore_invalid(vec![b"A".to_vec()], para_id),
            vec![]
        );
        assert_eq!(
            parse_boot_nodes_ignore_invalid(vec![b"\xff".to_vec()], para_id),
            vec![]
        );
        // Valid boot nodes are not ignored
        assert_eq!(
            parse_boot_nodes_ignore_invalid(vec![bootnode1], para_id).len(),
            1
        );
    }

    #[test]
    fn path_ancestors() {
        // Test the implementation of `delete_container_chain_db`
        let db_path = PathBuf::from("/tmp/zombienet/Collator2002-01/data/containers/chains/simple_container_2002/paritydb/full-container-2002");
        let parent = db_path.ancestors().nth(2).unwrap();

        assert_eq!(
            parent,
            PathBuf::from(
                "/tmp/zombienet/Collator2002-01/data/containers/chains/simple_container_2002"
            )
        )
    }
}
