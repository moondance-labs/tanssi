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

use crate::{
    Candidate, CandidateSummaries, CandidateSummary, Config, Delegator,
    DelegatorCandidateSummaries, DelegatorCandidateSummary, PoolKind, Pools, PoolsKey,
};
use frame_support::pallet_prelude::Zero;
use sp_core::Get;
use sp_runtime::Weight;
use sp_std::collections::btree_map::BTreeMap;

pub fn generate_summaries<T: Config>(_available_weight: Weight) -> Weight {
    let mut delegator_summaries =
        BTreeMap::<(Delegator<T>, Candidate<T>), DelegatorCandidateSummary>::new();
    let mut candidate_summaries = BTreeMap::<Candidate<T>, CandidateSummary>::new();

    let mut read_count = 0;
    let mut write_count = 0;

    for (candidate, key, value) in Pools::<T>::iter() {
        read_count += 1;

        let (pool, delegator) = match key {
            PoolsKey::JoiningShares { delegator } => (PoolKind::Joining, delegator),
            PoolsKey::AutoCompoundingShares { delegator } => (PoolKind::AutoCompounding, delegator),
            PoolsKey::ManualRewardsShares { delegator } => (PoolKind::ManualRewards, delegator),
            PoolsKey::LeavingShares { delegator } => (PoolKind::Leaving, delegator),
            _ => continue, // we only care about share values
        };

        // Are 0/Default values automatically from storage?
        // In case they aren't we check the amount of shares isn't 0.
        if value.is_zero() {
            continue;
        }

        let summary = delegator_summaries
            .entry((delegator, candidate.clone()))
            .or_default();
        summary.set_pool(pool, true);

        let summary = candidate_summaries.entry(candidate.clone()).or_default();
        let pool_count = summary.pool_delegators_mut(pool);
        *pool_count = pool_count.saturating_add(1);
    }

    // We now iterate over delegator_summaries to fill candidate_summaries with delegators
    // count without risking having duplicates due to multiple pools.
    for (_delegator, candidate) in delegator_summaries.keys() {
        let summary = candidate_summaries.entry(candidate.clone()).or_default();
        summary.delegators = summary.delegators.saturating_add(1);
    }

    // Now that we have collected all info, we put them in real storage.
    for ((delegator, candidate), summary) in delegator_summaries.into_iter() {
        write_count += 1;
        DelegatorCandidateSummaries::<T>::insert(delegator.clone(), candidate.clone(), summary);
    }

    for (candidate, summary) in candidate_summaries.into_iter() {
        write_count += 1;
        CandidateSummaries::<T>::insert(candidate, summary);
    }

    T::DbWeight::get().reads_writes(read_count, write_count)
}
