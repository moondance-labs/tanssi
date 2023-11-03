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

use {
    sc_service::{KeystoreContainer, TaskManager},
    sp_block_builder::BlockBuilder,
};

use {
    cumulus_client_consensus_common::ParachainBlockImport as TParachainBlockImport,
    sc_executor::{
        HeapAllocStrategy, NativeElseWasmExecutor, NativeExecutionDispatch, WasmExecutor,
        DEFAULT_HEAP_ALLOC_STRATEGY,
    },
    sc_service::{Configuration, TFullBackend, TFullClient},
    sc_telemetry::{Telemetry, TelemetryWorker, TelemetryWorkerHandle},
    sp_api::ConstructRuntimeApi,
    sp_transaction_pool::runtime_api::TaggedTransactionQueue,
    std::sync::Arc,
};

pub type ParachainExecutor<ParachainNativeExecutor> =
    NativeElseWasmExecutor<ParachainNativeExecutor>;
pub type ParachainClient<Block, RuntimeApi, ParachainNativeExecutor> =
    TFullClient<Block, RuntimeApi, ParachainExecutor<ParachainNativeExecutor>>;
pub type ParachainBackend<Block> = TFullBackend<Block>;
pub type ParachainBlockImport<Block, RuntimeApi, ParachainNativeExecutor> = TParachainBlockImport<
    Block,
    Arc<ParachainClient<Block, RuntimeApi, ParachainNativeExecutor>>,
    ParachainBackend<Block>,
>;
pub type ConstructedRuntimeApi<Block, Client, RuntimeApi> =
    <RuntimeApi as ConstructRuntimeApi<Block, Client>>::RuntimeApi;

pub struct NewPartial<Block, RuntimeApi, ParachainNativeExecutor>
where
    Block: cumulus_primitives_core::BlockT,
    ParachainNativeExecutor: NativeExecutionDispatch + 'static,
    RuntimeApi: ConstructRuntimeApi<Block, ParachainClient<Block, RuntimeApi, ParachainNativeExecutor>>
        + Sync
        + Send
        + 'static,
    ConstructedRuntimeApi<
        Block,
        ParachainClient<Block, RuntimeApi, ParachainNativeExecutor>,
        RuntimeApi,
    >: TaggedTransactionQueue<Block> + BlockBuilder<Block>,
{
    pub client: Arc<ParachainClient<Block, RuntimeApi, ParachainNativeExecutor>>,
    pub backend: Arc<ParachainBackend<Block>>,
    pub task_manager: TaskManager,
    pub keystore_container: KeystoreContainer,
    pub transaction_pool: Arc<
        sc_transaction_pool::FullPool<
            Block,
            ParachainClient<Block, RuntimeApi, ParachainNativeExecutor>,
        >,
    >,
    pub telemetry: Option<Telemetry>,
    pub telemetry_worker_handle: Option<TelemetryWorkerHandle>,
}

pub fn new_partial<Block, RuntimeApi, ParachainNativeExecutor>(
    config: &Configuration,
) -> Result<
    // PartialComponents<
    //     ParachainClient<Block, RuntimeApi, ParachainNativeExecutor>,
    //     ParachainBackend<Block>,
    //     SelectChain,
    //     sc_consensus::DefaultImportQueue<Block>,
    //     sc_transaction_pool::FullPool<
    //         Block,
    //         ParachainClient<Block, RuntimeApi, ParachainNativeExecutor>,
    //     >,
    //     (
    //         ParachainBlockImport<Block, RuntimeApi, ParachainNativeExecutor>,
    //         Option<Telemetry>,
    //         Option<TelemetryWorkerHandle>,
    //     ),
    // >,
    NewPartial<Block, RuntimeApi, ParachainNativeExecutor>,
    sc_service::Error,
>
where
    Block: cumulus_primitives_core::BlockT,
    ParachainNativeExecutor: NativeExecutionDispatch + 'static,
    RuntimeApi: ConstructRuntimeApi<Block, ParachainClient<Block, RuntimeApi, ParachainNativeExecutor>>
        + Sync
        + Send
        + 'static,
    ConstructedRuntimeApi<
        Block,
        ParachainClient<Block, RuntimeApi, ParachainNativeExecutor>,
        RuntimeApi,
    >: TaggedTransactionQueue<Block> + BlockBuilder<Block>,
{
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

    let heap_pages = config
        .default_heap_pages
        .map_or(DEFAULT_HEAP_ALLOC_STRATEGY, |h| HeapAllocStrategy::Static {
            extra_pages: h as _,
        });

    // Default runtime_cache_size is 2
    // For now we can work with this, but it will likely need
    // to change once we start having runtime_cache_sizes, or
    // run nodes with the maximum for this value
    let wasm = WasmExecutor::builder()
        .with_execution_method(config.wasm_method)
        .with_onchain_heap_alloc_strategy(heap_pages)
        .with_offchain_heap_alloc_strategy(heap_pages)
        .with_max_runtime_instances(config.max_runtime_instances)
        .with_runtime_cache_size(config.runtime_cache_size)
        .build();

    let executor: ParachainExecutor<ParachainNativeExecutor> =
        ParachainExecutor::new_with_wasm_executor(wasm);

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

    let transaction_pool = sc_transaction_pool::BasicPool::new_full(
        config.transaction_pool.clone(),
        config.role.is_authority().into(),
        config.prometheus_registry(),
        task_manager.spawn_essential_handle(),
        client.clone(),
    );

    // let block_import = ParachainBlockImport::new(client.clone(), backend.clone());

    // let import_queue = nimbus_consensus::import_queue(
    //     client.clone(),
    //     block_import.clone(),
    //     move |_, _| async move {
    //         let time = sp_timestamp::InherentDataProvider::from_system_time();

    //         Ok((time,))
    //     },
    //     &task_manager.spawn_essential_handle(),
    //     config.prometheus_registry(),
    //     false,
    // )?;

    // Ok(PartialComponents {
    //     backend,
    //     client,
    //     import_queue,
    //     keystore_container,
    //     task_manager,
    //     transaction_pool,
    //     select_chain,
    //     other: (block_import, telemetry, telemetry_worker_handle),
    // })
    Ok(NewPartial {
        client,
        backend,
        transaction_pool,
        telemetry,
        telemetry_worker_handle,
        task_manager,
        keystore_container,
    })
}
