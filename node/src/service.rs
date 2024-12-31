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

use frame_support::__private::sp_tracing::tracing::Instrument;
use {
    crate::command::solochain::{
        build_solochain_config_dir, copy_zombienet_keystore, dummy_config, keystore_config,
    },
    core::marker::PhantomData,
    cumulus_client_cli::CollatorOptions,
    cumulus_client_collator::service::CollatorService,
    cumulus_client_consensus_proposer::Proposer,
    cumulus_client_parachain_inherent::{MockValidationDataInherentDataProvider, MockXcmConfig},
    cumulus_client_service::{
        prepare_node_config, start_relay_chain_tasks, DARecoveryProfile, StartRelayChainTasksParams,
    },
    cumulus_primitives_core::{
        relay_chain::{well_known_keys as RelayWellKnownKeys, CollatorPair},
        ParaId,
    },
    cumulus_relay_chain_interface::{
        call_remote_runtime_function, OverseerHandle, RelayChainInterface,
    },
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
    node_common::service::{ManualSealConfiguration, NodeBuilder, NodeBuilderConfig, Sealing},
    pallet_author_noting_runtime_api::AuthorNotingApi,
    pallet_data_preservers_runtime_api::DataPreserversApi,
    pallet_registrar_runtime_api::RegistrarApi,
    parity_scale_codec::{Decode, Encode},
    polkadot_cli::ProvideRuntimeApi,
    polkadot_parachain_primitives::primitives::HeadData,
    polkadot_service::Handle,
    sc_cli::CliConfiguration,
    sc_client_api::{
        AuxStore, Backend as BackendT, BlockchainEvents, HeaderBackend, UsageProvider,
    },
    sc_consensus::BasicQueue,
    sc_network::NetworkBlock,
    sc_network_common::role::Role,
    sc_network_sync::SyncingService,
    sc_service::{Configuration, KeystoreContainer, SpawnTaskHandle, TFullBackend, TaskManager},
    sc_telemetry::TelemetryHandle,
    sc_transaction_pool::TransactionPoolHandle,
    sp_api::StorageProof,
    sp_consensus::SyncOracle,
    sp_consensus_slots::Slot,
    sp_core::{traits::SpawnEssentialNamed, H256},
    sp_keystore::KeystorePtr,
    sp_state_machine::{Backend as StateBackend, StorageValue},
    std::{pin::Pin, sync::Arc, time::Duration},
    tc_consensus::{
        collators::lookahead::{
            self as lookahead_tanssi_aura, BuyCoreParams, Params as LookaheadTanssiAuraParams,
        },
        OnDemandBlockProductionApi, OrchestratorAuraWorkerAuxData, TanssiAuthorityAssignmentApi,
    },
    tc_service_container_chain::{
        cli::ContainerChainCli,
        monitor,
        service::{
            DevParachainBlockImport, ParachainBlockImport, ParachainClient, ParachainExecutor,
            ParachainProposerFactory,
        },
        spawner::{self, CcSpawnMsg, ContainerChainSpawnParams, ContainerChainSpawner},
    },
    tokio::sync::mpsc::{unbounded_channel, UnboundedSender},
    tokio_util::sync::CancellationToken,
};

mod mocked_relay_keys;

// We use this to detect whether randomness is activated
const RANDOMNESS_ACTIVATED_AUX_KEY: &[u8] = b"__DEV_RANDOMNESS_ACTIVATED";

type FullBackend = TFullBackend<Block>;

pub struct NodeConfig;
impl NodeBuilderConfig for NodeConfig {
    type Block = Block;
    type RuntimeApi = RuntimeApi;
    type ParachainExecutor = ParachainExecutor;
}

thread_local!(static TIMESTAMP: std::cell::RefCell<u64> = const { std::cell::RefCell::new(0) });

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
    client: Arc<dyn OrchestratorChainInterface>,
    sync_keystore: KeystorePtr,
    cc_spawn_tx: UnboundedSender<CcSpawnMsg>,
    spawner: impl SpawnEssentialNamed,
) {
    let check_assigned_para_id_task = async move {
        // Subscribe to new blocks in order to react to para id assignment
        // This must be the stream of finalized blocks, otherwise the collators may rotate to a
        // different chain before the block is finalized, and that could lead to a stalled chain
        let mut import_notifications = client.finality_notification_stream().await.unwrap();

        while let Some(msg) = import_notifications.next().await {
            let block_hash = msg.hash();
            let client_set_aside_for_cidp = client.clone();
            let sync_keystore = sync_keystore.clone();
            let cc_spawn_tx = cc_spawn_tx.clone();

            check_assigned_para_id(
                cc_spawn_tx,
                sync_keystore,
                client_set_aside_for_cidp,
                block_hash,
            )
            .await
            .unwrap();
        }
    };

    spawner.spawn_essential(
        "check-assigned-para-id",
        None,
        Box::pin(check_assigned_para_id_task),
    );
}

/// Check the parachain assignment using the orchestrator chain client, and send a `CcSpawnMsg` to
/// start or stop the required container chains.
///
/// Checks the assignment for the next block, so if there is a session change on block 15, this will
/// detect the assignment change after importing block 14.
async fn check_assigned_para_id(
    cc_spawn_tx: UnboundedSender<CcSpawnMsg>,
    sync_keystore: KeystorePtr,
    client_set_aside_for_cidp: Arc<dyn OrchestratorChainInterface>,
    block_hash: H256,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Check current assignment
    let current_container_chain_para_id =
        tc_consensus::first_eligible_key::<dyn OrchestratorChainInterface, NimbusPair>(
            client_set_aside_for_cidp.as_ref(),
            &block_hash,
            sync_keystore.clone(),
        )
        .await
        .map(|(_nimbus_key, para_id)| para_id);

    // Check assignment in the next session
    let next_container_chain_para_id = tc_consensus::first_eligible_key_next_session::<
        dyn OrchestratorChainInterface,
        NimbusPair,
    >(
        client_set_aside_for_cidp.as_ref(),
        &block_hash,
        sync_keystore,
    )
    .await
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
        .build_cumulus_network::<_, sc_network::NetworkWorker<_, _>>(
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
            let deps = crate::rpc::FullDeps {
                client: client.clone(),
                pool: transaction_pool.clone(),
                command_sink: None,
                xcm_senders: None,
                randomness_sender: None,
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
    })?;

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
            build_check_assigned_para_id(
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
                )
            }
        };
        // Save callback for later, used when collator rotates from container chain back to orchestrator chain
        collate_on_tanssi = Arc::new(start_collation);
    }

    node_builder.network.start_network.start_network();

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
                orchestrator_para_id: para_id,
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
                generate_rpc_builder: tc_service_container_chain::rpc::GenerateSubstrateRpcBuilder::<
                    dancebox_runtime::RuntimeApi,
                >::new(),
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
        )
    }

    Ok((node_builder.task_manager, node_builder.client))
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
    .instrument(sc_tracing::tracing::info_span!(
        sc_tracing::logging::PREFIX_LOG_SPAN,
        name = "Orchestrator",
    ))
    .await
}

/// Start a solochain node.
pub async fn start_solochain_node(
    polkadot_config: Configuration,
    container_chain_cli: ContainerChainCli,
    collator_options: CollatorOptions,
    hwbench: Option<sc_sysinfo::HwBench>,
) -> sc_service::error::Result<TaskManager> {
    let tokio_handle = polkadot_config.tokio_handle.clone();
    let orchestrator_para_id = Default::default();

    let chain_type = polkadot_config.chain_spec.chain_type().clone();
    let relay_chain = polkadot_config.chain_spec.id().to_string();

    let base_path = container_chain_cli
        .base
        .base
        .shared_params
        .base_path
        .as_ref()
        .expect("base_path is always set");
    let config_dir = build_solochain_config_dir(base_path);
    let keystore = keystore_config(container_chain_cli.keystore_params(), &config_dir)
        .map_err(|e| sc_service::Error::Application(Box::new(e) as Box<_>))?;

    // Instead of putting keystore in
    // Collator1000-01/data/chains/simple_container_2000/keystore
    // We put it in
    // Collator1000-01/data/config/keystore
    // And same for "network" folder
    // But zombienet will put the keys in the old path, so we need to manually copy it if we
    // are running under zombienet
    copy_zombienet_keystore(&keystore)?;

    let keystore_container = KeystoreContainer::new(&keystore)?;

    // No metrics so no prometheus registry
    let prometheus_registry = None;
    let mut task_manager = TaskManager::new(tokio_handle.clone(), prometheus_registry)?;

    // Each container chain will spawn its own telemetry
    let telemetry_worker_handle = None;

    // Dummy parachain config only needed because `build_relay_chain_interface` needs to know if we
    // are collators or not
    let validator = container_chain_cli.base.collator;
    let mut dummy_parachain_config = dummy_config(
        polkadot_config.tokio_handle.clone(),
        polkadot_config.base_path.clone(),
    );
    dummy_parachain_config.role = if validator {
        Role::Authority
    } else {
        Role::Full
    };
    let (relay_chain_interface, collator_key) =
        cumulus_client_service::build_relay_chain_interface(
            polkadot_config,
            &dummy_parachain_config,
            telemetry_worker_handle.clone(),
            &mut task_manager,
            collator_options.clone(),
            hwbench.clone(),
        )
        .await
        .map_err(|e| sc_service::Error::Application(Box::new(e) as Box<_>))?;

    log::info!("start_solochain_node: is validator? {}", validator);

    let overseer_handle = relay_chain_interface
        .overseer_handle()
        .map_err(|e| sc_service::Error::Application(Box::new(e)))?;
    let sync_keystore = keystore_container.keystore();
    let collate_on_tanssi: Arc<
        dyn Fn() -> (CancellationToken, futures::channel::oneshot::Receiver<()>) + Send + Sync,
    > = Arc::new(move || {
        // collate_on_tanssi will not be called in solochains because solochains use a different consensus
        // mechanism and need validators instead of collators.
        // The runtime enforces this because the orchestrator_chain is never assigned any collators.
        panic!("Called collate_on_tanssi on solochain collator. This is unsupported and the runtime shouldn't allow this, it is a bug")
    });

    let orchestrator_chain_interface_builder = OrchestratorChainSolochainInterfaceBuilder {
        overseer_handle: overseer_handle.clone(),
        relay_chain_interface: relay_chain_interface.clone(),
    };
    let orchestrator_chain_interface = orchestrator_chain_interface_builder.build();
    // Channel to send messages to start/stop container chains
    let (cc_spawn_tx, cc_spawn_rx) = unbounded_channel();

    if validator {
        // Start task which detects para id assignment, and starts/stops container chains.
        build_check_assigned_para_id(
            orchestrator_chain_interface.clone(),
            sync_keystore.clone(),
            cc_spawn_tx.clone(),
            task_manager.spawn_essential_handle(),
        );
    }

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
    let spawn_handle = task_manager.spawn_handle();

    let container_chain_spawner = ContainerChainSpawner {
        params: ContainerChainSpawnParams {
            orchestrator_chain_interface,
            container_chain_cli,
            tokio_handle,
            chain_type,
            relay_chain,
            relay_chain_interface,
            sync_keystore,
            orchestrator_para_id,
            collation_params: if validator {
                Some(spawner::CollationParams {
                    // TODO: all these args must be solochain instead of orchestrator
                    orchestrator_client: None,
                    orchestrator_tx_pool: None,
                    orchestrator_para_id,
                    collator_key: collator_key
                        .expect("there should be a collator key if we're a validator"),
                    solochain: true,
                })
            } else {
                None
            },
            spawn_handle,
            data_preserver: false,
            generate_rpc_builder: tc_service_container_chain::rpc::GenerateSubstrateRpcBuilder::<
                dancebox_runtime::RuntimeApi,
            >::new(),
            phantom: PhantomData,
        },
        state: Default::default(),
        db_folder_cleanup_done: false,
        collate_on_tanssi,
        collation_cancellation_constructs: None,
    };
    let state = container_chain_spawner.state.clone();

    task_manager.spawn_essential_handle().spawn(
        "container-chain-spawner-rx-loop",
        None,
        container_chain_spawner.rx_loop(cc_spawn_rx, validator, true),
    );

    task_manager.spawn_essential_handle().spawn(
        "container-chain-spawner-debug-state",
        None,
        monitor::monitor_task(state),
    );

    Ok(task_manager)
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
    let mut node_builder = node_builder
        .build_substrate_network::<sc_network::NetworkWorker<_, _>>(
            &parachain_config,
            import_queue,
        )?;

    // If we're running a collator dev node we must install manual seal block
    // production.
    let mut command_sink = None;
    let mut xcm_senders = None;
    let mut randomness_sender = None;
    if parachain_config.role.is_authority() {
        let client = node_builder.client.clone();
        let (downward_xcm_sender, downward_xcm_receiver) = flume::bounded::<Vec<u8>>(100);
        let (hrmp_xcm_sender, hrmp_xcm_receiver) = flume::bounded::<(ParaId, Vec<u8>)>(100);
        // Create channels for mocked parachain candidates.
        let (mock_randomness_sender, mock_randomness_receiver) =
            flume::bounded::<(bool, Option<[u8; 32]>)>(100);

        xcm_senders = Some((downward_xcm_sender, hrmp_xcm_sender));
        randomness_sender = Some(mock_randomness_sender);

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

                let hash = client
                    .hash(current_para_block.saturating_sub(1))
                    .expect("Hash of the desired block must be present")
                    .expect("Hash of the desired block should exist");

                let para_header = client
                    .expect_header(hash)
                    .expect("Expected parachain header should exist")
                    .encode();

                let para_head_data = HeadData(para_header).encode();
                let para_head_key = RelayWellKnownKeys::para_head(para_id);
                let relay_slot_key = RelayWellKnownKeys::CURRENT_SLOT.to_vec();

                let slot_duration = sc_consensus_aura::standalone::slot_duration_at(
                    &*client.clone(),
                    block,
                ).expect("Slot duration should be set");

                let mut timestamp = 0u64;
                TIMESTAMP.with(|x| {
                    timestamp = x.clone().take();
                });

                timestamp += dancebox_runtime::SLOT_DURATION;
                let relay_slot = sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
						timestamp.into(),
						slot_duration,
                    );
                let relay_slot = u64::from(*relay_slot);

                let downward_xcm_receiver = downward_xcm_receiver.clone();
                let hrmp_xcm_receiver = hrmp_xcm_receiver.clone();

                let randomness_enabler_messages: Vec<(bool, Option<[u8; 32]>)> = mock_randomness_receiver.drain().collect();

                // If there is a value to be updated, we update it
                if let Some((enable_randomness, new_seed)) = randomness_enabler_messages.last() {
                    let value = client
                        .get_aux(RANDOMNESS_ACTIVATED_AUX_KEY)
                        .expect("Should be able to query aux storage; qed").unwrap_or((false, Option::<[u8; 32]>::None).encode());
                    let (_mock_additional_randomness, mut mock_randomness_seed): (bool, Option<[u8; 32]>) = Decode::decode(&mut value.as_slice()).expect("Boolean non-decodable");

                    if let Some(new_seed) = new_seed {
                        mock_randomness_seed = Some(*new_seed);
                    }

                    client
                    .insert_aux(
                        &[(RANDOMNESS_ACTIVATED_AUX_KEY, (enable_randomness, mock_randomness_seed).encode().as_slice())],
                        &[],
                    )
                    .expect("Should be able to write to aux storage; qed");
                }

                // We read the value
                // If error when reading, we simply put false
                let value = client
                    .get_aux(RANDOMNESS_ACTIVATED_AUX_KEY)
                    .expect("Should be able to query aux storage; qed").unwrap_or((false, Option::<[u8; 32]>::None).encode());
                let (mock_additional_randomness, mock_randomness_seed): (bool, Option<[u8; 32]>) = Decode::decode(&mut value.as_slice()).expect("Boolean non-decodable");

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
                    let mut additional_keys = mocked_author_noting.get_key_values();
                    // Mock only chain 2002 in relay.
                    // This will allow any signed origin to deregister chains 2000 and 2001, and register 2002.
                    let (registrar_paras_key_2002, para_info_2002) = mocked_relay_keys::get_mocked_registrar_paras(2002.into());
                    additional_keys.extend([(para_head_key, para_head_data), (relay_slot_key, Slot::from(relay_slot).encode()), (registrar_paras_key_2002, para_info_2002)]);

                    if mock_additional_randomness {
                        let mut mock_randomness: [u8; 32] = [0u8; 32];
                        mock_randomness[..4].copy_from_slice(&current_para_block.to_be_bytes());
                        if let Some(seed) = mock_randomness_seed {
                            for i in 0..32 {
                                mock_randomness[i] ^= seed[i];
                            }
                        }
                        additional_keys.extend([(RelayWellKnownKeys::CURRENT_BLOCK_RANDOMNESS.to_vec(), Some(mock_randomness).encode())]);
                        log::info!("mokcing randomnessss!!! {}", current_para_block);
                    }

                    let time = MockTimestampInherentDataProvider;
                    let mocked_parachain = MockValidationDataInherentDataProvider {
                        current_para_block,
                        current_para_block_head: None,
                        relay_offset: 1000,
                        relay_blocks_per_para_block: 2,
                        // TODO: Recheck
                        para_blocks_per_relay_epoch: 10,
                        relay_randomness_config: (),
                        xcm_config: MockXcmConfig::new(
                            &*client_for_xcm,
                            block,
                            Default::default(),
                        ),
                        raw_downward_messages: downward_xcm_receiver.drain().collect(),
                        raw_horizontal_messages: hrmp_xcm_receiver.drain().collect(),
                        additional_key_values: Some(additional_keys),
                        para_id,
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

        Box::new(move |_| {
            let deps = crate::rpc::FullDeps {
                client: client.clone(),
                pool: transaction_pool.clone(),
                command_sink: command_sink.clone(),
                xcm_senders: xcm_senders.clone(),
                randomness_sender: randomness_sender.clone(),
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

/// Builder for a concrete relay chain interface, created from a full node. Builds
/// a [`RelayChainInProcessInterface`] to access relay chain data necessary for parachain operation.
///
/// The builder takes a [`polkadot_client::Client`]
/// that wraps a concrete instance. By using [`polkadot_client::ExecuteWithClient`]
/// the builder gets access to this concrete instance and instantiates a [`RelayChainInProcessInterface`] with it.
struct OrchestratorChainSolochainInterfaceBuilder {
    overseer_handle: Handle,
    relay_chain_interface: Arc<dyn RelayChainInterface>,
}

impl OrchestratorChainSolochainInterfaceBuilder {
    pub fn build(self) -> Arc<dyn OrchestratorChainInterface> {
        Arc::new(OrchestratorChainSolochainInterface::new(
            self.overseer_handle,
            self.relay_chain_interface,
        ))
    }
}

/// Provides an implementation of the [`RelayChainInterface`] using a local in-process relay chain node.
pub struct OrchestratorChainInProcessInterface<Client> {
    pub full_client: Arc<Client>,
    pub backend: Arc<FullBackend>,
    pub sync_oracle: Arc<dyn SyncOracle + Send + Sync>,
    pub overseer_handle: Handle,
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

/// Provides an implementation of the [`RelayChainInterface`] using a local in-process relay chain node.
pub struct OrchestratorChainSolochainInterface {
    pub overseer_handle: Handle,
    pub relay_chain_interface: Arc<dyn RelayChainInterface>,
}

impl OrchestratorChainSolochainInterface {
    /// Create a new instance of [`RelayChainInProcessInterface`]
    pub fn new(
        overseer_handle: Handle,
        relay_chain_interface: Arc<dyn RelayChainInterface>,
    ) -> Self {
        Self {
            overseer_handle,
            relay_chain_interface,
        }
    }
}

#[async_trait::async_trait]
impl OrchestratorChainInterface for OrchestratorChainSolochainInterface {
    async fn get_storage_by_key(
        &self,
        relay_parent: PHash,
        key: &[u8],
    ) -> OrchestratorChainResult<Option<StorageValue>> {
        self.relay_chain_interface
            .get_storage_by_key(relay_parent, key)
            .await
            .map_err(|e| OrchestratorChainError::Application(Box::new(e)))
    }

    async fn prove_read(
        &self,
        relay_parent: PHash,
        relevant_keys: &Vec<Vec<u8>>,
    ) -> OrchestratorChainResult<StorageProof> {
        self.relay_chain_interface
            .prove_read(relay_parent, relevant_keys)
            .await
            .map_err(|e| OrchestratorChainError::Application(Box::new(e)))
    }

    fn overseer_handle(&self) -> OrchestratorChainResult<Handle> {
        Ok(self.overseer_handle.clone())
    }

    /// Get a stream of import block notifications.
    async fn import_notification_stream(
        &self,
    ) -> OrchestratorChainResult<Pin<Box<dyn Stream<Item = PHeader> + Send>>> {
        self.relay_chain_interface
            .import_notification_stream()
            .await
            .map_err(|e| OrchestratorChainError::Application(Box::new(e)))
    }

    /// Get a stream of new best block notifications.
    async fn new_best_notification_stream(
        &self,
    ) -> OrchestratorChainResult<Pin<Box<dyn Stream<Item = PHeader> + Send>>> {
        self.relay_chain_interface
            .new_best_notification_stream()
            .await
            .map_err(|e| OrchestratorChainError::Application(Box::new(e)))
    }

    /// Get a stream of finality notifications.
    async fn finality_notification_stream(
        &self,
    ) -> OrchestratorChainResult<Pin<Box<dyn Stream<Item = PHeader> + Send>>> {
        self.relay_chain_interface
            .finality_notification_stream()
            .await
            .map_err(|e| OrchestratorChainError::Application(Box::new(e)))
    }

    async fn genesis_data(
        &self,
        relay_parent: PHash,
        para_id: ParaId,
    ) -> OrchestratorChainResult<Option<ContainerChainGenesisData>> {
        let res: Option<ContainerChainGenesisData> = call_remote_runtime_function(
            &self.relay_chain_interface,
            "RegistrarApi_genesis_data",
            relay_parent,
            &para_id,
        )
        .await
        .map_err(|e| OrchestratorChainError::Application(Box::new(e)))?;

        Ok(res)
    }

    async fn boot_nodes(
        &self,
        relay_parent: PHash,
        para_id: ParaId,
    ) -> OrchestratorChainResult<Vec<Vec<u8>>> {
        let res: Vec<Vec<u8>> = call_remote_runtime_function(
            &self.relay_chain_interface,
            "RegistrarApi_boot_nodes",
            relay_parent,
            &para_id,
        )
        .await
        .map_err(|e| OrchestratorChainError::Application(Box::new(e)))?;

        Ok(res)
    }

    async fn latest_block_number(
        &self,
        relay_parent: PHash,
        para_id: ParaId,
    ) -> OrchestratorChainResult<Option<BlockNumber>> {
        let res: Option<BlockNumber> = call_remote_runtime_function(
            &self.relay_chain_interface,
            "AuthorNotingApi_latest_block_number",
            relay_parent,
            &para_id,
        )
        .await
        .map_err(|e| OrchestratorChainError::Application(Box::new(e)))?;

        Ok(res)
    }

    async fn best_block_hash(&self) -> OrchestratorChainResult<PHash> {
        self.relay_chain_interface
            .best_block_hash()
            .await
            .map_err(|e| OrchestratorChainError::Application(Box::new(e)))
    }

    async fn finalized_block_hash(&self) -> OrchestratorChainResult<PHash> {
        self.relay_chain_interface
            .finalized_block_hash()
            .await
            .map_err(|e| OrchestratorChainError::Application(Box::new(e)))
    }

    async fn data_preserver_active_assignment(
        &self,
        _orchestrator_parent: PHash,
        _profile_id: DataPreserverProfileId,
    ) -> OrchestratorChainResult<DataPreserverAssignment<ParaId>> {
        unimplemented!("Data preserver node does not support Dancelight yet")
    }

    async fn check_para_id_assignment(
        &self,
        relay_parent: PHash,
        authority: NimbusId,
    ) -> OrchestratorChainResult<Option<ParaId>> {
        let res: Option<ParaId> = call_remote_runtime_function(
            &self.relay_chain_interface,
            "TanssiAuthorityAssignmentApi_check_para_id_assignment",
            relay_parent,
            &authority,
        )
        .await
        .map_err(|e| OrchestratorChainError::Application(Box::new(e)))?;

        Ok(res)
    }

    async fn check_para_id_assignment_next_session(
        &self,
        relay_parent: PHash,
        authority: NimbusId,
    ) -> OrchestratorChainResult<Option<ParaId>> {
        let res: Option<ParaId> = call_remote_runtime_function(
            &self.relay_chain_interface,
            "TanssiAuthorityAssignmentApi_check_para_id_assignment_next_session",
            relay_parent,
            &authority,
        )
        .await
        .map_err(|e| OrchestratorChainError::Application(Box::new(e)))?;

        Ok(res)
    }
}
