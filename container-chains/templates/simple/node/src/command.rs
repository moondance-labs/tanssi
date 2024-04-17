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
    crate::{
        chain_spec,
        cli::{Cli, RelayChainCli, Subcommand},
        service::{self, NodeConfig},
    },
    container_chain_template_simple_runtime::Block,
    cumulus_primitives_core::ParaId,
    dc_orchestrator_chain_interface::OrchestratorChainInterface,
    frame_benchmarking_cli::{BenchmarkCmd, SUBSTRATE_REFERENCE_HARDWARE},
    futures::stream::StreamExt,
    log::{info, warn},
    node_common::{command::generate_genesis_block, service::NodeBuilderConfig as _},
    parity_scale_codec::Encode,
    polkadot_service::{IdentifyVariant as _, TaskManager},
    sc_cli::{
        ChainSpec, CliConfiguration, DefaultConfigurationValues, ImportParams, KeystoreParams,
        NetworkParams, Result, SharedParams, SubstrateCli,
    },
    sc_service::config::{BasePath, PrometheusConfig},
    sp_core::hexdisplay::HexDisplay,
    sp_runtime::traits::{AccountIdConversion, Block as BlockT},
    std::net::SocketAddr,
};

fn load_spec(id: &str, para_id: ParaId) -> std::result::Result<Box<dyn ChainSpec>, String> {
    Ok(match id {
        "dev" => Box::new(chain_spec::development_config(para_id, vec![])),
        "template-rococo" => Box::new(chain_spec::local_testnet_config(para_id, vec![])),
        "" | "local" => Box::new(chain_spec::local_testnet_config(para_id, vec![])),
        path => Box::new(chain_spec::ChainSpec::from_json_file(
            std::path::PathBuf::from(path),
        )?),
    })
}

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "Container Chain Simple Node".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        format!(
            "Container Chain Simple Node\n\nThe command-line arguments provided first will be \
        passed to the parachain node, while the arguments provided after -- will be passed \
        to the relay chain node.\n\n\
        {} <parachain-args> -- <relay-chain-args>",
            Self::executable_name()
        )
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://github.com/paritytech/cumulus/issues/new".into()
    }

    fn copyright_start_year() -> i32 {
        2020
    }

    fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
        load_spec(id, self.para_id.unwrap_or(2000).into())
    }
}

impl SubstrateCli for RelayChainCli {
    fn impl_name() -> String {
        "Container Chain Simple Node".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        format!(
            "Container Chain Simple Node\n\nThe command-line arguments provided first will be \
        passed to the parachain node, while the arguments provided after -- will be passed \
        to the relay chain node.\n\n\
        {} <parachain-args> -- <relay-chain-args>",
            Self::executable_name()
        )
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://github.com/paritytech/cumulus/issues/new".into()
    }

    fn copyright_start_year() -> i32 {
        2020
    }

    fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
        polkadot_cli::Cli::from_iter([RelayChainCli::executable_name()].iter()).load_spec(id)
    }
}

macro_rules! construct_async_run {
    (|$components:ident, $cli:ident, $cmd:ident, $config:ident| $( $code:tt )* ) => {{
        let runner = $cli.create_runner($cmd)?;
        runner.async_run(|$config| {
            let $components = NodeConfig::new_builder(&$config, None)?;
            let inner = { $( $code )* };

            let task_manager = $components.task_manager;
            inner.map(|v| (v, task_manager))
        })
    }}
}

/// Parse command line arguments into service configuration.
pub fn run() -> Result<()> {
    let cli = Cli::from_args();

    match &cli.subcommand {
        Some(Subcommand::BuildSpec(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| {
                let chain_spec = if let Some(para_id) = cmd.parachain_id {
                    if cmd.base.shared_params.dev {
                        Box::new(chain_spec::development_config(
                            para_id.into(),
                            cmd.add_bootnode.clone(),
                        ))
                    } else {
                        Box::new(chain_spec::local_testnet_config(
                            para_id.into(),
                            cmd.add_bootnode.clone(),
                        ))
                    }
                } else {
                    config.chain_spec
                };
                cmd.base.run(chain_spec, config.network)
            })
        }
        Some(Subcommand::CheckBlock(cmd)) => {
            construct_async_run!(|components, cli, cmd, config| {
                let (_, import_queue) = service::import_queue(&config, &components);
                Ok(cmd.run(components.client, import_queue))
            })
        }
        Some(Subcommand::ExportBlocks(cmd)) => {
            construct_async_run!(|components, cli, cmd, config| {
                Ok(cmd.run(components.client, config.database))
            })
        }
        Some(Subcommand::ExportState(cmd)) => {
            construct_async_run!(|components, cli, cmd, config| {
                Ok(cmd.run(components.client, config.chain_spec))
            })
        }
        Some(Subcommand::ImportBlocks(cmd)) => {
            construct_async_run!(|components, cli, cmd, config| {
                let (_, import_queue) = service::import_queue(&config, &components);
                Ok(cmd.run(components.client, import_queue))
            })
        }
        Some(Subcommand::Revert(cmd)) => {
            construct_async_run!(|components, cli, cmd, config| {
                Ok(cmd.run(components.client, components.backend, None))
            })
        }
        Some(Subcommand::PurgeChain(cmd)) => {
            let runner = cli.create_runner(cmd)?;

            runner.sync_run(|config| {
                let polkadot_cli = RelayChainCli::new(
                    &config,
                    [RelayChainCli::executable_name()]
                        .iter()
                        .chain(cli.relay_chain_args.iter()),
                );

                let polkadot_config = SubstrateCli::create_configuration(
                    &polkadot_cli,
                    &polkadot_cli,
                    config.tokio_handle.clone(),
                )
                .map_err(|err| format!("Relay chain argument error: {}", err))?;

                cmd.run(config, polkadot_config)
            })
        }
        Some(Subcommand::ExportGenesisHead(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| {
                let partials = NodeConfig::new_builder(&config, None)?;
                cmd.run(partials.client)
            })
        }
        Some(Subcommand::ExportGenesisWasm(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|_config| {
                let spec = cli.load_spec(&cmd.shared_params.chain.clone().unwrap_or_default())?;
                cmd.run(&*spec)
            })
        }
        Some(Subcommand::Benchmark(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            // Switch on the concrete benchmark sub-command-
            match cmd {
                BenchmarkCmd::Pallet(cmd) => {
                    if cfg!(feature = "runtime-benchmarks") {
                        runner.sync_run(|config| cmd.run::<Block, ()>(config))
                    } else {
                        Err("Benchmarking wasn't enabled when building the node. \
                    You can enable it with `--features runtime-benchmarks`."
                            .into())
                    }
                }
                BenchmarkCmd::Block(cmd) => runner.sync_run(|config| {
                    let partials = NodeConfig::new_builder(&config, None)?;
                    cmd.run(partials.client)
                }),
                #[cfg(not(feature = "runtime-benchmarks"))]
                BenchmarkCmd::Storage(_) => Err(sc_cli::Error::Input(
                    "Compile with --features=runtime-benchmarks \
                        to enable storage benchmarks."
                        .into(),
                )),
                #[cfg(feature = "runtime-benchmarks")]
                BenchmarkCmd::Storage(cmd) => runner.sync_run(|config| {
                    let partials = NodeConfig::new_builder(&config, None)?;
                    let db = partials.backend.expose_db();
                    let storage = partials.backend.expose_storage();
                    cmd.run(config, partials.client.clone(), db, storage)
                }),
                BenchmarkCmd::Machine(cmd) => {
                    runner.sync_run(|config| cmd.run(&config, SUBSTRATE_REFERENCE_HARDWARE.clone()))
                }
                // NOTE: this allows the Client to leniently implement
                // new benchmark commands without requiring a companion MR.
                #[allow(unreachable_patterns)]
                _ => Err("Benchmarking sub-command unsupported".into()),
            }
        }
        #[cfg(feature = "try-runtime")]
        Some(Subcommand::TryRuntime(_)) => {
            Err("Substrate's `try-runtime` subcommand has been migrated \
            to a standalone CLI (https://github.com/paritytech/try-runtime-cli)"
                .into())
        }
        #[cfg(not(feature = "try-runtime"))]
        Some(Subcommand::TryRuntime) => {
            Err("Substrate's `try-runtime` subcommand has been migrated \
            to a standalone CLI (https://github.com/paritytech/try-runtime-cli)"
                .into())
        }
        Some(Subcommand::PrecompileWasm(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let partials = NodeConfig::new_builder(&config, None)?;
                Ok((
                    cmd.run(partials.backend, config.chain_spec),
                    partials.task_manager,
                ))
            })
        }
        Some(Subcommand::RpcProvider(cmd)) => {
            let runner = cli.create_runner(&cli.run.normalize())?;

            runner.run_node_until_exit(|_config| async move {
                let client: Box<dyn OrchestratorChainInterface>;
                let mut task_manager;

                if cmd.orchestrator_endpoints.is_empty() {
                    todo!("Start in process node")
                } else {
                    task_manager = TaskManager::new(tokio::runtime::Handle::current(), None)
                        .map_err(|e| sc_cli::Error::Application(Box::new(e)))?;

                    client = dc_orchestrator_chain_rpc_interface::create_client_and_start_worker(
                        cmd.orchestrator_endpoints.clone(),
                        &mut task_manager,
                        None,
                    )
                    .await
                    .map(Box::new)
                    .map_err(|e| sc_cli::Error::Application(Box::new(e)))?;
                };

                // POC: Try to fetch some data through the interface.
                task_manager
                    .spawn_handle()
                    .spawn("rpc_provider_exemple", None, async move {
                        let mut stream = client.new_best_notification_stream().await.unwrap();

                        while let Some(header) = stream.next().await {
                            log::info!("New best block: {}", header.hash());
                        }
                    });

                Ok(task_manager)
            })
        }
        None => {
            let runner = cli.create_runner(&cli.run.normalize())?;
            let collator_options = cli.run.collator_options();

            runner.run_node_until_exit(|config| async move {
                let hwbench = (!cli.no_hardware_benchmarks).then_some(
                    config.database.path().map(|database_path| {
                        let _ = std::fs::create_dir_all(database_path);
                        sc_sysinfo::gather_hwbench(Some(database_path))
                    })).flatten();

                let para_id = chain_spec::Extensions::try_get(&*config.chain_spec)
                    .map(|e| e.para_id)
                    .ok_or("Could not find parachain ID in chain-spec.")?;

                let polkadot_cli = RelayChainCli::new(
                    &config,
                    [RelayChainCli::executable_name()].iter().chain(cli.relay_chain_args.iter()),
                );

                let extension = chain_spec::Extensions::try_get(&*config.chain_spec);
                let relay_chain_id = extension.map(|e| e.relay_chain.clone());

                let dev_service =
                    config.chain_spec.is_dev() || relay_chain_id == Some("dev-service".to_string());

                let id = ParaId::from(para_id);

                if dev_service {
                    return crate::service::start_dev_node(config, cli.run.sealing, id, hwbench).await
                    .map_err(Into::into)
                }


                let parachain_account =
                    AccountIdConversion::<polkadot_primitives::AccountId>::into_account_truncating(&id);

                // We log both genesis states for reference, as fetching it from runtime would take significant time
                let block_state_v0: Block = generate_genesis_block(&*config.chain_spec, sp_runtime::StateVersion::V0)
                    .map_err(|e| format!("{:?}", e))?;
                let block_state_v1: Block = generate_genesis_block(&*config.chain_spec, sp_runtime::StateVersion::V1)
                    .map_err(|e| format!("{:?}", e))?;

                let genesis_state_v0 = format!("0x{:?}", HexDisplay::from(&block_state_v0.header().encode()));
                let genesis_state_v1 = format!("0x{:?}", HexDisplay::from(&block_state_v1.header().encode()));

                let tokio_handle = config.tokio_handle.clone();
                let polkadot_config =
                    SubstrateCli::create_configuration(&polkadot_cli, &polkadot_cli, tokio_handle)
                        .map_err(|err| format!("Relay chain argument error: {}", err))?;

                info!("Parachain id: {:?}", id);
                info!("Parachain Account: {}", parachain_account);
                info!("Parachain genesis state V0: {}", genesis_state_v0);
                info!("Parachain genesis state V1: {}", genesis_state_v1);
                info!("Is collating: {}", if config.role.is_authority() { "yes" } else { "no" });

                if let cumulus_client_cli::RelayChainMode::ExternalRpc(rpc_target_urls) =
                    collator_options.clone().relay_chain_mode {
                    if !rpc_target_urls.is_empty() && !cli.relay_chain_args.is_empty() {
                        warn!("Detected relay chain node arguments together with --relay-chain-rpc-url. This command starts a minimal Polkadot node that only uses a network-related subset of all relay chain CLI options.");
                    }
                }

                crate::service::start_parachain_node(
                    config,
                    polkadot_config,
                    collator_options,
                    id,
                    hwbench,
                )
                .await
                .map(|r| r.0)
                .map_err(Into::into)
            })
        }
    }
}

impl DefaultConfigurationValues for RelayChainCli {
    fn p2p_listen_port() -> u16 {
        30334
    }

    fn rpc_listen_port() -> u16 {
        9945
    }

    fn prometheus_listen_port() -> u16 {
        9616
    }
}

impl CliConfiguration<Self> for RelayChainCli {
    fn shared_params(&self) -> &SharedParams {
        self.base.base.shared_params()
    }

    fn import_params(&self) -> Option<&ImportParams> {
        self.base.base.import_params()
    }

    fn network_params(&self) -> Option<&NetworkParams> {
        self.base.base.network_params()
    }

    fn keystore_params(&self) -> Option<&KeystoreParams> {
        self.base.base.keystore_params()
    }

    fn base_path(&self) -> Result<Option<BasePath>> {
        Ok(self
            .shared_params()
            .base_path()?
            .or_else(|| Some(self.base_path.clone().into())))
    }

    fn rpc_addr(&self, default_listen_port: u16) -> Result<Option<SocketAddr>> {
        self.base.base.rpc_addr(default_listen_port)
    }
    fn prometheus_config(
        &self,
        default_listen_port: u16,
        chain_spec: &Box<dyn ChainSpec>,
    ) -> Result<Option<PrometheusConfig>> {
        self.base
            .base
            .prometheus_config(default_listen_port, chain_spec)
    }

    fn init<F>(
        &self,
        _support_url: &String,
        _impl_version: &String,
        _logger_hook: F,
        _config: &sc_service::Configuration,
    ) -> Result<()>
    where
        F: FnOnce(&mut sc_cli::LoggerBuilder, &sc_service::Configuration),
    {
        unreachable!("PolkadotCli is never initialized; qed");
    }

    fn chain_id(&self, is_dev: bool) -> Result<String> {
        let chain_id = self.base.base.chain_id(is_dev)?;

        Ok(if chain_id.is_empty() {
            self.chain_id.clone().unwrap_or_default()
        } else {
            chain_id
        })
    }

    fn role(&self, is_dev: bool) -> Result<sc_service::Role> {
        self.base.base.role(is_dev)
    }

    fn transaction_pool(&self, is_dev: bool) -> Result<sc_service::config::TransactionPoolOptions> {
        self.base.base.transaction_pool(is_dev)
    }

    fn trie_cache_maximum_size(&self) -> Result<Option<usize>> {
        self.base.base.trie_cache_maximum_size()
    }

    fn rpc_methods(&self) -> Result<sc_service::config::RpcMethods> {
        self.base.base.rpc_methods()
    }

    fn rpc_max_connections(&self) -> Result<u32> {
        self.base.base.rpc_max_connections()
    }

    fn rpc_cors(&self, is_dev: bool) -> Result<Option<Vec<String>>> {
        self.base.base.rpc_cors(is_dev)
    }

    fn default_heap_pages(&self) -> Result<Option<u64>> {
        self.base.base.default_heap_pages()
    }

    fn force_authoring(&self) -> Result<bool> {
        self.base.base.force_authoring()
    }

    fn disable_grandpa(&self) -> Result<bool> {
        self.base.base.disable_grandpa()
    }

    fn max_runtime_instances(&self) -> Result<Option<usize>> {
        self.base.base.max_runtime_instances()
    }

    fn announce_block(&self) -> Result<bool> {
        self.base.base.announce_block()
    }

    fn telemetry_endpoints(
        &self,
        chain_spec: &Box<dyn ChainSpec>,
    ) -> Result<Option<sc_telemetry::TelemetryEndpoints>> {
        self.base.base.telemetry_endpoints(chain_spec)
    }

    fn node_name(&self) -> Result<String> {
        self.base.base.node_name()
    }
}
