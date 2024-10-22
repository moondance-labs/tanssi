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
    frame_support::{pallet_prelude::*, traits::DefensiveSaturating},
    frame_system::pallet_prelude::*,
    log::log,
    pallet_staking::SessionInterface,
    parity_scale_codec::FullCodec,
    parity_scale_codec::{Decode, Encode},
    sp_runtime::traits::{Convert, Debug, One, Saturating, Zero},
    sp_runtime::DispatchResult,
    sp_runtime::Perbill,
    sp_staking::{
        offence::{OffenceDetails, OnOffenceHandler},
        EraIndex, SessionIndex,
    },
    tp_traits::{EraIndexProvider, OnEraEnd, OnEraStart},
};

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Removed author data
        SlashReported {
            validator: T::ValidatorId,
            fraction: Perbill,
            slash_era: EraIndex,
        },
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// A stable ID for a validator.
        type ValidatorId: Member
            + Parameter
            + MaybeSerializeDeserialize
            + MaxEncodedLen
            + TryFrom<Self::AccountId>;

        /// A conversion from account ID to validator ID.
        ///
        /// Its cost must be at most one storage read.
        type ValidatorIdOf: Convert<Self::AccountId, Option<Self::ValidatorId>>;

        /// Number of eras that slashes are deferred by, after computation.
        ///
        /// This should be less than the bonding duration. Set to 0 if slashes
        /// should be applied immediately, without opportunity for intervention.
        #[pallet::constant]
        type SlashDeferDuration: Get<EraIndex>;

        /// Number of eras that staked funds must remain bonded for.
        #[pallet::constant]
        type BondingDuration: Get<EraIndex>;

        type SlashId: Default
            + FullCodec
            + TypeInfo
            + Copy
            + Clone
            + Debug
            + Eq
            + Saturating
            + One
            + Ord
            + MaxEncodedLen;

        /// Interface for interacting with a session pallet.
        type SessionInterface: SessionInterface<Self::AccountId>;

        type EraIndexProvider: EraIndexProvider;
    }

    #[pallet::error]
    pub enum Error<T> {
        EmptyTargets,
        InvalidSlashIndex,
        NotSortedAndUnique,
        ProvidedFutureEra,
        ProvidedNonSlashableEra,
        ActiveEraNotSet,
        DeferPeriodIsOver,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(PhantomData<T>);

    /// All slashing events on validators, mapped by era to the highest slash proportion
    /// and slash value of the era.
    #[pallet::storage]
    pub(crate) type ValidatorSlashInEra<T: Config> =
        StorageDoubleMap<_, Twox64Concat, EraIndex, Twox64Concat, T::AccountId, Perbill>;

    /// A mapping from still-bonded eras to the first session index of that era.
    ///
    /// Must contains information for eras for the range:
    /// `[active_era - bounding_duration; active_era]`
    #[pallet::storage]
    #[pallet::unbounded]
    pub(crate) type BondedEras<T: Config> =
        StorageValue<_, Vec<(EraIndex, SessionIndex)>, ValueQuery>;

    #[pallet::storage]
    pub type NextSlashId<T: Config> = StorageValue<_, T::SlashId, ValueQuery>;

    /// All unapplied slashes that are queued for later.
    #[pallet::storage]
    #[pallet::unbounded]
    pub type Slashes<T: Config> =
        StorageMap<_, Twox64Concat, EraIndex, Vec<Slash<T::AccountId, T::SlashId>>, ValueQuery>;

    /// The session index at which the era start for the last [`Config::HistoryDepth`] eras.
    ///
    /// Note: This tracks the starting session (i.e. session index when era start being active)
    /// for the eras in `[CurrentEra - HISTORY_DEPTH, CurrentEra]`.
    #[pallet::storage]
    #[pallet::getter(fn eras_start_session_index)]
    pub type ErasStartSessionIndex<T> = StorageMap<_, Twox64Concat, EraIndex, SessionIndex>;

    /// Any validators that may never be slashed or forcibly kicked. It's a Vec since they're
    /// easy to initialize and the performance hit is minimal (we expect no more than four
    /// invulnerables) and restricted to testnets.
    #[pallet::storage]
    #[pallet::getter(fn invulnerables)]
    #[pallet::unbounded]
    pub type Invulnerables<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn cancel_deferred_slash(
            origin: OriginFor<T>,
            era: EraIndex,
            slash_indices: Vec<u32>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            let active_era = T::EraIndexProvider::active_era().index;
            ensure!(
                era >= active_era.saturating_sub(T::SlashDeferDuration::get()),
                Error::<T>::DeferPeriodIsOver
            );

            ensure!(!slash_indices.is_empty(), Error::<T>::EmptyTargets);
            ensure!(
                is_sorted_and_unique(&slash_indices),
                Error::<T>::NotSortedAndUnique
            );

            let mut era_slashes = Slashes::<T>::get(&era);
            let last_item = slash_indices[slash_indices.len() - 1];
            ensure!(
                (last_item as usize) < era_slashes.len(),
                Error::<T>::InvalidSlashIndex
            );

            for (removed, index) in slash_indices.into_iter().enumerate() {
                let index = (index as usize) - removed;
                era_slashes.remove(index);
            }

            Slashes::<T>::insert(&era, &era_slashes);
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn force_inject_slash(
            origin: OriginFor<T>,
            era: EraIndex,
            validator: T::AccountId,
            percentage: Perbill,
        ) -> DispatchResult {
            ensure_root(origin)?;
            let active_era = T::EraIndexProvider::active_era().index;
            ensure!(era <= active_era, Error::<T>::ProvidedFutureEra);

            let window_start = active_era.saturating_sub(T::BondingDuration::get());
            ensure!(era >= window_start, Error::<T>::ProvidedNonSlashableEra);

            let next_slash_id = NextSlashId::<T>::get();

            let slash = Slash::<T::AccountId, T::SlashId> {
                slash_id: next_slash_id,
                reporters: vec![],
                confirmed: false,

                validator,
                percentage,
            };
            let mut era_slashes = Slashes::<T>::get(&era);
            era_slashes.push(slash);
            Slashes::<T>::insert(&era, &era_slashes);
            NextSlashId::<T>::put(next_slash_id.saturating_add(One::one()));
            Ok(())
        }
    }
}

/// This is intended to be used with `FilterHistoricalOffences`.
impl<T: Config>
    OnOffenceHandler<T::AccountId, pallet_session::historical::IdentificationTuple<T>, Weight>
    for Pallet<T>
where
    T: Config<ValidatorId = <T as frame_system::Config>::AccountId>,
    T: pallet_session::Config<ValidatorId = <T as frame_system::Config>::AccountId>,
    T: pallet_session::historical::Config,
    T::SessionHandler: pallet_session::SessionHandler<<T as frame_system::Config>::AccountId>,
    T::SessionManager: pallet_session::SessionManager<<T as frame_system::Config>::AccountId>,
    <T as pallet::Config>::ValidatorIdOf: Convert<
        <T as frame_system::Config>::AccountId,
        Option<<T as frame_system::Config>::AccountId>,
    >,
{
    fn on_offence(
        offenders: &[OffenceDetails<
            T::AccountId,
            pallet_session::historical::IdentificationTuple<T>,
        >],
        slash_fraction: &[Perbill],
        slash_session: SessionIndex,
    ) -> Weight {
        let mut consumed_weight = Weight::from_parts(0, 0);
        let mut add_db_reads_writes = |reads, writes| {
            consumed_weight += T::DbWeight::get().reads_writes(reads, writes);
        };

        let active_era = {
            let active_era = T::EraIndexProvider::active_era().index;
            add_db_reads_writes(1, 0);
            active_era
        };
        let active_era_start_session_index = Self::eras_start_session_index(active_era)
            .unwrap_or_else(|| {
                frame_support::print("Error: start_session_index must be set for current_era");
                0
            });
        add_db_reads_writes(1, 0);

        // Fast path for active-era report - most likely.
        // `slash_session` cannot be in a future active era. It must be in `active_era` or before.
        let slash_era = if slash_session >= active_era_start_session_index {
            active_era
        } else {
            let eras = BondedEras::<T>::get();
            add_db_reads_writes(1, 0);

            // Reverse because it's more likely to find reports from recent eras.
            match eras.iter().rev().find(|&(_, sesh)| sesh <= &slash_session) {
                Some((slash_era, _)) => *slash_era,
                // Before bonding period. defensive - should be filtered out.
                None => return consumed_weight,
            }
        };

        add_db_reads_writes(1, 1);

        let slash_defer_duration = T::SlashDeferDuration::get();

        let invulnerables = Self::invulnerables();
        add_db_reads_writes(1, 0);

        let mut next_slash_id = NextSlashId::<T>::get();

        for (details, slash_fraction) in offenders.iter().zip(slash_fraction) {
            let (stash, _) = &details.offender;

            // Skip if the validator is invulnerable.
            if invulnerables.contains(stash) {
                continue;
            }

            let slash = compute_slash::<T>(
                slash_fraction.clone(),
                next_slash_id,
                slash_era,
                stash.clone(),
                slash_defer_duration,
            );

            Self::deposit_event(Event::<T>::SlashReported {
                validator: stash.clone(),
                fraction: *slash_fraction,
                slash_era,
            });

            if let Some(mut slash) = slash {
                slash.reporters = details.reporters.clone();

                // Defer to end of some `slash_defer_duration` from now.
                log!(
                    log::Level::Debug,
                    "deferring slash of {:?}% happened in {:?} (reported in {:?}) to {:?}",
                    slash_fraction,
                    slash_era,
                    active_era,
                    slash_era + slash_defer_duration + 1,
                );

                Slashes::<T>::mutate(
                    slash_era
                        .saturating_add(slash_defer_duration)
                        .saturating_add(One::one()),
                    move |for_later| for_later.push(slash),
                );

                // Fix unwrap
                next_slash_id = next_slash_id.saturating_add(One::one());
                add_db_reads_writes(1, 1);
            } else {
                add_db_reads_writes(4 /* fetch_spans */, 5 /* kick_out_if_recent */)
            }
        }
        NextSlashId::<T>::put(next_slash_id);
        consumed_weight
    }
}

impl<T: Config> OnEraStart for Pallet<T> {
    fn on_era_start(era_index: EraIndex, session_start: SessionIndex) {
        let bonding_duration = T::BondingDuration::get();

        BondedEras::<T>::mutate(|bonded| {
            bonded.push((era_index, session_start));

            if era_index > bonding_duration {
                let first_kept = era_index.defensive_saturating_sub(bonding_duration);

                // Prune out everything that's from before the first-kept index.
                let n_to_prune = bonded
                    .iter()
                    .take_while(|&&(era_idx, _)| era_idx < first_kept)
                    .count();

                // Kill slashing metadata.
                for (pruned_era, _) in bonded.drain(..n_to_prune) {
                    #[allow(deprecated)]
                    ValidatorSlashInEra::<T>::remove_prefix(&pruned_era, None);
                    #[allow(deprecated)]
                    Slashes::<T>::remove(&pruned_era);
                }

                if let Some(&(_, first_session)) = bonded.first() {
                    T::SessionInterface::prune_historical_up_to(first_session);
                }
            }
        });

        ErasStartSessionIndex::<T>::insert(&era_index, &session_start);

        Self::confirm_unconfirmed_slashes(era_index);
    }
}

impl<T: Config> Pallet<T> {
    /// Apply previously-unapplied slashes on the beginning of a new era, after a delay.
    fn confirm_unconfirmed_slashes(active_era: EraIndex) {
        let mut era_slashes = Slashes::<T>::take(&active_era);
        log!(
            log::Level::Debug,
            "found {} slashes scheduled to be confirmed in era {:?}",
            era_slashes.len(),
            active_era,
        );
        for slash in &mut era_slashes {
            slash.confirmed = true;
        }
        Slashes::<T>::insert(active_era, &era_slashes);
    }
}

/// A pending slash record. The value of the slash has been computed but not applied yet,
/// rather deferred for several eras.
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, Clone, PartialEq)]
pub struct Slash<AccountId, SlashId> {
    /// The stash ID of the offending validator.
    validator: AccountId,
    /// Reporters of the offence; bounty payout recipients.
    reporters: Vec<AccountId>,
    /// The amount of payout.
    slash_id: SlashId,
    percentage: Perbill,
    // Whether the slash is confirmed or still needs to go through deferred period
    confirmed: bool,
}

/// Computes a slash of a validator and nominators. It returns an unapplied
/// record to be applied at some later point. Slashing metadata is updated in storage,
/// since unapplied records are only rarely intended to be dropped.
///
/// The pending slash record returned does not have initialized reporters. Those have
/// to be set at a higher level, if any.
pub(crate) fn compute_slash<T: Config>(
    slash_fraction: Perbill,
    slash_id: T::SlashId,
    slash_era: EraIndex,
    stash: T::AccountId,
    slash_defer_duration: EraIndex,
) -> Option<Slash<T::AccountId, T::SlashId>> {
    let prior_slash_p = ValidatorSlashInEra::<T>::get(&slash_era, &stash).unwrap_or(Zero::zero());

    // compare slash proportions rather than slash values to avoid issues due to rounding
    // error.
    if slash_fraction.deconstruct() > prior_slash_p.deconstruct() {
        ValidatorSlashInEra::<T>::insert(&slash_era, &stash, &slash_fraction);
    } else {
        // we slash based on the max in era - this new event is not the max,
        // so neither the validator or any nominators will need an update.
        //
        // this does lead to a divergence of our system from the paper, which
        // pays out some reward even if the latest report is not max-in-era.
        // we opt to avoid the nominator lookups and edits and leave more rewards
        // for more drastic misbehavior.
        return None;
    }

    let confirmed = slash_defer_duration.is_zero();
    Some(Slash {
        validator: stash.clone(),
        percentage: slash_fraction,
        slash_id,
        reporters: Vec::new(),
        confirmed,
    })
}

/// Check that list is sorted and has no duplicates.
fn is_sorted_and_unique(list: &[u32]) -> bool {
    list.windows(2).all(|w| w[0] < w[1])
}
