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
        candidate::Candidates,
        pools::{self, Pool},
        traits::Timer,
        AllTargetPool, Candidate, Config, Delegator, Error, Event, HoldReason, Pallet,
        PendingOperationKey, PendingOperationQuery, PendingOperationQueryOf, PendingOperations,
        Shares, SharesOrStake, Stake, TargetPool,
    },
    frame_support::{
        dispatch::DispatchErrorWithPostInfo,
        pallet_prelude::*,
        traits::{
            fungible::{Mutate, MutateHold},
            tokens::{Precision, Preservation},
        },
    },
    sp_runtime::traits::{CheckedSub, Zero},
    sp_std::vec::Vec,
    tp_maths::{ErrAdd, ErrSub},
};

pub struct Calls<T>(PhantomData<T>);

impl<T: Config> Calls<T> {
    pub fn rebalance_hold(
        candidate: Candidate<T>,
        delegator: Delegator<T>,
        pool: AllTargetPool,
    ) -> DispatchResultWithPostInfo {
        let (held, stake) = match pool {
            AllTargetPool::Joining => {
                let held = pools::Joining::<T>::hold(&candidate, &delegator);
                let shares = pools::Joining::<T>::shares(&candidate, &delegator);
                let stake = pools::Joining::<T>::shares_to_stake(&candidate, shares)?;
                pools::Joining::<T>::set_hold(&candidate, &delegator, stake);
                (held, stake)
            }
            AllTargetPool::AutoCompounding => {
                let held = pools::AutoCompounding::<T>::hold(&candidate, &delegator);
                let shares = pools::AutoCompounding::<T>::shares(&candidate, &delegator);
                let stake = pools::AutoCompounding::<T>::shares_to_stake(&candidate, shares)?;
                pools::AutoCompounding::<T>::set_hold(&candidate, &delegator, stake);
                (held, stake)
            }
            AllTargetPool::ManualRewards => {
                let held = pools::ManualRewards::<T>::hold(&candidate, &delegator);
                let shares = pools::ManualRewards::<T>::shares(&candidate, &delegator);
                let stake = pools::ManualRewards::<T>::shares_to_stake(&candidate, shares)?;
                pools::ManualRewards::<T>::set_hold(&candidate, &delegator, stake);
                (held, stake)
            }
            AllTargetPool::Leaving => {
                let held = pools::Leaving::<T>::hold(&candidate, &delegator);
                let shares = pools::Leaving::<T>::shares(&candidate, &delegator);
                let stake = pools::Leaving::<T>::shares_to_stake(&candidate, shares)?;
                pools::Leaving::<T>::set_hold(&candidate, &delegator, stake);
                (held, stake)
            }
        };

        if stake == held {
            return Ok(().into());
        }

        if let Some(diff) = stake.0.checked_sub(&held.0) {
            T::Currency::transfer(
                &T::StakingAccount::get(),
                &delegator,
                diff,
                Preservation::Preserve,
            )?;
            T::Currency::hold(&HoldReason::PooledStake.into(), &delegator, diff)?;
            return Ok(().into());
        }

        if let Some(diff) = held.0.checked_sub(&stake.0) {
            T::Currency::release(
                &HoldReason::PooledStake.into(),
                &delegator,
                diff,
                Precision::Exact,
            )?;
            T::Currency::transfer(
                &delegator,
                &T::StakingAccount::get(),
                diff,
                Preservation::Preserve,
            )?;
            return Ok(().into());
        }

        // should be unreachable as diff must either be positive or negative
        Ok(().into())
    }

    pub fn request_delegate(
        candidate: Candidate<T>,
        delegator: Delegator<T>,
        pool: TargetPool,
        stake: T::Balance,
    ) -> DispatchResultWithPostInfo {
        ensure!(!stake.is_zero(), Error::<T>::StakeMustBeNonZero);

        // Convert stake into joining shares quantity.
        let shares = pools::Joining::<T>::stake_to_shares_or_init(&candidate, Stake(stake))?;

        // If the amount was stake and is less than the value of 1 share it will round down to
        // 0 share. We avoid doing any work for 0 shares.
        ensure!(!shares.0.is_zero(), Error::<T>::StakeMustBeNonZero);

        // We create the new joining shares. It returns the actual amount of stake those shares
        // represents (due to rounding).
        let stake = pools::Joining::<T>::add_shares(&candidate, &delegator, shares)?;

        // We hold the funds of the delegator and register its stake into the candidate stake.
        T::Currency::hold(&HoldReason::PooledStake.into(), &delegator, stake.0)?;
        pools::Joining::<T>::increase_hold(&candidate, &delegator, &stake)?;
        Candidates::<T>::add_total_stake(&candidate, &stake)?;

        // We create/mutate a request for joining.
        let now = T::JoiningRequestTimer::now();
        let operation_key = match pool {
            TargetPool::AutoCompounding => PendingOperationKey::JoiningAutoCompounding {
                candidate: candidate.clone(),
                at: now,
            },
            TargetPool::ManualRewards => PendingOperationKey::JoiningManualRewards {
                candidate: candidate.clone(),
                at: now,
            },
        };

        // We store/mutate the operation in storage.
        let operation = PendingOperations::<T>::get(&delegator, &operation_key);
        let operation = operation
            .err_add(&shares.0)
            .map_err(|_| Error::<T>::MathOverflow)?;
        PendingOperations::<T>::set(&delegator, &operation_key, operation);

        pools::check_candidate_consistency::<T>(&candidate)?;

        Pallet::<T>::deposit_event(Event::<T>::RequestedDelegate {
            candidate,
            delegator,
            pool,
            pending: stake.0,
        });

        Ok(().into())
    }

    pub fn request_undelegate(
        candidate: Candidate<T>,
        delegator: Delegator<T>,
        pool: TargetPool,
        amount: SharesOrStake<T::Balance>,
    ) -> DispatchResultWithPostInfo {
        // Converts amount to shares of the correct pool
        let shares = match (amount, pool) {
            (SharesOrStake::Shares(s), _) => s,
            (SharesOrStake::Stake(s), TargetPool::AutoCompounding) => {
                pools::AutoCompounding::<T>::stake_to_shares(&candidate, Stake(s))?.0
            }
            (SharesOrStake::Stake(s), TargetPool::ManualRewards) => {
                pools::ManualRewards::<T>::stake_to_shares(&candidate, Stake(s))?.0
            }
        };

        // Any change in the amount of Manual Rewards shares requires to claim manual rewards.
        if let TargetPool::ManualRewards = pool {
            Self::claim_manual_rewards(&[(candidate.clone(), delegator.clone())])?;
        }

        // Destroy shares
        let removed_stake = Self::destroy_shares(&candidate, &delegator, pool, Shares(shares))?;

        // All this stake no longer contribute to the election of the candidate.
        Candidates::<T>::sub_total_stake(&candidate, removed_stake)?;

        // We proceed with the leaving, which create Leaving shares and request,
        // and release the dust from the convertion to Leaving shares.
        let (leaving_stake, dust) = Self::leave_stake(&candidate, &delegator, removed_stake)?;

        pools::check_candidate_consistency::<T>(&candidate)?;

        Pallet::<T>::deposit_event(Event::<T>::RequestedUndelegate {
            candidate,
            delegator,
            from: pool,
            pending: leaving_stake.0,
            released: dust.0,
        });

        Ok(().into())
    }

    pub fn execute_pending_operations(
        operations: Vec<PendingOperationQueryOf<T>>,
    ) -> DispatchResultWithPostInfo {
        for (index, query) in operations.into_iter().enumerate() {
            // We deconstruct the query and find the balance associated with it.
            // If it is zero it may not exist or have been executed before, thus
            // we simply skip it instead of erroring.
            let PendingOperationQuery {
                delegator,
                operation,
            } = query;

            let value = PendingOperations::<T>::get(&delegator, &operation);

            if value.is_zero() {
                continue;
            }

            match &operation {
                PendingOperationKey::JoiningAutoCompounding { candidate, at } => {
                    ensure!(
                        T::JoiningRequestTimer::is_elapsed(at),
                        Error::<T>::RequestCannotBeExecuted(index as u16)
                    );

                    Self::execute_joining(
                        candidate.clone(),
                        delegator.clone(),
                        TargetPool::AutoCompounding,
                        Shares(value),
                    )?;
                }
                PendingOperationKey::JoiningManualRewards { candidate, at } => {
                    ensure!(
                        T::JoiningRequestTimer::is_elapsed(at),
                        Error::<T>::RequestCannotBeExecuted(index as u16)
                    );

                    Self::execute_joining(
                        candidate.clone(),
                        delegator.clone(),
                        TargetPool::ManualRewards,
                        Shares(value),
                    )?;
                }
                PendingOperationKey::Leaving { candidate, at } => {
                    ensure!(
                        T::LeavingRequestTimer::is_elapsed(at),
                        Error::<T>::RequestCannotBeExecuted(index as u16)
                    );

                    Self::execute_leaving(candidate.clone(), delegator.clone(), Shares(value))?;
                }
            }

            PendingOperations::<T>::remove(&delegator, &operation);
        }

        Ok(().into())
    }

    fn execute_joining(
        candidate: Candidate<T>,
        delegator: Delegator<T>,
        pool: TargetPool,
        joining_shares: Shares<T::Balance>,
    ) -> DispatchResultWithPostInfo {
        // Convert joining shares into stake.
        let stake = pools::Joining::<T>::sub_shares(&candidate, &delegator, joining_shares)?;

        // No rewards are distributed to the Joining pools, so there should always
        // be enough hold. Thus no need to rebalance.
        pools::Joining::<T>::decrease_hold(&candidate, &delegator, &stake)?;

        // Any change in the amount of Manual Rewards shares requires to claim manual rewards.
        if let TargetPool::ManualRewards = pool {
            Self::claim_manual_rewards(&[(candidate.clone(), delegator.clone())])?;
        }

        // Convert stake into shares quantity.
        let shares = match pool {
            TargetPool::AutoCompounding => {
                pools::AutoCompounding::<T>::stake_to_shares_or_init(&candidate, stake)?
            }
            TargetPool::ManualRewards => {
                pools::ManualRewards::<T>::stake_to_shares_or_init(&candidate, stake)?
            }
        };

        // If stake doesn't allow to get at least one share we release all the funds.
        if shares.0.is_zero() {
            T::Currency::release(
                &HoldReason::PooledStake.into(),
                &delegator,
                stake.0,
                Precision::Exact,
            )?;
            Candidates::<T>::sub_total_stake(&candidate, Stake(stake.0))?;
            pools::check_candidate_consistency::<T>(&candidate)?;
            return Ok(().into());
        }

        // We create the new shares. It returns the actual amount of stake those shares
        // represents (due to rounding).
        let actually_staked = match pool {
            TargetPool::AutoCompounding => {
                let stake =
                    pools::AutoCompounding::<T>::add_shares(&candidate, &delegator, shares)?;
                pools::AutoCompounding::<T>::increase_hold(&candidate, &delegator, &stake)?;
                stake
            }
            TargetPool::ManualRewards => {
                let stake = pools::ManualRewards::<T>::add_shares(&candidate, &delegator, shares)?;
                pools::ManualRewards::<T>::increase_hold(&candidate, &delegator, &stake)?;
                stake
            }
        };

        // We release currency that couldn't be converted to shares due to rounding.
        // This thus can reduce slighly the total stake of the candidate.
        let release = stake
            .0
            .err_sub(&actually_staked.0)
            .map_err(|_| Error::<T>::MathUnderflow)?;
        T::Currency::release(
            &HoldReason::PooledStake.into(),
            &delegator,
            release,
            Precision::Exact,
        )?;
        Candidates::<T>::sub_total_stake(&candidate, Stake(release))?;

        // Events
        let event = match pool {
            TargetPool::AutoCompounding => Event::<T>::StakedAutoCompounding {
                candidate: candidate.clone(),
                delegator: delegator.clone(),
                shares: shares.0,
                stake: actually_staked.0,
            },
            TargetPool::ManualRewards => Event::<T>::StakedManualRewards {
                candidate: candidate.clone(),
                delegator: delegator.clone(),
                shares: shares.0,
                stake: actually_staked.0,
            },
        };

        pools::check_candidate_consistency::<T>(&candidate)?;

        Pallet::<T>::deposit_event(event);
        Pallet::<T>::deposit_event(Event::<T>::ExecutedDelegate {
            candidate,
            delegator,
            pool,
            staked: actually_staked.0,
            released: release,
        });

        Ok(().into())
    }

    fn execute_leaving(
        candidate: Candidate<T>,
        delegator: Delegator<T>,
        leavinig_shares: Shares<T::Balance>,
    ) -> DispatchResultWithPostInfo {
        // Convert leaving shares into stake.
        let stake = pools::Leaving::<T>::sub_shares(&candidate, &delegator, leavinig_shares)?;

        // No rewards are distributed to the Leaving pools, so there should always
        // be enough hold. Thus no need to rebalance.
        pools::Leaving::<T>::decrease_hold(&candidate, &delegator, &stake)?;

        // We release the funds and consider them unstaked.
        T::Currency::release(
            &HoldReason::PooledStake.into(),
            &delegator,
            stake.0,
            Precision::Exact,
        )?;

        Pallet::<T>::deposit_event(Event::<T>::ExecutedUndelegate {
            candidate,
            delegator,
            released: stake.0,
        });

        Ok(().into())
    }

    pub fn claim_manual_rewards(
        pairs: &[(Candidate<T>, Delegator<T>)],
    ) -> DispatchResultWithPostInfo {
        for (candidate, delegator) in pairs {
            let Stake(rewards) = pools::ManualRewards::<T>::claim_rewards(candidate, delegator)?;

            if rewards.is_zero() {
                continue;
            }

            T::Currency::transfer(
                &T::StakingAccount::get(),
                delegator,
                rewards,
                Preservation::Preserve,
            )?;

            Pallet::<T>::deposit_event(Event::<T>::ClaimedManualRewards {
                candidate: candidate.clone(),
                delegator: delegator.clone(),
                rewards,
            });
        }

        Ok(().into())
    }

    pub fn update_candidate_position(candidates: &[Candidate<T>]) -> DispatchResultWithPostInfo {
        for candidate in candidates {
            let stake = Candidates::<T>::total_stake(candidate);
            Candidates::<T>::update_total_stake(candidate, stake)?;
        }

        Ok(().into())
    }

    pub fn swap_pool(
        candidate: Candidate<T>,
        delegator: Delegator<T>,
        source_pool: TargetPool,
        amount: SharesOrStake<T::Balance>,
    ) -> DispatchResultWithPostInfo {
        // Converts amount to shares of the correct pool
        let old_shares = match (amount, source_pool) {
            (SharesOrStake::Shares(s), _) => s,
            (SharesOrStake::Stake(s), TargetPool::AutoCompounding) => {
                pools::AutoCompounding::<T>::stake_to_shares(&candidate, Stake(s))?.0
            }
            (SharesOrStake::Stake(s), TargetPool::ManualRewards) => {
                pools::ManualRewards::<T>::stake_to_shares(&candidate, Stake(s))?.0
            }
        };

        // As it will either move in or out of the ManualRewards pool, manual rewards
        // needs to be claimed.
        Self::claim_manual_rewards(&[(candidate.clone(), delegator.clone())])?;

        // Destroy shares from the old pool.
        let removed_stake =
            Self::destroy_shares(&candidate, &delegator, source_pool, Shares(old_shares))?;

        // Convert removed amount to new pool shares.
        let new_shares = match source_pool {
            TargetPool::AutoCompounding => {
                pools::ManualRewards::<T>::stake_to_shares_or_init(&candidate, removed_stake)?
            }
            TargetPool::ManualRewards => {
                pools::AutoCompounding::<T>::stake_to_shares_or_init(&candidate, removed_stake)?
            }
        };

        ensure!(!new_shares.0.is_zero(), Error::<T>::SwapResultsInZeroShares);

        // We create new shares in the new pool. It returns the actual amount of stake those shares
        // represents (due to rounding).
        let actually_staked = match source_pool {
            TargetPool::ManualRewards => {
                let stake =
                    pools::AutoCompounding::<T>::add_shares(&candidate, &delegator, new_shares)?;
                pools::AutoCompounding::<T>::increase_hold(&candidate, &delegator, &stake)?;
                stake
            }
            TargetPool::AutoCompounding => {
                let stake =
                    pools::ManualRewards::<T>::add_shares(&candidate, &delegator, new_shares)?;
                pools::ManualRewards::<T>::increase_hold(&candidate, &delegator, &stake)?;
                stake
            }
        };

        let stake_decrease = removed_stake
            .0
            .err_sub(&actually_staked.0)
            .map_err(Error::<T>::from)?;

        // The left-over no longer contribute to the election of the candidate.
        Candidates::<T>::sub_total_stake(&candidate, Stake(stake_decrease))?;

        // We proceed with the leaving, which create Leaving shares and request,
        // and release the dust from the convertion to Leaving shares.
        let (leaving_stake, dust) = if stake_decrease.is_zero() {
            (Stake(0u32.into()), Stake(0u32.into()))
        } else {
            Self::leave_stake(&candidate, &delegator, Stake(stake_decrease))?
        };

        pools::check_candidate_consistency::<T>(&candidate)?;

        Pallet::<T>::deposit_event(Event::<T>::SwappedPool {
            candidate: candidate.clone(),
            delegator: delegator.clone(),
            source_pool,
            source_shares: old_shares,
            source_stake: removed_stake.0,
            target_shares: new_shares.0,
            target_stake: actually_staked.0,
            pending_leaving: leaving_stake.0,
            released: dust.0,
        });

        Ok(().into())
    }

    /// Destory ManualReward or AutoCompounding shares while performing hold rebalancing if
    /// necessary.
    fn destroy_shares(
        candidate: &Candidate<T>,
        delegator: &Delegator<T>,
        pool: TargetPool,
        shares: Shares<T::Balance>,
    ) -> Result<Stake<T::Balance>, DispatchErrorWithPostInfo> {
        match pool {
            TargetPool::AutoCompounding => {
                let stake = pools::AutoCompounding::<T>::shares_to_stake(candidate, shares)?;

                if stake.0 > pools::AutoCompounding::<T>::hold(candidate, delegator).0 {
                    Self::rebalance_hold(
                        candidate.clone(),
                        delegator.clone(),
                        AllTargetPool::AutoCompounding,
                    )?;
                }

                // This should be the same `stake` as before.
                let stake = pools::AutoCompounding::<T>::sub_shares(candidate, delegator, shares)?;

                pools::AutoCompounding::<T>::decrease_hold(candidate, delegator, &stake)?;
                Ok(stake)
            }
            TargetPool::ManualRewards => {
                let stake = pools::ManualRewards::<T>::shares_to_stake(candidate, shares)?;

                if stake.0 > pools::ManualRewards::<T>::hold(candidate, delegator).0 {
                    Self::rebalance_hold(
                        candidate.clone(),
                        delegator.clone(),
                        AllTargetPool::ManualRewards,
                    )?;
                }

                // This should be the same `stake` as before.
                let stake = pools::ManualRewards::<T>::sub_shares(candidate, delegator, shares)?;

                pools::ManualRewards::<T>::decrease_hold(candidate, delegator, &stake)?;
                Ok(stake)
            }
        }
    }

    /// Perform the leaving proceduce with provided stake, which will create
    /// Leaving shares and request, and release the rounding dust. It DOES NOT
    /// destroy shares in other pools.
    /// Returns a tuple of the amount of stake in the leaving pool and the dust
    /// that was released.
    fn leave_stake(
        candidate: &Candidate<T>,
        delegator: &Delegator<T>,
        stake: Stake<T::Balance>,
    ) -> Result<(Stake<T::Balance>, Stake<T::Balance>), DispatchErrorWithPostInfo> {
        // Create leaving shares.
        // As with all pools there will be some rounding error, this amount
        // should be small enough so that it is safe to directly release it
        // in the delegator account.
        let leaving_shares = pools::Leaving::<T>::stake_to_shares_or_init(candidate, stake)?;
        let leaving_stake = pools::Leaving::<T>::add_shares(candidate, delegator, leaving_shares)?;
        pools::Leaving::<T>::increase_hold(candidate, delegator, &leaving_stake)?;

        // We create/mutate a request for leaving.
        let now = T::LeavingRequestTimer::now();
        let operation_key = PendingOperationKey::Leaving {
            candidate: candidate.clone(),
            at: now,
        };
        let operation = PendingOperations::<T>::get(delegator, &operation_key);
        let operation = operation
            .err_add(&leaving_shares.0)
            .map_err(|_| Error::<T>::MathOverflow)?;
        PendingOperations::<T>::set(delegator, &operation_key, operation);

        // We release the dust if non-zero.
        let dust = stake
            .0
            .err_sub(&leaving_stake.0)
            .map_err(Error::<T>::from)?;

        if !dust.is_zero() {
            T::Currency::release(
                &HoldReason::PooledStake.into(),
                delegator,
                dust,
                Precision::Exact,
            )?;
        }

        Ok((leaving_stake, Stake(dust)))
    }
}
