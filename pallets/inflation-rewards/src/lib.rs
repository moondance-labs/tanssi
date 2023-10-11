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

//! # Inflation Rewards Pallet
//!
//! This pallet handle native token inflation and rewards dsitribution.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use {
    frame_support::{
        pallet_prelude::*,
        traits::{
            fungible::{Balanced, Credit, Inspect},
            tokens::{Fortitude, Precision, Preservation},
            Imbalance, OnUnbalanced,
        },
    },
    frame_system::pallet_prelude::*,
    sp_runtime::{traits::Get, Perbill},
    tp_core::{BlockNumber, ParaId},
    tp_traits::{AuthorNotingHook, DistributeRewards, GetCurrentContainerChains},
};

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    pub type BalanceOf<T> =
        <<T as Config>::Currency as Inspect<<T as frame_system::Config>::AccountId>>::Balance;
    pub type CreditOf<T> = Credit<<T as frame_system::Config>::AccountId, <T as Config>::Currency>;

    /// Inflation rewards pallet.
    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(core::marker::PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_: T::BlockNumber) -> Weight {
            let mut weight = T::DbWeight::get().reads(1);

            // Collect indistributed rewards, if any
            let not_distributed_rewards =
                if let Some(chains_to_reward) = ChainsToReward::<T>::take() {
                    // Collect and sum all undistributed rewards
                    let rewards_not_distributed: BalanceOf<T> = chains_to_reward.rewards_per_chain
                        * (chains_to_reward.para_ids.len() as u32).into();
                    T::Currency::withdraw(
                        &T::PendingRewardsAccount::get(),
                        rewards_not_distributed,
                        Precision::BestEffort,
                        Preservation::Expendable,
                        Fortitude::Force,
                    )
                    .unwrap_or(CreditOf::<T>::zero())
                } else {
                    CreditOf::<T>::zero()
                };

            // Get the number of chains at this block
            weight += T::DbWeight::get().reads_writes(1, 1);
            let registered_para_ids = T::ContainerChains::current_container_chains();

            let number_of_chains: BalanceOf<T> =
                ((registered_para_ids.len() as u32).saturating_add(1)).into();

            // Issue new supply
            let new_supply =
                T::Currency::issue(T::InflationRate::get() * T::Currency::total_issuance());

            // Split staking reward portion
            let total_rewards = T::RewardsPortion::get() * new_supply.peek();
            let (rewards_credit, reminder_credit) = new_supply.split(total_rewards);
            let rewards_per_chain: BalanceOf<T> = rewards_credit.peek() / number_of_chains;
            let (mut total_reminder, staking_rewards) = rewards_credit.split_merge(
                total_rewards % number_of_chains,
                (reminder_credit, CreditOf::<T>::zero()),
            );

            // Deposit the new supply dedicated to rewards in the pending rewards account
            if let Err(undistributed_rewards) =
                T::Currency::resolve(&T::PendingRewardsAccount::get(), staking_rewards)
            {
                total_reminder = total_reminder.merge(undistributed_rewards);
            }

            // Keep track of chains to reward
            ChainsToReward::<T>::put(ChainsToRewardValue {
                para_ids: registered_para_ids,
                rewards_per_chain,
            });

            // Let the runtime handle the non-staking part
            T::OnUnbalanced::on_unbalanced(not_distributed_rewards.merge(total_reminder));

            // Reward orchestrator chain author
            weight += Self::reward_orchestrator_author();

            weight
        }
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Currency: Inspect<Self::AccountId> + Balanced<Self::AccountId>;

        type ContainerChains: GetCurrentContainerChains;

        /// Get block author for self chain
        type GetSelfChainBlockAuthor: Get<Self::AccountId>;

        /// Inflation rate per relay block (proportion of the total issuance)
        type InflationRate: Get<Perbill>;

        /// The maximum number of block authors
        type MaxAuthors: Get<u32>;

        /// What to do with the new supply not dedicated to staking
        type OnUnbalanced: OnUnbalanced<CreditOf<Self>>;

        /// The account that will store rewards waiting to be paid out
        type PendingRewardsAccount: Get<Self::AccountId>;

        /// Staking rewards distribution implementation
        type StakingRewardsDistributor: DistributeRewards<Self::AccountId, CreditOf<Self>>;

        /// Proportion of the new supply dedicated to staking
        type RewardsPortion: Get<Perbill>;
    }

    /// Container chains to reward per block
    #[pallet::storage]
    #[pallet::getter(fn container_chains_to_reward)]
    pub(super) type ChainsToReward<T: Config> =
        StorageValue<_, ChainsToRewardValue<T>, OptionQuery>;
    #[derive(Clone, Encode, Decode, PartialEq, sp_core::RuntimeDebug, scale_info::TypeInfo)]
    #[scale_info(skip_type_params(T))]
    pub struct ChainsToRewardValue<T: Config> {
        pub para_ids: BoundedVec<
            ParaId,
            <T::ContainerChains as GetCurrentContainerChains>::MaxContainerChains,
        >,
        pub rewards_per_chain: BalanceOf<T>,
    }

    /// Pending authors per block and per container chain
    #[pallet::storage]
    #[pallet::getter(fn pending_rewards)]
    pub(super) type PendingAuthors<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        BlockNumber,
        Twox64Concat,
        ParaId,
        PendingAuthorsValue<T>,
        OptionQuery,
    >;

    /// Information extracted from a container chain header
    #[derive(
        Clone, Encode, Decode, PartialEq, sp_core::RuntimeDebug, scale_info::TypeInfo, MaxEncodedLen,
    )]
    #[scale_info(skip_type_params(T))]
    pub struct PendingAuthorsValue<T: Config> {
        pub authors: BoundedVec<(T::AccountId, u32), T::MaxAuthors>,
        pub last_block_number: BlockNumber,
    }
    impl<T: Config> PendingAuthorsValue<T> {
        pub(super) fn new(author: T::AccountId, block_number: BlockNumber) -> Self {
            let mut authors = BoundedVec::new();
            authors
                .try_push((author, 1))
                .expect("MaxAuthors should be greather than zero");
            Self {
                authors,
                last_block_number: block_number,
            }
        }
    }

    impl<T: Config> Pallet<T> {
        fn reward_orchestrator_author() -> Weight {
            let mut total_weight = T::DbWeight::get().reads(1);
            let orchestrator_author = T::GetSelfChainBlockAuthor::get();

            if let Some(chains_to_reward) = ChainsToReward::<T>::get() {
                total_weight += T::DbWeight::get().reads(1);
                match T::StakingRewardsDistributor::distribute_rewards(
                    orchestrator_author,
                    T::Currency::withdraw(
                        &T::PendingRewardsAccount::get(),
                        chains_to_reward.rewards_per_chain,
                        Precision::BestEffort,
                        Preservation::Expendable,
                        Fortitude::Force,
                    )
                    .unwrap_or(CreditOf::<T>::zero()),
                ) {
                    Ok(frame_support::dispatch::PostDispatchInfo {
                        actual_weight: Some(weight),
                        ..
                    }) => total_weight += weight,
                    Err(e) => {
                        log::debug!("Fail to distribute rewards: {:?}", e)
                    }
                    _ => {}
                }
            } else {
                panic!("ChainsToReward not filled");
            }

            total_weight
        }
    }
}

impl<T: Config> AuthorNotingHook<T::AccountId> for Pallet<T> {
    fn on_container_author_noted(
        author: &T::AccountId,
        block_number: BlockNumber,
        para_id: ParaId,
    ) -> Weight {
        let mut total_weight = T::DbWeight::get().reads_writes(1, 1);
        let pending_authors_per_chain = if let Some(mut pending_authors_per_chain) =
            PendingAuthors::<T>::get(block_number, para_id)
        {
            // track how many blocks produced by each author
            if block_number > pending_authors_per_chain.last_block_number {
                match pending_authors_per_chain
                    .authors
                    .binary_search_by(|(account, _)| account.cmp(author))
                {
                    Ok(index) => {
                        pending_authors_per_chain.authors[index].1 += 1;
                    }
                    Err(index) => {
                        if pending_authors_per_chain
                            .authors
                            .try_insert(index, (author.clone(), 1))
                            .is_err()
                        {
                            log::warn!("too many authors");
                        }
                    }
                }
            }
            pending_authors_per_chain
        } else if block_number > 0 {
            let previous_block_number = block_number - 1;
            if let Some(authors_to_reward) =
                PendingAuthors::<T>::take(previous_block_number, para_id)
            {
                if !authors_to_reward.authors.is_empty() {
                    total_weight += T::DbWeight::get().reads(1);
                    if let Some(container_chains_to_reward) = ChainsToReward::<T>::get() {
                        total_weight += T::DbWeight::get().reads(1);

                        // Compute amount to reward per block
                        let rewards_per_block: BalanceOf<T> = container_chains_to_reward
                            .rewards_per_chain
                            / (authors_to_reward.authors.len() as u32).into();

                        // Reward all blocks authors in `authors_to_reward`
                        for (author, blocks) in authors_to_reward.authors {
                            match T::StakingRewardsDistributor::distribute_rewards(
                                author,
                                T::Currency::withdraw(
                                    &T::PendingRewardsAccount::get(),
                                    rewards_per_block * blocks.into(),
                                    Precision::BestEffort,
                                    Preservation::Expendable,
                                    Fortitude::Force,
                                )
                                .unwrap_or(CreditOf::<T>::zero()),
                            ) {
                                Ok(frame_support::dispatch::PostDispatchInfo {
                                    actual_weight: Some(weight),
                                    ..
                                }) => total_weight += weight,
                                Err(e) => {
                                    log::debug!("Fail to distribute rewards: {:?}", e)
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            PendingAuthorsValue::new(author.clone(), block_number)
        } else {
            PendingAuthorsValue::new(author.clone(), block_number)
        };
        PendingAuthors::<T>::insert(block_number, para_id, pending_authors_per_chain);
        total_weight
    }
}
