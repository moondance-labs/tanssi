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
        container_chain_monitor::{SpawnedContainer, SpawnedContainersMonitor},
        service::{start_node_impl_container, NodeConfig, ParachainClient},
    },
    cumulus_primitives_core::ParaId,
    cumulus_relay_chain_interface::RelayChainInterface,
    dancebox_runtime::{AccountId, Block, BlockNumber},
    dc_orchestrator_chain_interface::OrchestratorChainInterface,
    futures::FutureExt,
    node_common::{command::generate_genesis_block, service::NodeBuilderConfig},
    pallet_author_noting_runtime_api::AuthorNotingApi,
    pallet_registrar_runtime_api::RegistrarApi,
    polkadot_primitives::CollatorPair,
    sc_cli::{Database, SyncMode},
    sc_network::config::MultiaddrWithPeerId,
    sc_service::SpawnTaskHandle,
    sp_api::{ApiExt, ProvideRuntimeApi},
    sp_keystore::KeystorePtr,
    sp_runtime::traits::Block as BlockT,
    std::{
        collections::{HashMap, HashSet},
        future::Future,
        path::Path,
        pin::Pin,
        sync::{Arc, Mutex},
        time::Instant,
    },
    tokio::{
        sync::{mpsc, oneshot},
        time::{sleep, Duration},
    },
    tokio_util::sync::CancellationToken,
};

/// Struct with all the params needed to start a container chain node given the CLI arguments,
/// and creating the ChainSpec from on-chain data from the orchestrator chain.

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
    pub sync_keystore: KeystorePtr,
    pub orchestrator_para_id: ParaId,
    pub validator: bool,
    pub spawn_handle: SpawnTaskHandle,

    // State
    pub state: Arc<Mutex<ContainerChainSpawnerState>>,

    // Async callback that enables collation on the orchestrator chain
    pub collate_on_tanssi:
        Arc<dyn Fn() -> (CancellationToken, futures::channel::oneshot::Receiver<()>) + Send + Sync>,
    // Stores the cancellation token used to stop the orchestrator chain collator process.
    // When this is None, the orchestrator collator is not running.
    pub collation_cancellation_constructs:
        Option<(CancellationToken, futures::channel::oneshot::Receiver<()>)>,
}

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

impl ContainerChainSpawner {
    /// Try to start a new container chain. In case of an error, this does not stop the node, and
    /// the container chain will be attempted to spawn again when the collator is reassigned to it.
    #[must_use]
    fn spawn(
        &self,
        container_chain_para_id: ParaId,
        start_collation: bool,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let (
            orchestrator_chain_interface,
            orchestrator_client,
            mut container_chain_cli,
            tokio_handle,
            chain_type,
            relay_chain,
            relay_chain_interface,
            collator_key,
            sync_keystore,
            orchestrator_para_id,
            validator,
            spawn_handle,
            state,
        ) = (
            self.orchestrator_chain_interface.clone(),
            self.orchestrator_client.clone(),
            self.container_chain_cli.clone(),
            self.tokio_handle.clone(),
            self.chain_type.clone(),
            self.relay_chain.clone(),
            self.relay_chain_interface.clone(),
            self.collator_key.clone(),
            self.sync_keystore.clone(),
            self.orchestrator_para_id,
            self.validator,
            self.spawn_handle.clone(),
            self.state.clone(),
        );
        let state2 = state.clone();
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
                .genesis_data(orchestrator_chain_info.best_hash, container_chain_para_id)
                .map_err(|e| format!("Failed to call genesis_data runtime api: {}", e))?
                .ok_or_else(|| {
                    format!(
                        "No genesis data registered for container chain id {}",
                        container_chain_para_id
                    )
                })?;

            let boot_nodes_raw = orchestrator_runtime_api
                .boot_nodes(orchestrator_chain_info.best_hash, container_chain_para_id)
                .map_err(|e| format!("Failed to call boot_nodes runtime api: {}", e))?;
            if boot_nodes_raw.is_empty() {
                log::warn!(
                    "No boot nodes registered on-chain for container chain {}",
                    container_chain_para_id
                );
            }
            let boot_nodes =
                parse_boot_nodes_ignore_invalid(boot_nodes_raw, container_chain_para_id);
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
                .map_err(|e| format!("failed to create container chain chain spec from on chain genesis data: {}", e))?;

            log::info!(
                "Loaded chain spec for container chain {}",
                container_chain_para_id
            );

            if !start_collation {
                log::info!("This is a syncing container chain, using random ports");
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

            // Update CLI params
            container_chain_cli.base.para_id = Some(container_chain_para_id.into());
            container_chain_cli
                .base
                .base
                .import_params
                .database_params
                .database = Some(Database::ParityDb);

            let create_container_chain_cli_config = || {
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

                sc_service::error::Result::Ok((container_chain_cli_config, db_path))
            };

            let (_container_chain_cli_config, db_path) = create_container_chain_cli_config()?;
            let db_exists = db_path.exists();
            let db_exists_but_may_need_removal = db_exists && validator;
            if db_exists_but_may_need_removal {
                // If the database exists it may be invalid (genesis hash mismatch), so check if it is valid
                // and if not, delete it.
                // Create a new cli config because otherwise the tasks spawned in `open_and_maybe_delete_db` don't stop
                let (container_chain_cli_config, db_path) = create_container_chain_cli_config()?;
                open_and_maybe_delete_db(
                    container_chain_cli_config,
                    &db_path,
                    &orchestrator_client,
                    container_chain_para_id,
                    &container_chain_cli,
                    container_chain_cli.base.keep_db,
                )?;
                // Need to add a sleep here to ensure that the partial components created in
                // `open_and_maybe_delete_db` have enough time to close.
                log::info!("Restarting container chain {}", container_chain_para_id);
                sleep(Duration::from_secs(10)).await;
            }

            // Select appropiate sync mode. We want to use WarpSync unless the db still exists,
            // or the block number is 0 (because of a warp sync bug in that case).
            let db_still_exists = db_path.exists();
            container_chain_cli.base.base.network_params.sync = select_sync_mode(
                db_still_exists,
                &orchestrator_client,
                container_chain_para_id,
            )?;
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
            container_chain_cli_config.database.set_path(&db_path);

            // Start container chain node
            let (mut container_chain_task_manager, container_chain_client, container_chain_db) =
                start_node_impl_container(
                    container_chain_cli_config,
                    orchestrator_client.clone(),
                    relay_chain_interface.clone(),
                    orchestrator_chain_interface.clone(),
                    collator_key.clone(),
                    sync_keystore.clone(),
                    container_chain_para_id,
                    orchestrator_para_id,
                    validator && start_collation,
                )
                .await?;

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

                state.spawned_container_chains.insert(
                    container_chain_para_id,
                    ContainerChainState {
                        stop_handle: StopContainerChain {
                            signal,
                            id: monitor_id,
                        },
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
                        if validator && stop_unassigned == Ok(false) && !container_chain_cli.base.keep_db {
                            delete_container_chain_db(&db_path);
                        }
                    }
                }

                let mut state = state.lock().expect("poison error");
                state
                    .spawned_containers_monitor
                    .set_stop_task_manager_time(monitor_id, Instant::now());
            });

            sc_service::error::Result::Ok(())
        };

        async move {
            match try_closure().await {
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
        .boxed()
    }

    /// Stop a container chain. Prints a warning if the container chain was not running.
    fn stop(&self, container_chain_para_id: ParaId, keep_db: bool) {
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
            }
        }
    }

    /// Receive and process `CcSpawnMsg`s indefinitely
    pub async fn rx_loop(mut self, mut rx: mpsc::UnboundedReceiver<CcSpawnMsg>, validator: bool) {
        // The node always starts as an orchestrator chain collator.
        // This is because the assignment is detected after importing a new block, so if all
        // collators stop at the same time, when they start again nobody will produce the new block.
        // So all nodes start as orchestrator chain collators, until the first block is imported,
        // then the real assignment is used.
        if validator {
            self.handle_update_assignment(Some(self.orchestrator_para_id), None)
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
            need_to_restart,
        } = handle_update_assignment_state_change(
            &mut self.state.lock().expect("poison error"),
            self.orchestrator_para_id,
            current,
            next,
        );

        if current != Some(self.orchestrator_para_id) {
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
        for para_id in chains_to_stop {
            // Keep db if we are currently assigned to this chain
            let keep_db = Some(para_id) == current;
            self.stop(para_id, keep_db);
        }

        if need_to_restart {
            // Give it some time to stop properly
            sleep(Duration::from_secs(10)).await;
        }

        // Start all new container chains (usually 1)
        for para_id in chains_to_start {
            // Edge case: when starting the node it may be assigned to a container chain, so we need to
            // start a container chain already collating.
            let start_collation = Some(para_id) == current;
            self.spawn(para_id, start_collation).await;
        }
    }
}

struct HandleUpdateAssignmentResult {
    chains_to_stop: Vec<ParaId>,
    chains_to_start: Vec<ParaId>,
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

/// Select `SyncMode` to use for a container chain.
/// We want to use warp sync unless the db still exists, or the block number is 0 (because of a warp sync bug in that case).
/// The reason is that warp sync doesn't work if a database already exists, it falls back to full sync instead.
fn select_sync_mode(
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

    // Force container chains to use warp sync, unless full sync is needed for some reason
    let full_sync_needed = if !orchestrator_runtime_api
        .has_api::<dyn AuthorNotingApi<Block, AccountId, BlockNumber, ParaId>>(
            orchestrator_chain_info.best_hash,
        )
        .map_err(|e| format!("Failed to check if runtime has AuthorNotingApi: {}", e))?
    {
        // Before runtime API was implemented we don't know if the container chain has any blocks,
        // so use full sync because that always works
        true
    } else {
        // If the container chain is still at genesis block, use full sync because warp sync is broken
        orchestrator_runtime_api
            .latest_author(orchestrator_chain_info.best_hash, container_chain_para_id)
            .map_err(|e| format!("Failed to read latest author: {}", e))?
            .is_none()
    };

    if full_sync_needed {
        Ok(SyncMode::Full)
    } else {
        Ok(SyncMode::Warp)
    }
}

/// Start a container chain using `new_partial` and check if the database is valid. If not, delete the db.
/// The caller may need to wait a few seconds before trying to start the same container chain again, to
/// give the database enough time to close.
// TODO: instead of waiting, we could also return Weak references to the components `temp_cli.backend`
// and `temp_cli.client`, and then the caller would only need to check if the reference counts are 0.
fn open_and_maybe_delete_db(
    container_chain_cli_config: sc_service::Configuration,
    db_path: &Path,
    orchestrator_client: &Arc<ParachainClient>,
    container_chain_para_id: ParaId,
    container_chain_cli: &ContainerChainCli,
    keep_db: bool,
) -> sc_service::error::Result<()> {
    let temp_cli = NodeConfig::new_builder(&container_chain_cli_config, None)?;

    // Check block diff, only needed if keep-db is false
    if !keep_db {
        // Get latest block number from the container chain client
        let last_container_block_temp = temp_cli.client.chain_info().best_number;

        let orchestrator_runtime_api = orchestrator_client.runtime_api();
        let orchestrator_chain_info = orchestrator_client.chain_info();
        // Get the container chain's latest block from orchestrator chain and compare with client's one
        let last_container_block_from_orchestrator = orchestrator_runtime_api
            .latest_block_number(orchestrator_chain_info.best_hash, container_chain_para_id)
            .unwrap_or_default();

        let max_block_diff_allowed = 100u32;
        if last_container_block_from_orchestrator
            .unwrap_or(0u32)
            .abs_diff(last_container_block_temp)
            > max_block_diff_allowed
        {
            // if the diff is big, delete db and restart using warp sync
            delete_container_chain_db(db_path);
            return Ok(());
        }
    }

    // Generate genesis hash to compare against container client's genesis hash
    let container_preloaded_genesis = container_chain_cli.preloaded_chain_spec.as_ref().unwrap();

    // Check with both state versions
    let block_v0: Block =
        generate_genesis_block(&**container_preloaded_genesis, sp_runtime::StateVersion::V0)
            .map_err(|e| format!("{:?}", e))?;
    let chain_spec_genesis_hash_v0 = block_v0.header().hash();

    let block_v1: Block =
        generate_genesis_block(&**container_preloaded_genesis, sp_runtime::StateVersion::V1)
            .map_err(|e| format!("{:?}", e))?;
    let chain_spec_genesis_hash_v1 = block_v1.header().hash();

    let container_client_genesis_hash = temp_cli.client.chain_info().genesis_hash;

    if container_client_genesis_hash != chain_spec_genesis_hash_v0
        && container_client_genesis_hash != chain_spec_genesis_hash_v1
    {
        log::info!("Container genesis V0: {:?}", chain_spec_genesis_hash_v0);
        log::info!("Container genesis V1: {:?}", chain_spec_genesis_hash_v1);
        log::info!(
            "Chain spec genesis {:?} did not match with any container genesis - Restarting...",
            container_client_genesis_hash
        );
        delete_container_chain_db(db_path);
        return Ok(());
    }

    Ok(())
}

// TODO: this leaves some empty folders behind, because it is called with db_path:
//     Collator2002-01/data/containers/chains/simple_container_2002/paritydb/full-container-2002
// but we want to delete everything under
//     Collator2002-01/data/containers/chains/simple_container_2002
fn delete_container_chain_db(db_path: &Path) {
    if db_path.exists() {
        std::fs::remove_dir_all(db_path).expect("failed to remove old container chain db");
    }
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

#[cfg(test)]
mod tests {
    use super::*;

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

            let old = self
                .state
                .lock()
                .expect("poison error")
                .spawned_container_chains
                .insert(
                    container_chain_para_id,
                    ContainerChainState {
                        stop_handle: StopContainerChain { signal, id: 0 },
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
}
