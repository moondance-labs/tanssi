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
    dp_collator_assignment::AssignedCollators,
    sp_core::Get,
    sp_std::{
        cmp,
        collections::{btree_map::BTreeMap, btree_set::BTreeSet},
        marker::PhantomData,
        vec,
        vec::Vec,
    },
    tp_traits::{ParaId, RemoveInvulnerables as RemoveInvulnerablesT},
};

/// Helper methods to implement collator assignment algorithm
pub struct Assignment<T>(PhantomData<T>);

impl<T> Assignment<T>
where
    T: crate::Config,
{
    /// Recompute collator assignment from scratch. If the list of collators and the list of
    /// container chains are shuffled, this returns a random assignment.
    pub fn assign_collators_rotate_all(
        collators: Vec<T::AccountId>,
        container_chains: Vec<ContainerChain>,
    ) -> AssignedCollators<T::AccountId> {
        // This is just the "always_keep_old" algorithm but with an empty "old"
        let old_assigned = Default::default();

        Self::assign_collators_always_keep_old(collators, container_chains, old_assigned)
    }

    /// Assign new collators to missing container_chains.
    /// Old collators always have preference to remain on the same chain.
    /// If there are no missing collators, nothing is changed.
    ///
    /// `container_chain_ids` should be shuffled or at least rotated on every session to ensure
    /// a fair distribution, because the order of that list affects container chain priority:
    /// the first container chain on that list will be the first one to get new collators.
    pub fn assign_collators_always_keep_old(
        collators: Vec<T::AccountId>,
        container_chains: Vec<ContainerChain>,
        mut old_assigned: AssignedCollators<T::AccountId>,
    ) -> AssignedCollators<T::AccountId> {
        let all_para_ids: Vec<ParaId> = container_chains.iter().map(|cc| cc.para_id).collect();
        let collators_set = BTreeSet::from_iter(collators.iter().cloned());
        let chains_with_collators =
            Self::select_chains_with_collators(collators.len() as u32, &container_chains);
        let chains_with_collators_set: BTreeSet<ParaId> = chains_with_collators
            .iter()
            .map(|(para_id, _num_collators)| *para_id)
            .collect();
        // The rest of this function mostly treats orchestrator chain as another container chain, so move it into
        // `old_assigned.container_chains`
        let old_orchestrator_assigned = core::mem::take(&mut old_assigned.orchestrator_chain);
        old_assigned
            .container_chains
            .insert(T::SelfParaId::get(), old_orchestrator_assigned);
        let mut old_assigned = old_assigned.container_chains;
        Self::retain_valid_old_assigned(
            &mut old_assigned,
            chains_with_collators_set,
            collators_set,
        );

        // Ensure the first `min_orchestrator_collators` of orchestrator chain are invulnerables
        Self::prioritize_invulnerables(&collators, &container_chains, &mut old_assigned);

        let new_assigned_chains = Self::assign_full(collators, chains_with_collators, old_assigned);
        let mut new_assigned = AssignedCollators::default();
        new_assigned.container_chains = new_assigned_chains;

        // Add container chains with 0 collators so that they are shown in UI
        for para_id in all_para_ids {
            new_assigned.container_chains.entry(para_id).or_default();
        }

        // The rest of this function mostly treats orchestrator chain as another container chain, remove it from
        // container chains before returning the final assignment.
        let orchestrator_assigned = new_assigned
            .container_chains
            .remove(&T::SelfParaId::get())
            .unwrap();
        new_assigned.orchestrator_chain = orchestrator_assigned;

        new_assigned
    }

    /// Select which container chains will be assigned collators and how many collators, but do not specify which
    /// collator goes to which chain.
    ///
    /// Each chain has a min and max number of collators. If the number of collators is not enough to reach the min,
    /// no collators are assigned to that chain.
    ///
    /// If the available number of collators is:
    /// * lower than the min of the first chain: we assign all the collators to the first chain. This is the
    /// orchestrator chain and we always want it to have collators.
    /// * lower than the sum of all the min: we cannot assign collators to all the chains. So remove chains until
    /// we can. The order is important, the first chains will be assigned collators and the last ones will not.
    /// * lower than the sum of all the max: we can assign the min value to all the chains, and have some leftover.
    /// We use the same order to decide where this extra collators will go, by filling the max of the first chain,
    /// then the max of the second chain, and so on.
    /// * greater than the sum of all the max: all the chains will be assigned their max number of collators.
    ///
    /// # Returns
    ///
    /// A list of `(para_id, num_collators)`.
    pub fn select_chains_with_collators(
        num_collators: u32,
        container_chains: &[ContainerChain],
    ) -> Vec<(ParaId, u32)> {
        // Let's count how many container chains we can support with the current number of collators
        let mut available_collators = num_collators;
        // Handle orchestrator chain in a special way, we always want to assign collators to it, even if we don't
        // reach the min.
        assert_eq!(container_chains[0].para_id, T::SelfParaId::get());
        let min_orchestrator_collators = container_chains[0].min_collators;
        available_collators = available_collators.saturating_sub(min_orchestrator_collators);

        let mut container_chains_with_collators = vec![container_chains[0]];
        // Skipping orchestrator chain because it was handled above
        for cc in container_chains.iter().skip(1) {
            if available_collators >= cc.min_collators {
                available_collators -= cc.min_collators;
                container_chains_with_collators.push(*cc);
            } else {
                // Do not break here because we want to push all the remaining para_ids
            }
        }

        let mut required_collators_min = 0;
        for cc in &container_chains_with_collators {
            required_collators_min += cc.min_collators;
        }

        if num_collators < min_orchestrator_collators {
            // Edge case: num collators less than min orchestrator collators: fill as much as we can
            vec![(container_chains[0].para_id, num_collators)]
        } else {
            // Set a variable number of collators, some chains at max and some chains at min
            let mut required_collators_remainder = num_collators - required_collators_min;
            let mut container_chains_variable = vec![];
            for cc in &container_chains_with_collators {
                let extra = cmp::min(
                    required_collators_remainder,
                    cc.max_collators.saturating_sub(cc.min_collators),
                );
                let num = cc.min_collators + extra;
                required_collators_remainder -= extra;
                container_chains_variable.push((cc.para_id, num));
            }

            container_chains_variable
        }
    }

    /// Same as `prioritize_invulnerables` but return the invulnerables instead of inserting them into `old_assigned`.
    ///
    /// Mutates `old_assigned` by removing invulnerables from their old chain, even if they will later be assigned to
    /// the same chain.
    pub fn remove_invulnerables(
        collators: &[T::AccountId],
        container_chains: &[ContainerChain],
        old_assigned: &mut BTreeMap<ParaId, Vec<T::AccountId>>,
    ) -> Vec<T::AccountId> {
        // TODO: clean this up, maybe change remove_invulnerables trait into something more ergonomic
        let min_orchestrator_collators = container_chains[0].min_collators as usize;
        let invulnerables = T::RemoveInvulnerables::remove_invulnerables(
            &mut old_assigned.get(&T::SelfParaId::get()).unwrap().clone(),
            min_orchestrator_collators,
        );
        let mut new_invulnerables = invulnerables;
        if new_invulnerables.len() >= min_orchestrator_collators {
            // We already had invulnerables, we will just move them to the front of the list if they weren't already
            return new_invulnerables;
        }

        // Not enough invulnerables currently assigned, get rest from new_collators
        let mut new_collators = collators.to_vec();
        for (_id, cs) in old_assigned.iter() {
            new_collators.retain(|c| !cs.contains(c));
        }
        let num_missing_invulnerables = min_orchestrator_collators - new_invulnerables.len();
        let missing_invulnerables = T::RemoveInvulnerables::remove_invulnerables(
            &mut new_collators,
            num_missing_invulnerables,
        );
        new_invulnerables.extend(missing_invulnerables);

        if new_invulnerables.len() >= min_orchestrator_collators {
            // Got invulnerables from new_collators, and maybe some were already assigned
            return new_invulnerables;
        }

        // Still not enough invulnerables, try to get an invulnerable that is currently assigned somewhere else
        let num_missing_invulnerables = min_orchestrator_collators - new_invulnerables.len();
        let mut collators = collators.to_vec();
        let new_invulnerables_set = BTreeSet::from_iter(new_invulnerables.iter().cloned());
        collators.retain(|c| {
            // Remove collators already selected
            !new_invulnerables_set.contains(c)
        });
        let reassigned_invulnerables =
            T::RemoveInvulnerables::remove_invulnerables(&mut collators, num_missing_invulnerables);

        if reassigned_invulnerables.is_empty() {
            // If at this point we still do not have enough invulnerables, it means that there are no
            // enough invulnerables, so no problem, but return the invulnerables
            return new_invulnerables;
        }

        let reassigned_invulnerables_set =
            BTreeSet::from_iter(reassigned_invulnerables.iter().cloned());
        new_invulnerables.extend(reassigned_invulnerables);

        // In this case we must delete the old assignment of the invulnerables
        // old_assigned.remove_collators_in_set
        for (_id, cs) in old_assigned.iter_mut() {
            cs.retain(|c| !reassigned_invulnerables_set.contains(c));
        }

        new_invulnerables
    }

    /// Ensure orchestrator chain has `min_orchestrator` invulnerables. If that's not possible, it tries to add as
    /// many invulnerables as possible.
    ///
    /// Get invulnerables from:
    /// * old_assigned in orchestrator
    /// * new collators
    /// * old_assigned elsewhere
    ///
    /// In that order.
    ///
    /// Mutates `old_assigned` because invulnerables will be inserted there, and if invulnerables were already
    /// assigned to some other chain, they will be removed from that other chain as well.
    ///
    /// # Params
    ///
    /// * `old_assigned` must be a subset of `collators`
    /// * `old_assigned` must not have duplicate collators.
    ///
    /// # Returns
    ///
    /// The number of invulnerables, capped to `min_collators`.
    ///
    /// # Panics
    ///
    /// * If `container_chains` is empty, or if the first element of `container_chains` does not have `para_id == SelfParaId::get()`.
    /// * If `old_assigned` does not have an entry for `SelfParaId::get()`.
    pub fn prioritize_invulnerables(
        collators: &[T::AccountId],
        container_chains: &[ContainerChain],
        old_assigned: &mut BTreeMap<ParaId, Vec<T::AccountId>>,
    ) -> usize {
        let new_invulnerables =
            Self::remove_invulnerables(collators, container_chains, old_assigned);
        Self::insert_invulnerables(
            old_assigned.get_mut(&T::SelfParaId::get()).unwrap(),
            &new_invulnerables,
        );

        new_invulnerables.len()
    }

    /// Assign collators assuming that the number of collators is greater than or equal to the required.
    /// The order of both container chains and collators is important to ensure randomness when `old_assigned` is
    /// empty.
    ///
    /// # Params
    ///
    /// * `old_assigned` does not need to be a subset of `collators`: collators are checked and removed.
    /// * `old_assigned` does not need to be a subset of `container_chains`, unused para ids are removed. Collators
    /// assigned to a para_id not present in `container_chains` may be reassigned to another para_id.
    /// * `container_chains` `num_collators` can be 0. In that case an empty vec is returned for that para id.
    /// * `old_assigned` must not have duplicate collators.
    ///
    /// # Returns
    ///
    /// The collator assigment, a map from `ParaId` to `Vec<T>`.
    ///
    /// # Panics
    ///
    /// This function panics if the number of collators is not enough to fill all the container chains.
    pub fn assign_full(
        collators: Vec<T::AccountId>,
        container_chains: Vec<(ParaId, u32)>,
        mut old_assigned: BTreeMap<ParaId, Vec<T::AccountId>>,
    ) -> BTreeMap<ParaId, Vec<T::AccountId>> {
        let mut required_collators = 0;
        for (_para_id, num_collators) in container_chains.iter() {
            required_collators += num_collators;
        }

        // This invariant is necessary to ensure priority: if the number of collators is less than required, it is
        // possible that the chain with the least priority could be assigned collators (since they are in
        // old_assigned), while some chains with higher priority might have no collators.
        assert!(
            collators.len() >= required_collators as usize,
            "assign_full: not enough collators: {}, required {}, chains: {:?}",
            collators.len(),
            required_collators,
            container_chains
        );

        // Remove para_ids not in list
        let para_ids_set = BTreeSet::from_iter(
            container_chains
                .iter()
                .map(|(para_id, _num_collators)| *para_id),
        );
        let collators_set = BTreeSet::from_iter(collators.iter().cloned());

        Self::retain_valid_old_assigned(&mut old_assigned, para_ids_set, collators_set);

        // Truncate num collators to required
        for (para_id, num_collators) in container_chains.iter() {
            let entry = old_assigned.entry(*para_id).or_default();
            entry.truncate(*num_collators as usize);
        }

        // Remove already assigned from `collators`
        let mut new_collators = collators;
        for (_para_id, para_id_collators) in old_assigned.iter() {
            for collator in para_id_collators {
                let idx = new_collators
                    .iter()
                    .position(|x| x == collator)
                    .expect("duplicate collator in old_assigned");
                new_collators.remove(idx);
            }
        }

        let mut next_collator = new_collators.into_iter();

        // Fill missing collators
        for (para_id, num_collators) in container_chains.iter() {
            let cs = old_assigned.entry(*para_id).or_default();

            while cs.len() < *num_collators as usize {
                // unwrap is safe because we checked that `collators.len() >= required_collators`
                let nc = next_collator.next().unwrap();
                cs.push(nc);
            }
        }

        old_assigned
    }

    /// Insert invulnerables ensuring that they are always the first in the list.
    /// The order of both lists is preserved.
    /// `assigned` may already contain the invulnerables, in that case they are only moved to the front.
    ///
    /// Invulnerables need to be the first of the list because we may truncate the list of collators if the number of
    /// collators changes, and in that case we want invulnerables to stay assigned there.
    pub fn insert_invulnerables(assigned: &mut Vec<T::AccountId>, invulnerables: &[T::AccountId]) {
        assigned.retain(|item| !invulnerables.contains(item));

        let mut new_assigned = invulnerables.to_vec();
        new_assigned.extend(assigned.iter().cloned());

        *assigned = new_assigned;
    }

    /// Removes invalid entries from `old_assigned`:
    ///
    /// * para ids not in `chains_with_collators`
    /// * collators not in `collators`
    pub fn retain_valid_old_assigned(
        old_assigned: &mut BTreeMap<ParaId, Vec<T::AccountId>>,
        chains_with_collators: BTreeSet<ParaId>,
        collators: BTreeSet<T::AccountId>,
    ) {
        // old_assigned.remove_container_chains_not_in_set
        old_assigned.retain(|id, _cs| chains_with_collators.contains(id));
        // old_assigned.remove_collators_not_in_set
        for (_id, cs) in old_assigned.iter_mut() {
            cs.retain(|c| collators.contains(c));
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ContainerChain {
    pub para_id: ParaId,
    pub min_collators: u32,
    // This will only be filled if all the other min have been reached
    pub max_collators: u32,
}
