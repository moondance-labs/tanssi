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

#[allow(deprecated)]
use {
    container_chain_template_simple_runtime::{opaque::Block, RuntimeApi},
    cumulus_client_cli::CollatorOptions,
    cumulus_client_consensus_common::ParachainBlockImport as TParachainBlockImport,
    cumulus_client_parachain_inherent::{MockValidationDataInherentDataProvider, MockXcmConfig},
    cumulus_client_service::prepare_node_config,
    cumulus_primitives_core::{relay_chain::well_known_keys as RelayWellKnownKeys, ParaId},
    nimbus_primitives::NimbusId,
    node_common::service::ManualSealConfiguration,
    node_common::service::Sealing,
    node_common::service::{NodeBuilder, NodeBuilderConfig},
    parity_scale_codec::Encode,
    polkadot_parachain_primitives::primitives::HeadData,
    sc_consensus::BasicQueue,
    sc_executor::WasmExecutor,
    sc_service::{Configuration, TFullBackend, TFullClient, TaskManager},
    sp_blockchain::HeaderBackend,
    sp_consensus_slots::{Slot, SlotDuration},
    sp_core::Pair,
    sp_core::H256,
    std::{sync::Arc, time::Duration},
};

type ParachainExecutor = WasmExecutor<sp_io::SubstrateHostFunctions>;
type ParachainClient = TFullClient<Block, RuntimeApi, ParachainExecutor>;
type ParachainBackend = TFullBackend<Block>;
type ParachainBlockImport = TParachainBlockImport<Block, Arc<ParachainClient>, ParachainBackend>;

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
            *x.borrow_mut() += container_chain_template_simple_runtime::SLOT_DURATION;
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
#[sc_tracing::logging::prefix_logs_with("Parachain")]
pub async fn start_parachain_node(
    parachain_config: Configuration,
    polkadot_config: Configuration,
    collator_options: CollatorOptions,
    para_id: ParaId,
    hwbench: Option<sc_sysinfo::HwBench>,
) -> sc_service::error::Result<(TaskManager, Arc<ParachainClient>)> {
    let parachain_config = prepare_node_config(parachain_config);

    // Create a `NodeBuilder` which helps setup parachain nodes common systems.
    let mut node_builder = NodeConfig::new_builder(&parachain_config, hwbench.clone())?;

    let (_, import_queue) = import_queue(&parachain_config, &node_builder);

    // Relay chain interface
    let (relay_chain_interface, _collator_key) = node_builder
        .build_relay_chain_interface(&parachain_config, polkadot_config, collator_options.clone())
        .await?;

    // Build cumulus network, allowing to access network-related services.
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
    let node_builder = node_builder.start_full_node(
        para_id,
        relay_chain_interface.clone(),
        relay_chain_slot_duration,
    )?;

    node_builder.network.start_network.start_network();

    Ok((node_builder.task_manager, node_builder.client))
}

/// Helper function to generate a crypto pair from seed
fn get_aura_id_from_seed(seed: &str) -> NimbusId {
    sp_core::sr25519::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
        .into()
}

/// Start a node with the given parachain `Configuration` and relay chain `Configuration`.
///
/// This is the actual implementation that is abstract over the executor and the runtime api.
#[sc_tracing::logging::prefix_logs_with("Parachain Dev Node")]
pub async fn start_dev_node(
    parachain_config: Configuration,
    sealing: Sealing,
    para_id: ParaId,
    hwbench: Option<sc_sysinfo::HwBench>,
) -> sc_service::error::Result<TaskManager> {
    let parachain_config = prepare_node_config(parachain_config);

    // Create a `NodeBuilder` which helps setup parachain nodes common systems.
    let node_builder = NodeConfig::new_builder(&parachain_config, hwbench.clone())?;

    let (parachain_block_import, import_queue) = import_queue(&parachain_config, &node_builder);

    // Build a Substrate Network. (not cumulus since it is a dev node, it mocks
    // the relaychain)
    let mut node_builder = node_builder.build_substrate_network(&parachain_config, import_queue)?;

    let mut command_sink = None;
    let mut xcm_senders = None;

    if parachain_config.role.is_authority() {
        let client = node_builder.client.clone();
        let (downward_xcm_sender, downward_xcm_receiver) = flume::bounded::<Vec<u8>>(100);
        let (hrmp_xcm_sender, hrmp_xcm_receiver) = flume::bounded::<(ParaId, Vec<u8>)>(100);
        xcm_senders = Some((downward_xcm_sender, hrmp_xcm_sender));

        let authorities = vec![get_aura_id_from_seed("alice")];

        command_sink = node_builder.install_manual_seal(ManualSealConfiguration {
            block_import: parachain_block_import,
            sealing,
            soft_deadline: None,
            select_chain: sc_consensus::LongestChain::new(node_builder.backend.clone()),
            consensus_data_provider: Some(Box::new(
                tc_consensus::ContainerManualSealAuraConsensusDataProvider::new(
                    SlotDuration::from_millis(
                        container_chain_template_simple_runtime::SLOT_DURATION,
                    ),
                    authorities.clone(),
                ),
            )),
            create_inherent_data_providers: move |block: H256, ()| {
                let current_para_block = client
                    .number(block)
                    .expect("Header lookup should succeed")
                    .expect("Header passed in as parent should be present in backend.");

                let hash = client
                    .hash(current_para_block.saturating_sub(1))
                    .expect("Hash of the desired block must be present")
                    .expect("Hash of the desired block should exist");

                let para_header = client
                    .expect_header(hash)
                    .expect("Expected parachain header should exist")
                    .encode();

                let para_head_data: Vec<u8> = HeadData(para_header).encode();
                let client_for_xcm = client.clone();
                let authorities_for_cidp = authorities.clone();
                let para_head_key = RelayWellKnownKeys::para_head(para_id);
                let relay_slot_key = RelayWellKnownKeys::CURRENT_SLOT.to_vec();
                let slot_duration = container_chain_template_simple_runtime::SLOT_DURATION;

                let mut timestamp = 0u64;
                TIMESTAMP.with(|x| {
                    timestamp = x.clone().take();
                });

                timestamp += slot_duration;

                let relay_slot = sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
						timestamp.into(),
						SlotDuration::from_millis(slot_duration),
                    );
                let relay_slot = u64::from(*relay_slot);

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

                    let mut additional_keys = mocked_authorities_noting.get_key_values();
                    additional_keys.append(&mut vec![(para_head_key, para_head_data), (relay_slot_key, Slot::from(relay_slot).encode())]);

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
                        additional_key_values: Some(additional_keys),
                    };

                    Ok((time, mocked_parachain, mocked_authorities_noting))
                }
            },
        })?;
    }

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

    let node_builder = node_builder.spawn_common_tasks(parachain_config, rpc_builder)?;

    log::info!("Development Service Ready");

    node_builder.network.start_network.start_network();

    Ok(node_builder.task_manager)
}
