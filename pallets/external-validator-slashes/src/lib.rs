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

//! ExternalValidatorSlashes pallet.
//!
//! A pallet to store slashes based on offences committed by validators
//! Slashes can be cancelled during the DeferPeriod through cancel_deferred_slash
//! Slashes can also be forcedly injected via the force_inject_slash extrinsic
//! Slashes for a particular era are removed after the bondingPeriod has elapsed
//!
//! ## OnOffence trait
//!
//! The pallet also implements the OnOffence trait that reacts to offences being injected by other pallets
//! Invulnerables are not slashed and no slashing information is stored for them

#![cfg_attr(not(feature = "std"), no_std)]

use {
    frame_support::{pallet_prelude::*, traits::DefensiveSaturating},
    frame_system::pallet_prelude::*,
    log::log,
    pallet_staking::SessionInterface,
    parity_scale_codec::FullCodec,
    parity_scale_codec::{Decode, Encode},
    sp_core::H256,
    sp_runtime::traits::{Convert, Debug, One, Saturating, Zero},
    sp_runtime::DispatchResult,
    sp_runtime::Perbill,
    sp_staking::{
        offence::{OffenceDetails, OnOffenceHandler},
        EraIndex, SessionIndex,
    },
    sp_std::collections::vec_deque::VecDeque,
    sp_std::vec,
    sp_std::vec::Vec,
    tp_traits::{EraIndexProvider, InvulnerablesProvider, OnEraStart},
};

use snowbridge_core::ChannelId;
use tp_bridge::{Command, DeliverMessage, Message, ValidateMessage};

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    pub use crate::weights::WeightInfo;

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

        // SlashId type, used as a counter on the number of slashes
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

        /// Era index provider, used to fetch the active era among other things
        type EraIndexProvider: EraIndexProvider;

        /// Invulnerable provider, used to get the invulnerables to know when not to slash
        type InvulnerablesProvider: InvulnerablesProvider<Self::ValidatorId>;

        /// Validate a message that will be sent to Ethereum.
        type ValidateMessage: ValidateMessage;

        /// Send a message to Ethereum. Needs to be validated first.
        type OutboundQueue: DeliverMessage<
            Ticket = <<Self as pallet::Config>::ValidateMessage as ValidateMessage>::Ticket,
        >;

        /// Provider to retrieve the current block timestamp.
        type TimestampProvider: Get<u64>;

        /// How many queued slashes are being processed per block.
        #[pallet::constant]
        type QueuedSlashesProcessedPerBlock: Get<u32>;

        /// The weight information of this pallet.
        type WeightInfo: WeightInfo;
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The era for which the slash wants to be cancelled has no slashes
        EmptyTargets,
        /// No slash was found to be cancelled at the given index
        InvalidSlashIndex,
        /// Slash indices to be cancelled are not sorted or unique
        NotSortedAndUnique,
        /// Provided an era in the future
        ProvidedFutureEra,
        /// Provided an era that is not slashable
        ProvidedNonSlashableEra,
        /// The slash to be cancelled has already elapsed the DeferPeriod
        DeferPeriodIsOver,
        /// There was an error computing the slash
        ErrorComputingSlash,
        /// Failed to validate the message that was going to be sent to Ethereum
        EthereumValidateFail,
        /// Failed to deliver the message to Ethereum
        EthereumDeliverFail,
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
    pub type BondedEras<T: Config> = StorageValue<_, Vec<(EraIndex, SessionIndex)>, ValueQuery>;

    /// A counter on the number of slashes we have performed
    #[pallet::storage]
    #[pallet::getter(fn next_slash_id)]
    pub type NextSlashId<T: Config> = StorageValue<_, T::SlashId, ValueQuery>;

    /// All unapplied slashes that are queued for later.
    #[pallet::storage]
    #[pallet::unbounded]
    #[pallet::getter(fn slashes)]
    pub type Slashes<T: Config> =
        StorageMap<_, Twox64Concat, EraIndex, Vec<Slash<T::AccountId, T::SlashId>>, ValueQuery>;

    /// All unreported slashes that will be processed in the future.
    #[pallet::storage]
    #[pallet::unbounded]
    #[pallet::getter(fn unreported_slashes)]
    pub type UnreportedSlashesQueue<T: Config> =
        StorageValue<_, VecDeque<Slash<T::AccountId, T::SlashId>>, ValueQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Cancel a slash that was deferred for a later era
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::cancel_deferred_slash(slash_indices.len() as u32))]
        pub fn cancel_deferred_slash(
            origin: OriginFor<T>,
            era: EraIndex,
            slash_indices: Vec<u32>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            let active_era = T::EraIndexProvider::active_era().index;

            // We need to be in the defer period
            ensure!(
                era <= active_era
                    .saturating_add(T::SlashDeferDuration::get().saturating_add(One::one()))
                    && era > active_era,
                Error::<T>::DeferPeriodIsOver
            );

            ensure!(!slash_indices.is_empty(), Error::<T>::EmptyTargets);
            ensure!(
                is_sorted_and_unique(&slash_indices),
                Error::<T>::NotSortedAndUnique
            );
            // fetch slashes for the era in which we want to defer
            let mut era_slashes = Slashes::<T>::get(&era);

            let last_item = slash_indices[slash_indices.len() - 1];
            ensure!(
                (last_item as usize) < era_slashes.len(),
                Error::<T>::InvalidSlashIndex
            );

            // Remove elements starting from the highest index to avoid shifting issues.
            for index in slash_indices.into_iter().rev() {
                era_slashes.remove(index as usize);
            }
            // insert back slashes
            Slashes::<T>::insert(&era, &era_slashes);
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::force_inject_slash())]
        pub fn force_inject_slash(
            origin: OriginFor<T>,
            era: EraIndex,
            validator: T::AccountId,
            percentage: Perbill,
        ) -> DispatchResult {
            ensure_root(origin)?;
            let active_era = T::EraIndexProvider::active_era().index;

            ensure!(era <= active_era, Error::<T>::ProvidedFutureEra);

            let slash_defer_duration = T::SlashDeferDuration::get();

            let _ = T::EraIndexProvider::era_to_session_start(era)
                .ok_or(Error::<T>::ProvidedNonSlashableEra)?;

            let next_slash_id = NextSlashId::<T>::get();

            let slash = compute_slash::<T>(
                percentage,
                next_slash_id,
                era,
                validator,
                slash_defer_duration,
            )
            .ok_or(Error::<T>::ErrorComputingSlash)?;

            // If we defer duration is 0, we immediately apply and confirm
            let era_to_consider = if slash_defer_duration == 0 {
                era.saturating_add(One::one())
            } else {
                era.saturating_add(slash_defer_duration)
                    .saturating_add(One::one())
            };

            Slashes::<T>::mutate(&era_to_consider, |era_slashes| {
                era_slashes.push(slash);
            });

            NextSlashId::<T>::put(next_slash_id.saturating_add(One::one()));
            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::root_test_send_msg_to_eth())]
        pub fn root_test_send_msg_to_eth(
            origin: OriginFor<T>,
            nonce: H256,
            num_msgs: u32,
            msg_size: u32,
        ) -> DispatchResult {
            ensure_root(origin)?;

            for i in 0..num_msgs {
                // Make sure each message has a different payload
                let mut payload = sp_core::blake2_256((nonce, i).encode().as_ref()).to_vec();
                // Extend with zeros until msg_size is reached
                payload.resize(msg_size as usize, 0);
                // Example command, this should be something like "ReportSlashes"
                let command = Command::Test(payload);

                // Validate
                let channel_id: ChannelId = snowbridge_core::PRIMARY_GOVERNANCE_CHANNEL;

                let outbound_message = Message {
                    id: None,
                    channel_id,
                    command,
                };

                // validate the message
                // Ignore fee because for now only root can send messages
                let (ticket, _fee) =
                    T::ValidateMessage::validate(&outbound_message).map_err(|err| {
                        log::error!(
                            "root_test_send_msg_to_eth: validation of message {i} failed. {err:?}"
                        );
                        crate::pallet::Error::<T>::EthereumValidateFail
                    })?;

                // Deliver
                T::OutboundQueue::deliver(ticket).map_err(|err| {
                    log::error!(
                        "root_test_send_msg_to_eth: delivery of message {i} failed. {err:?}"
                    );
                    crate::pallet::Error::<T>::EthereumDeliverFail
                })?;
            }

            Ok(())
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
            log::info!("inside on-initialize");

            let processed = Self::process_slashes_queue(T::QueuedSlashesProcessedPerBlock::get());
            T::WeightInfo::process_slashes_queue(processed)
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
        let active_era = {
            let active_era = T::EraIndexProvider::active_era().index;
            active_era
        };
        let active_era_start_session_index = T::EraIndexProvider::era_to_session_start(active_era)
            .unwrap_or_else(|| {
                frame_support::print("Error: start_session_index must be set for current_era");
                0
            });

        // Fast path for active-era report - most likely.
        // `slash_session` cannot be in a future active era. It must be in `active_era` or before.
        let slash_era = if slash_session >= active_era_start_session_index {
            active_era
        } else {
            let eras = BondedEras::<T>::get();

            // Reverse because it's more likely to find reports from recent eras.
            match eras.iter().rev().find(|&(_, sesh)| sesh <= &slash_session) {
                Some((slash_era, _)) => *slash_era,
                // Before bonding period. defensive - should be filtered out.
                None => return Weight::default(),
            }
        };

        let slash_defer_duration = T::SlashDeferDuration::get();

        let invulnerables = T::InvulnerablesProvider::invulnerables();

        let mut next_slash_id = NextSlashId::<T>::get();

        for (details, slash_fraction) in offenders.iter().zip(slash_fraction) {
            let (stash, _) = &details.offender;

            // Skip if the validator is invulnerable.
            if invulnerables.contains(stash) {
                continue;
            }

            let slash = compute_slash::<T>(
                *slash_fraction,
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

                // Cover slash defer duration equal to 0
                // Slashes are applied at the end of the current era
                if slash_defer_duration == 0 {
                    Slashes::<T>::mutate(active_era.saturating_add(One::one()), move |for_now| {
                        for_now.push(slash)
                    });
                } else {
                    // Else, slashes are applied after slash_defer_period since the slashed era
                    Slashes::<T>::mutate(
                        slash_era
                            .saturating_add(slash_defer_duration)
                            .saturating_add(One::one()),
                        move |for_later| for_later.push(slash),
                    );
                }

                // Fix unwrap
                next_slash_id = next_slash_id.saturating_add(One::one());
            }
        }
        NextSlashId::<T>::put(next_slash_id);
        Weight::default()
    }
}

impl<T: Config> OnEraStart for Pallet<T> {
    fn on_era_start(era_index: EraIndex, session_start: SessionIndex) {
        // This should be small, as slashes are limited by the num of validators
        // let's put 1000 as a conservative measure
        const REMOVE_LIMIT: u32 = 1000;

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
                    let removal_result =
                        ValidatorSlashInEra::<T>::clear_prefix(&pruned_era, REMOVE_LIMIT, None);
                    if removal_result.maybe_cursor.is_some() {
                        log::error!(
                            "Not all validator slashes were remove for era {:?}",
                            pruned_era
                        );
                    }
                    Slashes::<T>::remove(&pruned_era);
                }

                if let Some(&(_, first_session)) = bonded.first() {
                    T::SessionInterface::prune_historical_up_to(first_session);
                }
            }
        });

        Self::add_era_slashes_to_queue(era_index);
    }
}

impl<T: Config> Pallet<T> {
    fn add_era_slashes_to_queue(active_era: EraIndex) {
        let mut slashes: VecDeque<_> = Slashes::<T>::get(&active_era).into();

        UnreportedSlashesQueue::<T>::mutate(|queue| queue.append(&mut slashes));
    }

    fn process_slashes_queue(amount: u32) -> u32 {
        let mut slashes_to_send: Vec<_> = vec![];
        let era_index = T::EraIndexProvider::active_era().index;

        // prepare up to QueuedSlashesProcessedPerBlock slashes to be sent
        UnreportedSlashesQueue::<T>::mutate(|queue| {
            for _ in 0..amount {
                let Some(slash) = queue.pop_front() else {
                    // no more slashes to process in the queue
                    break;
                };

                slashes_to_send.push((
                    slash.validator.clone().encode(),
                    slash.percentage.deconstruct(),
                ));
            }
        });

        if slashes_to_send.is_empty() {
            return 0;
        }

        let slashes_count = slashes_to_send.len() as u32;

        // Build command with slashes.
        let command = Command::ReportSlashes {
            // TODO: change this
            timestamp: T::TimestampProvider::get(),
            era_index,
            slashes: slashes_to_send,
        };

        let channel_id: ChannelId = snowbridge_core::PRIMARY_GOVERNANCE_CHANNEL;

        let outbound_message = Message {
            id: None,
            channel_id,
            command,
        };

        // Validate and deliver the message
        match T::ValidateMessage::validate(&outbound_message) {
            Ok((ticket, _fee)) => {
                if let Err(err) = T::OutboundQueue::deliver(ticket) {
                    log::error!(target: "ext_validators_slashes", "OutboundQueue delivery of message failed. {err:?}");
                }
            }
            Err(err) => {
                log::error!(target: "ext_validators_slashes", "OutboundQueue validation of message failed. {err:?}");
            }
        };

        slashes_count
    }
}

/// A pending slash record. The value of the slash has been computed but not applied yet,
/// rather deferred for several eras.
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, Clone, PartialEq)]
pub struct Slash<AccountId, SlashId> {
    /// The stash ID of the offending validator.
    pub validator: AccountId,
    /// Reporters of the offence; bounty payout recipients.
    pub reporters: Vec<AccountId>,
    /// The amount of payout.
    pub slash_id: SlashId,
    pub percentage: Perbill,
    // Whether the slash is confirmed or still needs to go through deferred period
    pub confirmed: bool,
}

impl<AccountId, SlashId: One> Slash<AccountId, SlashId> {
    /// Initializes the default object using the given `validator`.
    pub fn default_from(validator: AccountId) -> Self {
        Self {
            validator,
            reporters: vec![],
            slash_id: One::one(),
            percentage: Perbill::from_percent(50),
            confirmed: false,
        }
    }
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
