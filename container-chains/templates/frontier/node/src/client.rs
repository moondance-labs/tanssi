// Substrate
use {
    sc_executor::{NativeExecutionDispatch, NativeVersion},
    sp_consensus_aura::sr25519::AuthorityId as AuraId,
    sp_runtime::traits::BlakeTwo256,
};
// Local
use container_chain_template_frontier_runtime::{opaque::Block, AccountId, Index};

use crate::eth::EthCompatRuntimeApiCollection;

/// Only enable the benchmarking host functions when we actually want to benchmark.
#[cfg(feature = "runtime-benchmarks")]
pub type HostFunctions = frame_benchmarking::benchmarking::HostFunctions;
/// Otherwise we use empty host functions for ext host functions.
#[cfg(not(feature = "runtime-benchmarks"))]
pub type HostFunctions = ();

pub struct TemplateRuntimeExecutor;
impl NativeExecutionDispatch for TemplateRuntimeExecutor {
    type ExtendHostFunctions = HostFunctions;

    fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
        container_chain_template_frontier_runtime::api::dispatch(method, data)
    }

    fn native_version() -> NativeVersion {
        container_chain_template_frontier_runtime::native_version()
    }
}

/// A set of APIs that every runtimes must implement.
pub trait BaseRuntimeApiCollection:
    sp_api::ApiExt<Block>
    + sp_api::Metadata<Block>
    + sp_block_builder::BlockBuilder<Block>
    + sp_offchain::OffchainWorkerApi<Block>
    + sp_session::SessionKeys<Block>
    + sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
where
    <Self as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
{
}

impl<Api> BaseRuntimeApiCollection for Api
where
    Api: sp_api::ApiExt<Block>
        + sp_api::Metadata<Block>
        + sp_block_builder::BlockBuilder<Block>
        + sp_offchain::OffchainWorkerApi<Block>
        + sp_session::SessionKeys<Block>
        + sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>,
    <Self as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
{
}

/// A set of APIs that template runtime must implement.
pub trait RuntimeApiCollection:
    BaseRuntimeApiCollection
    + EthCompatRuntimeApiCollection
    + sp_consensus_aura::AuraApi<Block, AuraId>
    + frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index>
where
    <Self as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
{
}

impl<Api> RuntimeApiCollection for Api
where
    Api: BaseRuntimeApiCollection
        + EthCompatRuntimeApiCollection
        + sp_consensus_aura::AuraApi<Block, AuraId>
        + frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index>,
    <Self as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
{
}
