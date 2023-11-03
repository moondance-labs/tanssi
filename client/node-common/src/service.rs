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

use {sp_api::ConstructRuntimeApi, sp_transaction_pool::runtime_api::TaggedTransactionQueue};

use {
    cumulus_client_consensus_common::ParachainBlockImport as TParachainBlockImport,
    sc_executor::{NativeElseWasmExecutor, NativeExecutionDispatch},
    sc_service::{Configuration, PartialComponents, TFullBackend, TFullClient},
    sc_telemetry::{Telemetry, TelemetryWorkerHandle},
    sc_transaction_pool::ChainApi,
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

type ConstructedRuntimeApi<Block, Client, RuntimeApi> =
    <RuntimeApi as ConstructRuntimeApi<Block, Client>>::RuntimeApi;

pub trait BlockT: cumulus_primitives_core::BlockT {}

pub fn new_partial<Block, RuntimeApi, ParachainNativeExecutor, SelectChain>(
    config: &Configuration,
) -> Result<
    PartialComponents<
        ParachainClient<Block, RuntimeApi, ParachainNativeExecutor>,
        ParachainBackend<Block>,
        SelectChain,
        sc_consensus::DefaultImportQueue<Block>,
        sc_transaction_pool::FullPool<
            Block,
            ParachainClient<Block, RuntimeApi, ParachainNativeExecutor>,
        >,
        (
            ParachainBlockImport<Block, RuntimeApi, ParachainNativeExecutor>,
            Option<Telemetry>,
            Option<TelemetryWorkerHandle>,
        ),
    >,
    sc_service::Error,
>
where
    Block: BlockT,
    ParachainNativeExecutor: NativeExecutionDispatch + 'static,
    RuntimeApi: ConstructRuntimeApi<Block, ParachainClient<Block, RuntimeApi, ParachainNativeExecutor>>
        + Sync
        + Send
        + 'static,
    ConstructedRuntimeApi<
        Block,
        ParachainClient<Block, RuntimeApi, ParachainNativeExecutor>,
        RuntimeApi,
    >: TaggedTransactionQueue<Block>,
{
    todo!()
}
