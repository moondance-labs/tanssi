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
    node_common::{cli::BuildSpecCmd, service::node_builder::Sealing},
    tc_service_container_chain_rpc_provider::RpcProviderCmd,
};

pub type BaseSubcommand = node_common::cli::Subcommand<BuildSpecCmdSimple>;

/// Custom subcommand enum with `rpc-provider`
#[derive(Debug, clap::Subcommand)]
#[allow(clippy::large_enum_variant)]
pub enum Subcommand {
    RpcProvider(RpcProviderCmd),
    #[command(flatten)]
    Base(BaseSubcommand),
}

#[derive(Debug, Parser)]
#[group(skip)]
pub struct RunCmd {
    #[clap(flatten)]
    pub base: cumulus_client_cli::RunCmd,

    /// Id of the parachain this collator collates for.
    #[arg(long)]
    pub parachain_id: Option<u32>,

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

    // ===== WARNING =====
    // The following arguments are only parsed if `subcommand` is `None`. They
    // get default values when a subcommand is used!
    // TODO: Fix usage of those wrong values in subcommands.
    // SEE: https://github.com/paritytech/polkadot-sdk/issues/9356
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
    pub relay_chain_args: Vec<String>,
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

pub type BuildSpecCmdSimple = BuildSpecCmd<BuildSpecCmdExtraFields>;
