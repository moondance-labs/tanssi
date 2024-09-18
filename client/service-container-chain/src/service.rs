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

use {
    cumulus_client_consensus_common::{
        ParachainBlockImport as TParachainBlockImport, ParachainBlockImportMarker,
    },
    cumulus_client_service::{
        prepare_node_config, start_relay_chain_tasks, DARecoveryProfile, ParachainHostFunctions,
        StartRelayChainTasksParams,
    },
    cumulus_primitives_core::ParaId,
    cumulus_relay_chain_interface::{
        call_remote_runtime_function, OverseerHandle, RelayChainInterface,
    },
    dancebox_runtime::{
        opaque::{Block, Hash},
        RuntimeApi,
    },
    dc_orchestrator_chain_interface::OrchestratorChainInterface,
    dp_slot_duration_runtime_api::TanssiSlotDurationApi,
    nimbus_primitives::{NimbusId, NimbusPair},
    node_common::service::{NodeBuilder, NodeBuilderConfig},
    sc_basic_authorship::ProposerFactory,
    sc_consensus::{BasicQueue, BlockImport},
    sc_executor::WasmExecutor,
    sc_network::NetworkBlock,
    sc_network_sync::SyncingService,
    sc_service::{
        Configuration, ImportQueue, SpawnTaskHandle, TFullBackend, TFullClient, TaskManager,
    },
    sc_telemetry::TelemetryHandle,
    sp_api::ProvideRuntimeApi,
    sp_consensus::EnableProofRecording,
    sp_consensus_aura::SlotDuration,
    sp_keystore::KeystorePtr,
    std::{sync::Arc, time::Duration},
    substrate_prometheus_endpoint::Registry,
    tc_consensus::{
        collators::lookahead::{
            self as lookahead_tanssi_aura, Params as LookaheadTanssiAuraParams,
        },
        OrchestratorAuraWorkerAuxData,
    },
    tokio_util::sync::CancellationToken,
};

#[allow(deprecated)]
use sc_executor::NativeElseWasmExecutor;

type FullBackend = TFullBackend<Block>;

/// Native executor type.
pub struct ParachainNativeExecutor;

impl sc_executor::NativeExecutionDispatch for ParachainNativeExecutor {
    type ExtendHostFunctions = ParachainHostFunctions;

    fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
        dancebox_runtime::api::dispatch(method, data)
    }

    fn native_version() -> sc_executor::NativeVersion {
        dancebox_runtime::native_version()
    }
}

pub struct ContainerChainNodeConfig;
impl NodeBuilderConfig for ContainerChainNodeConfig {
    type Block = Block;
    // TODO: RuntimeApi here should be the subset of runtime apis available for all containers
    // Currently we are using the orchestrator runtime apis
    type RuntimeApi = RuntimeApi;
    type ParachainExecutor = ContainerChainExecutor;
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
    BI: BlockImport<Block> + Send + Sync,
{
    type Error = BI::Error;

    async fn check_block(
        &self,
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

// Orchestrator chain types
#[allow(deprecated)]
pub type ParachainExecutor = NativeElseWasmExecutor<ParachainNativeExecutor>;
pub type ParachainClient = TFullClient<Block, RuntimeApi, ParachainExecutor>;
pub type ParachainBackend = TFullBackend<Block>;
pub type DevParachainBlockImport = OrchestratorParachainBlockImport<Arc<ParachainClient>>;
pub type ParachainBlockImport =
    TParachainBlockImport<Block, Arc<ParachainClient>, ParachainBackend>;
pub type ParachainProposerFactory = ProposerFactory<
    sc_transaction_pool::TransactionPoolImpl<Block, ParachainClient>,
    ParachainClient,
    EnableProofRecording,
>;

// Container chains types
type ContainerChainExecutor = WasmExecutor<ParachainHostFunctions>;
pub type ContainerChainClient = TFullClient<Block, RuntimeApi, ContainerChainExecutor>;
pub type ContainerChainBackend = TFullBackend<Block>;
type ContainerChainBlockImport =
    TParachainBlockImport<Block, Arc<ContainerChainClient>, ContainerChainBackend>;

/// Start a node with the given parachain `Configuration` and relay chain `Configuration`.
///
/// This is the actual implementation that is abstract over the executor and the runtime api.
#[sc_tracing::logging::prefix_logs_with(container_log_str(para_id))]
pub async fn start_node_impl_container(
    parachain_config: Configuration,
    relay_chain_interface: Arc<dyn RelayChainInterface>,
    orchestrator_chain_interface: Arc<dyn OrchestratorChainInterface>,
    keystore: KeystorePtr,
    para_id: ParaId,
    collation_params: Option<crate::spawner::CollationParams>,
) -> sc_service::error::Result<(
    TaskManager,
    Arc<ContainerChainClient>,
    Arc<ParachainBackend>,
)> {
    let parachain_config = prepare_node_config(parachain_config);

    // Create a `NodeBuilder` which helps setup parachain nodes common systems.
    let node_builder = ContainerChainNodeConfig::new_builder(&parachain_config, None)?;

    let (block_import, import_queue) =
        container_chain_import_queue(&parachain_config, &node_builder);
    let import_queue_service = import_queue.service();

    log::info!("are we collators? {:?}", collation_params.is_some());
    let node_builder = node_builder
        .build_cumulus_network::<_, sc_network::NetworkWorker<_, _>>(
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

    let overseer_handle = relay_chain_interface
        .overseer_handle()
        .map_err(|e| sc_service::Error::Application(Box::new(e)))?;
    let (mut node_builder, _) = node_builder.extract_import_queue_service();

    start_relay_chain_tasks(StartRelayChainTasksParams {
        client: node_builder.client.clone(),
        announce_block: announce_block.clone(),
        para_id,
        relay_chain_interface: relay_chain_interface.clone(),
        task_manager: &mut node_builder.task_manager,
        da_recovery_profile: if collation_params.is_some() {
            DARecoveryProfile::Collator
        } else {
            DARecoveryProfile::FullNode
        },
        import_queue: import_queue_service,
        relay_chain_slot_duration,
        recovery_handle: Box::new(overseer_handle.clone()),
        sync_service: node_builder.network.sync_service.clone(),
    })?;

    if let Some(collation_params) = collation_params {
        let node_spawn_handle = node_builder.task_manager.spawn_handle().clone();
        let node_client = node_builder.client.clone();
        let node_backend = node_builder.backend.clone();

        start_consensus_container(
            node_client.clone(),
            node_backend.clone(),
            collation_params,
            block_import.clone(),
            prometheus_registry.clone(),
            node_builder.telemetry.as_ref().map(|t| t.handle()).clone(),
            node_spawn_handle.clone(),
            relay_chain_interface.clone(),
            orchestrator_chain_interface.clone(),
            node_builder.transaction_pool.clone(),
            node_builder.network.sync_service.clone(),
            keystore.clone(),
            force_authoring,
            relay_chain_slot_duration,
            para_id,
            overseer_handle.clone(),
            announce_block.clone(),
        );
    }

    node_builder.network.start_network.start_network();

    Ok((
        node_builder.task_manager,
        node_builder.client,
        node_builder.backend,
    ))
}

pub fn container_chain_import_queue(
    parachain_config: &Configuration,
    node_builder: &NodeBuilder<ContainerChainNodeConfig>,
) -> (ContainerChainBlockImport, BasicQueue<Block>) {
    // The nimbus import queue ONLY checks the signature correctness
    // Any other checks corresponding to the author-correctness should be done
    // in the runtime
    let block_import =
        ContainerChainBlockImport::new(node_builder.client.clone(), node_builder.backend.clone());

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

#[sc_tracing::logging::prefix_logs_with(container_log_str(para_id))]
fn start_consensus_container(
    client: Arc<ContainerChainClient>,
    backend: Arc<FullBackend>,
    collation_params: crate::spawner::CollationParams,
    block_import: ContainerChainBlockImport,
    prometheus_registry: Option<Registry>,
    telemetry: Option<TelemetryHandle>,
    spawner: SpawnTaskHandle,
    relay_chain_interface: Arc<dyn RelayChainInterface>,
    orchestrator_chain_interface: Arc<dyn OrchestratorChainInterface>,
    transaction_pool: Arc<sc_transaction_pool::TransactionPoolImpl<Block, ContainerChainClient>>,
    sync_oracle: Arc<SyncingService<Block>>,
    keystore: KeystorePtr,
    force_authoring: bool,
    relay_chain_slot_duration: Duration,
    para_id: ParaId,
    overseer_handle: OverseerHandle,
    announce_block: Arc<dyn Fn(Hash, Option<Vec<u8>>) + Send + Sync>,
) {
    let crate::spawner::CollationParams {
        collator_key,
        orchestrator_tx_pool,
        orchestrator_client,
        orchestrator_para_id,
        solochain,
    } = collation_params;
    let slot_duration = if solochain {
        // Solochains use Babe instead of Aura, which has 6s slot duration
        let relay_slot_ms = relay_chain_slot_duration.as_millis();
        SlotDuration::from_millis(
            u64::try_from(relay_slot_ms).expect("relay chain slot duration overflows u64"),
        )
    } else {
        cumulus_client_consensus_aura::slot_duration(&*orchestrator_client)
            .expect("start_consensus_container: slot duration should exist")
    };

    let proposer_factory = sc_basic_authorship::ProposerFactory::with_proof_recording(
        spawner.clone(),
        client.clone(),
        transaction_pool,
        prometheus_registry.as_ref(),
        telemetry.clone(),
    );

    let proposer = cumulus_client_consensus_proposer::Proposer::new(proposer_factory);

    let collator_service = cumulus_client_collator::service::CollatorService::new(
        client.clone(),
        Arc::new(spawner.clone()),
        announce_block,
        client.clone(),
    );

    let relay_chain_interace_for_cidp = relay_chain_interface.clone();
    let relay_chain_interace_for_orch = relay_chain_interface.clone();
    let orchestrator_client_for_cidp = orchestrator_client.clone();
    let client_for_cidp = client.clone();
    let client_for_hash_provider = client.clone();
    let client_for_slot_duration = client.clone();

    let code_hash_provider = move |block_hash| {
        client_for_hash_provider
            .code_at(block_hash)
            .ok()
            .map(polkadot_primitives::ValidationCode)
            .map(|c| c.hash())
    };

    let params = LookaheadTanssiAuraParams {
        get_current_slot_duration: move |block_hash| {
            // Default to 12s if runtime API does not exist
            let slot_duration_ms = client_for_slot_duration
                .runtime_api()
                .slot_duration(block_hash)
                .unwrap_or(12_000);

            SlotDuration::from_millis(slot_duration_ms)
        },
        create_inherent_data_providers: move |block_hash, (relay_parent, _validation_data)| {
            let relay_chain_interface = relay_chain_interace_for_cidp.clone();
            let orchestrator_chain_interface = orchestrator_chain_interface.clone();
            let client = client_for_cidp.clone();

            async move {
                let authorities_noting_inherent = if solochain {
                    ccp_authorities_noting_inherent::ContainerChainAuthoritiesInherentData::create_at_solochain(
                        relay_parent,
                        &relay_chain_interface,
                    )
                        .await
                } else {
                    ccp_authorities_noting_inherent::ContainerChainAuthoritiesInherentData::create_at(
                        relay_parent,
                        &relay_chain_interface,
                        &orchestrator_chain_interface,
                        orchestrator_para_id,
                    )
                        .await
                };

                let slot_duration = {
                    // Default to 12s if runtime API does not exist
                    let slot_duration_ms = client
                        .runtime_api()
                        .slot_duration(block_hash)
                        .unwrap_or(12_000);

                    SlotDuration::from_millis(slot_duration_ms)
                };

                let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

                let slot =
						sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
							*timestamp,
							slot_duration,
						);

                let authorities_noting_inherent = authorities_noting_inherent.ok_or_else(|| {
                    Box::<dyn std::error::Error + Send + Sync>::from(
                        "Failed to create authoritiesnoting inherent",
                    )
                })?;

                Ok((slot, timestamp, authorities_noting_inherent))
            }
        },
        get_orchestrator_aux_data: move |_block_hash, (relay_parent, _validation_data)| {
            let relay_chain_interace_for_orch = relay_chain_interace_for_orch.clone();
            let orchestrator_client_for_cidp = orchestrator_client_for_cidp.clone();

            async move {
                if solochain {
                    let authorities: Option<Vec<NimbusId>> = call_remote_runtime_function(
                        &relay_chain_interace_for_orch,
                        "TanssiAuthorityAssignmentApi_para_id_authorities",
                        relay_parent,
                        &para_id,
                    )
                    .await?;

                    let authorities = authorities.ok_or_else(|| {
                        Box::<dyn std::error::Error + Send + Sync>::from(
                            "Failed to fetch authorities with error",
                        )
                    })?;

                    log::info!(
                        "Authorities {:?} found for header {:?}",
                        authorities,
                        relay_parent
                    );

                    let slot_freq: Option<_> = call_remote_runtime_function(
                        &relay_chain_interace_for_orch,
                        "OnDemandBlockProductionApi_parathread_slot_frequency",
                        relay_parent,
                        &para_id,
                    )
                    .await?;

                    let aux_data = OrchestratorAuraWorkerAuxData {
                        authorities,
                        slot_freq,
                    };

                    Ok(aux_data)
                } else {
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

                    let authorities = authorities.ok_or_else(|| {
                        Box::<dyn std::error::Error + Send + Sync>::from(
                            "Failed to fetch authorities with error",
                        )
                    })?;

                    log::info!(
                        "Authorities {:?} found for header {:?}",
                        authorities,
                        latest_header
                    );

                    let slot_freq = tc_consensus::min_slot_freq::<Block, ParachainClient, NimbusPair>(
                        orchestrator_client_for_cidp.as_ref(),
                        &latest_header.hash(),
                        para_id,
                    );

                    let aux_data = OrchestratorAuraWorkerAuxData {
                        authorities,
                        slot_freq,
                    };

                    Ok(aux_data)
                }
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
        force_authoring,
        relay_chain_slot_duration,
        proposer,
        collator_service,
        authoring_duration: Duration::from_millis(2000),
        para_backend: backend,
        code_hash_provider,
        // This cancellation token is no-op as it is not shared outside.
        cancellation_token: CancellationToken::new(),
        orchestrator_tx_pool,
        orchestrator_client,
        solochain,
    };

    let (fut, _exit_notification_receiver) =
        lookahead_tanssi_aura::run::<_, Block, NimbusPair, _, _, _, _, _, _, _, _, _, _, _, _, _>(
            params,
        );
    spawner.spawn("tanssi-aura-container", None, fut);
}

// Log string that will be shown for the container chain: `[Container-2000]`.
// This needs to be a separate function because the `prefix_logs_with` macro
// has trouble parsing expressions.
fn container_log_str(para_id: ParaId) -> String {
    format!("Container-{}", para_id)
}
