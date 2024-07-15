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

//! Container Chain Spawner
//!
//! Controls the starting and stopping of container chains.
//!
//! For more information about when the database is deleted, check the
//! [Keep db flowchart](https://raw.githubusercontent.com/moondance-labs/tanssi/master/docs/keep_db_flowchart.png)

use {
    crate::service::ParachainClient,
    cumulus_primitives_core::ParaId,
    dancebox_runtime::{AccountId, Block, BlockNumber},
    pallet_author_noting_runtime_api::AuthorNotingApi,
    sc_cli::SyncMode,
    sp_api::{ApiExt, ProvideRuntimeApi},
    std::sync::Arc,
};

/// Select `SyncMode` to use for a container chain.
/// We want to use warp sync unless the db still exists, or the block number is 0 (because of a warp sync bug in that case).
/// The reason is that warp sync doesn't work if a database already exists, it falls back to full sync instead.
pub fn select_sync_mode_based_on_api(
    db_exists: bool,
    orchestrator_client: &Arc<ParachainClient>,
    container_chain_para_id: ParaId,
) -> sc_service::error::Result<SyncMode> {
    if db_exists {
        // If the user wants to use warp sync, they should have already removed the database
        return Ok(SyncMode::Full);
    }

    // The following check is only needed because of this bug:
    // https://github.com/paritytech/polkadot-sdk/issues/1930

    let orchestrator_runtime_api = orchestrator_client.runtime_api();
    let orchestrator_chain_info = orchestrator_client.chain_info();

    // Force container chains to use warp sync, unless full sync is needed for some reason
    let full_sync_needed = if !orchestrator_runtime_api
        .has_api::<dyn AuthorNotingApi<Block, AccountId, BlockNumber, ParaId>>(
            orchestrator_chain_info.best_hash,
        )
        .map_err(|e| format!("Failed to check if runtime has AuthorNotingApi: {}", e))?
    {
        // Before runtime API was implemented we don't know if the container chain has any blocks,
        // so use full sync because that always works
        true
    } else {
        // If the container chain is still at genesis block, use full sync because warp sync is broken
        orchestrator_runtime_api
            .latest_author(orchestrator_chain_info.best_hash, container_chain_para_id)
            .map_err(|e| format!("Failed to read latest author: {}", e))?
            .is_none()
    };

    if full_sync_needed {
        Ok(SyncMode::Full)
    } else {
        Ok(SyncMode::Warp)
    }
}
