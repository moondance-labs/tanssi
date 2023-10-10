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

use {
    cumulus_client_consensus_common::ParachainBlockImport,
    sc_executor::{HeapAllocStrategy, WasmExecutor, DEFAULT_HEAP_ALLOC_STRATEGY},
    sc_network::config::FullNetworkConfiguration,
};
// std
use futures::FutureExt;
use sc_client_api::Backend;
use sc_transaction_pool_api::OffchainTransactionPoolFactory;
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    time::Duration,
};
use {
    cumulus_client_cli::CollatorOptions,
    cumulus_primitives_parachain_inherent::{
        MockValidationDataInherentDataProvider, MockXcmConfig,
    },
    fc_consensus::FrontierBlockImport,
    nimbus_primitives::NimbusId,
    sp_consensus_aura::SlotDuration,
    sp_core::Pair,
};
// Local Runtime Types
use {
    container_chain_template_frontier_runtime::{opaque::Block, RuntimeApi},
    futures::StreamExt,
};

// Cumulus Imports
#[allow(deprecated)]
use {
    cumulus_client_service::{
        build_relay_chain_interface, prepare_node_config, start_full_node, CollatorSybilResistance,
        StartFullNodeParams,
    },
    cumulus_primitives_core::ParaId,
    cumulus_relay_chain_interface::RelayChainInterface,
};

// Substrate Imports
use {
    fc_db::DatabaseSource,
    fc_rpc_core::types::{FeeHistoryCache, FilterPool},
    sc_consensus::ImportQueue,
    sc_executor::NativeElseWasmExecutor,
    sc_network::NetworkBlock,
    sc_service::{Configuration, PartialComponents, TFullBackend, TFullClient, TaskManager},
    sc_telemetry::{Telemetry, TelemetryWorker, TelemetryWorkerHandle},
};

/// Native executor type.
use crate::client::TemplateRuntimeExecutor;

pub type ParachainExecutor = NativeElseWasmExecutor<TemplateRuntimeExecutor>;

type ParachainClient = TFullClient<Block, RuntimeApi, ParachainExecutor>;

type ParachainBackend = TFullBackend<Block>;

type MaybeSelectChain = Option<sc_consensus::LongestChain<ParachainBackend, Block>>;

pub fn frontier_database_dir(config: &Configuration, path: &str) -> std::path::PathBuf {
    let config_dir = config
        .base_path
        .config_dir(config.chain_spec.id())
        .join("frontier")
        .join(path);

    config_dir
}

// TODO This is copied from frontier. It should be imported instead after
// https://github.com/paritytech/frontier/issues/333 is solved
pub fn open_frontier_backend<C>(
    client: Arc<C>,
    config: &Configuration,
) -> Result<fc_db::kv::Backend<Block>, String>
where
    C: sp_blockchain::HeaderBackend<Block>,
{
    fc_db::kv::Backend::<Block>::new(
        client,
        &fc_db::kv::DatabaseSettings {
            source: match config.database {
                DatabaseSource::RocksDb { .. } => DatabaseSource::RocksDb {
                    path: frontier_database_dir(config, "db"),
                    cache_size: 0,
                },
                DatabaseSource::ParityDb { .. } => DatabaseSource::ParityDb {
                    path: frontier_database_dir(config, "paritydb"),
                },
                DatabaseSource::Auto { .. } => DatabaseSource::Auto {
                    rocksdb_path: frontier_database_dir(config, "db"),
                    paritydb_path: frontier_database_dir(config, "paritydb"),
                    cache_size: 0,
                },
                _ => {
                    return Err("Supported db sources: `rocksdb` | `paritydb` | `auto`".to_string())
                }
            },
        },
    )
}

thread_local!(static TIMESTAMP: std::cell::RefCell<u64> = std::cell::RefCell::new(0));

/// Provide a mock duration starting at 0 in millisecond for timestamp inherent.
/// Each call will increment timestamp by slot_duration making Aura think time has passed.
struct MockTimestampInherentDataProvider;

/// Starts a `ServiceBuilder` for a full service.
///
/// Use this macro if you don't actually need the full service, but just the builder in order to
/// be able to perform chain operations.
pub fn new_partial(
    config: &mut Configuration,
    dev_service: bool,
) -> Result<
    PartialComponents<
        ParachainClient,
        ParachainBackend,
        MaybeSelectChain,
        sc_consensus::DefaultImportQueue<Block>,
        sc_transaction_pool::FullPool<Block, ParachainClient>,
        (
            ParachainBlockImport<
                Block,
                FrontierBlockImport<Block, Arc<ParachainClient>, ParachainClient>,
                ParachainBackend,
            >,
            Option<FilterPool>,
            Option<Telemetry>,
            Option<TelemetryWorkerHandle>,
            fc_db::Backend<Block>,
            FeeHistoryCache,
        ),
    >,
    sc_service::Error,
> {
    // Use ethereum style for subscription ids
    config.rpc_id_provider = Some(Box::new(fc_rpc::EthereumSubIdProvider));

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

    // Default runtime_cache_size is 2
    // For now we can work with this, but it will likely need
    // to change once we start having runtime_cache_sizes, or
    // run nodes with the maximum for this value
    let heap_pages = config
        .default_heap_pages
        .map_or(DEFAULT_HEAP_ALLOC_STRATEGY, |h| HeapAllocStrategy::Static {
            extra_pages: h as _,
        });

    let wasm = WasmExecutor::builder()
        .with_execution_method(config.wasm_method)
        .with_onchain_heap_alloc_strategy(heap_pages)
        .with_offchain_heap_alloc_strategy(heap_pages)
        .with_max_runtime_instances(config.max_runtime_instances)
        .with_runtime_cache_size(config.runtime_cache_size)
        .build();

    let executor = ParachainExecutor::new_with_wasm_executor(wasm);

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

    let maybe_select_chain = if dev_service {
        Some(sc_consensus::LongestChain::new(backend.clone()))
    } else {
        None
    };

    let transaction_pool = sc_transaction_pool::BasicPool::new_full(
        config.transaction_pool.clone(),
        config.role.is_authority().into(),
        config.prometheus_registry(),
        task_manager.spawn_essential_handle(),
        client.clone(),
    );

    let filter_pool: Option<FilterPool> = Some(Arc::new(Mutex::new(BTreeMap::new())));
    let fee_history_cache: FeeHistoryCache = Arc::new(Mutex::new(BTreeMap::new()));

    let frontier_backend = fc_db::Backend::KeyValue(open_frontier_backend(client.clone(), config)?);

    let frontier_block_import = FrontierBlockImport::new(client.clone(), client.clone());

    let parachain_block_import = cumulus_client_consensus_common::ParachainBlockImport::new(
        frontier_block_import,
        backend.clone(),
    );

    let import_queue = nimbus_consensus::import_queue(
        client.clone(),
        parachain_block_import.clone(),
        move |_, _| async move {
            let time = sp_timestamp::InherentDataProvider::from_system_time();

            Ok((time,))
        },
        &task_manager.spawn_essential_handle(),
        config.prometheus_registry(),
        !dev_service,
    )?;

    Ok(PartialComponents {
        backend,
        client,
        import_queue,
        keystore_container,
        task_manager,
        transaction_pool,
        select_chain: maybe_select_chain,
        other: (
            parachain_block_import,
            filter_pool,
            telemetry,
            telemetry_worker_handle,
            frontier_backend,
            fee_history_cache,
        ),
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
    rpc_config: crate::cli::RpcConfig,
    hwbench: Option<sc_sysinfo::HwBench>,
) -> sc_service::error::Result<(TaskManager, Arc<ParachainClient>)> {
    let mut parachain_config = prepare_node_config(parachain_config);

    let params = new_partial(&mut parachain_config, false)?;
    let (
        _block_import,
        filter_pool,
        mut telemetry,
        telemetry_worker_handle,
        frontier_backend,
        fee_history_cache,
    ) = params.other;

    let client = params.client.clone();
    let backend = params.backend.clone();
    let mut task_manager = params.task_manager;

    let (relay_chain_interface, _collator_key) = build_relay_chain_interface(
        polkadot_config,
        &parachain_config,
        telemetry_worker_handle,
        &mut task_manager,
        collator_options.clone(),
        hwbench.clone(),
    )
    .await
    .map_err(|e| sc_service::Error::Application(Box::new(e) as Box<_>))?;

    let transaction_pool = params.transaction_pool.clone();
    let import_queue_service = params.import_queue.service();
    let prometheus_registry = parachain_config.prometheus_registry().cloned();
    let net_config = FullNetworkConfiguration::new(&parachain_config.network);

    let (network, system_rpc_tx, tx_handler_controller, start_network, sync_service) =
        cumulus_client_service::build_network(cumulus_client_service::BuildNetworkParams {
            parachain_config: &parachain_config,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            spawn_handle: task_manager.spawn_handle(),
            import_queue: params.import_queue,
            para_id,
            relay_chain_interface: relay_chain_interface.clone(),
            net_config,
            sybil_resistance_level: CollatorSybilResistance::Resistant,
        })
        .await?;

    let overrides = crate::rpc::overrides_handle(client.clone());
    let fee_history_limit = rpc_config.fee_history_limit;

    let pubsub_notification_sinks: fc_mapping_sync::EthereumBlockNotificationSinks<
        fc_mapping_sync::EthereumBlockNotification<Block>,
    > = Default::default();
    let pubsub_notification_sinks = Arc::new(pubsub_notification_sinks);

    crate::rpc::spawn_essential_tasks(crate::rpc::SpawnTasksParams {
        task_manager: &task_manager,
        client: client.clone(),
        substrate_backend: backend.clone(),
        frontier_backend: frontier_backend.clone(),
        filter_pool: filter_pool.clone(),
        overrides: overrides.clone(),
        fee_history_limit,
        fee_history_cache: fee_history_cache.clone(),
        sync_service: sync_service.clone(),
        pubsub_notification_sinks: pubsub_notification_sinks.clone(),
    });

    if parachain_config.offchain_worker.enabled {
        task_manager.spawn_handle().spawn(
            "offchain-workers-runner",
            "offchain-work",
            sc_offchain::OffchainWorkers::new(sc_offchain::OffchainWorkerOptions {
                runtime_api_provider: client.clone(),
                keystore: Some(params.keystore_container.keystore()),
                offchain_db: backend.offchain_storage(),
                transaction_pool: Some(OffchainTransactionPoolFactory::new(
                    transaction_pool.clone(),
                )),
                network_provider: network.clone(),
                is_validator: parachain_config.role.is_authority(),
                enable_http_requests: false,
                custom_extensions: move |_| vec![],
            })
            .run(client.clone(), task_manager.spawn_handle())
            .boxed(),
        );
    }

    let block_data_cache = Arc::new(fc_rpc::EthBlockDataCacheTask::new(
        task_manager.spawn_handle(),
        overrides.clone(),
        rpc_config.eth_log_block_cache,
        rpc_config.eth_statuses_cache,
        prometheus_registry.clone(),
    ));

    let rpc_builder = {
        let client = client.clone();
        let pool = transaction_pool.clone();
        let pubsub_notification_sinks = pubsub_notification_sinks;
        let network = network.clone();
        let sync = sync_service.clone();
        let filter_pool = filter_pool.clone();
        let frontier_backend = frontier_backend.clone();
        let backend = backend.clone();
        let max_past_logs = rpc_config.max_past_logs;
        let overrides = overrides;
        let fee_history_cache = fee_history_cache.clone();
        let block_data_cache = block_data_cache;

        move |deny_unsafe, subscription_task_executor| {
            let deps = crate::rpc::FullDeps {
                backend: backend.clone(),
                client: client.clone(),
                deny_unsafe,
                filter_pool: filter_pool.clone(),
                frontier_backend: match frontier_backend.clone() {
                    fc_db::Backend::KeyValue(b) => Arc::new(b),
                    fc_db::Backend::Sql(b) => Arc::new(b),
                },
                graph: pool.pool().clone(),
                pool: pool.clone(),
                max_past_logs,
                fee_history_limit,
                fee_history_cache: fee_history_cache.clone(),
                network: network.clone(),
                sync: sync.clone(),
                block_data_cache: block_data_cache.clone(),
                overrides: overrides.clone(),
                is_authority: false,
                command_sink: None,
                xcm_senders: None,
            };
            crate::rpc::create_full(
                deps,
                subscription_task_executor,
                pubsub_notification_sinks.clone(),
            )
            .map_err(Into::into)
        }
    };

    sc_service::spawn_tasks(sc_service::SpawnTasksParams {
        rpc_builder: Box::new(rpc_builder),
        client: client.clone(),
        transaction_pool: transaction_pool.clone(),
        task_manager: &mut task_manager,
        config: parachain_config,
        keystore: params.keystore_container.keystore(),
        backend,
        network,
        system_rpc_tx,
        tx_handler_controller,
        telemetry: telemetry.as_mut(),
        sync_service: sync_service.clone(),
    })?;

    let overseer_handle = relay_chain_interface
        .overseer_handle()
        .map_err(|e| sc_service::Error::Application(Box::new(e)))?;

    let announce_block = {
        let sync_service = sync_service.clone();
        Arc::new(move |hash, data| sync_service.announce_block(hash, data))
    };

    let relay_chain_slot_duration = Duration::from_secs(6);
    let params = StartFullNodeParams {
        client: client.clone(),
        announce_block,
        task_manager: &mut task_manager,
        para_id,
        relay_chain_interface,
        relay_chain_slot_duration,
        import_queue: import_queue_service,
        recovery_handle: Box::new(overseer_handle),
        sync_service,
    };

    // TODO: change for async backing
    #[allow(deprecated)]
    start_full_node(params)?;

    start_network.start_network();

    Ok((task_manager, client))
}

/// Start a parachain node.
pub async fn start_parachain_node(
    parachain_config: Configuration,
    polkadot_config: Configuration,
    collator_options: CollatorOptions,
    para_id: ParaId,
    rpc_config: crate::cli::RpcConfig,
    hwbench: Option<sc_sysinfo::HwBench>,
) -> sc_service::error::Result<(TaskManager, Arc<ParachainClient>)> {
    start_node_impl(
        parachain_config,
        polkadot_config,
        collator_options,
        para_id,
        rpc_config,
        hwbench,
    )
    .await
}

/// Helper function to generate a crypto pair from seed
fn get_aura_id_from_seed(seed: &str) -> NimbusId {
    sp_core::sr25519::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
        .into()
}

use {sp_blockchain::HeaderBackend, std::str::FromStr};
/// Builds a new development service. This service uses manual seal, and mocks
/// the parachain inherent.
pub async fn start_dev_node(
    mut config: Configuration,
    sealing: Sealing,
    rpc_config: crate::cli::RpcConfig,
    para_id: ParaId,
    hwbench: Option<sc_sysinfo::HwBench>,
) -> Result<TaskManager, sc_service::error::Error> {
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
        other:
            (
                block_import,
                filter_pool,
                mut telemetry,
                _telemetry_worker_handle,
                frontier_backend,
                fee_history_cache,
            ),
    } = new_partial(&mut config, true)?;

    let net_config = FullNetworkConfiguration::new(&config.network);

    let (network, system_rpc_tx, tx_handler_controller, network_starter, sync_service) =
        sc_service::build_network(sc_service::BuildNetworkParams {
            config: &config,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            spawn_handle: task_manager.spawn_handle(),
            import_queue,
            block_announce_validator_builder: None,
            warp_sync_params: None,
            net_config,
        })?;

    if config.offchain_worker.enabled {
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
                network_provider: network.clone(),
                is_validator: config.role.is_authority(),
                enable_http_requests: false,
                custom_extensions: move |_| vec![],
            })
            .run(client.clone(), task_manager.spawn_handle())
            .boxed(),
        );
    }

    let prometheus_registry = config.prometheus_registry().cloned();
    let overrides = crate::rpc::overrides_handle(client.clone());
    let fee_history_limit = rpc_config.fee_history_limit;
    let collator = config.role.is_authority();
    let mut command_sink = None;
    let mut xcm_senders = None;

    let pubsub_notification_sinks: fc_mapping_sync::EthereumBlockNotificationSinks<
        fc_mapping_sync::EthereumBlockNotification<Block>,
    > = Default::default();
    let pubsub_notification_sinks = Arc::new(pubsub_notification_sinks);

    if collator {
        let env = sc_basic_authorship::ProposerFactory::with_proof_recording(
            task_manager.spawn_handle(),
            client.clone(),
            transaction_pool.clone(),
            prometheus_registry.as_ref(),
            telemetry.as_ref().map(|x| x.handle()),
        );

        // Create channels for mocked XCM messages.
        let (downward_xcm_sender, downward_xcm_receiver) = flume::bounded::<Vec<u8>>(100);
        let (hrmp_xcm_sender, hrmp_xcm_receiver) = flume::bounded::<(ParaId, Vec<u8>)>(100);
        xcm_senders = Some((downward_xcm_sender, hrmp_xcm_sender));

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
                Sealing::Interval(millis) => Box::new(StreamExt::map(
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
                    *x.borrow_mut() += container_chain_template_frontier_runtime::SLOT_DURATION;
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

        let authorities = vec![get_aura_id_from_seed("alice")];

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
				consensus_data_provider: Some(Box::new(tc_consensus::ContainerManualSealAuraConsensusDataProvider::new(
                    client.clone(),
                    keystore_container.keystore(),
                    SlotDuration::from_millis(container_chain_template_frontier_runtime::SLOT_DURATION),
                    authorities.clone(),
                ))),
				create_inherent_data_providers: move |block: H256, ()| {
					let current_para_block = client_set_aside_for_cidp
						.number(block)
						.expect("Header lookup should succeed")
						.expect("Header passed in as parent should be present in backend.");

                    let client_for_xcm = client_set_aside_for_cidp.clone();
                    let authorities_for_cidp = authorities.clone();

                    let downward_xcm_receiver = downward_xcm_receiver.clone();
                    let hrmp_xcm_receiver = hrmp_xcm_receiver.clone();

					async move {
                        let mocked_authorities_noting =
                            ccp_authorities_noting_inherent::MockAuthoritiesNotingInherentDataProvider {
                                current_para_block,
                                relay_offset: 1000,
                                relay_blocks_per_para_block: 2,
                                orchestrator_para_id: crate::chain_spec::ORCHESTRATOR,
                                container_para_id: para_id,
                                authorities: authorities_for_cidp
                        };

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
                                para_id,
                                Default::default(),
                            ),
                            raw_downward_messages: downward_xcm_receiver.drain().collect(),
                            raw_horizontal_messages: hrmp_xcm_receiver.drain().collect(),
                            additional_key_values: Some(mocked_authorities_noting.get_key_values())
                        };

						Ok((time, mocked_parachain, mocked_authorities_noting))
					}
				},
			}),
		);
    }

    crate::rpc::spawn_essential_tasks(crate::rpc::SpawnTasksParams {
        task_manager: &task_manager,
        client: client.clone(),
        substrate_backend: backend.clone(),
        frontier_backend: frontier_backend.clone(),
        filter_pool: filter_pool.clone(),
        overrides: overrides.clone(),
        fee_history_limit,
        fee_history_cache: fee_history_cache.clone(),
        sync_service: sync_service.clone(),
        pubsub_notification_sinks: pubsub_notification_sinks.clone(),
    });

    let block_data_cache = Arc::new(fc_rpc::EthBlockDataCacheTask::new(
        task_manager.spawn_handle(),
        overrides.clone(),
        rpc_config.eth_log_block_cache,
        rpc_config.eth_statuses_cache,
        prometheus_registry,
    ));

    let rpc_builder = {
        let client = client.clone();
        let pool = transaction_pool.clone();
        let pubsub_notification_sinks = pubsub_notification_sinks;
        let network = network.clone();
        let sync = sync_service.clone();
        let filter_pool = filter_pool;
        let frontier_backend = frontier_backend;
        let backend = backend.clone();
        let max_past_logs = rpc_config.max_past_logs;
        let overrides = overrides;
        let fee_history_cache = fee_history_cache;
        let block_data_cache = block_data_cache;

        move |deny_unsafe, subscription_task_executor| {
            let deps = crate::rpc::FullDeps {
                backend: backend.clone(),
                client: client.clone(),
                deny_unsafe,
                filter_pool: filter_pool.clone(),
                frontier_backend: match frontier_backend.clone() {
                    fc_db::Backend::KeyValue(b) => Arc::new(b),
                    fc_db::Backend::Sql(b) => Arc::new(b),
                },
                graph: pool.pool().clone(),
                pool: pool.clone(),
                max_past_logs,
                fee_history_limit,
                fee_history_cache: fee_history_cache.clone(),
                network: network.clone(),
                sync: sync.clone(),
                block_data_cache: block_data_cache.clone(),
                overrides: overrides.clone(),
                is_authority: false,
                command_sink: command_sink.clone(),
                xcm_senders: xcm_senders.clone(),
            };
            crate::rpc::create_full(
                deps,
                subscription_task_executor,
                pubsub_notification_sinks.clone(),
            )
            .map_err(Into::into)
        }
    };

    let _rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
        network,
        client,
        keystore: keystore_container.keystore(),
        task_manager: &mut task_manager,
        transaction_pool,
        rpc_builder: Box::new(rpc_builder),
        backend,
        system_rpc_tx,
        sync_service,
        config,
        tx_handler_controller,
        telemetry: None,
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

    network_starter.start_network();
    Ok(task_manager)
}

/// TODO: move it somewhere common, code duplication
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
