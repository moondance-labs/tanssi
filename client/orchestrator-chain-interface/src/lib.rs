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

//! # Orchestrator chain interface client primitives
//!
//! This file contains the OrchestratorChainInterface trait which serves to generate
//! storage proofs to be provided to containerchains
//!
//! get_storage_by_key: retrieves a storage item from the Orchestrator interface at a given
//! Orchestrator parent
//!
//! prove_read: generates a storage proof of a given set of keys at a given Orchestrator parent

pub use cumulus_primitives_core::relay_chain::Hash as PHash;
use {
    polkadot_overseer::Handle, sc_client_api::StorageProof, sp_api::ApiError,
    sp_state_machine::StorageValue, std::sync::Arc,
};

#[derive(thiserror::Error, Debug)]
pub enum OrchestratorChainError {
    #[error("Blockchain returned an error: {0}")]
    BlockchainError(#[from] sp_blockchain::Error),
    #[error("State machine error occured: {0}")]
    StateMachineError(Box<dyn sp_state_machine::Error>),
    #[error(transparent)]
    Application(#[from] Box<dyn std::error::Error + Send + Sync + 'static>),
    #[error("Unspecified error occured: {0}")]
    GenericError(String),
}

impl From<OrchestratorChainError> for ApiError {
    fn from(r: OrchestratorChainError) -> Self {
        sp_api::ApiError::Application(Box::new(r))
    }
}

impl From<OrchestratorChainError> for sp_blockchain::Error {
    fn from(r: OrchestratorChainError) -> Self {
        sp_blockchain::Error::Application(Box::new(r))
    }
}

impl<T: std::error::Error + Send + Sync + 'static> From<Box<T>> for OrchestratorChainError {
    fn from(r: Box<T>) -> Self {
        OrchestratorChainError::Application(r)
    }
}

impl From<Box<dyn sp_state_machine::Error>> for OrchestratorChainError {
    fn from(r: Box<dyn sp_state_machine::Error>) -> Self {
        OrchestratorChainError::StateMachineError(r)
    }
}

// TODO: proper errors
pub type OrchestratorChainResult<T> = Result<T, OrchestratorChainError>;

/// Trait that provides all necessary methods for interaction between collator and orchestrator chain.
#[async_trait::async_trait]
pub trait OrchestratorChainInterface: Send + Sync {
    /// Fetch a storage item by key.
    async fn get_storage_by_key(
        &self,
        orchestrator_parent: PHash,
        key: &[u8],
    ) -> OrchestratorChainResult<Option<StorageValue>>;

    /// Get a handle to the overseer.
    fn overseer_handle(&self) -> OrchestratorChainResult<Handle>;

    /// Generate a storage read proof.
    async fn prove_read(
        &self,
        orchestrator_parent: PHash,
        relevant_keys: &[Vec<u8>],
    ) -> OrchestratorChainResult<StorageProof>;
}

#[async_trait::async_trait]
impl<T> OrchestratorChainInterface for Arc<T>
where
    T: OrchestratorChainInterface + ?Sized,
{
    fn overseer_handle(&self) -> OrchestratorChainResult<Handle> {
        (**self).overseer_handle()
    }

    async fn get_storage_by_key(
        &self,
        orchestrator_parent: PHash,
        key: &[u8],
    ) -> OrchestratorChainResult<Option<StorageValue>> {
        (**self).get_storage_by_key(orchestrator_parent, key).await
    }

    async fn prove_read(
        &self,
        orchestrator_parent: PHash,
        relevant_keys: &[Vec<u8>],
    ) -> OrchestratorChainResult<StorageProof> {
        (**self)
            .prove_read(orchestrator_parent, relevant_keys)
            .await
    }
}
