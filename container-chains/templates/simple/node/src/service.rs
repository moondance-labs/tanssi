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
    node_common::service::{NodeBuilderConfig as _, NodeBuilder},
    sc_consensus::BasicQueue,
};

#[allow(deprecated)]
use {
    container_chain_template_simple_runtime::{opaque::Block, RuntimeApi},
    cumulus_client_cli::CollatorOptions,
    cumulus_client_consensus_common::ParachainBlockImport as TParachainBlockImport,
    cumulus_client_service::prepare_node_config,
    cumulus_primitives_core::ParaId,
    sc_executor::NativeElseWasmExecutor,
    sc_service::{Configuration, TFullBackend, TFullClient, TaskManager},
    std::{sync::Arc, time::Duration},
};

/// Native executor type.
pub struct ParachainNativeExecutor;

impl sc_executor::NativeExecutionDispatch for ParachainNativeExecutor {
    type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;

    fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
        container_chain_template_simple_runtime::api::dispatch(method, data)
    }

    fn native_version() -> sc_executor::NativeVersion {
        container_chain_template_simple_runtime::native_version()
    }
}

type ParachainExecutor = NativeElseWasmExecutor<ParachainNativeExecutor>;
type ParachainClient = TFullClient<Block, RuntimeApi, ParachainExecutor>;
type ParachainBackend = TFullBackend<Block>;
type ParachainBlockImport = TParachainBlockImport<Block, Arc<ParachainClient>, ParachainBackend>;

pub struct NodeBuilderConfig;
impl node_common::service::NodeBuilderConfig for NodeBuilderConfig {
    type Block = Block;
    type RuntimeApi = RuntimeApi;
    type ParachainNativeExecutor = ParachainNativeExecutor;
}

pub fn import_queue(
    parachain_config: &Configuration,
    node_builder: &NodeBuilder<NodeBuilderConfig>,
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
    let mut node_builder = NodeBuilderConfig::new_builder(&parachain_config, hwbench.clone())?;

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
