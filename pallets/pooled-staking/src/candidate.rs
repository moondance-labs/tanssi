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
        pools::{self, Pool},
        traits::IsCandidateEligible,
        Candidate, Config, Error, Event, Pallet, Pools, PoolsKey, SortedEligibleCandidates, Stake,
    },
    core::{cmp::Ordering, marker::PhantomData},
    parity_scale_codec::{Decode, Encode},
    scale_info::TypeInfo,
    sp_core::{Get, RuntimeDebug},
    sp_runtime::traits::Zero,
    tp_maths::{ErrAdd, ErrSub},
};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

/// Eligible candidate with its stake.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(RuntimeDebug, PartialEq, Eq, Encode, Decode, Clone, TypeInfo)]
pub struct EligibleCandidate<C, S> {
    pub candidate: C,
    pub stake: S,
}

impl<C: Ord, S: Ord> Ord for EligibleCandidate<C, S> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.stake
            .cmp(&other.stake)
            .reverse()
            .then_with(|| self.candidate.cmp(&other.candidate))
    }
}

impl<C: Ord, S: Ord> PartialOrd for EligibleCandidate<C, S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Candidates<T>(PhantomData<T>);

impl<T: Config> Candidates<T> {
    pub fn total_stake(candidate: &Candidate<T>) -> Stake<T::Balance> {
        Stake(Pools::<T>::get(candidate, &PoolsKey::CandidateTotalStake))
    }

    pub fn add_total_stake(
        candidate: &Candidate<T>,
        stake: &Stake<T::Balance>,
    ) -> Result<(), Error<T>> {
        if stake.0.is_zero() {
            return Ok(());
        }

        let new_stake = Self::total_stake(candidate).0.err_add(&stake.0)?;

        Pallet::<T>::deposit_event(Event::<T>::IncreasedStake {
            candidate: candidate.clone(),
            stake_diff: stake.0,
        });

        Self::update_total_stake(candidate, Stake(new_stake))?;

        Ok(())
    }

    pub fn sub_total_stake(
        candidate: &Candidate<T>,
        stake: Stake<T::Balance>,
    ) -> Result<(), Error<T>> {
        if stake.0.is_zero() {
            return Ok(());
        }

        let new_stake = Self::total_stake(candidate).0.err_sub(&stake.0)?;

        Pallet::<T>::deposit_event(Event::<T>::DecreasedStake {
            candidate: candidate.clone(),
            stake_diff: stake.0,
        });

        Self::update_total_stake(candidate, Stake(new_stake))?;

        Ok(())
    }

    pub fn update_total_stake(
        candidate: &Candidate<T>,
        new_stake: Stake<T::Balance>,
    ) -> Result<(), Error<T>> {
        let stake_before = Pools::<T>::get(candidate, &PoolsKey::CandidateTotalStake);
        Pools::<T>::set(candidate, &PoolsKey::CandidateTotalStake, new_stake.0);

        // Compute self delegation.
        let ac_self = if pools::AutoCompounding::<T>::shares_supply(candidate)
            .0
            .is_zero()
        {
            Zero::zero()
        } else {
            let shares = pools::AutoCompounding::<T>::shares(candidate, candidate);
            pools::AutoCompounding::shares_to_stake(candidate, shares)?.0
        };

        let mr_self = if pools::ManualRewards::<T>::shares_supply(candidate)
            .0
            .is_zero()
        {
            Zero::zero()
        } else {
            let shares = pools::ManualRewards::<T>::shares(candidate, candidate);
            pools::ManualRewards::shares_to_stake(candidate, shares)?.0
        };

        let joining_self = if pools::Joining::<T>::shares_supply(candidate).0.is_zero() {
            Zero::zero()
        } else {
            let shares = pools::Joining::<T>::shares(candidate, candidate);
            pools::Joining::shares_to_stake(candidate, shares)?.0
        };

        let self_delegation = ac_self.err_add(&mr_self)?.err_add(&joining_self)?;

        let mut list = SortedEligibleCandidates::<T>::get();

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

        let eligible = self_delegation >= T::MinimumSelfDelegation::get()
            && T::EligibleCandidatesFilter::is_candidate_eligible(candidate);

        // Find new position in the sorted list.
        // It will not be inserted if under the minimum self delegation.
        let new_position = if eligible {
            let entry = EligibleCandidate {
                candidate: candidate.clone(),
                stake: new_stake.0,
            };

            // Candidate should not appear in the list, we're instead searching where
            // to insert it.
            let Err(pos) = list.binary_search(&entry) else {
                return Err(Error::<T>::InconsistentState);
            };

            if pos >= T::EligibleCandidatesBufferSize::get() as usize {
                None
            } else {
                // Insert in correct position then truncate the list if necessary.
                list = list
                    .try_mutate(move |list| {
                        list.insert(pos, entry.clone());
                        list.truncate(T::EligibleCandidatesBufferSize::get() as usize)
                    })
                    // This should not occur as we truncate the list above.
                    .ok_or(Error::<T>::InconsistentState)?;

                Some(pos as u32)
            }
        } else {
            None
        };

        Pallet::<T>::deposit_event(Event::<T>::UpdatedCandidatePosition {
            candidate: candidate.clone(),
            stake: new_stake.0,
            self_delegation,
            before: old_position,
            after: new_position,
        });

        SortedEligibleCandidates::<T>::set(list);

        Ok(())
    }
}
