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

pub mod rpc;

use cumulus_client_bootnodes::{start_bootnode_tasks, StartBootnodeTasksParams};
use node_common::service::node_builder::StartBootnodeParams;
use sc_client_api::TrieCacheContext;
use sc_service::TaskRegistry;
use tokio::runtime::Handle;
use tokio::time::sleep;
use {
    cumulus_client_cli::CollatorOptions,
    cumulus_client_collator::service::CollatorService,
    cumulus_client_consensus_proposer::Proposer,
    cumulus_client_service::{
        prepare_node_config, start_relay_chain_tasks, DARecoveryProfile, StartRelayChainTasksParams,
    },
    cumulus_primitives_core::{relay_chain::CollatorPair, ParaId},
    cumulus_relay_chain_interface::{OverseerHandle, RelayChainInterface},
    dancebox_runtime::{
        opaque::{Block, Hash},
        AccountId, RuntimeApi,
    },
    dc_orchestrator_chain_interface::{
        BlockNumber, ContainerChainGenesisData, DataPreserverAssignment, DataPreserverProfileId,
        OrchestratorChainError, OrchestratorChainInterface, OrchestratorChainResult, PHash,
        PHeader,
    },
    futures::{Stream, StreamExt},
    nimbus_primitives::{NimbusId, NimbusPair},
    node_common::{service::node_builder::NodeBuilder, service::node_builder::NodeBuilderConfig},
    pallet_author_noting_runtime_api::AuthorNotingApi,
    pallet_collator_assignment_runtime_api::CollatorAssignmentApi,
    pallet_data_preservers_runtime_api::DataPreserversApi,
    pallet_registrar_runtime_api::RegistrarApi,
    polkadot_cli::ProvideRuntimeApi,
    sc_client_api::{
        AuxStore, Backend as BackendT, BlockchainEvents, HeaderBackend, UsageProvider,
    },
    sc_consensus::BasicQueue,
    sc_network::{NetworkBackend, NetworkBlock},
    sc_network_sync::SyncingService,
    sc_service::{Configuration, SpawnTaskHandle, TaskManager},
    sc_tracing::tracing::Instrument,
    sc_transaction_pool::TransactionPoolHandle,
    sp_api::{ApiExt, StorageProof},
    sp_consensus::SyncOracle,
    sp_consensus_slots::Slot,
    sp_core::H256,
    sp_keystore::KeystorePtr,
    sp_state_machine::{Backend as StateBackend, StorageValue},
    std::{marker::PhantomData, pin::Pin, sync::Arc, time::Duration},
    tc_consensus::{
        collators::lookahead::{
            self as lookahead_tanssi_aura, BuyCoreParams, Params as LookaheadTanssiAuraParams,
        },
        OnDemandBlockProductionApi, OrchestratorAuraWorkerAuxData, TanssiAuthorityAssignmentApi,
    },
    tc_service_container_chain_spawner::{
        cli::ContainerChainCli,
        monitor,
        service::{
            ParachainBlockImport, ParachainClient, ParachainExecutor, ParachainProposerFactory,
        },
        spawner::{self, CcSpawnMsg, ContainerChainSpawnParams, ContainerChainSpawner},
    },
    tokio::sync::mpsc,
    tokio_util::sync::CancellationToken,
};

type FullBackend = sc_service::TFullBackend<Block>;

pub struct NodeConfig;
impl NodeBuilderConfig for NodeConfig {
    type Block = Block;
    type RuntimeApi = RuntimeApi;
    type ParachainExecutor = ParachainExecutor;
}

pub struct ParachainNodeStarted {
    pub task_manager: TaskManager,
    pub client: Arc<ParachainClient>,
    pub relay_chain_interface: Arc<dyn RelayChainInterface>,
    pub orchestrator_chain_interface: Arc<dyn OrchestratorChainInterface>,
    pub keystore: KeystorePtr,
}

/// Start a parachain node.
pub async fn start_parachain_node<Net>(
    parachain_config: Configuration,
    polkadot_config: Configuration,
    container_config: Option<(ContainerChainCli, tokio::runtime::Handle)>,
    collator_options: CollatorOptions,
    para_id: ParaId,
    hwbench: Option<sc_sysinfo::HwBench>,
    max_pov_percentage: Option<u32>,
) -> sc_service::error::Result<ParachainNodeStarted>
where
    Net: NetworkBackend<Block, Hash>,
{
    start_node_impl::<Net>(
        parachain_config,
        polkadot_config,
        container_config,
        collator_options,
        para_id,
        hwbench,
        max_pov_percentage,
    )
    .instrument(sc_tracing::tracing::info_span!(
        sc_tracing::logging::PREFIX_LOG_SPAN,
        name = "Orchestrator",
    ))
    .await
}

/// Start collator task for orchestrator chain.
/// Returns a `CancellationToken` that can be used to cancel the collator task,
/// and a `oneshot::Receiver<()>` that can be used to wait until the task has ended.
fn start_consensus_orchestrator(
    client: Arc<ParachainClient>,
    backend: Arc<FullBackend>,
    block_import: ParachainBlockImport,
    spawner: SpawnTaskHandle,
    relay_chain_interface: Arc<dyn RelayChainInterface>,
    sync_oracle: Arc<SyncingService<Block>>,
    keystore: KeystorePtr,
    force_authoring: bool,
    relay_chain_slot_duration: Duration,
    para_id: ParaId,
    collator_key: CollatorPair,
    overseer_handle: OverseerHandle,
    announce_block: Arc<dyn Fn(Hash, Option<Vec<u8>>) + Send + Sync>,
    proposer_factory: ParachainProposerFactory,
    orchestrator_tx_pool: Arc<TransactionPoolHandle<Block, ParachainClient>>,
    max_pov_percentage: Option<u32>,
) -> (CancellationToken, futures::channel::oneshot::Receiver<()>) {
    let slot_duration = cumulus_client_consensus_aura::slot_duration(&*client)
        .expect("start_consensus_orchestrator: slot duration should exist");

    let proposer = Proposer::new(proposer_factory);

    let collator_service = CollatorService::new(
        client.clone(),
        Arc::new(spawner.clone()),
        announce_block,
        client.clone(),
    );

    let relay_chain_interace_for_cidp = relay_chain_interface.clone();
    let client_set_aside_for_cidp = client.clone();
    let client_set_aside_for_orch = client.clone();
    let client_for_hash_provider = client.clone();
    let client_for_slot_duration_provider = client.clone();

    let code_hash_provider = move |block_hash| {
        client_for_hash_provider
            .code_at(block_hash)
            .ok()
            .map(polkadot_primitives::ValidationCode)
            .map(|c| c.hash())
    };

    let cancellation_token = CancellationToken::new();
    let buy_core_params = BuyCoreParams::Orchestrator {
        orchestrator_tx_pool,
        orchestrator_client: client.clone(),
    };

    let params = LookaheadTanssiAuraParams {
        max_pov_percentage,
        get_current_slot_duration: move |block_hash| {
            sc_consensus_aura::standalone::slot_duration_at(
                &*client_for_slot_duration_provider,
                block_hash,
            )
            .expect("Slot duration should be set")
        },
        create_inherent_data_providers: move |block_hash, (relay_parent, _validation_data)| {
            let relay_chain_interface = relay_chain_interace_for_cidp.clone();
            let client_set_aside_for_cidp = client_set_aside_for_cidp.clone();
            async move {
                // We added a new runtime api that allows to know which parachains have
                // some collators assigned to them. We'll now only include those. For older
                // runtimes we continue to write all of them.
                let para_ids = match client_set_aside_for_cidp
                    .runtime_api()
                    .api_version::<dyn CollatorAssignmentApi<Block, AccountId, ParaId>>(
                    block_hash,
                )? {
                    Some(version) if version >= 2 => client_set_aside_for_cidp
                        .runtime_api()
                        .parachains_with_some_collators(block_hash)?,
                    _ => client_set_aside_for_cidp
                        .runtime_api()
                        .registered_paras(block_hash)?,
                };
                let para_ids: Vec<_> = para_ids.into_iter().collect();
                let author_noting_inherent =
                    tp_author_noting_inherent::OwnParachainInherentData::create_at(
                        relay_parent,
                        &relay_chain_interface,
                        &para_ids,
                    )
                    .await;

                // Fetch duration every block to avoid downtime when passing from 12 to 6s
                let slot_duration = sc_consensus_aura::standalone::slot_duration_at(
                    &*client_set_aside_for_cidp.clone(),
                    block_hash,
                )
                .expect("Slot duration should be set");

                let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

                let slot =
						sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
							*timestamp,
							slot_duration,
						);

                let author_noting_inherent = author_noting_inherent.ok_or_else(|| {
                    Box::<dyn std::error::Error + Send + Sync>::from(
                        "Failed to create author noting inherent",
                    )
                })?;

                Ok((slot, timestamp, author_noting_inherent))
            }
        },
        get_orchestrator_aux_data: move |block_hash: H256, (_relay_parent, _validation_data)| {
            let client_set_aside_for_orch = client_set_aside_for_orch.clone();

            async move {
                let authorities = tc_consensus::authorities::<Block, ParachainClient, NimbusPair>(
                    client_set_aside_for_orch.as_ref(),
                    &block_hash,
                    para_id,
                );

                let authorities = authorities.ok_or_else(|| {
                    Box::<dyn std::error::Error + Send + Sync>::from(
                        "Failed to fetch authorities with error",
                    )
                })?;

                log::info!(
                    "Authorities {:?} found for header {:?}",
                    authorities,
                    block_hash
                );

                let aux_data = OrchestratorAuraWorkerAuxData {
                    authorities,
                    // This is the orchestrator consensus, it does not have a slot frequency
                    slot_freq: None,
                };

                Ok(aux_data)
            }
        },
        block_import,
        para_client: client,
        relay_client: relay_chain_interface,
        sync_oracle,
        keystore,
        collator_key,
        para_id,
        overseer_handle,
        orchestrator_slot_duration: slot_duration,
        relay_chain_slot_duration,
        force_authoring,
        proposer,
        collator_service,
        authoring_duration: Duration::from_millis(2000),
        code_hash_provider,
        para_backend: backend,
        cancellation_token: cancellation_token.clone(),
        buy_core_params,
    };

    let (fut, exit_notification_receiver) =
        lookahead_tanssi_aura::run::<_, Block, NimbusPair, _, _, _, _, _, _, _, _, _, _, _, _, _>(
            params,
        );
    spawner.spawn("tanssi-aura", None, fut);

    (cancellation_token, exit_notification_receiver)
}

/// Start a node with the given parachain `Configuration` and relay chain `Configuration`.
///
/// This is the actual implementation that is abstract over the executor and the runtime api.
async fn start_node_impl<Net>(
    orchestrator_config: Configuration,
    polkadot_config: Configuration,
    container_chain_config: Option<(ContainerChainCli, tokio::runtime::Handle)>,
    collator_options: CollatorOptions,
    para_id: ParaId,
    hwbench: Option<sc_sysinfo::HwBench>,
    max_pov_percentage: Option<u32>,
) -> sc_service::error::Result<ParachainNodeStarted>
where
    Net: NetworkBackend<Block, Hash>,
{
    let parachain_config = prepare_node_config(orchestrator_config);
    let chain_type: sc_chain_spec::ChainType = parachain_config.chain_spec.chain_type();
    let relay_chain = node_common::chain_spec::Extensions::try_get(&*parachain_config.chain_spec)
        .map(|e| e.relay_chain.clone())
        .ok_or("Could not find relay_chain extension in chain-spec.")?;

    // Channel to send messages to start/stop container chains
    let (cc_spawn_tx, cc_spawn_rx) = mpsc::unbounded_channel();

    // Create a `NodeBuilder` which helps setup parachain nodes common systems.
    let mut node_builder = NodeConfig::new_builder(&parachain_config, hwbench.clone())?;

    let (block_import, import_queue) = import_queue(&parachain_config, &node_builder);

    // TODO: start bootnode tasks
    let (relay_chain_interface, collator_key, start_bootnode_params) = node_builder
        .build_relay_chain_interface(&parachain_config, polkadot_config, collator_options.clone())
        .await?;

    let validator = parachain_config.role.is_authority();
    let force_authoring = parachain_config.force_authoring;

    let node_builder = node_builder
        .build_cumulus_network::<_, Net>(
            &parachain_config,
            para_id,
            import_queue,
            relay_chain_interface.clone(),
        )
        .await?;

    let rpc_builder = {
        let client = node_builder.client.clone();
        let transaction_pool = node_builder.transaction_pool.clone();

        Box::new(move |_| {
            let deps = rpc::FullDeps {
                client: client.clone(),
                pool: transaction_pool.clone(),
                command_sink: None,
                xcm_senders: None,
                randomness_sender: None,
                container_chain_exclusion_sender: None,
            };

            rpc::create_full(deps).map_err(Into::into)
        })
    };

    let node_builder = node_builder.spawn_common_tasks(parachain_config, rpc_builder)?;

    let relay_chain_slot_duration = Duration::from_secs(6);
    let overseer_handle = relay_chain_interface
        .overseer_handle()
        .map_err(|e| sc_service::Error::Application(Box::new(e)))?;
    let sync_keystore = node_builder.keystore_container.keystore();
    let mut collate_on_tanssi: Arc<
        dyn Fn() -> (CancellationToken, futures::channel::oneshot::Receiver<()>) + Send + Sync,
    > = Arc::new(move || {
        if validator {
            panic!("Called uninitialized collate_on_tanssi");
        } else {
            panic!("Called collate_on_tanssi when node is not running as a validator");
        }
    });

    let announce_block = {
        let sync_service = node_builder.network.sync_service.clone();
        Arc::new(move |hash, data| sync_service.announce_block(hash, data))
    };

    let (mut node_builder, import_queue_service) = node_builder.extract_import_queue_service();

    start_relay_chain_tasks(StartRelayChainTasksParams {
        client: node_builder.client.clone(),
        announce_block: announce_block.clone(),
        para_id,
        relay_chain_interface: relay_chain_interface.clone(),
        task_manager: &mut node_builder.task_manager,
        da_recovery_profile: if validator {
            DARecoveryProfile::Collator
        } else {
            DARecoveryProfile::FullNode
        },
        import_queue: import_queue_service,
        relay_chain_slot_duration,
        recovery_handle: Box::new(overseer_handle.clone()),
        sync_service: node_builder.network.sync_service.clone(),
        prometheus_registry: node_builder.prometheus_registry.as_ref(),
    })?;

    {
        let StartBootnodeParams {
            relay_chain_fork_id,
            parachain_fork_id,
            advertise_non_global_ips,
            parachain_public_addresses,
            relay_chain_network,
            paranode_rx,
            embedded_dht_bootnode,
            dht_bootnode_discovery,
        } = start_bootnode_params;

        // Advertise parachain bootnode address in relay chain DHT
        start_bootnode_tasks(StartBootnodeTasksParams {
            embedded_dht_bootnode,
            dht_bootnode_discovery,
            para_id,
            task_manager: &mut node_builder.task_manager,
            relay_chain_interface: relay_chain_interface.clone(),
            relay_chain_fork_id,
            relay_chain_network,
            request_receiver: paranode_rx,
            parachain_network: node_builder.network.network.clone(),
            advertise_non_global_ips,
            parachain_genesis_hash: node_builder.client.chain_info().genesis_hash,
            parachain_fork_id,
            parachain_public_addresses,
        });
    }

    let orchestrator_chain_interface_builder = OrchestratorChainInProcessInterfaceBuilder {
        client: node_builder.client.clone(),
        backend: node_builder.backend.clone(),
        sync_oracle: node_builder.network.sync_service.clone(),
        overseer_handle: overseer_handle.clone(),
    };
    let orchestrator_chain_interface = orchestrator_chain_interface_builder.build();

    if validator {
        let collator_key = collator_key
            .clone()
            .expect("Command line arguments do not allow this. qed");

        // Start task which detects para id assignment, and starts/stops container chains.
        // Note that if this node was started without a `container_chain_config`, we don't
        // support collation on container chains, so there is no need to detect changes to assignment
        if container_chain_config.is_some() {
            crate::build_check_assigned_para_id(
                orchestrator_chain_interface.clone(),
                sync_keystore.clone(),
                cc_spawn_tx.clone(),
                node_builder.task_manager.spawn_essential_handle(),
            );
        }

        let start_collation = {
            // Params for collate_on_tanssi closure
            let node_spawn_handle = node_builder.task_manager.spawn_handle().clone();
            let node_keystore = node_builder.keystore_container.keystore().clone();
            let node_telemetry_handle = node_builder.telemetry.as_ref().map(|t| t.handle()).clone();
            let node_client = node_builder.client.clone();
            let node_backend = node_builder.backend.clone();
            let relay_interface = relay_chain_interface.clone();
            let node_sync_service = node_builder.network.sync_service.clone();
            let orchestrator_tx_pool = node_builder.transaction_pool.clone();
            let overseer = overseer_handle.clone();
            let proposer_factory = sc_basic_authorship::ProposerFactory::with_proof_recording(
                node_spawn_handle.clone(),
                node_client.clone(),
                node_builder.transaction_pool.clone(),
                node_builder.prometheus_registry.as_ref(),
                node_telemetry_handle.clone(),
            );

            move || {
                start_consensus_orchestrator(
                    node_client.clone(),
                    node_backend.clone(),
                    block_import.clone(),
                    node_spawn_handle.clone(),
                    relay_interface.clone(),
                    node_sync_service.clone(),
                    node_keystore.clone(),
                    force_authoring,
                    relay_chain_slot_duration,
                    para_id,
                    collator_key.clone(),
                    overseer.clone(),
                    announce_block.clone(),
                    proposer_factory.clone(),
                    orchestrator_tx_pool.clone(),
                    max_pov_percentage,
                )
            }
        };
        // Save callback for later, used when collator rotates from container chain back to orchestrator chain
        collate_on_tanssi = Arc::new(start_collation);
    }

    let sync_keystore = node_builder.keystore_container.keystore();

    if let Some((container_chain_cli, tokio_handle)) = container_chain_config {
        // If the orchestrator chain is running as a full-node, we start a full node for the
        // container chain immediately, because only collator nodes detect their container chain
        // assignment so otherwise it will never start.
        if !validator {
            if let Some(container_chain_para_id) = container_chain_cli.base.para_id {
                // Spawn new container chain node
                cc_spawn_tx
                    .send(CcSpawnMsg::UpdateAssignment {
                        current: Some(container_chain_para_id.into()),
                        next: Some(container_chain_para_id.into()),
                    })
                    .map_err(|e| sc_service::Error::Application(Box::new(e) as Box<_>))?;
            }
        }

        // Start container chain spawner task. This will start and stop container chains on demand.
        let orchestrator_client = node_builder.client.clone();
        let orchestrator_tx_pool = node_builder.transaction_pool.clone();
        let spawn_handle = node_builder.task_manager.spawn_handle();
        let relay_chain_interface = relay_chain_interface.clone();
        let orchestrator_chain_interface = orchestrator_chain_interface.clone();

        // This considers that the container chains have the same APIs as dancebox, which
        // is not the case. However the spawner don't call APIs that are not part of the expected
        // common APIs for a container chain.
        // TODO: Depend on the simple container chain runtime which should be the minimal api?
        let container_chain_spawner = ContainerChainSpawner {
            params: ContainerChainSpawnParams {
                orchestrator_chain_interface,
                container_chain_cli,
                tokio_handle,
                chain_type,
                relay_chain,
                relay_chain_interface,
                sync_keystore,
                data_preserver: false,
                collation_params: if validator {
                    Some(spawner::CollationParams {
                        orchestrator_client: Some(orchestrator_client.clone()),
                        orchestrator_tx_pool: Some(orchestrator_tx_pool),
                        orchestrator_para_id: para_id,
                        collator_key: collator_key
                            .expect("there should be a collator key if we're a validator"),
                        solochain: false,
                    })
                } else {
                    None
                },
                spawn_handle,
                generate_rpc_builder:
                    tc_service_container_chain_spawner::rpc::GenerateSubstrateRpcBuilder::<
                        dancebox_runtime::RuntimeApi,
                    >::new(),
                override_sync_mode: Some(sc_cli::SyncMode::Warp),
                phantom: PhantomData,
            },
            state: Default::default(),
            db_folder_cleanup_done: false,
            collate_on_tanssi,
            collation_cancellation_constructs: None,
        };
        let state = container_chain_spawner.state.clone();

        node_builder.task_manager.spawn_essential_handle().spawn(
            "container-chain-spawner-rx-loop",
            None,
            container_chain_spawner.rx_loop(cc_spawn_rx, validator, false),
        );

        node_builder.task_manager.spawn_essential_handle().spawn(
            "container-chain-spawner-debug-state",
            None,
            monitor::monitor_task(state),
        );

        if false {
            node_builder.task_manager.spawn_essential_handle().spawn(
                "container-chain-task-list-monitor",
                None,
                monitor_task_manager(node_builder.task_manager.spawn_handle()),
            );
        }
    }

    Ok(ParachainNodeStarted {
        task_manager: node_builder.task_manager,
        client: node_builder.client,
        relay_chain_interface,
        orchestrator_chain_interface,
        keystore: node_builder.keystore_container.keystore(),
    })
}

/*
monitor task manager:
* [default] import-queue
* [default] cumulus-dht-bootnode-discovery
* [networking] peer-store
* [networking] libp2p-node x35
* [default] parachain-informant
* [transaction-pool] txpool-notifications
* [default] notification-pinning-worker
* [networking] block-request-handler
* [networking] state-request-handler
* [transaction-pool] on-transaction-imported
* [default] cumulus-dht-bootnode-advertisement
* [default] container-chain-spawner-debug-state
* [default] container-chain-task-list-monitor
* [default] tanssi-aura
* [networking] network-transactions-handler
* [networking] system-rpc-handler
* [default] container-chain-spawner-rx-loop
* [default] cumulus-consensus
* [transaction-pool] transaction-pool-task-1
* [offchain-work] offchain-workers-runner
* [block-import] basic-block-import-worker
* [transaction-pool] transaction-pool-task-0
* [default] cumulus-pov-recovery
* [default] syncing
* [default] check-assigned-para-id
* [networking] light-client-request-handler
* [transaction-pool] txpool-background x2
* [networking] chain-sync-network-service-provider
* [default] telemetry-periodic-send
* [default] prometheus-endpoint
* [default] informant
* [networking] network-worker
 */
async fn monitor_task_manager(spawn_task_handle: SpawnTaskHandle) {
    // Main loop frequency, doesn't need to be fast
    let monitor_period = Duration::from_secs(300 * 0 + 10);

    loop {
        sleep(monitor_period).await;
        log::debug!("Monitor tick");

        let handle_private: &SpawnTaskHandle = &spawn_task_handle;
        #[allow(unsafe_code)]
        let handle_public: &MySpawnTaskHandle = unsafe { &*(handle_private as *const SpawnTaskHandle as *const MySpawnTaskHandle) };
        let tasks = handle_public.task_registry.running_tasks();

        log::info!("monitor task manager:");
        for (task, count) in tasks {
            log::info!("* [{}] {}{}", task.group, task.name, if count > 1 { format!(" x{}", count) } else { " ".to_string() });
        }
    }
}

/// An handle for spawning tasks in the service.
#[derive(Clone)]
pub struct MySpawnTaskHandle {
    on_exit: exit_future::Exit,
    tokio_handle: Handle,
    metrics: Option<MyMetrics>,
    task_registry: TaskRegistry,
}

use prometheus_endpoint::{
    exponential_buckets, register, CounterVec, HistogramOpts, HistogramVec, Opts, PrometheusError,
    Registry, U64,
};

#[derive(Clone)]
struct MyMetrics {
    // This list is ordered alphabetically
    poll_duration: HistogramVec,
    poll_start: CounterVec<U64>,
    tasks_spawned: CounterVec<U64>,
    tasks_ended: CounterVec<U64>,
}

pub fn import_queue(
    parachain_config: &Configuration,
    node_builder: &NodeBuilder<NodeConfig>,
) -> (ParachainBlockImport, BasicQueue<Block>) {
    // The nimbus import queue ONLY checks the signature correctness
    // Any other checks corresponding to the author-correctness should be done
    // in the runtime
    let block_import =
        ParachainBlockImport::new(node_builder.client.clone(), node_builder.backend.clone());

    let import_queue = nimbus_consensus::import_queue(
        node_builder.client.clone(),
        block_import.clone(),
        move |_, _| async move {
            let time = sp_timestamp::InherentDataProvider::from_system_time();

            Ok((time,))
        },
        &node_builder.task_manager.spawn_essential_handle(),
        parachain_config.prometheus_registry(),
        false,
        false,
    )
    .expect("function never fails");

    (block_import, import_queue)
}

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
    overseer_handle: OverseerHandle,
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
    pub full_client: Arc<Client>,
    pub backend: Arc<FullBackend>,
    pub sync_oracle: Arc<dyn SyncOracle + Send + Sync>,
    pub overseer_handle: OverseerHandle,
}

impl<Client> OrchestratorChainInProcessInterface<Client> {
    /// Create a new instance of [`RelayChainInProcessInterface`]
    pub fn new(
        full_client: Arc<Client>,
        backend: Arc<FullBackend>,
        sync_oracle: Arc<dyn SyncOracle + Send + Sync>,
        overseer_handle: OverseerHandle,
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
    Client::Api: TanssiAuthorityAssignmentApi<Block, NimbusId>
        + OnDemandBlockProductionApi<Block, ParaId, Slot>
        + RegistrarApi<Block, ParaId>
        + AuthorNotingApi<Block, AccountId, BlockNumber, ParaId>
        + DataPreserversApi<Block, DataPreserverProfileId, ParaId>,
{
    async fn get_storage_by_key(
        &self,
        orchestrator_parent: PHash,
        key: &[u8],
    ) -> OrchestratorChainResult<Option<StorageValue>> {
        // TODO: trusted or untrusted?
        let state = self
            .backend
            .state_at(orchestrator_parent, TrieCacheContext::Untrusted)?;
        state
            .storage(key)
            .map_err(OrchestratorChainError::GenericError)
    }

    async fn prove_read(
        &self,
        orchestrator_parent: PHash,
        relevant_keys: &Vec<Vec<u8>>,
    ) -> OrchestratorChainResult<StorageProof> {
        let state_backend = self
            .backend
            .state_at(orchestrator_parent, TrieCacheContext::Untrusted)?;

        sp_state_machine::prove_read(state_backend, relevant_keys)
            .map_err(OrchestratorChainError::StateMachineError)
    }

    fn overseer_handle(&self) -> OrchestratorChainResult<OverseerHandle> {
        Ok(self.overseer_handle.clone())
    }

    /// Get a stream of import block notifications.
    async fn import_notification_stream(
        &self,
    ) -> OrchestratorChainResult<Pin<Box<dyn Stream<Item = PHeader> + Send>>> {
        let notification_stream = self
            .full_client
            .import_notification_stream()
            .map(|notification| notification.header);
        Ok(Box::pin(notification_stream))
    }

    /// Get a stream of new best block notifications.
    async fn new_best_notification_stream(
        &self,
    ) -> OrchestratorChainResult<Pin<Box<dyn Stream<Item = PHeader> + Send>>> {
        let notifications_stream =
            self.full_client
                .import_notification_stream()
                .filter_map(|notification| async move {
                    notification.is_new_best.then_some(notification.header)
                });
        Ok(Box::pin(notifications_stream))
    }

    /// Get a stream of finality notifications.
    async fn finality_notification_stream(
        &self,
    ) -> OrchestratorChainResult<Pin<Box<dyn Stream<Item = PHeader> + Send>>> {
        let notification_stream = self
            .full_client
            .finality_notification_stream()
            .map(|notification| notification.header);
        Ok(Box::pin(notification_stream))
    }

    async fn genesis_data(
        &self,
        orchestrator_parent: PHash,
        para_id: ParaId,
    ) -> OrchestratorChainResult<Option<ContainerChainGenesisData>> {
        let runtime_api = self.full_client.runtime_api();

        Ok(runtime_api.genesis_data(orchestrator_parent, para_id)?)
    }

    async fn boot_nodes(
        &self,
        orchestrator_parent: PHash,
        para_id: ParaId,
    ) -> OrchestratorChainResult<Vec<Vec<u8>>> {
        let runtime_api = self.full_client.runtime_api();

        Ok(runtime_api.boot_nodes(orchestrator_parent, para_id)?)
    }

    async fn latest_block_number(
        &self,
        orchestrator_parent: PHash,
        para_id: ParaId,
    ) -> OrchestratorChainResult<Option<BlockNumber>> {
        let runtime_api = self.full_client.runtime_api();

        Ok(runtime_api.latest_block_number(orchestrator_parent, para_id)?)
    }

    async fn best_block_hash(&self) -> OrchestratorChainResult<PHash> {
        Ok(self.backend.blockchain().info().best_hash)
    }

    async fn finalized_block_hash(&self) -> OrchestratorChainResult<PHash> {
        Ok(self.backend.blockchain().info().finalized_hash)
    }

    async fn data_preserver_active_assignment(
        &self,
        orchestrator_parent: PHash,
        profile_id: DataPreserverProfileId,
    ) -> OrchestratorChainResult<DataPreserverAssignment<ParaId>> {
        let runtime_api = self.full_client.runtime_api();

        use {
            dc_orchestrator_chain_interface::DataPreserverAssignment as InterfaceAssignment,
            pallet_data_preservers_runtime_api::Assignment as RuntimeAssignment,
        };

        Ok(
            match runtime_api.get_active_assignment(orchestrator_parent, profile_id)? {
                RuntimeAssignment::NotAssigned => InterfaceAssignment::NotAssigned,
                RuntimeAssignment::Active(para_id) => InterfaceAssignment::Active(para_id),
                RuntimeAssignment::Inactive(para_id) => InterfaceAssignment::Inactive(para_id),
            },
        )
    }

    async fn check_para_id_assignment(
        &self,
        orchestrator_parent: PHash,
        authority: NimbusId,
    ) -> OrchestratorChainResult<Option<ParaId>> {
        let runtime_api = self.full_client.runtime_api();

        Ok(runtime_api.check_para_id_assignment(orchestrator_parent, authority)?)
    }

    async fn check_para_id_assignment_next_session(
        &self,
        orchestrator_parent: PHash,
        authority: NimbusId,
    ) -> OrchestratorChainResult<Option<ParaId>> {
        let runtime_api = self.full_client.runtime_api();

        Ok(runtime_api.check_para_id_assignment_next_session(orchestrator_parent, authority)?)
    }
}
