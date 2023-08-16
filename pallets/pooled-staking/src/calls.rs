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
            fungible::{Balanced, Mutate, MutateHold},
            tokens::{Fortitude, Precision, Preservation},
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
            println!("transfer");
            T::Currency::transfer(
                &T::StakingAccount::get(),
                &delegator,
                dbg!(diff),
                Preservation::Preserve,
            )?;

            // println!("withdraw");
            // let credit = T::Currency::withdraw(
            //     &T::StakingAccount::get(),
            //     dbg!(diff),
            //     Precision::Exact,
            //     Preservation::Preserve,
            //     Fortitude::Force,
            // )?;
            // println!("resolve");
            // T::Currency::resolve(
            //     &delegator,
            //     credit
            // ).map_err(|_| TokenError::BelowMinimum)?;

            println!("hold");
            T::Currency::hold(&T::CurrencyHoldReason::get(), &delegator, diff)?;
            println!("done");
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
        let stake = pools::Joining::<T>::add_shares(&candidate, &delegator, shares)?;

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
            .err_add(&stake.0)
            .map_err(|_| Error::<T>::MathOverflow)?;
        PendingOperations::<T>::set(&delegator, &operation_key, operation);

        Pallet::<T>::deposit_event(Event::<T>::RequestedDelegate {
            candidate,
            delegator,
            towards: pool,
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
        let stake = match pool {
            TargetPool::AutoCompounding => {
                let stake = pools::AutoCompounding::<T>::sub_shares(
                    &candidate,
                    &delegator,
                    Shares(shares),
                )?;
                pools::AutoCompounding::<T>::decrease_hold(&candidate, &delegator, &stake)?;
                stake
            }
            TargetPool::ManualRewards => {
                let stake =
                    pools::ManualRewards::<T>::sub_shares(&candidate, &delegator, Shares(shares))?;
                pools::ManualRewards::<T>::decrease_hold(&candidate, &delegator, &stake)?;
                stake
            }
        };

        todo!()
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
                        &candidate,
                        &delegator,
                        TargetPool::AutoCompounding,
                        Shares(value),
                    )?;

                    PendingOperations::<T>::set(&delegator, &operation, Zero::zero());
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
                        &candidate,
                        &delegator,
                        TargetPool::ManualRewards,
                        Shares(value),
                    )?;

                    PendingOperations::<T>::set(&delegator, &operation, Zero::zero());
                }
                _ => todo!(),
            }
        }

        Ok(().into())
    }

    fn execute_joining(
        candidate: &Candidate<T>,
        delegator: &Delegator<T>,
        pool: TargetPool,
        joining_shares: Shares<T>,
    ) -> DispatchResultWithPostInfo {
        // Convert joining shares into stake.
        let stake = pools::Joining::<T>::sub_shares(candidate, delegator, joining_shares)?;
        pools::Joining::<T>::decrease_hold(candidate, delegator, &stake)?;

        // Convert stake into shares quantity.
        let shares = match pool {
            TargetPool::AutoCompounding => {
                pools::AutoCompounding::<T>::stake_to_shares_or_init(candidate, stake.clone())?
            }
            TargetPool::ManualRewards => {
                pools::ManualRewards::<T>::stake_to_shares_or_init(candidate, stake.clone())?
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
                pools::AutoCompounding::<T>::increase_hold(candidate, delegator, &stake)?;
                stake
            }
            TargetPool::ManualRewards => {
                let stake =
                    pools::ManualRewards::<T>::add_shares(&candidate, &delegator, shares.clone())?;
                pools::ManualRewards::<T>::increase_hold(candidate, delegator, &stake)?;
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
        Candidates::<T>::sub_total_stake(candidate, Stake(release))?;

        let candidate = candidate.clone();
        let delegator = delegator.clone();

        let event = match pool {
            TargetPool::AutoCompounding => Event::<T>::StakedAutoCompounding {
                candidate,
                delegator,
                shares: shares.0,
                stake: actually_staked.0,
            },
            TargetPool::ManualRewards => Event::<T>::StakedManualRewards {
                candidate,
                delegator,
                shares: shares.0,
                stake: actually_staked.0,
            },
        };

        Pallet::<T>::deposit_event(event);

        Ok(().into())
    }
}
