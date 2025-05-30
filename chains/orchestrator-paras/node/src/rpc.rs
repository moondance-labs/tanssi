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

use {
    cumulus_primitives_core::ParaId,
    dancebox_runtime::{opaque::Block, AccountId, Index as Nonce},
    manual_container_chains_exclusion_rpc::{
        ManualContainerChainsExclusion, ManualContainerChainsExclusionApiServer,
    },
    manual_randomness_rpc::{ManualRandomness, ManualRandomnessApiServer},
    manual_xcm_rpc::{ManualXcm, ManualXcmApiServer},
    polkadot_primitives::Hash,
    sc_client_api::{AuxStore, UsageProvider},
    sc_consensus_manual_seal::{
        rpc::{ManualSeal, ManualSealApiServer},
        EngineCommand,
    },
    sc_transaction_pool_api::TransactionPool,
    services_payment_rpc::{
        ServicesPayment, ServicesPaymentApiServer as _, ServicesPaymentRuntimeApi,
    },
    sp_api::ProvideRuntimeApi,
    sp_block_builder::BlockBuilder,
    sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata},
    std::sync::Arc,
    stream_payment_rpc::{StreamPayment, StreamPaymentApiServer as _, StreamPaymentRuntimeApi},
};

/// A type representing all RPC extensions.
pub type RpcExtension = jsonrpsee::RpcModule<()>;

/// Full client dependencies
pub struct FullDeps<C, P> {
    /// The client instance to use.
    pub client: Arc<C>,
    /// Transaction pool instance.
    pub pool: Arc<P>,
    /// Manual seal command sink
    pub command_sink: Option<futures::channel::mpsc::Sender<EngineCommand<Hash>>>,
    /// Channels for manual xcm messages (downward, hrmp)
    pub xcm_senders: Option<(flume::Sender<Vec<u8>>, flume::Sender<(ParaId, Vec<u8>)>)>,
    /// Channels for manually activating the randomness
    pub randomness_sender: Option<flume::Sender<(bool, Option<[u8; 32]>)>>,
    /// Channels for manually excluding container chains from producing blocks
    pub container_chain_exclusion_sender: Option<flume::Sender<Vec<ParaId>>>,
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
        + UsageProvider<Block>
        + 'static,
    C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>,
    C::Api: BlockBuilder<Block>,
    C::Api: StreamPaymentRuntimeApi<Block, u64, u128, u128>,
    C::Api: ServicesPaymentRuntimeApi<Block, u128, ParaId>,
    P: TransactionPool + Sync + Send + 'static,
{
    use substrate_frame_rpc_system::{System, SystemApiServer};

    let mut module = RpcExtension::new(());
    let FullDeps {
        client,
        pool,
        command_sink,
        xcm_senders,
        randomness_sender,
        container_chain_exclusion_sender,
    } = deps;

    module.merge(System::new(client.clone(), pool).into_rpc())?;
    module.merge(StreamPayment::<_, Block>::new(client.clone()).into_rpc())?;
    module.merge(ServicesPayment::<_, Block>::new(client).into_rpc())?;

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

    if let Some(randomness_message_channel) = randomness_sender {
        module.merge(
            ManualRandomness {
                randomness_message_channel,
            }
            .into_rpc(),
        )?;
    }

    if let Some(container_chain_exclusion_message_channel) = container_chain_exclusion_sender {
        module.merge(
            ManualContainerChainsExclusion {
                container_chain_exclusion_message_channel,
            }
            .into_rpc(),
        )?;
    }

    Ok(module)
}
