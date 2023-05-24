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

//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

// std
use std::{sync::Arc, time::Duration};

use cumulus_client_cli::CollatorOptions;
// Local Runtime Types
use container_chain_template_frontier_runtime::{opaque::Block, RuntimeApi};

// Cumulus Imports
use {
    cumulus_client_consensus_aura::SlotProportion,
    cumulus_client_consensus_common::{
        ParachainBlockImport as TParachainBlockImport, ParachainConsensus,
    },
    cumulus_client_service::{
        build_relay_chain_interface, prepare_node_config, start_collator, start_full_node,
        StartCollatorParams, StartFullNodeParams,
    },
    cumulus_primitives_core::ParaId,
    cumulus_relay_chain_interface::RelayChainInterface,
    nimbus_primitives::NimbusPair,
    tc_consensus::{BuildOrchestratorAuraConsensusParams, OrchestratorAuraConsensus},
    crate::eth::FrontierBackend,
};

// Substrate Imports
use {
    frame_benchmarking_cli::SUBSTRATE_REFERENCE_HARDWARE,
    sc_consensus::ImportQueue,
    sc_executor::NativeElseWasmExecutor,
    sc_network::NetworkBlock,
    sc_network_sync::SyncingService,
    sc_service::{Configuration, PartialComponents, TFullBackend, TFullClient, TaskManager},
    sc_telemetry::{Telemetry, TelemetryHandle, TelemetryWorker, TelemetryWorkerHandle},
    sp_keystore::SyncCryptoStorePtr,
    substrate_prometheus_endpoint::Registry,
};

/// Native executor type.
pub struct ParachainNativeExecutor;

impl sc_executor::NativeExecutionDispatch for ParachainNativeExecutor {
    type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;

    fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
        container_chain_template_frontier_runtime::api::dispatch(method, data)
    }

    fn native_version() -> sc_executor::NativeVersion {
        container_chain_template_frontier_runtime::native_version()
    }
}

type ParachainExecutor = NativeElseWasmExecutor<ParachainNativeExecutor>;

type ParachainClient = TFullClient<Block, RuntimeApi, ParachainExecutor>;

type ParachainBackend = TFullBackend<Block>;

type ParachainBlockImport = TParachainBlockImport<Block, Arc<ParachainClient>, ParachainBackend>;

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
        (),
        sc_consensus::DefaultImportQueue<Block, ParachainClient>,
        sc_transaction_pool::FullPool<Block, ParachainClient>,
        (
            ParachainBlockImport,
            Option<Telemetry>,
            Option<TelemetryWorkerHandle>,
            Arc<FrontierBackend>,
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

    let import_queue = nimbus_consensus::import_queue(
        client.clone(),
        block_import.clone(),
        move |_, _| async move {
            let time = sp_timestamp::InherentDataProvider::from_system_time();

            Ok((time,))
        },
        &task_manager.spawn_essential_handle(),
        config.prometheus_registry(),
        false,
    )?;

    Ok(PartialComponents {
        backend,
        client,
        import_queue,
        keystore_container,
        task_manager,
        transaction_pool,
        select_chain: (),
        other: (block_import, telemetry, telemetry_worker_handle),
    })
}

/// Start a node with the given parachain `Configuration` and relay chain `Configuration`.
///
/// This is the actual implementation that is abstract over the executor and the runtime api.
#[sc_tracing::logging::prefix_logs_with("Parachain")]
async fn start_node_impl(
    parachain_config: Configuration,
    polkadot_config: Configuration,
    collator_options: CollatorOptions,
    para_id: ParaId,
    hwbench: Option<sc_sysinfo::HwBench>,
) -> sc_service::error::Result<(TaskManager, Arc<ParachainClient>)> {
    let parachain_config = prepare_node_config(parachain_config);

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
        backend,
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
    let overseer_handle = relay_chain_interface
        .overseer_handle()
        .map_err(|e| sc_service::Error::Application(Box::new(e)))?;

    let announce_block = {
        let sync_service = sync_service.clone();
        Arc::new(move |hash, data| sync_service.announce_block(hash, data))
    };

    let relay_chain_slot_duration = Duration::from_secs(6);

    if validator {
        let parachain_consensus = build_consensus(
            client.clone(),
            block_import,
            prometheus_registry.as_ref(),
            telemetry.as_ref().map(|t| t.handle()),
            &task_manager,
            relay_chain_interface.clone(),
            transaction_pool,
            sync_service,
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

fn build_consensus(
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

    let params = BuildOrchestratorAuraConsensusParams {
        proposer_factory,
        create_inherent_data_providers: move |_, (relay_parent, validation_data)| {
            let relay_chain_interface = relay_chain_interface.clone();
            async move {
                let parachain_inherent =
                    cumulus_primitives_parachain_inherent::ParachainInherentData::create_at(
                        relay_parent,
                        &relay_chain_interface,
                        &validation_data,
                        para_id,
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
                Ok((slot, timestamp, parachain_inherent))
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

    Ok(OrchestratorAuraConsensus::build::<
        NimbusPair,
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
    collator_options: CollatorOptions,
    para_id: ParaId,
    hwbench: Option<sc_sysinfo::HwBench>,
) -> sc_service::error::Result<(TaskManager, Arc<ParachainClient>)> {
    start_node_impl(
        parachain_config,
        polkadot_config,
        collator_options,
        para_id,
        hwbench,
    )
    .await
}

use crate::client::RuntimeApiCollection;
use crate::client::FullBackend;
use sp_api::ConstructRuntimeApi;
use crate::client::FullClient;
use sc_client_api::{BlockchainEvents, StateBackendFor};
use crate::eth::EthConfiguration;
use sc_service::Error as ServiceError;
use crate::eth::new_frontier_partial;
use crate::eth::FrontierPartialComponents;
use crate::eth::spawn_frontier_tasks;

/// Builds a new service for a full client.
pub fn new_full<RuntimeApi, Executor>(
	mut config: Configuration,
	eth_config: EthConfiguration,
) -> Result<TaskManager, ServiceError>
where
	RuntimeApi: ConstructRuntimeApi<Block, FullClient<RuntimeApi, Executor>>,
	RuntimeApi: Send + Sync + 'static,
	RuntimeApi::RuntimeApi:
		RuntimeApiCollection<StateBackend = StateBackendFor<FullBackend, Block>>,
	Executor: sc_executor::NativeExecutionDispatch + 'static,
{
	let PartialComponents {
		client,
		backend,
		mut task_manager,
		import_queue,
		keystore_container,
		select_chain,
		transaction_pool,
		other: (mut telemetry, block_import, grandpa_link, frontier_backend),
	} = new_partial(&config, &eth_config)?;

	let FrontierPartialComponents {
		filter_pool,
		fee_history_cache,
		fee_history_cache_limit,
	} = new_frontier_partial(&eth_config)?;

	let (network, system_rpc_tx, tx_handler_controller, network_starter, sync_service) =
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

	let role = config.role.clone();
	let force_authoring = config.force_authoring;
	let name = config.network.node_name.clone();
	let enable_grandpa = false;
	let prometheus_registry = config.prometheus_registry().cloned();

	// Channel for the rpc handler to communicate with the authorship task.
	let (command_sink, commands_stream) = mpsc::channel(1000);

	// Sinks for pubsub notifications.
	// Everytime a new subscription is created, a new mpsc channel is added to the sink pool.
	// The MappingSyncWorker sends through the channel on block import and the subscription emits a notification to the subscriber on receiving a message through this channel.
	// This way we avoid race conditions when using native substrate block import notification stream.
	let pubsub_notification_sinks: fc_mapping_sync::EthereumBlockNotificationSinks<
		fc_mapping_sync::EthereumBlockNotification<Block>,
	> = Default::default();
	let pubsub_notification_sinks = Arc::new(pubsub_notification_sinks);

	// for ethereum-compatibility rpc.
	config.rpc_id_provider = Some(Box::new(fc_rpc::EthereumSubIdProvider));
	let overrides = crate::rpc::overrides_handle(client.clone());
	let eth_rpc_params = crate::rpc::EthDeps {
		client: client.clone(),
		pool: transaction_pool.clone(),
		graph: transaction_pool.pool().clone(),
		converter: Some(TransactionConverter),
		is_authority: config.role.is_authority(),
		enable_dev_signer: eth_config.enable_dev_signer,
		network: network.clone(),
		sync: sync_service.clone(),
		frontier_backend: frontier_backend.clone(),
		overrides: overrides.clone(),
		block_data_cache: Arc::new(fc_rpc::EthBlockDataCacheTask::new(
			task_manager.spawn_handle(),
			overrides.clone(),
			eth_config.eth_log_block_cache,
			eth_config.eth_statuses_cache,
			prometheus_registry.clone(),
		)),
		filter_pool: filter_pool.clone(),
		max_past_logs: eth_config.max_past_logs,
		fee_history_cache: fee_history_cache.clone(),
		fee_history_cache_limit,
		execute_gas_limit_multiplier: eth_config.execute_gas_limit_multiplier,
		forced_parent_hashes: None,
	};

	let rpc_builder = {
		let client = client.clone();
		let pool = transaction_pool.clone();
		let pubsub_notification_sinks = pubsub_notification_sinks.clone();

		Box::new(move |deny_unsafe, subscription_task_executor| {
			let deps = crate::rpc::FullDeps {
				client: client.clone(),
				pool: pool.clone(),
				deny_unsafe,
				command_sink:None,
				eth: eth_rpc_params.clone(),
			};

			crate::rpc::create_full(
				deps,
				subscription_task_executor,
				pubsub_notification_sinks.clone(),
			)
			.map_err(Into::into)
		})
	};

	let _rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
		config,
		client: client.clone(),
		backend: backend.clone(),
		task_manager: &mut task_manager,
		keystore: keystore_container.sync_keystore(),
		transaction_pool: transaction_pool.clone(),
		rpc_builder,
		network: network.clone(),
		system_rpc_tx,
		tx_handler_controller,
		sync_service: sync_service.clone(),
		telemetry: telemetry.as_mut(),
	})?;

	spawn_frontier_tasks(
		&task_manager,
		client.clone(),
		backend,
		frontier_backend,
		filter_pool,
		overrides,
		fee_history_cache,
		fee_history_cache_limit,
		sync_service.clone(),
		pubsub_notification_sinks,
	);

	network_starter.start_network();
	Ok(task_manager)
}
