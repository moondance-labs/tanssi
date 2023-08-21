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
    crate::{
        cli::ContainerChainCli,
        service::{start_node_impl_container, ParachainClient},
    },
    cumulus_primitives_core::ParaId,
    cumulus_relay_chain_interface::RelayChainInterface,
    dancebox_runtime::{AccountId, Block, BlockNumber},
    futures::FutureExt,
    pallet_author_noting_runtime_api::AuthorNotingApi,
    pallet_registrar_runtime_api::RegistrarApi,
    polkadot_primitives::CollatorPair,
    sc_cli::SyncMode,
    sc_service::SpawnTaskHandle,
    sp_api::{ApiExt, ProvideRuntimeApi},
    sp_keystore::KeystorePtr,
    std::{
        collections::{HashMap, HashSet},
        future::Future,
        path::Path,
        pin::Pin,
        sync::{Arc, Mutex},
    },
    tc_orchestrator_chain_interface::OrchestratorChainInterface,
    tokio::sync::mpsc::UnboundedReceiver,
};

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
    pub sync_keystore: KeystorePtr,
    pub orchestrator_para_id: ParaId,
    pub validator: bool,
    pub spawn_handle: SpawnTaskHandle,

    // State
    pub state: Arc<Mutex<ContainerChainSpawnerState>>,

    // Async callback that enables collation on the orchestrator chain
    pub collate_on_tanssi: Arc<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>,
}

#[derive(Default)]
pub struct ContainerChainSpawnerState {
    spawned_container_chains: HashMap<ParaId, ContainerChainState>,
    assigned_para_id: Option<ParaId>,
    next_assigned_para_id: Option<ParaId>,
}

pub struct ContainerChainState {
    /// Async callback that enables collation on the orchestrator chain
    collate_on: Arc<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>,
    /// Handle that stops the container chain when dropped
    #[allow(dead_code)]
    stop_handle: StopContainerChain,
}

/// Stops a container chain when dropped
pub struct StopContainerChain(exit_future::Signal);

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
    /// Try to start a new container chain. In case of error, this panics and stops the node.
    fn spawn(
        &self,
        container_chain_para_id: ParaId,
        start_collation: bool,
    ) -> impl Future<Output = ()> {
        let ContainerChainSpawner {
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
            collate_on_tanssi: _,
        } = self.clone();

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
                .expect("error")
                .ok_or_else(|| {
                    format!(
                        "No genesis data registered for container chain id {}",
                        container_chain_para_id
                    )
                })?;

            let boot_nodes_raw = orchestrator_runtime_api
                .boot_nodes(orchestrator_chain_info.best_hash, container_chain_para_id)
                .expect("error");
            let boot_nodes: Vec<String> = boot_nodes_raw
                .into_iter()
                .map(|x| String::from_utf8(x).map_err(|e| format!("{}", e)))
                .collect::<Result<_, _>>()?;

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

            // Update CLI params
            container_chain_cli.base.para_id = Some(container_chain_para_id.into());

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
                container_chain_cli.base.base.network_params.sync = SyncMode::Full;
            } else {
                container_chain_cli.base.base.network_params.sync = SyncMode::Warp;
            }

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

            // Delete existing database if running as collator
            if validator {
                delete_container_chain_db(&db_path);
            }

            // Start container chain node
            let (mut container_chain_task_manager, _container_chain_client, collate_on) =
                start_node_impl_container(
                    container_chain_cli_config,
                    orchestrator_client.clone(),
                    relay_chain_interface.clone(),
                    orchestrator_chain_interface.clone(),
                    collator_key.clone(),
                    sync_keystore.clone(),
                    container_chain_para_id,
                    orchestrator_para_id,
                    validator,
                )
                .await?;

            // Signal that allows to gracefully stop a container chain
            let (signal, on_exit) = exit_future::signal();
            let collate_on = collate_on.unwrap_or_else(|| {
                assert!(
                    !validator,
                    "collate_on should be Some if validator flag is true"
                );

                // When running a full node we don't need to send any collate_on messages, so make this a noop
                Arc::new(move || Box::pin(std::future::ready(())))
            });

            state
                .lock()
                .expect("poison error")
                .spawned_container_chains
                .insert(
                    container_chain_para_id,
                    ContainerChainState {
                        collate_on: collate_on.clone(),
                        stop_handle: StopContainerChain(signal),
                    },
                );

            if start_collation {
                collate_on().await;
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
                        // Delete existing database if running as collator
                        if validator {
                            delete_container_chain_db(&db_path);
                        }
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
                CcSpawnMsg::UpdateAssignment { current, next } => {
                    self.handle_update_assignment(current, next).await;
                }
            }
        }

        // The while loop can end if all the senders get dropped, but since this is an
        // essential task we don't want it to stop. So await a future that never completes.
        // This should only happen when starting a full node.
        std::future::pending().await
    }

    /// Handle `CcSpawnMsg::UpdateAssignment`
    async fn handle_update_assignment(&self, current: Option<ParaId>, next: Option<ParaId>) {
        let HandleUpdateAssignmentResult {
            call_collate_on,
            chains_to_stop,
            chains_to_start,
        } = handle_update_assignment_state_change(
            &mut self.state.lock().expect("poison error"),
            self.orchestrator_para_id,
            self.collate_on_tanssi.clone(),
            current,
            next,
        );

        // Call collate_on, to start collation on a chain that was already running before
        if let Some(f) = call_collate_on {
            f().await;
        }

        // Stop all container chains that are no longer needed
        for para_id in chains_to_stop {
            self.stop(para_id);
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
    call_collate_on:
        Option<Arc<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>>,
    chains_to_stop: Vec<ParaId>,
    chains_to_start: Vec<ParaId>,
}

// This is a separate function to allow testing
fn handle_update_assignment_state_change(
    state: &mut ContainerChainSpawnerState,
    orchestrator_para_id: ParaId,
    collate_on_tanssi: Arc<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>,
    current: Option<ParaId>,
    next: Option<ParaId>,
) -> HandleUpdateAssignmentResult {
    if (state.assigned_para_id, state.next_assigned_para_id) == (current, next) {
        // If nothing changed there is nothing to update
        return HandleUpdateAssignmentResult {
            call_collate_on: None,
            chains_to_stop: Default::default(),
            chains_to_start: Default::default(),
        };
    }

    // Create a set with the container chains that were running before, and the container
    // chains that should be running after the updated assignment. This is used to calculate
    // the difference, and stop and start the required container chains.
    let mut running_chains_before = HashSet::new();
    let mut running_chains_after = HashSet::new();
    let mut call_collate_on = None;

    running_chains_before.extend(state.assigned_para_id);
    running_chains_before.extend(state.next_assigned_para_id);
    // Ignore orchestrator_para_id because it cannot be stopped or started, it is always running
    running_chains_before.remove(&orchestrator_para_id);

    running_chains_after.extend(current);
    running_chains_after.extend(next);
    running_chains_after.remove(&orchestrator_para_id);

    if state.assigned_para_id != current {
        // If the assigned container chain was already running but not collating, we need to call collate_on
        if let Some(para_id) = current {
            // Check if we get assigned to orchestrator chain
            if para_id == orchestrator_para_id {
                call_collate_on = Some(collate_on_tanssi);
            } else {
                // When we get assigned to a different container chain, only need to call collate_on if it was already
                // running before
                if running_chains_before.contains(&para_id) {
                    let c = state.spawned_container_chains.get(&para_id).expect("container chain was running before so it should exist in spawned_container_chains");
                    call_collate_on = Some(c.collate_on.clone());
                }
            }
        }
    }

    state.assigned_para_id = current;
    state.next_assigned_para_id = next;

    let chains_to_stop = running_chains_before
        .difference(&running_chains_after)
        .copied()
        .collect();
    let chains_to_start = running_chains_after
        .difference(&running_chains_before)
        .copied()
        .collect();

    HandleUpdateAssignmentResult {
        call_collate_on,
        chains_to_stop,
        chains_to_start,
    }
}

// TODO: this leaves some empty folders behind, because it is called with db_path:
//     Collator2002-01/data/containers/chains/simple_container_2002/db/full-container-2002
// but we want to delete everything under
//     Collator2002-01/data/containers/chains/simple_container_2002
fn delete_container_chain_db(db_path: &Path) {
    if db_path.exists() {
        std::fs::remove_dir_all(&db_path).expect("failed to remove old container chain db");
    }
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;

    use super::*;

    // Copy of ContainerChainSpawner with extra assertions for tests, and mocked spawn function.
    struct MockContainerChainSpawner {
        state: Arc<Mutex<ContainerChainSpawnerState>>,
        orchestrator_para_id: ParaId,
        collate_on_tanssi: Arc<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>,
        // Keep track of the last CollateOn message, for tests
        currently_collating_on: Arc<Mutex<Option<ParaId>>>,
    }

    impl MockContainerChainSpawner {
        fn new() -> Self {
            let orchestrator_para_id = 1000.into();
            // The node always starts as an orchestrator chain collator
            let currently_collating_on = Arc::new(Mutex::new(Some(orchestrator_para_id)));
            let currently_collating_on2 = currently_collating_on.clone();
            let collate_closure = move || async move {
                let mut cco = currently_collating_on2.lock().unwrap();
                // TODO: this sometimes fails, see comment in stop_collating_orchestrator
                /*
                assert_ne!(
                    *cco,
                    Some(orchestrator_para_id),
                    "Received CollateOn message when we were already collating on this chain: {}",
                    orchestrator_para_id
                );
                */
                *cco = Some(orchestrator_para_id);
            };
            let collate_on_tanssi: Arc<
                dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync,
            > = Arc::new(move || Box::pin((collate_closure.clone())()));

            Self {
                state: Arc::new(Mutex::new(ContainerChainSpawnerState {
                    spawned_container_chains: Default::default(),
                    assigned_para_id: None,
                    next_assigned_para_id: None,
                })),
                orchestrator_para_id,
                collate_on_tanssi,
                currently_collating_on,
            }
        }

        async fn spawn(&self, container_chain_para_id: ParaId, start_collation: bool) {
            let (signal, _on_exit) = exit_future::signal();
            let currently_collating_on2 = self.currently_collating_on.clone();
            let collate_closure = move || async move {
                let mut cco = currently_collating_on2.lock().unwrap();
                // TODO: this is also wrong, see comment in test keep_collating_on_container
                /*
                assert_ne!(
                    *cco,
                    Some(container_chain_para_id),
                    "Received CollateOn message when we were already collating on this chain: {}",
                    container_chain_para_id
                );
                */
                *cco = Some(container_chain_para_id);
            };
            let collate_on: Arc<
                dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync,
            > = Arc::new(move || Box::pin((collate_closure.clone())()));

            let old = self
                .state
                .lock()
                .expect("poison error")
                .spawned_container_chains
                .insert(
                    container_chain_para_id,
                    ContainerChainState {
                        collate_on: collate_on.clone(),
                        stop_handle: StopContainerChain(signal),
                    },
                );

            assert!(
                old.is_none(),
                "tried to spawn a container chain that was already running: {}",
                container_chain_para_id
            );

            if start_collation {
                collate_on().await;
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

        fn handle_update_assignment(&self, current: Option<ParaId>, next: Option<ParaId>) {
            let HandleUpdateAssignmentResult {
                call_collate_on,
                chains_to_stop,
                chains_to_start,
            } = handle_update_assignment_state_change(
                &mut *self.state.lock().unwrap(),
                self.orchestrator_para_id,
                self.collate_on_tanssi.clone(),
                current,
                next,
            );

            // Assert we never start and stop the same container chain
            for para_id in &chains_to_start {
                assert!(
                    !chains_to_stop.contains(para_id),
                    "Tried to start and stop same container chain: {}",
                    para_id
                );
            }
            // Assert we never start or stop the orchestrator chain
            assert!(!chains_to_start.contains(&self.orchestrator_para_id));
            assert!(!chains_to_stop.contains(&self.orchestrator_para_id));

            // Call collate_on, to start collation on a chain that was already running before
            if let Some(f) = call_collate_on {
                block_on(async { f().await });
            }

            // Stop all container chains that are no longer needed
            for para_id in chains_to_stop {
                self.stop(para_id);
            }

            // Start all new container chains (usually 1)
            for para_id in chains_to_start {
                // Edge case: when starting the node it may be assigned to a container chain, so we need to
                // start a container chain already collating.
                let start_collation = Some(para_id) == current;
                block_on(async { self.spawn(para_id, start_collation).await });
            }

            // Assert that if we are currently assigned to a container chain, we are collating there
            if let Some(para_id) = current {
                self.assert_collating_on(Some(para_id));
            } else {
                // If we are not assigned anywhere we may be collating on the orchestrator chain,
                // or we may not be collating anywhere, or we may be collating on a container chain that is currently in "next"
                let currently_collating_on = *self.currently_collating_on.lock().unwrap();
                assert!(
                    currently_collating_on.is_none()
                        || currently_collating_on == Some(self.orchestrator_para_id)
                        || currently_collating_on == Some(next.unwrap())
                );
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
        let m = MockContainerChainSpawner::new();
        m.assert_collating_on(Some(1000.into()));
        m.assert_running_chains(&[]);

        m.handle_update_assignment(None, None);
        m.assert_collating_on(Some(1000.into()));
        m.assert_running_chains(&[]);
    }

    #[test]
    fn assigned_to_orchestrator_chain() {
        let m = MockContainerChainSpawner::new();

        m.handle_update_assignment(Some(1000.into()), Some(1000.into()));
        m.assert_collating_on(Some(1000.into()));
        m.assert_running_chains(&[]);

        m.handle_update_assignment(Some(1000.into()), None);
        m.assert_collating_on(Some(1000.into()));
        m.assert_running_chains(&[]);

        m.handle_update_assignment(None, None);
        m.assert_collating_on(Some(1000.into()));
        m.assert_running_chains(&[]);

        m.handle_update_assignment(None, Some(1000.into()));
        m.assert_collating_on(Some(1000.into()));
        m.assert_running_chains(&[]);

        m.handle_update_assignment(Some(1000.into()), Some(1000.into()));
        m.assert_collating_on(Some(1000.into()));
        m.assert_running_chains(&[]);
    }

    #[test]
    fn assigned_to_container_chain() {
        let m = MockContainerChainSpawner::new();

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
        let m = MockContainerChainSpawner::new();

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
        let m: MockContainerChainSpawner = MockContainerChainSpawner::new();

        m.handle_update_assignment(Some(2000.into()), Some(2001.into()));
        m.assert_collating_on(Some(2000.into()));
        m.assert_running_chains(&[2000.into(), 2001.into()]);

        m.handle_update_assignment(Some(2001.into()), Some(2000.into()));
        m.assert_collating_on(Some(2001.into()));
        m.assert_running_chains(&[2000.into(), 2001.into()]);
    }

    #[test]
    fn stop_collating_orchestrator() {
        let m: MockContainerChainSpawner = MockContainerChainSpawner::new();

        m.handle_update_assignment(Some(1000.into()), Some(1000.into()));
        m.assert_collating_on(Some(1000.into()));
        m.assert_running_chains(&[]);

        m.handle_update_assignment(Some(1000.into()), None);
        m.assert_collating_on(Some(1000.into()));
        m.assert_running_chains(&[]);

        m.handle_update_assignment(None, None);
        m.assert_collating_on(Some(1000.into()));
        m.assert_running_chains(&[]);

        // TODO: this will send an unneeded CollateOn message, because the ContainerChainSpawner
        // doesn't remember that the last message has been sent to the orchestrator chain,
        // which is always running, so it is still collating.
        m.handle_update_assignment(Some(1000.into()), None);
        m.assert_collating_on(Some(1000.into()));
        m.assert_running_chains(&[]);
    }

    #[test]
    fn stop_collating_container() {
        let m: MockContainerChainSpawner = MockContainerChainSpawner::new();

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
        let m: MockContainerChainSpawner = MockContainerChainSpawner::new();

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
        let m: MockContainerChainSpawner = MockContainerChainSpawner::new();

        m.handle_update_assignment(Some(2000.into()), Some(2001.into()));
        m.assert_collating_on(Some(2000.into()));
        m.assert_running_chains(&[2000.into(), 2001.into()]);

        m.handle_update_assignment(None, None);
        m.assert_collating_on(None);
        m.assert_running_chains(&[]);
    }

    #[test]
    fn keep_collating_on_container() {
        let m: MockContainerChainSpawner = MockContainerChainSpawner::new();

        m.handle_update_assignment(Some(2000.into()), None);
        m.assert_collating_on(Some(2000.into()));
        m.assert_running_chains(&[2000.into()]);

        m.handle_update_assignment(None, Some(2000.into()));
        m.assert_collating_on(Some(2000.into()));
        m.assert_running_chains(&[2000.into()]);

        // TODO: this will send an unneeded CollateOn message, because the ContainerChainSpawner
        // doesn't remember that the last message has been sent to this chain,
        // which is still running, so it is still collating.
        m.handle_update_assignment(Some(2000.into()), Some(2000.into()));
        m.assert_collating_on(Some(2000.into()));
        m.assert_running_chains(&[2000.into()]);
    }
}
