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
    cumulus_primitives_core::ParaId,
    std::sync::{Arc, Mutex},
    tc_container_chain_spawner::container_chain_spawner::{CcSpawnMsg, ContainerChainSpawnerState},
    tokio::{
        sync::mpsc::UnboundedSender,
        time::{sleep, Duration},
    },
};

/// Background task that monitors the number of running container chains.
pub async fn monitor_task(state: Arc<Mutex<ContainerChainSpawnerState>>) {
    // Main loop frequency, doesn't need to be fast
    let monitor_period = Duration::from_secs(300 * 0 + 10);
    // Max number of allowed container chains before printing warnings.
    // There should be at most 2 container chains running at the same time (1 syncing + 1 collating),
    // but add a margin of error because a container chain may take a few seconds to stop.
    let max_running_container_chains = 4;

    loop {
        sleep(monitor_period).await;
        log::debug!("Monitor tick");
        let mut state = state.lock().unwrap();
        let monitor_state = &mut state.spawned_containers_monitor;

        let running_chains = monitor_state.running_chains();
        let running_para_ids: Vec<ParaId> = running_chains.iter().map(|x| x.para_id).collect();
        if running_chains.len() > max_running_container_chains {
            log::warn!("Too many container chains running at the same time");
            log::warn!(
                "Running container chains: {}: {:?}",
                running_chains.len(),
                running_para_ids
            );
            log::debug!(
                "{:?}",
                running_chains
                    .iter()
                    .map(|x| x.summary())
                    .collect::<Vec<_>>()
            )
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

#[allow(unused)]
/// Start and stop the same container chain in a loop, used for testing and debugging
pub async fn debug_start_and_stop_same_cc(cc_spawn_tx: UnboundedSender<CcSpawnMsg>) {
    let sleep_delay = Duration::from_secs(10);

    loop {
        sleep(sleep_delay).await;
        cc_spawn_tx
            .send(CcSpawnMsg::UpdateAssignment {
                current: Some(2000u32.into()),
                next: None,
            })
            .unwrap();
        sleep(sleep_delay).await;
        cc_spawn_tx
            .send(CcSpawnMsg::UpdateAssignment {
                current: None,
                next: None,
            })
            .unwrap();
        sleep(sleep_delay).await;
        cc_spawn_tx
            .send(CcSpawnMsg::UpdateAssignment {
                current: None,
                next: Some(2001u32.into()),
            })
            .unwrap();
        sleep(sleep_delay).await;
        cc_spawn_tx
            .send(CcSpawnMsg::UpdateAssignment {
                current: None,
                next: None,
            })
            .unwrap();
    }
}
