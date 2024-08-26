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

use {
    crate::spawner::{ContainerChainSpawner, TSelectSyncMode},
    dc_orchestrator_chain_interface::{
        DataPreserverAssignment, OrchestratorChainInterface, OrchestratorChainResult,
    },
    futures::stream::StreamExt,
    std::{future::Future},
    tc_consensus::ParaId,
};

pub type ProfileId = <dancebox_runtime::Runtime as pallet_data_preservers::Config>::ProfileId;

async fn try_fut<T, E>(fut: impl Future<Output = Result<T, E>>) -> Result<T, E> {
    fut.await
}

/// Watch assignements by indefinitly listening to finalized block notifications and switching to
/// the chain the profile is assigned to.
pub async fn task_watch_assignment<S: TSelectSyncMode>(
    spawner: ContainerChainSpawner<S>,
    profile_id: ProfileId,
) {
    use dc_orchestrator_chain_interface::DataPreserverAssignment as Assignment;

    if let OrchestratorChainResult::Err(e) = try_fut(async move {
        let orchestrator_chain_interface = spawner.params.orchestrator_chain_interface.clone();

        let mut current_assignment = DataPreserverAssignment::<ParaId>::NotAssigned;

        let mut stream = orchestrator_chain_interface
            .finality_notification_stream()
            .await?;

        while let Some(header) = stream.next().await {
            let hash = header.hash();

            let new_assignment = orchestrator_chain_interface
                .data_preserver_active_assignment(hash, profile_id)
                .await?;

            log::info!("Assignement for block {hash}: {new_assignment:?}");

            match (current_assignment, new_assignment) {
                // no change
                (x, y) if x == y => continue,
                (
                    Assignment::NotAssigned | Assignment::Inactive(_),
                    Assignment::Active(para_id),
                ) => {
                    spawner.spawn(para_id, false).await;
                }
                (Assignment::Active(para_id), Assignment::Inactive(x)) if para_id == x => {
                    spawner.stop(para_id, true); // keep db
                }
                (Assignment::Active(para_id), _) => {
                    spawner.stop(para_id, false); // don't keep db
                }
                // don't do anything yet
                (
                    Assignment::NotAssigned | Assignment::Inactive(_),
                    Assignment::NotAssigned | Assignment::Inactive(_),
                ) => (),
            }

            current_assignment = new_assignment;
        }

        Ok(())
    })
    .await
    {
        log::error!("Error in data preservers assignement watching task: {e:?}");
    }
}
