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
    core_extensions::TypeIdentity,
    cumulus_client_cli::CollatorOptions,
    cumulus_client_service::{build_relay_chain_interface, CollatorSybilResistance},
    cumulus_primitives_core::ParaId,
    cumulus_relay_chain_interface::RelayChainInterface,
    frame_benchmarking_cli::SUBSTRATE_REFERENCE_HARDWARE,
    futures::FutureExt,
    jsonrpsee::RpcModule,
    polkadot_primitives::CollatorPair,
    sc_client_api::Backend,
    sc_consensus::ImportQueue,
    sc_executor::{
        HeapAllocStrategy, NativeElseWasmExecutor, NativeExecutionDispatch, WasmExecutor,
        DEFAULT_HEAP_ALLOC_STRATEGY,
    },
    sc_network::{config::FullNetworkConfiguration, NetworkService},
    sc_network_sync::SyncingService,
    sc_network_transactions::TransactionsHandlerController,
    sc_rpc::{DenyUnsafe, SubscriptionTaskExecutor},
    sc_service::{
        Configuration, KeystoreContainer, NetworkStarter, TFullBackend, TFullClient, TaskManager,
    },
    sc_telemetry::{Telemetry, TelemetryWorker, TelemetryWorkerHandle},
    sc_transaction_pool_api::OffchainTransactionPoolFactory,
    sc_utils::mpsc::TracingUnboundedSender,
    sp_api::ConstructRuntimeApi,
    sp_block_builder::BlockBuilder,
    sp_offchain::OffchainWorkerApi,
    sp_transaction_pool::runtime_api::TaggedTransactionQueue,
    std::sync::Arc,
};

/// Functions in this module are generic over `Block`, `RuntimeApi`, and
/// `ParachainNativeExecutor`. Using type aliases requires them to be
/// generic too, which makes them still verbose to use. For that reason we use
/// a macro that expect the above types to already be in scope.
macro_rules! T {
    [Executor] => { NativeElseWasmExecutor<ParachainNativeExecutor> };
    [Client] => { TFullClient<Block, RuntimeApi, T![Executor]> };
    [Backend] => { TFullBackend<Block> };
    [ConstructedRuntimeApi] => {
        <RuntimeApi as ConstructRuntimeApi<Block, T![Client]>>::RuntimeApi
    };
    [Where] => {
        Block: cumulus_primitives_core::BlockT,
        ParachainNativeExecutor: NativeExecutionDispatch + 'static,
        RuntimeApi: ConstructRuntimeApi<Block, T![Client]> + Sync + Send + 'static,
        T![ConstructedRuntimeApi]: TaggedTransactionQueue<Block> + BlockBuilder<Block>,
    }
}

pub struct CumulusNetwork<Block: cumulus_primitives_core::BlockT> {
    pub network: Arc<NetworkService<Block, Block::Hash>>,
    pub system_rpc_tx: TracingUnboundedSender<sc_rpc::system::Request<Block>>,
    pub start_network: NetworkStarter,
    pub sync_service: Arc<SyncingService<Block>>,
}

// `Cumulus` and `TxHandler` are types that will change during the life of
// a `NodeBuilder` because they are generated and consumed when calling
// certain functions, with absence of data represented with `()`. Some
// function are implemented only for a given concrete type, which ensure it
// can only be called if the required data is available (generated and not yet
// consumed).
//
// While this could be implemented with multiple impl blocks with concrete types,
// we use here `core_extensions::TypeIdentity` which allow to express type
// identity/equality as a trait bound on each function as it removes the
// boilerplate of many impl block with duplicated trait bounds. 2 impl blocks
// are still required since Rust can't infer the types in the `new` function
// that doesn't take `self`.
pub struct NodeBuilder<
    Block,
    RuntimeApi,
    ParachainNativeExecutor,
    // `cumulus_client_service::build_network` returns many important systems,
    // but can only be called with an `import_queue` which can be different in
    // each node. For that reason it is a `()` when calling `new`, then the
    // caller create the `import_queue` using systems contained in `NodeBuilder`,
    // then call `build_cumulus_network` with it to generate the cumulus systems.
    Cumulus = (),
    // The `TxHandler` is constructed in `build_cumulus_network`
    // and is then consumed when calling `spawn_common_tasks`.
    TxHandler = (),
> where
    Block: cumulus_primitives_core::BlockT,
    ParachainNativeExecutor: NativeExecutionDispatch + 'static,
    RuntimeApi: ConstructRuntimeApi<Block, T![Client]> + Sync + Send + 'static,
    T![ConstructedRuntimeApi]: TaggedTransactionQueue<Block> + BlockBuilder<Block>,
{
    pub client: Arc<T![Client]>,
    pub backend: Arc<T![Backend]>,
    pub task_manager: TaskManager,
    pub keystore_container: KeystoreContainer,
    pub transaction_pool: Arc<sc_transaction_pool::FullPool<Block, T![Client]>>,
    pub telemetry: Option<Telemetry>,
    pub telemetry_worker_handle: Option<TelemetryWorkerHandle>,

    pub relay_chain_interface: Arc<dyn RelayChainInterface>,
    pub collator_key: Option<CollatorPair>,
    pub hwbench: Option<sc_sysinfo::HwBench>,

    pub cumulus: Cumulus,
    pub tx_handler_controller: TxHandler,
}

// `new` function doesn't take self, and the Rust compiler cannot infer that
// only one type T implements `TypeIdentity`. With thus need a separate impl
// block with concrete types `()`.
impl<Block, RuntimeApi, ParachainNativeExecutor>
    NodeBuilder<Block, RuntimeApi, ParachainNativeExecutor, (), ()>
where
    Block: cumulus_primitives_core::BlockT,
    ParachainNativeExecutor: NativeExecutionDispatch + 'static,
    RuntimeApi: ConstructRuntimeApi<Block, T![Client]> + Sync + Send + 'static,
    T![ConstructedRuntimeApi]: TaggedTransactionQueue<Block>
        + BlockBuilder<Block>
{
    /// Create a new `NodeBuilder` which prepare objects required to launch a
    /// node. However it doesn't start anything, and doesn't provide any
    /// cumulus-dependent objects (as it requires an import queue, which usually
    /// is different for each node).
    pub async fn new(
        parachain_config: &Configuration,
        polkadot_config: Configuration,
        collator_options: CollatorOptions,
        hwbench: Option<sc_sysinfo::HwBench>,
    ) -> Result<Self, sc_service::Error> {
        // Refactor: old new_partial + build_relay_chain_interface

        let telemetry = parachain_config
            .telemetry_endpoints
            .clone()
            .filter(|x| !x.is_empty())
            .map(|endpoints| -> Result<_, sc_telemetry::Error> {
                let worker = TelemetryWorker::new(16)?;
                let telemetry = worker.handle().new_telemetry(endpoints);
                Ok((worker, telemetry))
            })
            .transpose()?;

        let heap_pages =
            parachain_config
                .default_heap_pages
                .map_or(DEFAULT_HEAP_ALLOC_STRATEGY, |h| HeapAllocStrategy::Static {
                    extra_pages: h as _,
                });

        // Default runtime_cache_size is 2
        // For now we can work with this, but it will likely need
        // to change once we start having runtime_cache_sizes, or
        // run nodes with the maximum for this value
        let wasm = WasmExecutor::builder()
            .with_execution_method(parachain_config.wasm_method)
            .with_onchain_heap_alloc_strategy(heap_pages)
            .with_offchain_heap_alloc_strategy(heap_pages)
            .with_max_runtime_instances(parachain_config.max_runtime_instances)
            .with_runtime_cache_size(parachain_config.runtime_cache_size)
            .build();

        let executor = <T![Executor]>::new_with_wasm_executor(wasm);

        let (client, backend, keystore_container, mut task_manager) =
            sc_service::new_full_parts::<Block, RuntimeApi, _>(
                parachain_config,
                telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
                executor,
            )?;
        let client = Arc::new(client);

        let telemetry_worker_handle = telemetry.as_ref().map(|(worker, _)| worker.handle());

        let telemetry = telemetry.map(|(worker, telemetry)| {
            task_manager
                .spawn_handle()
                .spawn("telemetry", None, worker.run());
            telemetry
        });

        let transaction_pool = sc_transaction_pool::BasicPool::new_full(
            parachain_config.transaction_pool.clone(),
            parachain_config.role.is_authority().into(),
            parachain_config.prometheus_registry(),
            task_manager.spawn_essential_handle(),
            client.clone(),
        );

        let (relay_chain_interface, collator_key) = build_relay_chain_interface(
            polkadot_config,
            &parachain_config,
            telemetry_worker_handle.clone(),
            &mut task_manager,
            collator_options.clone(),
            hwbench.clone(),
        )
        .await
        .map_err(|e| sc_service::Error::Application(Box::new(e) as Box<_>))?;

        Ok(Self {
            client,
            backend,
            transaction_pool,
            telemetry,
            telemetry_worker_handle,
            task_manager,
            keystore_container,
            relay_chain_interface,
            collator_key,
            hwbench,
            cumulus: TypeIdentity::from_type(()),
            tx_handler_controller: TypeIdentity::from_type(()),
        })
    }
}

impl<Block, RuntimeApi, ParachainNativeExecutor, Cumulus, TxHandler>
    NodeBuilder<Block, RuntimeApi, ParachainNativeExecutor, Cumulus, TxHandler>
where
    Block: cumulus_primitives_core::BlockT,
    ParachainNativeExecutor: NativeExecutionDispatch + 'static,
    RuntimeApi: ConstructRuntimeApi<Block, T![Client]> + Sync + Send + 'static,
    T![ConstructedRuntimeApi]: TaggedTransactionQueue<Block>
        + BlockBuilder<Block>
        + cumulus_primitives_core::CollectCollationInfo<Block>,
{
    /// Given an import queue, calls `cumulus_client_service::build_network` and
    /// stores the returned objects in `self.cumulus` and `self.tx_handler_controller`.
    ///
    /// Can only be called once on a `NodeBuilder` that doesn't have yet cumulus
    /// data.
    pub async fn build_cumulus_network(
        self,
        parachain_config: &Configuration,
        para_id: ParaId,
        import_queue: impl ImportQueue<Block> + 'static,
    ) -> sc_service::error::Result<
        NodeBuilder<
            Block,
            RuntimeApi,
            ParachainNativeExecutor,
            CumulusNetwork<Block>,
            TransactionsHandlerController<Block::Hash>,
        >,
    >
    where
        Cumulus: TypeIdentity<Type = ()>,
        TxHandler: TypeIdentity<Type = ()>,
    {
        let Self {
            client,
            backend,
            transaction_pool,
            telemetry,
            telemetry_worker_handle,
            task_manager,
            keystore_container,
            relay_chain_interface,
            collator_key,
            hwbench,
            cumulus: _,
            tx_handler_controller: _,
        } = self;

        let net_config = FullNetworkConfiguration::new(&parachain_config.network);

        let (network, system_rpc_tx, tx_handler_controller, start_network, sync_service) =
            cumulus_client_service::build_network(cumulus_client_service::BuildNetworkParams {
                parachain_config: &parachain_config,
                client: client.clone(),
                transaction_pool: transaction_pool.clone(),
                spawn_handle: task_manager.spawn_handle(),
                import_queue: import_queue,
                para_id,
                relay_chain_interface: relay_chain_interface.clone(),
                net_config,
                sybil_resistance_level: CollatorSybilResistance::Resistant,
            })
            .await?;

        Ok(NodeBuilder {
            client,
            backend,
            transaction_pool,
            telemetry,
            telemetry_worker_handle,
            task_manager,
            keystore_container,
            relay_chain_interface,
            collator_key,
            hwbench,
            cumulus: CumulusNetwork {
                network,
                system_rpc_tx,
                start_network,
                sync_service,
            },
            tx_handler_controller,
        })
    }

    /// Given an `rpc_builder`, spawns the common tasks of a Substrate + Cumulus
    /// node. It consumes `self.tx_handler_controller` in the process, which means
    /// it can only be called once, and any other code that would need this
    /// controller should interact with it before calling this function.
    pub fn spawn_common_tasks<TRpc>(
        self,
        parachain_config: Configuration,
        rpc_builder: Box<
            dyn Fn(
                DenyUnsafe,
                SubscriptionTaskExecutor,
            ) -> Result<RpcModule<TRpc>, sc_service::Error>,
        >,
    ) -> sc_service::error::Result<
        NodeBuilder<Block, RuntimeApi, ParachainNativeExecutor, CumulusNetwork<Block>, ()>,
    >
    where
        Cumulus: TypeIdentity<Type = CumulusNetwork<Block>>,
        TxHandler: TypeIdentity<Type = TransactionsHandlerController<Block::Hash>>,
        Block::Hash: Unpin,
        Block::Header: Unpin,
        T![ConstructedRuntimeApi]: TaggedTransactionQueue<Block>
            + BlockBuilder<Block>
            + OffchainWorkerApi<Block>
            + sp_api::Metadata<Block>
            + sp_session::SessionKeys<Block>,
    {
        let NodeBuilder {
            client,
            backend,
            transaction_pool,
            mut telemetry,
            telemetry_worker_handle,
            mut task_manager,
            keystore_container,
            relay_chain_interface,
            collator_key,
            hwbench,
            cumulus,
            tx_handler_controller,
        } = self;

        let cumulus = TypeIdentity::into_type(cumulus);
        let tx_handler_controller = TypeIdentity::into_type(tx_handler_controller);

        let collator = parachain_config.role.is_authority();

        if parachain_config.offchain_worker.enabled {
            task_manager.spawn_handle().spawn(
                "offchain-workers-runner",
                "offchain-work",
                sc_offchain::OffchainWorkers::new(sc_offchain::OffchainWorkerOptions {
                    runtime_api_provider: client.clone(),
                    keystore: Some(keystore_container.keystore()),
                    offchain_db: backend.offchain_storage(),
                    transaction_pool: Some(OffchainTransactionPoolFactory::new(
                        transaction_pool.clone(),
                    )),
                    network_provider: cumulus.network.clone(),
                    is_validator: parachain_config.role.is_authority(),
                    enable_http_requests: false,
                    custom_extensions: move |_| vec![],
                })
                .run(client.clone(), task_manager.spawn_handle())
                .boxed(),
            );
        }

        sc_service::spawn_tasks(sc_service::SpawnTasksParams {
            rpc_builder,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            task_manager: &mut task_manager,
            config: parachain_config,
            keystore: keystore_container.keystore(),
            backend: backend.clone(),
            network: cumulus.network.clone(),
            system_rpc_tx: cumulus.system_rpc_tx.clone(),
            tx_handler_controller,
            telemetry: telemetry.as_mut(),
            sync_service: cumulus.sync_service.clone(),
        })?;

        if let Some(hwbench) = &hwbench {
            sc_sysinfo::print_hwbench(&hwbench);
            // Here you can check whether the hardware meets your chains' requirements. Putting a link
            // in there and swapping out the requirements for your own are probably a good idea. The
            // requirements for a para-chain are dictated by its relay-chain.
            if collator && !SUBSTRATE_REFERENCE_HARDWARE.check_hardware(&hwbench) {
                log::warn!(
                    "⚠️  The hardware does not meet the minimal requirements for role 'Authority'."
                );
            }

            if let Some(ref mut telemetry) = telemetry {
                let telemetry_handle = telemetry.handle();
                task_manager.spawn_handle().spawn(
                    "telemetry_hwbench",
                    None,
                    sc_sysinfo::initialize_hwbench_telemetry(telemetry_handle, hwbench.clone()),
                );
            }
        }

        Ok(NodeBuilder {
            client,
            backend,
            transaction_pool,
            telemetry,
            telemetry_worker_handle,
            task_manager,
            keystore_container,
            relay_chain_interface,
            collator_key,
            hwbench,
            cumulus: TypeIdentity::from_type(cumulus),
            tx_handler_controller: TypeIdentity::from_type(()),
        })
    }
}
