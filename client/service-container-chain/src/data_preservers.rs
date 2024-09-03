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
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>

use {
    crate::spawner::{ContainerChainSpawner, Spawner, TSelectSyncMode},
    dc_orchestrator_chain_interface::{
        DataPreserverAssignment, OrchestratorChainInterface, OrchestratorChainResult,
    },
    futures::stream::StreamExt,
    std::future::Future,
    tc_consensus::ParaId,
};

pub type ProfileId = <dancebox_runtime::Runtime as pallet_data_preservers::Config>::ProfileId;

async fn try_fut<T, E>(fut: impl Future<Output = Result<T, E>>) -> Result<T, E> {
    fut.await
}

/// Watch assignements by indefinitly listening to finalized block notifications and switching to
/// the chain the profile is assigned to.
pub async fn task_watch_assignment(spawner: impl Spawner, profile_id: ProfileId) {
    use dc_orchestrator_chain_interface::DataPreserverAssignment as Assignment;

    if let OrchestratorChainResult::Err(e) = try_fut(async move {
        let orchestrator_chain_interface = spawner.orchestrator_chain_interface();

        let mut current_assignment = DataPreserverAssignment::<ParaId>::NotAssigned;

        let mut stream = orchestrator_chain_interface
            .finality_notification_stream()
            .await?;

        while let Some(header) = stream.next().await {
            let hash = header.hash();

            let new_assignment = orchestrator_chain_interface
                .data_preserver_active_assignment(hash, profile_id)
                .await?;

            log::info!("Assignement for block {hash}: {new_assignment:?}");

            match (current_assignment, new_assignment) {
                // no change
                (x, y) if x == y => continue,
                // switch from not assigned/inactive to active, start embeded node
                (
                    Assignment::NotAssigned | Assignment::Inactive(_),
                    Assignment::Active(para_id),
                ) => {
                    spawner.spawn(para_id, false).await;
                }
                // Assignement switches from active to inactive for same para_id, we stop the
                // embeded node but keep db
                (Assignment::Active(para_id), Assignment::Inactive(x)) if para_id == x => {
                    spawner.stop(para_id, true); // keep db
                }
                // No longer assigned or assigned inactive to other para id, remove previous node
                (
                    Assignment::Active(para_id),
                    Assignment::Inactive(_) | Assignment::NotAssigned,
                ) => {
                    spawner.stop(para_id, false); // don't keep db
                }
                // Changed para id, remove previous node and start new one
                (Assignment::Active(previous_para_id), Assignment::Active(para_id)) => {
                    spawner.stop(previous_para_id, false); // don't keep db
                    spawner.spawn(para_id, false).await;
                }
                // don't do anything yet
                (
                    Assignment::NotAssigned | Assignment::Inactive(_),
                    Assignment::NotAssigned | Assignment::Inactive(_),
                ) => (),
            }

            current_assignment = new_assignment;
        }

        Ok(())
    })
    .await
    {
        log::error!("Error in data preservers assignement watching task: {e:?}");
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        dc_orchestrator_chain_interface::{
            BlockNumber, DataPreserverProfileId, OrchestratorChainError, PHash, PHeader,
        },
        dp_container_chain_genesis_data::ContainerChainGenesisData,
        futures::{FutureExt, Stream},
        polkadot_overseer::Handle,
        sc_client_api::StorageProof,
        sp_core::H256,
        sp_runtime::offchain::storage::StorageValue,
        std::{
            collections::{BTreeMap, HashSet},
            path::PathBuf,
            pin::Pin,
            sync::{Arc, Mutex},
        },
        tokio::sync::{broadcast, oneshot},
    };

    struct MockChainInterface {
        state: Mutex<MockChainInterfaceState>,
        notification_sender: broadcast::Sender<PHeader>,
    }

    struct MockChainInterfaceState {
        next_block_number: BlockNumber,
        blocks: BTreeMap<H256, BlockAssignment>,
    }

    struct BlockAssignment {
        assignments: BTreeMap<ProfileId, DataPreserverAssignment<ParaId>>,
    }

    impl MockChainInterface {
        fn new() -> Self {
            Self {
                state: Mutex::new(MockChainInterfaceState {
                    next_block_number: 0,
                    blocks: BTreeMap::new(),
                }),

                notification_sender: broadcast::Sender::new(100),
            }
        }

        fn mock_block(&self, assignments: BTreeMap<ProfileId, DataPreserverAssignment<ParaId>>) {
            let mut state = self.state.lock().unwrap();
            state.next_block_number += 1;

            let header = PHeader {
                parent_hash: H256::zero(),
                number: state.next_block_number,
                state_root: H256::zero(),
                extrinsics_root: H256::zero(),
                digest: Default::default(),
            };
            let hash = header.hash();

            state.blocks.insert(hash, BlockAssignment { assignments });

            self.notification_sender
                .send(header)
                .expect("to properly send block header");
        }
    }

    #[async_trait::async_trait]
    impl OrchestratorChainInterface for MockChainInterface {
        fn overseer_handle(&self) -> OrchestratorChainResult<Handle> {
            unimplemented!("not used in test")
        }

        async fn get_storage_by_key(
            &self,
            _orchestrator_parent: PHash,
            _key: &[u8],
        ) -> OrchestratorChainResult<Option<Vec<u8>>> {
            unimplemented!("not used in test")
        }

        async fn prove_read(
            &self,
            _orchestrator_parent: PHash,
            _relevant_keys: &Vec<Vec<u8>>,
        ) -> OrchestratorChainResult<StorageProof> {
            unimplemented!("not used in test")
        }

        async fn import_notification_stream(
            &self,
        ) -> OrchestratorChainResult<Pin<Box<dyn Stream<Item = PHeader> + Send>>> {
            unimplemented!("not used in test")
        }

        async fn new_best_notification_stream(
            &self,
        ) -> OrchestratorChainResult<Pin<Box<dyn Stream<Item = PHeader> + Send>>> {
            unimplemented!("not used in test")
        }

        async fn finality_notification_stream(
            &self,
        ) -> OrchestratorChainResult<Pin<Box<dyn Stream<Item = PHeader> + Send>>> {
            let receiver = self.notification_sender.subscribe();
            let stream = tokio_stream::wrappers::BroadcastStream::new(receiver)
                .filter_map(|x| async { x.ok() });
            let stream = Box::pin(stream);
            Ok(stream)
        }

        async fn genesis_data(
            &self,
            _orchestrator_parent: PHash,
            _para_id: ParaId,
        ) -> OrchestratorChainResult<Option<ContainerChainGenesisData>> {
            unimplemented!("not used in test")
        }

        async fn boot_nodes(
            &self,
            _orchestrator_parent: PHash,
            _para_id: ParaId,
        ) -> OrchestratorChainResult<Vec<Vec<u8>>> {
            unimplemented!("not used in test")
        }

        async fn latest_block_number(
            &self,
            _orchestrator_parent: PHash,
            _para_id: ParaId,
        ) -> OrchestratorChainResult<Option<BlockNumber>> {
            unimplemented!("not used in test")
        }

        async fn best_block_hash(&self) -> OrchestratorChainResult<PHash> {
            unimplemented!("not used in test")
        }

        async fn finalized_block_hash(&self) -> OrchestratorChainResult<PHash> {
            unimplemented!("not used in test")
        }

        async fn data_preserver_active_assignment(
            &self,
            orchestrator_parent: PHash,
            profile_id: DataPreserverProfileId,
        ) -> OrchestratorChainResult<DataPreserverAssignment<ParaId>> {
            let mut state = self.state.lock().unwrap();
            let block = state.blocks.get_mut(&orchestrator_parent).ok_or_else(|| {
                OrchestratorChainError::GenericError("this block is not mocked".into())
            })?;

            Ok(block
                .assignments
                .get(&profile_id)
                .cloned()
                .unwrap_or(DataPreserverAssignment::NotAssigned))
        }
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    enum SpawnerEvent {
        Started(ParaId, bool),
        Stopped(ParaId, bool),
    }

    #[derive(Clone)]
    struct MockSpawner {
        state: Arc<Mutex<HashSet<SpawnerEvent>>>,
        chain_interface: Arc<MockChainInterface>,
    }

    impl MockSpawner {
        fn new() -> Self {
            Self {
                state: Arc::new(Mutex::new(HashSet::new())),
                chain_interface: Arc::new(MockChainInterface::new()),
            }
        }

        fn set_expectations(&self, events: Vec<SpawnerEvent>) {
            let mut set = self.state.lock().unwrap();

            set.clear();

            for e in events {
                set.insert(e);
            }
        }

        fn ensure_all_events_were_emitted(&self) {
            let set = self.state.lock().unwrap();

            assert!(set.is_empty(), "Not all events were emitted: {set:?}");
        }
    }

    impl Spawner for MockSpawner {
        fn orchestrator_chain_interface(&self) -> Arc<dyn OrchestratorChainInterface> {
            self.chain_interface.clone()
        }

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
        ) -> impl std::future::Future<Output = ()> + Send {
            let mut set = self.state.lock().unwrap();

            let event = SpawnerEvent::Started(container_chain_para_id, start_collation);

            assert!(set.remove(&event), "Unexpected event {event:?}");

            async {}
        }

        /// Stop a container chain. Prints a warning if the container chain was not running.
        /// Returns the database path for the container chain, can be used with `wait_for_paritydb_lock`
        /// to ensure that the container chain has fully stopped. The database path can be `None` if the
        /// chain was not running.
        fn stop(&self, container_chain_para_id: ParaId, keep_db: bool) -> Option<PathBuf> {
            let mut set = self.state.lock().unwrap();

            let event = SpawnerEvent::Stopped(container_chain_para_id, keep_db);

            assert!(set.remove(&event), "Unexpected event {event:?}");

            None
        }
    }

    #[tokio::test]
    async fn task_logic_works() {
        let spawner = MockSpawner::new();

        let profile_id = 0;
        let para_id1 = ParaId::from(1);
        let para_id2 = ParaId::from(2);

        tokio::spawn(task_watch_assignment(spawner.clone(), profile_id));

        spawner.set_expectations(vec![SpawnerEvent::Started(para_id1, false)]);
        spawner.chain_interface.mock_block({
            let mut map = BTreeMap::new();
            map.insert(profile_id, DataPreserverAssignment::Active(para_id1));
            map
        });
        spawner.ensure_all_events_were_emitted();
    }
}
