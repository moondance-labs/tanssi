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
        cli::{BaseSubcommand, Cli, Subcommand},
        service::{self, frontier_database_dir, NodeConfig},
    },
    clap::Parser,
    container_chain_template_frontier_runtime::Block,
    core::marker::PhantomData,
    cumulus_client_service::storage_proof_size::HostFunctions as ReclaimHostFunctions,
    cumulus_primitives_core::ParaId,
    frame_benchmarking_cli::{BenchmarkCmd, SUBSTRATE_REFERENCE_HARDWARE},
    log::{info, warn},
    node_common::{
        chain_spec as node_common_chain_spec, cli::ContainerNodeRelayChainCli,
        command::generate_genesis_block, service::node_builder::NodeBuilderConfig as _,
    },
    parity_scale_codec::Encode,
    polkadot_cli::IdentifyVariant,
    sc_cli::{ChainSpec, Result, SubstrateCli},
    sc_service::DatabaseSource,
    sp_core::hexdisplay::HexDisplay,
    sp_runtime::traits::{AccountIdConversion, Block as BlockT, Get},
    tc_service_container_chain_rpc_provider::RpcProviderMode,
    tc_service_container_chain_spawner::cli::ContainerChainCli,
};

pub struct NodeName;

impl Get<&'static str> for NodeName {
    fn get() -> &'static str {
        "Frontier"
    }
}

fn load_spec(id: &str, para_id: ParaId) -> std::result::Result<Box<dyn ChainSpec>, String> {
    Ok(match id {
        "dev" => Box::new(chain_spec::development_config(para_id, vec![])),
        "template-rococo" => Box::new(chain_spec::local_testnet_config(para_id, vec![])),
        "" | "local" => Box::new(chain_spec::local_testnet_config(para_id, vec![])),

        // dummy container chain spec, it will not be used to actually spawn a chain
        "container-chain-unknown" => Box::new(
            sc_service::GenericChainSpec::<node_common_chain_spec::Extensions, ()>::builder(
                b"",
                node_common_chain_spec::Extensions {
                    relay_chain: "westend-local".into(),
                    para_id: 2000,
                },
            )
            .build(),
        ),

        path => Box::new(chain_spec::ChainSpec::from_json_file(
            std::path::PathBuf::from(path),
        )?),
    })
}

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "Container Chain Frontier Node".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        format!(
            "Container Chain Frontier Node\n\nThe command-line arguments provided first will be \
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

macro_rules! construct_async_run {
	(|$components:ident, $cli:ident, $cmd:ident, $config:ident| $( $code:tt )* ) => {{
		let runner = $cli.create_runner($cmd)?;
		runner.async_run(|mut $config| {
			let $components = NodeConfig::new_builder(&mut $config, None)?;
			let inner = { $( $code )* };

            let task_manager = $components.task_manager;
			inner.map(|v| (v, task_manager))
		})
	}}
}

/// Parse command line arguments into service configuration.
pub fn run() -> Result<()> {
    let cli = Cli::from_args();

    // Match rpc provider subcommand in wrapper
    let subcommand = match &cli.subcommand {
        Some(Subcommand::RpcProvider(cmd)) => {
            return rpc_provider_mode(&cli, cmd);
        }
        Some(Subcommand::Base(cmd)) => Some(cmd),
        None => None,
    };

    match subcommand {
        Some(BaseSubcommand::BuildSpec(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| {
                let chain_spec = if let Some(para_id) = cmd.extra.parachain_id {
                    if cmd.base.shared_params.dev {
                        Box::new(chain_spec::development_config(
                            para_id.into(),
                            cmd.extra.add_bootnode.clone(),
                        ))
                    } else {
                        Box::new(chain_spec::local_testnet_config(
                            para_id.into(),
                            cmd.extra.add_bootnode.clone(),
                        ))
                    }
                } else {
                    config.chain_spec
                };
                cmd.base.run(chain_spec, config.network)
            })
        }
        Some(BaseSubcommand::CheckBlock(cmd)) => {
            construct_async_run!(|components, cli, cmd, config| {
                let (_, import_queue) = service::import_queue(&config, &components);
                Ok(cmd.run(components.client, import_queue))
            })
        }
        Some(BaseSubcommand::ExportBlocks(cmd)) => {
            construct_async_run!(|components, cli, cmd, config| {
                Ok(cmd.run(components.client, config.database))
            })
        }
        Some(BaseSubcommand::ExportState(cmd)) => {
            construct_async_run!(|components, cli, cmd, config| {
                Ok(cmd.run(components.client, config.chain_spec))
            })
        }
        Some(BaseSubcommand::ImportBlocks(cmd)) => {
            construct_async_run!(|components, cli, cmd, config| {
                let (_, import_queue) = service::import_queue(&config, &components);
                Ok(cmd.run(components.client, import_queue))
            })
        }
        Some(BaseSubcommand::Revert(cmd)) => {
            construct_async_run!(|components, cli, cmd, config| {
                Ok(cmd.run(components.client, components.backend, None))
            })
        }
        Some(BaseSubcommand::PurgeChain(cmd)) => {
            let runner = cli.create_runner(cmd)?;

            runner.sync_run(|config| {
                // Remove Frontier offchain db
                let frontier_database_config = match config.database {
                    DatabaseSource::RocksDb { .. } => DatabaseSource::RocksDb {
                        path: frontier_database_dir(&config, "db"),
                        cache_size: 0,
                    },
                    DatabaseSource::ParityDb { .. } => DatabaseSource::ParityDb {
                        path: frontier_database_dir(&config, "paritydb"),
                    },
                    _ => {
                        return Err(format!("Cannot purge `{:?}` database", config.database).into())
                    }
                };

                cmd.base.run(frontier_database_config)?;

                let polkadot_cli = ContainerNodeRelayChainCli::<NodeName>::new(
                    &config,
                    [ContainerNodeRelayChainCli::<NodeName>::executable_name()]
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
        Some(BaseSubcommand::ExportGenesisHead(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| {
                let partials = NodeConfig::new_builder(&config, None)?;
                cmd.run(partials.client)
            })
        }
        Some(BaseSubcommand::ExportGenesisWasm(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|_config| {
                let spec = cli.load_spec(&cmd.shared_params.chain.clone().unwrap_or_default())?;
                cmd.run(&*spec)
            })
        }
        Some(BaseSubcommand::Benchmark(cmd)) => {
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
        Some(BaseSubcommand::PrecompileWasm(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let partials = NodeConfig::new_builder(&config, None)?;
                Ok((
                    cmd.run(partials.backend, config.chain_spec),
                    partials.task_manager,
                ))
            })
        }
        None => {
            let runner = cli.create_runner(&cli.run.normalize())?;
            let collator_options = cli.run.collator_options();

            runner.run_node_until_exit(|config| async move {
                let relaychain_args = cli.relaychain_args();
                let hwbench = (!cli.no_hardware_benchmarks).then(||
                    config.database.path().map(|database_path| {
                        let _ = std::fs::create_dir_all(database_path);
                        sc_sysinfo::gather_hwbench(Some(database_path), &SUBSTRATE_REFERENCE_HARDWARE)
                    })).flatten();

                let para_id = node_common_chain_spec::Extensions::try_get(&*config.chain_spec)
                    .map(|e| e.para_id)
                    .ok_or("Could not find parachain ID in chain-spec.")?;

                let polkadot_cli = ContainerNodeRelayChainCli::<NodeName>::new(
                    &config,
                    [ContainerNodeRelayChainCli::<NodeName>::executable_name()].iter().chain(relaychain_args.iter()),
                );

                let rpc_config = crate::cli::RpcConfig {
                    eth_log_block_cache: cli.run.eth.eth_log_block_cache,
                    eth_statuses_cache: cli.run.eth.eth_statuses_cache,
                    fee_history_limit: cli.run.eth.fee_history_limit,
                    max_past_logs: cli.run.eth.max_past_logs,
                    max_block_range: cli.run.eth.max_block_range,
                };

                let extension = node_common_chain_spec::Extensions::try_get(&*config.chain_spec);

                let relay_chain_id = extension.map(|e| e.relay_chain.clone());

                let dev_service =
                    config.chain_spec.is_dev() || relay_chain_id == Some("dev-service".to_string());

                let id = ParaId::from(para_id);

                if dev_service {
                    return crate::service::start_dev_node(config, cli.run.sealing, rpc_config, id, hwbench).await
                        .map_err(Into::into);
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
                    if !rpc_target_urls.is_empty() && !relaychain_args.is_empty() {
                        warn!("Detected relay chain node arguments together with --relay-chain-rpc-url. This command starts a minimal Polkadot node that only uses a network-related subset of all relay chain CLI options.");
                    }
                }

                match config.network.network_backend.unwrap_or(sc_network::config::NetworkBackendType::Libp2p) {
                    sc_network::config::NetworkBackendType::Libp2p => {
                        crate::service::start_parachain_node::<sc_network::NetworkWorker<_, _>>(
                            config,
                            polkadot_config,
                            collator_options,
                            id,
                            rpc_config,
                            hwbench,
                        )
                            .await
                            .map(|r| r.0)
                            .map_err(Into::into)
                    }
                    sc_network::config::NetworkBackendType::Litep2p => {
                        crate::service::start_parachain_node::<sc_network::Litep2pNetworkBackend>(
                            config,
                            polkadot_config,
                            collator_options,
                            id,
                            rpc_config,
                            hwbench,
                        )
                            .await
                            .map(|r| r.0)
                            .map_err(Into::into)

                    }
                }
            })
        }
    }
}

fn rpc_provider_mode(cli: &Cli, cmd: &crate::cli::RpcProviderCmd) -> Result<()> {
    let runner = cli.create_runner(&cmd.base.container_run.normalize())?;

    runner.run_node_until_exit(|config| async move {
        info!("Starting in RPC provider mode!");

        let container_chain_cli = ContainerChainCli {
            base: cmd.base.container_run.clone(),
            preloaded_chain_spec: None,
        };

        let polkadot_cli = ContainerNodeRelayChainCli::<NodeName>::new(
            &config,
            [ContainerNodeRelayChainCli::<NodeName>::executable_name()]
                .iter()
                .chain(cmd.base.relaychain_args().iter()),
        );

        let mut orchestrator_cli = None;
        if !cmd.base.solochain {
            orchestrator_cli = Some(cumulus_client_cli::RunCmd::parse_from(
                [String::from("orchestrator")]
                    .iter()
                    .chain(cmd.base.orchestrator_chain_args().iter()),
            ));
        }

        let rpc_config = crate::cli::RpcConfig {
            eth_log_block_cache: cmd.eth.eth_log_block_cache,
            eth_statuses_cache: cmd.eth.eth_statuses_cache,
            fee_history_limit: cmd.eth.fee_history_limit,
            max_past_logs: cmd.eth.max_past_logs,
            max_block_range: cmd.eth.max_block_range,
        };

        let generate_rpc_builder = crate::rpc::GenerateFrontierRpcBuilder::<
            container_chain_template_frontier_runtime::RuntimeApi,
        >::new(rpc_config);

        RpcProviderMode {
            config,
            provider_profile_id: cmd.base.profile_id,
            orchestrator_endpoints: cmd.base.orchestrator_endpoints.clone(),
            collator_options: cmd.base.container_run.collator_options(),
            polkadot_cli,
            orchestrator_cli,
            container_chain_cli,
            generate_rpc_builder,
            phantom: PhantomData,
        }
        .run()
        .await
    })
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
