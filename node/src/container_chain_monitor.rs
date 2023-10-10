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

use crate::service::ParachainBackend;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;
use {
    crate::{
        cli::ContainerChainCli,
        container_chain_spawner::ContainerChainSpawnerState,
        service::{start_node_impl_container, ParachainClient},
    },
    cumulus_primitives_core::ParaId,
};

#[derive(Default)]
pub struct SpawnedContainersMonitor {
    pub list: Vec<SpawnedContainer>,
}

impl SpawnedContainersMonitor {
    pub fn push(&mut self, x: SpawnedContainer) {
        self.list.push(x);
    }

    pub fn running_chains(&self) -> Vec<&SpawnedContainer> {
        self.list
            .iter()
            .filter_map(|container| {
                if container.backend.strong_count() == 0 && container.client.strong_count() == 0 {
                    None
                } else {
                    Some(container)
                }
            })
            .collect()
    }

    pub fn truncate_old(&mut self, new_len: usize) {
        if self.list.len() <= new_len {
            return;
        }

        let mut to_retain = self.list.len() - new_len;
        self.list.retain(|_| {
            if to_retain == 0 {
                return true;
            }

            to_retain -= 1;
            return false;
        });
    }

    pub fn truncate_old_stopped_chains(&mut self, new_len: usize) -> Result<(), ()> {
        if self.list.len() <= new_len {
            return Ok(());
        }

        let mut to_retain = self.list.len() - new_len;
        self.list.retain(|container| {
            if to_retain == 0 {
                return true;
            }

            if container.backend.strong_count() == 0 && container.client.strong_count() == 0 {
                to_retain -= 1;
                return false;
            } else {
                return true;
            }
        });

        if self.list.len() <= new_len {
            return Ok(());
        } else {
            return Err(());
        }
    }
}

pub struct SpawnedContainer {
    pub para_id: ParaId,
    pub start_time: Instant,
    // TODO: how to set this?
    pub stop_time: Option<Instant>,
    pub backend: std::sync::Weak<ParachainBackend>,
    pub client: std::sync::Weak<ParachainClient>,
}

pub async fn monitor_task(state: Arc<Mutex<ContainerChainSpawnerState>>) {
    use tokio::time::{sleep, Duration};
    loop {
        let monitor_period = Duration::from_secs(300);
        sleep(monitor_period).await;
        let mut state = state.lock().unwrap();
        let monitor_state = &mut state.spawned_containers_monitor;

        // There should be at most 2 container chains running at the same time (1 syncing + 1 collating)
        // But add a margin of error because a container chain may take a few seconds to stop
        let max_running_container_chains = 4;
        let running_chains = monitor_state.running_chains();
        let running_para_ids: Vec<ParaId> = running_chains.iter().map(|x| x.para_id).collect();
        if running_chains.len() > max_running_container_chains {
            log::warn!("Too many container chains running at the same time");
            log::warn!(
                "Running container chains: {}: {:?}",
                running_chains.len(),
                running_para_ids
            );
        } else {
            log::debug!(
                "Running container chains: {}: {:?}",
                running_chains.len(),
                running_para_ids
            );
        }

        // Remove stopped container chains to keep the list small
        let _ = monitor_state.truncate_old_stopped_chains(10);
    }
}
