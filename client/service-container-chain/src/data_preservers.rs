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
    dc_orchestrator_chain_interface::{OrchestratorChainInterface, OrchestratorChainResult, PHash},
    futures::stream::StreamExt,
    std::{future::Future, sync::Arc},
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
    todo!()
}
