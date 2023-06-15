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

//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

pub use sc_rpc::{DenyUnsafe, SubscriptionTaskExecutor};
use {
    jsonrpsee::{core::RpcResult, proc_macros::rpc},
    orchestrator_runtime::{opaque::Block, AccountId, Index as Nonce},
    polkadot_primitives::{Hash, Id as ParaId},
    sc_chain_spec::ChainType,
    sc_client_api::AuxStore,
    sc_consensus_manual_seal::{
        rpc::{ManualSeal, ManualSealApiServer},
        EngineCommand,
    },
    sc_transaction_pool_api::TransactionPool,
    sp_api::ProvideRuntimeApi,
    sp_block_builder::BlockBuilder,
    sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata},
    std::sync::Arc,
    tp_container_chain_genesis_data::ContainerChainGenesisData,
};

/// A type representing all RPC extensions.
pub type RpcExtension = jsonrpsee::RpcModule<()>;

/// Full client dependencies
pub struct FullDeps<C, P> {
    /// The client instance to use.
    pub client: Arc<C>,
    /// Transaction pool instance.
    pub pool: Arc<P>,
    /// Whether to deny unsafe calls
    pub deny_unsafe: DenyUnsafe,
    /// Manual seal command sink
    pub command_sink: Option<futures::channel::mpsc::Sender<EngineCommand<Hash>>>,
    /// Orchestrator chain utils
    pub utils: Option<Utils>,
}

/// Instantiate all RPC extensions.
pub fn create_full<C, P>(
    deps: FullDeps<C, P>,
) -> Result<RpcExtension, Box<dyn std::error::Error + Send + Sync>>
where
    C: ProvideRuntimeApi<Block>
        + HeaderBackend<Block>
        + AuxStore
        + HeaderMetadata<Block, Error = BlockChainError>
        + Send
        + Sync
        + 'static,
    C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>,
    C::Api: BlockBuilder<Block>,
    P: TransactionPool + Sync + Send + 'static,
{
    use substrate_frame_rpc_system::{System, SystemApiServer};

    let mut module = RpcExtension::new(());
    let FullDeps {
        client,
        pool,
        deny_unsafe,
        command_sink,
        utils,
    } = deps;

    module.merge(System::new(client, pool, deny_unsafe).into_rpc())?;

    if let Some(utils) = utils {
        module.merge(utils.into_rpc())?;
    }

    if let Some(command_sink) = command_sink {
        module.merge(
            // We provide the rpc handler with the sending end of the channel to allow the rpc
            // send EngineCommands to the background block authorship task.
            ManualSeal::new(command_sink).into_rpc(),
        )?;
    };

    Ok(module)
}

/// Utils API implementation.
pub struct Utils {
    /// Chain name
    pub chain_name: String,
    /// Chain type
    pub chain_type: ChainType,
}

/// Utils rpc interface.
#[rpc(server)]
pub trait UtilsApi {
    #[method(name = "utils_raw_chain_spec_into_container_chain_genesis_data")]
    fn raw_chain_spec_into_container_chain_genesis_data(
        &self,
        raw_chain_spec: String,
    ) -> RpcResult<(ParaId, ContainerChainGenesisData)>;

    #[method(name = "utils_container_chain_genesis_data_into_raw_chain_spec")]
    fn container_chain_genesis_data_into_raw_chain_spec(
        &self,
        para_id: ParaId,
        container_chain_genesis_data: ContainerChainGenesisData,
    ) -> RpcResult<String>;
}

impl UtilsApiServer for Utils {
    fn raw_chain_spec_into_container_chain_genesis_data(
        &self,
        raw_chain_spec: String,
    ) -> RpcResult<(ParaId, ContainerChainGenesisData)> {
        tp_container_chain_genesis_data::json::container_chain_genesis_data_from_str(
            &raw_chain_spec,
        )
        .map_err(|e| jsonrpsee::core::Error::Custom(e))
    }

    fn container_chain_genesis_data_into_raw_chain_spec(
        &self,
        para_id: ParaId,
        container_chain_genesis_data: ContainerChainGenesisData,
    ) -> RpcResult<String> {
        let relay_chain = match self.chain_name.as_str() {
            "dancebox" => "westend",
            _ => "rococo-local",
        };
        let chain_type = self.chain_type.clone();
        let raw_chain_spec = crate::cli::ContainerChainCli::chain_spec_from_genesis_data(
            para_id.into(),
            container_chain_genesis_data,
            chain_type,
            relay_chain.to_string(),
        )
        .map_err(|e| jsonrpsee::core::Error::Custom(e))?;

        raw_chain_spec
            .as_json(true)
            .map_err(|e| jsonrpsee::core::Error::Custom(e))
    }
}
