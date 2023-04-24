use {
    crate::{chain_spec::RawGenesisConfigDummy, service::Sealing},
    pallet_registrar_runtime_api::ContainerChainGenesisData,
    sc_cli::{CliConfiguration, NodeKeyParams, SharedParams},
    std::{
        collections::{BTreeMap, HashMap},
        path::PathBuf,
        sync::{Arc, RwLock},
    },
};

/// Sub-commands supported by the collator.
#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    /// Build a chain specification.
    BuildSpec(BuildSpecCmd),

    /// Validate blocks.
    CheckBlock(sc_cli::CheckBlockCmd),

    /// Export blocks.
    ExportBlocks(sc_cli::ExportBlocksCmd),

    /// Export the state of a given block into a chain spec.
    ExportState(sc_cli::ExportStateCmd),

    /// Import blocks.
    ImportBlocks(sc_cli::ImportBlocksCmd),

    /// Revert the chain to a previous state.
    Revert(sc_cli::RevertCmd),

    /// Remove the whole chain.
    PurgeChain(cumulus_client_cli::PurgeChainCmd),

    /// Export the genesis state of the parachain.
    ExportGenesisState(ExportGenesisStateCommand),

    /// Export the genesis wasm of the parachain.
    ExportGenesisWasm(ExportGenesisWasmCommand),

    /// Sub-commands concerned with benchmarking.
    /// The pallet benchmarking moved to the `pallet` sub-command.
    #[command(subcommand)]
    Benchmark(frame_benchmarking_cli::BenchmarkCmd),

    /// Try some testing command against a specified runtime state.
    #[cfg(feature = "try-runtime")]
    TryRuntime(try_runtime_cli::TryRuntimeCmd),

    /// Errors since the binary was not build with `--features try-runtime`.
    #[cfg(not(feature = "try-runtime"))]
    TryRuntime,
}

/// The `build-spec` command used to build a specification.
#[derive(Debug, Clone, clap::Parser)]
pub struct BuildSpecCmd {
    #[clap(flatten)]
    pub base: sc_cli::BuildSpecCmd,

    /// Id of the parachain this spec is for. Note that this overrides the `--chain` param.
    #[clap(long)]
    pub parachain_id: Option<u32>,
}

impl CliConfiguration for BuildSpecCmd {
    fn shared_params(&self) -> &SharedParams {
        &self.base.shared_params
    }

    fn node_key_params(&self) -> Option<&NodeKeyParams> {
        Some(&self.base.node_key_params)
    }
}

/// Command for exporting the genesis state of the parachain
#[derive(Debug, clap::Parser)]
pub struct ExportGenesisStateCommand {
    /// Output file name or stdout if unspecified.
    #[clap(value_parser)]
    pub output: Option<PathBuf>,

    /// Id of the parachain this state is for.
    #[clap(long)]
    pub parachain_id: Option<u32>,

    /// Write output in binary. Default is to write in hex.
    #[clap(short, long)]
    pub raw: bool,

    /// The name of the chain for that the genesis state should be exported.
    #[clap(long)]
    pub chain: Option<String>,
}

/// Command for exporting the genesis wasm file.
#[derive(Debug, clap::Parser)]
pub struct ExportGenesisWasmCommand {
    /// Output file name or stdout if unspecified.
    #[clap(value_parser)]
    pub output: Option<PathBuf>,

    /// Write output in binary. Default is to write in hex.
    #[clap(short, long)]
    pub raw: bool,

    /// The name of the chain for that the genesis wasm file should be exported.
    #[clap(long)]
    pub chain: Option<String>,
}

#[derive(Debug, clap::Parser)]
#[group(skip)]
pub struct RunCmd {
    #[clap(flatten)]
    pub base: cumulus_client_cli::RunCmd,

    /// Enable the development service to run without a backing relay chain
    #[clap(long)]
    pub dev_service: bool,

    /// When blocks should be sealed in the dev service.
    ///
    /// Options are "instant", "manual", or timer interval in milliseconds
    #[clap(long, default_value = "instant")]
    pub sealing: Sealing,

    /// Id of the parachain this collator collates for.
    #[clap(long)]
    pub parachain_id: Option<u32>,
}

impl std::ops::Deref for RunCmd {
    type Target = cumulus_client_cli::RunCmd;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

#[derive(Debug, clap::Parser)]
#[command(
    propagate_version = true,
    args_conflicts_with_subcommands = true,
    subcommand_negates_reqs = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Option<Subcommand>,

    #[command(flatten)]
    pub run: RunCmd,

    /// Disable automatic hardware benchmarks.
    ///
    /// By default these benchmarks are automatically ran at startup and measure
    /// the CPU speed, the memory bandwidth and the disk speed.
    ///
    /// The results are then printed out in the logs, and also sent as part of
    /// telemetry, if telemetry is enabled.
    #[arg(long)]
    pub no_hardware_benchmarks: bool,

    /// Optional parachain id that should be used to build chain spec.
    #[arg(long)]
    pub para_id: Option<u32>,

    /// Relay chain arguments, optionally followed by "--" and Tanssi arguments
    #[arg(raw = true)]
    extra_args: Vec<String>,
}

impl Cli {
    pub fn relaychain_args(&self) -> &[String] {
        let (relay_chain_args, _tanssi_args) = self.split_extra_args_at_first_dashdash();

        relay_chain_args
    }

    pub fn tanssi_args(&self) -> &[String] {
        let (_relay_chain_args, tanssi_args) = self.split_extra_args_at_first_dashdash();

        tanssi_args
    }

    fn split_extra_args_at_first_dashdash(&self) -> (&[String], &[String]) {
        let index_of_dashdash = self.extra_args.iter().position(|x| *x == "--");

        if let Some(i) = index_of_dashdash {
            let (container_chain_args, extra_extra) = self.extra_args.split_at(i);
            (&extra_extra[1..], container_chain_args)
        } else {
            // Only relay chain args
            (&self.extra_args, &[])
        }
    }
}

#[derive(Debug)]
pub struct RelayChainCli {
    /// The actual relay chain cli object.
    pub base: polkadot_cli::RunCmd,

    /// Optional chain id that should be passed to the relay chain.
    pub chain_id: Option<String>,

    /// The base path that should be used by the relay chain.
    pub base_path: Option<PathBuf>,
}

impl RelayChainCli {
    /// Parse the relay chain CLI parameters using the para chain `Configuration`.
    pub fn new<'a>(
        para_config: &sc_service::Configuration,
        relay_chain_args: impl Iterator<Item = &'a String>,
    ) -> Self {
        let extension = crate::chain_spec::Extensions::try_get(&*para_config.chain_spec);
        let chain_id = extension.map(|e| e.relay_chain.clone());
        let base_path = para_config
            .base_path
            .as_ref()
            .map(|x| x.path().join("polkadot"));
        Self {
            base_path,
            chain_id,
            base: clap::Parser::parse_from(relay_chain_args),
        }
    }
}

/// The `run` command used to run a node.
#[derive(Debug, clap::Parser, Clone)]
#[group(skip)]
pub struct TanssiRunCmd {
    /// The cumulus RunCmd inherits from sc_cli's
    #[command(flatten)]
    pub base: sc_cli::RunCmd,

    /// Run node as collator.
    ///
    /// Note that this is the same as running with `--validator`.
    #[arg(long, conflicts_with = "validator")]
    pub collator: bool,

    /// Optional parachain id that should be used to build chain spec.
    #[arg(long)]
    pub para_id: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct TanssiCli {
    /// The actual Tanssi cli object.
    pub base: TanssiRunCmd,

    /// Optional chain id that should be passed to Tanssi.
    pub chain_id: Option<String>,

    /// The base path that should be used by Tanssi.
    pub base_path: Option<PathBuf>,

    /// The ChainSpecs that this struct can initialize. This starts empty and gets filled
    /// by calling preload_chain_spec_file.
    pub preloaded_chain_specs: Arc<RwLock<HashMap<String, Box<dyn sc_chain_spec::ChainSpec>>>>,
}

impl TanssiCli {
    /// Parse the Tanssi CLI parameters using the para chain `Configuration`.
    pub fn new<'a>(
        para_config: &sc_service::Configuration,
        tanssi_args: impl Iterator<Item = &'a String>,
    ) -> Self {
        let extension = crate::chain_spec::Extensions::try_get(&*para_config.chain_spec);
        let chain_id = extension.map(|e| e.relay_chain.clone());
        let base_path = para_config
            .base_path
            .as_ref()
            .map(|x| x.path().join("polkadot"));
        Self {
            base_path,
            chain_id,
            base: clap::Parser::parse_from(tanssi_args),
            preloaded_chain_specs: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn preload_chain_spec_from_genesis_data(
        &mut self,
        para_id: u32,
        genesis_data: ContainerChainGenesisData,
    ) -> Result<(), String> {
        let name = format!("Local testnet");
        let id = format!("local_testnet");
        let map: BTreeMap<_, _> = genesis_data.storage.into_iter().map(|x| x.into()).collect();
        let boot_nodes = vec![];
        let properties_json_bytes = genesis_data.properties;
        // TODO: definitely do not unwrap here, as this is reading on chain data that may not be valid json
        let properties = Some(serde_json::from_slice(&properties_json_bytes).unwrap());
        let extensions = crate::chain_spec::Extensions {
            relay_chain: "rococo_local_testnet".to_string(),
            para_id,
        };
        let chain_spec = Box::new(crate::chain_spec::RawChainSpec::from_genesis(
            &name,
            &id,
            sc_chain_spec::ChainType::Local,
            move || RawGenesisConfigDummy { map: map.clone() },
            boot_nodes,
            None,
            Some("template-local"),
            None,
            properties,
            extensions,
        ));

        self.preloaded_chain_specs
            .write()
            .unwrap()
            .insert(format!("container-chain-{}", para_id), chain_spec);

        Ok(())
    }
}
