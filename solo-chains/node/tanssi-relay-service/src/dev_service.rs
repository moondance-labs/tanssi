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

//! Development Polkadot service. Adapted from `polkadot_service` crate
//! and removed un-necessary components which are not required in dev node.
//!
//! Following major changes are made:
//! 1. Removed beefy and grandpa notification service and request response protocols
//! 2. Removed support for parachains which also eliminated the need to start overseer and all other subsystems associated with collation + network request/response protocols for the same
//! 3. Removed support for hardware benchmarking
//! 4. Removed authority discovery service
//! 5. Removed spawning of beefy, grandpa and MMR worker
//! 6. Removed rpc extensions for beefy, grandpa and babe and added support for manual seal
//! 7. Removed beefy and grandpa block import from block import pipeline (Babe remains)
//! 8. Using manual seal import queue instead of babe import queue
//! 9. Started manual seal worker
//! 10. If amount of time passed between two block is less than slot duration, we emulate passing of time babe block import and runtime
//!     by incrementing timestamp by slot duration.

use {
    async_io::Timer,
    babe::{BabeBlockImport, BabeLink},
    codec::{Decode, Encode},
    consensus_common::SelectChain,
    dancelight_runtime::RuntimeApi,
    futures::{Stream, StreamExt},
    jsonrpsee::RpcModule,
    node_common::service::Sealing,
    polkadot_core_primitives::{AccountId, Balance, Block, Hash, Nonce},
    polkadot_node_core_parachains_inherent::Error as InherentError,
    polkadot_overseer::Handle,
    polkadot_primitives::InherentData as ParachainsInherentData,
    polkadot_rpc::RpcExtension,
    polkadot_service::{
        BlockT, Error, IdentifyVariant, NewFullParams, OverseerGen, SelectRelayChain,
    },
    sc_client_api::{AuxStore, Backend},
    sc_consensus_manual_seal::{
        consensus::babe::BabeConsensusDataProvider,
        rpc::{ManualSeal, ManualSealApiServer},
        run_manual_seal, EngineCommand, ManualSealParams,
    },
    sc_executor::{HeapAllocStrategy, WasmExecutor, DEFAULT_HEAP_ALLOC_STRATEGY},
    sc_transaction_pool_api::{OffchainTransactionPoolFactory, TransactionPool},
    service::{Configuration, KeystoreContainer, RpcHandlers, TaskManager},
    sp_api::ProvideRuntimeApi,
    sp_block_builder::BlockBuilder,
    sp_blockchain::{HeaderBackend, HeaderMetadata},
    sp_consensus_babe::SlotDuration,
    std::{cmp::max, ops::Add, sync::Arc, time::Duration},
    telemetry::{Telemetry, TelemetryWorker, TelemetryWorkerHandle},
};

pub type FullBackend = service::TFullBackend<Block>;

pub type FullClient = service::TFullClient<
    Block,
    RuntimeApi,
    WasmExecutor<(
        sp_io::SubstrateHostFunctions,
        frame_benchmarking::benchmarking::HostFunctions,
    )>,
>;

pub struct NewFull {
    pub task_manager: TaskManager,
    pub client: Arc<FullClient>,
    pub overseer_handle: Option<Handle>,
    pub network: Arc<dyn sc_network::service::traits::NetworkService>,
    pub sync_service: Arc<sc_network_sync::SyncingService<Block>>,
    pub rpc_handlers: RpcHandlers,
    pub backend: Arc<FullBackend>,
}

/// Custom Deps for dev Rpc extension
struct DevDeps<C, P> {
    /// The client instance to use.
    pub client: Arc<C>,
    /// Transaction pool instance.
    pub pool: Arc<P>,
    /// Manual seal command sink
    pub command_sink: Option<futures::channel::mpsc::Sender<EngineCommand<Hash>>>,
}

fn create_dev_rpc_extension<C, P>(
    DevDeps {
        client,
        pool,
        command_sink: maybe_command_sink,
    }: DevDeps<C, P>,
) -> Result<RpcExtension, Box<dyn std::error::Error + Send + Sync>>
where
    C: ProvideRuntimeApi<Block>
        + HeaderBackend<Block>
        + AuxStore
        + HeaderMetadata<Block, Error = sp_blockchain::Error>
        + Send
        + Sync
        + 'static,
    C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>,
    C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
    C::Api: BlockBuilder<Block>,
    P: TransactionPool + Sync + Send + 'static,
{
    use {
        pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer},
        substrate_frame_rpc_system::{System, SystemApiServer},
    };

    let mut io = RpcModule::new(());
    io.merge(System::new(client.clone(), pool.clone()).into_rpc())?;
    io.merge(TransactionPayment::new(client.clone()).into_rpc())?;

    if let Some(command_sink) = maybe_command_sink {
        io.merge(ManualSeal::new(command_sink).into_rpc())?;
    }

    Ok(io)
}

/// We use EmptyParachainsInherentDataProvider to insert an empty parachain inherent in the block
/// to satisfy runtime
struct EmptyParachainsInherentDataProvider<C: HeaderBackend<Block>> {
    pub client: Arc<C>,
    pub parent: Hash,
}

/// Copied from polkadot service just so that this code retains same structure as
/// polkadot_service crate.
struct Basics {
    task_manager: TaskManager,
    client: Arc<FullClient>,
    backend: Arc<FullBackend>,
    keystore_container: KeystoreContainer,
    telemetry: Option<Telemetry>,
}

impl<C: HeaderBackend<Block>> EmptyParachainsInherentDataProvider<C> {
    pub fn new(client: Arc<C>, parent: Hash) -> Self {
        EmptyParachainsInherentDataProvider { client, parent }
    }

    pub async fn create(
        client: Arc<C>,
        parent: Hash,
    ) -> Result<ParachainsInherentData, InherentError> {
        let parent_header = match client.header(parent) {
            Ok(Some(h)) => h,
            Ok(None) => return Err(InherentError::ParentHeaderNotFound(parent)),
            Err(err) => return Err(InherentError::Blockchain(err)),
        };

        Ok(ParachainsInherentData {
            bitfields: Vec::new(),
            backed_candidates: Vec::new(),
            disputes: Vec::new(),
            parent_header,
        })
    }
}

#[async_trait::async_trait]
impl<C: HeaderBackend<Block>> sp_inherents::InherentDataProvider
    for EmptyParachainsInherentDataProvider<C>
{
    async fn provide_inherent_data(
        &self,
        dst_inherent_data: &mut sp_inherents::InherentData,
    ) -> Result<(), sp_inherents::Error> {
        let inherent_data =
            EmptyParachainsInherentDataProvider::create(self.client.clone(), self.parent)
                .await
                .map_err(|e| sp_inherents::Error::Application(Box::new(e)))?;

        dst_inherent_data.put_data(
            polkadot_primitives::PARACHAINS_INHERENT_IDENTIFIER,
            &inherent_data,
        )
    }

    async fn try_handle_error(
        &self,
        _identifier: &sp_inherents::InherentIdentifier,
        _error: &[u8],
    ) -> Option<Result<(), sp_inherents::Error>> {
        // Inherent isn't checked and can not return any error
        None
    }
}

/// Creates new development full node with manual seal
pub fn build_full<OverseerGenerator: OverseerGen>(
    sealing: Sealing,
    config: Configuration,
    mut params: NewFullParams<OverseerGenerator>,
) -> Result<NewFull, Error> {
    let is_polkadot = config.chain_spec.is_polkadot();

    params.overseer_message_channel_capacity_override = params
        .overseer_message_channel_capacity_override
        .map(move |capacity| {
            if is_polkadot {
                gum::warn!("Channel capacity should _never_ be tampered with on polkadot!");
            }
            capacity
        });

    match config.network.network_backend {
        sc_network::config::NetworkBackendType::Libp2p => {
            new_full::<_, sc_network::NetworkWorker<Block, Hash>>(sealing, config, params)
        }
        sc_network::config::NetworkBackendType::Litep2p => {
            new_full::<_, sc_network::Litep2pNetworkBackend>(sealing, config, params)
        }
    }
}

/// We store past timestamp we created in the aux storage, which enable us to return timestamp which is increased by
/// slot duration from previous timestamp or current timestamp if in reality more time is passed.
fn get_next_timestamp(
    client: Arc<FullClient>,
    slot_duration: SlotDuration,
) -> sp_timestamp::InherentDataProvider {
    const TIMESTAMP_AUX_KEY: &[u8] = b"__DEV_TIMESTAMP";

    let maybe_last_timestamp = client
        .get_aux(TIMESTAMP_AUX_KEY)
        .expect("Should be able to query aux storage; qed");
    if let Some(last_timestamp) = maybe_last_timestamp {
        let last_inherent_data = sp_timestamp::InherentType::decode(&mut last_timestamp.as_slice())
            .expect("Timestamp data must be decoded; qed");
        let new_inherent_data: sp_timestamp::InherentType = max(
            last_inherent_data.add(slot_duration.as_millis()),
            sp_timestamp::InherentType::current(),
        );
        client
            .insert_aux(
                &[(TIMESTAMP_AUX_KEY, new_inherent_data.encode().as_slice())],
                &[],
            )
            .expect("Should be able to write to aux storage; qed");
        sp_timestamp::InherentDataProvider::new(new_inherent_data)
    } else {
        let current_timestamp = sp_timestamp::InherentType::current();
        client
            .insert_aux(
                &[(TIMESTAMP_AUX_KEY, current_timestamp.encode().as_slice())],
                &[],
            )
            .expect("Should be able to write to aux storage; qed");
        sp_timestamp::InherentDataProvider::new(current_timestamp)
    }
}

fn new_full<
    OverseerGenerator: OverseerGen,
    Network: sc_network::NetworkBackend<Block, <Block as BlockT>::Hash>,
>(
    sealing: Sealing,
    mut config: Configuration,
    NewFullParams {
        telemetry_worker_handle,
        ..
    }: NewFullParams<OverseerGenerator>,
) -> Result<NewFull, Error> {
    let role = config.role;

    let basics = new_partial_basics(&mut config, telemetry_worker_handle)?;

    let prometheus_registry = config.prometheus_registry().cloned();

    let keystore = basics.keystore_container.local_keystore();

    let select_chain = SelectRelayChain::new_longest_chain(basics.backend.clone());

    let service::PartialComponents::<_, _, SelectRelayChain<_>, _, _, _> {
        client,
        backend,
        mut task_manager,
        keystore_container,
        select_chain,
        import_queue,
        transaction_pool,
        other: (block_import, babe_link, slot_duration, mut telemetry),
    } = new_partial::<SelectRelayChain<_>>(&mut config, basics, select_chain)?;

    let metrics = Network::register_notification_metrics(
        config.prometheus_config.as_ref().map(|cfg| &cfg.registry),
    );

    let net_config = sc_network::config::FullNetworkConfiguration::<_, _, Network>::new(
        &config.network,
        prometheus_registry.clone(),
    );

    let (network, system_rpc_tx, tx_handler_controller, network_starter, sync_service) =
        service::build_network(service::BuildNetworkParams {
            config: &config,
            net_config,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            spawn_handle: task_manager.spawn_handle(),
            import_queue,
            block_announce_validator_builder: None,
            warp_sync_config: None,
            block_relay: None,
            metrics,
        })?;

    if config.offchain_worker.enabled {
        use futures::FutureExt;

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
                network_provider: Arc::new(network.clone()),
                is_validator: role.is_authority(),
                enable_http_requests: false,
                custom_extensions: move |_| vec![],
            })
            .run(client.clone(), task_manager.spawn_handle())
            .boxed(),
        );
    }

    let mut command_sink = None;

    if role.is_authority() {
        let proposer = sc_basic_authorship::ProposerFactory::with_proof_recording(
            task_manager.spawn_handle(),
            client.clone(),
            transaction_pool.clone(),
            prometheus_registry.as_ref(),
            telemetry.as_ref().map(|x| x.handle()),
        );

        let commands_stream: Box<
            dyn Stream<Item = EngineCommand<<Block as BlockT>::Hash>> + Send + Sync + Unpin,
        > = match sealing {
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
                    finalize: true,
                    parent_hash: None,
                    sender: None,
                },
            )),
        };

        let babe_config = babe_link.config();
        let babe_consensus_provider = BabeConsensusDataProvider::new(
            client.clone(),
            keystore,
            babe_link.epoch_changes().clone(),
            babe_config.authorities.clone(),
        )
        .map_err(|babe_error| {
            Error::Consensus(consensus_common::Error::Other(babe_error.into()))
        })?;

        // Need to clone it and store here to avoid moving of `client`
        // variable in closure below.
        let client_clone = client.clone();
        task_manager.spawn_essential_handle().spawn_blocking(
            "authorship_task",
            Some("block-authoring"),
            run_manual_seal(ManualSealParams {
                block_import,
                env: proposer,
                client: client.clone(),
                pool: transaction_pool.clone(),
                commands_stream,
                select_chain,
                create_inherent_data_providers: move |parent, ()| {
                    let client_clone = client_clone.clone();

                    async move {
                        let parachain =
                            EmptyParachainsInherentDataProvider::new(
                                client_clone.clone(),
                                parent,
                            );

                        let timestamp = get_next_timestamp(client_clone, slot_duration);

                        let slot =
                            sp_consensus_babe::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
                                *timestamp,
                                slot_duration,
                            );

                        Ok((slot, timestamp, parachain))
                    }
                },
                consensus_data_provider: Some(Box::new(babe_consensus_provider)),
            }),
        );
    }

    let rpc_extensions_builder = {
        let client = client.clone();
        let transaction_pool = transaction_pool.clone();

        move |_subscription_executor: polkadot_rpc::SubscriptionTaskExecutor|
            -> Result<RpcExtension, service::Error> {
            let deps = DevDeps {
                client: client.clone(),
                pool: transaction_pool.clone(),
                command_sink: command_sink.clone(),
            };

            create_dev_rpc_extension(deps).map_err(Into::into)
        }
    };

    let rpc_handlers = service::spawn_tasks(service::SpawnTasksParams {
        config,
        backend: backend.clone(),
        client: client.clone(),
        keystore: keystore_container.keystore(),
        network: network.clone(),
        sync_service: sync_service.clone(),
        rpc_builder: Box::new(rpc_extensions_builder),
        transaction_pool: transaction_pool.clone(),
        task_manager: &mut task_manager,
        system_rpc_tx,
        tx_handler_controller,
        telemetry: telemetry.as_mut(),
    })?;

    network_starter.start_network();

    Ok(NewFull {
        task_manager,
        client,
        overseer_handle: None,
        network,
        sync_service,
        rpc_handlers,
        backend,
    })
}

fn new_partial<ChainSelection>(
    config: &mut Configuration,
    Basics {
        task_manager,
        backend,
        client,
        keystore_container,
        telemetry,
    }: Basics,
    select_chain: ChainSelection,
) -> Result<
    service::PartialComponents<
        FullClient,
        FullBackend,
        ChainSelection,
        sc_consensus::DefaultImportQueue<Block>,
        sc_transaction_pool::FullPool<Block, FullClient>,
        (
            BabeBlockImport<Block, FullClient, Arc<FullClient>>,
            BabeLink<Block>,
            SlotDuration,
            Option<Telemetry>,
        ),
    >,
    Error,
>
where
    ChainSelection: 'static + SelectChain<Block>,
{
    let transaction_pool = sc_transaction_pool::BasicPool::new_full(
        config.transaction_pool.clone(),
        config.role.is_authority().into(),
        config.prometheus_registry(),
        task_manager.spawn_essential_handle(),
        client.clone(),
    );

    // Create babe block import queue; this is required to have correct epoch data
    // available for manual seal to produce block
    let babe_config = babe::configuration(&*client)?;
    let (babe_block_import, babe_link) =
        babe::block_import(babe_config.clone(), client.clone(), client.clone())?;
    let slot_duration = babe_link.config().slot_duration();

    // Create manual seal block import with manual seal block import queue
    let import_queue = sc_consensus_manual_seal::import_queue(
        Box::new(babe_block_import.clone()),
        &task_manager.spawn_essential_handle(),
        config.prometheus_registry(),
    );

    Ok(service::PartialComponents {
        client,
        backend,
        task_manager,
        keystore_container,
        select_chain,
        import_queue,
        transaction_pool,
        other: (babe_block_import, babe_link, slot_duration, telemetry),
    })
}

fn new_partial_basics(
    config: &mut Configuration,
    telemetry_worker_handle: Option<TelemetryWorkerHandle>,
) -> Result<Basics, Error> {
    let telemetry = config
        .telemetry_endpoints
        .clone()
        .filter(|x| !x.is_empty())
        .map(move |endpoints| -> Result<_, telemetry::Error> {
            let (worker, mut worker_handle) = if let Some(worker_handle) = telemetry_worker_handle {
                (None, worker_handle)
            } else {
                let worker = TelemetryWorker::new(16)?;
                let worker_handle = worker.handle();
                (Some(worker), worker_handle)
            };
            let telemetry = worker_handle.new_telemetry(endpoints);
            Ok((worker, telemetry))
        })
        .transpose()?;

    let heap_pages = config
        .executor
        .default_heap_pages
        .map_or(DEFAULT_HEAP_ALLOC_STRATEGY, |h| HeapAllocStrategy::Static {
            extra_pages: h as u32,
        });

    let mut wasm_builder = WasmExecutor::builder()
        .with_execution_method(config.executor.wasm_method)
        .with_onchain_heap_alloc_strategy(heap_pages)
        .with_offchain_heap_alloc_strategy(heap_pages)
        .with_max_runtime_instances(config.executor.max_runtime_instances)
        .with_runtime_cache_size(config.executor.runtime_cache_size);
    if let Some(ref wasmtime_precompiled_path) = config.executor.wasmtime_precompiled {
        wasm_builder = wasm_builder.with_wasmtime_precompiled_path(wasmtime_precompiled_path);
    }
    let executor = wasm_builder.build();

    let (client, backend, keystore_container, task_manager) =
        service::new_full_parts::<Block, RuntimeApi, _>(
            config,
            telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
            executor,
        )?;
    let client = Arc::new(client);

    let telemetry = telemetry.map(|(worker, telemetry)| {
        if let Some(worker) = worker {
            task_manager.spawn_handle().spawn(
                "telemetry",
                Some("telemetry"),
                Box::pin(worker.run()),
            );
        }
        telemetry
    });

    Ok(Basics {
        task_manager,
        client,
        backend,
        keystore_container,
        telemetry,
    })
}
