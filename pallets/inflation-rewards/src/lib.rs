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
//! This pallet handle native token inflation and rewards distribution.

#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
use tp_traits::BlockNumber;
use {
    alloc::vec::Vec,
    dp_core::ParaId,
    frame_support::{
        pallet_prelude::*,
        traits::{
            fungible::{Balanced, Credit, Inspect},
            tokens::{Fortitude, Precision, Preservation},
            Imbalance, OnUnbalanced,
        },
    },
    frame_system::pallet_prelude::*,
    sp_runtime::{
        traits::{Get, Saturating, Zero},
        Perbill,
    },
    tp_traits::{
        AuthorNotingHook, AuthorNotingInfo, DistributeRewards, ForSession,
        GetContainerChainsWithCollators, MaybeSelfChainBlockAuthor,
    },
};

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    pub type BalanceOf<T> =
        <<T as Config>::Currency as Inspect<<T as frame_system::Config>::AccountId>>::Balance;
    pub type CreditOf<T> = Credit<<T as frame_system::Config>::AccountId, <T as Config>::Currency>;

    /// Inflation rewards pallet.
    #[pallet::pallet]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_: BlockNumberFor<T>) -> Weight {
            let mut weight = T::DbWeight::get().reads(1);

            // Collect indistributed rewards, if any
            // Any parachain we have not rewarded is handled by onUnbalanced
            let not_distributed_rewards =
                if let Some(chains_to_reward) = ChainsToReward::<T>::take() {
                    // Collect and sum all undistributed rewards
                    let rewards_not_distributed: BalanceOf<T> = chains_to_reward
                        .rewards_per_chain
                        .saturating_mul((chains_to_reward.para_ids.len() as u32).into());
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

            // Get the number of chains at this block (tanssi + container chain blocks)
            weight.saturating_accrue(T::DbWeight::get().reads_writes(1, 1));
            let container_chains_to_check_unbounded: Vec<_> =
                T::ContainerChains::container_chains_with_collators(ForSession::Current)
                    .into_iter()
                    .filter_map(|(para_id, collators)| (!collators.is_empty()).then_some(para_id))
                    .collect();

            // Convert to BoundedVec. If the number of chains is greater than the limit, truncate
            // and emit a warning. This should never happen because we assume that
            // MaxContainerChains has the same value in all the pallets, but that's not enforced.
            // A better solution would be for container_chains_with_collators to return an already
            // bounded data structure.
            let unbounded_len = container_chains_to_check_unbounded.len();
            let container_chains_to_check =
                BoundedVec::truncate_from(container_chains_to_check_unbounded);
            if container_chains_to_check.len() != unbounded_len {
                log::warn!("inflation_rewards: got more chains than max. ")
            }

            let mut number_of_chains: BalanceOf<T> =
                (container_chains_to_check.len() as u32).into();

            // We only add 1 extra chain to number_of_chains if we are
            // in a parachain context with an orchestrator configured.
            if T::GetSelfChainBlockAuthor::get_block_author().is_some() {
                number_of_chains.saturating_inc();
            }

            // Only create new supply and rewards if number_of_chains is not zero.
            if !number_of_chains.is_zero() {
                // Issue new supply
                let new_supply =
                    T::Currency::issue(T::InflationRate::get() * T::Currency::total_issuance());

                // Split staking reward portion
                let total_rewards = T::RewardsPortion::get() * new_supply.peek();
                let (rewards_credit, reminder_credit) = new_supply.split(total_rewards);

                let rewards_per_chain: BalanceOf<T> = rewards_credit
                    .peek()
                    .checked_div(&number_of_chains)
                    .unwrap_or_else(|| {
                        log::error!("Rewards per chain is zero");
                        BalanceOf::<T>::zero()
                    });
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
                    para_ids: container_chains_to_check,
                    rewards_per_chain,
                });

                // Let the runtime handle the non-staking part
                T::OnUnbalanced::on_unbalanced(not_distributed_rewards.merge(total_reminder));

                // We don't reward the orchestrator in solochain mode
                if let Some(orchestrator_author) = T::GetSelfChainBlockAuthor::get_block_author() {
                    weight.saturating_accrue(Self::reward_orchestrator_author(orchestrator_author));
                }
            }

            weight
        }
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Currency: Inspect<Self::AccountId> + Balanced<Self::AccountId>;

        /// Get container chains with collators. The number of chains returned affects inflation:
        /// we mint tokens to reward the collator of each chain.
        type ContainerChains: GetContainerChainsWithCollators<Self::AccountId>;

        /// Hard limit on number of container chains with collators. Used to define bounded storage.
        type MaxContainerChains: Get<u32>;

        /// Get block author for self chain
        type GetSelfChainBlockAuthor: MaybeSelfChainBlockAuthor<Self::AccountId>;

        /// Inflation rate per orchestrator block (proportion of the total issuance)
        #[pallet::constant]
        type InflationRate: Get<Perbill>;

        /// What to do with the new supply not dedicated to staking
        type OnUnbalanced: OnUnbalanced<CreditOf<Self>>;

        /// The account that will store rewards waiting to be paid out
        #[pallet::constant]
        type PendingRewardsAccount: Get<Self::AccountId>;

        /// Staking rewards distribution implementation
        type StakingRewardsDistributor: DistributeRewards<Self::AccountId, CreditOf<Self>>;

        /// Proportion of the new supply dedicated to staking
        #[pallet::constant]
        type RewardsPortion: Get<Perbill>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Rewarding orchestrator author
        RewardedOrchestrator {
            account_id: T::AccountId,
            balance: BalanceOf<T>,
        },
        /// Rewarding container author
        RewardedContainer {
            account_id: T::AccountId,
            para_id: ParaId,
            balance: BalanceOf<T>,
        },
    }

    /// Container chains to reward per block
    #[pallet::storage]
    pub(super) type ChainsToReward<T: Config> =
        StorageValue<_, ChainsToRewardValue<T>, OptionQuery>;

    #[derive(
        Clone, Encode, Decode, PartialEq, sp_core::RuntimeDebug, scale_info::TypeInfo, MaxEncodedLen,
    )]
    #[scale_info(skip_type_params(T))]
    pub struct ChainsToRewardValue<T: Config> {
        pub para_ids: BoundedVec<ParaId, <T as Config>::MaxContainerChains>,
        pub rewards_per_chain: BalanceOf<T>,
    }

    impl<T: Config> Pallet<T> {
        fn reward_orchestrator_author(orchestrator_author: T::AccountId) -> Weight {
            let mut total_weight = T::DbWeight::get().reads(1);
            if let Some(chains_to_reward) = ChainsToReward::<T>::get() {
                total_weight.saturating_accrue(T::DbWeight::get().reads(1));
                match T::StakingRewardsDistributor::distribute_rewards(
                    orchestrator_author.clone(),
                    T::Currency::withdraw(
                        &T::PendingRewardsAccount::get(),
                        chains_to_reward.rewards_per_chain,
                        Precision::BestEffort,
                        Preservation::Expendable,
                        Fortitude::Force,
                    )
                    .unwrap_or(CreditOf::<T>::zero()),
                ) {
                    Ok(frame_support::dispatch::PostDispatchInfo { actual_weight, .. }) => {
                        Self::deposit_event(Event::RewardedOrchestrator {
                            account_id: orchestrator_author,
                            balance: chains_to_reward.rewards_per_chain,
                        });

                        if let Some(weight) = actual_weight {
                            total_weight.saturating_accrue(weight)
                        }
                    }
                    Err(e) => {
                        log::debug!("Fail to distribute rewards: {:?}", e)
                    }
                }
            } else {
                panic!("ChainsToReward not filled");
            }
            total_weight
        }

        pub fn container_chains_to_reward() -> Option<ChainsToRewardValue<T>> {
            ChainsToReward::<T>::get()
        }
    }
}

// This function should only be used to **reward** a container author.
// There will be no additional check other than checking if we have already
// rewarded this author for **in this tanssi block**
// Any additional check should be done in the calling function
impl<T: Config> AuthorNotingHook<T::AccountId> for Pallet<T> {
    fn on_container_authors_noted(info: &[AuthorNotingInfo<T::AccountId>]) -> Weight {
        if info.is_empty() {
            return Weight::zero();
        }
        let mut total_weight = T::DbWeight::get().reads_writes(1, 0);
        // We take chains to reward, to see what containers are left to reward
        if let Some(mut container_chains_to_reward) = ChainsToReward::<T>::get() {
            for info in info {
                let author = &info.author;
                let para_id = info.para_id;

                // If we find the index is because we still have not rewarded it
                if let Ok(index) = container_chains_to_reward.para_ids.binary_search(&para_id) {
                    // we distribute rewards to the author
                    match T::StakingRewardsDistributor::distribute_rewards(
                        author.clone(),
                        T::Currency::withdraw(
                            &T::PendingRewardsAccount::get(),
                            container_chains_to_reward.rewards_per_chain,
                            Precision::BestEffort,
                            Preservation::Expendable,
                            Fortitude::Force,
                        )
                        .unwrap_or(CreditOf::<T>::zero()),
                    ) {
                        Ok(frame_support::dispatch::PostDispatchInfo { actual_weight, .. }) => {
                            Self::deposit_event(Event::RewardedContainer {
                                account_id: author.clone(),
                                balance: container_chains_to_reward.rewards_per_chain,
                                para_id,
                            });
                            if let Some(weight) = actual_weight {
                                total_weight.saturating_accrue(weight)
                            }
                        }
                        Err(e) => {
                            log::warn!("Fail to distribute rewards: {:?}", e)
                        }
                    }
                    // we remove the para id from container-chains to reward
                    // this makes sure we dont reward it twice in the same block
                    container_chains_to_reward.para_ids.remove(index);
                }
            }

            total_weight.saturating_accrue(T::DbWeight::get().writes(1));
            // Keep track of chains to reward
            ChainsToReward::<T>::put(container_chains_to_reward);
        } else {
            // TODO: why would ChainsToReward ever be None?
            log::warn!("ChainsToReward is None");
        }

        total_weight
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn prepare_worst_case_for_bench(_a: &T::AccountId, _b: BlockNumber, para_id: ParaId) {
        // arbitrary amount to perform rewarding
        // we mint twice as much to the rewards account to make it possible
        let reward_amount = 1_000_000_000u32;
        let mint = reward_amount.saturating_mul(2);

        T::Currency::resolve(
            &T::PendingRewardsAccount::get(),
            T::Currency::issue(BalanceOf::<T>::from(mint)),
        )
        .expect("to mint tokens");

        // TODO: this API doesn't make sense, we want to add a para id to the list of chains to
        // reward. So change `prepare_worst_case_for_bench` into something that takes a list
        let old_para_ids: alloc::vec::Vec<_> = ChainsToReward::<T>::get()
            .map(|x| x.para_ids.into_iter().collect())
            .unwrap_or_default();
        ChainsToReward::<T>::put(ChainsToRewardValue {
            para_ids: alloc::vec::Vec::from_iter(
                old_para_ids.into_iter().chain([para_id]),
            )
            .try_into()
            .expect("to be in bound"),
            rewards_per_chain: BalanceOf::<T>::from(reward_amount),
        });

        T::StakingRewardsDistributor::prepare_worst_case_for_bench(a);
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn bench_advance_block() {
        T::StakingRewardsDistributor::bench_advance_block()
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn bench_execute_pending() {
        T::StakingRewardsDistributor::bench_execute_pending()
    }
}
