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
    super::*,
    async_trait::async_trait,
    sc_client_api::{StorageKey, StorageProvider},
    sp_runtime::traits::HashFor,
    sp_state_machine::prove_read,
    sp_trie::{HashDBT, EMPTY_PREFIX},
    substrate_test_runtime::Block,
    substrate_test_runtime_client::{
        ClientExt, DefaultTestClientBuilderExt, TestClient, TestClientBuilder, TestClientBuilderExt,
    },
};

#[derive(Clone)]
struct DummyOrchestratorChainInterface {
    orchestrator_client: Arc<TestClient>,
}
const KEY: &[u8] = b":mock";
const VALUE: &[u8] = b"hello world";

impl DummyOrchestratorChainInterface {
    fn new() -> Self {
        let builder = TestClientBuilder::new().add_extra_storage(KEY.to_vec(), VALUE.to_vec());

        Self {
            orchestrator_client: Arc::new(builder.build()),
        }
    }
}

#[async_trait]
impl OrchestratorChainInterface for DummyOrchestratorChainInterface {
    fn overseer_handle(&self) -> OrchestratorChainResult<Handle> {
        unimplemented!("Not needed for test")
    }

    async fn get_storage_by_key(
        &self,
        hash: PHash,
        key: &[u8],
    ) -> OrchestratorChainResult<Option<StorageValue>> {
        self.orchestrator_client
            .storage(hash.into(), &StorageKey(key.clone().to_vec()))
            .map(|a| a.map(|b| b.0))
            .map_err(|e| e.into())
    }

    async fn prove_read(
        &self,
        hash: PHash,
        keys: &[Vec<u8>],
    ) -> OrchestratorChainResult<sc_client_api::StorageProof> {
        self.orchestrator_client
            .state_at(hash)
            .map(|state| prove_read(state, keys))
            .unwrap()
            .map_err(|e| e.into())
    }
}

#[tokio::test]
async fn test_get_storage() {
    let orchestrator_chain_interface = Arc::new(DummyOrchestratorChainInterface::new());
    let genesis_hash = orchestrator_chain_interface
        .orchestrator_client
        .genesis_hash();
    assert_eq!(
        (orchestrator_chain_interface
            .get_storage_by_key(genesis_hash, KEY)
            .await)
            .unwrap(),
        Some(VALUE.to_vec())
    )
}

#[tokio::test]
async fn test_prove_read() {
    let orchestrator_chain_interface = Arc::new(DummyOrchestratorChainInterface::new());
    let genesis_hash = orchestrator_chain_interface
        .orchestrator_client
        .genesis_hash();
    let storage_proof = (orchestrator_chain_interface
        .prove_read(genesis_hash, &[KEY.to_vec()])
        .await)
        .unwrap();

    let header = orchestrator_chain_interface
        .orchestrator_client
        .header(genesis_hash)
        .unwrap()
        .unwrap();
    let db = storage_proof.into_memory_db::<HashFor<Block>>();
    assert!(db.contains(&header.state_root, EMPTY_PREFIX))
}

#[tokio::test]
#[should_panic]
async fn test_overseer() {
    let orchestrator_chain_interface = Arc::new(DummyOrchestratorChainInterface::new());

    let _ = orchestrator_chain_interface.overseer_handle();
}
