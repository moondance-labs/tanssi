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

use {
    crate::cli::{Cli, Subcommand, NODE_VERSION},
    frame_benchmarking_cli::{BenchmarkCmd, ExtrinsicFactory, SUBSTRATE_REFERENCE_HARDWARE},
    futures::future::TryFutureExt,
    node_common::service::Sealing,
    polkadot_service::{
        self,
        benchmarking::{benchmark_inherent_data, RemarkBuilder, TransferKeepAliveBuilder},
        HeaderBackend, IdentifyVariant, ParaId,
    },
    sc_cli::{CliConfiguration, SubstrateCli},
    sp_core::crypto::Ss58AddressFormatRegistry,
    sp_keyring::Sr25519Keyring,
    std::net::ToSocketAddrs,
    tanssi_relay_service::dev_service::build_full as build_full_dev,
};

pub use crate::error::Error;

#[cfg(feature = "pyroscope")]
use pyroscope_pprofrs::{pprof_backend, PprofConfig};

type Result<T> = std::result::Result<T, Error>;

fn get_exec_name() -> Option<String> {
    std::env::current_exe()
        .ok()
        .and_then(|pb| pb.file_name().map(|s| s.to_os_string()))
        .and_then(|s| s.into_string().ok())
}

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "Tanssi".into()
    }

    fn impl_version() -> String {
        let commit_hash = env!("SUBSTRATE_CLI_COMMIT_HASH");
        format!("{}-{commit_hash}", NODE_VERSION)
    }

    fn description() -> String {
        env!("CARGO_PKG_DESCRIPTION").into()
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://github.com/moondance-labs/tanssi/issues/new".into()
    }

    fn copyright_start_year() -> i32 {
        2017
    }

    fn executable_name() -> String {
        "tanssi".into()
    }

    #[cfg(not(feature = "runtime-benchmarks"))]
    fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
        load_spec(id, vec![], vec![2000, 2001], None)
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
        load_spec(id, vec![], vec![], None)
    }
}

#[allow(clippy::borrowed_box)]
fn set_default_ss58_version(_spec: &Box<dyn polkadot_service::ChainSpec>) {
    let ss58_version = Ss58AddressFormatRegistry::PolkadotAccount.into();

    sp_core::crypto::set_default_ss58_version(ss58_version);
}

/// Launch a node, accepting arguments just like a regular node,
/// accepts an alternative overseer generator, to adjust behavior
/// for integration tests as needed.
/// `malus_finality_delay` restrict finality votes of this node
/// to be at most `best_block - malus_finality_delay` height.
#[cfg(feature = "malus")]
pub fn run_node(
    run: Cli,
    overseer_gen: impl polkadot_service::OverseerGen,
    malus_finality_delay: Option<u32>,
) -> Result<()> {
    run_node_inner(
        run,
        overseer_gen,
        malus_finality_delay,
        |_logger_builder, _config| {},
    )
}

fn run_node_inner<F>(
    cli: Cli,
    overseer_gen: impl polkadot_service::OverseerGen,
    maybe_malus_finality_delay: Option<u32>,
    logger_hook: F,
) -> Result<()>
where
    F: FnOnce(&mut sc_cli::LoggerBuilder, &sc_service::Configuration),
{
    let runner = cli
        .create_runner_with_logger_hook::<sc_cli::RunCmd, _, F>(&cli.run.base, logger_hook)
        .map_err(Error::from)?;
    let chain_spec = &runner.config().chain_spec;

    // By default, enable BEEFY on all networks, unless explicitly disabled through CLI.
    let enable_beefy = !cli.run.no_beefy;

    set_default_ss58_version(chain_spec);

    let jaeger_agent = if let Some(ref jaeger_agent) = cli.run.jaeger_agent {
        Some(
            jaeger_agent
                .to_socket_addrs()
                .map_err(Error::AddressResolutionFailure)?
                .next()
                .ok_or_else(|| Error::AddressResolutionMissing)?,
        )
    } else {
        None
    };

    let node_version = if cli.run.disable_worker_version_check {
        None
    } else {
        Some(NODE_VERSION.to_string())
    };

    let secure_validator_mode = cli.run.base.validator && !cli.run.insecure_validator;

    runner.run_node_until_exit(move |config| async move {
        let hwbench = (!cli.run.no_hardware_benchmarks)
            .then(|| {
                config.database.path().map(|database_path| {
                    let _ = std::fs::create_dir_all(database_path);
                    sc_sysinfo::gather_hwbench(Some(database_path))
                })
            })
            .flatten();

        let database_source = config.database.clone();

        let task_manager = if config.chain_spec.is_dev() || cli.run.dev_service {
            log::info!("Starting service in Development mode");
            build_full_dev(
                Sealing::Manual,
                config,
                polkadot_service::NewFullParams {
                    is_parachain_node: polkadot_service::IsParachainNode::No,
                    enable_beefy,
                    force_authoring_backoff: cli.run.force_authoring_backoff,
                    jaeger_agent,
                    telemetry_worker_handle: None,
                    node_version,
                    secure_validator_mode,
                    workers_path: cli.run.workers_path,
                    workers_names: Some((
                        (&"tanssi-relay-prepare-worker").to_string(),
                        (&"tanssi-relay-execute-worker").to_string(),
                    )),
                    overseer_gen,
                    overseer_message_channel_capacity_override: cli
                        .run
                        .overseer_channel_capacity_override,
                    malus_finality_delay: maybe_malus_finality_delay,
                    hwbench,
                    execute_workers_max_num: cli.run.execute_workers_max_num,
                    prepare_workers_hard_max_num: cli.run.prepare_workers_hard_max_num,
                    prepare_workers_soft_max_num: cli.run.prepare_workers_soft_max_num,
                },
            )
            .map(|full| full.task_manager)?
        } else {
            polkadot_service::build_full(
                config,
                polkadot_service::NewFullParams {
                    is_parachain_node: polkadot_service::IsParachainNode::No,
                    enable_beefy,
                    force_authoring_backoff: cli.run.force_authoring_backoff,
                    jaeger_agent,
                    telemetry_worker_handle: None,
                    node_version,
                    secure_validator_mode,
                    workers_path: cli.run.workers_path,
                    workers_names: Some((
                        (&"tanssi-relay-prepare-worker").to_string(),
                        (&"tanssi-relay-execute-worker").to_string(),
                    )),
                    overseer_gen,
                    overseer_message_channel_capacity_override: cli
                        .run
                        .overseer_channel_capacity_override,
                    malus_finality_delay: maybe_malus_finality_delay,
                    hwbench,
                    execute_workers_max_num: cli.run.execute_workers_max_num,
                    prepare_workers_hard_max_num: cli.run.prepare_workers_hard_max_num,
                    prepare_workers_soft_max_num: cli.run.prepare_workers_soft_max_num,
                },
            )
            .map(|full| full.task_manager)?
        };

        if let Some(path) = database_source.path() {
            sc_storage_monitor::StorageMonitorService::try_spawn(
                cli.storage_monitor,
                path.to_path_buf(),
                &task_manager.spawn_essential_handle(),
            )?;
        }

        Ok(task_manager)
    })
}

/// Parses polkadot specific CLI arguments and run the service.
pub fn run() -> Result<()> {
    let cli: Cli = Cli::from_args();

    #[cfg(feature = "pyroscope")]
    let mut pyroscope_agent_maybe = if let Some(ref agent_addr) = cli.run.pyroscope_server {
        let address = agent_addr
            .to_socket_addrs()
            .map_err(Error::AddressResolutionFailure)?
            .next()
            .ok_or_else(|| Error::AddressResolutionMissing)?;
        // The pyroscope agent requires a `http://` prefix, so we just do that.
        let agent = pyro::PyroscopeAgent::builder(
            "http://".to_owned() + address.to_string().as_str(),
            "polkadot".to_owned(),
        )
        .backend(pprof_backend(PprofConfig::new().sample_rate(113)))
        .build()?;
        Some(agent.start()?)
    } else {
        None
    };

    #[cfg(not(feature = "pyroscope"))]
    if cli.run.pyroscope_server.is_some() {
        return Err(Error::PyroscopeNotCompiledIn);
    }

    match &cli.subcommand {
        None => run_node_inner(
            cli,
            polkadot_service::ValidatorOverseerGen,
            None,
            polkadot_node_metrics::logger_hook(),
        ),
        Some(Subcommand::BuildSpec(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            let chain_spec = load_spec(
                &cmd.base.chain_id(cmd.base.is_dev()?)?,
                cmd.add_container_chain.clone().unwrap_or_default(),
                cmd.mock_container_chain.clone().unwrap_or_default(),
                cmd.invulnerable.clone(),
            )?;
            Ok(runner.sync_run(|config| cmd.base.run(chain_spec, config.network))?)
        }
        Some(Subcommand::CheckBlock(cmd)) => {
            let runner = cli.create_runner(cmd).map_err(Error::SubstrateCli)?;
            let chain_spec = &runner.config().chain_spec;

            set_default_ss58_version(chain_spec);

            runner.async_run(|mut config| {
                let (client, _, import_queue, task_manager) =
                    polkadot_service::new_chain_ops(&mut config, None)?;
                Ok((
                    cmd.run(client, import_queue).map_err(Error::SubstrateCli),
                    task_manager,
                ))
            })
        }
        Some(Subcommand::ExportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            let chain_spec = &runner.config().chain_spec;

            set_default_ss58_version(chain_spec);

            Ok(runner.async_run(|mut config| {
                let (client, _, _, task_manager) =
                    polkadot_service::new_chain_ops(&mut config, None)
                        .map_err(Error::PolkadotService)?;
                Ok((
                    cmd.run(client, config.database)
                        .map_err(Error::SubstrateCli),
                    task_manager,
                ))
            })?)
        }
        Some(Subcommand::ExportState(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            let chain_spec = &runner.config().chain_spec;

            set_default_ss58_version(chain_spec);

            Ok(runner.async_run(|mut config| {
                let (client, _, _, task_manager) =
                    polkadot_service::new_chain_ops(&mut config, None)?;
                Ok((
                    cmd.run(client, config.chain_spec)
                        .map_err(Error::SubstrateCli),
                    task_manager,
                ))
            })?)
        }
        Some(Subcommand::ImportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            let chain_spec = &runner.config().chain_spec;

            set_default_ss58_version(chain_spec);

            Ok(runner.async_run(|mut config| {
                let (client, _, import_queue, task_manager) =
                    polkadot_service::new_chain_ops(&mut config, None)?;
                Ok((
                    cmd.run(client, import_queue).map_err(Error::SubstrateCli),
                    task_manager,
                ))
            })?)
        }
        Some(Subcommand::PurgeChain(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            Ok(runner.sync_run(|config| cmd.run(config.database))?)
        }
        Some(Subcommand::Revert(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            let chain_spec = &runner.config().chain_spec;

            set_default_ss58_version(chain_spec);

            Ok(runner.async_run(|mut config| {
                let (client, backend, _, task_manager) =
                    polkadot_service::new_chain_ops(&mut config, None)?;
                let aux_revert = Box::new(|client, backend, blocks| {
                    polkadot_service::revert_backend(client, backend, blocks, config).map_err(
                        |err| {
                            match err {
                                polkadot_service::Error::Blockchain(err) => err.into(),
                                // Generic application-specific error.
                                err => sc_cli::Error::Application(err.into()),
                            }
                        },
                    )
                });
                Ok((
                    cmd.run(client, backend, Some(aux_revert))
                        .map_err(Error::SubstrateCli),
                    task_manager,
                ))
            })?)
        }
        Some(Subcommand::Benchmark(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            let chain_spec = &runner.config().chain_spec;

            match cmd {
                #[cfg(not(feature = "runtime-benchmarks"))]
                BenchmarkCmd::Storage(_) => {
                    return Err(sc_cli::Error::Input(
                        "Compile with --features=runtime-benchmarks \
						to enable storage benchmarks."
                            .into(),
                    )
                    .into())
                }
                #[cfg(feature = "runtime-benchmarks")]
                BenchmarkCmd::Storage(cmd) => runner.sync_run(|mut config| {
                    let (client, backend, _, _) =
                        polkadot_service::new_chain_ops(&mut config, None)?;
                    let db = backend.expose_db();
                    let storage = backend.expose_storage();

                    cmd.run(config, client.clone(), db, storage)
                        .map_err(Error::SubstrateCli)
                }),
                BenchmarkCmd::Block(cmd) => runner.sync_run(|mut config| {
                    let (client, _, _, _) = polkadot_service::new_chain_ops(&mut config, None)?;

                    cmd.run(client.clone()).map_err(Error::SubstrateCli)
                }),
                // These commands are very similar and can be handled in nearly the same way.
                BenchmarkCmd::Extrinsic(_) | BenchmarkCmd::Overhead(_) => {
                    runner.sync_run(|mut config| {
                        let (client, _, _, _) = polkadot_service::new_chain_ops(&mut config, None)?;
                        let header = client.header(client.info().genesis_hash).unwrap().unwrap();
                        let inherent_data = benchmark_inherent_data(header)
                            .map_err(|e| format!("generating inherent data: {:?}", e))?;
                        let remark_builder =
                            RemarkBuilder::new(client.clone(), config.chain_spec.identify_chain());

                        match cmd {
                            BenchmarkCmd::Extrinsic(cmd) => {
                                let tka_builder = TransferKeepAliveBuilder::new(
                                    client.clone(),
                                    Sr25519Keyring::Alice.to_account_id(),
                                    config.chain_spec.identify_chain(),
                                );

                                let ext_factory = ExtrinsicFactory(vec![
                                    Box::new(remark_builder),
                                    Box::new(tka_builder),
                                ]);

                                cmd.run(client.clone(), inherent_data, Vec::new(), &ext_factory)
                                    .map_err(Error::SubstrateCli)
                            }
                            BenchmarkCmd::Overhead(cmd) => cmd
                                .run(
                                    config,
                                    client.clone(),
                                    inherent_data,
                                    Vec::new(),
                                    &remark_builder,
                                )
                                .map_err(Error::SubstrateCli),
                            _ => unreachable!("Ensured by the outside match; qed"),
                        }
                    })
                }
                BenchmarkCmd::Pallet(cmd) => {
                    set_default_ss58_version(chain_spec);

                    if cfg!(feature = "runtime-benchmarks") {
                        runner.sync_run(|config| {
                            cmd.run_with_spec::<sp_runtime::traits::HashingFor<polkadot_service::Block>, ()>(
                                Some(config.chain_spec),
                            )
                            .map_err(Error::SubstrateCli)
                        })
                    } else {
                        Err(sc_cli::Error::Input(
                            "Benchmarking wasn't enabled when building the node. \
				You can enable it with `--features runtime-benchmarks`."
                                .into(),
                        )
                        .into())
                    }
                }
                BenchmarkCmd::Machine(cmd) => runner.sync_run(|config| {
                    cmd.run(&config, SUBSTRATE_REFERENCE_HARDWARE.clone())
                        .map_err(Error::SubstrateCli)
                }),
                // NOTE: this allows the Polkadot client to leniently implement
                // new benchmark commands.
                #[allow(unreachable_patterns)]
                _ => Err(Error::CommandNotImplemented),
            }
        }
        Some(Subcommand::Key(cmd)) => Ok(cmd.run(&cli)?),
        Some(Subcommand::PrecompileWasm(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            Ok(runner.async_run(|mut config| {
                let (_, backend, _, task_manager) =
                    polkadot_service::new_chain_ops(&mut config, None)?;
                Ok((
                    cmd.run(backend, config.chain_spec)
                        .map_err(Error::SubstrateCli),
                    task_manager,
                ))
            })?)
        }
        Some(Subcommand::ChainInfo(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            Ok(runner.sync_run(|config| cmd.run::<polkadot_service::Block>(&config))?)
        }
    }?;

    #[cfg(feature = "pyroscope")]
    if let Some(pyroscope_agent) = pyroscope_agent_maybe.take() {
        let agent = pyroscope_agent.stop()?;
        agent.shutdown();
    }
    Ok(())
}

fn load_spec(
    id: &str,
    container_chains: Vec<String>,
    mock_container_chains: Vec<u32>,
    invulnerables: Option<Vec<String>>,
) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
    let id = if id.is_empty() {
        let n = get_exec_name().unwrap_or_default();
        ["dancelight"]
            .iter()
            .cloned()
            .find(|&chain| n.starts_with(chain))
            .unwrap_or("dancelight")
    } else {
        id
    };
    let mock_container_chains: Vec<ParaId> =
        mock_container_chains.iter().map(|&x| x.into()).collect();
    Ok(match id {
        #[cfg(feature = "dancelight-native")]
        "dancelight" => Box::new(tanssi_relay_service::chain_spec::dancelight_config()?),
        #[cfg(feature = "dancelight-native")]
        "dev" | "dancelight-dev" => {
            // Default invulnerables for dev mode, used in dev_tanssi_relay tests
            let invulnerables = invulnerables.unwrap_or(vec![
                "Bob".to_string(),
                "Charlie".to_string(),
                "Dave".to_string(),
                "Eve".to_string(),
            ]);

            Box::new(
                tanssi_relay_service::chain_spec::dancelight_development_config(
                    container_chains,
                    mock_container_chains,
                    invulnerables,
                )?,
            )
        }
        #[cfg(feature = "dancelight-native")]
        "dancelight-local" => {
            let invulnerables = invulnerables.unwrap_or_default();

            Box::new(
                tanssi_relay_service::chain_spec::dancelight_local_testnet_config(
                    container_chains,
                    mock_container_chains,
                    invulnerables,
                )?,
            )
        }
        #[cfg(feature = "dancelight-native")]
        "dancelight-staging" => {
            Box::new(tanssi_relay_service::chain_spec::dancelight_staging_testnet_config()?)
        }
        #[cfg(not(feature = "dancelight-native"))]
        name if name.starts_with("dancelight-") && !name.ends_with(".json") || name == "dev" => {
            Err(format!(
                "`{}` only supported with `dancelight-native` feature enabled.",
                name
            ))?
        }
        path => {
            let path = std::path::PathBuf::from(path);

            (Box::new(polkadot_service::GenericChainSpec::from_json_file(
                path.clone(),
            )?)) as std::boxed::Box<dyn sc_cli::ChainSpec>
        }
    })
}
