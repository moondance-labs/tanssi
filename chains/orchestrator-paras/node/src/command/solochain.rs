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

//! Helper functions used to implement solochain collator

use {
    crate::cli::{Cli, RelayChainCli},
    futures::FutureExt,
    jsonrpsee::server::BatchRequestConfig,
    log::{info, warn},
    sc_chain_spec::{ChainType, GenericChainSpec, NoExtension},
    sc_cli::{CliConfiguration, DefaultConfigurationValues, Signals, SubstrateCli},
    sc_network::config::{NetworkBackendType, NetworkConfiguration, TransportConfig},
    sc_network_common::role::Role,
    sc_service::{
        config::{ExecutorConfiguration, KeystoreConfig},
        BasePath, BlocksPruning, Configuration, DatabaseSource, TaskManager,
    },
    sc_tracing::logging::LoggerBuilder,
    std::{
        future::Future,
        num::NonZeroUsize,
        path::{Path, PathBuf},
        time::Duration,
    },
    tc_service_container_chain::cli::ContainerChainCli,
};

/// Alternative to [Configuration] struct used in solochain context.
pub struct SolochainConfig {
    pub tokio_handle: tokio::runtime::Handle,
    pub base_path: BasePath,
    pub network_node_name: String,
    pub role: Role,
    pub relay_chain: String,
}

/// Alternative to [Runner](sc_cli::Runner) struct used in solochain context.
pub struct SolochainRunner {
    config: SolochainConfig,
    tokio_runtime: tokio::runtime::Runtime,
    signals: Signals,
}

impl SolochainRunner {
    /// Log information about the node itself.
    ///
    /// # Example:
    ///
    /// ```text
    /// 2020-06-03 16:14:21 Substrate Node
    /// 2020-06-03 16:14:21 ✌️  version 2.0.0-rc3-f4940588c-x86_64-linux-gnu
    /// 2020-06-03 16:14:21 ❤️  by Parity Technologies <admin@parity.io>, 2017-2020
    /// 2020-06-03 16:14:21 📋 Chain specification: Flaming Fir
    /// 2020-06-03 16:14:21 🏷  Node name: jolly-rod-7462
    /// 2020-06-03 16:14:21 👤 Role: FULL
    /// 2020-06-03 16:14:21 💾 Database: RocksDb at /tmp/c/chains/flamingfir7/db
    /// 2020-06-03 16:14:21 ⛓  Native runtime: node-251 (substrate-node-1.tx1.au10)
    /// ```
    fn print_node_infos(&self) {
        use chrono::{offset::Local, Datelike};
        type C = ContainerChainCli;
        info!("{}", C::impl_name());
        info!("✌️  version {}", C::impl_version());
        info!(
            "❤️  by {}, {}-{}",
            C::author(),
            C::copyright_start_year(),
            Local::now().year()
        );
        // No chain spec
        //info!("📋 Chain specification: {}", config.chain_spec.name());
        info!("🏷  Node name: {}", self.config.network_node_name);
        info!("👤 Role: {}", self.config.role);
        info!(
            "💾 Database: {} at {}",
            // Container chains only support paritydb
            "ParityDb",
            // Print base path instead of db path because each container will have its own db in a
            // different subdirectory.
            self.config.base_path.path().display(),
        );
    }

    /// A helper function that runs a node with tokio and stops if the process receives the signal
    /// `SIGTERM` or `SIGINT`.
    pub fn run_node_until_exit<F, E>(
        self,
        initialize: impl FnOnce(SolochainConfig) -> F,
    ) -> std::result::Result<(), E>
    where
        F: Future<Output = std::result::Result<TaskManager, E>>,
        E: std::error::Error + Send + Sync + 'static + From<sc_service::Error>,
    {
        self.print_node_infos();

        let mut task_manager = self.tokio_runtime.block_on(initialize(self.config))?;

        let res = self
            .tokio_runtime
            .block_on(self.signals.run_until_signal(task_manager.future().fuse()));
        // We need to drop the task manager here to inform all tasks that they should shut down.
        //
        // This is important to be done before we instruct the tokio runtime to shutdown. Otherwise
        // the tokio runtime will wait the full 60 seconds for all tasks to stop.
        let task_registry = task_manager.into_task_registry();

        // Give all futures 60 seconds to shutdown, before tokio "leaks" them.
        let shutdown_timeout = Duration::from_secs(60);
        self.tokio_runtime.shutdown_timeout(shutdown_timeout);

        let running_tasks = task_registry.running_tasks();

        if !running_tasks.is_empty() {
            log::error!("Detected running(potentially stalled) tasks on shutdown:");
            running_tasks.iter().for_each(|(task, count)| {
                let instances_desc = if *count > 1 {
                    format!("with {} instances ", count)
                } else {
                    "".to_string()
                };

                if task.is_default_group() {
                    log::error!(
                        "Task \"{}\" was still running {}after waiting {} seconds to finish.",
                        task.name,
                        instances_desc,
                        shutdown_timeout.as_secs(),
                    );
                } else {
                    log::error!(
						"Task \"{}\" (Group: {}) was still running {}after waiting {} seconds to finish.",
						task.name,
						task.group,
						instances_desc,
						shutdown_timeout.as_secs(),
					);
                }
            });
        }

        res.map_err(Into::into)
    }
}

/// Equivalent to [Cli::create_runner]
pub fn create_runner<T: CliConfiguration<DVC>, DVC: DefaultConfigurationValues>(
    command: &T,
) -> sc_cli::Result<SolochainRunner> {
    let tokio_runtime = sc_cli::build_runtime()?;

    // `capture` needs to be called in a tokio context.
    // Also capture them as early as possible.
    let signals = tokio_runtime.block_on(async { Signals::capture() })?;

    init_cmd(command, &Cli::support_url(), &Cli::impl_version())?;

    let base_path = command.base_path()?.unwrap();
    let network_node_name = command.node_name()?;
    let is_dev = command.is_dev()?;
    let role = command.role(is_dev)?;
    // This relay chain id is only used when the relay chain args have no `--chain` value
    // TODO: check if this works with an external relay rpc / light client
    let relay_chain_id = "dancelight_local_testnet".to_string();

    let config = SolochainConfig {
        tokio_handle: tokio_runtime.handle().clone(),
        base_path,
        network_node_name,
        role,
        relay_chain: relay_chain_id,
    };

    Ok(SolochainRunner {
        config,
        tokio_runtime,
        signals,
    })
}

/// The recommended open file descriptor limit to be configured for the process.
const RECOMMENDED_OPEN_FILE_DESCRIPTOR_LIMIT: u64 = 10_000;

/// Equivalent to [CliConfiguration::init]
fn init_cmd<T: CliConfiguration<DVC>, DVC: DefaultConfigurationValues>(
    this: &T,
    support_url: &String,
    impl_version: &String,
) -> sc_cli::Result<()> {
    sp_panic_handler::set(support_url, impl_version);

    let mut logger = LoggerBuilder::new(this.log_filters()?);
    logger
        .with_log_reloading(this.enable_log_reloading()?)
        .with_detailed_output(this.detailed_log_output()?);

    if let Some(tracing_targets) = this.tracing_targets()? {
        let tracing_receiver = this.tracing_receiver()?;
        logger.with_profiling(tracing_receiver, tracing_targets);
    }

    if this.disable_log_color()? {
        logger.with_colors(false);
    }

    logger.init()?;

    match fdlimit::raise_fd_limit() {
        Ok(fdlimit::Outcome::LimitRaised { to, .. }) => {
            if to < RECOMMENDED_OPEN_FILE_DESCRIPTOR_LIMIT {
                warn!(
                    "Low open file descriptor limit configured for the process. \
                        Current value: {:?}, recommended value: {:?}.",
                    to, RECOMMENDED_OPEN_FILE_DESCRIPTOR_LIMIT,
                );
            }
        }
        Ok(fdlimit::Outcome::Unsupported) => {
            // Unsupported platform (non-Linux)
        }
        Err(error) => {
            warn!(
                "Failed to configure file descriptor limit for the process: \
                    {}, recommended value: {:?}.",
                error, RECOMMENDED_OPEN_FILE_DESCRIPTOR_LIMIT,
            );
        }
    }

    Ok(())
}

/// Equivalent to [RelayChainCli::new]
pub fn relay_chain_cli_new<'a>(
    config: &SolochainConfig,
    relay_chain_args: impl Iterator<Item = &'a String>,
) -> RelayChainCli {
    let base_path = config.base_path.path().join("polkadot");

    RelayChainCli {
        base_path,
        chain_id: Some(config.relay_chain.clone()),
        base: clap::Parser::parse_from(relay_chain_args),
    }
}

/// Create a dummy [Configuration] that should only be used as input to polkadot-sdk functions that
/// take this struct as input but only use one field of it.
/// This is needed because [Configuration] does not implement [Default].
pub fn dummy_config(tokio_handle: tokio::runtime::Handle, base_path: BasePath) -> Configuration {
    Configuration {
        impl_name: "".to_string(),
        impl_version: "".to_string(),
        role: Role::Full,
        tokio_handle,
        transaction_pool: Default::default(),
        network: NetworkConfiguration {
            net_config_path: None,
            listen_addresses: vec![],
            public_addresses: vec![],
            boot_nodes: vec![],
            node_key: Default::default(),
            default_peers_set: Default::default(),
            default_peers_set_num_full: 0,
            client_version: "".to_string(),
            node_name: "".to_string(),
            transport: TransportConfig::MemoryOnly,
            max_parallel_downloads: 0,
            max_blocks_per_request: 0,
            sync_mode: Default::default(),
            enable_dht_random_walk: false,
            allow_non_globals_in_dht: false,
            kademlia_disjoint_query_paths: false,
            kademlia_replication_factor: NonZeroUsize::new(20).unwrap(),
            ipfs_server: false,
            yamux_window_size: None,
            network_backend: NetworkBackendType::Libp2p,
        },
        keystore: KeystoreConfig::InMemory,
        database: DatabaseSource::ParityDb {
            path: Default::default(),
        },
        trie_cache_maximum_size: None,
        state_pruning: None,
        blocks_pruning: BlocksPruning::KeepAll,
        chain_spec: Box::new(
            GenericChainSpec::<NoExtension, ()>::builder(Default::default(), NoExtension::None)
                .with_name("test")
                .with_id("test_id")
                .with_chain_type(ChainType::Development)
                .with_genesis_config_patch(Default::default())
                .build(),
        ),
        executor: ExecutorConfiguration {
            wasm_method: Default::default(),
            wasmtime_precompiled: None,
            default_heap_pages: None,
            max_runtime_instances: 0,
            runtime_cache_size: 0,
        },
        wasm_runtime_overrides: None,
        rpc: sc_service::config::RpcConfiguration {
            addr: None,
            max_connections: 0,
            cors: None,
            methods: Default::default(),
            max_request_size: 0,
            max_response_size: 0,
            id_provider: None,
            max_subs_per_conn: 0,
            port: 0,
            message_buffer_capacity: 0,
            batch_config: BatchRequestConfig::Disabled,
            rate_limit: None,
            rate_limit_whitelisted_ips: vec![],
            rate_limit_trust_proxy_headers: false,
        },
        prometheus_config: None,
        telemetry_endpoints: None,
        offchain_worker: Default::default(),
        force_authoring: false,
        disable_grandpa: false,
        dev_key_seed: None,
        tracing_targets: None,
        tracing_receiver: Default::default(),
        announce_block: false,
        data_path: Default::default(),
        base_path,
    }
}

/// Get the zombienet keystore path from the container base path.
fn zombienet_keystore_path(container_base_path: &Path) -> PathBuf {
    // container base path:
    // Collator-01/data/containers
    let mut zombienet_path = container_base_path.to_owned();
    zombienet_path.pop();
    // Collator-01/data/
    zombienet_path.push("chains/simple_container_2000/keystore/");
    // Collator-01/data/chains/simple_container_2000/keystore/

    zombienet_path
}

/// When running under zombienet, collator keys are injected in a different folder from what we
/// expect. This function will check if the zombienet folder exists, and if so, copy all the keys
/// from there into the expected folder.
pub fn copy_zombienet_keystore(
    keystore: &KeystoreConfig,
    container_base_path: sc_cli::Result<Option<BasePath>>,
) -> std::io::Result<()> {
    let container_base_path = match container_base_path {
        Ok(Some(base_path)) => base_path,
        _ => {
            // If base_path is not explicitly set, we are not running under zombienet, so there is nothing to do
            return Ok(());
        }
    };
    let keystore_path = keystore.path();
    let keystore_path = match keystore_path {
        Some(x) => x,
        None => {
            // In-memory keystore, zombienet does not use it by default so ignore it
            return Ok(());
        }
    };
    let zombienet_path = zombienet_keystore_path(container_base_path.path());

    if zombienet_path.exists() {
        // Copy to keystore folder
        let mut files_copied = 0;
        copy_dir_all(zombienet_path, keystore_path, &mut files_copied)?;
        log::info!("Copied {} keys from zombienet keystore", files_copied);

        Ok(())
    } else {
        // Zombienet folder does not exist, assume we are not running under zombienet
        Ok(())
    }
}

/// Equivalent to `cp -r src/* dst`
// https://stackoverflow.com/a/65192210
fn copy_dir_all(
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
    files_copied: &mut u32,
) -> std::io::Result<()> {
    use std::fs;
    fs::create_dir_all(&dst)?;
    // no-op if src and dst are the same dir
    let src_root = src.as_ref().canonicalize()?;
    let dst_root = dst.as_ref().canonicalize()?;
    if src_root == dst_root {
        return Ok(());
    }
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(
                entry.path(),
                dst.as_ref().join(entry.file_name()),
                files_copied,
            )?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
            *files_copied += 1;
        }
    }
    Ok(())
}
