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
    crate::dev_rpcs::{DevApiServer, DevRpc},
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
    polkadot_parachain_primitives::primitives::UpwardMessages,
    polkadot_primitives::{
        runtime_api::ParachainHost, BackedCandidate, CandidateCommitments, CandidateDescriptor,
        CollatorPair, CommittedCandidateReceipt, CompactStatement, EncodeAs,
        InherentData as ParachainsInherentData, OccupiedCoreAssumption, SigningContext,
        ValidityAttestation,
    },
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
    sc_keystore::Keystore,
    sc_transaction_pool_api::{OffchainTransactionPoolFactory, TransactionPool},
    service::{Configuration, KeystoreContainer, RpcHandlers, TaskManager},
    sp_api::ProvideRuntimeApi,
    sp_block_builder::BlockBuilder,
    sp_blockchain::{HeaderBackend, HeaderMetadata},
    sp_consensus_aura::{inherents::InherentType as AuraInherentType, AURA_ENGINE_ID},
    sp_consensus_babe::SlotDuration,
    sp_core::{ByteArray, Pair, H256},
    sp_keystore::KeystorePtr,
    sp_runtime::{traits::BlakeTwo256, DigestItem, RuntimeAppPublic},
    std::{cmp::max, ops::Add, sync::Arc, time::Duration},
    telemetry::{Telemetry, TelemetryWorker, TelemetryWorkerHandle},
};

// We use this key to store whether we want the para inherent mocker to be active
const PARA_INHERENT_SELECTOR_AUX_KEY: &[u8] = b"__DEV_PARA_INHERENT_SELECTOR";

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
    /// Dev rpcs
    pub dev_rpc: Option<DevRpc>,
}

fn create_dev_rpc_extension<C, P>(
    DevDeps {
        client,
        pool,
        command_sink: maybe_command_sink,
        dev_rpc: maybe_dev_rpc,
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

    if let Some(dev_rpc_data) = maybe_dev_rpc {
        io.merge(dev_rpc_data.into_rpc())?;
    }

    Ok(io)
}

/// We use EmptyParachainsInherentDataProvider to insert an empty parachain inherent in the block
/// to satisfy runtime
struct EmptyParachainsInherentDataProvider;

/// Copied from polkadot service just so that this code retains same structure as
/// polkadot_service crate.
struct Basics {
    task_manager: TaskManager,
    client: Arc<FullClient>,
    backend: Arc<FullBackend>,
    keystore_container: KeystoreContainer,
    telemetry: Option<Telemetry>,
}

impl EmptyParachainsInherentDataProvider {
    pub async fn create<C: HeaderBackend<Block>>(
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

/// We use MockParachainsInherentDataProvider to insert an parachain inherent with mocked
/// candidates
/// We detect whether any of the keys in our keystore is assigned to a core and provide
/// a mocked candidate in such core
struct MockParachainsInherentDataProvider<C: HeaderBackend<Block> + ProvideRuntimeApi<Block>> {
    pub client: Arc<C>,
    pub parent: Hash,
    pub keystore: KeystorePtr,
    pub upward_messages_receiver: flume::Receiver<Vec<u8>>,
}

impl<C: HeaderBackend<Block> + ProvideRuntimeApi<Block>> MockParachainsInherentDataProvider<C>
where
    C::Api: ParachainHost<Block>,
    C: AuxStore,
{
    pub fn new(
        client: Arc<C>,
        parent: Hash,
        keystore: KeystorePtr,
        upward_messages_receiver: flume::Receiver<Vec<u8>>,
    ) -> Self {
        MockParachainsInherentDataProvider {
            client,
            parent,
            keystore,
            upward_messages_receiver,
        }
    }

    pub async fn create(
        client: Arc<C>,
        parent: Hash,
        keystore: KeystorePtr,
        upward_messages_receiver: flume::Receiver<Vec<u8>>,
    ) -> Result<ParachainsInherentData, InherentError> {
        let parent_header = match client.header(parent) {
            Ok(Some(h)) => h,
            Ok(None) => return Err(InherentError::ParentHeaderNotFound(parent)),
            Err(err) => return Err(InherentError::Blockchain(err)),
        };

        // Strategy:
        // we usually have 1 validator per core, and we usually run with --alice
        // the idea is that at least alice will be assigned to one core
        // if we find in the keystore the validator attached to a particular core,
        // we generate a signature for the parachain assigned to that core
        // To retrieve the validator keys, cal runtime api:

        // this following piece of code predicts whether the validator is assigned to a particular
        // core where a candidate for a parachain needs to be created
        let runtime_api = client.runtime_api();

        // we get all validators

        // we get the current claim queue to know core availability
        let claim_queue = runtime_api.claim_queue(parent).unwrap();

        // we get the validator groups
        let (groups, rotation_info) = runtime_api.validator_groups(parent).unwrap();

        // we calculate rotation since start, which will define the core assignation
        // to validators
        let rotations_since_session_start = (parent_header.number
            - rotation_info.session_start_block)
            / rotation_info.group_rotation_frequency;

        // Get all the available keys in the keystore
        let available_keys = keystore
            .keys(polkadot_primitives::PARACHAIN_KEY_TYPE_ID)
            .unwrap();

        // create a slot number identical to the parent block num
        let slot_number = AuraInherentType::from(u64::from(parent_header.number));

        // create a mocked header
        let parachain_mocked_header = sp_runtime::generic::Header::<u32, BlakeTwo256> {
            parent_hash: Default::default(),
            number: parent_header.number,
            state_root: Default::default(),
            extrinsics_root: Default::default(),
            digest: sp_runtime::generic::Digest {
                logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot_number.encode())],
            },
        };

        // retrieve availability cores
        let availability_cores = runtime_api.availability_cores(parent).unwrap();

        // retrieve current session_idx
        let session_idx = runtime_api.session_index_for_child(parent).unwrap();

        // retrieve all validators
        let all_validators = runtime_api.validators(parent).unwrap();

        // construct full availability bitvec
        let availability_bitvec = availability_bitvec(1, availability_cores.len());

        let signature_ctx = SigningContext {
            parent_hash: parent,
            session_index: session_idx,
        };

        // we generate the availability bitfield sigs
        // TODO: here we assume all validator keys are able to sign with our keystore
        // we need to make sure the key is there before we try to sign
        // this is mostly to indicate that the erasure coding chunks where received by all val
        let bitfields: Vec<UncheckedSigned<AvailabilityBitfield>> = all_validators
            .iter()
            .enumerate()
            .map(|(i, public)| {
                keystore_sign(
                    &keystore,
                    availability_bitvec.clone(),
                    &signature_ctx,
                    ValidatorIndex(i as u32),
                    &public,
                )
                .unwrap()
                .unwrap()
            })
            .collect();

        // generate a random collator pair
        let collator_pair = CollatorPair::generate().0;
        let mut backed_cand: Vec<BackedCandidate<H256>> = vec![];

        // iterate over every core|para pair
        for (core, para) in claim_queue {
            // check which group is assigned to each core
            let group_assigned_to_core =
                core.0 + rotations_since_session_start % groups.len() as u32;
            // check validator indices associated to the core
            let indices_associated_to_core = groups.get(group_assigned_to_core as usize).unwrap();
            for index in indices_associated_to_core {
                // fetch validator keys
                let validator_keys_to_find = all_validators.get(index.0 as usize).unwrap();
                // Iterate keys until we find an eligible one, or run out of candidates.
                for type_public_pair in &available_keys {
                    if let Ok(validator) =
                        polkadot_primitives::ValidatorId::from_slice(&type_public_pair)
                    {
                        // if we find the validator in keystore, we try to create a backed cand
                        if validator_keys_to_find == &validator {
                            // we work with the previous included data
                            let mut persisted_validation_data = runtime_api
                                .persisted_validation_data(
                                    parent,
                                    para[0],
                                    OccupiedCoreAssumption::Included,
                                )
                                .unwrap()
                                .unwrap();

                            // if we dont do this we have a backed candidate every 2 blocks
                            // TODO: figure out why
                            persisted_validation_data.relay_parent_storage_root =
                                parent_header.state_root;

                            let persisted_validation_data_hash = persisted_validation_data.hash();
                            // retrieve the validation code hash
                            let validation_code_hash = runtime_api
                                .validation_code_hash(
                                    parent,
                                    para[0],
                                    OccupiedCoreAssumption::Included,
                                )
                                .unwrap()
                                .unwrap();
                            let pov_hash = Default::default();
                            // generate a fake collator signature
                            let payload = polkadot_primitives::collator_signature_payload(
                                &parent,
                                &para[0],
                                &persisted_validation_data_hash,
                                &pov_hash,
                                &validation_code_hash,
                            );
                            let collator_signature = collator_pair.sign(&payload);

                            let upward_messages = UpwardMessages::try_from(
                                upward_messages_receiver.drain().collect::<Vec<_>>(),
                            )
                            .expect("create upward messages from raw messages");

                            // generate a candidate with most of the values mocked
                            let candidate = CommittedCandidateReceipt::<H256> {
                                descriptor: CandidateDescriptor::<H256> {
                                    para_id: para[0],
                                    relay_parent: parent,
                                    collator: collator_pair.public(),
                                    persisted_validation_data_hash,
                                    pov_hash,
                                    erasure_root: Default::default(),
                                    signature: collator_signature,
                                    para_head: parachain_mocked_header.clone().hash(),
                                    validation_code_hash,
                                },
                                commitments: CandidateCommitments::<u32> {
                                    upward_messages,
                                    horizontal_messages: Default::default(),
                                    new_validation_code: None,
                                    head_data: parachain_mocked_header.clone().encode().into(),
                                    processed_downward_messages: 0,
                                    hrmp_watermark: parent_header.number,
                                },
                            };
                            let candidate_hash = candidate.hash();
                            let payload = CompactStatement::Valid(candidate_hash);

                            let signature_ctx = SigningContext {
                                parent_hash: parent,
                                session_index: session_idx,
                            };

                            // sign the candidate with the validator key
                            let signature = keystore_sign(
                                &keystore,
                                payload,
                                &signature_ctx,
                                *index,
                                &validator,
                            )
                            .unwrap()
                            .unwrap()
                            .benchmark_signature();

                            // construct a validity vote
                            let validity_votes = vec![ValidityAttestation::Explicit(signature)];

                            // push the candidate
                            backed_cand.push(BackedCandidate::<H256>::new(
                                candidate,
                                validity_votes.clone(),
                                bitvec::bitvec![u8, bitvec::order::Lsb0; 1; indices_associated_to_core.len()],
                                Some(core),
                            ));
                        }
                    }
                }
            }
        }

        Ok(ParachainsInherentData {
            bitfields: bitfields,
            backed_candidates: backed_cand,
            disputes: Vec::new(),
            parent_header,
        })
    }
}

#[async_trait::async_trait]
impl<C: HeaderBackend<Block> + ProvideRuntimeApi<Block>> sp_inherents::InherentDataProvider
    for MockParachainsInherentDataProvider<C>
where
    C::Api: ParachainHost<Block>,
    C: AuxStore,
{
    async fn provide_inherent_data(
        &self,
        dst_inherent_data: &mut sp_inherents::InherentData,
    ) -> Result<(), sp_inherents::Error> {
        // fetch whether the para inherent selector has been set
        let maybe_para_selector = self
            .client
            .get_aux(PARA_INHERENT_SELECTOR_AUX_KEY)
            .expect("Should be able to query aux storage; qed");

        let inherent_data = {
            if let Some(aux) = maybe_para_selector {
                // if it is true, the candidates need to be mocked
                // else, we output the empty parachain inherent data provider
                if aux == true.encode() {
                    MockParachainsInherentDataProvider::create(
                        self.client.clone(),
                        self.parent,
                        self.keystore.clone(),
                        self.upward_messages_receiver.clone(),
                    )
                    .await
                    .map_err(|e| sp_inherents::Error::Application(Box::new(e)))?
                } else {
                    EmptyParachainsInherentDataProvider::create(self.client.clone(), self.parent)
                        .await
                        .map_err(|e| sp_inherents::Error::Application(Box::new(e)))?
                }
            } else {
                EmptyParachainsInherentDataProvider::create(self.client.clone(), self.parent)
                    .await
                    .map_err(|e| sp_inherents::Error::Application(Box::new(e)))?
            }
        };

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

    // Create channels for mocked parachain candidates.
    let (downward_mock_para_inherent_sender, downward_mock_para_inherent_receiver) =
        flume::bounded::<Vec<u8>>(100);

    let (upward_mock_sender, upward_mock_receiver) = flume::bounded::<Vec<u8>>(100);

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
            })?
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
                    transaction_pool.import_notification_stream().map(|_| {
                        EngineCommand::SealNewBlock {
                            create_empty: false,
                            finalize: false,
                            parent_hash: None,
                            sender: None,
                        }
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
        let keystore_clone = keystore.clone();

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
                    let keystore = keystore_clone.clone();
                    let downward_mock_para_inherent_receiver = downward_mock_para_inherent_receiver.clone();
                    let upward_mock_receiver = upward_mock_receiver.clone();
                    async move {

                        let downward_mock_para_inherent_receiver = downward_mock_para_inherent_receiver.clone();
                        // here we only take the last one
                        let para_inherent_decider_messages: Vec<Vec<u8>> = downward_mock_para_inherent_receiver.drain().collect();

                        let upward_messages_receiver = upward_mock_receiver.clone();

                        // If there is a value to be updated, we update it
                        if let Some(value) = para_inherent_decider_messages.last() {
                            client_clone
                            .insert_aux(
                                &[(PARA_INHERENT_SELECTOR_AUX_KEY, value.as_slice())],
                                &[],
                            )
                            .expect("Should be able to write to aux storage; qed");
                        }

                        let parachain = MockParachainsInherentDataProvider::new(
                            client_clone.clone(),
                            parent,
                            keystore,
                            upward_messages_receiver,
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

    let dev_rpc = if role.clone().is_authority() {
        Some(DevRpc {
            mock_para_inherent_channel: downward_mock_para_inherent_sender,
            upward_message_channel: upward_mock_sender,
        })
    } else {
        None
    };

    let rpc_extensions_builder = {
        let client = client.clone();
        let transaction_pool = transaction_pool.clone();

        move |_subscription_executor: polkadot_rpc::SubscriptionTaskExecutor|
            -> Result<RpcExtension, service::Error> {
            let deps = DevDeps {
                client: client.clone(),
                pool: transaction_pool.clone(),
                command_sink: command_sink.clone(),
                dev_rpc: dev_rpc.clone(),
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
        sc_transaction_pool::TransactionPoolHandle<Block, FullClient>,
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
    let transaction_pool = sc_transaction_pool::Builder::new(
        task_manager.spawn_essential_handle(),
        client.clone(),
        config.role.is_authority().into(),
    )
    .with_options(config.transaction_pool.clone())
    .with_prometheus(config.prometheus_registry())
    .build();

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
        transaction_pool: transaction_pool.into(),
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

use polkadot_primitives::{AvailabilityBitfield, UncheckedSigned, ValidatorId, ValidatorIndex};
use sp_keystore::Error as KeystoreError;
fn keystore_sign<H: Encode, Payload: Encode>(
    keystore: &KeystorePtr,
    payload: Payload,
    context: &SigningContext<H>,
    validator_index: ValidatorIndex,
    key: &ValidatorId,
) -> Result<Option<UncheckedSigned<Payload>>, KeystoreError> {
    let data = payload_data(&payload, context);
    let signature = keystore
        .sr25519_sign(ValidatorId::ID, key.as_ref(), &data)?
        .map(|sig| UncheckedSigned::new(payload, validator_index, sig.into()));
    Ok(signature)
}

fn payload_data<H: Encode, Payload: Encode>(
    payload: &Payload,
    context: &SigningContext<H>,
) -> Vec<u8> {
    // equivalent to (`real_payload`, context).encode()
    let mut out = payload.encode_as();
    out.extend(context.encode());
    out
}

/// Create an `AvailabilityBitfield` with size `total_cores`. The first `used_cores` set to true (occupied),
/// and the remaining to false (available).
fn availability_bitvec(used_cores: usize, total_cores: usize) -> AvailabilityBitfield {
    let mut bitfields = bitvec::bitvec![u8, bitvec::order::Lsb0; 0; 0];
    for i in 0..total_cores {
        if i < used_cores {
            bitfields.push(true);
        } else {
            bitfields.push(false)
        }
    }

    bitfields.into()
}
