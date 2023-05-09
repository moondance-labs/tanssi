//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

use {
    crate::cli::ContainerChainCli,
    cumulus_client_cli::CollatorOptions,
    cumulus_client_consensus_aura::{AuraConsensus, BuildAuraConsensusParams, SlotProportion},
    cumulus_client_consensus_common::{
        ParachainBlockImport as TParachainBlockImport, ParachainBlockImportMarker,
        ParachainConsensus,
    },
    cumulus_client_service::{
        build_relay_chain_interface, prepare_node_config, start_collator, start_full_node,
        StartCollatorParams, StartFullNodeParams,
    },
    cumulus_primitives_core::{
        relay_chain::{CollatorPair, Hash as PHash},
        ParaId,
    },
    cumulus_primitives_parachain_inherent::{
        MockValidationDataInherentDataProvider, MockXcmConfig,
    },
    cumulus_relay_chain_interface::RelayChainInterface,
    frame_benchmarking_cli::SUBSTRATE_REFERENCE_HARDWARE,
    futures::StreamExt,
    pallet_registrar_runtime_api::RegistrarApi,
    polkadot_cli::ProvideRuntimeApi,
    polkadot_service::Handle,
    sc_client_api::{AuxStore, Backend, BlockchainEvents, HeaderBackend, UsageProvider},
    sc_consensus::{BlockImport, ImportQueue},
    sc_executor::NativeElseWasmExecutor,
    sc_network::NetworkBlock,
    sc_network_sync::SyncingService,
    sc_service::{
        Configuration, Error as ServiceError, PartialComponents, TFullBackend, TFullClient,
        TaskManager,
    },
    sc_telemetry::{Telemetry, TelemetryHandle, TelemetryWorker, TelemetryWorkerHandle},
    sp_api::StorageProof,
    sp_consensus::SyncOracle,
    sp_keystore::SyncCryptoStorePtr,
    sp_state_machine::{Backend as StateBackend, StorageValue},
    std::{future::Future, str::FromStr, sync::Arc, time::Duration},
    substrate_prometheus_endpoint::Registry,
    tc_orchestrator_chain_interface::{
        OrchestratorChainError, OrchestratorChainInterface, OrchestratorChainResult,
    },
    test_runtime::{opaque::Block, AccountId, RuntimeApi},
};

type FullBackend = TFullBackend<Block>;
type MaybeSelectChain = Option<sc_consensus::LongestChain<FullBackend, Block>>;

/// Native executor type.
pub struct ParachainNativeExecutor;

impl sc_executor::NativeExecutionDispatch for ParachainNativeExecutor {
    type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;

    fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
        test_runtime::api::dispatch(method, data)
    }

    fn native_version() -> sc_executor::NativeVersion {
        test_runtime::native_version()
    }
}

type ParachainExecutor = NativeElseWasmExecutor<ParachainNativeExecutor>;

type ParachainClient = TFullClient<Block, RuntimeApi, ParachainExecutor>;

type ParachainBackend = TFullBackend<Block>;

type DevParachainBlockImport = OrchestratorParachainBlockImport<Arc<ParachainClient>>;

type ParachainBlockImport = TParachainBlockImport<Block, Arc<ParachainClient>, ParachainBackend>;

thread_local!(static TIMESTAMP: std::cell::RefCell<u64> = std::cell::RefCell::new(0));

/// Provide a mock duration starting at 0 in millisecond for timestamp inherent.
/// Each call will increment timestamp by slot_duration making Aura think time has passed.
struct MockTimestampInherentDataProvider;

/// Starts a `ServiceBuilder` for a full service.
///
/// Use this macro if you don't actually need the full service, but just the builder in order to
/// be able to perform chain operations.
pub fn new_partial(
    config: &Configuration,
) -> Result<
    PartialComponents<
        ParachainClient,
        ParachainBackend,
        MaybeSelectChain,
        sc_consensus::DefaultImportQueue<Block, ParachainClient>,
        sc_transaction_pool::FullPool<Block, ParachainClient>,
        (
            ParachainBlockImport,
            Option<Telemetry>,
            Option<TelemetryWorkerHandle>,
        ),
    >,
    sc_service::Error,
> {
    let telemetry = config
        .telemetry_endpoints
        .clone()
        .filter(|x| !x.is_empty())
        .map(|endpoints| -> Result<_, sc_telemetry::Error> {
            let worker = TelemetryWorker::new(16)?;
            let telemetry = worker.handle().new_telemetry(endpoints);
            Ok((worker, telemetry))
        })
        .transpose()?;

    let executor = ParachainExecutor::new(
        config.wasm_method,
        config.default_heap_pages,
        config.max_runtime_instances,
        config.runtime_cache_size,
    );

    let (client, backend, keystore_container, task_manager) =
        sc_service::new_full_parts::<Block, RuntimeApi, _>(
            config,
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
        config.transaction_pool.clone(),
        config.role.is_authority().into(),
        config.prometheus_registry(),
        task_manager.spawn_essential_handle(),
        client.clone(),
    );

    let block_import = ParachainBlockImport::new(client.clone(), backend.clone());
    let import_queue = build_import_queue(
        client.clone(),
        block_import.clone(),
        config,
        telemetry.as_ref().map(|telemetry| telemetry.handle()),
        &task_manager,
    )?;

    let maybe_select_chain = None;

    Ok(PartialComponents {
        backend,
        client,
        import_queue,
        keystore_container,
        task_manager,
        transaction_pool,
        select_chain: maybe_select_chain,
        other: (block_import, telemetry, telemetry_worker_handle),
    })
}

/// Starts a `ServiceBuilder` for a dev service.
pub fn new_partial_dev(
    config: &Configuration,
) -> Result<
    PartialComponents<
        ParachainClient,
        ParachainBackend,
        MaybeSelectChain,
        sc_consensus::DefaultImportQueue<Block, ParachainClient>,
        sc_transaction_pool::FullPool<Block, ParachainClient>,
        (
            DevParachainBlockImport,
            Option<Telemetry>,
            Option<TelemetryWorkerHandle>,
        ),
    >,
    sc_service::Error,
> {
    let telemetry = config
        .telemetry_endpoints
        .clone()
        .filter(|x| !x.is_empty())
        .map(|endpoints| -> Result<_, sc_telemetry::Error> {
            let worker = TelemetryWorker::new(16)?;
            let telemetry = worker.handle().new_telemetry(endpoints);
            Ok((worker, telemetry))
        })
        .transpose()?;

    let executor = ParachainExecutor::new(
        config.wasm_method,
        config.default_heap_pages,
        config.max_runtime_instances,
        config.runtime_cache_size,
    );

    let (client, backend, keystore_container, task_manager) =
        sc_service::new_full_parts::<Block, RuntimeApi, _>(
            config,
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
        config.transaction_pool.clone(),
        config.role.is_authority().into(),
        config.prometheus_registry(),
        task_manager.spawn_essential_handle(),
        client.clone(),
    );

    let block_import = DevParachainBlockImport::new(client.clone());
    let import_queue = build_manual_seal_import_queue(
        client.clone(),
        block_import.clone(),
        config,
        telemetry.as_ref().map(|telemetry| telemetry.handle()),
        &task_manager,
    )?;

    let maybe_select_chain = Some(sc_consensus::LongestChain::new(backend.clone()));

    Ok(PartialComponents {
        backend,
        client,
        import_queue,
        keystore_container,
        task_manager,
        transaction_pool,
        select_chain: maybe_select_chain,
        other: (block_import, telemetry, telemetry_worker_handle),
    })
}

/// Start a node with the given parachain `Configuration` and relay chain `Configuration`.
///
/// This is the actual implementation that is abstract over the executor and the runtime api.
#[sc_tracing::logging::prefix_logs_with("Orchestrator")]
async fn start_node_impl(
    orchestrator_config: Configuration,
    polkadot_config: Configuration,
    container_chain_config: Option<(ContainerChainCli, tokio::runtime::Handle)>,
    collator_options: CollatorOptions,
    para_id: ParaId,
    hwbench: Option<sc_sysinfo::HwBench>,
) -> sc_service::error::Result<(TaskManager, Arc<ParachainClient>)> {
    let parachain_config = prepare_node_config(orchestrator_config);

    let chain_type: sc_chain_spec::ChainType = parachain_config.chain_spec.chain_type();
    let relay_chain = crate::chain_spec::Extensions::try_get(&*parachain_config.chain_spec)
        .map(|e| e.relay_chain.clone())
        .ok_or_else(|| "Could not find relay_chain extension in chain-spec.")?;

    let params = new_partial(&parachain_config)?;
    let (block_import, mut telemetry, telemetry_worker_handle) = params.other;

    let client = params.client.clone();
    let backend = params.backend.clone();
    let mut task_manager = params.task_manager;

    let (relay_chain_interface, collator_key) = build_relay_chain_interface(
        polkadot_config,
        &parachain_config,
        telemetry_worker_handle,
        &mut task_manager,
        collator_options.clone(),
        hwbench.clone(),
    )
    .await
    .map_err(|e| sc_service::Error::Application(Box::new(e) as Box<_>))?;

    let force_authoring = parachain_config.force_authoring;
    let validator = parachain_config.role.is_authority();
    let prometheus_registry = parachain_config.prometheus_registry().cloned();
    let transaction_pool = params.transaction_pool.clone();
    let import_queue_service = params.import_queue.service();

    let (network, system_rpc_tx, tx_handler_controller, start_network, sync_service) =
        cumulus_client_service::build_network(cumulus_client_service::BuildNetworkParams {
            parachain_config: &parachain_config,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            spawn_handle: task_manager.spawn_handle(),
            import_queue: params.import_queue,
            para_id,
            relay_chain_interface: relay_chain_interface.clone(),
        })
        .await?;

    if parachain_config.offchain_worker.enabled {
        sc_service::build_offchain_workers(
            &parachain_config,
            task_manager.spawn_handle(),
            client.clone(),
            network.clone(),
        );
    }

    let rpc_builder = {
        let client = client.clone();
        let transaction_pool = transaction_pool.clone();

        Box::new(move |deny_unsafe, _| {
            let deps = crate::rpc::FullDeps {
                client: client.clone(),
                pool: transaction_pool.clone(),
                deny_unsafe,
                command_sink: None,
            };

            crate::rpc::create_full(deps).map_err(Into::into)
        })
    };

    sc_service::spawn_tasks(sc_service::SpawnTasksParams {
        rpc_builder,
        client: client.clone(),
        transaction_pool: transaction_pool.clone(),
        task_manager: &mut task_manager,
        config: parachain_config,
        keystore: params.keystore_container.sync_keystore(),
        backend: backend.clone(),
        network: network.clone(),
        system_rpc_tx,
        tx_handler_controller,
        telemetry: telemetry.as_mut(),
        sync_service: sync_service.clone(),
    })?;

    if let Some(hwbench) = hwbench {
        sc_sysinfo::print_hwbench(&hwbench);
        // Here you can check whether the hardware meets your chains' requirements. Putting a link
        // in there and swapping out the requirements for your own are probably a good idea. The
        // requirements for a para-chain are dictated by its relay-chain.
        if !SUBSTRATE_REFERENCE_HARDWARE.check_hardware(&hwbench) && validator {
            log::warn!(
                "⚠️  The hardware does not meet the minimal requirements for role 'Authority'."
            );
        }

        if let Some(ref mut telemetry) = telemetry {
            let telemetry_handle = telemetry.handle();
            task_manager.spawn_handle().spawn(
                "telemetry_hwbench",
                None,
                sc_sysinfo::initialize_hwbench_telemetry(telemetry_handle, hwbench),
            );
        }
    }

    let announce_block = {
        let sync_service = sync_service.clone();
        Arc::new(move |hash, data| sync_service.announce_block(hash, data))
    };

    let relay_chain_slot_duration = Duration::from_secs(6);

    let overseer_handle = relay_chain_interface
        .overseer_handle()
        .map_err(|e| sc_service::Error::Application(Box::new(e)))?;

    let orchestrator_chain_interface_builder = OrchestratorChainInProcessInterfaceBuilder {
        client: client.clone(),
        backend: backend.clone(),
        sync_oracle: sync_service.clone(),
        overseer_handle: overseer_handle.clone(),
    };

    let container_collator = container_chain_config.is_some();
    let sync_keystore = params.keystore_container.sync_keystore();

    // TODO: Investigate why CollateOn cannot be sent for two chains
    // Last one has priority apparently
    if validator && !container_collator {
        let parachain_consensus = build_consensus_orchestrator(
            client.clone(),
            block_import,
            prometheus_registry.as_ref(),
            telemetry.as_ref().map(|t| t.handle()),
            &task_manager,
            relay_chain_interface.clone(),
            transaction_pool,
            sync_service.clone(),
            params.keystore_container.sync_keystore(),
            force_authoring,
            para_id,
        )?;

        let spawner = task_manager.spawn_handle();
        let params = StartCollatorParams {
            para_id,
            block_status: client.clone(),
            announce_block,
            client: client.clone(),
            task_manager: &mut task_manager,
            relay_chain_interface: relay_chain_interface.clone(),
            spawner,
            parachain_consensus,
            import_queue: import_queue_service,
            collator_key: collator_key
                .clone()
                .expect("Command line arguments do not allow this. qed"),
            relay_chain_slot_duration,
            recovery_handle: Box::new(overseer_handle),
        };

        start_collator(params).await?;
    } else {
        let params = StartFullNodeParams {
            client: client.clone(),
            announce_block,
            task_manager: &mut task_manager,
            para_id,
            relay_chain_interface: relay_chain_interface.clone(),
            relay_chain_slot_duration,
            import_queue: import_queue_service,
            recovery_handle: Box::new(overseer_handle),
        };

        start_full_node(params)?;
    }

    start_network.start_network();

    if let Some((container_chain_cli, tokio_handle)) = container_chain_config {
        let orchestrator_client = client.clone();
        let container_chain_spawner = ContainerChainSpawner {
            orchestrator_chain_interface_builder,
            orchestrator_client,
            container_chain_cli,
            tokio_handle,
            chain_type,
            relay_chain,
            relay_chain_interface,
            collator_key,
            sync_keystore,
            orchestrator_para_id: para_id,
            validator,
            task_manager: &task_manager,
        };

        task_manager.spawn_handle().spawn(
            "container-chain-spawner",
            None,
            container_chain_spawner.spawn(),
        );
    }

    Ok((task_manager, client))
}

/// Struct with all the params needed to start a container chain node given the CLI arguments,
/// and creating the ChainSpec from on-chain data from the orchestrator chain.
struct ContainerChainSpawner<'a> {
    orchestrator_chain_interface_builder: OrchestratorChainInProcessInterfaceBuilder,
    orchestrator_client: Arc<ParachainClient>,
    container_chain_cli: ContainerChainCli,
    tokio_handle: tokio::runtime::Handle,
    chain_type: sc_chain_spec::ChainType,
    relay_chain: String,
    relay_chain_interface: Arc<dyn RelayChainInterface>,
    collator_key: Option<CollatorPair>,
    sync_keystore: SyncCryptoStorePtr,
    orchestrator_para_id: ParaId,
    validator: bool,
    // TODO: this could be a `SpawnEssentialTaskHandle`, but that type is private
    task_manager: &'a TaskManager,
}

impl<'a> ContainerChainSpawner<'a> {
    /// Try to start the container chain node. In case of error, this panics and stops the node.
    fn spawn(self) -> impl Future<Output = ()> {
        let ContainerChainSpawner {
            orchestrator_chain_interface_builder,
            orchestrator_client,
            mut container_chain_cli,
            tokio_handle,
            chain_type,
            relay_chain,
            relay_chain_interface,
            collator_key,
            sync_keystore,
            orchestrator_para_id,
            validator,
            task_manager,
        } = self;
        let spawn_cc_as_child_handle = task_manager.spawn_essential_handle();

        // This closure is used to emulate a try block, it enables using the `?` operator inside
        let try_closure = move || async move {
            let orchestrator_chain_interface = orchestrator_chain_interface_builder.build();
            // Preload genesis data from orchestrator chain storage.
            // The preload must finish before calling create_configuration, so any async operations
            // need to be awaited.

            // TODO: the orchestrator chain node may not be fully synced yet,
            // in that case we will be reading an old state.
            let orchestrator_chain_info = orchestrator_client.chain_info();
            log::info!(
                "Reading container chain genesis data from orchestrator chain at block #{} {}",
                orchestrator_chain_info.best_number,
                orchestrator_chain_info.best_hash,
            );
            let orchestrator_runtime_api = orchestrator_client.runtime_api();

            let container_chain_para_id = container_chain_cli
                .base
                .para_id
                .ok_or("missing --para-id CLI argument for container chain")?;

            let genesis_data = loop {
                let genesis_data = orchestrator_runtime_api
                    .genesis_data(
                        orchestrator_chain_info.best_hash,
                        container_chain_para_id.into(),
                    )
                    .expect("error")
                    .ok_or_else(|| {
                        format!(
                            "No genesis data registered for container chain id {}",
                            container_chain_para_id
                        )
                    });

                match genesis_data {
                    Ok(genesis_data) => break genesis_data,
                    Err(e) => {
                        // The genesis data does not exist, so we may want to wait for it to be created
                        // (for example when running tests), or we may want to simply panic
                        if container_chain_cli.base.wait_until_container_chain_exists {
                            log::debug!("{}", e);
                            log::debug!(
                                "Waiting for container chain {} to be registered...",
                                container_chain_para_id
                            );
                            tokio::time::sleep(std::time::Duration::from_secs(30)).await;
                            continue;
                        } else {
                            // Return error
                            return Err(e.into());
                        }
                    }
                }
            };

            container_chain_cli
                .preload_chain_spec_from_genesis_data(
                    container_chain_para_id,
                    genesis_data,
                    chain_type,
                    relay_chain,
                )
                .map_err(|e| format!("failed to create container chain chain spec from on chain genesis data: {}", e))?;

            log::info!(
                "Loaded chain spec for container chain {}",
                container_chain_para_id
            );

            let container_chain_cli_config = sc_cli::SubstrateCli::create_configuration(
                &container_chain_cli,
                &container_chain_cli,
                tokio_handle,
            )
            .map_err(|err| format!("Container chain argument error: {}", err))?;

            // Start container chain node
            let (mut container_chain_task_manager, _container_chain_client) =
                start_node_impl_container(
                    container_chain_cli_config,
                    relay_chain_interface.clone(),
                    orchestrator_chain_interface,
                    collator_key.clone(),
                    sync_keystore,
                    container_chain_para_id.into(),
                    orchestrator_para_id,
                    validator,
                )
                .await?;

            // Emulate task_manager.add_child by using task_manager.spawn_essential_task,
            // to make the parent task manager stop if the container chain task manager stops.
            // The reverse is also true, if the parent task manager stops, the container chain
            // task manager will also stop.
            spawn_cc_as_child_handle.spawn("container-chain-task-manager", None, async move {
                container_chain_task_manager
                    .future()
                    .await
                    .expect("container_chain_task_manager failed")
            });

            sc_service::error::Result::Ok(())
        };

        async {
            match try_closure().await {
                Ok(()) => {}
                Err(e) => {
                    panic!("Failed to start container chain node: {}", e);
                }
            }
        }
    }
}

// Log string that will be shown for the container chain: `[Container-2000]`.
// This needs to be a separate function because the `prefix_logs_with` macro
// has trouble parsing expressions.
fn container_log_str(para_id: ParaId) -> String {
    format!("Container-{}", para_id)
}

/// Start a node with the given parachain `Configuration` and relay chain `Configuration`.
///
/// This is the actual implementation that is abstract over the executor and the runtime api.
#[sc_tracing::logging::prefix_logs_with(container_log_str(para_id))]
async fn start_node_impl_container(
    parachain_config: Configuration,
    relay_chain_interface: Arc<dyn RelayChainInterface>,
    orchestrator_chain_interface: Arc<dyn OrchestratorChainInterface>,
    collator_key: Option<CollatorPair>,
    keystore: SyncCryptoStorePtr,
    para_id: ParaId,
    orchestrator_para_id: ParaId,
    collator: bool,
) -> sc_service::error::Result<(TaskManager, Arc<ParachainClient>)> {
    let parachain_config = prepare_node_config(parachain_config);
    let block_import;
    let mut telemetry;
    let client;
    let backend;
    let mut task_manager;
    let transaction_pool;
    let import_queue_service;
    let params_import_queue;
    {
        // Some fields of params are not `Send`, and that causes problems with async/await.
        // We take all the needed fields here inside a block to ensure that params
        // gets dropped before the first instance of `.await`.
        // Change this to use the syntax `PartialComponents { client, backend, .. } = params;`
        // when this issue is fixed:
        // https://github.com/rust-lang/rust/issues/104883
        let params = new_partial(&parachain_config)?;
        let (l_block_import, l_telemetry, _telemetry_worker_handle) = params.other;
        block_import = l_block_import;
        telemetry = l_telemetry;
        client = params.client.clone();
        backend = params.backend.clone();
        task_manager = params.task_manager;
        transaction_pool = params.transaction_pool.clone();
        import_queue_service = params.import_queue.service();
        params_import_queue = params.import_queue;
    }

    let spawn_handle = task_manager.spawn_handle();

    let force_authoring = parachain_config.force_authoring;
    let prometheus_registry = parachain_config.prometheus_registry().cloned();

    log::info!("are we collators? {:?}", collator);
    let (network, system_rpc_tx, tx_handler_controller, start_network, sync_service) =
        cumulus_client_service::build_network(cumulus_client_service::BuildNetworkParams {
            parachain_config: &parachain_config,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            spawn_handle,
            import_queue: params_import_queue,
            para_id,
            relay_chain_interface: relay_chain_interface.clone(),
        })
        .await?;

    if parachain_config.offchain_worker.enabled {
        sc_service::build_offchain_workers(
            &parachain_config,
            task_manager.spawn_handle(),
            client.clone(),
            network.clone(),
        );
    }

    let rpc_builder = {
        let client = client.clone();
        let transaction_pool = transaction_pool.clone();

        Box::new(move |deny_unsafe, _| {
            let deps = crate::rpc::FullDeps {
                client: client.clone(),
                pool: transaction_pool.clone(),
                deny_unsafe,
                command_sink: None,
            };

            crate::rpc::create_full(deps).map_err(Into::into)
        })
    };

    sc_service::spawn_tasks(sc_service::SpawnTasksParams {
        rpc_builder,
        client: client.clone(),
        transaction_pool: transaction_pool.clone(),
        task_manager: &mut task_manager,
        config: parachain_config,
        keystore: keystore.clone(),
        backend,
        network: network.clone(),
        system_rpc_tx,
        tx_handler_controller,
        telemetry: telemetry.as_mut(),
        sync_service: sync_service.clone(),
    })?;

    let announce_block = {
        let sync_service = sync_service.clone();
        Arc::new(move |hash, data| sync_service.announce_block(hash, data))
    };

    let relay_chain_slot_duration = Duration::from_secs(6);

    let overseer_handle = relay_chain_interface
        .overseer_handle()
        .map_err(|e| sc_service::Error::Application(Box::new(e)))?;

    if collator {
        let parachain_consensus = build_consensus_container(
            client.clone(),
            block_import,
            prometheus_registry.as_ref(),
            telemetry.as_ref().map(|t| t.handle()),
            &task_manager,
            relay_chain_interface.clone(),
            orchestrator_chain_interface.clone(),
            transaction_pool,
            sync_service,
            keystore,
            force_authoring,
            para_id,
            orchestrator_para_id,
        )?;

        let spawner = task_manager.spawn_handle();
        let params = StartCollatorParams {
            para_id,
            block_status: client.clone(),
            announce_block,
            client: client.clone(),
            task_manager: &mut task_manager,
            relay_chain_interface,
            spawner,
            parachain_consensus,
            import_queue: import_queue_service,
            collator_key: collator_key.expect("Command line arguments do not allow this. qed"),
            relay_chain_slot_duration,
            recovery_handle: Box::new(overseer_handle),
        };

        start_collator(params).await?;
    } else {
        let params = StartFullNodeParams {
            client: client.clone(),
            announce_block,
            task_manager: &mut task_manager,
            para_id,
            relay_chain_interface,
            relay_chain_slot_duration,
            import_queue: import_queue_service,
            recovery_handle: Box::new(overseer_handle),
        };

        start_full_node(params)?;
    }

    start_network.start_network();

    Ok((task_manager, client))
}

/// Build the import queue for the parachain runtime.
fn build_import_queue(
    client: Arc<ParachainClient>,
    block_import: ParachainBlockImport,
    config: &Configuration,
    telemetry: Option<TelemetryHandle>,
    task_manager: &TaskManager,
) -> Result<sc_consensus::DefaultImportQueue<Block, ParachainClient>, sc_service::Error> {
    let slot_duration = cumulus_client_consensus_aura::slot_duration(&*client)?;

    cumulus_client_consensus_aura::import_queue::<
        sp_consensus_aura::sr25519::AuthorityPair,
        _,
        _,
        _,
        _,
        _,
    >(cumulus_client_consensus_aura::ImportQueueParams {
        block_import,
        client,
        create_inherent_data_providers: move |_, _| async move {
            let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

            let slot =
				sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
					*timestamp,
					slot_duration,
				);

            Ok((slot, timestamp))
        },
        registry: config.prometheus_registry(),
        spawner: &task_manager.spawn_essential_handle(),
        telemetry,
    })
    .map_err(Into::into)
}

/// Build the import queue for the parachain runtime (manual seal).
fn build_manual_seal_import_queue(
    _client: Arc<ParachainClient>,
    block_import: DevParachainBlockImport,
    config: &Configuration,
    _telemetry: Option<TelemetryHandle>,
    task_manager: &TaskManager,
) -> Result<sc_consensus::DefaultImportQueue<Block, ParachainClient>, sc_service::Error> {
    Ok(sc_consensus_manual_seal::import_queue(
        Box::new(block_import.clone()),
        &task_manager.spawn_essential_handle(),
        config.prometheus_registry(),
    ))
}

fn build_consensus_container(
    client: Arc<ParachainClient>,
    block_import: ParachainBlockImport,
    prometheus_registry: Option<&Registry>,
    telemetry: Option<TelemetryHandle>,
    task_manager: &TaskManager,
    relay_chain_interface: Arc<dyn RelayChainInterface>,
    orchestrator_chain_interface: Arc<dyn OrchestratorChainInterface>,
    transaction_pool: Arc<sc_transaction_pool::FullPool<Block, ParachainClient>>,
    sync_oracle: Arc<SyncingService<Block>>,
    keystore: SyncCryptoStorePtr,
    force_authoring: bool,
    para_id: ParaId,
    orchestrator_para_id: ParaId,
) -> Result<Box<dyn ParachainConsensus<Block>>, sc_service::Error> {
    let slot_duration = cumulus_client_consensus_aura::slot_duration(&*client)?;

    let proposer_factory = sc_basic_authorship::ProposerFactory::with_proof_recording(
        task_manager.spawn_handle(),
        client.clone(),
        transaction_pool,
        prometheus_registry,
        telemetry.clone(),
    );

    let params = BuildAuraConsensusParams {
        proposer_factory,
        create_inherent_data_providers: move |_block_hash, (relay_parent, validation_data)| {
            let relay_chain_interface = relay_chain_interface.clone();
            let orchestrator_chain_interface = orchestrator_chain_interface.clone();

            async move {
                let parachain_inherent =
                    cumulus_primitives_parachain_inherent::ParachainInherentData::create_at(
                        relay_parent,
                        &relay_chain_interface,
                        &validation_data,
                        para_id,
                    )
                    .await;

                let authorities_noting_inherent =
                    tp_authorities_noting_inherent::ContainerChainAuthoritiesInherentData::create_at(
                        relay_parent,
                        &relay_chain_interface,
                        &orchestrator_chain_interface,
                        orchestrator_para_id,
                    )
                    .await;

                let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

                let slot =
						sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
							*timestamp,
							slot_duration,
						);

                let parachain_inherent = parachain_inherent.ok_or_else(|| {
                    Box::<dyn std::error::Error + Send + Sync>::from(
                        "Failed to create parachain inherent",
                    )
                })?;

                let authorities_noting_inherent = authorities_noting_inherent.ok_or_else(|| {
                    Box::<dyn std::error::Error + Send + Sync>::from(
                        "Failed to create authoritiesnoting inherent",
                    )
                })?;

                Ok((
                    slot,
                    timestamp,
                    parachain_inherent,
                    authorities_noting_inherent,
                ))
            }
        },
        block_import,
        para_client: client,
        backoff_authoring_blocks: Option::<()>::None,
        sync_oracle,
        keystore,
        force_authoring,
        slot_duration,
        // We got around 500ms for proposing
        block_proposal_slot_portion: SlotProportion::new(1f32 / 24f32),
        // And a maximum of 750ms if slots are skipped
        max_block_proposal_slot_portion: Some(SlotProportion::new(1f32 / 16f32)),
        telemetry,
    };

    Ok(AuraConsensus::build::<
        sp_consensus_aura::sr25519::AuthorityPair,
        _,
        _,
        _,
        _,
        _,
        _,
    >(params))
}

fn build_consensus_orchestrator(
    client: Arc<ParachainClient>,
    block_import: ParachainBlockImport,
    prometheus_registry: Option<&Registry>,
    telemetry: Option<TelemetryHandle>,
    task_manager: &TaskManager,
    relay_chain_interface: Arc<dyn RelayChainInterface>,
    transaction_pool: Arc<sc_transaction_pool::FullPool<Block, ParachainClient>>,
    sync_oracle: Arc<SyncingService<Block>>,
    keystore: SyncCryptoStorePtr,
    force_authoring: bool,
    para_id: ParaId,
) -> Result<Box<dyn ParachainConsensus<Block>>, sc_service::Error> {
    let slot_duration = cumulus_client_consensus_aura::slot_duration(&*client)?;

    let proposer_factory = sc_basic_authorship::ProposerFactory::with_proof_recording(
        task_manager.spawn_handle(),
        client.clone(),
        transaction_pool,
        prometheus_registry,
        telemetry.clone(),
    );
    let client_set_aside_for_cidp = client.clone();

    let params = BuildAuraConsensusParams {
        proposer_factory,
        create_inherent_data_providers: move |block_hash, (relay_parent, validation_data)| {
            let relay_chain_interface = relay_chain_interface.clone();
            let client_set_aside_for_cidp = client_set_aside_for_cidp.clone();
            async move {
                let parachain_inherent =
                    cumulus_primitives_parachain_inherent::ParachainInherentData::create_at(
                        relay_parent,
                        &relay_chain_interface,
                        &validation_data,
                        para_id,
                    )
                    .await;

                let para_ids = client_set_aside_for_cidp
                    .runtime_api()
                    .registered_paras(block_hash)?;
                let para_ids: Vec<_> = para_ids.into_iter().map(|x| x.into()).collect();
                let author_noting_inherent =
                    tp_author_noting_inherent::OwnParachainInherentData::create_at(
                        relay_parent,
                        &relay_chain_interface,
                        &para_ids,
                    )
                    .await;

                let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

                let slot =
						sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
							*timestamp,
							slot_duration,
						);

                let parachain_inherent = parachain_inherent.ok_or_else(|| {
                    Box::<dyn std::error::Error + Send + Sync>::from(
                        "Failed to create parachain inherent",
                    )
                })?;

                let author_noting_inherent = author_noting_inherent.ok_or_else(|| {
                    Box::<dyn std::error::Error + Send + Sync>::from(
                        "Failed to create author noting inherent",
                    )
                })?;

                Ok((slot, timestamp, parachain_inherent, author_noting_inherent))
            }
        },
        block_import,
        para_client: client,
        backoff_authoring_blocks: Option::<()>::None,
        sync_oracle,
        keystore,
        force_authoring,
        slot_duration,
        // We got around 500ms for proposing
        block_proposal_slot_portion: SlotProportion::new(1f32 / 24f32),
        // And a maximum of 750ms if slots are skipped
        max_block_proposal_slot_portion: Some(SlotProportion::new(1f32 / 16f32)),
        telemetry,
    };

    Ok(AuraConsensus::build::<
        sp_consensus_aura::sr25519::AuthorityPair,
        _,
        _,
        _,
        _,
        _,
        _,
    >(params))
}

/// Start a parachain node.
pub async fn start_parachain_node(
    parachain_config: Configuration,
    polkadot_config: Configuration,
    orchestrator_config: Option<(ContainerChainCli, tokio::runtime::Handle)>,
    collator_options: CollatorOptions,
    para_id: ParaId,
    hwbench: Option<sc_sysinfo::HwBench>,
) -> sc_service::error::Result<(TaskManager, Arc<ParachainClient>)> {
    start_node_impl(
        parachain_config,
        polkadot_config,
        orchestrator_config,
        collator_options,
        para_id,
        hwbench,
    )
    .await
}

pub const SOFT_DEADLINE_PERCENT: sp_runtime::Percent = sp_runtime::Percent::from_percent(100);

/// Builds a new development service. This service uses manual seal, and mocks
/// the parachain inherent.
pub fn new_dev(
    config: Configuration,
    _author_id: Option<AccountId>,
    sealing: Sealing,
    hwbench: Option<sc_sysinfo::HwBench>,
) -> Result<TaskManager, ServiceError> {
    use {
        async_io::Timer,
        futures::Stream,
        sc_consensus_manual_seal::{run_manual_seal, EngineCommand, ManualSealParams},
        sp_core::H256,
    };

    let sc_service::PartialComponents {
        client,
        backend,
        mut task_manager,
        import_queue,
        keystore_container,
        select_chain: maybe_select_chain,
        transaction_pool,
        other: (block_import, mut telemetry, _telemetry_worker_handle),
    } = new_partial_dev(&config)?;

    let (network, system_rpc_tx, tx_handler_controller, start_network, sync_service) =
        sc_service::build_network(sc_service::BuildNetworkParams {
            config: &config,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            spawn_handle: task_manager.spawn_handle(),
            import_queue,
            block_announce_validator_builder: None,
            warp_sync_params: None,
        })?;

    if config.offchain_worker.enabled {
        sc_service::build_offchain_workers(
            &config,
            task_manager.spawn_handle(),
            client.clone(),
            network.clone(),
        );
    }

    let prometheus_registry = config.prometheus_registry().cloned();
    let collator = config.role.is_authority();
    let mut command_sink = None;

    if collator {
        let mut env = sc_basic_authorship::ProposerFactory::new(
            task_manager.spawn_handle(),
            client.clone(),
            transaction_pool.clone(),
            prometheus_registry.as_ref(),
            telemetry.as_ref().map(|x| x.handle()),
        );
        env.set_soft_deadline(SOFT_DEADLINE_PERCENT);
        let commands_stream: Box<dyn Stream<Item = EngineCommand<H256>> + Send + Sync + Unpin> =
            match sealing {
                Sealing::Instant => {
                    Box::new(
                        // This bit cribbed from the implementation of instant seal.
                        transaction_pool
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
                        finalize: false,
                        parent_hash: None,
                        sender: None,
                    },
                )),
            };

        let select_chain = maybe_select_chain.expect(
            "`new_partial` builds a `LongestChainRule` when building dev service.\
				We specified the dev service when calling `new_partial`.\
				Therefore, a `LongestChainRule` is present. qed.",
        );

        let client_set_aside_for_cidp = client.clone();

        #[async_trait::async_trait]
        impl sp_inherents::InherentDataProvider for MockTimestampInherentDataProvider {
            async fn provide_inherent_data(
                &self,
                inherent_data: &mut sp_inherents::InherentData,
            ) -> Result<(), sp_inherents::Error> {
                TIMESTAMP.with(|x| {
                    *x.borrow_mut() += test_runtime::SLOT_DURATION;
                    inherent_data.put_data(sp_timestamp::INHERENT_IDENTIFIER, &*x.borrow())
                })
            }

            async fn try_handle_error(
                &self,
                _identifier: &sp_inherents::InherentIdentifier,
                _error: &[u8],
            ) -> Option<Result<(), sp_inherents::Error>> {
                // The pallet never reports error.
                None
            }
        }

        task_manager.spawn_essential_handle().spawn_blocking(
            "authorship_task",
            Some("block-authoring"),
            run_manual_seal(ManualSealParams {
                block_import,
                env,
                client: client.clone(),
                pool: transaction_pool.clone(),
                commands_stream,
                select_chain,
                consensus_data_provider: Some(Box::new(
                    sc_consensus_manual_seal::consensus::aura::AuraConsensusDataProvider::new(
                        client.clone(),
                    ),
                )),
                create_inherent_data_providers: move |block: H256, ()| {
                    let current_para_block = client_set_aside_for_cidp
                        .number(block)
                        .expect("Header lookup should succeed")
                        .expect("Header passed in as parent should be present in backend.");

                    let para_ids = client_set_aside_for_cidp
                        .runtime_api()
                        .registered_paras(block)
                        .expect("registered_paras runtime API should exist")
                        .into_iter()
                        .map(|x| x.into())
                        .collect();

                    let client_for_xcm = client_set_aside_for_cidp.clone();
                    async move {
                        //let time = sp_timestamp::InherentDataProvider::from_system_time();
                        let time = MockTimestampInherentDataProvider;
                        let mocked_parachain = MockValidationDataInherentDataProvider {
                            current_para_block,
                            relay_offset: 1000,
                            relay_blocks_per_para_block: 2,
                            // TODO: Recheck
                            para_blocks_per_relay_epoch: 10,
                            relay_randomness_config: (),
                            xcm_config: MockXcmConfig::new(
                                &*client_for_xcm,
                                block,
                                Default::default(),
                                Default::default(),
                            ),
                            raw_downward_messages: vec![],
                            raw_horizontal_messages: vec![],
                        };

                        let mocked_author_noting =
                            tp_author_noting_inherent::MockAuthorNotingInherentDataProvider {
                                current_para_block,
                                relay_offset: 1000,
                                relay_blocks_per_para_block: 2,
                                para_ids,
                                slots_per_para_block: 1,
                            };

                        Ok((time, mocked_parachain, mocked_author_noting))
                    }
                },
            }),
        );
    }

    let rpc_builder = {
        let client = client.clone();
        let transaction_pool = transaction_pool.clone();

        Box::new(move |deny_unsafe, _| {
            let deps = crate::rpc::FullDeps {
                client: client.clone(),
                pool: transaction_pool.clone(),
                deny_unsafe,
                command_sink: command_sink.clone(),
            };

            crate::rpc::create_full(deps).map_err(Into::into)
        })
    };

    let _rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
        rpc_builder,
        client: client.clone(),
        transaction_pool,
        task_manager: &mut task_manager,
        config,
        keystore: keystore_container.sync_keystore(),
        backend,
        network,
        system_rpc_tx,
        tx_handler_controller,
        telemetry: telemetry.as_mut(),
        sync_service,
    })?;

    if let Some(hwbench) = hwbench {
        sc_sysinfo::print_hwbench(&hwbench);

        if let Some(ref mut telemetry) = telemetry {
            let telemetry_handle = telemetry.handle();
            task_manager.spawn_handle().spawn(
                "telemetry_hwbench",
                None,
                sc_sysinfo::initialize_hwbench_telemetry(telemetry_handle, hwbench),
            );
        }
    }

    log::info!("Development Service Ready");

    start_network.start_network();

    Ok(task_manager)
}

/// Can be called for a `Configuration` to check if it is a configuration for
/// the orchestrator network.
pub trait IdentifyVariant {
    /// Returns `true` if this is a configuration for a dev network.
    fn is_dev(&self) -> bool;
}

impl IdentifyVariant for Box<dyn sc_service::ChainSpec> {
    fn is_dev(&self) -> bool {
        self.chain_type() == sc_chain_spec::ChainType::Development
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

/// Orchestrator Parachain Block import. We cannot use the one in cumulus as it overrides the best
/// chain selection rule
#[derive(Clone)]
pub struct OrchestratorParachainBlockImport<BI> {
    inner: BI,
}

impl<BI> OrchestratorParachainBlockImport<BI> {
    /// Create a new instance.
    pub fn new(inner: BI) -> Self {
        Self { inner }
    }
}

/// We simply rely on the inner
#[async_trait::async_trait]
impl<BI> BlockImport<Block> for OrchestratorParachainBlockImport<BI>
where
    BI: BlockImport<Block> + Send,
{
    type Error = BI::Error;
    type Transaction = BI::Transaction;

    async fn check_block(
        &mut self,
        block: sc_consensus::BlockCheckParams<Block>,
    ) -> Result<sc_consensus::ImportResult, Self::Error> {
        self.inner.check_block(block).await
    }

    async fn import_block(
        &mut self,
        params: sc_consensus::BlockImportParams<Block, Self::Transaction>,
    ) -> Result<sc_consensus::ImportResult, Self::Error> {
        let res = self.inner.import_block(params).await?;

        Ok(res)
    }
}

/// But we need to implement the ParachainBlockImportMarker trait to fullfil
impl<BI> ParachainBlockImportMarker for OrchestratorParachainBlockImport<BI> {}

/// Builder for a concrete relay chain interface, created from a full node. Builds
/// a [`RelayChainInProcessInterface`] to access relay chain data necessary for parachain operation.
///
/// The builder takes a [`polkadot_client::Client`]
/// that wraps a concrete instance. By using [`polkadot_client::ExecuteWithClient`]
/// the builder gets access to this concrete instance and instantiates a [`RelayChainInProcessInterface`] with it.
struct OrchestratorChainInProcessInterfaceBuilder {
    client: Arc<ParachainClient>,
    backend: Arc<FullBackend>,
    sync_oracle: Arc<dyn SyncOracle + Send + Sync>,
    overseer_handle: Handle,
}

impl OrchestratorChainInProcessInterfaceBuilder {
    pub fn build(self) -> Arc<dyn OrchestratorChainInterface> {
        Arc::new(OrchestratorChainInProcessInterface::new(
            self.client,
            self.backend,
            self.sync_oracle,
            self.overseer_handle,
        ))
    }
}

/// Provides an implementation of the [`RelayChainInterface`] using a local in-process relay chain node.
pub struct OrchestratorChainInProcessInterface<Client> {
    full_client: Arc<Client>,
    backend: Arc<FullBackend>,
    sync_oracle: Arc<dyn SyncOracle + Send + Sync>,
    overseer_handle: Handle,
}

impl<Client> OrchestratorChainInProcessInterface<Client> {
    /// Create a new instance of [`RelayChainInProcessInterface`]
    pub fn new(
        full_client: Arc<Client>,
        backend: Arc<FullBackend>,
        sync_oracle: Arc<dyn SyncOracle + Send + Sync>,
        overseer_handle: Handle,
    ) -> Self {
        Self {
            full_client,
            backend,
            sync_oracle,
            overseer_handle,
        }
    }
}

impl<T> Clone for OrchestratorChainInProcessInterface<T> {
    fn clone(&self) -> Self {
        Self {
            full_client: self.full_client.clone(),
            backend: self.backend.clone(),
            sync_oracle: self.sync_oracle.clone(),
            overseer_handle: self.overseer_handle.clone(),
        }
    }
}

#[async_trait::async_trait]
impl<Client> OrchestratorChainInterface for OrchestratorChainInProcessInterface<Client>
where
    Client: ProvideRuntimeApi<Block>
        + BlockchainEvents<Block>
        + AuxStore
        + UsageProvider<Block>
        + Sync
        + Send,
{
    async fn get_storage_by_key(
        &self,
        orchestrator_parent: PHash,
        key: &[u8],
    ) -> OrchestratorChainResult<Option<StorageValue>> {
        let state = self.backend.state_at(orchestrator_parent)?;
        state
            .storage(key)
            .map_err(OrchestratorChainError::GenericError)
    }

    async fn prove_read(
        &self,
        orchestrator_parent: PHash,
        relevant_keys: &Vec<Vec<u8>>,
    ) -> OrchestratorChainResult<StorageProof> {
        let state_backend = self.backend.state_at(orchestrator_parent)?;

        sp_state_machine::prove_read(state_backend, relevant_keys)
            .map_err(OrchestratorChainError::StateMachineError)
    }

    fn overseer_handle(&self) -> OrchestratorChainResult<Handle> {
        Ok(self.overseer_handle.clone())
    }
}
