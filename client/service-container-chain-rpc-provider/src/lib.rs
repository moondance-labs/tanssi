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

pub mod watch_assignment;

use {
    cumulus_client_cli::CollatorOptions,
    cumulus_client_service::build_relay_chain_interface,
    cumulus_primitives_core::ParaId,
    dc_orchestrator_chain_interface::OrchestratorChainInterface,
    sc_service::{Configuration, KeystoreContainer, TaskManager},
    sc_telemetry::TelemetryWorker,
    std::{marker::PhantomData, sync::Arc},
    tc_service_container_chain_spawner::{
        cli::{ContainerChainCli, ContainerChainRunCmd},
        rpc::generate_rpc_builder::GenerateRpcBuilder,
        service::MinimalContainerRuntimeApi,
        spawner::{ContainerChainSpawnParams, ContainerChainSpawner},
    },
    url::Url,
};

#[derive(Clone, Debug, clap::Parser)]
pub struct RpcProviderCmd {
    /// Arguments to run a container chain node.
    #[command(flatten)]
    pub container_run: ContainerChainRunCmd,

    /// Profile id associated with the node, whose assignements will be followed to provide RPC services.
    #[arg(long)]
    pub profile_id: u64,

    /// Endpoints to connect to orchestrator nodes, avoiding to start a local orchestrator node.
    /// If this list is empty, a local embeded orchestrator node is started.
    #[arg(long)]
    pub orchestrator_endpoints: Vec<Url>,

    /// If running an embeded node, will run it as a solochain orchestrator.
    /// If not present, will run it as a parachain orchestrator.
    #[arg(long)]
    pub solochain: bool,

    /// Either:
    /// - `relay chain args`
    /// - `orchestrator chain args -- relay chain args`
    #[arg(raw = true)]
    pub extra_args: Vec<String>,
}

impl RpcProviderCmd {
    fn split_extra_args_at_first_dashdash(&self) -> (&[String], &[String]) {
        let index_of_dashdash = self.extra_args.iter().position(|x| *x == "--");

        if let Some(i) = index_of_dashdash {
            let (orchestrator_chain_args, extra_extra) = self.extra_args.split_at(i);
            (&extra_extra[1..], orchestrator_chain_args)
        } else {
            // Only relay chain args
            (&self.extra_args, &[])
        }
    }

    pub fn relaychain_args(&self) -> &[String] {
        let (relay_chain_args, _) = self.split_extra_args_at_first_dashdash();

        relay_chain_args
    }

    pub fn orchestrator_chain_args(&self) -> &[String] {
        let (_, orchestrator_chain_args) = self.split_extra_args_at_first_dashdash();

        orchestrator_chain_args
    }
}

pub struct RpcProviderMode<PolkaCli, GRB, RuntimeApi, DVC> {
    /// General configuration
    pub config: Configuration,
    pub provider_profile_id: u64,

    pub solochain: bool,
    pub orchestrator_endpoints: Vec<Url>,
    pub collator_options: CollatorOptions,
    pub polkadot_cli: PolkaCli,
    pub container_chain_cli: ContainerChainCli,
    pub generate_rpc_builder: GRB,

    pub phantom: PhantomData<(RuntimeApi, DVC)>,
}

impl<PolkaCli, GRB, RuntimeApi, DVC> RpcProviderMode<PolkaCli, GRB, RuntimeApi, DVC>
where
    DVC: sc_cli::DefaultConfigurationValues,
    PolkaCli: sc_cli::CliConfiguration<DVC> + sc_cli::SubstrateCli,
    RuntimeApi: MinimalContainerRuntimeApi,
    GRB: GenerateRpcBuilder<RuntimeApi> + 'static,
{
    pub async fn run(self) -> sc_cli::Result<TaskManager> {
        let orchestrator_chain_interface: Arc<dyn OrchestratorChainInterface>;
        let mut task_manager;

        if self.orchestrator_endpoints.is_empty() {
            todo!("Start in process node")
        } else {
            task_manager = TaskManager::new(self.config.tokio_handle.clone(), None)
                .map_err(|e| sc_cli::Error::Application(Box::new(e)))?;

            orchestrator_chain_interface =
                tc_orchestrator_chain_interface_through_rpc::create_client_and_start_worker(
                    self.orchestrator_endpoints.clone(),
                    &mut task_manager,
                    None,
                )
                .await
                .map(Arc::new)
                .map_err(|e| sc_cli::Error::Application(Box::new(e)))?;
        };

        // Spawn assignment watcher
        {
            let mut container_chain_cli = self.container_chain_cli;

            // If the container chain args have no --wasmtime-precompiled flag, use the same as the orchestrator
            if container_chain_cli
                .base
                .base
                .import_params
                .wasmtime_precompiled
                .is_none()
            {
                container_chain_cli
                    .base
                    .base
                    .import_params
                    .wasmtime_precompiled
                    .clone_from(&self.config.executor.wasmtime_precompiled);
            }

            log::info!("Container chain CLI: {container_chain_cli:?}");

            let para_id = node_common::chain_spec::Extensions::try_get(&*self.config.chain_spec)
                .map(|e| e.para_id)
                .ok_or("Could not find parachain ID in chain-spec.")?;

            let para_id = ParaId::from(para_id);

            // TODO: Once there is an embeded node this should use it.
            let keystore_container = KeystoreContainer::new(&self.config.keystore)?;

            let collator_options = self.collator_options;

            let tokio_handle = self.config.tokio_handle.clone();
            let polkadot_config = sc_cli::SubstrateCli::create_configuration(
                &self.polkadot_cli,
                &self.polkadot_cli,
                tokio_handle,
            )
            .map_err(|err| format!("Relay chain argument error: {}", err))?;

            let telemetry = self
                .config
                .telemetry_endpoints
                .clone()
                .filter(|x| !x.is_empty())
                .map(|endpoints| -> std::result::Result<_, sc_telemetry::Error> {
                    let worker = TelemetryWorker::new(16)?;
                    let telemetry = worker.handle().new_telemetry(endpoints);
                    Ok((worker, telemetry))
                })
                .transpose()
                .map_err(sc_service::Error::Telemetry)?;

            let telemetry_worker_handle = telemetry.as_ref().map(|(worker, _)| worker.handle());

            let (relay_chain_interface, _collation_pair) = build_relay_chain_interface(
                polkadot_config,
                &self.config,
                telemetry_worker_handle,
                &mut task_manager,
                collator_options,
                None,
            )
            .await
            .map_err(|e| sc_service::Error::Application(Box::new(e) as Box<_>))?;

            let relay_chain =
                node_common::chain_spec::Extensions::try_get(&*self.config.chain_spec)
                    .map(|e| e.relay_chain.clone())
                    .ok_or("Could not find relay_chain extension in chain-spec.")?;

            let container_chain_spawner = ContainerChainSpawner {
                params: ContainerChainSpawnParams {
                    orchestrator_chain_interface,
                    container_chain_cli,
                    tokio_handle: self.config.tokio_handle.clone(),
                    chain_type: self.config.chain_spec.chain_type(),
                    relay_chain,
                    relay_chain_interface,
                    sync_keystore: keystore_container.keystore(),
                    orchestrator_para_id: para_id,
                    collation_params: None,
                    spawn_handle: task_manager.spawn_handle().clone(),
                    data_preserver: true,
                    generate_rpc_builder: self.generate_rpc_builder,
                    phantom: PhantomData,
                },
                state: Default::default(),
                // db cleanup task disabled here because it uses collator assignment to decide
                // which folders to keep and this is not a collator, this is an rpc node
                db_folder_cleanup_done: true,
                collate_on_tanssi: Arc::new(|| {
                    panic!("Called collate_on_tanssi outside of Tanssi node")
                }),
                collation_cancellation_constructs: None,
            };
            let state = container_chain_spawner.state.clone();

            task_manager.spawn_essential_handle().spawn(
                "container-chain-assignment-watcher",
                None,
                crate::watch_assignment::task_watch_assignment(
                    container_chain_spawner,
                    self.provider_profile_id,
                ),
            );

            task_manager.spawn_essential_handle().spawn(
                "container-chain-spawner-debug-state",
                None,
                tc_service_container_chain_spawner::monitor::monitor_task(state),
            );
        }

        Ok(task_manager)
    }
}
