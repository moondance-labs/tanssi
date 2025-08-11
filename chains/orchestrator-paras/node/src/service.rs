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

use {
    cumulus_client_parachain_inherent::{MockValidationDataInherentDataProvider, MockXcmConfig},
    cumulus_client_service::prepare_node_config,
    cumulus_primitives_core::{
        relay_chain::well_known_keys as RelayWellKnownKeys, CollectCollationInfo, ParaId,
    },
    dancebox_runtime::opaque::Block,
    node_common::service::node_builder::{ManualSealConfiguration, NodeBuilderConfig, Sealing},
    pallet_registrar_runtime_api::RegistrarApi,
    parity_scale_codec::{Decode, Encode},
    polkadot_cli::ProvideRuntimeApi,
    polkadot_parachain_primitives::primitives::HeadData,
    polkadot_primitives::UpgradeGoAhead,
    sc_client_api::{AuxStore, HeaderBackend},
    sc_service::{Configuration, TaskManager},
    sc_telemetry::TelemetryHandle,
    sp_consensus_slots::Slot,
    sp_core::H256,
    std::sync::Arc,
    tc_service_container_chain_spawner::service::{DevParachainBlockImport, ParachainClient},
    tc_service_orchestrator_chain::parachain::NodeConfig,
};

mod mocked_relay_keys;

// We use this to detect whether randomness is activated
const RANDOMNESS_ACTIVATED_AUX_KEY: &[u8] = b"__DEV_RANDOMNESS_ACTIVATED";

const CONTAINER_CHAINS_EXCLUSION_AUX_KEY: &[u8] = b"__DEV_CONTAINER_CHAINS_EXCLUSION";

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
    let mut container_chains_exclusion_sender = None;
    if parachain_config.role.is_authority() {
        let client = node_builder.client.clone();
        let (downward_xcm_sender, downward_xcm_receiver) = flume::bounded::<Vec<u8>>(100);
        let (hrmp_xcm_sender, hrmp_xcm_receiver) = flume::bounded::<(ParaId, Vec<u8>)>(100);
        // Create channels for mocked parachain candidates.
        let (mock_randomness_sender, mock_randomness_receiver) =
            flume::bounded::<(bool, Option<[u8; 32]>)>(100);
        // Create channels for mocked exclusion of parachains from producing blocks
        let (mock_container_chains_exclusion_sender, mock_container_chains_exclusion_receiver) =
            flume::bounded::<Vec<ParaId>>(100);

        xcm_senders = Some((downward_xcm_sender, hrmp_xcm_sender));
        randomness_sender = Some(mock_randomness_sender);
        container_chains_exclusion_sender = Some(mock_container_chains_exclusion_sender);

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

                let mut para_ids: Vec<ParaId> = client
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

                let container_chains_exclusion_messages: Vec<Vec<ParaId>> = mock_container_chains_exclusion_receiver.drain().collect();
                // If there is a new set of excluded container chains, we update it
                if let Some(mock_excluded_container_chains) = container_chains_exclusion_messages.last() {
                    client
                        .insert_aux(
                            &[(CONTAINER_CHAINS_EXCLUSION_AUX_KEY, mock_excluded_container_chains.encode().as_slice())],
                            &[],
                        )
                        .expect("Should be able to write to aux storage; qed");
                }
                let new_excluded_container_chains_value = client
                    .get_aux(CONTAINER_CHAINS_EXCLUSION_AUX_KEY)
                    .expect("Should be able to query aux storage; qed").unwrap_or(Vec::<ParaId>::new().encode());
                let mock_excluded_container_chains: Vec<ParaId> = Decode::decode(&mut new_excluded_container_chains_value.as_slice()).expect("Vector non-decodable");
                para_ids.retain(|x| !mock_excluded_container_chains.contains(x));
                let client_set_aside_for_cidp = client.clone();
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

                    let current_para_head = client_set_aside_for_cidp
                            .header(block)
                            .expect("Header lookup should succeed")
                            .expect("Header passed in as parent should be present in backend.");
                    let should_send_go_ahead = match client_set_aside_for_cidp
                            .runtime_api()
                            .collect_collation_info(block, &current_para_head)
                    {
                            Ok(info) => info.new_validation_code.is_some(),
                            Err(e) => {
                                    log::error!("Failed to collect collation info: {:?}", e);
                                    false
                            },
                    };

                    let time = MockTimestampInherentDataProvider;
                    let mocked_parachain = MockValidationDataInherentDataProvider {
                        current_para_block,
                        current_para_block_head: None,
                        relay_offset: 1000,
                        relay_blocks_per_para_block: 2,
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
                        upgrade_go_ahead: should_send_go_ahead.then(|| {
                            log::info!(
                                "Detected pending validation code, sending go-ahead signal."
                            );
                            UpgradeGoAhead::GoAhead
                        }),
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
            let deps = tc_service_orchestrator_chain::parachain::rpc::FullDeps {
                client: client.clone(),
                pool: transaction_pool.clone(),
                command_sink: command_sink.clone(),
                xcm_senders: xcm_senders.clone(),
                randomness_sender: randomness_sender.clone(),
                container_chain_exclusion_sender: container_chains_exclusion_sender.clone(),
            };

            tc_service_orchestrator_chain::parachain::rpc::create_full(deps).map_err(Into::into)
        })
    };

    // We spawn all the common substrate tasks to properly run a node.
    let node_builder = node_builder.spawn_common_tasks(parachain_config, rpc_builder)?;

    log::info!("Development Service Ready");

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
