//! # Tanssi chain interface client primitives
//!
//! This file contains the TanssiChainInterface trait which serves to generate
//! storage proofs to be provided to containerchains
//!
//! get_storage_by_key: retrieves a storage item from the tanssi interface at a given
//! tanssi parent
//!
//! prove_read: generates a storage proof of a given set of keys at a given tanssi parent
pub use cumulus_primitives_core::relay_chain::Hash as PHash;
use polkadot_overseer::Handle;
use sc_client_api::StorageProof;
use sp_api::ApiError;
use sp_state_machine::StorageValue;
use std::sync::Arc;

#[derive(thiserror::Error, Debug)]
pub enum TanssiChainError {
    #[error("Error occured while calling relay chain runtime: {0}")]
    ApiError(#[from] ApiError),
    #[error("Timeout while waiting for relay-chain block `{0}` to be imported.")]
    WaitTimeout(PHash),
    #[error("Import listener closed while waiting for relay-chain block `{0}` to be imported.")]
    ImportListenerClosed(PHash),
    #[error(
		"Blockchain returned an error while waiting for relay-chain block `{0}` to be imported: {1}"
	)]
    WaitBlockchainError(PHash, sp_blockchain::Error),
    #[error("Blockchain returned an error: {0}")]
    BlockchainError(#[from] sp_blockchain::Error),
    #[error("State machine error occured: {0}")]
    StateMachineError(Box<dyn sp_state_machine::Error>),
    #[error("Unable to call RPC method '{0}'")]
    RpcCallError(String),
    #[error("Unable to communicate with RPC worker: {0}")]
    WorkerCommunicationError(String),
    #[error(transparent)]
    Application(#[from] Box<dyn std::error::Error + Send + Sync + 'static>),
    #[error("Unspecified error occured: {0}")]
    GenericError(String),
}

impl From<TanssiChainError> for ApiError {
    fn from(r: TanssiChainError) -> Self {
        sp_api::ApiError::Application(Box::new(r))
    }
}

impl<T: std::error::Error + Send + Sync + 'static> From<Box<T>> for TanssiChainError {
    fn from(r: Box<T>) -> Self {
        TanssiChainError::Application(r)
    }
}

// TODO: proper errors
pub type TanssiChainResult<T> = Result<T, TanssiChainError>;

/// Trait that provides all necessary methods for interaction between collator and tanssi chain.
#[async_trait::async_trait]
pub trait TanssiChainInterface: Send + Sync {
    /// Fetch a storage item by key.
    async fn get_storage_by_key(
        &self,
        tanssi_parent: PHash,
        key: &[u8],
    ) -> TanssiChainResult<Option<StorageValue>>;

    /// Get a handle to the overseer.
    fn overseer_handle(&self) -> TanssiChainResult<Handle>;

    /// Generate a storage read proof.
    async fn prove_read(
        &self,
        relay_parent: PHash,
        relevant_keys: &Vec<Vec<u8>>,
    ) -> TanssiChainResult<StorageProof>;
}

#[async_trait::async_trait]
impl<T> TanssiChainInterface for Arc<T>
where
    T: TanssiChainInterface + ?Sized,
{
    fn overseer_handle(&self) -> TanssiChainResult<Handle> {
        (**self).overseer_handle()
    }

    async fn get_storage_by_key(
        &self,
        tanssi_parent: PHash,
        key: &[u8],
    ) -> TanssiChainResult<Option<StorageValue>> {
        (**self).get_storage_by_key(tanssi_parent, key).await
    }

    async fn prove_read(
        &self,
        tanssi_parent: PHash,
        relevant_keys: &Vec<Vec<u8>>,
    ) -> TanssiChainResult<StorageProof> {
        (**self).prove_read(tanssi_parent, relevant_keys).await
    }
}
