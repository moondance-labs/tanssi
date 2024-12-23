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
    frame_support::{CloneNoBound, DefaultNoBound},
    manual_xcm_rpc::{ManualXcm, ManualXcmApiServer},
    polkadot_primitives::Hash,
    sc_client_api::{AuxStore, UsageProvider},
    sc_consensus_manual_seal::{
        rpc::{ManualSeal, ManualSealApiServer},
        EngineCommand,
    },
    sc_transaction_pool_api::TransactionPool,
    sp_api::ProvideRuntimeApi,
    sp_block_builder::BlockBuilder,
    sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata},
    std::{marker::PhantomData, sync::Arc},
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
    pub xcm_senders: Option<(
        flume::Sender<Vec<u8>>,
        flume::Sender<Vec<u8>>,
        flume::Sender<(ParaId, Vec<u8>)>,
    )>,
}

tp_traits::alias!(
    /// Test
    pub trait SubstrateRpcRuntimeApi<Client : (sp_api::CallApiAt<Block>)>:
        sp_api::ConstructRuntimeApi<
            Block,
            Client,
            RuntimeApi:
                substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>
                + BlockBuilder<Block>
        > + Send + Sync + 'static
);

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
    P: TransactionPool + Sync + Send + 'static,
{
    use substrate_frame_rpc_system::{System, SystemApiServer};

    let mut module = RpcExtension::new(());
    let FullDeps {
        client,
        pool,
        command_sink,
        xcm_senders,
    } = deps;

    module.merge(System::new(client.clone(), pool).into_rpc())?;

    if let Some(command_sink) = command_sink {
        module.merge(
            // We provide the rpc handler with the sending end of the channel to allow the rpc
            // send EngineCommands to the background block authorship task.
            ManualSeal::new(command_sink).into_rpc(),
        )?;
    };

    if let Some((downward_message_channel, upward_message_channel, hrmp_message_channel)) =
        xcm_senders
    {
        module.merge(
            ManualXcm {
                downward_message_channel,
                upward_message_channel,
                hrmp_message_channel,
            }
            .into_rpc(),
        )?;
    }

    Ok(module)
}

/// Contains the `GenerateRpcBuilder` trait and defines or re-exports all types it uses.
pub mod generate_rpc_builder {
    // We re-export types with specific type parameters, no need to be verbose documenting that.
    #![allow(missing_docs)]

    pub use {
        crate::service::{ContainerChainBackend, ContainerChainClient, MinimalContainerRuntimeApi},
        sc_service::{Error as ServiceError, TaskManager},
        std::sync::Arc,
        substrate_prometheus_endpoint::Registry as PrometheusRegistry,
        tc_consensus::ParaId,
    };

    // TODO: It would be better to use a container chain types.
    pub use dancebox_runtime::{opaque::Block, Hash};

    pub type SyncingService = sc_network_sync::SyncingService<Block>;
    pub type TransactionPool<RuntimeApi> =
        sc_transaction_pool::TransactionPoolHandle<Block, ContainerChainClient<RuntimeApi>>;
    pub type CommandSink =
        futures::channel::mpsc::Sender<sc_consensus_manual_seal::EngineCommand<Hash>>;
    pub type XcmSenders = (
        flume::Sender<Vec<u8>>,
        flume::Sender<Vec<u8>>,
        flume::Sender<(ParaId, Vec<u8>)>,
    );
    pub type Network = dyn sc_network::service::traits::NetworkService;
    pub type CompleteRpcBuilder = Box<
        dyn Fn(sc_rpc::SubscriptionTaskExecutor) -> Result<jsonrpsee::RpcModule<()>, ServiceError>,
    >;

    pub struct GenerateRpcBuilderParams<'a, RuntimeApi: MinimalContainerRuntimeApi> {
        pub task_manager: &'a TaskManager,
        pub container_chain_config: &'a sc_service::Configuration,

        pub client: Arc<ContainerChainClient<RuntimeApi>>,
        pub backend: Arc<ContainerChainBackend>,
        pub sync_service: Arc<SyncingService>,
        pub transaction_pool: Arc<TransactionPool<RuntimeApi>>,
        pub prometheus_registry: Option<PrometheusRegistry>,
        pub command_sink: Option<CommandSink>,
        pub xcm_senders: Option<XcmSenders>,
        pub network: Arc<Network>,
    }

    pub trait GenerateRpcBuilder<RuntimeApi: MinimalContainerRuntimeApi>:
        Clone + Sync + Send
    {
        fn generate(
            &self,
            params: GenerateRpcBuilderParams<RuntimeApi>,
        ) -> Result<CompleteRpcBuilder, ServiceError>;
    }
}

/// Generate an rpc builder for simple substrate container chains.
#[derive(CloneNoBound, DefaultNoBound)]
pub struct GenerateSubstrateRpcBuilder<RuntimeApi>(pub PhantomData<RuntimeApi>);
impl<RuntimeApi> GenerateSubstrateRpcBuilder<RuntimeApi> {
    /// Creates a new instance.
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

mod impl_generate_rpc_builder {
    use {super::*, generate_rpc_builder::*};

    impl<
            RuntimeApi: MinimalContainerRuntimeApi
                + crate::rpc::SubstrateRpcRuntimeApi<ContainerChainClient<RuntimeApi>>,
        > GenerateRpcBuilder<RuntimeApi> for GenerateSubstrateRpcBuilder<RuntimeApi>
    {
        fn generate(
            &self,
            GenerateRpcBuilderParams {
                client,
                transaction_pool,
                command_sink,
                xcm_senders,
                ..
            }: GenerateRpcBuilderParams<RuntimeApi>,
        ) -> Result<CompleteRpcBuilder, ServiceError> {
            let client = client.clone();
            let transaction_pool = transaction_pool.clone();

            Ok(Box::new(move |_| {
                let deps = FullDeps {
                    client: client.clone(),
                    pool: transaction_pool.clone(),
                    command_sink: command_sink.clone(),
                    xcm_senders: xcm_senders.clone(),
                };

                create_full(deps).map_err(Into::into)
            }))
        }
    }
}
