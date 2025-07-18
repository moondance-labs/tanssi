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

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;

pub use pallet::*;

use {
    frame_support::traits::{
        fungible::{self, Mutate},
        Defensive, Get, ValidatorSet,
    },
    parity_scale_codec::Encode,
    polkadot_primitives::ValidatorIndex,
    runtime_parachains::session_info,
    snowbridge_core::{ChannelId, TokenId},
    snowbridge_merkle_tree::{merkle_proof, merkle_root, verify_proof, MerkleProof},
    sp_core::H256,
    sp_runtime::traits::{Hash, MaybeEquivalence, Zero},
    sp_staking::SessionIndex,
    sp_std::{collections::btree_set::BTreeSet, vec::Vec},
    tp_bridge::{Command, DeliverMessage, Message, TicketInfo, ValidateMessage},
    tp_traits::ExternalIndexProvider,
    xcm::prelude::*,
};

/// Utils needed to generate/verify merkle roots/proofs inside this pallet.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EraRewardsUtils {
    pub rewards_merkle_root: H256,
    pub leaves: Vec<H256>,
    pub leaf_index: Option<u64>,
    pub total_points: u128,
}

#[frame_support::pallet]
pub mod pallet {
    pub use crate::weights::WeightInfo;
    use {
        super::*, frame_support::pallet_prelude::*, sp_runtime::Saturating,
        sp_std::collections::btree_map::BTreeMap, tp_traits::EraIndexProvider,
    };

    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

    pub type RewardPoints = u32;
    pub type EraIndex = u32;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

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

        /// Provider to know how may tokens were inflated (added) in a specific era.
        type EraInflationProvider: Get<u128>;

        /// Provider to retrieve the current external index indetifying the validators
        type ExternalIndexProvider: ExternalIndexProvider;

        type GetWhitelistedValidators: Get<Vec<Self::AccountId>>;

        /// Hashing tool used to generate/verify merkle roots and proofs.
        type Hashing: Hash<Output = H256>;

        /// Validate a message that will be sent to Ethereum.
        type ValidateMessage: ValidateMessage;

        /// Send a message to Ethereum. Needs to be validated first.
        type OutboundQueue: DeliverMessage<
            Ticket = <<Self as pallet::Config>::ValidateMessage as ValidateMessage>::Ticket,
        >;

        /// Currency the rewards are minted in
        type Currency: fungible::Inspect<Self::AccountId, Balance: From<u128>>
            + fungible::Mutate<Self::AccountId>;

        /// Ethereum Sovereign Account where rewards will be minted
        type RewardsEthereumSovereignAccount: Get<Self::AccountId>;

        /// Token Location from the external chain's point of view.
        type TokenLocationReanchored: Get<Location>;

        /// How to convert from a given Location to a specific TokenId.
        type TokenIdFromLocation: MaybeEquivalence<TokenId, Location>;

        /// The weight information of this pallet.
        type WeightInfo: WeightInfo;

        #[cfg(feature = "runtime-benchmarks")]
        type BenchmarkHelper: tp_bridge::TokenChannelSetterBenchmarkHelperTrait;
    }

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// The rewards message was sent correctly.
        RewardsMessageSent {
            message_id: H256,
            rewards_command: Command,
        },
    }

    /// Keep tracks of distributed points per validator and total.
    #[derive(RuntimeDebug, Encode, Decode, PartialEq, Eq, TypeInfo)]
    pub struct EraRewardPoints<AccountId> {
        pub total: RewardPoints,
        pub individual: BTreeMap<AccountId, RewardPoints>,
    }

    impl<AccountId: Ord + sp_runtime::traits::Debug + Parameter> EraRewardPoints<AccountId> {
        // Helper function used to generate the following utils:
        //  - rewards_merkle_root: merkle root corresponding [(validatorId, rewardPoints)]
        //      for the era_index specified.
        //  - leaves: that were used to generate the previous merkle root.
        //  - leaf_index: index of the validatorId's leaf in the previous leaves array (if any).
        //  - total_points: number of total points of the era_index specified.
        pub fn generate_era_rewards_utils<Hasher: sp_runtime::traits::Hash<Output = H256>>(
            &self,
            era_index: EraIndex,
            maybe_account_id_check: Option<AccountId>,
        ) -> Option<EraRewardsUtils> {
            let total_points: u128 = u128::from(self.total);
            let mut leaves = Vec::with_capacity(self.individual.len());
            let mut leaf_index = None;

            if let Some(account) = &maybe_account_id_check {
                if !self.individual.contains_key(account) {
                    log::error!(
                        target: "ext_validators_rewards",
                        "AccountId {:?} not found for era {:?}!",
                        account,
                        era_index
                    );
                    return None;
                }
            }

            for (index, (account_id, reward_points)) in self.individual.iter().enumerate() {
                let encoded = (account_id, reward_points).encode();
                let hashed = <Hasher as sp_runtime::traits::Hash>::hash(&encoded);

                leaves.push(hashed);

                if let Some(ref check_account_id) = maybe_account_id_check {
                    if account_id == check_account_id {
                        leaf_index = Some(index as u64);
                    }
                }
            }

            let rewards_merkle_root = merkle_root::<Hasher, _>(leaves.iter().cloned());

            Some(EraRewardsUtils {
                rewards_merkle_root,
                leaves,
                leaf_index,
                total_points,
            })
        }
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
        /// Reward validators. Does not check if the validators are valid, caller needs to make sure of that.
        pub fn reward_by_ids(points: impl IntoIterator<Item = (T::AccountId, RewardPoints)>) {
            let active_era = T::EraIndexProvider::active_era();

            RewardPointsForEra::<T>::mutate(active_era.index, |era_rewards| {
                for (validator, points) in points.into_iter() {
                    (*era_rewards.individual.entry(validator.clone()).or_default())
                        .saturating_accrue(points);
                    era_rewards.total.saturating_accrue(points);
                }
            })
        }

        pub fn generate_rewards_merkle_proof(
            account_id: T::AccountId,
            era_index: EraIndex,
        ) -> Option<MerkleProof> {
            let era_rewards = RewardPointsForEra::<T>::get(era_index);
            let utils = era_rewards.generate_era_rewards_utils::<<T as Config>::Hashing>(
                era_index,
                Some(account_id),
            )?;
            utils.leaf_index.map(|index| {
                merkle_proof::<<T as Config>::Hashing, _>(utils.leaves.into_iter(), index)
            })
        }

        pub fn verify_rewards_merkle_proof(merkle_proof: MerkleProof) -> bool {
            verify_proof::<<T as Config>::Hashing, _, _>(
                &merkle_proof.root,
                merkle_proof.proof,
                merkle_proof.number_of_leaves,
                merkle_proof.leaf_index,
                merkle_proof.leaf,
            )
        }
    }

    impl<T: Config> tp_traits::OnEraStart for Pallet<T> {
        fn on_era_start(era_index: EraIndex, _session_start: u32, _external_idx: u64) {
            let Some(era_index_to_delete) = era_index.checked_sub(T::HistoryDepth::get()) else {
                return;
            };

            RewardPointsForEra::<T>::remove(era_index_to_delete);
        }
    }

    impl<T: Config> tp_traits::OnEraEnd for Pallet<T> {
        fn on_era_end(era_index: EraIndex) {
            // Will send a ReportRewards message to Ethereum unless:
            // - the reward token is misconfigured
            // - the tokens inflation is 0 (misconfigured inflation)
            // - the total points is 0 (no rewards to distribute)
            // - it fails to mint the tokens in the Ethereum Sovereign Account
            // - the generated message doesn't pass validation

            let token_location = T::TokenLocationReanchored::get();
            let token_id = T::TokenIdFromLocation::convert_back(&token_location);

            if let Some(token_id) = token_id {
                let era_rewards = RewardPointsForEra::<T>::get(era_index);
                if let Some(utils) = era_rewards
                    .generate_era_rewards_utils::<<T as Config>::Hashing>(era_index, None)
                {
                    let tokens_inflated = T::EraInflationProvider::get();

                    if tokens_inflated.is_zero() {
                        log::error!(target: "ext_validators_rewards", "Not sending message because tokens_inflated is 0");
                        return;
                    }

                    if utils.total_points.is_zero() {
                        log::error!(target: "ext_validators_rewards", "Not sending message because total_points is 0");
                        return;
                    }

                    let ethereum_sovereign_account = T::RewardsEthereumSovereignAccount::get();
                    if let Err(err) =
                        T::Currency::mint_into(&ethereum_sovereign_account, tokens_inflated.into())
                    {
                        log::error!(target: "ext_validators_rewards", "Failed to mint inflation into Ethereum Soverein Account: {err:?}");
                        log::error!(target: "ext_validators_rewards", "Not sending message since there are no rewards to distribute");
                        return;
                    }

                    let command = Command::ReportRewards {
                        external_idx: T::ExternalIndexProvider::get_external_index(),
                        era_index,
                        total_points: utils.total_points,
                        tokens_inflated,
                        rewards_merkle_root: utils.rewards_merkle_root,
                        token_id,
                    };

                    let channel_id: ChannelId = snowbridge_core::PRIMARY_GOVERNANCE_CHANNEL;

                    let outbound_message = Message {
                        id: None,
                        channel_id,
                        command: command.clone(),
                    };

                    // Validate and deliver the message
                    match T::ValidateMessage::validate(&outbound_message) {
                        Ok((ticket, _fee)) => {
                            let message_id = ticket.message_id();
                            if let Err(err) = T::OutboundQueue::deliver(ticket) {
                                log::error!(target: "ext_validators_rewards", "OutboundQueue delivery of message failed. {err:?}");
                            } else {
                                Self::deposit_event(Event::RewardsMessageSent {
                                    message_id,
                                    rewards_command: command,
                                });
                            }
                        }
                        Err(err) => {
                            log::error!(target: "ext_validators_rewards", "OutboundQueue validation of message failed. {err:?}");
                        }
                    }

                    frame_system::Pallet::<T>::register_extra_weight_unchecked(
                        T::WeightInfo::on_era_end(),
                        DispatchClass::Mandatory,
                    );
                } else {
                    // Unreachable, this should never happen as we are sending
                    // None as the second param in Self::generate_era_rewards_utils.
                    log::error!(
                        target: "ext_validators_rewards",
                        "Outbound message not sent for era {:?}!",
                        era_index
                    );
                }
            } else {
                log::error!(target: "ext_validators_rewards", "no token id found for location {:?}", token_location);
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
        let validators = session_info::AccountKeys::<C>::get(session_index);
        let validators = match validators
            .defensive_proof("account_keys are present for dispute_period sessions")
        {
            Some(validators) => validators,
            None => return,
        };
        // limit rewards to the active validator set
        let mut active_set: BTreeSet<_> = C::ValidatorSet::validators().into_iter().collect();

        // Remove whitelisted validators, we don't want to reward them
        let whitelisted_validators = C::GetWhitelistedValidators::get();
        for validator in whitelisted_validators {
            active_set.remove(&validator);
        }

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
