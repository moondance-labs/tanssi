//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use polkadot_primitives::Id as ParaId;
pub use sc_rpc::{DenyUnsafe, SubscriptionTaskExecutor};
use tp_container_chain_genesis_data::ContainerChainGenesisData;
use {
    polkadot_primitives::Hash,
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
    test_runtime::{opaque::Block, AccountId, Index as Nonce},
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
    } = deps;

    module.merge(System::new(client, pool, deny_unsafe).into_rpc())?;

    module.merge(Utils::new().into_rpc())?;

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
pub struct Utils {}

impl Utils {
    pub fn new() -> Self {
        Self {}
    }
}

/// Utils rpc interface.
#[rpc(server)]
pub trait UtilsApi {
    #[method(name = "utils_raw_chain_spec_into_container_chain_genesis_data")]
    fn raw_chain_spec_into_container_chain_genesis_data(
        &self,
        raw_chain_spec: &str,
    ) -> RpcResult<(ParaId, ContainerChainGenesisData)>;
}

impl UtilsApiServer for Utils {
    fn raw_chain_spec_into_container_chain_genesis_data(
        &self,
        raw_chain_spec: &str,
    ) -> RpcResult<(ParaId, ContainerChainGenesisData)> {
        tp_container_chain_genesis_data::json::container_chain_genesis_data_from_str(raw_chain_spec)
            .map_err(|e| jsonrpsee::core::Error::Custom(e))
    }
}
