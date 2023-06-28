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

#![cfg_attr(not(feature = "std"), no_std)]

use {
    core::mem,
    parity_scale_codec::{Decode, Encode},
    scale_info::prelude::collections::BTreeMap,
    sp_std::{
        collections::vec_deque::VecDeque,
        // This must be separate from vec::Vec because it imports the vec! macro
        vec,
        vec::Vec,
    },
    tp_traits::ParaId,
};

#[derive(Clone, Encode, Decode, PartialEq, sp_core::RuntimeDebug, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct AssignedCollators<AccountId> {
    pub orchestrator_chain: Vec<AccountId>,
    pub container_chains: BTreeMap<ParaId, Vec<AccountId>>,
}

// Manual default impl that does not require AccountId: Default
impl<AccountId> Default for AssignedCollators<AccountId> {
    fn default() -> Self {
        Self {
            orchestrator_chain: Default::default(),
            container_chains: Default::default(),
        }
    }
}

impl<AccountId> AssignedCollators<AccountId>
where
    AccountId: PartialEq,
{
    pub fn para_id_of(&self, x: &AccountId, orchestrator_chain_para_id: ParaId) -> Option<ParaId> {
        for (id, cs) in self.container_chains.iter() {
            if cs.contains(x) {
                return Some(*id);
            }
        }

        if self.orchestrator_chain.contains(x) {
            return Some(orchestrator_chain_para_id);
        }

        None
    }

    pub fn find_collator(&self, x: &AccountId) -> bool {
        self.para_id_of(x, ParaId::from(0)).is_some()
    }

    pub fn remove_container_chains_not_in_list(&mut self, container_chains: &[ParaId]) {
        self.container_chains
            .retain(|id, _cs| container_chains.contains(id));
    }

    pub fn remove_collators_not_in_list(&mut self, collators: &[AccountId]) {
        self.orchestrator_chain.retain(|c| collators.contains(c));
        for (_id, cs) in self.container_chains.iter_mut() {
            cs.retain(|c| collators.contains(c))
        }
    }

    pub fn remove_orchestrator_chain_excess_collators(
        &mut self,
        num_orchestrator_chain: usize,
    ) -> Vec<AccountId> {
        if num_orchestrator_chain <= self.orchestrator_chain.len() {
            self.orchestrator_chain.split_off(num_orchestrator_chain)
        } else {
            vec![]
        }
    }

    pub fn remove_container_chain_excess_collators(&mut self, num_each_container_chain: usize) {
        for (_id, cs) in self.container_chains.iter_mut() {
            cs.truncate(num_each_container_chain);
        }
    }

    pub fn fill_orchestrator_chain_collators<I>(
        &mut self,
        num_orchestrator_chain: usize,
        next_collator: &mut I,
    ) where
        I: Iterator<Item = AccountId>,
    {
        while self.orchestrator_chain.len() < num_orchestrator_chain {
            if let Some(nc) = next_collator.next() {
                self.orchestrator_chain.push(nc);
            } else {
                return;
            }
        }
    }

    pub fn fill_container_chain_collators<I>(
        &mut self,
        num_each_container_chain: usize,
        next_collator: &mut I,
    ) where
        I: Iterator<Item = AccountId>,
    {
        for (_id, cs) in self.container_chains.iter_mut() {
            while cs.len() < num_each_container_chain {
                if let Some(nc) = next_collator.next() {
                    cs.push(nc);
                } else {
                    return;
                }
            }
        }
    }

    pub fn add_new_container_chains(&mut self, container_chains: &[ParaId]) {
        for para_id in container_chains {
            self.container_chains.entry(*para_id).or_default();
        }
    }

    /// Check container chains and remove all collators from container chains
    /// that do not reach the target number of collators. Reassign those to other
    /// container chains.
    ///
    /// Returns the collators that could not be assigned to any container chain,
    /// those can be assigned to the orchestrator chain by the caller.
    pub fn reorganize_incomplete_container_chains_collators(
        &mut self,
        num_each_container_chain: usize,
    ) -> Vec<AccountId> {
        let mut incomplete_container_chains: VecDeque<_> = VecDeque::new();

        for (para_id, collators) in self.container_chains.iter_mut() {
            if !collators.is_empty() && collators.len() < num_each_container_chain {
                // Do not remove the para_id from the map, instead replace the list of
                // collators with an empty vec using mem::take.
                // This is to ensure that the UI shows "1001: []" when a container chain
                // has zero assigned collators.
                let removed_collators = mem::take(collators);
                incomplete_container_chains.push_back((*para_id, removed_collators));
            }
        }

        incomplete_container_chains
            .make_contiguous()
            .sort_by_key(|(_para_id, collators)| collators.len());

        // The first element in `incomplete_container_chains` will be the para_id with lowest
        // non-zero number of collators, we want to move those collators to the para_id with
        // most collators
        while let Some((_para_id, mut collators_min_chain)) =
            incomplete_container_chains.pop_front()
        {
            while !collators_min_chain.is_empty() {
                match incomplete_container_chains.back_mut() {
                    Some(back) => {
                        back.1.push(collators_min_chain.pop().unwrap());
                        if back.1.len() == num_each_container_chain {
                            // Container chain complete, remove from incomplete list and insert into self
                            let (completed_para_id, completed_collators) =
                                incomplete_container_chains.pop_back().unwrap();
                            self.container_chains
                                .insert(completed_para_id, completed_collators);
                        }
                    }
                    None => {
                        return collators_min_chain;
                    }
                }
            }
        }

        vec![]
    }

    pub fn map<T, F>(&self, mut f: F) -> AssignedCollators<T>
    where
        F: FnMut(&AccountId) -> T,
    {
        let mut a = AssignedCollators {
            orchestrator_chain: self.orchestrator_chain.iter().map(&mut f).collect(),
            ..Default::default()
        };

        for (para_id, collators) in self.container_chains.iter() {
            let a_collators = collators.iter().map(&mut f).collect();
            a.container_chains.insert(*para_id, a_collators);
        }

        a
    }
}
