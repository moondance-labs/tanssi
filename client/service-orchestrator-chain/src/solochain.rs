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
    cumulus_client_cli::CollatorOptions,
    futures::FutureExt,
    log::{info, warn},
    node_common::{
        cli::RelayChainCli, service::solochain::RelayAsOrchestratorChainInterfaceBuilder,
    },
    sc_cli::{CliConfiguration, DefaultConfigurationValues, LoggerBuilder, Signals, SubstrateCli},
    sc_network::config::NetworkBackendType,
    sc_service::{
        config::{ExecutorConfiguration, KeystoreConfig, NetworkConfiguration, TransportConfig},
        BasePath, BlocksPruning, ChainType, Configuration, DatabaseSource, GenericChainSpec,
        KeystoreContainer, NoExtension, Role, TaskManager,
    },
    sp_keystore::KeystorePtr,
    std::{
        future::Future,
        marker::PhantomData,
        num::NonZeroUsize,
        path::{Path, PathBuf},
        sync::Arc,
        time::Duration,
    },
    tc_consensus::RelayChainInterface,
    tc_service_container_chain_spawner::cli::ContainerChainCli,
    tc_service_container_chain_spawner::{
        spawner,
        spawner::{CcSpawnMsg, ContainerChainSpawnParams, ContainerChainSpawner},
    },
    tokio::sync::mpsc::unbounded_channel,
    tokio_util::sync::CancellationToken,
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EnableContainerChainSpawner {
    Yes,
    No,
}

pub struct SolochainNodeStarted {
    pub task_manager: TaskManager,
    pub relay_chain_interface: Arc<dyn RelayChainInterface>,
    pub keystore: KeystorePtr,
}

/// Start a solochain node.
pub async fn start_solochain_node(
    polkadot_config: Configuration,
    container_chain_cli: ContainerChainCli,
    collator_options: CollatorOptions,
    hwbench: Option<sc_sysinfo::HwBench>,
    // In container chain rpc provider mode, it manages its own spawner.
    enable_cc_spawner: EnableContainerChainSpawner,
) -> sc_service::error::Result<SolochainNodeStarted> {
    let tokio_handle = polkadot_config.tokio_handle.clone();
    let orchestrator_para_id = Default::default();

    let chain_type = polkadot_config.chain_spec.chain_type().clone();
    let relay_chain = polkadot_config.chain_spec.id().to_string();

    // We use the relaychain keystore config for collators
    // Ensure that the user did not provide any custom keystore path for collators
    if container_chain_cli
        .base
        .base
        .keystore_params
        .keystore_path
        .is_some()
    {
        panic!(
            "--keystore-path not allowed here, must be set in relaychain args, after the first --"
        )
    }
    let keystore = &polkadot_config.keystore;

    // Instead of putting keystore in
    // Collator1000-01/data/chains/simple_container_2000/keystore
    // We put it in
    // Collator1000-01/relay-data/chains/dancelight_local_testnet/keystore
    // And same for "network" folder
    // But zombienet will put the keys in the old path, so we need to manually copy it if we
    // are running under zombienet
    copy_zombienet_keystore(keystore, container_chain_cli.base_path())?;

    let keystore_container = KeystoreContainer::new(keystore)?;

    // No metrics so no prometheus registry
    let prometheus_registry = None;
    let mut task_manager = TaskManager::new(tokio_handle.clone(), prometheus_registry)?;

    // Each container chain will spawn its own telemetry
    let telemetry_worker_handle = None;

    // Dummy parachain config only needed because `build_relay_chain_interface` needs to know if we
    // are collators or not
    let validator = container_chain_cli.base.collator;

    let mut dummy_parachain_config = dummy_config(
        polkadot_config.tokio_handle.clone(),
        polkadot_config.base_path.clone(),
    );
    dummy_parachain_config.role = if validator {
        Role::Authority
    } else {
        Role::Full
    };
    let (relay_chain_interface, collator_key) =
        cumulus_client_service::build_relay_chain_interface(
            polkadot_config,
            &dummy_parachain_config,
            telemetry_worker_handle.clone(),
            &mut task_manager,
            collator_options.clone(),
            hwbench.clone(),
        )
        .await
        .map_err(|e| sc_service::Error::Application(Box::new(e) as Box<_>))?;

    log::info!("start_solochain_node: is validator? {}", validator);

    let overseer_handle = relay_chain_interface
        .overseer_handle()
        .map_err(|e| sc_service::Error::Application(Box::new(e)))?;
    let sync_keystore = keystore_container.keystore();
    let collate_on_tanssi: Arc<
        dyn Fn() -> (CancellationToken, futures::channel::oneshot::Receiver<()>) + Send + Sync,
    > = Arc::new(move || {
        // collate_on_tanssi will not be called in solochains because solochains use a different consensus
        // mechanism and need validators instead of collators.
        // The runtime enforces this because the orchestrator_chain is never assigned any collators.
        panic!("Called collate_on_tanssi on solochain collator. This is unsupported and the runtime shouldn't allow this, it is a bug")
    });

    let orchestrator_chain_interface_builder = RelayAsOrchestratorChainInterfaceBuilder {
        overseer_handle: overseer_handle.clone(),
        relay_chain_interface: relay_chain_interface.clone(),
    };
    let orchestrator_chain_interface = orchestrator_chain_interface_builder.build();
    // Channel to send messages to start/stop container chains
    let (cc_spawn_tx, cc_spawn_rx) = unbounded_channel();

    if validator {
        if enable_cc_spawner == EnableContainerChainSpawner::No {
            panic!("cannot be a validator if container chain spawner is disabled");
        }

        // Start task which detects para id assignment, and starts/stops container chains.
        crate::build_check_assigned_para_id(
            orchestrator_chain_interface.clone(),
            sync_keystore.clone(),
            cc_spawn_tx.clone(),
            task_manager.spawn_essential_handle(),
        );
    }

    // If the orchestrator chain is running as a full-node, we start a full node for the
    // container chain immediately, because only collator nodes detect their container chain
    // assignment so otherwise it will never start.
    if !validator && enable_cc_spawner == EnableContainerChainSpawner::Yes {
        if let Some(container_chain_para_id) = container_chain_cli.base.para_id {
            // Spawn new container chain node
            cc_spawn_tx
                .send(CcSpawnMsg::UpdateAssignment {
                    current: Some(container_chain_para_id.into()),
                    next: Some(container_chain_para_id.into()),
                })
                .map_err(|e| sc_service::Error::Application(Box::new(e) as Box<_>))?;
        }
    }

    if enable_cc_spawner == EnableContainerChainSpawner::Yes {
        // Start container chain spawner task. This will start and stop container chains on demand.
        let spawn_handle = task_manager.spawn_handle();
        let relay_chain_interface = relay_chain_interface.clone();

        let container_chain_spawner = ContainerChainSpawner {
            params: ContainerChainSpawnParams {
                orchestrator_chain_interface,
                container_chain_cli,
                tokio_handle,
                chain_type,
                relay_chain,
                relay_chain_interface,
                sync_keystore,
                collation_params: if validator {
                    Some(spawner::CollationParams {
                        // TODO: all these args must be solochain instead of orchestrator
                        orchestrator_client: None,
                        orchestrator_tx_pool: None,
                        orchestrator_para_id,
                        collator_key: collator_key
                            .expect("there should be a collator key if we're a validator"),
                        solochain: true,
                    })
                } else {
                    None
                },
                spawn_handle,
                data_preserver: false,
                generate_rpc_builder:
                    tc_service_container_chain_spawner::rpc::GenerateSubstrateRpcBuilder::<
                        dancebox_runtime::RuntimeApi,
                    >::new(),
                phantom: PhantomData,
            },
            state: Default::default(),
            db_folder_cleanup_done: false,
            collate_on_tanssi,
            collation_cancellation_constructs: None,
        };
        let state = container_chain_spawner.state.clone();

        task_manager.spawn_essential_handle().spawn(
            "container-chain-spawner-rx-loop",
            None,
            container_chain_spawner.rx_loop(cc_spawn_rx, validator, true),
        );

        task_manager.spawn_essential_handle().spawn(
            "container-chain-spawner-debug-state",
            None,
            tc_service_container_chain_spawner::monitor::monitor_task(state),
        );
    }

    Ok(SolochainNodeStarted {
        task_manager,
        relay_chain_interface,
        keystore: keystore_container.keystore(),
    })
}

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
pub fn create_runner<T: SubstrateCli + CliConfiguration<DVC>, DVC: DefaultConfigurationValues>(
    command: &T,
) -> sc_cli::Result<SolochainRunner> {
    let tokio_runtime = sc_cli::build_runtime()?;

    // `capture` needs to be called in a tokio context.
    // Also capture them as early as possible.
    let signals = tokio_runtime.block_on(async { Signals::capture() })?;

    init_cmd(command, &T::support_url(), &T::impl_version())?;

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
    support_url: &str,
    impl_version: &str,
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
        solochain: true,
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
            network_backend: Some(NetworkBackendType::Libp2p),
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
            batch_config: jsonrpsee::server::BatchRequestConfig::Disabled,
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
