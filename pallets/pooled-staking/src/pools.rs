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
        traits::{ErrAdd, ErrMul, ErrSub, MulDiv},
        Candidate, Config, Delegator, Error, Event, Pallet, Pools, PoolsKey, Shares, Stake,
    },
    core::marker::PhantomData,
    frame_support::ensure,
    sp_core::Get,
    sp_runtime::traits::{CheckedAdd, CheckedDiv, One, Zero},
};

pub trait Pool<T: Config> {
    /// Get the amount of shares a delegator have for given candidate.
    fn shares(candidate: &Candidate<T>, delegator: &Delegator<T>) -> Shares<T>;
    /// Get the total amount of shares all delegators have for given candidate.
    fn shares_supply(candidate: &Candidate<T>) -> Shares<T>;
    /// Get the total amount of currency staked for given candidate / the value of all shares.
    fn total_staked(candidate: &Candidate<T>) -> Stake<T>;
    /// Get the amount of currency held for that pool in the delegator account.
    fn hold(candidate: &Candidate<T>, delegator: &Delegator<T>) -> Stake<T>;

    /// Set the amount of shares a delegator have for given candidate.
    fn set_shares(candidate: &Candidate<T>, delegator: &Delegator<T>, value: Shares<T>);
    /// Set the total amount of shares all delegators have for given candidate.
    fn set_shares_supply(candidate: &Candidate<T>, value: Shares<T>);
    /// Set the total amount of currency staked for given candidate / the value of all shares.
    fn set_total_staked(candidate: &Candidate<T>, value: Stake<T>);
    /// Set the amount of currency held for that pool in the delegator account.
    fn set_hold(candidate: &Candidate<T>, delegator: &Delegator<T>, value: Stake<T>);

    /// Get the initial value of a share in case none exist yet.
    fn initial_share_value() -> Stake<T>;

    /// Convert an amount of shares to an amount of staked currency for given candidate.
    /// Returns an error if there are no shares for that candidate.
    fn shares_to_stake(candidate: &Candidate<T>, shares: Shares<T>) -> Result<Stake<T>, Error<T>> {
        let total_staked = Self::total_staked(candidate).0;
        let supply = Self::shares_supply(candidate).0;
        ensure!(!supply.is_zero(), Error::NoOneIsStaking);

        Ok(Stake(shares.0.mul_div(total_staked, supply)?))
    }

    /// Convert an amount of shares to an amount of staked currency for given candidate.
    /// If this candidate have no shares then it uses `initial_share_value` to compute the value.
    fn shares_to_stake_or_init(
        candidate: &Candidate<T>,
        shares: Shares<T>,
    ) -> Result<Stake<T>, Error<T>> {
        if Zero::is_zero(&Self::total_staked(candidate).0) {
            Ok(Stake(shares.0.err_mul(&Self::initial_share_value().0)?))
        } else {
            Self::shares_to_stake(candidate, shares)
        }
    }

    /// Convert an amount of staked currency to an amount of shares for given candidate.
    /// Returns an error if there are no shares for that candidate.
    fn stake_to_shares(candidate: &Candidate<T>, stake: Stake<T>) -> Result<Shares<T>, Error<T>> {
        let total_staked = Self::total_staked(candidate).0;
        let supply = Self::shares_supply(candidate).0;
        ensure!(!supply.is_zero(), Error::NoOneIsStaking);

        Ok(Shares(stake.0.mul_div(supply, total_staked)?))
    }

    fn computed_stake(
        candidate: &Candidate<T>,
        delegator: &Delegator<T>,
    ) -> Result<Stake<T>, Error<T>> {
        let shares = Self::shares(candidate, delegator);
        if shares.0.is_zero() {
            return Ok(Stake(Zero::zero()));
        }

        Self::shares_to_stake(candidate, shares)
    }

    /// Convert an amount of staked currency to an amount of shares for given candidate.
    /// If this candidate have no shares then it uses `initial_share_value` to compute the value.
    fn stake_to_shares_or_init(
        candidate: &Candidate<T>,
        stake: Stake<T>,
    ) -> Result<Shares<T>, Error<T>> {
        if Zero::is_zero(&Self::total_staked(candidate).0) {
            Ok(Shares(
                stake
                    .0
                    .checked_div(&Self::initial_share_value().0)
                    .ok_or(Error::<T>::InvalidPalletSetting)?,
            ))
        } else {
            Self::stake_to_shares(candidate, stake)
        }
    }

    /// Increase the total stake of a pool without creating new shares, which basically increase
    /// the value of each share.
    fn share_stake_among_holders(
        candidate: &Candidate<T>,
        stake: Stake<T>,
    ) -> Result<(), Error<T>> {
        let total_staked = Self::total_staked(candidate).0;
        let total_staked = total_staked.err_add(&stake.0)?;
        Self::set_total_staked(candidate, Stake(total_staked));
        Ok(())
    }

    /// Decrease the total stake of a pool without creating new shares, which basically decrease
    /// the value of each share.
    fn slash_stake_among_holders(
        candidate: &Candidate<T>,
        stake: Stake<T>,
    ) -> Result<(), Error<T>> {
        let total_staked = Self::total_staked(candidate).0;
        let total_staked = total_staked.err_sub(&stake.0)?;
        Self::set_total_staked(candidate, Stake(total_staked));
        Ok(())
    }

    /// Add new shares for that delegator towards the given candidate.
    /// Function returns the value of those new shares.
    /// Returns an error if underflow/overflows occurs.
    fn add_shares(
        candidate: &Candidate<T>,
        delegator: &Delegator<T>,
        shares: Shares<T>,
    ) -> Result<Stake<T>, Error<T>> {
        ensure!(!Zero::is_zero(&shares.0), Error::StakeMustBeNonZero);

        let stake = Self::shares_to_stake_or_init(candidate, shares.clone())?;

        let new_shares_supply = Self::shares_supply(candidate).0.err_add(&shares.0)?;
        let new_shares = Self::shares(candidate, delegator).0.err_add(&shares.0)?;
        let new_total_stake = Self::total_staked(candidate).0.err_add(&stake.0)?;

        Self::set_shares_supply(candidate, Shares(new_shares_supply));
        Self::set_shares(candidate, delegator, Shares(new_shares));
        Self::set_total_staked(candidate, Stake(new_total_stake));

        Ok(stake)
    }

    /// Remove shares for that delegator towards the given candidate.
    /// Function returns the value of those removed shares.
    /// Returns an error if that delegator don't have enough shares or if underflow/overflows occurs.
    fn sub_shares(
        candidate: &Candidate<T>,
        delegator: &Delegator<T>,
        shares: Shares<T>,
    ) -> Result<Stake<T>, Error<T>> {
        ensure!(!Zero::is_zero(&shares.0), Error::StakeMustBeNonZero);

        let stake = Self::shares_to_stake_or_init(candidate, shares.clone())?;

        let new_shares_supply = Self::shares_supply(candidate).0.err_sub(&shares.0)?;
        let new_shares = Self::shares(candidate, delegator).0.err_sub(&shares.0)?;
        let new_total_stake = Self::total_staked(candidate).0.err_sub(&stake.0)?;

        Self::set_shares_supply(candidate, Shares(new_shares_supply));
        Self::set_shares(candidate, delegator, Shares(new_shares));
        Self::set_total_staked(candidate, Stake(new_total_stake));

        Ok(stake)
    }

    fn increase_hold(
        candidate: &Candidate<T>,
        delegator: &Delegator<T>,
        stake: &Stake<T>,
    ) -> Result<(), Error<T>> {
        let hold = Self::hold(candidate, delegator);
        let hold = hold.0.err_add(&stake.0)?;
        Self::set_hold(candidate, delegator, Stake(hold));
        Ok(())
    }

    fn decrease_hold(
        candidate: &Candidate<T>,
        delegator: &Delegator<T>,
        stake: &Stake<T>,
    ) -> Result<(), Error<T>> {
        let hold = Self::hold(candidate, delegator);
        let hold = hold.0.err_sub(&stake.0)?;
        Self::set_hold(candidate, delegator, Stake(hold));
        Ok(())
    }
}

pub fn check_candidate_consistency<T: Config>(candidate: &Candidate<T>) -> Result<(), Error<T>> {
    let total0 = Pools::<T>::get(candidate, &PoolsKey::CandidateTotalStake);

    let joining = Joining::<T>::total_staked(&candidate).0;
    let auto = AutoCompounding::<T>::total_staked(&candidate).0;
    let manual = ManualRewards::<T>::total_staked(&candidate).0;
    let leaving = Leaving::<T>::total_staked(&candidate).0;

    let total1 = joining
        .checked_add(&auto)
        .ok_or(Error::InconsistentState)?
        .checked_add(&manual)
        .ok_or(Error::InconsistentState)?
        .checked_add(&leaving)
        .ok_or(Error::InconsistentState)?;

    ensure!(total0 == total1, Error::InconsistentState);

    Ok(())
}

macro_rules! impl_pool {
    ($name:ident, $shares:ident, $supply:ident, $total:ident, $hold: ident, $init:expr $(,)?) => {
        pub struct $name<T>(PhantomData<T>);
        impl<T: Config> Pool<T> for $name<T> {
            fn shares(candidate: &Candidate<T>, delegator: &Delegator<T>) -> Shares<T> {
                Shares(Pools::<T>::get(
                    candidate,
                    &PoolsKey::$shares {
                        delegator: delegator.clone(),
                    },
                ))
            }

            fn shares_supply(candidate: &Candidate<T>) -> Shares<T> {
                Shares(Pools::<T>::get(candidate, &PoolsKey::$supply))
            }

            fn total_staked(candidate: &Candidate<T>) -> Stake<T> {
                Stake(Pools::<T>::get(candidate, &PoolsKey::$total))
            }

            fn hold(candidate: &Candidate<T>, delegator: &Delegator<T>) -> Stake<T> {
                Stake(Pools::<T>::get(
                    candidate,
                    &PoolsKey::$hold {
                        delegator: delegator.clone(),
                    },
                ))
            }

            fn set_shares(candidate: &Candidate<T>, delegator: &Delegator<T>, value: Shares<T>) {
                Pools::<T>::set(
                    candidate,
                    &PoolsKey::$shares {
                        delegator: delegator.clone(),
                    },
                    value.0,
                )
            }

            fn set_shares_supply(candidate: &Candidate<T>, value: Shares<T>) {
                Pools::<T>::set(candidate, &PoolsKey::$supply, value.0)
            }

            fn set_total_staked(candidate: &Candidate<T>, value: Stake<T>) {
                Pools::<T>::set(candidate, &PoolsKey::$total, value.0)
            }

            fn set_hold(candidate: &Candidate<T>, delegator: &Delegator<T>, value: Stake<T>) {
                Pools::<T>::set(
                    candidate,
                    &PoolsKey::$hold {
                        delegator: delegator.clone(),
                    },
                    value.0,
                )
            }

            fn initial_share_value() -> Stake<T> {
                Stake($init)
            }
        }
    };
}

impl_pool!(
    Joining,
    JoiningShares,
    JoiningSharesSupply,
    JoiningSharesTotalStaked,
    JoiningSharesHeldStake,
    T::Balance::one(),
);

impl_pool!(
    AutoCompounding,
    AutoCompoundingShares,
    AutoCompoundingSharesSupply,
    AutoCompoundingSharesTotalStaked,
    AutoCompoundingSharesHeldStake,
    T::InitialAutoCompoundingShareValue::get(),
);

impl_pool!(
    ManualRewards,
    ManualRewardsShares,
    ManualRewardsSharesSupply,
    ManualRewardsSharesTotalStaked,
    ManualRewardsSharesHeldStake,
    T::InitialManualClaimShareValue::get(),
);

impl_pool!(
    Leaving,
    LeavingShares,
    LeavingSharesSupply,
    LeavingSharesTotalStaked,
    LeavingSharesHeldStake,
    T::Balance::one(),
);

impl<T: Config> ManualRewards<T> {
    pub fn pending_rewards(
        candidate: &Candidate<T>,
        delegator: &Delegator<T>,
    ) -> Result<Stake<T>, Error<T>> {
        let shares = Self::shares(candidate, delegator);

        if Zero::is_zero(&shares.0) {
            return Ok(Stake(0u32.into()));
        }

        let counter = Pools::<T>::get(candidate, &PoolsKey::ManualRewardsCounter);
        let checkpoint = Pools::<T>::get(
            candidate,
            &PoolsKey::ManualRewardsCheckpoint {
                delegator: delegator.clone(),
            },
        );

        // TODO: Should be safe to wrap around.
        let diff = counter.err_sub(&checkpoint)?;
        Ok(Stake(diff.err_mul(&shares.0)?))
    }

    pub fn claim_rewards(
        candidate: &Candidate<T>,
        delegator: &Delegator<T>,
    ) -> Result<Stake<T>, Error<T>> {
        let shares = Self::shares(candidate, delegator);

        if Zero::is_zero(&shares.0) {
            return Ok(Stake(0u32.into()));
        }

        let counter = Pools::<T>::get(candidate, &PoolsKey::ManualRewardsCounter);
        let checkpoint = Pools::<T>::get(
            candidate,
            &PoolsKey::ManualRewardsCheckpoint {
                delegator: delegator.clone(),
            },
        );

        // TODO: Should be safe to wrap around.
        let diff = counter.err_sub(&checkpoint)?;

        if Zero::is_zero(&diff) {
            return Ok(Stake(0u32.into()));
        }

        let rewards = diff.err_mul(&shares.0)?;

        // Update checkpoint
        Pools::<T>::set(
            candidate,
            &PoolsKey::ManualRewardsCheckpoint {
                delegator: delegator.clone(),
            },
            checkpoint,
        );

        Pallet::<T>::deposit_event(Event::<T>::ClaimedManualRewards {
            candidate: candidate.clone(),
            delegator: delegator.clone(),
            rewards,
        });

        Ok(Stake(rewards))
    }
}
