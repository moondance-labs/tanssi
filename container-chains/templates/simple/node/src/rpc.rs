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

//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

pub use sc_rpc::DenyUnsafe;

use {
    container_chain_template_simple_runtime::{opaque::Block, AccountId, Hash, Index as Nonce},
    cumulus_primitives_core::ParaId,
    manual_xcm_rpc::{ManualXcm, ManualXcmApiServer as _},
    sc_client_api::AuxStore,
    sc_consensus_manual_seal::{
        rpc::{ManualSeal, ManualSealApiServer as _},
        EngineCommand,
    },
    sc_transaction_pool_api::TransactionPool,
    sp_api::ProvideRuntimeApi,
    sp_block_builder::BlockBuilder,
    sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata},
    std::sync::Arc,
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
    /// Channels for manual xcm messages (downward, hrmp)
    pub xcm_senders: Option<(flume::Sender<Vec<u8>>, flume::Sender<(ParaId, Vec<u8>)>)>,
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
        xcm_senders,
    } = deps;

    module.merge(System::new(client, pool, deny_unsafe).into_rpc())?;

    // Manual seal
    if let Some(command_sink) = command_sink {
        module.merge(
            // We provide the rpc handler with the sending end of the channel to allow the rpc
            // send EngineCommands to the background block authorship task.
            ManualSeal::new(command_sink).into_rpc(),
        )?;
    };

    if let Some((downward_message_channel, hrmp_message_channel)) = xcm_senders {
        module.merge(
            ManualXcm {
                downward_message_channel,
                hrmp_message_channel,
            }
            .into_rpc(),
        )?;
    }

    Ok(module)
}
