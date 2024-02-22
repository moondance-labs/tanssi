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
    sc_network::NetworkService,
    sc_network_sync::SyncingService,
    sc_transaction_pool::{ChainApi, Pool},
    sp_core::H256,
    sp_runtime::traits::Block as BlockT,
    std::{collections::BTreeMap, sync::Arc},
};
// Frontier
use fc_db::Backend as FrontierBackend;
pub use {
    fc_rpc::{EthBlockDataCacheTask, OverrideHandle},
    fc_rpc_core::types::{FeeHistoryCache, FeeHistoryCacheLimit, FilterPool},
    fc_storage::overrides_handle,
};

/// Extra dependencies for Ethereum compatibility.
pub struct EthDeps<C, P, A: ChainApi, CT, B: BlockT> {
    /// The client instance to use.
    pub client: Arc<C>,
    /// Transaction pool instance.
    pub pool: Arc<P>,
    /// Graph pool instance.
    pub graph: Arc<Pool<A>>,
    /// Ethereum transaction converter.
    pub converter: Option<CT>,
    /// The Node authority flag
    pub is_authority: bool,
    /// Whether to enable dev signer
    pub enable_dev_signer: bool,
    /// Network service
    pub network: Arc<NetworkService<B, B::Hash>>,
    /// Chain syncing service
    pub sync: Arc<SyncingService<B>>,
    /// Frontier Backend.
    pub frontier_backend: Arc<FrontierBackend<B>>,
    /// Ethereum data access overrides.
    pub overrides: Arc<OverrideHandle<B>>,
    /// Cache for Ethereum block data.
    pub block_data_cache: Arc<EthBlockDataCacheTask<B>>,
    /// EthFilterApi pool.
    pub filter_pool: Option<FilterPool>,
    /// Maximum number of logs in a query.
    pub max_past_logs: u32,
    /// Fee history cache.
    pub fee_history_cache: FeeHistoryCache,
    /// Maximum fee history cache size.
    pub fee_history_cache_limit: FeeHistoryCacheLimit,
    /// Maximum allowed gas limit will be ` block.gas_limit * execute_gas_limit_multiplier` when
    /// using eth_call/eth_estimateGas.
    pub execute_gas_limit_multiplier: u64,
    /// Mandated parent hashes for a given block hash.
    pub forced_parent_hashes: Option<BTreeMap<H256, H256>>,
}

impl<C, P, A: ChainApi, CT: Clone, B: BlockT> Clone for EthDeps<C, P, A, CT, B> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            pool: self.pool.clone(),
            graph: self.graph.clone(),
            converter: self.converter.clone(),
            is_authority: self.is_authority,
            enable_dev_signer: self.enable_dev_signer,
            network: self.network.clone(),
            sync: self.sync.clone(),
            frontier_backend: self.frontier_backend.clone(),
            overrides: self.overrides.clone(),
            block_data_cache: self.block_data_cache.clone(),
            filter_pool: self.filter_pool.clone(),
            max_past_logs: self.max_past_logs,
            fee_history_cache: self.fee_history_cache.clone(),
            fee_history_cache_limit: self.fee_history_cache_limit,
            execute_gas_limit_multiplier: self.execute_gas_limit_multiplier,
            forced_parent_hashes: self.forced_parent_hashes.clone(),
        }
    }
}
