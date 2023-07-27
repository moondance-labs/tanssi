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
    crate::{
        pools::{AutoCompounding, ManualRewards, Pool},
        traits::ErrAdd,
        Candidate, CollatorSet, Config, Error, Event, MaxCollatorSetSize, Pallet, Pools, PoolsKey,
        SortedEligibleCandidates, Stake,
    },
    core::cmp::Ordering,
    parity_scale_codec::{Decode, Encode},
    scale_info::TypeInfo,
    sp_core::{Get, RuntimeDebug},
    sp_runtime::traits::Zero,
    sp_std::collections::btree_set::BTreeSet,
};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

/// Eligible candidate with its stake.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(RuntimeDebug, PartialEq, Eq, Encode, Decode, Clone, TypeInfo)]
pub struct EligibleCandidate<C, S> {
    candidate: C,
    stake: S,
}

impl<C: Ord, S: Ord> Ord for EligibleCandidate<C, S> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.stake
            .cmp(&other.stake)
            .reverse()
            .then(self.candidate.cmp(&self.candidate))
    }
}

impl<C: Ord, S: Ord> PartialOrd for EligibleCandidate<C, S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn update_candidate_stake<T: Config>(
    candidate: Candidate<T>,
    new_stake: Stake<T>,
) -> Result<(), Error<T>> {
    let stake_before = Pools::<T>::get(&candidate, &PoolsKey::<T>::CandidateTotalStake);
    Pools::<T>::set(&candidate, &PoolsKey::<T>::CandidateTotalStake, new_stake.0);

    // Compute self delegation.
    let ac_self = if AutoCompounding::<T>::shares_supply(&candidate).0.is_zero() {
        Zero::zero()
    } else {
        let shares = AutoCompounding::shares(&candidate, &candidate);
        AutoCompounding::shares_to_stake(&candidate, shares)?.0
    };

    let mr_self = if ManualRewards::<T>::shares_supply(&candidate).0.is_zero() {
        Zero::zero()
    } else {
        let shares = ManualRewards::shares(&candidate, &candidate);
        ManualRewards::shares_to_stake(&candidate, shares)?.0
    };

    // TODO: Count joining stake?
    let self_delegation = ac_self.err_add(&mr_self)?;

    SortedEligibleCandidates::<T>::mutate(|list| {
        // Remove old data if it exists.
        let old_position = match list.binary_search(&EligibleCandidate {
            candidate: candidate.clone(),
            stake: stake_before,
        }) {
            Ok(pos) => {
                let _ = list.remove(pos);
                Some(pos as u32)
            }
            Err(_) => None,
        };

        // Find new position in the sorted list.
        // It will not be inserted if under the minimum self delegation.
        let new_position = if self_delegation >= T::MinimumSelfDelegation::get() {
            let entry = EligibleCandidate {
                candidate: candidate.clone(),
                stake: new_stake.0,
            };

            let pos = list
                .binary_search(&entry)
                .expect_err("Candidate should be present at most once in the list.");
            list.insert(pos, entry);
            Some(pos as u32)
        } else {
            None
        };

        // If candidate was or is now in the top we need to update
        // the collator set.
        let set_size = MaxCollatorSetSize::<T>::get();
        match (old_position, new_position) {
            (Some(pos), _) | (_, Some(pos)) if pos < set_size => {
                let set: BTreeSet<_> = list
                    .iter()
                    .take(set_size as usize)
                    .map(|c| c.candidate.clone())
                    .collect();
                CollatorSet::<T>::put(set);
            }
            _ => (),
        }

        Pallet::<T>::deposit_event(Event::<T>::UpdatedCandidatePosition {
            candidate,
            stake: new_stake.0,
            self_delegation,
            before: old_position,
            after: new_position,
        });
    });

    Ok(())
}
