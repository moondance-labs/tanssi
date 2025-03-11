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

use sc_cli::{CliConfiguration, NodeKeyParams, SharedParams};

/// The `build-spec` command used to build a specification.
#[derive(Debug, Clone, clap::Parser)]
pub struct BuildSpecCmd {
    #[clap(flatten)]
    pub base: sc_cli::BuildSpecCmd,

    /// Id of the parachain this spec is for. Note that this overrides the `--chain` param.
    #[arg(long, conflicts_with = "chain")]
    #[arg(long)]
    pub parachain_id: Option<u32>,

    /// List of bootnodes to add to chain spec
    #[arg(long)]
    pub add_bootnode: Vec<String>,
}

impl CliConfiguration for BuildSpecCmd {
    fn shared_params(&self) -> &SharedParams {
        &self.base.shared_params
    }

    fn node_key_params(&self) -> Option<&NodeKeyParams> {
        Some(&self.base.node_key_params)
    }
}
