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
        traits::{ErrAdd, ErrSub},
        AllTargetPool, Candidate, Config, Delegator, Error, Event, Pallet, PendingOperationKey,
        PendingOperationQuery, PendingOperations, RequestFilter, Shares, SharesOrStake, Stake,
        TargetPool,
    },
    frame_support::{
        pallet_prelude::*,
        traits::{
            fungible::{Mutate, MutateHold},
            tokens::{Precision, Preservation},
            OriginTrait,
        },
    },
    frame_system::pallet_prelude::*,
    sp_runtime::traits::{CheckedSub, Zero},
};

pub struct Calls<T>(PhantomData<T>);

impl<T: Config> Calls<T> {
    pub fn rebalance_hold(
        _: OriginFor<T>,
        candidate: Candidate<T>,
        delegator: Delegator<T>,
        pool: AllTargetPool,
    ) -> DispatchResultWithPostInfo {
        let (held, stake) = match pool {
            AllTargetPool::Joining => {
                let held = pools::Joining::<T>::hold(&candidate, &delegator);
                let shares = pools::Joining::<T>::shares(&candidate, &delegator);
                let stake = pools::Joining::<T>::shares_to_stake(&candidate, shares)?;
                pools::Joining::<T>::set_hold(&candidate, &delegator, stake.clone());
                (held, stake)
            }
            AllTargetPool::AutoCompounding => {
                let held = pools::AutoCompounding::<T>::hold(&candidate, &delegator);
                let shares = pools::AutoCompounding::<T>::shares(&candidate, &delegator);
                let stake = pools::AutoCompounding::<T>::shares_to_stake(&candidate, shares)?;
                pools::AutoCompounding::<T>::set_hold(&candidate, &delegator, stake.clone());
                (held, stake)
            }
            AllTargetPool::ManualRewards => {
                let held = pools::ManualRewards::<T>::hold(&candidate, &delegator);
                let shares = pools::ManualRewards::<T>::shares(&candidate, &delegator);
                let stake = pools::ManualRewards::<T>::shares_to_stake(&candidate, shares)?;
                pools::ManualRewards::<T>::set_hold(&candidate, &delegator, stake.clone());
                (held, stake)
            }
            AllTargetPool::Leaving => {
                let held = pools::Leaving::<T>::hold(&candidate, &delegator);
                let shares = pools::Leaving::<T>::shares(&candidate, &delegator);
                let stake = pools::Leaving::<T>::shares_to_stake(&candidate, shares)?;
                pools::Leaving::<T>::set_hold(&candidate, &delegator, stake.clone());
                (held, stake)
            }
        };

        // Transfer is done using withdraw to ensure it works regardless of ED.
        if let Some(diff) = stake.0.checked_sub(&held.0) {
            T::Currency::transfer(
                &T::StakingAccount::get(),
                &delegator,
                dbg!(diff),
                Preservation::Preserve,
            )?;
            T::Currency::hold(&T::CurrencyHoldReason::get(), &delegator, diff)?;
            return Ok(().into());
        }

        if let Some(diff) = held.0.checked_sub(&stake.0) {
            T::Currency::release(
                &T::CurrencyHoldReason::get(),
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

        return Ok(().into());
    }

    pub fn request_delegate(
        origin: OriginFor<T>,
        candidate: Candidate<T>,
        pool: TargetPool,
        stake: T::Balance,
    ) -> DispatchResultWithPostInfo {
        let delegator = ensure_signed(origin)?;
        ensure!(!Zero::is_zero(&stake), Error::<T>::StakeMustBeNonZero);

        // Convert stake into joining shares quantity.
        let shares = pools::Joining::<T>::stake_to_shares_or_init(&candidate, Stake(stake))?;

        // If the amount was stake and is less than the value of 1 share it will round down to
        // 0 share. We avoid doing any work for 0 shares.
        ensure!(!Zero::is_zero(&shares.0), Error::<T>::StakeMustBeNonZero);

        // We create the new joining shares. It returns the actual amount of stake those shares
        // represents (due to rounding).
        let stake = pools::Joining::<T>::add_shares(&candidate, &delegator, shares.clone())?;

        // We hold the funds of the delegator and register its stake into the candidate stake.
        T::Currency::hold(&T::CurrencyHoldReason::get(), &delegator, stake.0)?;
        pools::Joining::<T>::increase_hold(&candidate, &delegator, &stake)?;
        Candidates::<T>::add_total_stake(&candidate, &stake)?;

        // We create/mutate a request for joining.
        let now = frame_system::Pallet::<T>::block_number();
        let operation_key = match pool {
            TargetPool::AutoCompounding => PendingOperationKey::JoiningAutoCompounding {
                candidate: candidate.clone(),
                at_block: now,
            },
            TargetPool::ManualRewards => PendingOperationKey::JoiningManualRewards {
                candidate: candidate.clone(),
                at_block: now,
            },
        };

        // We store/mutate the operation in storage.
        let operation = PendingOperations::<T>::get(&delegator, &operation_key);
        let operation = operation
            .err_add(&shares.0)
            .map_err(|_| Error::<T>::MathOverflow)?;
        PendingOperations::<T>::set(&delegator, &operation_key, operation);

        Pallet::<T>::deposit_event(Event::<T>::RequestedDelegate {
            candidate,
            delegator,
            towards: pool,
            pending: stake.0,
        });

        Ok(().into())
    }

    pub fn request_undelegate(
        origin: OriginFor<T>,
        candidate: Candidate<T>,
        pool: TargetPool,
        amount: SharesOrStake<T::Balance>,
    ) -> DispatchResultWithPostInfo {
        let delegator = ensure_signed(origin)?;

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

        // Destroy shares
        let removed_stake = match pool {
            TargetPool::AutoCompounding => {
                let stake = pools::AutoCompounding::<T>::sub_shares(
                    &candidate,
                    &delegator,
                    Shares(shares),
                )?;

                if stake.0 > pools::AutoCompounding::<T>::hold(&candidate, &delegator).0 {
                    Self::rebalance_hold(
                        OriginFor::<T>::none(),
                        candidate.clone(),
                        delegator.clone(),
                        AllTargetPool::AutoCompounding,
                    )?;
                }

                pools::AutoCompounding::<T>::decrease_hold(&candidate, &delegator, &stake)?;
                stake
            }
            TargetPool::ManualRewards => {
                let stake =
                    pools::ManualRewards::<T>::sub_shares(&candidate, &delegator, Shares(shares))?;

                if stake.0 > pools::ManualRewards::<T>::hold(&candidate, &delegator).0 {
                    Self::rebalance_hold(
                        OriginFor::<T>::none(),
                        candidate.clone(),
                        delegator.clone(),
                        AllTargetPool::ManualRewards,
                    )?;
                }

                pools::ManualRewards::<T>::decrease_hold(&candidate, &delegator, &stake)?;
                stake
            }
        };

        // Create leaving shares.
        // As with all pools there will be some rounding error, this amount
        // should be small enough so that it is safe to directly release it
        // in the delegator account.
        let leaving_shares =
            pools::Leaving::<T>::stake_to_shares_or_init(&candidate, removed_stake.clone())?;
        let leaving_stake =
            pools::Leaving::<T>::add_shares(&candidate, &delegator, leaving_shares.clone())?;
        pools::Leaving::<T>::increase_hold(&candidate, &delegator, &leaving_stake)?;

        // We create/mutate a request for leaving.
        let now = frame_system::Pallet::<T>::block_number();
        let operation_key = PendingOperationKey::Leaving {
            candidate: candidate.clone(),
            at_block: now,
        };
        let operation = PendingOperations::<T>::get(&delegator, &operation_key);
        let operation = operation
            .err_add(&leaving_shares.0)
            .map_err(|_| Error::<T>::MathOverflow)?;
        PendingOperations::<T>::set(&delegator, &operation_key, operation);

        // We release the dust if non-zero.
        let dust = removed_stake
            .0
            .err_sub(&leaving_stake.0)
            .map_err(Error::<T>::from)?;

        if dust.is_zero() {
            T::Currency::release(
                &T::CurrencyHoldReason::get(),
                &delegator,
                dust,
                Precision::Exact,
            )?;
            Candidates::<T>::sub_total_stake(&candidate, Stake(dust))?;
        }

        Pallet::<T>::deposit_event(Event::<T>::RequestedUndelegate {
            candidate,
            delegator,
            from: pool,
            pending: leaving_stake.0,
            released: dust,
        });

        Ok(().into())
    }

    pub fn execute_pending_operations(
        origin: OriginFor<T>,
        operations: Vec<PendingOperationQuery<T::AccountId, T::BlockNumber>>,
    ) -> DispatchResultWithPostInfo {
        // We don't care about the sender.
        let _ = ensure_signed(origin)?;

        for (index, query) in operations.into_iter().enumerate() {
            // We deconstruct the query and find the balance associated with it.
            // If it is zero it may not exist or have been executed before, thus
            // we simply skip it instead of erroring.
            let PendingOperationQuery {
                delegator,
                operation,
            } = query;

            let value = PendingOperations::<T>::get(&delegator, &operation);

            if Zero::is_zero(&value) {
                continue;
            }

            match &operation {
                PendingOperationKey::JoiningAutoCompounding {
                    candidate,
                    at_block,
                } => {
                    ensure!(
                        T::JoiningRequestFilter::can_be_executed(&candidate, &delegator, *at_block),
                        Error::<T>::RequestCannotBeExecuted(index as u16)
                    );

                    Self::execute_joining(
                        candidate.clone(),
                        delegator.clone(),
                        TargetPool::AutoCompounding,
                        Shares(value),
                    )?;
                }
                PendingOperationKey::JoiningManualRewards {
                    candidate,
                    at_block,
                } => {
                    ensure!(
                        T::JoiningRequestFilter::can_be_executed(&candidate, &delegator, *at_block),
                        Error::<T>::RequestCannotBeExecuted(index as u16)
                    );

                    Self::execute_joining(
                        candidate.clone(),
                        delegator.clone(),
                        TargetPool::ManualRewards,
                        Shares(value),
                    )?;
                }
                PendingOperationKey::Leaving {
                    candidate,
                    at_block,
                } => {
                    ensure!(
                        T::LeavingRequestFilter::can_be_executed(&candidate, &delegator, *at_block),
                        Error::<T>::RequestCannotBeExecuted(index as u16)
                    );

                    Self::execute_leaving(candidate.clone(), delegator.clone(), Shares(value))?;
                }
            }

            PendingOperations::<T>::set(&delegator, &operation, Zero::zero());
        }

        Ok(().into())
    }

    fn execute_joining(
        candidate: Candidate<T>,
        delegator: Delegator<T>,
        pool: TargetPool,
        joining_shares: Shares<T>,
    ) -> DispatchResultWithPostInfo {
        // Convert joining shares into stake.
        let stake = pools::Joining::<T>::sub_shares(&candidate, &delegator, joining_shares)?;
        pools::Joining::<T>::decrease_hold(&candidate, &delegator, &stake)?;

        // Convert stake into shares quantity.
        let shares = match pool {
            TargetPool::AutoCompounding => {
                pools::AutoCompounding::<T>::stake_to_shares_or_init(&candidate, stake.clone())?
            }
            TargetPool::ManualRewards => {
                pools::ManualRewards::<T>::stake_to_shares_or_init(&candidate, stake.clone())?
            }
        };

        // If stake doesn't allow to get at least one share we release all the funds.
        if Zero::is_zero(&shares.0) {
            T::Currency::release(
                &T::CurrencyHoldReason::get(),
                &delegator,
                stake.0,
                Precision::Exact,
            )?;
            return Ok(().into());
        }

        // We create the new shares. It returns the actual amount of stake those shares
        // represents (due to rounding).
        let actually_staked = match pool {
            TargetPool::AutoCompounding => {
                let stake = pools::AutoCompounding::<T>::add_shares(
                    &candidate,
                    &delegator,
                    shares.clone(),
                )?;
                pools::AutoCompounding::<T>::increase_hold(&candidate, &delegator, &stake)?;
                stake
            }
            TargetPool::ManualRewards => {
                let stake =
                    pools::ManualRewards::<T>::add_shares(&candidate, &delegator, shares.clone())?;
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
            &T::CurrencyHoldReason::get(),
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

        Pallet::<T>::deposit_event(event);

        Pallet::<T>::deposit_event(Event::<T>::ExecutedDelegate {
            candidate,
            delegator,
            towards: pool,
            staked: actually_staked.0,
            released: release,
        });

        Ok(().into())
    }

    fn execute_leaving(
        candidate: Candidate<T>,
        delegator: Delegator<T>,
        leavinig_shares: Shares<T>,
    ) -> DispatchResultWithPostInfo {
        // Convert leaving shares into stake.
        let stake = pools::Leaving::<T>::sub_shares(&candidate, &delegator, leavinig_shares)?;
        pools::Leaving::<T>::decrease_hold(&candidate, &delegator, &stake)?;

        // We release the funds and consider them unstaked.
        T::Currency::release(
            &T::CurrencyHoldReason::get(),
            &delegator,
            stake.0,
            Precision::Exact,
        )?;
        Candidates::<T>::sub_total_stake(&candidate, stake)?;

        Ok(().into())
    }
}
