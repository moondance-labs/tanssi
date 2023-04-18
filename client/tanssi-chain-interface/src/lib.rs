pub use cumulus_primitives_core::relay_chain::Hash as PHash;
use polkadot_overseer::Handle;
use sc_client_api::StorageProof;
use sp_state_machine::StorageValue;
use std::sync::Arc;

// TODO: proper errors
pub type TanssiChainResult<T> = Result<T, ()>;

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
