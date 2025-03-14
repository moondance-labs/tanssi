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
    clap::Parser,
    node_common::{cli::BuildSpecCmd, cli::Subcommand, service::Sealing},
    url::Url,
};

#[derive(Debug, Parser)]
#[group(skip)]
pub struct RunCmd {
    #[clap(flatten)]
    pub base: cumulus_client_cli::RunCmd,

    /// Size in bytes of the LRU cache for block data.
    #[arg(long, default_value = "300000000")]
    pub eth_log_block_cache: usize,

    /// Size in bytes of the LRU cache for transactions statuses data.
    #[arg(long, default_value = "300000000")]
    pub eth_statuses_cache: usize,

    /// Maximum number of logs in a query.
    #[arg(long, default_value = "10000")]
    pub max_past_logs: u32,

    /// Id of the parachain this collator collates for.
    #[arg(long)]
    pub parachain_id: Option<u32>,

    /// Maximum fee history cache size.
    #[arg(long, default_value = "2048")]
    pub fee_history_limit: u64,

    /// When blocks should be sealed in the dev service.
    ///
    /// Options are "instant", "manual", or timer interval in milliseconds
    #[arg(long, default_value = "instant")]
    pub sealing: Sealing,
}

impl std::ops::Deref for RunCmd {
    type Target = cumulus_client_cli::RunCmd;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

pub type FrontierSubcommand = Subcommand<BuildSpecCmdFrontier>;

#[derive(Debug, clap::Parser)]
#[command(
    propagate_version = true,
    args_conflicts_with_subcommands = true,
    subcommand_negates_reqs = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Option<FrontierSubcommand>,

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

    /// Profile id associated with the node, whose assignements will be followed to provide RPC services.
    #[arg(long)]
    pub rpc_provider_profile_id: Option<u64>,

    /// Endpoints to connect to orchestrator nodes, avoiding to start a local orchestrator node.
    /// If this list is empty, a local embeded orchestrator node is started.
    #[arg(long)]
    pub orchestrator_endpoints: Vec<Url>,

    /// Optional parachain id that should be used to build chain spec.
    #[arg(long)]
    pub para_id: Option<u32>,

    /// Relay chain arguments, optionally followed by "--" and container chain arguments
    #[arg(raw = true)]
    extra_args: Vec<String>,
}

impl Cli {
    pub fn relaychain_args(&self) -> &[String] {
        let (relay_chain_args, _) = self.split_extra_args_at_first_dashdash();

        relay_chain_args
    }

    pub fn container_chain_args(&self) -> &[String] {
        let (_, container_chain_args) = self.split_extra_args_at_first_dashdash();

        container_chain_args
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

#[derive(Debug, Clone, clap::Args)]
pub struct BuildSpecCmdExtraFields {
    /// List of bootnodes to add to chain spec
    #[arg(long)]
    pub add_bootnode: Vec<String>,

    /// Id of the parachain this spec is for. Note that this overrides the `--chain` param.
    #[arg(long, conflicts_with = "chain")]
    #[arg(long)]
    pub parachain_id: Option<u32>,
}

pub type BuildSpecCmdFrontier = BuildSpecCmd<BuildSpecCmdExtraFields>;

#[derive(Clone)]
pub struct RpcConfig {
    pub eth_log_block_cache: usize,
    pub eth_statuses_cache: usize,
    pub fee_history_limit: u64,
    pub max_past_logs: u32,
}
