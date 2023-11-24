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
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>

//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

#[allow(deprecated)]
use {
    crate::{
        cli::ContainerChainCli,
        container_chain_spawner::{CcSpawnMsg, ContainerChainSpawner},
    },
    cumulus_client_cli::CollatorOptions,
    cumulus_client_consensus_aura::SlotProportion,
    cumulus_client_consensus_common::{
        ParachainBlockImport as TParachainBlockImport, ParachainBlockImportMarker,
        ParachainConsensus,
    },
    cumulus_client_pov_recovery::{PoVRecovery, RecoveryDelayRange},
    cumulus_client_service::prepare_node_config,
    cumulus_primitives_core::{
        relay_chain::{CollatorPair, Hash as PHash},
        ParaId,
    },
    cumulus_primitives_parachain_inherent::{
        MockValidationDataInherentDataProvider, MockXcmConfig,
    },
    cumulus_relay_chain_interface::RelayChainInterface,
    dancebox_runtime::{opaque::Block, RuntimeApi},
    dc_orchestrator_chain_interface::{
        OrchestratorChainError, OrchestratorChainInterface, OrchestratorChainResult,
    },
    futures::{channel::mpsc, StreamExt},
    nimbus_primitives::NimbusPair,
    node_common::service::NodeBuilderConfig,
    node_common::service::{ManualSealConfiguration, NodeBuilder, Sealing},
    pallet_registrar_runtime_api::RegistrarApi,
    polkadot_cli::ProvideRuntimeApi,
    polkadot_service::Handle,
    sc_client_api::{
        AuxStore, Backend as BackendT, BlockchainEvents, HeaderBackend, UsageProvider,
    },
    sc_consensus::BasicQueue,
    sc_consensus::BlockImport,
    sc_executor::NativeElseWasmExecutor,
    sc_network::NetworkBlock,
    sc_network_sync::SyncingService,
    sc_service::{Configuration, TFullBackend, TFullClient, TaskManager},
    sc_telemetry::TelemetryHandle,
    sp_api::StorageProof,
    sp_consensus::SyncOracle,
    sp_core::{traits::SpawnEssentialNamed, H256},
    sp_keystore::KeystorePtr,
    sp_state_machine::{Backend as StateBackend, StorageValue},
    std::{future::Future, pin::Pin, sync::Arc, time::Duration},
    substrate_prometheus_endpoint::Registry,
    tc_consensus::{BuildOrchestratorAuraConsensusParams, OrchestratorAuraConsensus},
    tokio::sync::mpsc::{unbounded_channel, UnboundedSender},
};

type FullBackend = TFullBackend<Block>;

/// Native executor type.
pub struct ParachainNativeExecutor;

impl sc_executor::NativeExecutionDispatch for ParachainNativeExecutor {
    type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;

    fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
        dancebox_runtime::api::dispatch(method, data)
    }

    fn native_version() -> sc_executor::NativeVersion {
        dancebox_runtime::native_version()
    }
}

pub struct NodeConfig;
impl NodeBuilderConfig for NodeConfig {
    type Block = Block;
    type RuntimeApi = RuntimeApi;
    type ParachainNativeExecutor = ParachainNativeExecutor;
}

type ParachainExecutor = NativeElseWasmExecutor<ParachainNativeExecutor>;

pub type ParachainClient = TFullClient<Block, RuntimeApi, ParachainExecutor>;

pub type ParachainBackend = TFullBackend<Block>;

type DevParachainBlockImport = OrchestratorParachainBlockImport<Arc<ParachainClient>>;

type ParachainBlockImport = TParachainBlockImport<Block, Arc<ParachainClient>, ParachainBackend>;

thread_local!(static TIMESTAMP: std::cell::RefCell<u64> = std::cell::RefCell::new(0));

/// Provide a mock duration starting at 0 in millisecond for timestamp inherent.
/// Each call will increment timestamp by slot_duration making Aura think time has passed.
struct MockTimestampInherentDataProvider;
#[async_trait::async_trait]
impl sp_inherents::InherentDataProvider for MockTimestampInherentDataProvider {
    async fn provide_inherent_data(
        &self,
        inherent_data: &mut sp_inherents::InherentData,
    ) -> Result<(), sp_inherents::Error> {
        TIMESTAMP.with(|x| {
            *x.borrow_mut() += dancebox_runtime::SLOT_DURATION;
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

/// Background task used to detect changes to container chain assignment,
/// and start/stop container chains on demand. The check runs on every new block.
pub fn build_check_assigned_para_id(
    client: Arc<ParachainClient>,
    sync_keystore: KeystorePtr,
    cc_spawn_tx: UnboundedSender<CcSpawnMsg>,
    spawner: impl SpawnEssentialNamed,
) {
    // Subscribe to new blocks in order to react to para id assignment
    // This must be the stream of finalized blocks, otherwise the collators may rotate to a
    // different chain before the block is finalized, and that could lead to a stalled chain
    let mut import_notifications = client.finality_notification_stream();

    let check_assigned_para_id_task = async move {
        while let Some(msg) = import_notifications.next().await {
            let block_hash = msg.hash;
            let client_set_aside_for_cidp = client.clone();
            let sync_keystore = sync_keystore.clone();
            let cc_spawn_tx = cc_spawn_tx.clone();

            check_assigned_para_id(
                cc_spawn_tx,
                sync_keystore,
                client_set_aside_for_cidp,
                block_hash,
            )
            .unwrap();
        }
    };

    spawner.spawn_essential(
        "check-assigned-para-id",
        None,
        Box::pin(check_assigned_para_id_task),
    );
}

/// Check the parachain assignment using the orchestrator chain client, and send a `CcSpawnMsg` if
/// the para id has changed since the last call to this function.
///
/// Checks the assignment for the next block, so if there is a session change on block 15, this will
/// detect the assignment change after importing block 14.
fn check_assigned_para_id(
    cc_spawn_tx: UnboundedSender<CcSpawnMsg>,
    sync_keystore: KeystorePtr,
    client_set_aside_for_cidp: Arc<ParachainClient>,
    block_hash: H256,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Check current assignment
    let current_container_chain_para_id =
        tc_consensus::first_eligible_key::<Block, ParachainClient, NimbusPair>(
            client_set_aside_for_cidp.as_ref(),
            &block_hash,
            sync_keystore.clone(),
        )
        .map(|(_nimbus_key, para_id)| para_id);

    // Check assignment in the next session
    let next_container_chain_para_id =
        tc_consensus::first_eligible_key_next_session::<Block, ParachainClient, NimbusPair>(
            client_set_aside_for_cidp.as_ref(),
            &block_hash,
            sync_keystore,
        )
        .map(|(_nimbus_key, para_id)| para_id);

    cc_spawn_tx.send(CcSpawnMsg::UpdateAssignment {
        current: current_container_chain_para_id,
        next: next_container_chain_para_id,
    })?;

    Ok(())
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
    )
    .expect("function never fails");

    (block_import, import_queue)
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
        .ok_or("Could not find relay_chain extension in chain-spec.")?;

    // Channel to send messages to start/stop container chains
    let (cc_spawn_tx, cc_spawn_rx) = unbounded_channel();

    // Create a `NodeBuilder` which helps setup parachain nodes common systems.
    let mut node_builder = NodeConfig::new_builder(&parachain_config, hwbench.clone())?;

    let (block_import, import_queue) = import_queue(&parachain_config, &node_builder);

    let (relay_chain_interface, collator_key) = node_builder
        .build_relay_chain_interface(&parachain_config, polkadot_config, collator_options.clone())
        .await?;

    let validator = parachain_config.role.is_authority();
    let force_authoring = parachain_config.force_authoring;

    let node_builder = node_builder
        .build_cumulus_network(
            &parachain_config,
            para_id,
            import_queue,
            relay_chain_interface.clone(),
        )
        .await?;

    let rpc_builder = {
        let client = node_builder.client.clone();
        let transaction_pool = node_builder.transaction_pool.clone();

        Box::new(move |deny_unsafe, _| {
            let deps = crate::rpc::FullDeps {
                client: client.clone(),
                pool: transaction_pool.clone(),
                deny_unsafe,
                command_sink: None,
                xcm_senders: None,
            };

            crate::rpc::create_full(deps).map_err(Into::into)
        })
    };

    let node_builder = node_builder.spawn_common_tasks(parachain_config, rpc_builder)?;

    let relay_chain_slot_duration = Duration::from_secs(6);
    let overseer_handle = relay_chain_interface
        .overseer_handle()
        .map_err(|e| sc_service::Error::Application(Box::new(e)))?;
    let sync_keystore = node_builder.keystore_container.keystore();
    let mut collate_on_tanssi = None;

    let node_builder = if validator {
        let collator_key = collator_key
            .clone()
            .expect("Command line arguments do not allow this. qed");

        // Start task which detects para id assignment, and starts/stops container chains.
        // Note that if this node was started without a `container_chain_config`, we don't
        // support collation on container chains, so there is no need to detect changes to assignment
        if container_chain_config.is_some() {
            build_check_assigned_para_id(
                node_builder.client.clone(),
                sync_keystore.clone(),
                cc_spawn_tx.clone(),
                node_builder.task_manager.spawn_essential_handle(),
            );
        }

        let parachain_consensus = build_consensus_orchestrator(
            node_builder.client.clone(),
            block_import,
            node_builder.prometheus_registry.as_ref(),
            node_builder.telemetry.as_ref().map(|t| t.handle()),
            &node_builder.task_manager,
            relay_chain_interface.clone(),
            node_builder.transaction_pool.clone(),
            node_builder.network.sync_service.clone(),
            node_builder.keystore_container.keystore(),
            force_authoring,
            para_id,
        )?;

        let params_generator = node_builder.cumulus_client_collator_params_generator(
            para_id,
            overseer_handle.clone(),
            collator_key.clone(),
            parachain_consensus.clone(),
        );

        // TODO: change for async backing
        collate_on_tanssi = Some(move || async move {
            #[allow(deprecated)]
            cumulus_client_collator::start_collator(params_generator()).await;
        });

        node_builder
            .start_collator(
                para_id,
                relay_chain_interface.clone(),
                relay_chain_slot_duration,
                parachain_consensus,
                collator_key,
            )
            .await?
    } else {
        node_builder.start_full_node(
            para_id,
            relay_chain_interface.clone(),
            relay_chain_slot_duration,
        )?
    };

    node_builder.network.start_network.start_network();

    let sync_keystore = node_builder.keystore_container.keystore();
    let orchestrator_chain_interface_builder = OrchestratorChainInProcessInterfaceBuilder {
        client: node_builder.client.clone(),
        backend: node_builder.backend.clone(),
        sync_oracle: node_builder.network.sync_service.clone(),
        overseer_handle: overseer_handle.clone(),
    };

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
        let spawn_handle = node_builder.task_manager.spawn_handle();
        let container_chain_spawner = ContainerChainSpawner {
            orchestrator_chain_interface: orchestrator_chain_interface_builder.build(),
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
            spawn_handle,
            state: Default::default(),
            collate_on_tanssi: Arc::new(move || Box::pin((collate_on_tanssi.clone().unwrap())())),
        };
        let state = container_chain_spawner.state.clone();

        node_builder.task_manager.spawn_essential_handle().spawn(
            "container-chain-spawner-rx-loop",
            None,
            container_chain_spawner.rx_loop(cc_spawn_rx),
        );

        node_builder.task_manager.spawn_essential_handle().spawn(
            "container-chain-spawner-debug-state",
            None,
            crate::container_chain_monitor::monitor_task(state),
        )
    }

    Ok((node_builder.task_manager, node_builder.client))
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
pub async fn start_node_impl_container(
    parachain_config: Configuration,
    orchestrator_client: Arc<ParachainClient>,
    relay_chain_interface: Arc<dyn RelayChainInterface>,
    orchestrator_chain_interface: Arc<dyn OrchestratorChainInterface>,
    collator_key: Option<CollatorPair>,
    keystore: KeystorePtr,
    para_id: ParaId,
    orchestrator_para_id: ParaId,
    collator: bool,
) -> sc_service::error::Result<(
    TaskManager,
    Arc<ParachainClient>,
    Arc<ParachainBackend>,
    Option<Arc<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>>,
)> {
    let parachain_config = prepare_node_config(parachain_config);

    // Create a `NodeBuilder` which helps setup parachain nodes common systems.
    let node_builder = NodeConfig::new_builder(&parachain_config, None)?;

    let (block_import, import_queue) = import_queue(&parachain_config, &node_builder);

    log::info!("are we collators? {:?}", collator);
    let node_builder = node_builder
        .build_cumulus_network(
            &parachain_config,
            para_id,
            import_queue,
            relay_chain_interface.clone(),
        )
        .await?;

    let force_authoring = parachain_config.force_authoring;
    let prometheus_registry = parachain_config.prometheus_registry().cloned();

    let rpc_builder = {
        let client = node_builder.client.clone();
        let transaction_pool = node_builder.transaction_pool.clone();

        Box::new(move |deny_unsafe, _| {
            let deps = crate::rpc::FullDeps {
                client: client.clone(),
                pool: transaction_pool.clone(),
                deny_unsafe,
                command_sink: None,
                xcm_senders: None,
            };

            crate::rpc::create_full(deps).map_err(Into::into)
        })
    };

    let node_builder = node_builder.spawn_common_tasks(parachain_config, rpc_builder)?;

    let announce_block = {
        let sync_service = node_builder.network.sync_service.clone();
        Arc::new(move |hash, data| sync_service.announce_block(hash, data))
    };

    let relay_chain_slot_duration = Duration::from_secs(6);

    let mut start_collation: Option<
        Arc<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>,
    > = None;

    let node_builder = if collator {
        let (node_builder, import_queue) = node_builder.extract_import_queue_service();

        let collator_key = collator_key
            .clone()
            .expect("Command line arguments do not allow this. qed");

        let overseer_handle = relay_chain_interface
            .overseer_handle()
            .map_err(|e| sc_service::Error::Application(Box::new(e)))?;

        let parachain_consensus = build_consensus_container(
            node_builder.client.clone(),
            orchestrator_client.clone(),
            block_import,
            prometheus_registry.as_ref(),
            node_builder.telemetry.as_ref().map(|t| t.handle()),
            &node_builder.task_manager,
            relay_chain_interface.clone(),
            orchestrator_chain_interface.clone(),
            node_builder.transaction_pool.clone(),
            node_builder.network.sync_service.clone(),
            keystore,
            force_authoring,
            para_id,
            orchestrator_para_id,
        )?;

        // Given the sporadic nature of the explicit recovery operation and the
        // possibility to retry infinite times this value is more than enough.
        // In practice here we expect no more than one queued messages.
        const RECOVERY_CHAN_SIZE: usize = 8;

        let (recovery_chan_tx, recovery_chan_rx) = mpsc::channel(RECOVERY_CHAN_SIZE);

        let consensus = cumulus_client_consensus_common::run_parachain_consensus(
            para_id,
            node_builder.client.clone(),
            relay_chain_interface.clone(),
            announce_block,
            Some(recovery_chan_tx),
        );

        node_builder
            .task_manager
            .spawn_essential_handle()
            .spawn_blocking("cumulus-consensus", None, consensus);

        let pov_recovery = PoVRecovery::new(
            Box::new(overseer_handle.clone()),
            // We want that collators wait at maximum the relay chain slot duration before starting
            // to recover blocks. Additionally, we wait at least half the slot time to give the
            // relay chain the chance to increase availability.
            RecoveryDelayRange {
                min: relay_chain_slot_duration / 2,
                max: relay_chain_slot_duration,
            },
            node_builder.client.clone(),
            import_queue,
            relay_chain_interface.clone(),
            para_id,
            recovery_chan_rx,
            node_builder.network.sync_service.clone(),
        );

        node_builder.task_manager.spawn_essential_handle().spawn(
            "cumulus-pov-recovery",
            None,
            pov_recovery.run(),
        );

        let params_generator = node_builder.cumulus_client_collator_params_generator(
            para_id,
            overseer_handle,
            collator_key,
            parachain_consensus,
        );

        // Hack to fix logs, if this future is awaited by the ContainerChainSpawner thread,
        // the logs will say "Orchestrator" instead of "Container-2000".
        // Wrapping the future in this function fixes that.
        #[sc_tracing::logging::prefix_logs_with(container_log_str(para_id))]
        async fn wrap<F, O>(para_id: ParaId, f: F) -> O
        where
            F: Future<Output = O>,
        {
            f.await
        }

        start_collation = Some(Arc::new(move || {
            Box::pin(wrap(
                para_id,
                #[allow(deprecated)]
                cumulus_client_collator::start_collator(params_generator()),
            ))
        }));

        node_builder
    } else {
        node_builder.start_full_node(
            para_id,
            relay_chain_interface.clone(),
            relay_chain_slot_duration,
        )?
    };

    node_builder.network.start_network.start_network();

    Ok((
        node_builder.task_manager,
        node_builder.client,
        node_builder.backend,
        start_collation,
    ))
}

/// Build the import queue for the parachain runtime (manual seal).
fn build_manual_seal_import_queue(
    _client: Arc<ParachainClient>,
    block_import: DevParachainBlockImport,
    config: &Configuration,
    _telemetry: Option<TelemetryHandle>,
    task_manager: &TaskManager,
) -> Result<sc_consensus::DefaultImportQueue<Block>, sc_service::Error> {
    Ok(sc_consensus_manual_seal::import_queue(
        Box::new(block_import),
        &task_manager.spawn_essential_handle(),
        config.prometheus_registry(),
    ))
}

fn build_consensus_container(
    client: Arc<ParachainClient>,
    orchestrator_client: Arc<ParachainClient>,
    block_import: ParachainBlockImport,
    prometheus_registry: Option<&Registry>,
    telemetry: Option<TelemetryHandle>,
    task_manager: &TaskManager,
    relay_chain_interface: Arc<dyn RelayChainInterface>,
    orchestrator_chain_interface: Arc<dyn OrchestratorChainInterface>,
    transaction_pool: Arc<sc_transaction_pool::FullPool<Block, ParachainClient>>,
    sync_oracle: Arc<SyncingService<Block>>,
    keystore: KeystorePtr,
    force_authoring: bool,
    para_id: ParaId,
    orchestrator_para_id: ParaId,
) -> Result<Box<dyn ParachainConsensus<Block>>, sc_service::Error> {
    let slot_duration = cumulus_client_consensus_aura::slot_duration(&*orchestrator_client)?;

    let proposer_factory = sc_basic_authorship::ProposerFactory::with_proof_recording(
        task_manager.spawn_handle(),
        client.clone(),
        transaction_pool,
        prometheus_registry,
        telemetry.clone(),
    );

    let relay_chain_interace_for_orch = relay_chain_interface.clone();
    let orchestrator_client_for_cidp = orchestrator_client;

    let params = tc_consensus::BuildOrchestratorAuraConsensusParams {
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
                    ccp_authorities_noting_inherent::ContainerChainAuthoritiesInherentData::create_at(
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
        get_authorities_from_orchestrator: move |_block_hash, (relay_parent, _validation_data)| {
            let relay_chain_interace_for_orch = relay_chain_interace_for_orch.clone();
            let orchestrator_client_for_cidp = orchestrator_client_for_cidp.clone();

            async move {
                let latest_header =
                    ccp_authorities_noting_inherent::ContainerChainAuthoritiesInherentData::get_latest_orchestrator_head_info(
                        relay_parent,
                        &relay_chain_interace_for_orch,
                        orchestrator_para_id,
                    )
                    .await;

                let latest_header = latest_header.ok_or_else(|| {
                    Box::<dyn std::error::Error + Send + Sync>::from(
                        "Failed to fetch latest header",
                    )
                })?;

                let authorities = tc_consensus::authorities::<Block, ParachainClient, NimbusPair>(
                    orchestrator_client_for_cidp.as_ref(),
                    &latest_header.hash(),
                    para_id,
                );

                let aux_data = authorities.ok_or_else(|| {
                    Box::<dyn std::error::Error + Send + Sync>::from(
                        "Failed to fetch authorities with error",
                    )
                })?;

                log::info!(
                    "Authorities {:?} found for header {:?}",
                    aux_data,
                    latest_header
                );

                Ok(aux_data)
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

    Ok(tc_consensus::OrchestratorAuraConsensus::build::<
        NimbusPair,
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
    keystore: KeystorePtr,
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
    let client_set_aside_for_orch = client.clone();

    let params = BuildOrchestratorAuraConsensusParams {
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
                let para_ids: Vec<_> = para_ids.into_iter().collect();
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
        get_authorities_from_orchestrator:
            move |block_hash: H256, (_relay_parent, _validation_data)| {
                let client_set_aside_for_orch = client_set_aside_for_orch.clone();

                async move {
                    let authorities = tc_consensus::authorities::<Block, ParachainClient, NimbusPair>(
                        client_set_aside_for_orch.as_ref(),
                        &block_hash,
                        para_id,
                    );

                    let aux_data = authorities.ok_or_else(|| {
                        Box::<dyn std::error::Error + Send + Sync>::from(
                            "Failed to fetch authorities with error",
                        )
                    })?;

                    log::info!(
                        "Authorities {:?} found for header {:?}",
                        aux_data,
                        block_hash
                    );

                    Ok(aux_data)
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
    container_config: Option<(ContainerChainCli, tokio::runtime::Handle)>,
    collator_options: CollatorOptions,
    para_id: ParaId,
    hwbench: Option<sc_sysinfo::HwBench>,
) -> sc_service::error::Result<(TaskManager, Arc<ParachainClient>)> {
    start_node_impl(
        parachain_config,
        polkadot_config,
        container_config,
        collator_options,
        para_id,
        hwbench,
    )
    .await
}

pub const SOFT_DEADLINE_PERCENT: sp_runtime::Percent = sp_runtime::Percent::from_percent(100);

/// Start a node with the given parachain `Configuration` and relay chain `Configuration`.
///
/// This is the actual implementation that is abstract over the executor and the runtime api.
#[sc_tracing::logging::prefix_logs_with("Orchestrator Dev Node")]
pub fn start_dev_node(
    orchestrator_config: Configuration,
    sealing: Sealing,
    hwbench: Option<sc_sysinfo::HwBench>,
    para_id: ParaId,
) -> sc_service::error::Result<TaskManager> {
    let parachain_config = prepare_node_config(orchestrator_config);

    // Create a `NodeBuilder` which helps setup parachain nodes common systems.
    let node_builder = NodeConfig::new_builder(&parachain_config, hwbench)?;

    // This node block import.
    let block_import = DevParachainBlockImport::new(node_builder.client.clone());
    let import_queue = build_manual_seal_import_queue(
        node_builder.client.clone(),
        block_import.clone(),
        &parachain_config,
        node_builder
            .telemetry
            .as_ref()
            .map(|telemetry| telemetry.handle()),
        &node_builder.task_manager,
    )?;

    // Build a Substrate Network. (not cumulus since it is a dev node, it mocks
    // the relaychain)
    let mut node_builder = node_builder.build_substrate_network(&parachain_config, import_queue)?;

    // If we're running a collator dev node we must install manual seal block
    // production.
    let mut command_sink = None;
    let mut xcm_senders = None;
    if parachain_config.role.is_authority() {
        let client = node_builder.client.clone();
        let (downward_xcm_sender, downward_xcm_receiver) = flume::bounded::<Vec<u8>>(100);
        let (hrmp_xcm_sender, hrmp_xcm_receiver) = flume::bounded::<(ParaId, Vec<u8>)>(100);
        xcm_senders = Some((downward_xcm_sender, hrmp_xcm_sender));

        command_sink = node_builder.install_manual_seal(ManualSealConfiguration {
            block_import,
            sealing,
            soft_deadline: Some(SOFT_DEADLINE_PERCENT),
            select_chain: sc_consensus::LongestChain::new(node_builder.backend.clone()),
            consensus_data_provider: Some(Box::new(
                tc_consensus::OrchestratorManualSealAuraConsensusDataProvider::new(
                    node_builder.client.clone(),
                    node_builder.keystore_container.keystore(),
                    para_id,
                ),
            )),
            create_inherent_data_providers: move |block: H256, ()| {
                let current_para_block = client
                    .number(block)
                    .expect("Header lookup should succeed")
                    .expect("Header passed in as parent should be present in backend.");

                let para_ids = client
                    .runtime_api()
                    .registered_paras(block)
                    .expect("registered_paras runtime API should exist")
                    .into_iter()
                    .collect();

                let downward_xcm_receiver = downward_xcm_receiver.clone();
                let hrmp_xcm_receiver = hrmp_xcm_receiver.clone();

                let client_for_xcm = client.clone();
                async move {
                    let mocked_author_noting =
                        tp_author_noting_inherent::MockAuthorNotingInherentDataProvider {
                            current_para_block,
                            relay_offset: 1000,
                            relay_blocks_per_para_block: 2,
                            para_ids,
                            slots_per_para_block: 1,
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
                        additional_key_values: Some(mocked_author_noting.get_key_values()),
                    };

                    Ok((time, mocked_parachain, mocked_author_noting))
                }
            },
        })?;
    }

    // This node RPC builder.
    let rpc_builder = {
        let client = node_builder.client.clone();
        let transaction_pool = node_builder.transaction_pool.clone();

        Box::new(move |deny_unsafe, _| {
            let deps = crate::rpc::FullDeps {
                client: client.clone(),
                pool: transaction_pool.clone(),
                deny_unsafe,
                command_sink: command_sink.clone(),
                xcm_senders: xcm_senders.clone(),
            };

            crate::rpc::create_full(deps).map_err(Into::into)
        })
    };

    // We spawn all the common substrate tasks to properly run a node.
    let node_builder = node_builder.spawn_common_tasks(parachain_config, rpc_builder)?;

    log::info!("Development Service Ready");

    // We start the networking part.
    node_builder.network.start_network.start_network();

    Ok(node_builder.task_manager)
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

    async fn check_block(
        &mut self,
        block: sc_consensus::BlockCheckParams<Block>,
    ) -> Result<sc_consensus::ImportResult, Self::Error> {
        self.inner.check_block(block).await
    }

    async fn import_block(
        &mut self,
        params: sc_consensus::BlockImportParams<Block>,
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
        relevant_keys: &[Vec<u8>],
    ) -> OrchestratorChainResult<StorageProof> {
        let state_backend = self.backend.state_at(orchestrator_parent)?;

        sp_state_machine::prove_read(state_backend, relevant_keys)
            .map_err(OrchestratorChainError::StateMachineError)
    }

    fn overseer_handle(&self) -> OrchestratorChainResult<Handle> {
        Ok(self.overseer_handle.clone())
    }
}
