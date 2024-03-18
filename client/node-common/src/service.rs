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
    async_io::Timer,
    core::time::Duration,
    core_extensions::TypeIdentity,
    cumulus_client_cli::CollatorOptions,
    cumulus_client_consensus_common::ParachainConsensus,
    cumulus_client_service::{
        build_relay_chain_interface, CollatorSybilResistance, StartFullNodeParams,
    },
    cumulus_primitives_core::ParaId,
    cumulus_relay_chain_interface::RelayChainInterface,
    frame_benchmarking_cli::SUBSTRATE_REFERENCE_HARDWARE,
    futures::{channel::mpsc, FutureExt, Stream, StreamExt},
    jsonrpsee::RpcModule,
    polkadot_primitives::CollatorPair,
    sc_client_api::Backend,
    sc_consensus::{import_queue::ImportQueueService, BlockImport, ImportQueue},
    sc_consensus_manual_seal::{
        run_manual_seal, ConsensusDataProvider, EngineCommand, ManualSealParams,
    },
    sc_executor::{
        sp_wasm_interface::{ExtendedHostFunctions, HostFunctions},
        HeapAllocStrategy, NativeElseWasmExecutor, NativeExecutionDispatch, RuntimeVersionOf,
        WasmExecutor, DEFAULT_HEAP_ALLOC_STRATEGY,
    },
    sc_network::{config::FullNetworkConfiguration, NetworkBlock, NetworkService},
    sc_network_sync::SyncingService,
    sc_network_transactions::TransactionsHandlerController,
    sc_rpc::{DenyUnsafe, SubscriptionTaskExecutor},
    sc_service::{
        Configuration, KeystoreContainer, NetworkStarter, SpawnTaskHandle, TFullBackend,
        TFullClient, TaskManager,
    },
    sc_telemetry::{Telemetry, TelemetryWorker, TelemetryWorkerHandle},
    sc_transaction_pool_api::OffchainTransactionPoolFactory,
    sc_utils::mpsc::TracingUnboundedSender,
    sp_api::ConstructRuntimeApi,
    sp_block_builder::BlockBuilder,
    sp_consensus::SelectChain,
    sp_core::traits::CodeExecutor,
    sp_inherents::CreateInherentDataProviders,
    sp_offchain::OffchainWorkerApi,
    sp_runtime::Percent,
    sp_transaction_pool::runtime_api::TaggedTransactionQueue,
    std::{str::FromStr, sync::Arc},
};

/// Trait to configure the main types the builder rely on, bundled in a single
/// type to reduce verbosity and the amount of type parameters.
pub trait NodeBuilderConfig {
    type Block;
    type RuntimeApi;
    type ParachainExecutor;

    /// Create a new `NodeBuilder` using the types of this `Config`, along
    /// with the parachain `Configuration` and an optional `HwBench`.
    fn new_builder(
        parachain_config: &Configuration,
        hwbench: Option<sc_sysinfo::HwBench>,
    ) -> Result<NodeBuilder<Self>, sc_service::Error>
    where
        Self: Sized,
        BlockOf<Self>: cumulus_primitives_core::BlockT,
        ExecutorOf<Self>:
            Clone + CodeExecutor + RuntimeVersionOf + TanssiExecutorExt + Sync + Send + 'static,
        RuntimeApiOf<Self>:
            ConstructRuntimeApi<BlockOf<Self>, ClientOf<Self>> + Sync + Send + 'static,
        ConstructedRuntimeApiOf<Self>:
            TaggedTransactionQueue<BlockOf<Self>> + BlockBuilder<BlockOf<Self>>,
    {
        NodeBuilder::<Self>::new(parachain_config, hwbench)
    }
}

pub type BlockOf<T> = <T as NodeBuilderConfig>::Block;
pub type BlockHashOf<T> = <BlockOf<T> as cumulus_primitives_core::BlockT>::Hash;
pub type BlockHeaderOf<T> = <BlockOf<T> as cumulus_primitives_core::BlockT>::Header;
pub type RuntimeApiOf<T> = <T as NodeBuilderConfig>::RuntimeApi;
pub type ExecutorOf<T> = <T as NodeBuilderConfig>::ParachainExecutor;
pub type ClientOf<T> = TFullClient<BlockOf<T>, RuntimeApiOf<T>, ExecutorOf<T>>;
pub type BackendOf<T> = TFullBackend<BlockOf<T>>;
pub type ConstructedRuntimeApiOf<T> =
    <RuntimeApiOf<T> as ConstructRuntimeApi<BlockOf<T>, ClientOf<T>>>::RuntimeApi;
pub type ImportQueueServiceOf<T> = Box<dyn ImportQueueService<BlockOf<T>>>;
pub type ParachainConsensusOf<T> = Box<dyn ParachainConsensus<BlockOf<T>>>;

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
    T: NodeBuilderConfig,
    // `(cumulus_client_service/sc_service)::build_network` returns many important systems,
    // but can only be called with an `import_queue` which can be different in
    // each node. For that reason it is a `()` when calling `new`, then the
    // caller create the `import_queue` using systems contained in `NodeBuilder`,
    // then call `build_cumulus_network` with it to generate the cumulus systems.
    SNetwork = (),
    // The `TxHandler` is constructed in `build_X_network`
    // and is then consumed when calling `spawn_common_tasks`.
    STxHandler = (),
    // The import queue service is obtained from the import queue in
    // `build_cumulus_network` or `build_substrate_network`, which also
    // consumes the import queue. Neither of them are clonable, so we need to
    // to store the service here to be able to consume it later in
    // `start_full_node`.
    SImportQueueService = (),
> where
    BlockOf<T>: cumulus_primitives_core::BlockT,
    ExecutorOf<T>: Clone + CodeExecutor + RuntimeVersionOf + Sync + Send + 'static,
    RuntimeApiOf<T>: ConstructRuntimeApi<BlockOf<T>, ClientOf<T>> + Sync + Send + 'static,
    ConstructedRuntimeApiOf<T>: TaggedTransactionQueue<BlockOf<T>> + BlockBuilder<BlockOf<T>>,
{
    pub client: Arc<ClientOf<T>>,
    pub backend: Arc<BackendOf<T>>,
    pub task_manager: TaskManager,
    pub keystore_container: KeystoreContainer,
    pub transaction_pool: Arc<sc_transaction_pool::FullPool<BlockOf<T>, ClientOf<T>>>,
    pub telemetry: Option<Telemetry>,
    pub telemetry_worker_handle: Option<TelemetryWorkerHandle>,

    pub hwbench: Option<sc_sysinfo::HwBench>,
    pub prometheus_registry: Option<substrate_prometheus_endpoint::Registry>,

    pub network: SNetwork,
    pub tx_handler_controller: STxHandler,
    pub import_queue_service: SImportQueueService,
}

pub struct Network<Block: cumulus_primitives_core::BlockT> {
    pub network: Arc<NetworkService<Block, Block::Hash>>,
    pub system_rpc_tx: TracingUnboundedSender<sc_rpc::system::Request<Block>>,
    pub start_network: NetworkStarter,
    pub sync_service: Arc<SyncingService<Block>>,
}

/// Allows to create a parachain-defined executor from a `WasmExecutor`
pub trait TanssiExecutorExt {
    type HostFun: HostFunctions;
    fn new_with_wasm_executor(wasm_executor: WasmExecutor<Self::HostFun>) -> Self;
}

impl TanssiExecutorExt for WasmExecutor<sp_io::SubstrateHostFunctions> {
    type HostFun = sp_io::SubstrateHostFunctions;

    fn new_with_wasm_executor(wasm_executor: WasmExecutor<Self::HostFun>) -> Self {
        wasm_executor
    }
}

impl<D> TanssiExecutorExt for NativeElseWasmExecutor<D>
where
    D: NativeExecutionDispatch,
{
    type HostFun = ExtendedHostFunctions<sp_io::SubstrateHostFunctions, D::ExtendHostFunctions>;

    fn new_with_wasm_executor(wasm_executor: WasmExecutor<Self::HostFun>) -> Self {
        NativeElseWasmExecutor::new_with_wasm_executor(wasm_executor)
    }
}

// `new` function doesn't take self, and the Rust compiler cannot infer that
// only one type T implements `TypeIdentity`. With thus need a separate impl
// block with concrete types `()`.
impl<T: NodeBuilderConfig> NodeBuilder<T>
where
    BlockOf<T>: cumulus_primitives_core::BlockT,
    ExecutorOf<T>:
        Clone + CodeExecutor + RuntimeVersionOf + TanssiExecutorExt + Sync + Send + 'static,
    RuntimeApiOf<T>: ConstructRuntimeApi<BlockOf<T>, ClientOf<T>> + Sync + Send + 'static,
    ConstructedRuntimeApiOf<T>: TaggedTransactionQueue<BlockOf<T>> + BlockBuilder<BlockOf<T>>,
{
    /// Create a new `NodeBuilder` which prepare objects required to launch a
    /// node. However it only starts telemetry, and doesn't provide any
    /// network-dependent objects (as it requires an import queue, which usually
    /// is different for each node).
    fn new(
        parachain_config: &Configuration,
        hwbench: Option<sc_sysinfo::HwBench>,
    ) -> Result<Self, sc_service::Error> {
        // Refactor: old new_partial

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
                    extra_pages: h as u32,
                });

        // Default runtime_cache_size is 2
        // For now we can work with this, but it will likely need
        // to change once we start having runtime_cache_sizes, or
        // run nodes with the maximum for this value
        let mut wasm_builder = WasmExecutor::builder()
            .with_execution_method(parachain_config.wasm_method)
            .with_onchain_heap_alloc_strategy(heap_pages)
            .with_offchain_heap_alloc_strategy(heap_pages)
            .with_max_runtime_instances(parachain_config.max_runtime_instances)
            .with_runtime_cache_size(parachain_config.runtime_cache_size);
        if let Some(ref wasmtime_precompiled_path) = parachain_config.wasmtime_precompiled {
            wasm_builder = wasm_builder.with_wasmtime_precompiled_path(wasmtime_precompiled_path);
        }

        let executor = ExecutorOf::<T>::new_with_wasm_executor(wasm_builder.build());

        let (client, backend, keystore_container, task_manager) =
            sc_service::new_full_parts::<BlockOf<T>, RuntimeApiOf<T>, _>(
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

        Ok(Self {
            client,
            backend,
            transaction_pool,
            telemetry,
            telemetry_worker_handle,
            task_manager,
            keystore_container,
            hwbench,
            prometheus_registry: parachain_config.prometheus_registry().cloned(),
            network: TypeIdentity::from_type(()),
            tx_handler_controller: TypeIdentity::from_type(()),
            import_queue_service: TypeIdentity::from_type(()),
        })
    }
}

impl<T: NodeBuilderConfig, SNetwork, STxHandler, SImportQueueService>
    NodeBuilder<T, SNetwork, STxHandler, SImportQueueService>
where
    BlockOf<T>: cumulus_primitives_core::BlockT,
    ExecutorOf<T>: Clone + CodeExecutor + RuntimeVersionOf + Sync + Send + 'static,
    RuntimeApiOf<T>: ConstructRuntimeApi<BlockOf<T>, ClientOf<T>> + Sync + Send + 'static,
    ConstructedRuntimeApiOf<T>: TaggedTransactionQueue<BlockOf<T>>
        + BlockBuilder<BlockOf<T>>
        + cumulus_primitives_core::CollectCollationInfo<BlockOf<T>>,
{
    pub async fn build_relay_chain_interface(
        &mut self,
        parachain_config: &Configuration,
        polkadot_config: Configuration,
        collator_options: CollatorOptions,
    ) -> sc_service::error::Result<(
        Arc<(dyn RelayChainInterface + 'static)>,
        Option<CollatorPair>,
    )> {
        build_relay_chain_interface(
            polkadot_config,
            parachain_config,
            self.telemetry_worker_handle.clone(),
            &mut self.task_manager,
            collator_options.clone(),
            self.hwbench.clone(),
        )
        .await
        .map_err(|e| sc_service::Error::Application(Box::new(e) as Box<_>))
    }

    /// Given an import queue, calls `cumulus_client_service::build_network` and
    /// stores the returned objects in `self.network` and `self.tx_handler_controller`.
    ///
    /// Can only be called once on a `NodeBuilder` that doesn't have yet network
    /// data.
    pub async fn build_cumulus_network<RCInterface>(
        self,
        parachain_config: &Configuration,
        para_id: ParaId,
        import_queue: impl ImportQueue<BlockOf<T>> + 'static,
        relay_chain_interface: RCInterface,
    ) -> sc_service::error::Result<
        NodeBuilder<
            T,
            Network<BlockOf<T>>,
            TransactionsHandlerController<BlockHashOf<T>>,
            ImportQueueServiceOf<T>,
        >,
    >
    where
        SNetwork: TypeIdentity<Type = ()>,
        STxHandler: TypeIdentity<Type = ()>,
        SImportQueueService: TypeIdentity<Type = ()>,
        RCInterface: RelayChainInterface + Clone + 'static,
    {
        let Self {
            client,
            backend,
            transaction_pool,
            telemetry,
            telemetry_worker_handle,
            task_manager,
            keystore_container,
            hwbench,
            prometheus_registry,
            network: _,
            tx_handler_controller: _,
            import_queue_service: _,
        } = self;

        let net_config = FullNetworkConfiguration::new(&parachain_config.network);
        let import_queue_service = import_queue.service();
        let spawn_handle = task_manager.spawn_handle();

        let (network, system_rpc_tx, tx_handler_controller, start_network, sync_service) =
            cumulus_client_service::build_network(cumulus_client_service::BuildNetworkParams {
                parachain_config,
                client: client.clone(),
                transaction_pool: transaction_pool.clone(),
                spawn_handle,
                import_queue,
                para_id,
                relay_chain_interface,
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
            hwbench,
            prometheus_registry,
            network: Network {
                network,
                system_rpc_tx,
                start_network,
                sync_service,
            },
            tx_handler_controller,
            import_queue_service,
        })
    }

    /// Given an import queue, calls `sc_service::build_network` and
    /// stores the returned objects in `self.network` and `self.tx_handler_controller`.
    ///
    /// Can only be called once on a `NodeBuilder` that doesn't have yet network
    /// data.
    pub fn build_substrate_network(
        self,
        parachain_config: &Configuration,
        import_queue: impl ImportQueue<BlockOf<T>> + 'static,
    ) -> sc_service::error::Result<
        NodeBuilder<
            T,
            Network<BlockOf<T>>,
            TransactionsHandlerController<BlockHashOf<T>>,
            ImportQueueServiceOf<T>,
        >,
    >
    where
        SNetwork: TypeIdentity<Type = ()>,
        STxHandler: TypeIdentity<Type = ()>,
        SImportQueueService: TypeIdentity<Type = ()>,
    {
        let Self {
            client,
            backend,
            transaction_pool,
            telemetry,
            telemetry_worker_handle,
            task_manager,
            keystore_container,
            hwbench,
            prometheus_registry,
            network: _,
            tx_handler_controller: _,
            import_queue_service: _,
        } = self;

        let net_config = FullNetworkConfiguration::new(&parachain_config.network);
        let import_queue_service = import_queue.service();

        let (network, system_rpc_tx, tx_handler_controller, start_network, sync_service) =
            sc_service::build_network(sc_service::BuildNetworkParams {
                config: parachain_config,
                client: client.clone(),
                transaction_pool: transaction_pool.clone(),
                spawn_handle: task_manager.spawn_handle(),
                import_queue,
                warp_sync_params: None,
                block_announce_validator_builder: None,
                net_config,
                block_relay: None,
            })?;

        Ok(NodeBuilder {
            client,
            backend,
            transaction_pool,
            telemetry,
            telemetry_worker_handle,
            task_manager,
            keystore_container,
            hwbench,
            prometheus_registry,
            network: Network {
                network,
                system_rpc_tx,
                start_network,
                sync_service,
            },
            tx_handler_controller,
            import_queue_service,
        })
    }

    /// Given an `rpc_builder`, spawns the common tasks of a Substrate
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
    ) -> sc_service::error::Result<NodeBuilder<T, Network<BlockOf<T>>, (), SImportQueueService>>
    where
        SNetwork: TypeIdentity<Type = Network<BlockOf<T>>>,
        STxHandler: TypeIdentity<Type = TransactionsHandlerController<BlockHashOf<T>>>,
        BlockHashOf<T>: Unpin,
        BlockHeaderOf<T>: Unpin,
        ConstructedRuntimeApiOf<T>: TaggedTransactionQueue<BlockOf<T>>
            + BlockBuilder<BlockOf<T>>
            + OffchainWorkerApi<BlockOf<T>>
            + sp_api::Metadata<BlockOf<T>>
            + sp_session::SessionKeys<BlockOf<T>>,
    {
        let NodeBuilder {
            client,
            backend,
            transaction_pool,
            mut telemetry,
            telemetry_worker_handle,
            mut task_manager,
            keystore_container,
            hwbench,
            prometheus_registry,
            network,
            tx_handler_controller,
            import_queue_service,
        } = self;

        let network = TypeIdentity::into_type(network);
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
                    network_provider: network.network.clone(),
                    is_validator: parachain_config.role.is_authority(),
                    enable_http_requests: false,
                    custom_extensions: move |_| vec![],
                })
                .run(client.clone(), task_manager.spawn_handle())
                .boxed(),
            );
        }

        let _rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
            rpc_builder,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            task_manager: &mut task_manager,
            config: parachain_config,
            keystore: keystore_container.keystore(),
            backend: backend.clone(),
            network: network.network.clone(),
            system_rpc_tx: network.system_rpc_tx.clone(),
            tx_handler_controller,
            telemetry: telemetry.as_mut(),
            sync_service: network.sync_service.clone(),
        })?;

        if let Some(hwbench) = &hwbench {
            sc_sysinfo::print_hwbench(hwbench);
            // Here you can check whether the hardware meets your chains' requirements. Putting a link
            // in there and swapping out the requirements for your own are probably a good idea. The
            // requirements for a para-chain are dictated by its relay-chain.
            if collator {
                if let Err(err) = SUBSTRATE_REFERENCE_HARDWARE.check_hardware(hwbench) {
                    log::warn!(
                        "⚠️  The hardware does not meet the minimal requirements {} for role 'Authority'.",
                        err
                    );
                }
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
            hwbench,
            prometheus_registry,
            network: TypeIdentity::from_type(network),
            tx_handler_controller: TypeIdentity::from_type(()),
            import_queue_service,
        })
    }

    pub fn install_manual_seal<BI, SC, CIDP>(
        &mut self,
        manual_seal_config: ManualSealConfiguration<BlockOf<T>, BI, SC, CIDP>,
    ) -> sc_service::error::Result<Option<mpsc::Sender<EngineCommand<BlockHashOf<T>>>>>
    where
        BI: BlockImport<BlockOf<T>, Error = sp_consensus::Error> + Send + Sync + 'static,
        SC: SelectChain<BlockOf<T>> + 'static,
        CIDP: CreateInherentDataProviders<BlockOf<T>, ()> + 'static,
    {
        let ManualSealConfiguration {
            sealing,
            soft_deadline,
            block_import,
            select_chain,
            consensus_data_provider,
            create_inherent_data_providers,
        } = manual_seal_config;

        let prometheus_registry = self.prometheus_registry.clone();

        let mut env = sc_basic_authorship::ProposerFactory::new(
            self.task_manager.spawn_handle(),
            self.client.clone(),
            self.transaction_pool.clone(),
            prometheus_registry.as_ref(),
            self.telemetry.as_ref().map(|x| x.handle()),
        );

        let mut command_sink = None;

        if let Some(deadline) = soft_deadline {
            env.set_soft_deadline(deadline);
        }

        let commands_stream: Box<
            dyn Stream<Item = EngineCommand<BlockHashOf<T>>> + Send + Sync + Unpin,
        > = match sealing {
            Sealing::Instant => {
                Box::new(
                    // This bit cribbed from the implementation of instant seal.
                    self.transaction_pool
                        .pool()
                        .validated_pool()
                        .import_notification_stream()
                        .map(|_| EngineCommand::SealNewBlock {
                            create_empty: false,
                            finalize: false,
                            parent_hash: None,
                            sender: None,
                        }),
                )
            }
            Sealing::Manual => {
                let (sink, stream) = futures::channel::mpsc::channel(1000);
                // Keep a reference to the other end of the channel. It goes to the RPC.
                command_sink = Some(sink);
                Box::new(stream)
            }
            Sealing::Interval(millis) => Box::new(futures::StreamExt::map(
                Timer::interval(Duration::from_millis(millis)),
                |_| EngineCommand::SealNewBlock {
                    create_empty: true,
                    finalize: true,
                    parent_hash: None,
                    sender: None,
                },
            )),
        };

        self.task_manager.spawn_essential_handle().spawn_blocking(
            "authorship_task",
            Some("block-authoring"),
            run_manual_seal(ManualSealParams {
                block_import,
                env,
                client: self.client.clone(),
                pool: self.transaction_pool.clone(),
                commands_stream,
                select_chain,
                consensus_data_provider,
                create_inherent_data_providers,
            }),
        );

        Ok(command_sink)
    }

    pub fn start_full_node<RCInterface>(
        self,
        para_id: ParaId,
        relay_chain_interface: RCInterface,
        relay_chain_slot_duration: Duration,
    ) -> sc_service::error::Result<NodeBuilder<T, SNetwork, STxHandler, ()>>
    where
        SNetwork: TypeIdentity<Type = Network<BlockOf<T>>>,
        SImportQueueService: TypeIdentity<Type = ImportQueueServiceOf<T>>,
        RCInterface: RelayChainInterface + Clone + 'static,
    {
        let NodeBuilder {
            client,
            backend,
            transaction_pool,
            telemetry,
            telemetry_worker_handle,
            mut task_manager,
            keystore_container,
            hwbench,
            prometheus_registry,
            network,
            tx_handler_controller,
            import_queue_service,
        } = self;

        let network = TypeIdentity::into_type(network);
        let import_queue_service = TypeIdentity::into_type(import_queue_service);

        let announce_block = {
            let sync_service = network.sync_service.clone();
            Arc::new(move |hash, data| sync_service.announce_block(hash, data))
        };

        let overseer_handle = relay_chain_interface
            .overseer_handle()
            .map_err(|e| sc_service::Error::Application(Box::new(e)))?;

        let params = StartFullNodeParams {
            client: client.clone(),
            announce_block,
            task_manager: &mut task_manager,
            para_id,
            relay_chain_interface,
            relay_chain_slot_duration,
            import_queue: import_queue_service,
            recovery_handle: Box::new(overseer_handle),
            sync_service: network.sync_service.clone(),
        };

        // TODO: change for async backing
        #[allow(deprecated)]
        cumulus_client_service::start_full_node(params)?;

        Ok(NodeBuilder {
            client,
            backend,
            transaction_pool,
            telemetry,
            telemetry_worker_handle,
            task_manager,
            keystore_container,
            hwbench,
            prometheus_registry,
            network: TypeIdentity::from_type(network),
            tx_handler_controller,
            import_queue_service: (),
        })
    }

    pub async fn start_collator<RCInterface>(
        self,
        para_id: ParaId,
        relay_chain_interface: RCInterface,
        relay_chain_slot_duration: Duration,
        parachain_consensus: ParachainConsensusOf<T>,
        collator_key: CollatorPair,
    ) -> sc_service::error::Result<NodeBuilder<T, SNetwork, STxHandler, ()>>
    where
        SNetwork: TypeIdentity<Type = Network<BlockOf<T>>>,
        SImportQueueService: TypeIdentity<Type = ImportQueueServiceOf<T>>,
        RCInterface: RelayChainInterface + Clone + 'static,
    {
        let NodeBuilder {
            client,
            backend,
            transaction_pool,
            telemetry,
            telemetry_worker_handle,
            mut task_manager,
            keystore_container,
            hwbench,
            prometheus_registry,
            network,
            tx_handler_controller,
            import_queue_service,
        } = self;

        let network = TypeIdentity::into_type(network);
        let import_queue_service = TypeIdentity::into_type(import_queue_service);

        let spawner = task_manager.spawn_handle();
        let announce_block = {
            let sync_service = network.sync_service.clone();
            Arc::new(move |hash, data| sync_service.announce_block(hash, data))
        };
        let overseer_handle = relay_chain_interface
            .overseer_handle()
            .map_err(|e| sc_service::Error::Application(Box::new(e)))?;

        let params = cumulus_client_service::StartCollatorParams {
            para_id,
            block_status: client.clone(),
            announce_block: announce_block.clone(),
            client: client.clone(),
            task_manager: &mut task_manager,
            relay_chain_interface: relay_chain_interface.clone(),
            spawner: spawner.clone(),
            parachain_consensus,
            import_queue: import_queue_service,
            collator_key,
            relay_chain_slot_duration,
            recovery_handle: Box::new(overseer_handle.clone()),
            sync_service: network.sync_service.clone(),
        };

        // TODO: change for async backing
        #[allow(deprecated)]
        cumulus_client_service::start_collator(params).await?;

        Ok(NodeBuilder {
            client,
            backend,
            transaction_pool,
            telemetry,
            telemetry_worker_handle,
            task_manager,
            keystore_container,
            hwbench,
            prometheus_registry,
            network: TypeIdentity::from_type(network),
            tx_handler_controller,
            import_queue_service: (),
        })
    }

    pub fn extract_import_queue_service(
        self,
    ) -> (
        NodeBuilder<T, SNetwork, STxHandler, ()>,
        SImportQueueService,
    )
    where
        SNetwork: TypeIdentity<Type = Network<BlockOf<T>>>,
    {
        let NodeBuilder {
            client,
            backend,
            transaction_pool,
            telemetry,
            telemetry_worker_handle,
            task_manager,
            keystore_container,
            hwbench,
            prometheus_registry,
            network,
            tx_handler_controller,
            import_queue_service,
        } = self;

        (
            NodeBuilder {
                client,
                backend,
                transaction_pool,
                telemetry,
                telemetry_worker_handle,
                task_manager,
                keystore_container,
                hwbench,
                prometheus_registry,
                network,
                tx_handler_controller,
                import_queue_service: (),
            },
            import_queue_service,
        )
    }

    pub fn cumulus_client_collator_params_generator(
        &self,
        para_id: ParaId,
        overseer_handle: cumulus_relay_chain_interface::OverseerHandle,
        collator_key: CollatorPair,
        parachain_consensus: ParachainConsensusOf<T>,
    ) -> impl Fn() -> cumulus_client_collator::StartCollatorParams<
        BlockOf<T>,
        ClientOf<T>,
        ClientOf<T>,
        SpawnTaskHandle,
    > + Send
           + Clone
           + 'static
    where
        SNetwork: TypeIdentity<Type = Network<BlockOf<T>>>,
    {
        let network = TypeIdentity::as_type(&self.network);

        let client = self.client.clone();
        let announce_block = {
            let sync_service = network.sync_service.clone();
            Arc::new(move |hash, data| sync_service.announce_block(hash, data))
        };
        let spawner = self.task_manager.spawn_handle();

        move || cumulus_client_collator::StartCollatorParams {
            runtime_api: client.clone(),
            block_status: client.clone(),
            announce_block: announce_block.clone(),
            overseer_handle: overseer_handle.clone(),
            spawner: spawner.clone(),
            para_id,
            key: collator_key.clone(),
            parachain_consensus: parachain_consensus.clone(),
        }
    }
}

/// Block authoring scheme to be used by the dev service.
#[derive(Debug, Copy, Clone)]
pub enum Sealing {
    /// Author a block immediately upon receiving a transaction into the transaction pool
    Instant,
    /// Author a block upon receiving an RPC command
    Manual,
    /// Author blocks at a regular interval specified in milliseconds
    Interval(u64),
}

impl FromStr for Sealing {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "instant" => Self::Instant,
            "manual" => Self::Manual,
            s => {
                let millis = s
                    .parse::<u64>()
                    .map_err(|_| "couldn't decode sealing param")?;
                Self::Interval(millis)
            }
        })
    }
}

pub struct ManualSealConfiguration<B, BI, SC, CIDP> {
    pub sealing: Sealing,
    pub block_import: BI,
    pub soft_deadline: Option<Percent>,
    pub select_chain: SC,
    pub consensus_data_provider: Option<Box<dyn ConsensusDataProvider<B, Proof = ()>>>,
    pub create_inherent_data_providers: CIDP,
}
