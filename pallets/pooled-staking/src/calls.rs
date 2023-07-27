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
        traits::ErrAdd,
        Candidate, Config, Error, PendingOperationKey, PendingOperations, Shares, SharesOrStake,
        Stake, TargetPool,
    },
    frame_support::{
        pallet_prelude::*,
        traits::{
            fungible::{Mutate, MutateHold},
            tokens::Preservation,
        },
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

        Ok(().into())
    }
}
