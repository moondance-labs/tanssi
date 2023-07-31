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
        Candidate, Config, Delegator, Error, PendingOperationKey, PendingOperationQuery,
        PendingOperations, RequestFilter, Stake, TargetPool,
    },
    frame_support::{
        pallet_prelude::*,
        traits::{fungible::MutateHold, tokens::Precision},
    },
    frame_system::pallet_prelude::*,
    sp_runtime::traits::Zero,
};

pub struct Calls<T>(PhantomData<T>);

impl<T: Config> Calls<T> {
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
        Candidates::<T>::add_total_stake(&candidate, stake.clone())?;

        // We create/mutate a request for joining.
        let now = frame_system::Pallet::<T>::block_number();
        let operation_key = match pool {
            TargetPool::AutoCompounding => PendingOperationKey::JoiningAutoCompounding {
                candidate,
                at_block: now,
            },
            TargetPool::ManualRewards => PendingOperationKey::JoiningManualRewards {
                candidate,
                at_block: now,
            },
        };

        // We store/mutate the operation in storage.
        let operation = PendingOperations::<T>::get(&delegator, &operation_key);
        let operation = operation
            .err_add(&stake.0)
            .map_err(|_| Error::<T>::MathOverflow)?;
        PendingOperations::<T>::set(&delegator, &operation_key, operation);

        // TODO: Event?

        Ok(().into())
    }

    pub fn execute_pending_operations(
        origin: OriginFor<T>,
        operations: Vec<PendingOperationQuery<T::AccountId, T::BlockNumber>>,
    ) -> DispatchResultWithPostInfo {
        // We don't care about the sender.
        let _ = ensure_signed(origin)?;

        for (
            index,
            PendingOperationQuery {
                delegator,
                operation,
            },
        ) in operations.into_iter().enumerate()
        {
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
                        Error::<T>::RequestCannotBeExecuted(index as u32)
                    );

                    Self::execute_joining(
                        &candidate,
                        &delegator,
                        TargetPool::AutoCompounding,
                        Stake(value),
                    )?;

                    PendingOperations::<T>::set(&delegator, &operation, Zero::zero());
                }
                PendingOperationKey::JoiningManualRewards {
                    candidate,
                    at_block,
                } => {
                    ensure!(
                        T::JoiningRequestFilter::can_be_executed(&candidate, &delegator, *at_block),
                        Error::<T>::RequestCannotBeExecuted(index as u32)
                    );

                    Self::execute_joining(
                        &candidate,
                        &delegator,
                        TargetPool::ManualRewards,
                        Stake(value),
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
        stake: Stake<T>,
    ) -> DispatchResultWithPostInfo {
        // Convert stake into shares quantity.
        let shares = match pool {
            TargetPool::AutoCompounding => {
                pools::AutoCompounding::<T>::stake_to_shares_or_init(candidate, stake.clone())?
            }
            TargetPool::ManualRewards => {
                pools::ManualRewards::<T>::stake_to_shares_or_init(candidate, stake.clone())?
            }
        };

        // If stake doesn't allow to get at least one share we release the funds.
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
                pools::AutoCompounding::<T>::add_shares(&candidate, &delegator, shares)?
            }
            TargetPool::ManualRewards => {
                pools::ManualRewards::<T>::add_shares(&candidate, &delegator, shares)?
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

        // TODO: Event?

        Ok(().into())
    }
}
