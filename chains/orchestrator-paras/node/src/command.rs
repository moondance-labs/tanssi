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
        service::{self, IdentifyVariant, NodeConfig},
    },
    cumulus_client_cli::extract_genesis_wasm,
    cumulus_client_service::storage_proof_size::HostFunctions as ReclaimHostFunctions,
    cumulus_primitives_core::ParaId,
    dancebox_runtime::Block,
    frame_benchmarking_cli::{BenchmarkCmd, SUBSTRATE_REFERENCE_HARDWARE},
    log::{info, warn},
    node_common::{command::generate_genesis_block, service::NodeBuilderConfig as _},
    parity_scale_codec::Encode,
    polkadot_service::WestendChainSpec,
    sc_cli::{
        ChainSpec, CliConfiguration, DefaultConfigurationValues, ImportParams, KeystoreParams,
        NetworkParams, Result, SharedParams, SubstrateCli,
    },
    sc_service::config::{BasePath, PrometheusConfig},
    sp_core::hexdisplay::HexDisplay,
    sp_runtime::traits::{AccountIdConversion, Block as BlockT},
    std::io::Write,
    tc_service_container_chain::{chain_spec::RawChainSpec, cli::ContainerChainCli},
};

pub mod solochain;

fn load_spec(
    id: &str,
    para_id: Option<u32>,
    container_chains: Vec<String>,
    mock_container_chains: Vec<u32>,
    invulnerables: Option<Vec<String>>,
) -> std::result::Result<Box<dyn ChainSpec>, String> {
    let para_id: ParaId = para_id.unwrap_or(1000).into();
    let mock_container_chains: Vec<ParaId> =
        mock_container_chains.iter().map(|&x| x.into()).collect();
    let invulnerables = invulnerables.unwrap_or(vec![
        "Alice".to_string(),
        "Bob".to_string(),
        "Charlie".to_string(),
        "Dave".to_string(),
    ]);

    Ok(match id {
        "dev" | "dancebox-dev" | "dancebox_dev" => {
            Box::new(chain_spec::dancebox::development_config(
                para_id,
                container_chains,
                mock_container_chains,
                invulnerables,
            ))
        }
        "" | "dancebox-local" | "dancebox_local" => {
            Box::new(chain_spec::dancebox::local_dancebox_config(
                para_id,
                container_chains,
                mock_container_chains,
                invulnerables,
            ))
        }
        "dancebox" => Box::new(RawChainSpec::from_json_bytes(
            &include_bytes!("../../../../specs/dancebox/dancebox-raw-specs.json")[..],
        )?),
        "flashbox-dev" | "flashbox_dev" => Box::new(chain_spec::flashbox::development_config(
            para_id,
            container_chains,
            mock_container_chains,
            invulnerables,
        )),
        "flashbox-local" | "flashbox_local" => {
            Box::new(chain_spec::flashbox::local_flashbox_config(
                para_id,
                container_chains,
                mock_container_chains,
                invulnerables,
            ))
        }
        path => Box::new(chain_spec::dancebox::ChainSpec::from_json_file(
            std::path::PathBuf::from(path),
        )?),
    })
}

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "Tanssi Collator".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        format!(
            "Tanssi Collator\n\nThe command-line arguments provided first will be \
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
        load_spec(id, self.para_id, vec![], vec![2000, 2001], None)
    }
}

impl SubstrateCli for RelayChainCli {
    fn impl_name() -> String {
        "Tanssi Collator".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        format!(
            "Tanssi Collator\n\nThe command-line arguments provided first will be \
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
        match id {
            "westend_moonbase_relay_testnet" => Ok(Box::new(WestendChainSpec::from_json_bytes(
                &include_bytes!("../../../../specs/dancebox/alphanet-relay-raw-specs.json")[..],
            )?)),
            // If we are not using a moonbeam-centric pre-baked relay spec, then fall back to the
            // Polkadot service to interpret the id.
            _ => polkadot_cli::Cli::from_iter([RelayChainCli::executable_name()].iter())
                .load_spec(id),
        }
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
                let chain_spec = load_spec(
                    &cmd.base.chain_id(cmd.base.is_dev()?)?,
                    cmd.extra.parachain_id,
                    cmd.extra.add_container_chain.clone().unwrap_or_default(),
                    cmd.extra.mock_container_chain.clone().unwrap_or_default(),
                    cmd.extra.invulnerable.clone(),
                )?;
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
                        .chain(cli.relaychain_args().iter()),
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
                let client = NodeConfig::new_builder(&config, None)?.client;
                cmd.run(client)
            })
        }
        Some(Subcommand::ExportGenesisWasm(params)) => {
            let mut builder = sc_cli::LoggerBuilder::new("");
            builder.with_profiling(sc_tracing::TracingReceiver::Log, "");
            let _ = builder.init();

            let raw_wasm_blob =
                extract_genesis_wasm(&*cli.load_spec(&params.chain.clone().unwrap_or_default())?)?;
            let output_buf = if params.raw {
                raw_wasm_blob
            } else {
                format!("0x{:?}", HexDisplay::from(&raw_wasm_blob)).into_bytes()
            };

            if let Some(output) = &params.output {
                std::fs::write(output, output_buf)?;
            } else {
                std::io::stdout().write_all(&output_buf)?;
            }

            Ok(())
        }
        Some(Subcommand::Benchmark(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            // Switch on the concrete benchmark sub-command-
            match cmd {
                BenchmarkCmd::Pallet(cmd) => {
                    if cfg!(feature = "runtime-benchmarks") {
                        runner.sync_run(|config| {
                            cmd.run_with_spec::<sp_runtime::traits::HashingFor<Block>, ReclaimHostFunctions>(Some(
                                config.chain_spec,
                            ))
                        })
                    } else {
                        Err("Benchmarking wasn't enabled when building the node. \
			  You can enable it with `--features runtime-benchmarks`."
                            .into())
                    }
                }
                BenchmarkCmd::Block(cmd) => runner.sync_run(|config| {
                    let client = NodeConfig::new_builder(&config, None)?.client;
                    cmd.run(client)
                }),
                #[cfg(not(feature = "runtime-benchmarks"))]
                BenchmarkCmd::Storage(_) => Err(sc_cli::Error::Input(
                    "Compile with --features=runtime-benchmarks \
						to enable storage benchmarks."
                        .into(),
                )),
                #[cfg(feature = "runtime-benchmarks")]
                BenchmarkCmd::Storage(cmd) => runner.sync_run(|config| {
                    let builder = NodeConfig::new_builder(&config, None)?;
                    let db = builder.backend.expose_db();
                    let storage = builder.backend.expose_storage();
                    cmd.run(config, builder.client, db, storage)
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
        Some(Subcommand::Key(cmd)) => Ok(cmd.run(&cli)?),
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
        Some(Subcommand::SoloChain(cmd)) => {
            // Cannot use create_configuration function because that needs a chain spec.
            // So write our own `create_runner` function that doesn't need chain spec.
            let container_chain_cli = cmd.run.normalize();
            let runner = solochain::create_runner(&container_chain_cli)?;

            // The expected usage is
            // `tanssi-node solochain --flag`
            // So `cmd` stores the flags from after `solochain`, and `cli` has the flags from between
            // `tanssi-node` and `solo-chain`. We are ignoring the flags from `cli` intentionally.
            // Would be nice to error if the user passes any flag there, but it's not easy to detect.

            // Zombienet appends a --chain flag after "solo-chain" subcommand, which is ignored, so it's fine,
            // but warn users that this is not expected here.
            // We cannot do this before create_runner because logging is not setup there yet.
            if container_chain_cli.base.base.shared_params.chain.is_some() {
                log::warn!(
                    "Ignoring --chain argument: solochain mode does only need the relay chain-spec"
                );
            }

            let collator_options = container_chain_cli.base.collator_options();

            runner.run_node_until_exit(|config| async move {
                let containers_base_path = container_chain_cli
                    .base
                    .base
                    .shared_params
                    .base_path
                    .as_ref()
                    .expect("base_path is always set");
                let hwbench = (!cmd.no_hardware_benchmarks)
                    .then(|| {
                        Some(containers_base_path).map(|database_path| {
                            let _ = std::fs::create_dir_all(database_path);
                            sc_sysinfo::gather_hwbench(
                                Some(database_path),
                                &SUBSTRATE_REFERENCE_HARDWARE,
                            )
                        })
                    })
                    .flatten();

                let polkadot_cli = solochain::relay_chain_cli_new(
                    &config,
                    [RelayChainCli::executable_name()]
                        .iter()
                        .chain(cmd.relay_chain_args.iter()),
                );
                let tokio_handle = config.tokio_handle.clone();
                let polkadot_config =
                    SubstrateCli::create_configuration(&polkadot_cli, &polkadot_cli, tokio_handle)
                        .map_err(|err| format!("Relay chain argument error: {}", err))?;

                info!(
                    "Is collating: {}",
                    if config.role.is_authority() {
                        "yes"
                    } else {
                        "no"
                    }
                );

                crate::service::start_solochain_node(
                    polkadot_config,
                    container_chain_cli,
                    collator_options,
                    hwbench,
                )
                    .await
                    .map_err(Into::into)
            })
        }
        None => {
            let runner = cli.create_runner(&cli.run.normalize())?;
            let collator_options = cli.run.collator_options();

            runner.run_node_until_exit(|config| async move {
                let hwbench = (!cli.no_hardware_benchmarks).then(||
                    config.database.path().map(|database_path| {
                        let _ = std::fs::create_dir_all(database_path);
                        sc_sysinfo::gather_hwbench(Some(database_path), &SUBSTRATE_REFERENCE_HARDWARE)
                    })).flatten();

                let para_id = chain_spec::Extensions::try_get(&*config.chain_spec)
                    .map(|e| e.para_id)
                    .ok_or("Could not find parachain ID in chain-spec.")?;

                let id = ParaId::from(para_id);

                let polkadot_cli = RelayChainCli::new(
                    &config,
                    [RelayChainCli::executable_name()].iter().chain(cli.relaychain_args().iter()),
                );

                let extension = chain_spec::Extensions::try_get(&*config.chain_spec);

                let relay_chain_id = extension.map(|e| e.relay_chain.clone());

                let dev_service =
                    config.chain_spec.is_dev() || relay_chain_id == Some("dev-service".to_string()) || cli.run.dev_service;

                if dev_service {
                    return crate::service::start_dev_node(config, cli.run.sealing, hwbench, id).map_err(Into::into);
                }

                let tokio_handle = config.tokio_handle.clone();
                let polkadot_config =
                    SubstrateCli::create_configuration(&polkadot_cli, &polkadot_cli, tokio_handle)
                        .map_err(|err| format!("Relay chain argument error: {}", err))?;

                let parachain_account =
                    AccountIdConversion::<polkadot_primitives::AccountId>::into_account_truncating(&id);

                let block: Block = generate_genesis_block(&*config.chain_spec, sp_runtime::StateVersion::V1)
                    .map_err(|e| format!("{:?}", e))?;
                let genesis_state = format!("0x{:?}", HexDisplay::from(&block.header().encode()));

                info!("Parachain id: {:?}", id);
                info!("Parachain Account: {}", parachain_account);
                info!("Parachain genesis state: {}", genesis_state);
                info!("Is collating: {}", if config.role.is_authority() { "yes" } else { "no" });

                if let cumulus_client_cli::RelayChainMode::ExternalRpc(rpc_target_urls) =
                    collator_options.clone().relay_chain_mode {
                    if !rpc_target_urls.is_empty() && !cli.relaychain_args().is_empty() {
                        warn!("Detected relay chain node arguments together with --relay-chain-rpc-url. This command starts a minimal Polkadot node that only uses a network-related subset of all relay chain CLI options.");
                    }
                }

                let mut container_chain_config = None;
                // Even if container-chain-args are empty, we need to spawn the container-detection
                // collation taks if the role is authority.

                // We need to bake in some container-chain args
                if !cli.container_chain_args().is_empty() || config.role.is_authority() {
                    let container_chain_cli = ContainerChainCli::new(
                        &config,
                        [ContainerChainCli::executable_name()].iter().chain(cli.container_chain_args().iter()),
                    );
                    let tokio_handle = config.tokio_handle.clone();
                    container_chain_config = Some((container_chain_cli, tokio_handle));
                }

                crate::service::start_parachain_node(
                    config,
                    polkadot_config,
                    container_chain_config,
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

    fn rpc_addr(&self, default_listen_port: u16) -> Result<Option<Vec<sc_cli::RpcEndpoint>>> {
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

    fn init<F>(&self, _support_url: &String, _impl_version: &String, _logger_hook: F) -> Result<()>
    where
        F: FnOnce(&mut sc_cli::LoggerBuilder),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_impl_version() {
        // Impl version depends on version in Cargo.toml
        // This is to verify we didn't forget to change one of them
        let v1 = ContainerChainCli::impl_version();
        let v2 = Cli::impl_version();

        assert_eq!(v1, v2);
    }
}
