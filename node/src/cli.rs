use crate::service::Sealing;
use std::path::PathBuf;

/// Sub-commands supported by the collator.
#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    /// Build a chain specification.
    BuildSpec(sc_cli::BuildSpecCmd),

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
    ExportGenesisState(cumulus_client_cli::ExportGenesisStateCommand),

    /// Export the genesis wasm of the parachain.
    ExportGenesisWasm(cumulus_client_cli::ExportGenesisWasmCommand),

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

    /// Relay chain arguments
    #[arg(raw = true)]
    pub extra_args: Vec<String>,
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
            let (relay_chain_args, extra_extra) = self.extra_args.split_at(i);
            (relay_chain_args, &extra_extra[1..])
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
#[derive(Debug, clap::Parser)]
#[group(skip)]
pub struct TanssiRunCmd {
	/// The cumulus RunCmd inherents from sc_cli's
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

#[derive(Debug)]
pub struct TanssiCli {
    /// The actual relay chain cli object.
    pub base: TanssiRunCmd,

    /// Optional chain id that should be passed to the relay chain.
    pub chain_id: Option<String>,

    /// The base path that should be used by the relay chain.
    pub base_path: Option<PathBuf>,
}

impl TanssiCli {
    /// Parse the relay chain CLI parameters using the para chain `Configuration`.
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
        }
    }
}
