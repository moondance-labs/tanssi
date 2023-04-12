#![cfg_attr(not(feature = "std"), no_std)]

use parity_scale_codec::{Decode, Encode};
use scale_info::prelude::collections::BTreeMap;
use sp_std::vec::Vec;

#[derive(Clone, Encode, Decode, PartialEq, sp_core::RuntimeDebug, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct AssignedCollators<AccountId> {
    pub orchestrator_chain: Vec<AccountId>,
    pub container_chains: BTreeMap<u32, Vec<AccountId>>,
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
    pub fn para_id_of(&self, x: &AccountId, orchestrator_chain_para_id: u32) -> Option<u32> {
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
        self.para_id_of(x, 0).is_some()
    }

    pub fn remove_container_chains_not_in_list(&mut self, container_chains: &[u32]) {
        self.container_chains
            .retain(|id, _cs| container_chains.contains(id));
    }

    pub fn remove_collators_not_in_list(&mut self, collators: &[AccountId]) {
        self.orchestrator_chain.retain(|c| collators.contains(c));
        for (_id, cs) in self.container_chains.iter_mut() {
            cs.retain(|c| collators.contains(c))
        }
    }

    pub fn remove_orchestrator_chain_excess_collators(&mut self, num_orchestrator_chain: usize) {
        self.orchestrator_chain.truncate(num_orchestrator_chain);
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

    pub fn add_new_container_chains(&mut self, container_chains: &[u32]) {
        for para_id in container_chains {
            self.container_chains.entry(*para_id).or_default();
        }
    }
}

pub const COLLATOR_ASSIGNMENT_INDEX: &[u8] =
    &hex_literal::hex!["4a97b7c32fd2bcd103026654b3408079170f16afec7d161bc6acec3964492a0c"];
