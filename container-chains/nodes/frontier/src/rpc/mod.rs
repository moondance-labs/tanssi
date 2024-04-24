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

//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

pub use sc_rpc::{DenyUnsafe, SubscriptionTaskExecutor};

use {
    container_chain_template_frontier_runtime::{opaque::Block, AccountId, Hash, Index},
    cumulus_client_parachain_inherent::ParachainInherentData,
    cumulus_primitives_core::{ParaId, PersistedValidationData},
    cumulus_test_relay_sproof_builder::RelayStateSproofBuilder,
    fc_rpc::{EthTask, TxPool},
    fc_rpc_core::TxPoolApiServer,
    fp_rpc::EthereumRuntimeRPCApi,
    futures::StreamExt,
    jsonrpsee::RpcModule,
    manual_xcm_rpc::{ManualXcm, ManualXcmApiServer},
    sc_client_api::{
        backend::{Backend, StateBackend},
        client::BlockchainEvents,
        AuxStore, BlockOf, StorageProvider,
    },
    sc_consensus_manual_seal::rpc::{EngineCommand, ManualSeal, ManualSealApiServer},
    sc_network::NetworkService,
    sc_network_sync::SyncingService,
    sc_service::TaskManager,
    sc_transaction_pool::{ChainApi, Pool},
    sc_transaction_pool_api::TransactionPool,
    sp_api::{CallApiAt, ProvideRuntimeApi},
    sp_block_builder::BlockBuilder,
    sp_blockchain::{
        Backend as BlockchainBackend, Error as BlockChainError, HeaderBackend, HeaderMetadata,
    },
    sp_consensus_aura::SlotDuration,
    sp_core::H256,
    sp_runtime::traits::{BlakeTwo256, Block as BlockT, Header as HeaderT},
    std::{sync::Arc, time::Duration},
};
pub struct DefaultEthConfig<C, BE>(std::marker::PhantomData<(C, BE)>);

impl<C, BE> fc_rpc::EthConfig<Block, C> for DefaultEthConfig<C, BE>
where
    C: StorageProvider<Block, BE> + Sync + Send + 'static,
    BE: Backend<Block> + 'static,
{
    type EstimateGasAdapter = ();
    type RuntimeStorageOverride =
        fc_rpc::frontier_backend_client::SystemAccountId20StorageOverride<Block, C, BE>;
}

mod eth;
pub use eth::*;
mod finality;

/// Full client dependencies.
pub struct FullDeps<C, P, A: ChainApi, BE> {
    /// The client instance to use.
    pub client: Arc<C>,
    /// Transaction pool instance.
    pub pool: Arc<P>,
    /// Graph pool instance.
    pub graph: Arc<Pool<A>>,
    /// Whether to deny unsafe calls
    pub deny_unsafe: DenyUnsafe,
    /// Network service
    pub network: Arc<NetworkService<Block, Hash>>,
    /// Chain syncing service
    pub sync: Arc<SyncingService<Block>>,
    /// EthFilterApi pool.
    pub filter_pool: Option<FilterPool>,
    /// Frontier Backend.
    // TODO: log indexer?
    pub frontier_backend: Arc<dyn fc_api::Backend<Block>>,
    /// Backend.
    pub backend: Arc<BE>,
    /// Maximum number of logs in a query.
    pub max_past_logs: u32,
    /// Maximum fee history cache size.
    pub fee_history_limit: u64,
    /// Fee history cache.
    pub fee_history_cache: FeeHistoryCache,
    /// Ethereum data access overrides.
    pub overrides: Arc<OverrideHandle<Block>>,
    /// Cache for Ethereum block data.
    pub block_data_cache: Arc<EthBlockDataCacheTask<Block>>,
    /// The Node authority flag
    pub is_authority: bool,
    /// Manual seal command sink
    pub command_sink: Option<futures::channel::mpsc::Sender<EngineCommand<Hash>>>,
    /// Channels for manual xcm messages (downward, hrmp)
    pub xcm_senders: Option<(flume::Sender<Vec<u8>>, flume::Sender<(ParaId, Vec<u8>)>)>,
}

/// Instantiate all Full RPC extensions.
pub fn create_full<C, P, BE, A>(
    deps: FullDeps<C, P, A, BE>,
    subscription_task_executor: SubscriptionTaskExecutor,
    pubsub_notification_sinks: Arc<
        fc_mapping_sync::EthereumBlockNotificationSinks<
            fc_mapping_sync::EthereumBlockNotification<Block>,
        >,
    >,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
    BE: Backend<Block> + 'static,
    BE::State: StateBackend<BlakeTwo256>,
    BE::Blockchain: BlockchainBackend<Block>,
    C: ProvideRuntimeApi<Block> + StorageProvider<Block, BE> + AuxStore,
    C: BlockchainEvents<Block>,
    C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
    C: CallApiAt<Block>,
    C: Send + Sync + 'static,
    A: ChainApi<Block = Block> + 'static,
    C::Api: RuntimeApiCollection,
    P: TransactionPool<Block = Block> + 'static,
{
    use finality::{FrontierFinality, FrontierFinalityApiServer};
    use {
        fc_rpc::{
            Eth, EthApiServer, EthFilter, EthFilterApiServer, EthPubSub, EthPubSubApiServer, Net,
            NetApiServer, Web3, Web3ApiServer,
        },
        substrate_frame_rpc_system::{System, SystemApiServer},
    };

    let mut io = RpcModule::new(());
    let FullDeps {
        client,
        pool,
        graph,
        deny_unsafe,
        network,
        sync,
        filter_pool,
        frontier_backend,
        backend: _,
        max_past_logs,
        fee_history_limit,
        fee_history_cache,
        overrides,
        block_data_cache,
        is_authority,
        command_sink,
        xcm_senders,
    } = deps;

    io.merge(System::new(Arc::clone(&client), Arc::clone(&pool), deny_unsafe).into_rpc())?;

    // TODO: are we supporting signing?
    let signers = Vec::new();

    enum Never {}
    impl<T> fp_rpc::ConvertTransaction<T> for Never {
        fn convert_transaction(&self, _transaction: pallet_ethereum::Transaction) -> T {
            // The Never type is not instantiable, but this method requires the type to be
            // instantiated to be called (`&self` parameter), so if the code compiles we have the
            // guarantee that this function will never be called.
            unreachable!()
        }
    }
    let convert_transaction: Option<Never> = None;
    let authorities = vec![tc_consensus::get_aura_id_from_seed("alice")];
    let authorities_for_cdp = authorities.clone();

    let pending_create_inherent_data_providers = move |_, _| {
        let authorities_for_cidp = authorities.clone();

        async move {
            let mocked_authorities_noting =
                ccp_authorities_noting_inherent::MockAuthoritiesNotingInherentDataProvider {
                    current_para_block: 1000,
                    relay_offset: 1000,
                    relay_blocks_per_para_block: 2,
                    orchestrator_para_id: 1000u32.into(),
                    container_para_id: 2000u32.into(),
                    authorities: authorities_for_cidp,
                };

            let timestamp = sp_timestamp::InherentDataProvider::from_system_time();
            // Create a dummy parachain inherent data provider which is required to pass
            // the checks by the para chain system. We use dummy values because in the 'pending context'
            // neither do we have access to the real values nor do we need them.
            let (relay_parent_storage_root, relay_chain_state) = RelayStateSproofBuilder {
                additional_key_values: mocked_authorities_noting.get_key_values(),
                ..Default::default()
            }
            .into_state_root_and_proof();
            let vfp = PersistedValidationData {
                // This is a hack to make `cumulus_pallet_parachain_system::RelayNumberStrictlyIncreases`
                // happy. Relay parent number can't be bigger than u32::MAX.
                relay_parent_number: u32::MAX,
                relay_parent_storage_root,
                ..Default::default()
            };
            let parachain_inherent_data = ParachainInherentData {
                validation_data: vfp,
                relay_chain_state,
                downward_messages: Default::default(),
                horizontal_messages: Default::default(),
            };
            Ok((
                timestamp,
                parachain_inherent_data,
                mocked_authorities_noting,
            ))
        }
    };

    let pending_consensus_data_provider_frontier: Option<
        Box<(dyn fc_rpc::pending::ConsensusDataProvider<_>)>,
    > = Some(Box::new(
        tc_consensus::ContainerManualSealAuraConsensusDataProvider::new(
            SlotDuration::from_millis(container_chain_template_frontier_runtime::SLOT_DURATION),
            authorities_for_cdp,
        ),
    ));

    io.merge(
        Eth::<_, _, _, _, _, _, _, DefaultEthConfig<C, BE>>::new(
            Arc::clone(&client),
            Arc::clone(&pool),
            Arc::clone(&graph),
            convert_transaction,
            Arc::clone(&sync),
            signers,
            Arc::clone(&overrides),
            Arc::clone(&frontier_backend),
            is_authority,
            Arc::clone(&block_data_cache),
            fee_history_cache,
            fee_history_limit,
            10,
            None,
            pending_create_inherent_data_providers,
            pending_consensus_data_provider_frontier,
        )
        .into_rpc(),
    )?;

    let tx_pool = TxPool::new(client.clone(), graph.clone());
    if let Some(filter_pool) = filter_pool {
        io.merge(
            EthFilter::new(
                client.clone(),
                frontier_backend.clone(),
                graph,
                filter_pool,
                500_usize, // max stored filters
                max_past_logs,
                block_data_cache,
            )
            .into_rpc(),
        )?;
    }

    io.merge(
        Net::new(
            Arc::clone(&client),
            network,
            // Whether to format the `peer_count` response as Hex (default) or not.
            true,
        )
        .into_rpc(),
    )?;

    if let Some(command_sink) = command_sink {
        io.merge(
            // We provide the rpc handler with the sending end of the channel to allow the rpc
            // send EngineCommands to the background block authorship task.
            ManualSeal::new(command_sink).into_rpc(),
        )?;
    };

    io.merge(Web3::new(Arc::clone(&client)).into_rpc())?;
    io.merge(
        EthPubSub::new(
            pool,
            Arc::clone(&client),
            sync,
            subscription_task_executor,
            overrides,
            pubsub_notification_sinks,
        )
        .into_rpc(),
    )?;
    io.merge(tx_pool.into_rpc())?;

    if let Some((downward_message_channel, hrmp_message_channel)) = xcm_senders {
        io.merge(
            ManualXcm {
                downward_message_channel,
                hrmp_message_channel,
            }
            .into_rpc(),
        )?;
    }

    io.merge(FrontierFinality::new(client.clone(), frontier_backend.clone()).into_rpc())?;

    Ok(io)
}

pub struct SpawnTasksParams<'a, B: BlockT, C, BE> {
    pub task_manager: &'a TaskManager,
    pub client: Arc<C>,
    pub substrate_backend: Arc<BE>,
    pub frontier_backend: fc_db::Backend<B>,
    pub filter_pool: Option<FilterPool>,
    pub overrides: Arc<OverrideHandle<B>>,
    pub fee_history_limit: u64,
    pub fee_history_cache: FeeHistoryCache,
    /// Chain syncing service
    pub sync_service: Arc<SyncingService<B>>,
    /// Chain syncing service
    pub pubsub_notification_sinks: Arc<
        fc_mapping_sync::EthereumBlockNotificationSinks<
            fc_mapping_sync::EthereumBlockNotification<B>,
        >,
    >,
}

use fc_mapping_sync::{kv::MappingSyncWorker, SyncStrategy};
/// Spawn the tasks that are required to run Moonbeam.
pub fn spawn_essential_tasks<B, C, BE>(params: SpawnTasksParams<B, C, BE>)
where
    C: ProvideRuntimeApi<B> + BlockOf,
    C: HeaderBackend<B> + HeaderMetadata<B, Error = BlockChainError> + 'static,
    C: BlockchainEvents<B> + StorageProvider<B, BE>,
    C: Send + Sync + 'static,
    C::Api: EthereumRuntimeRPCApi<B>,
    C::Api: BlockBuilder<B>,
    B: BlockT<Hash = H256> + Send + Sync + 'static,
    B::Header: HeaderT<Number = u32>,
    BE: Backend<B> + 'static,
    BE::State: StateBackend<BlakeTwo256>,
{
    // Frontier offchain DB task. Essential.
    // Maps emulated ethereum data to substrate native data.
    match params.frontier_backend {
        fc_db::Backend::KeyValue(b) => {
            params.task_manager.spawn_essential_handle().spawn(
                "frontier-mapping-sync-worker",
                Some("frontier"),
                MappingSyncWorker::new(
                    params.client.import_notification_stream(),
                    Duration::new(6, 0),
                    params.client.clone(),
                    params.substrate_backend.clone(),
                    params.overrides.clone(),
                    Arc::new(b),
                    3,
                    0,
                    SyncStrategy::Parachain,
                    params.sync_service.clone(),
                    params.pubsub_notification_sinks.clone(),
                )
                .for_each(|()| futures::future::ready(())),
            );
        }
        fc_db::Backend::Sql(b) => {
            params.task_manager.spawn_essential_handle().spawn_blocking(
                "frontier-mapping-sync-worker",
                Some("frontier"),
                fc_mapping_sync::sql::SyncWorker::run(
                    params.client.clone(),
                    params.substrate_backend.clone(),
                    Arc::new(b),
                    params.client.import_notification_stream(),
                    fc_mapping_sync::sql::SyncWorkerConfig {
                        read_notification_timeout: Duration::from_secs(10),
                        check_indexed_blocks_interval: Duration::from_secs(60),
                    },
                    fc_mapping_sync::SyncStrategy::Parachain,
                    params.sync_service.clone(),
                    params.pubsub_notification_sinks.clone(),
                ),
            );
        }
    }

    // Frontier `EthFilterApi` maintenance.
    // Manages the pool of user-created Filters.
    if let Some(filter_pool) = params.filter_pool {
        // Each filter is allowed to stay in the pool for 100 blocks.
        // TODO: Re-visit this assumption with parathreads, as they
        // might have a block every good amount of time, and can be abused
        // likely we will need to implement a time-based filter
        const FILTER_RETAIN_THRESHOLD: u64 = 100;
        params.task_manager.spawn_essential_handle().spawn(
            "frontier-filter-pool",
            Some("frontier"),
            EthTask::filter_pool_task(
                Arc::clone(&params.client),
                filter_pool,
                FILTER_RETAIN_THRESHOLD,
            ),
        );
    }

    // Spawn Frontier FeeHistory cache maintenance task.
    params.task_manager.spawn_essential_handle().spawn(
        "frontier-fee-history",
        Some("frontier"),
        EthTask::fee_history_task(
            Arc::clone(&params.client),
            Arc::clone(&params.overrides),
            params.fee_history_cache,
            params.fee_history_limit,
        ),
    );
}

/// A set of APIs that polkadot-like runtimes must implement.
///
/// This trait has no methods or associated type. It is a concise marker for all the trait bounds
/// that it contains.
pub trait RuntimeApiCollection:
    sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
    + sp_api::ApiExt<Block>
    + sp_block_builder::BlockBuilder<Block>
    + substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>
    + sp_api::Metadata<Block>
    + sp_offchain::OffchainWorkerApi<Block>
    + sp_session::SessionKeys<Block>
    + fp_rpc::ConvertTransactionRuntimeApi<Block>
    + fp_rpc::EthereumRuntimeRPCApi<Block>
    + cumulus_primitives_core::CollectCollationInfo<Block>
{
}

impl<Api> RuntimeApiCollection for Api where
    Api: sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
        + sp_api::ApiExt<Block>
        + sp_block_builder::BlockBuilder<Block>
        + substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>
        + sp_api::Metadata<Block>
        + sp_offchain::OffchainWorkerApi<Block>
        + sp_session::SessionKeys<Block>
        + fp_rpc::ConvertTransactionRuntimeApi<Block>
        + fp_rpc::EthereumRuntimeRPCApi<Block>
        + cumulus_primitives_core::CollectCollationInfo<Block>
{
}
