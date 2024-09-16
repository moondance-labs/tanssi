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

use sc_cli::CliConfiguration;
use url::Url;
use {
    crate::chain_spec::RawGenesisConfig,
    cumulus_client_cli::{CollatorOptions, RelayChainMode},
    dc_orchestrator_chain_interface::ContainerChainGenesisData,
    dp_container_chain_genesis_data::json::properties_to_map,
    sc_chain_spec::ChainSpec,
    sc_network::config::MultiaddrWithPeerId,
    sp_runtime::Storage,
    std::{collections::BTreeMap, net::SocketAddr, path::PathBuf},
};

/// The `run` command used to run a container chain node.
#[derive(Debug, clap::Parser, Clone)]
#[group(skip)]
pub struct ContainerChainRunCmd {
    /// The cumulus RunCmd inherits from sc_cli's
    #[command(flatten)]
    pub base: sc_cli::RunCmd,

    /// Run node as collator.
    ///
    /// Note that this is the same as running with `--validator`.
    #[arg(long, conflicts_with = "validator")]
    pub collator: bool,

    /// Optional container chain para id that should be used to build chain spec.
    #[arg(long)]
    pub para_id: Option<u32>,

    /// Keep container-chain db after changing collator assignments
    #[arg(long)]
    pub keep_db: bool,

    /// Creates a less resource-hungry node that retrieves relay chain data from an RPC endpoint.
    ///
    /// The provided URLs should point to RPC endpoints of the relay chain.
    /// This node connects to the remote nodes following the order they were specified in. If the
    /// connection fails, it attempts to connect to the next endpoint in the list.
    ///
    /// Note: This option doesn't stop the node from connecting to the relay chain network but
    /// reduces bandwidth use.
    #[arg(
		long,
		value_parser = validate_relay_chain_url,
		num_args = 0..,
		alias = "relay-chain-rpc-url"
    )]
    pub relay_chain_rpc_urls: Vec<Url>,

    /// EXPERIMENTAL: Embed a light client for the relay chain. Only supported for full-nodes.
    /// Will use the specified relay chain chainspec.
    #[arg(long, conflicts_with_all = ["relay_chain_rpc_urls", "collator"])]
    pub relay_chain_light_client: bool,
}

impl ContainerChainRunCmd {
    /// Create a [`NormalizedRunCmd`] which merges the `collator` cli argument into `validator` to
    /// have only one.
    // Copied from polkadot-sdk/cumulus/client/cli/src/lib.rs
    // TODO: deduplicate this function and [ContainerChainCli::new]
    pub fn normalize(&self) -> ContainerChainCli {
        let mut new_base = self.clone();

        new_base.base.validator = self.base.validator || self.collator;

        // Append `containers/` to base_path for this object. This is to ensure that when spawning
        // a new container chain, its database is always inside the `containers` folder.
        // So if the user passes `--base-path /tmp/node`, we want the ephemeral container data in
        // `/tmp/node/containers`, and the persistent storage in `/tmp/node/config`.
        // TODO: there should be a way to avoid this if we refactor the code that creates the db,
        // but maybe that breaks dancebox
        let base_path = match self.base.base_path() {
            Ok(Some(x)) => x,
            _ => {
                // This is maybe unreachable. There is always a default base path, and if run in
                // `--dev` or `--tmp` mode, a temporary base path is created.
                panic!("No base path")
            }
        };
        
        let base_path = base_path.path().join("containers");

        ContainerChainCli {
            base: new_base,
            preloaded_chain_spec: None,
        }
    }

    /// Create [`CollatorOptions`] representing options only relevant to parachain collator nodes
    // Copied from polkadot-sdk/cumulus/client/cli/src/lib.rs
    pub fn collator_options(&self) -> CollatorOptions {
        let relay_chain_mode = match (
            self.relay_chain_light_client,
            !self.relay_chain_rpc_urls.is_empty(),
        ) {
            (true, _) => RelayChainMode::LightClient,
            (_, true) => RelayChainMode::ExternalRpc(self.relay_chain_rpc_urls.clone()),
            _ => RelayChainMode::Embedded,
        };

        CollatorOptions { relay_chain_mode }
    }
}

#[derive(Debug)]
pub struct ContainerChainCli {
    /// The actual container chain cli object.
    pub base: ContainerChainRunCmd,

    /// The ChainSpecs that this struct can initialize. This starts empty and gets filled
    /// by calling preload_chain_spec_file.
    pub preloaded_chain_spec: Option<Box<dyn sc_chain_spec::ChainSpec>>,
}

impl Clone for ContainerChainCli {
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
            preloaded_chain_spec: self.preloaded_chain_spec.as_ref().map(|x| x.cloned_box()),
        }
    }
}

impl ContainerChainCli {
    /// Parse the container chain CLI parameters using the para chain `Configuration`.
    pub fn new<'a>(
        para_config: &sc_service::Configuration,
        container_chain_args: impl Iterator<Item = &'a String>,
    ) -> Self {
        let mut base: ContainerChainRunCmd = clap::Parser::parse_from(container_chain_args);

        // Copy some parachain args into container chain args
        // TODO: warn the user if they try to set one of these args using container chain args,
        // because that args will be ignored
        let base_path = para_config.base_path.path().join("containers");
        base.base.shared_params.base_path = Some(base_path);
        // TODO: move wasmtime_precompiled here

        Self {
            base,
            preloaded_chain_spec: None,
        }
    }

    pub fn chain_spec_from_genesis_data(
        para_id: u32,
        genesis_data: ContainerChainGenesisData,
        chain_type: sc_chain_spec::ChainType,
        relay_chain: String,
        boot_nodes: Vec<MultiaddrWithPeerId>,
    ) -> Result<crate::chain_spec::RawChainSpec, String> {
        let name = String::from_utf8(genesis_data.name).map_err(|_e| "Invalid name".to_string())?;
        let id: String =
            String::from_utf8(genesis_data.id).map_err(|_e| "Invalid id".to_string())?;
        let storage_raw: BTreeMap<_, _> =
            genesis_data.storage.into_iter().map(|x| x.into()).collect();
        let protocol_id = format!("container-chain-{}", para_id);
        let properties = properties_to_map(&genesis_data.properties)
            .map_err(|e| format!("Invalid properties: {}", e))?;
        let extensions = crate::chain_spec::Extensions {
            relay_chain,
            para_id,
        };
        let raw_genesis_config = RawGenesisConfig {
            storage_raw: storage_raw.clone(),
        };

        let chain_spec = crate::chain_spec::RawChainSpec::builder(
            // This code is not used, we override it in `set_storage` below
            &[],
            // TODO: what to do with extensions? We are hardcoding the relay_chain and the para_id, any
            // other extensions are being ignored
            extensions,
        )
        .with_name(&name)
        .with_id(&id)
        .with_chain_type(chain_type)
        .with_properties(properties)
        .with_boot_nodes(boot_nodes)
        .with_protocol_id(&protocol_id);

        let chain_spec = if let Some(fork_id) = genesis_data.fork_id {
            let fork_id_string =
                String::from_utf8(fork_id).map_err(|_e| "Invalid fork_id".to_string())?;
            chain_spec.with_fork_id(&fork_id_string)
        } else {
            chain_spec
        };

        let mut chain_spec = chain_spec.build();

        chain_spec.set_storage(Storage {
            top: raw_genesis_config.storage_raw,
            children_default: Default::default(),
        });

        Ok(chain_spec)
    }

    pub fn preload_chain_spec_from_genesis_data(
        &mut self,
        para_id: u32,
        genesis_data: ContainerChainGenesisData,
        chain_type: sc_chain_spec::ChainType,
        relay_chain: String,
        boot_nodes: Vec<MultiaddrWithPeerId>,
    ) -> Result<(), String> {
        let chain_spec = Self::chain_spec_from_genesis_data(
            para_id,
            genesis_data,
            chain_type,
            relay_chain,
            boot_nodes,
        )?;
        self.preloaded_chain_spec = Some(Box::new(chain_spec));

        Ok(())
    }
}

impl sc_cli::SubstrateCli for ContainerChainCli {
    fn impl_name() -> String {
        "Container chain".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        format!(
            "Container chain\n\nThe command-line arguments provided first will be \
		passed to the orchestrator chain node, while the arguments provided after -- will be passed \
		to the container chain node, and the arguments provided after another -- will be passed \
		to the relay chain node\n\n\
		{} [orchestrator-args] -- [container-chain-args] -- [relay-chain-args] -- ",
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

    fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_cli::ChainSpec>, String> {
        // ContainerChain ChainSpec must be preloaded beforehand because we need to call async
        // functions to generate it, and this function is not async.
        let para_id = parse_container_chain_id_str(id)?;

        match &self.preloaded_chain_spec {
            Some(spec) => {
                let spec_para_id = crate::chain_spec::Extensions::try_get(&**spec)
                    .map(|extension| extension.para_id);

                if spec_para_id == Some(para_id) {
                    Ok(spec.cloned_box())
                } else {
                    Err(format!(
                        "Expected ChainSpec for id {}, found ChainSpec for id {:?} instead",
                        para_id, spec_para_id
                    ))
                }
            }
            None => Err(format!("ChainSpec for {} not found", id)),
        }
    }
}

impl sc_cli::DefaultConfigurationValues for ContainerChainCli {
    fn p2p_listen_port() -> u16 {
        30335
    }

    fn rpc_listen_port() -> u16 {
        9946
    }

    fn prometheus_listen_port() -> u16 {
        9617
    }
}

impl sc_cli::CliConfiguration<Self> for ContainerChainCli {
    fn shared_params(&self) -> &sc_cli::SharedParams {
        self.base.base.shared_params()
    }

    fn import_params(&self) -> Option<&sc_cli::ImportParams> {
        self.base.base.import_params()
    }

    fn network_params(&self) -> Option<&sc_cli::NetworkParams> {
        self.base.base.network_params()
    }

    fn keystore_params(&self) -> Option<&sc_cli::KeystoreParams> {
        self.base.base.keystore_params()
    }

    fn base_path(&self) -> sc_cli::Result<Option<sc_service::BasePath>> {
        Ok(self
            .shared_params()
            .base_path()?
            .or_else(|| Some(self.base_path.clone().into())))
    }

    fn rpc_addr(&self, default_listen_port: u16) -> sc_cli::Result<Option<SocketAddr>> {
        self.base.base.rpc_addr(default_listen_port)
    }

    fn prometheus_config(
        &self,
        default_listen_port: u16,
        chain_spec: &Box<dyn sc_cli::ChainSpec>,
    ) -> sc_cli::Result<Option<sc_service::config::PrometheusConfig>> {
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
    ) -> sc_cli::Result<()>
    where
        F: FnOnce(&mut sc_cli::LoggerBuilder, &sc_service::Configuration),
    {
        unreachable!("PolkadotCli is never initialized; qed");
    }

    fn chain_id(&self, _is_dev: bool) -> sc_cli::Result<String> {
        self.base
            .para_id
            .map(|para_id| format!("container-chain-{}", para_id))
            .ok_or("no para-id in container chain args".into())
    }

    fn role(&self, is_dev: bool) -> sc_cli::Result<sc_service::Role> {
        self.base.base.role(is_dev)
    }

    fn transaction_pool(
        &self,
        is_dev: bool,
    ) -> sc_cli::Result<sc_service::config::TransactionPoolOptions> {
        self.base.base.transaction_pool(is_dev)
    }

    fn trie_cache_maximum_size(&self) -> sc_cli::Result<Option<usize>> {
        self.base.base.trie_cache_maximum_size()
    }

    fn rpc_methods(&self) -> sc_cli::Result<sc_service::config::RpcMethods> {
        self.base.base.rpc_methods()
    }

    fn rpc_max_connections(&self) -> sc_cli::Result<u32> {
        self.base.base.rpc_max_connections()
    }

    fn rpc_cors(&self, is_dev: bool) -> sc_cli::Result<Option<Vec<String>>> {
        self.base.base.rpc_cors(is_dev)
    }

    fn default_heap_pages(&self) -> sc_cli::Result<Option<u64>> {
        self.base.base.default_heap_pages()
    }

    fn force_authoring(&self) -> sc_cli::Result<bool> {
        self.base.base.force_authoring()
    }

    fn disable_grandpa(&self) -> sc_cli::Result<bool> {
        self.base.base.disable_grandpa()
    }

    fn max_runtime_instances(&self) -> sc_cli::Result<Option<usize>> {
        self.base.base.max_runtime_instances()
    }

    fn announce_block(&self) -> sc_cli::Result<bool> {
        self.base.base.announce_block()
    }

    fn telemetry_endpoints(
        &self,
        chain_spec: &Box<dyn sc_chain_spec::ChainSpec>,
    ) -> sc_cli::Result<Option<sc_telemetry::TelemetryEndpoints>> {
        self.base.base.telemetry_endpoints(chain_spec)
    }

    fn node_name(&self) -> sc_cli::Result<String> {
        self.base.base.node_name()
    }
}

/// Parse ParaId(2000) from a string like "container-chain-2000"
fn parse_container_chain_id_str(id: &str) -> std::result::Result<u32, String> {
    // The id has been created using format!("container-chain-{}", para_id), so here we need
    // to reverse that.
    id.strip_prefix("container-chain-")
        .and_then(|s| {
            let id: u32 = s.parse().ok()?;

            // `.parse()` ignores leading zeros, so convert the id back to string to check
            // if we get the same string, this way we ensure a 1:1 mapping
            if id.to_string() == s {
                Some(id)
            } else {
                None
            }
        })
        .ok_or_else(|| format!("load_spec called with invalid id: {:?}", id))
}

// Copied from polkadot-sdk/cumulus/client/cli/src/lib.rs
fn validate_relay_chain_url(arg: &str) -> Result<Url, String> {
    let url = Url::parse(arg).map_err(|e| e.to_string())?;

    let scheme = url.scheme();
    if scheme == "ws" || scheme == "wss" {
        Ok(url)
    } else {
        Err(format!(
            "'{}' URL scheme not supported. Only websocket RPC is currently supported",
            url.scheme()
        ))
    }
}
