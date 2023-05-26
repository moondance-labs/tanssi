// Substrate
use sp_runtime::traits::BlakeTwo256;
// Frontier
pub use {
    fc_consensus::FrontierBlockImport,
    fc_rpc_core::types::{FeeHistoryCache, FeeHistoryCacheLimit, FilterPool},
};
// Local
use container_chain_template_frontier_runtime::opaque::Block;

/// A set of APIs that ethereum-compatible runtimes must implement.
pub trait EthCompatRuntimeApiCollection:
    sp_api::ApiExt<Block>
    + fp_rpc::EthereumRuntimeRPCApi<Block>
    + fp_rpc::ConvertTransactionRuntimeApi<Block>
where
    <Self as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
{
}

impl<Api> EthCompatRuntimeApiCollection for Api
where
    Api: sp_api::ApiExt<Block>
        + fp_rpc::EthereumRuntimeRPCApi<Block>
        + fp_rpc::ConvertTransactionRuntimeApi<Block>,
    <Self as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
{
}
