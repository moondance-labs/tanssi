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
    DelegatorCandidateSummaries, DelegatorCandidateSummary, Pallet, PausePoolsExtrinsics, PoolKind,
    Pools, PoolsKey,
};
use core::marker::PhantomData;
use frame_support::{
    migrations::{MigrationId, SteppedMigration, SteppedMigrationError},
    pallet_prelude::{StorageVersion, Zero},
    traits::GetStorageVersion,
    weights::WeightMeter,
    BoundedVec, Deserialize, Serialize,
};
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::{ConstU32, Get, MaxEncodedLen, RuntimeDebug};
use sp_runtime::{Saturating, Vec};

const LOG_TARGET: &'static str = "pallet_pooled_staking::migrations::stepped_generate_summaries";
pub const PALLET_MIGRATIONS_ID: &[u8; 21] = b"pallet-pooled-staking";

pub struct MigrationGenerateSummaries<T: Config>(PhantomData<T>);
impl<T: Config> SteppedMigration for MigrationGenerateSummaries<T> {
    type Cursor = MigrationState;
    type Identifier = MigrationId<21>;

    fn id() -> Self::Identifier {
        MigrationId {
            pallet_id: *PALLET_MIGRATIONS_ID,
            version_from: 0,
            version_to: 1,
        }
    }

    fn step(
        cursor: Option<Self::Cursor>,
        meter: &mut WeightMeter,
    ) -> Result<Option<Self::Cursor>, SteppedMigrationError> {
        // Consumes in advance the cost of reading and writing the storage version.
        meter.consume(T::DbWeight::get().reads_writes(1, 1));

        if Pallet::<T>::on_chain_storage_version() != Self::id().version_from as u16 {
            return Ok(None);
        }

        // We make a weight meter with 70% of allowed weight to be extra sure the migration will not
        // cause issues
        let mut meter2 = WeightMeter::with_limit(meter.remaining() * 70 / 100);

        let new_state = stepped_generate_summaries::<
            T,
            DelegatorCandidateSummaries<T>,
            CandidateSummaries<T>,
        >(cursor, &mut meter2)?;
        if new_state.is_none() {
            // migration is finished, we update the on chain storage version
            StorageVersion::new(Self::id().version_to as u16).put::<Pallet<T>>();
        }

        meter.consume(meter2.consumed());
        Ok(new_state)
    }

    #[cfg(feature = "try-runtime")]
    fn pre_upgrade() -> Result<Vec<u8>, sp_runtime::TryRuntimeError> {
        // Can we test it somehow without performing the same process? (which would be useless)
        Ok(Default::default())
    }

    #[cfg(feature = "try-runtime")]
    fn post_upgrade(_state: Vec<u8>) -> Result<(), sp_runtime::TryRuntimeError> {
        Ok(())
    }
}

#[derive(
    RuntimeDebug,
    PartialEq,
    Eq,
    Encode,
    Decode,
    Clone,
    TypeInfo,
    Serialize,
    Deserialize,
    MaxEncodedLen,
)]
pub struct MigrationState {
    /// Last key processed by the migration.
    /// Migration will resume from there on next step.
    last_raw_key: BoundedVec<u8, ConstU32<1024>>,
}

/// Generate summaries from `Pools` into provided `D` and `C`, which can be used both for
/// a migration (where pooled_staking was used before the introduction of summaries) and for
/// try_state (generate the summaries from `Pools` into separate storages, then check they
/// match the versions dynamically maintained).
pub fn stepped_generate_summaries<
    T: Config,
    D: frame_support::storage::StorageDoubleMap<
        Delegator<T>,
        Candidate<T>,
        DelegatorCandidateSummary,
        Query = DelegatorCandidateSummary,
    >,
    C: frame_support::storage::StorageMap<Candidate<T>, CandidateSummary, Query = CandidateSummary>,
>(
    state: Option<MigrationState>,
    meter: &mut WeightMeter,
) -> Result<Option<MigrationState>, SteppedMigrationError> {
    // If there is not yet a state, it means we're starting the migration process.
    // In this case we pause the pools extrinsics
    if state.is_none() {
        log::info!(target: LOG_TARGET, "Starting migration. Pools extrinsics will be paused until the end of the migration.");
        PausePoolsExtrinsics::<T>::put(true);
    } else {
        log::info!(target: LOG_TARGET, "Resuming migration.");
    }

    // One migration step is to read from the Pools iterator (1 read). If it is
    // a XXXShares PoolsKey, it will then read+write
    // an entry from both the candidates and delegators summaries (2 reads+writes).
    // We need to check before performing each step that we can consume that weight.
    let step_weight = T::DbWeight::get().reads_writes(3, 2);

    // If the available weight is less than the cost of 1 step, we'll never be able to migrate.
    if meter.remaining().any_lt(step_weight) {
        return Err(SteppedMigrationError::InsufficientWeight {
            required: step_weight,
        });
    }

    // Documentation if `StorageDoubleMap::iter_keys` warn sabout inserting/deleting keys
    // giving undefined results. While the risk is reduced by pausing the pools extrinsics,
    // it may happen while distributing manual rewards if this is the first time a candidate gets
    // manual rewards distributed (counter goes from absent to non-zero). However by looking at the
    // implementation of those iterators, they iterate the keys by lexicographic order. So even in
    // the case this entry is inserted, it will not affect iterating over the non-explored keys, and
    // we don't care missing reward counter entries since they are not used by the migration.
    //
    // We also prefer to use `iter_keys` over `iter` as it might optimize the POV for keys we skip.
    let mut iterator = match state {
        Some(MigrationState { last_raw_key }) => Pools::<T>::iter_keys_from(last_raw_key.to_vec()),
        None => Pools::<T>::iter_keys(),
    };

    let mut count = 0;

    loop {
        if !meter.can_consume(step_weight) {
            log::warn!(target: LOG_TARGET, "Migration limit reached. Processed {count} delegations.");
            return Ok(Some(MigrationState {
                last_raw_key: iterator
                    .last_raw_key()
                    .to_vec()
                    .try_into()
                    .expect("size should be larger than key length"),
            }));
        }

        meter.consume(T::DbWeight::get().reads_writes(1, 0));

        let Some((candidate, key)) = iterator.next() else {
            break;
        };

        count += 1;
        let (pool, delegator) = match key.clone() {
            PoolsKey::JoiningShares { delegator } => (PoolKind::Joining, delegator),
            PoolsKey::AutoCompoundingShares { delegator } => (PoolKind::AutoCompounding, delegator),
            PoolsKey::ManualRewardsShares { delegator } => (PoolKind::ManualRewards, delegator),
            PoolsKey::LeavingShares { delegator } => (PoolKind::Leaving, delegator),
            _ => continue, // we only care about share values
        };

        let value = Pools::<T>::get(&candidate, key);
        // Are 0/Default values automatically removed from storage?
        // In case they aren't we check the amount of shares isn't 0.
        if value.is_zero() {
            continue;
        }

        // We first modify the delegator summary. If the summary is empty it means we have not yet
        // encountered that delegator for this candidate, so we'll consider it a new delegator that
        // will increase the `delegators` count in the candidate summary.
        let mut new_delegator = false;
        meter.consume(T::DbWeight::get().reads_writes(2, 2));

        D::mutate(&delegator, &candidate, |summary| {
            if summary.is_empty() {
                new_delegator = true;
            }

            summary.set_pool(pool, true);
        });

        C::mutate(&candidate, |summary| {
            if new_delegator {
                summary.delegators.saturating_inc();
            }

            summary.pool_delegators_mut(pool).saturating_inc();
        });
    }

    log::info!(target: LOG_TARGET, "Migration finished. Processed {count} delegations");
    PausePoolsExtrinsics::<T>::put(false);

    Ok(None)
}
