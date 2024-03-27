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
    sp_std::{
        cmp,
        collections::{btree_map::BTreeMap, btree_set::BTreeSet},
        marker::PhantomData,
        mem,
        vec::Vec,
    },
    tp_traits::{ParaId, RemoveInvulnerables as RemoveInvulnerablesT},
};

// Separate import of `sp_std::vec!` macro, which cause issues with rustfmt if grouped
// with `sp_std::vec::Vec`.
use sp_std::vec;

/// Helper methods to implement collator assignment algorithm
pub struct Assignment<T>(PhantomData<T>);

impl<T> Assignment<T>
where
    T: crate::Config,
{
    /// Recompute collator assignment from scratch. If the list of collators and the list of
    /// container chains are shuffled, this returns a random assignment.
    pub fn assign_collators_rotate_all<TShuffle>(
        collators: Vec<T::AccountId>,
        orchestrator_chain: ChainNumCollators,
        chains: Vec<ChainNumCollators>,
        shuffle: Option<TShuffle>,
    ) -> Result<AssignedCollators<T::AccountId>, AssignmentError>
    where
        TShuffle: FnOnce(&mut Vec<T::AccountId>),
    {
        // This is just the "always_keep_old" algorithm but with an empty "old"
        let old_assigned = Default::default();

        Self::assign_collators_always_keep_old(
            collators,
            orchestrator_chain,
            chains,
            old_assigned,
            shuffle,
        )
    }

    /// Assign new collators to missing container_chains.
    /// Old collators always have preference to remain on the same chain.
    /// If there are no missing collators, nothing is changed.
    ///
    /// `chains` should be shuffled or at least rotated on every session to ensure
    /// a fair distribution, because the order of that list affects container chain priority:
    /// the first chain on that list will be the first one to get new collators.
    ///
    /// Similarly, in the `collators` list order means priority, the first collators will be more
    /// likely to get assigned. Unlike the list of `chains` which should already be shuffled,
    /// collators will be shuffled using the `shuffle` callback when needed. This allows the
    /// algorithm to truncate the list of collators and only shuffle the first N. This ensures that
    /// shuffling doesn't cause a collator with low priority to be assigned instead of a collator
    /// with higher priority.
    pub fn assign_collators_always_keep_old<TShuffle>(
        collators: Vec<T::AccountId>,
        orchestrator_chain: ChainNumCollators,
        mut chains: Vec<ChainNumCollators>,
        mut old_assigned: AssignedCollators<T::AccountId>,
        shuffle: Option<TShuffle>,
    ) -> Result<AssignedCollators<T::AccountId>, AssignmentError>
    where
        TShuffle: FnOnce(&mut Vec<T::AccountId>),
    {
        if collators.is_empty() {
            return Err(AssignmentError::ZeroCollators);
        }
        // The rest of this function mostly treats orchestrator chain as another container chain, so move it into
        // `old_assigned.container_chains`
        let old_orchestrator_assigned = mem::take(&mut old_assigned.orchestrator_chain);
        old_assigned
            .container_chains
            .insert(orchestrator_chain.para_id, old_orchestrator_assigned);
        let mut old_assigned = old_assigned.container_chains;
        // Orchestrator chain must be the first one in the list because it always has priority
        chains.insert(0, orchestrator_chain);
        let all_para_ids: Vec<ParaId> = chains.iter().map(|cc| cc.para_id).collect();
        let collators_set = BTreeSet::from_iter(collators.iter().cloned());
        let chains_with_collators =
            Self::select_chains_with_collators(collators.len() as u32, &chains);
        let chains_with_collators_set: BTreeSet<ParaId> = chains_with_collators
            .iter()
            .map(|(para_id, _num_collators)| *para_id)
            .collect();
        Self::retain_valid_old_assigned(
            &mut old_assigned,
            &chains_with_collators_set,
            &collators_set,
        );

        // Ensure the first `min_orchestrator_collators` of orchestrator chain are invulnerables
        Self::prioritize_invulnerables(&collators, orchestrator_chain, &mut old_assigned);

        let new_assigned_chains =
            Self::assign_full(collators, chains_with_collators, old_assigned, shuffle)?;

        let mut new_assigned = AssignedCollators {
            container_chains: new_assigned_chains,
            ..Default::default()
        };

        // Add container chains with 0 collators so that they are shown in UI
        for para_id in all_para_ids {
            new_assigned.container_chains.entry(para_id).or_default();
        }

        // The rest of this function mostly treats orchestrator chain as another container chain, remove it from
        // container chains before returning the final assignment.
        let orchestrator_assigned = new_assigned
            .container_chains
            .remove(&orchestrator_chain.para_id)
            .unwrap();
        // Sanity check to avoid bricking orchestrator chain
        if orchestrator_assigned.is_empty() {
            return Err(AssignmentError::EmptyOrchestrator);
        }
        new_assigned.orchestrator_chain = orchestrator_assigned;

        Ok(new_assigned)
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
    /// # Params
    ///
    /// The first item of `chains` should be the orchestrator chain, because it will be the first one to be assigned
    /// collators.
    ///
    /// # Returns
    ///
    /// A list of `(para_id, num_collators)`.
    pub fn select_chains_with_collators(
        num_collators: u32,
        chains: &[ChainNumCollators],
    ) -> Vec<(ParaId, u32)> {
        if chains.is_empty() {
            // Avoid panic if chains is empty
            return vec![];
        }
        // Let's count how many container chains we can support with the current number of collators
        let mut available_collators = num_collators;
        // Handle orchestrator chain in a special way, we always want to assign collators to it, even if we don't
        // reach the min.
        let min_orchestrator_collators = chains[0].min_collators;
        available_collators = available_collators.saturating_sub(min_orchestrator_collators);

        let mut container_chains_with_collators = vec![chains[0]];
        // Skipping orchestrator chain because it was handled above
        for cc in chains.iter().skip(1) {
            if available_collators >= cc.min_collators {
                available_collators -= cc.min_collators;
                container_chains_with_collators.push(*cc);
            } else if available_collators == 0 {
                // Do not break if there are still some available collators. Even if they were not enough to reach the
                // `min` of this chain, it is possible that one of the chains with less priority has a lower `min`, so
                // that chain should be assigned collators.
                break;
            }
        }

        let mut required_collators_min = 0;
        for cc in &container_chains_with_collators {
            required_collators_min += cc.min_collators;
        }

        if num_collators < min_orchestrator_collators {
            // Edge case: num collators less than min orchestrator collators: fill as much as we can
            vec![(chains[0].para_id, num_collators)]
        } else {
            // After assigning the min to all the chains we have this remainder. The remainder will be assigned until
            // all the chains reach the max value.
            let mut required_collators_remainder = num_collators - required_collators_min;
            let mut container_chains_variable = vec![];
            for cc in &container_chains_with_collators {
                // Each chain will have `min + extra` collators, where extra is capped so `min + extra <= max`.
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
        orchestrator_chain: ChainNumCollators,
        old_assigned: &mut BTreeMap<ParaId, Vec<T::AccountId>>,
    ) -> Vec<T::AccountId> {
        // TODO: clean this up, maybe change remove_invulnerables trait into something more ergonomic
        let min_orchestrator_collators = orchestrator_chain.min_collators as usize;
        let invulnerables_already_assigned = T::RemoveInvulnerables::remove_invulnerables(
            &mut old_assigned
                .get(&orchestrator_chain.para_id)
                .cloned()
                .unwrap_or_default(),
            min_orchestrator_collators,
        );
        let mut new_invulnerables = invulnerables_already_assigned;
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
        let invulnerables_not_assigned = T::RemoveInvulnerables::remove_invulnerables(
            &mut new_collators,
            num_missing_invulnerables,
        );
        new_invulnerables.extend(invulnerables_not_assigned);

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
        let invulnerables_assigned_elsewhere =
            T::RemoveInvulnerables::remove_invulnerables(&mut collators, num_missing_invulnerables);

        if invulnerables_assigned_elsewhere.is_empty() {
            // If at this point we still do not have enough invulnerables, it means that there are no
            // enough invulnerables, so no problem, but return the invulnerables
            return new_invulnerables;
        }

        new_invulnerables.extend(invulnerables_assigned_elsewhere.iter().cloned());

        // In this case we must delete the old assignment of the invulnerables
        let reassigned_invulnerables_set = BTreeSet::from_iter(invulnerables_assigned_elsewhere);
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
    /// The number of invulnerables assigned to the orchestrator chain, capped to `min_collators`.
    pub fn prioritize_invulnerables(
        collators: &[T::AccountId],
        orchestrator_chain: ChainNumCollators,
        old_assigned: &mut BTreeMap<ParaId, Vec<T::AccountId>>,
    ) -> usize {
        let new_invulnerables =
            Self::remove_invulnerables(collators, orchestrator_chain, old_assigned);

        if !new_invulnerables.is_empty() {
            Self::insert_invulnerables(
                old_assigned.entry(orchestrator_chain.para_id).or_default(),
                &new_invulnerables,
            );
        }

        new_invulnerables.len()
    }

    /// Assign collators assuming that the number of collators is greater than or equal to the required.
    /// The order of both container chains and collators is important to ensure randomness when `old_assigned` is
    /// empty.
    ///
    /// # Params
    ///
    /// * `old_assigned` does not need to be a subset of `collators`: collators are checked and removed.
    /// * `old_assigned` does not need to be a subset of `chains`, unused para ids are removed. Collators
    /// assigned to a para_id not present in `chains` may be reassigned to another para_id.
    /// * `chains` `num_collators` can be 0. In that case an empty vec is returned for that para id.
    /// * `old_assigned` must not have duplicate collators.
    /// * `shuffle` is used to shuffle the list collators. The list will be truncated to only have
    /// the number of required collators, to ensure that shuffling doesn't cause a collator with low
    /// priority to be assigned instead of a collator with higher priority.
    ///
    /// # Returns
    ///
    /// The collator assigment, a map from `ParaId` to `Vec<T>`.
    ///
    /// Or an error if the number of collators is not enough to fill all the chains, or if the required number
    /// of collators overflows a `u32`.
    pub fn assign_full<TShuffle>(
        collators: Vec<T::AccountId>,
        chains: Vec<(ParaId, u32)>,
        mut old_assigned: BTreeMap<ParaId, Vec<T::AccountId>>,
        shuffle: Option<TShuffle>,
    ) -> Result<BTreeMap<ParaId, Vec<T::AccountId>>, AssignmentError>
    where
        TShuffle: FnOnce(&mut Vec<T::AccountId>),
    {
        let mut required_collators = 0usize;
        for (_para_id, num_collators) in chains.iter() {
            let num_collators =
                usize::try_from(*num_collators).map_err(|_| AssignmentError::NotEnoughCollators)?;
            required_collators = required_collators
                .checked_add(num_collators)
                .ok_or(AssignmentError::NotEnoughCollators)?;
        }

        // This check is necessary to ensure priority: if the number of collators is less than required, it is
        // possible that the chain with the least priority could be assigned collators (since they are in
        // old_assigned), while some chains with higher priority might have no collators.
        if collators.len() < required_collators {
            return Err(AssignmentError::NotEnoughCollators);
        }
        // We checked that the sum of all `num_collators` fits in `usize`, so we can safely use `as usize`.

        // Remove invalid collators and para ids from `old_assigned`
        let para_ids_set =
            BTreeSet::from_iter(chains.iter().map(|(para_id, _num_collators)| *para_id));
        let collators_set = BTreeSet::from_iter(collators.iter().cloned());
        Self::retain_valid_old_assigned(&mut old_assigned, &para_ids_set, &collators_set);

        // Truncate num collators to required
        for (para_id, num_collators) in chains.iter() {
            let entry = old_assigned.entry(*para_id).or_default();
            entry.truncate(*num_collators as usize);
        }

        // Count number of needed new collators. This is equivalent to:
        // `required_collators - old_assigned.iter().map(|cs| cs.len()).sum()`.
        let mut needed_new_collators = 0;
        for (para_id, num_collators) in chains.iter() {
            let cs = old_assigned.entry(*para_id).or_default();
            needed_new_collators += (*num_collators as usize).saturating_sub(cs.len());
        }

        let assigned_collators: BTreeSet<T::AccountId> = old_assigned
            .iter()
            .flat_map(|(_para_id, para_collators)| para_collators.iter().cloned())
            .collect();

        // Truncate list of new_collators to `needed_new_collators` and shuffle it.
        // This has the effect of keeping collator priority (the first collator of that list is more
        // likely to be assigned to a chain than the last collator of that list), while also
        // ensuring randomness (the original order does not directly affect which chain the
        // collators are assigned to).
        let mut new_collators: Vec<_> = collators
            .into_iter()
            .filter(|x| {
                // Keep collators not already assigned
                !assigned_collators.contains(x)
            })
            .take(needed_new_collators)
            .collect();
        if let Some(shuffle) = shuffle {
            shuffle(&mut new_collators);
        }
        let mut new_collators = new_collators.into_iter();

        // Fill missing collators
        for (para_id, num_collators) in chains.iter() {
            let cs = old_assigned.entry(*para_id).or_default();

            while cs.len() < *num_collators as usize {
                // This error should never happen because we calculated `needed_new_collators`
                // using the same algorithm
                let nc = new_collators
                    .next()
                    .ok_or(AssignmentError::NotEnoughCollators)?;
                cs.push(nc);
            }
        }

        Ok(old_assigned)
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
        new_assigned.extend(mem::take(assigned));

        *assigned = new_assigned;
    }

    /// Removes invalid entries from `old_assigned`:
    ///
    /// * para ids not in `chains_with_collators`
    /// * collators not in `collators`
    pub fn retain_valid_old_assigned(
        old_assigned: &mut BTreeMap<ParaId, Vec<T::AccountId>>,
        chains_with_collators: &BTreeSet<ParaId>,
        collators: &BTreeSet<T::AccountId>,
    ) {
        // old_assigned.remove_container_chains_not_in_set
        old_assigned.retain(|id, _cs| chains_with_collators.contains(id));
        // old_assigned.remove_collators_not_in_set
        for (_id, cs) in old_assigned.iter_mut() {
            cs.retain(|c| collators.contains(c));
        }
    }
}

/// Errors than can happen during collator assignment
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssignmentError {
    /// An empty list of collators was passed to `assign_collators_always_keep_old`
    ZeroCollators,
    /// The required number of collators for `assign_full` is greater than the provided number of collators.
    /// Also includes possible overflows in number of collators.
    NotEnoughCollators,
    /// No collators were assigned to orchestrator chain
    EmptyOrchestrator,
}

/// A `ParaId` and a range of collators that need to be assigned to it.
/// This can be a container chain, a parathread, or the orchestrator chain.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ChainNumCollators {
    pub para_id: ParaId,
    pub min_collators: u32,
    // This will only be filled if all the other min have been reached
    pub max_collators: u32,
}
