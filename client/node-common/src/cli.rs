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
    sc_cli::{CliConfiguration, NodeKeyParams, SharedParams},
    sp_runtime::traits::Get,
    std::path::PathBuf,
};

/// The `build-spec` command used to build a specification.
#[derive(Debug, Clone, clap::Parser)]
pub struct BuildSpecCmd<ExtraFields = EmptyExtra>
where
    ExtraFields: clap::Args,
{
    #[clap(flatten)]
    pub base: sc_cli::BuildSpecCmd,

    #[clap(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, clap::Args, Default)]
pub struct EmptyExtra {}

impl<T> CliConfiguration for BuildSpecCmd<T>
where
    T: clap::Args,
{
    fn shared_params(&self) -> &SharedParams {
        &self.base.shared_params
    }

    fn node_key_params(&self) -> Option<&NodeKeyParams> {
        Some(&self.base.node_key_params)
    }
}

#[derive(Debug)]
pub struct RelayChainCli<N: Get<&'static str>> {
    /// The actual relay chain cli object.
    pub base: polkadot_cli::RunCmd,

    /// Optional chain id that should be passed to the relay chain.
    pub chain_id: Option<String>,

    /// The base path that should be used by the relay chain.
    pub base_path: PathBuf,

    /// Phantom type for storing node name
    _marker: std::marker::PhantomData<N>,
}

impl<N: Get<&'static str>> RelayChainCli<N> {
    /// Parse the relay chain CLI parameters using the para chain `Configuration`.
    pub fn new<'a>(
        para_config: &sc_service::Configuration,
        relay_chain_args: impl Iterator<Item = &'a String>,
    ) -> Self {
        let extension = crate::chain_spec::Extensions::try_get(&*para_config.chain_spec);
        let chain_id = extension.map(|e| e.relay_chain.clone());
        let base_path = para_config.base_path.path().join("polkadot");
        Self {
            base_path,
            chain_id,
            base: clap::Parser::parse_from(relay_chain_args),
            _marker: std::marker::PhantomData,
        }
    }
}

/// Sub-commands supported by the collator.
#[derive(Debug, clap::Subcommand)]
#[allow(clippy::large_enum_variant)]
pub enum Subcommand<B>
where
    B: clap::Args,
{
    /// Build a chain specification.
    BuildSpec(B),

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
    #[command(alias = "export-genesis-state")]
    ExportGenesisHead(cumulus_client_cli::ExportGenesisHeadCommand),

    /// Export the genesis wasm of the parachain.
    ExportGenesisWasm(cumulus_client_cli::ExportGenesisWasmCommand),

    /// Sub-commands concerned with benchmarking.
    /// The pallet benchmarking moved to the `pallet` sub-command.
    #[command(subcommand)]
    Benchmark(frame_benchmarking_cli::BenchmarkCmd),

    /// Precompile the WASM runtime into native code
    PrecompileWasm(sc_cli::PrecompileWasmCmd),
}
