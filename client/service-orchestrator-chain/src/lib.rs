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

//! Code responsible for spawning orchestrator chain node, either in solochain
//! or parachain mode. It is extracted outside of `tanssi-node`/`chains/orchestrator-paras/node`
//! to be used by `service-container-chain-rpc-provider` which can embed an orchestrator node too.

pub mod parachain;
pub mod solochain;

use {
    dc_orchestrator_chain_interface::OrchestratorChainInterface,
    futures::StreamExt,
    nimbus_primitives::NimbusPair,
    sp_core::{traits::SpawnEssentialNamed, H256},
    sp_keystore::KeystorePtr,
    std::sync::Arc,
    tc_service_container_chain_spawner::spawner::CcSpawnMsg,
    tokio::sync::mpsc,
};

/// Background task used to detect changes to container chain assignment,
/// and start/stop container chains on demand. The check runs on every new block.
pub fn build_check_assigned_para_id(
    client: Arc<dyn OrchestratorChainInterface>,
    sync_keystore: KeystorePtr,
    cc_spawn_tx: mpsc::UnboundedSender<CcSpawnMsg>,
    spawner: impl SpawnEssentialNamed,
) {
    let check_assigned_para_id_task = async move {
        // Subscribe to new blocks in order to react to para id assignment
        // This must be the stream of finalized blocks, otherwise the collators may rotate to a
        // different chain before the block is finalized, and that could lead to a stalled chain
        let mut import_notifications = client.finality_notification_stream().await.unwrap();

        while let Some(msg) = import_notifications.next().await {
            let block_hash = msg.hash();
            let client_set_aside_for_cidp = client.clone();
            let sync_keystore = sync_keystore.clone();
            let cc_spawn_tx = cc_spawn_tx.clone();

            check_assigned_para_id(
                cc_spawn_tx,
                sync_keystore,
                client_set_aside_for_cidp,
                block_hash,
            )
            .await
            .unwrap();
        }
    };

    spawner.spawn_essential(
        "check-assigned-para-id",
        None,
        Box::pin(check_assigned_para_id_task),
    );
}

/// Check the parachain assignment using the orchestrator chain client, and send a `CcSpawnMsg` to
/// start or stop the required container chains.
///
/// Checks the assignment for the next block, so if there is a session change on block 15, this will
/// detect the assignment change after importing block 14.
async fn check_assigned_para_id(
    cc_spawn_tx: mpsc::UnboundedSender<CcSpawnMsg>,
    sync_keystore: KeystorePtr,
    client_set_aside_for_cidp: Arc<dyn OrchestratorChainInterface>,
    block_hash: H256,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Check current assignment
    let current_container_chain_para_id =
        tc_consensus::first_eligible_key::<dyn OrchestratorChainInterface, NimbusPair>(
            client_set_aside_for_cidp.as_ref(),
            &block_hash,
            sync_keystore.clone(),
        )
        .await
        .map(|(_nimbus_key, para_id)| para_id);

    // Check assignment in the next session
    let next_container_chain_para_id = tc_consensus::first_eligible_key_next_session::<
        dyn OrchestratorChainInterface,
        NimbusPair,
    >(
        client_set_aside_for_cidp.as_ref(),
        &block_hash,
        sync_keystore,
    )
    .await
    .map(|(_nimbus_key, para_id)| para_id);

    cc_spawn_tx.send(CcSpawnMsg::UpdateAssignment {
        current: current_container_chain_para_id,
        next: next_container_chain_para_id,
    })?;

    Ok(())
}
