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

//! This pallet keep tracks of the validators reward points.
//! Storage will be cleared after a period of time.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;

use {
    frame_support::traits::{Defensive, Get, ValidatorSet},
    polkadot_primitives::ValidatorIndex,
    runtime_parachains::session_info,
    snowbridge_core::ChannelId,
    snowbridge_outbound_queue_merkle_tree::merkle_root,
    sp_core::H256,
    sp_runtime::traits::Hash,
    sp_staking::SessionIndex,
    sp_std::collections::btree_set::BTreeSet,
    sp_std::vec,
    sp_std::vec::Vec,
    tp_bridge::{Command, Message, ValidateMessage, DeliverMessage},
};

#[frame_support::pallet]
pub mod pallet {
    use {
        super::*,
        frame_support::pallet_prelude::*, sp_std::collections::btree_map::BTreeMap,
        tp_traits::EraIndexProvider,
    };

    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

    pub type RewardPoints = u32;
    pub type EraIndex = u32;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// How to fetch the current era info.
        type EraIndexProvider: EraIndexProvider;

        /// For how many eras points are kept in storage.
        #[pallet::constant]
        type HistoryDepth: Get<EraIndex>;

        /// The amount of era points given by backing a candidate that is included.
        #[pallet::constant]
        type BackingPoints: Get<u32>;

        /// The amount of era points given by dispute voting on a candidate.
        #[pallet::constant]
        type DisputeStatementPoints: Get<u32>;

        type Hashing: Hash<Output = H256>;

        type ValidateMessage: ValidateMessage;

        type OutboundQueue: DeliverMessage<
            Ticket = <<Self as pallet::Config>::ValidateMessage as ValidateMessage>::Ticket,
        >;
    }

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    /// Keep tracks of distributed points per validator and total.
    #[derive(RuntimeDebug, Encode, Decode, PartialEq, Eq, TypeInfo)]
    pub struct EraRewardPoints<AccountId> {
        pub total: RewardPoints,
        pub individual: BTreeMap<AccountId, RewardPoints>,
    }

    impl<AccountId> Default for EraRewardPoints<AccountId> {
        fn default() -> Self {
            EraRewardPoints {
                total: Default::default(),
                individual: BTreeMap::new(),
            }
        }
    }

    /// Store reward points per era.
    /// Note: EraRewardPoints is actually bounded by the amount of validators.
    #[pallet::storage]
    #[pallet::unbounded]
    pub type RewardPointsForEra<T: Config> =
        StorageMap<_, Twox64Concat, EraIndex, EraRewardPoints<T::AccountId>, ValueQuery>;

    impl<T: Config> Pallet<T> {
        pub fn reward_by_ids(points: impl IntoIterator<Item = (T::AccountId, RewardPoints)>) {
            let active_era = T::EraIndexProvider::active_era();

            RewardPointsForEra::<T>::mutate(active_era.index, |era_rewards| {
                for (validator, points) in points.into_iter() {
                    *era_rewards.individual.entry(validator).or_default() += points;
                    era_rewards.total += points;
                }
            })
        }
    }

    impl<T: Config> tp_traits::OnEraStart for Pallet<T> {
        fn on_era_start(era_index: EraIndex, _session_start: u32) {
            let Some(era_index_to_delete) = era_index.checked_sub(T::HistoryDepth::get()) else {
                return;
            };

            RewardPointsForEra::<T>::remove(era_index_to_delete);
        }
    }

    impl<T: Config> tp_traits::OnEraEnd for Pallet<T> {
        fn on_era_end(era_index: EraIndex) {
            let era_rewards = RewardPointsForEra::<T>::get(&era_index);
            let mut rewards_info_to_merkelize: Vec<H256> = vec![];

            for (account_id, reward_points) in era_rewards.individual {
                let encoded = (account_id, reward_points).encode();
                let hashed = <T as Config>::Hashing::hash(&encoded);
                rewards_info_to_merkelize.push(hashed);
            }

            let rewards_merkle_root = merkle_root::<<T as Config>::Hashing, _>(rewards_info_to_merkelize.into_iter());

            let command = Command::ReportRewards {
                era_index,
                rewards_merkle_root
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
                        log::error!(target: "xcm::ethereum_blob_exporter", "OutboundQueue delivery of message failed. {err:?}");
                    }
                }, 
                Err(err) => {
                    log::error!(target: "xcm::ethereum_blob_exporter", "OutboundQueue validation of message failed. {err:?}");
                }
            }
        }
    }
}

/// Rewards validators for participating in parachains with era points in pallet-staking.
pub struct RewardValidatorsWithEraPoints<C>(core::marker::PhantomData<C>);

impl<C> RewardValidatorsWithEraPoints<C>
where
    C: pallet::Config + session_info::Config,
    C::ValidatorSet: ValidatorSet<C::AccountId, ValidatorId = C::AccountId>,
{
    /// Reward validators in session with points, but only if they are in the active set.
    fn reward_only_active(
        session_index: SessionIndex,
        indices: impl IntoIterator<Item = ValidatorIndex>,
        points: u32,
    ) {
        let validators = session_info::AccountKeys::<C>::get(&session_index);
        let validators = match validators
            .defensive_proof("account_keys are present for dispute_period sessions")
        {
            Some(validators) => validators,
            None => return,
        };
        // limit rewards to the active validator set
        let active_set: BTreeSet<_> = C::ValidatorSet::validators().into_iter().collect();

        let rewards = indices
            .into_iter()
            .filter_map(|i| validators.get(i.0 as usize).cloned())
            .filter(|v| active_set.contains(v))
            .map(|v| (v, points));

        pallet::Pallet::<C>::reward_by_ids(rewards);
    }
}

impl<C> runtime_parachains::inclusion::RewardValidators for RewardValidatorsWithEraPoints<C>
where
    C: pallet::Config + runtime_parachains::shared::Config + session_info::Config,
    C::ValidatorSet: ValidatorSet<C::AccountId, ValidatorId = C::AccountId>,
{
    fn reward_backing(indices: impl IntoIterator<Item = ValidatorIndex>) {
        let session_index = runtime_parachains::shared::CurrentSessionIndex::<C>::get();
        Self::reward_only_active(session_index, indices, C::BackingPoints::get());
    }

    fn reward_bitfields(_validators: impl IntoIterator<Item = ValidatorIndex>) {}
}

impl<C> runtime_parachains::disputes::RewardValidators for RewardValidatorsWithEraPoints<C>
where
    C: pallet::Config + session_info::Config,
    C::ValidatorSet: ValidatorSet<C::AccountId, ValidatorId = C::AccountId>,
{
    fn reward_dispute_statement(
        session: SessionIndex,
        validators: impl IntoIterator<Item = ValidatorIndex>,
    ) {
        Self::reward_only_active(session, validators, C::DisputeStatementPoints::get());
    }
}
