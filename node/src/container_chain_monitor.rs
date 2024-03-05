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
    crate::{
        container_chain_spawner::{CcSpawnMsg, ContainerChainSpawnerState},
        service::{ContainerChainBackend, ContainerChainClient},
    },
    cumulus_primitives_core::ParaId,
    std::{
        cell::Cell,
        collections::VecDeque,
        sync::{Arc, Mutex},
        time::Instant,
    },
    tokio::{
        sync::mpsc::UnboundedSender,
        time::{sleep, Duration},
    },
};

#[derive(Default)]
pub struct SpawnedContainersMonitor {
    /// List of the N most recently started container chains, with some statistics related to
    /// stopping time and reference count.
    list: VecDeque<SpawnedContainer>,
    /// Count the number of times a container chain has been started
    count: usize,
}

pub struct SpawnedContainer {
    /// Unique identifier for a spawned container (not ParaId)
    pub id: usize,
    /// Container chain para id
    pub para_id: ParaId,
    /// When did the container chain start
    pub start_time: Instant,
    /// When the container chain was asked to stop (`StopContainerChain` was dropped)
    pub stop_signal_time: Option<Instant>,
    /// When the container chain task manager was dropped, this should finish all the background
    /// tasks except the ones started in separate threads.
    pub stop_task_manager_time: Option<Instant>,
    /// When the `monitor_task` first observed that the reference counts are all 0.
    /// This won't be precise because it is checked using polling with a high period.
    pub stop_refcount_time: Cell<Option<Instant>>,
    /// Used to check the reference count, if it's 0 it means the database has been closed
    pub backend: std::sync::Weak<ContainerChainBackend>,
    /// Used to check the reference count, if it's 0 it means that the client has been closed.
    pub client: std::sync::Weak<ContainerChainClient>,
}

impl SpawnedContainer {
    pub fn is_stopped(&self) -> bool {
        self.stop_refcount_time.get().is_some() || {
            // Check reference count, and set stop_refcount_time if zero
            let refcount_zero = self.backend.strong_count() == 0 && self.client.strong_count() == 0;
            if refcount_zero {
                self.stop_refcount_time.set(Some(Instant::now()));

                true
            } else {
                false
            }
        }
    }

    pub fn summary(&self) -> String {
        #[derive(Debug)]
        #[allow(unused)]
        struct SpawnedContainerSummary {
            id: usize,
            para_id: ParaId,
            time_start_to_now: Duration,
            time_start_to_stop_signal: Option<Duration>,
            time_stop_signal_to_stop_task_manager: Option<Duration>,
            time_stop_task_manager_to_stop_refcount: Option<Duration>,
            time_stop_refcount_to_now: Option<Duration>,
            backend_refcount: usize,
            client_refcount: usize,
        }

        let summary = SpawnedContainerSummary {
            id: self.id,
            para_id: self.para_id,
            time_start_to_now: Instant::now().duration_since(self.start_time),
            time_start_to_stop_signal: self
                .stop_signal_time
                .map(|x| x.duration_since(self.start_time)),
            time_stop_signal_to_stop_task_manager: self
                .stop_task_manager_time
                .and_then(|x| Some(x.duration_since(self.stop_signal_time?))),
            time_stop_task_manager_to_stop_refcount: self
                .stop_refcount_time
                .get()
                .and_then(|x| Some(x.duration_since(self.stop_task_manager_time?))),
            time_stop_refcount_to_now: self
                .stop_refcount_time
                .get()
                .map(|x| Instant::now().duration_since(x)),
            backend_refcount: self.backend.strong_count(),
            client_refcount: self.client.strong_count(),
        };

        format!("{:?}", summary)
    }
}

impl SpawnedContainersMonitor {
    /// Returns a unique id which is not the ParaId
    pub fn push(&mut self, mut x: SpawnedContainer) -> usize {
        assert_eq!(x.id, 0, "SpawnedContainer.id must be set to 0, the actual id will be returned from push function");
        let id = self.count;
        x.id = id;
        self.list.push_back(x);
        self.count += 1;

        id
    }

    pub fn set_stop_signal_time(&mut self, id: usize, when: Instant) {
        let i = self.list.iter().position(|x| x.id == id);

        if let Some(i) = i {
            self.list[i].stop_signal_time = Some(when);
        }
    }

    pub fn set_stop_task_manager_time(&mut self, id: usize, when: Instant) {
        let i = self.list.iter().position(|x| x.id == id);

        if let Some(i) = i {
            self.list[i].stop_task_manager_time = Some(when);
        }
    }

    #[allow(unused)]
    pub fn set_stop_refcount_time(&mut self, id: usize, when: Instant) {
        let i = self.list.iter().position(|x| x.id == id);

        if let Some(i) = i {
            self.list[i].stop_refcount_time.set(Some(when));
        }
    }

    pub fn running_chains(&self) -> Vec<&SpawnedContainer> {
        self.list
            .iter()
            .filter(|container| !container.is_stopped())
            .collect()
    }

    #[allow(unused)]
    pub fn truncate_old(&mut self, new_len: usize) {
        if self.list.len() <= new_len {
            return;
        }

        let idx_new_first_element = self.list.len() - new_len;
        self.list.drain(0..idx_new_first_element);
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

            if container.is_stopped() {
                to_retain -= 1;
                false
            } else {
                true
            }
        });

        if self.list.len() <= new_len {
            Ok(())
        } else {
            Err(())
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate() {
        let mut monitor = SpawnedContainersMonitor::default();
        let default_container = || SpawnedContainer {
            id: Default::default(),
            para_id: Default::default(),
            start_time: Instant::now(),
            stop_signal_time: Default::default(),
            stop_task_manager_time: Default::default(),
            stop_refcount_time: Default::default(),
            backend: Default::default(),
            client: Default::default(),
        };

        // Truncating empty list does not panic
        monitor.truncate_old(0);
        monitor.truncate_old_stopped_chains(0).unwrap();

        for _ in 0..20 {
            monitor.push(default_container());
        }

        assert_eq!(monitor.list.len(), 20);
        assert_eq!(monitor.count, 20);

        monitor.truncate_old(15);
        assert_eq!(monitor.list.len(), 15);
        assert_eq!(monitor.count, 20);
        // Truncate should remove the oldest stopped chains, so the first id is now 5
        assert_eq!(monitor.list.front().map(|x| x.id), Some(5));

        // We are using Default::default which has a refcount of 0, so all chains are considered stopped
        assert!(monitor.list.iter().all(|x| x.is_stopped()));
        monitor.truncate_old_stopped_chains(10).unwrap();
        assert_eq!(monitor.list.len(), 10);
        assert_eq!(monitor.count, 20);
        // Truncate should remove the oldest stopped chains, so the first id is now 10
        assert_eq!(monitor.list.front().map(|x| x.id), Some(10));
    }
}
