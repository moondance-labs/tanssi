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
        candidate::Candidates, weights::WeightInfo, Candidate, CandidateSummaries, Config,
        CreditOf, Delegator, DelegatorCandidateSummaries, Error, Event, Pallet, Pools, PoolsKey,
        Shares, Stake,
    },
    core::marker::PhantomData,
    frame_support::{
        ensure,
        pallet_prelude::*,
        traits::{fungible::Balanced, Imbalance},
    },
    serde::{Deserialize, Serialize},
    sp_core::Get,
    sp_runtime::traits::{CheckedAdd, CheckedDiv, Zero},
    tp_maths::{ErrAdd, ErrMul, ErrSub, MulDiv},
};

#[allow(dead_code)]
pub trait Pool<T: Config> {
    /// Get the amount of shares a delegator have for given candidate.
    fn shares(candidate: &Candidate<T>, delegator: &Delegator<T>) -> Shares<T::Balance>;
    /// Get the total amount of shares all delegators have for given candidate.
    fn shares_supply(candidate: &Candidate<T>) -> Shares<T::Balance>;
    /// Get the total amount of currency staked for given candidate / the value of all shares.
    fn total_staked(candidate: &Candidate<T>) -> Stake<T::Balance>;
    /// Get the amount of currency held for that pool in the delegator account.
    fn hold(candidate: &Candidate<T>, delegator: &Delegator<T>) -> Stake<T::Balance>;

    /// Set the amount of shares a delegator have for given candidate.
    fn set_shares(candidate: &Candidate<T>, delegator: &Delegator<T>, value: Shares<T::Balance>);
    /// Set the total amount of shares all delegators have for given candidate.
    fn set_shares_supply(candidate: &Candidate<T>, value: Shares<T::Balance>);
    /// Set the total amount of currency staked for given candidate / the value of all shares.
    fn set_total_staked(candidate: &Candidate<T>, value: Stake<T::Balance>);
    /// Set the amount of currency held for that pool in the delegator account.
    fn set_hold(candidate: &Candidate<T>, delegator: &Delegator<T>, value: Stake<T::Balance>);

    /// Get the initial value of a share in case none exist yet.
    fn initial_share_value() -> Stake<T::Balance>;
    /// Pool variant in PoolKind
    fn pool_kind() -> PoolKind;

    /// Convert an amount of shares to an amount of staked currency for given candidate.
    /// Returns an error if there are no shares for that candidate.
    fn shares_to_stake(
        candidate: &Candidate<T>,
        shares: Shares<T::Balance>,
    ) -> Result<Stake<T::Balance>, Error<T>> {
        let total_staked = Self::total_staked(candidate).0;
        let supply = Self::shares_supply(candidate).0;
        ensure!(!supply.is_zero(), Error::NoOneIsStaking);

        Ok(Stake(shares.0.mul_div(total_staked, supply)?))
    }

    /// Convert an amount of shares to an amount of staked currency for given candidate.
    /// If this candidate have no shares then it uses `initial_share_value` to compute the value.
    fn shares_to_stake_or_init(
        candidate: &Candidate<T>,
        shares: Shares<T::Balance>,
    ) -> Result<Stake<T::Balance>, Error<T>> {
        if Self::total_staked(candidate).0.is_zero() {
            Ok(Stake(shares.0.err_mul(&Self::initial_share_value().0)?))
        } else {
            Self::shares_to_stake(candidate, shares)
        }
    }

    /// Convert an amount of staked currency to an amount of shares for given candidate.
    /// Returns an error if there are no shares for that candidate.
    fn stake_to_shares(
        candidate: &Candidate<T>,
        stake: Stake<T::Balance>,
    ) -> Result<Shares<T::Balance>, Error<T>> {
        let total_staked = Self::total_staked(candidate).0;
        let supply = Self::shares_supply(candidate).0;
        ensure!(!supply.is_zero(), Error::NoOneIsStaking);

        Ok(Shares(stake.0.mul_div(supply, total_staked)?))
    }

    fn computed_stake(
        candidate: &Candidate<T>,
        delegator: &Delegator<T>,
    ) -> Result<Stake<T::Balance>, Error<T>> {
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
        stake: Stake<T::Balance>,
    ) -> Result<Shares<T::Balance>, Error<T>> {
        if Self::total_staked(candidate).0.is_zero() {
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
        stake: Stake<T::Balance>,
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
        stake: Stake<T::Balance>,
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
        shares: Shares<T::Balance>,
    ) -> Result<Stake<T::Balance>, Error<T>> {
        ensure!(!shares.0.is_zero(), Error::StakeMustBeNonZero);

        let stake = Self::shares_to_stake_or_init(candidate, shares)?;

        let old_shares = Self::shares(candidate, delegator);

        let new_shares_supply = Self::shares_supply(candidate).0.err_add(&shares.0)?;
        let new_shares = old_shares.0.err_add(&shares.0)?;
        let new_total_stake = Self::total_staked(candidate).0.err_add(&stake.0)?;

        let new_pool_member = old_shares.0.is_zero();
        let mut new_delegator = false;

        DelegatorCandidateSummaries::<T>::mutate(delegator, candidate, |summary| {
            if summary.is_empty() {
                new_delegator = true;
            }
            summary.set_pool(Self::pool_kind(), true);
        });

        CandidateSummaries::<T>::mutate(candidate, |summary| {
            if new_pool_member {
                let count = summary.pool_delegators_mut(Self::pool_kind());
                *count = count.saturating_add(1);
            }

            if new_delegator {
                summary.delegators = summary.delegators.saturating_add(1);
            }
        });

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
        shares: Shares<T::Balance>,
    ) -> Result<Stake<T::Balance>, Error<T>> {
        ensure!(!shares.0.is_zero(), Error::StakeMustBeNonZero);

        let stake = Self::shares_to_stake(candidate, shares)?;

        let new_shares_supply = Self::shares_supply(candidate).0.err_sub(&shares.0)?;
        let new_shares = Self::shares(candidate, delegator).0.err_sub(&shares.0)?;
        let new_total_stake = Self::total_staked(candidate).0.err_sub(&stake.0)?;

        let rem_pool_member = new_shares.is_zero();
        let mut rem_delegator = false;

        DelegatorCandidateSummaries::<T>::mutate_exists(delegator, candidate, |summary| {
            let mut s = summary.unwrap_or_default();

            if rem_pool_member {
                s.set_pool(Self::pool_kind(), false);
            }

            // storage entry will be deleted if empty
            if s.is_empty() {
                rem_delegator = true;
                *summary = None;
            } else {
                *summary = Some(s)
            };
        });

        CandidateSummaries::<T>::mutate_exists(candidate, |summary| {
            let mut s = summary.unwrap_or_default();

            if rem_pool_member {
                let count = s.pool_delegators_mut(Self::pool_kind());
                *count = count.saturating_sub(1);
            }

            if rem_delegator {
                s.delegators = s.delegators.saturating_sub(1);
            }

            // storage entry will be deleted if empty
            if s.delegators == 0 {
                *summary = None;
            } else {
                *summary = Some(s)
            }
        });

        Self::set_shares_supply(candidate, Shares(new_shares_supply));
        Self::set_shares(candidate, delegator, Shares(new_shares));
        Self::set_total_staked(candidate, Stake(new_total_stake));

        Ok(stake)
    }

    fn increase_hold(
        candidate: &Candidate<T>,
        delegator: &Delegator<T>,
        stake: &Stake<T::Balance>,
    ) -> Result<(), Error<T>> {
        let hold = Self::hold(candidate, delegator);
        let hold = hold.0.err_add(&stake.0)?;
        Self::set_hold(candidate, delegator, Stake(hold));
        Ok(())
    }

    fn decrease_hold(
        candidate: &Candidate<T>,
        delegator: &Delegator<T>,
        stake: &Stake<T::Balance>,
    ) -> Result<(), Error<T>> {
        let hold = Self::hold(candidate, delegator);
        let hold = hold.0.err_sub(&stake.0)?;
        Self::set_hold(candidate, delegator, Stake(hold));
        Ok(())
    }
}

pub fn check_candidate_consistency<T: Config>(candidate: &Candidate<T>) -> Result<(), Error<T>> {
    let total0 = Pools::<T>::get(candidate, &PoolsKey::CandidateTotalStake);

    let joining = Joining::<T>::total_staked(candidate).0;
    let auto = AutoCompounding::<T>::total_staked(candidate).0;
    let manual = ManualRewards::<T>::total_staked(candidate).0;

    let total1 = joining
        .checked_add(&auto)
        .ok_or(Error::InconsistentState)?
        .checked_add(&manual)
        .ok_or(Error::InconsistentState)?;

    ensure!(total0 == total1, Error::InconsistentState);

    Ok(())
}

macro_rules! impl_pool {
    ($name:ident, $shares:ident, $supply:ident, $total:ident, $hold: ident, $init:expr $(,)?) => {
        pub struct $name<T>(PhantomData<T>);
        impl<T: Config> Pool<T> for $name<T> {
            fn shares(candidate: &Candidate<T>, delegator: &Delegator<T>) -> Shares<T::Balance> {
                Shares(Pools::<T>::get(
                    candidate,
                    &PoolsKey::$shares {
                        delegator: delegator.clone(),
                    },
                ))
            }

            fn shares_supply(candidate: &Candidate<T>) -> Shares<T::Balance> {
                Shares(Pools::<T>::get(candidate, &PoolsKey::$supply))
            }

            fn total_staked(candidate: &Candidate<T>) -> Stake<T::Balance> {
                Stake(Pools::<T>::get(candidate, &PoolsKey::$total))
            }

            fn hold(candidate: &Candidate<T>, delegator: &Delegator<T>) -> Stake<T::Balance> {
                Stake(Pools::<T>::get(
                    candidate,
                    &PoolsKey::$hold {
                        delegator: delegator.clone(),
                    },
                ))
            }

            fn set_shares(
                candidate: &Candidate<T>,
                delegator: &Delegator<T>,
                value: Shares<T::Balance>,
            ) {
                Pools::<T>::set(
                    candidate,
                    &PoolsKey::$shares {
                        delegator: delegator.clone(),
                    },
                    value.0,
                )
            }

            fn set_shares_supply(candidate: &Candidate<T>, value: Shares<T::Balance>) {
                Pools::<T>::set(candidate, &PoolsKey::$supply, value.0)
            }

            fn set_total_staked(candidate: &Candidate<T>, value: Stake<T::Balance>) {
                Pools::<T>::set(candidate, &PoolsKey::$total, value.0)
            }

            fn set_hold(
                candidate: &Candidate<T>,
                delegator: &Delegator<T>,
                value: Stake<T::Balance>,
            ) {
                Pools::<T>::set(
                    candidate,
                    &PoolsKey::$hold {
                        delegator: delegator.clone(),
                    },
                    value.0,
                )
            }

            fn initial_share_value() -> Stake<T::Balance> {
                Stake($init)
            }

            fn pool_kind() -> PoolKind {
                PoolKind::$name
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
    if cfg!(test) { 2u32 } else { 1 }.into(),
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
    if cfg!(test) { 3u32 } else { 1u32 }.into(),
);

impl<T: Config> ManualRewards<T> {
    #[allow(dead_code)]
    pub fn pending_rewards(
        candidate: &Candidate<T>,
        delegator: &Delegator<T>,
    ) -> Result<Stake<T::Balance>, Error<T>> {
        let shares = Self::shares(candidate, delegator);

        if shares.0.is_zero() {
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

    /// Increase the rewards of the ManualRewards pool with best effort.
    /// Returns the actual amount distributed (after rounding).
    pub fn increase_rewards(
        candidate: &Candidate<T>,
        rewards: Stake<T::Balance>,
    ) -> Result<Stake<T::Balance>, Error<T>> {
        let Shares(supply) = Self::shares_supply(candidate);
        if supply.is_zero() {
            return Ok(Stake(Zero::zero()));
        }

        let rewards_per_share = rewards
            .0
            .checked_div(&supply)
            .ok_or(Error::<T>::InconsistentState)?;
        if rewards_per_share.is_zero() {
            return Ok(Stake(Zero::zero()));
        }

        let rewards = rewards_per_share.err_mul(&supply)?;

        let counter = Pools::<T>::get(candidate, &PoolsKey::ManualRewardsCounter);
        let counter = counter.err_add(&rewards_per_share)?;
        Pools::<T>::set(candidate, &PoolsKey::ManualRewardsCounter, counter);

        Ok(Stake(rewards))
    }

    pub fn claim_rewards(
        candidate: &Candidate<T>,
        delegator: &Delegator<T>,
    ) -> Result<Stake<T::Balance>, Error<T>> {
        let shares = Self::shares(candidate, delegator);

        let counter = Pools::<T>::get(candidate, &PoolsKey::ManualRewardsCounter);
        let checkpoint = Pools::<T>::get(
            candidate,
            &PoolsKey::ManualRewardsCheckpoint {
                delegator: delegator.clone(),
            },
        );

        // TODO: Should be safe to wrap around.
        let diff = counter.err_sub(&checkpoint)?;

        if diff.is_zero() {
            return Ok(Stake(0u32.into()));
        }

        let rewards = diff.err_mul(&shares.0)?;

        // Update checkpoint
        Pools::<T>::set(
            candidate,
            &PoolsKey::ManualRewardsCheckpoint {
                delegator: delegator.clone(),
            },
            counter,
        );

        Ok(Stake(rewards))
    }
}

/// Perform rewards distribution for the provided candidate.
///
/// The pallet considered that it already posses the rewards in its account,
/// and it is the responsibility of the caller to transfer or mint the currency
/// to the staking pallet account.
///
/// Rewards are split using `RewardsCollatorCommission` between the candidate
/// and all the delegators (including the candidate self-delegation). For each,
/// the rewards are then split according to the value of all the ManualRewards
/// and AutoCompounding shares.
///
/// As candidate rewards will give increase the candidate auto compounding
/// self-delegation, the delegator rewards are distributed first. ManualRewards
/// pool rewards are first distributed by increasing the pool counter, which may
/// result in some rounding. As distributing the AutoCompounding pool rewards
/// consists of simply increasing `AutoCompoundingSharesTotalStaked`, it is not
/// subject to rounding and it can absorb the rounding dust from ManualRewards
/// reward distribution.
///
/// Then it is time to distribute the candidate dedicated rewards. For
/// AutoCompounding, it is as if the candidate received the rewards then
/// self-delegated (instantly). It is thus implemented by creating new
/// AutoCompounding shares. This can lead to some rounding, which will be
/// absorbed in the ManualRewards distribution, which simply consist of
/// transfering the funds to the candidate account.
#[frame_support::transactional]
pub fn distribute_rewards<T: Config>(
    candidate: &Candidate<T>,
    rewards: CreditOf<T>,
) -> DispatchResultWithPostInfo {
    let candidate_manual_rewards = distribute_rewards_inner::<T>(candidate, rewards.peek())?;

    let (candidate_manual_rewards, other_rewards) = rewards.split(candidate_manual_rewards);

    if !candidate_manual_rewards.peek().is_zero() {
        T::Currency::resolve(candidate, candidate_manual_rewards)
            .map_err(|_| DispatchError::NoProviders)?;
    }

    T::Currency::resolve(&T::StakingAccount::get(), other_rewards)
        .map_err(|_| DispatchError::NoProviders)?;

    Ok(Some(T::WeightInfo::distribute_rewards()).into())
}

fn distribute_rewards_inner<T: Config>(
    candidate: &Candidate<T>,
    rewards: T::Balance,
) -> Result<T::Balance, Error<T>> {
    // `RewardsCollatorCommission` is a `Perbill` so we're not worried about overflow.
    let candidate_rewards = T::RewardsCollatorCommission::get() * rewards;
    let delegators_rewards = rewards.err_sub(&candidate_rewards)?;

    let Stake(auto_total_stake) = AutoCompounding::<T>::total_staked(candidate);
    let Stake(manual_total_stake) = ManualRewards::<T>::total_staked(candidate);
    let combined_total_stake = auto_total_stake.err_add(&manual_total_stake)?;

    let candidate_manual_stake = if manual_total_stake.is_zero() {
        Zero::zero()
    } else {
        ManualRewards::<T>::computed_stake(candidate, candidate)?.0
    };

    // Distribute delegators ManualRewards rewards, it implies some rounding.
    let delegators_manual_rewards = if manual_total_stake.is_zero() {
        Zero::zero()
    } else {
        let rewards = delegators_rewards.mul_div(manual_total_stake, combined_total_stake)?;
        ManualRewards::<T>::increase_rewards(candidate, Stake(rewards))?.0
    };

    // Distribute delegators AutoCompounding rewards with dust from ManualRewards.
    // If there is no auto compounding stake but auto compounding rewards it
    // means it comes from manual rewards rounding. Having non-zero total stake
    // with 0 share supply will cause issues, so in this case we distribute this
    // dust as candidate manual rewards.
    let delegators_auto_rewards = delegators_rewards.err_sub(&delegators_manual_rewards)?;
    let (delegators_auto_rewards, delegators_auto_dust) = if !auto_total_stake.is_zero() {
        AutoCompounding::<T>::share_stake_among_holders(candidate, Stake(delegators_auto_rewards))?;
        (delegators_auto_rewards, Zero::zero())
    } else {
        (Zero::zero(), delegators_auto_rewards)
    };

    // Distribute candidate AutoCompounding rewards, it implies some rounding.
    let candidate_auto_rewards = if auto_total_stake.is_zero() {
        Zero::zero()
    } else {
        'a: {
            let candidate_auto_stake =
                AutoCompounding::<T>::computed_stake(candidate, candidate)?.0;
            let candidate_combined_stake = candidate_manual_stake.err_add(&candidate_auto_stake)?;

            if candidate_combined_stake.is_zero() {
                break 'a Zero::zero();
            }

            let rewards =
                candidate_rewards.mul_div(candidate_auto_stake, candidate_combined_stake)?;
            let new_shares = AutoCompounding::<T>::stake_to_shares(candidate, Stake(rewards))?;

            if new_shares.0.is_zero() {
                break 'a Zero::zero();
            }

            AutoCompounding::<T>::add_shares(candidate, candidate, new_shares)?.0
        }
    };

    // Distribute candidate ManualRewards rewards with dust from AutoCompounding.
    // The amount is returned by the function and will be transfered to the candidate account.
    let candidate_manual_rewards = candidate_rewards
        .err_sub(&candidate_auto_rewards)?
        .err_add(&delegators_auto_dust)?;

    let additional_stake = delegators_auto_rewards.err_add(&candidate_auto_rewards)?;
    Candidates::<T>::add_total_stake(candidate, &Stake(additional_stake))?;

    Pallet::<T>::deposit_event(Event::<T>::RewardedCollator {
        collator: candidate.clone(),
        auto_compounding_rewards: candidate_auto_rewards,
        manual_claim_rewards: candidate_manual_rewards,
    });
    Pallet::<T>::deposit_event(Event::<T>::RewardedDelegators {
        collator: candidate.clone(),
        auto_compounding_rewards: delegators_auto_rewards,
        manual_claim_rewards: delegators_manual_rewards,
    });

    Ok(candidate_manual_rewards)
}

const MASK_JOINING: u8 = 1u8;
const MASK_AUTO: u8 = 1u8 << 1;
const MASK_MANUAL: u8 = 1u8 << 2;
const MASK_LEAVING: u8 = 1u8 << 3;

/// Enum of all available pools.
/// Must match with the names of the pool structs generated with `impl_pool!`.
#[derive(
    RuntimeDebug,
    PartialEq,
    Eq,
    Encode,
    Decode,
    DecodeWithMemTracking,
    Copy,
    Clone,
    TypeInfo,
    Serialize,
    Deserialize,
)]
pub enum PoolKind {
    Joining,
    AutoCompounding,
    ManualRewards,
    Leaving,
}

/// Restricted list of pools that represent active delegation. Those can be targetted by joining
/// requests.
#[derive(
    RuntimeDebug,
    PartialEq,
    Eq,
    Encode,
    Decode,
    DecodeWithMemTracking,
    Copy,
    Clone,
    TypeInfo,
    Serialize,
    Deserialize,
)]
pub enum ActivePoolKind {
    AutoCompounding,
    ManualRewards,
}

impl From<ActivePoolKind> for PoolKind {
    fn from(value: ActivePoolKind) -> Self {
        match value {
            ActivePoolKind::AutoCompounding => PoolKind::AutoCompounding,
            ActivePoolKind::ManualRewards => PoolKind::ManualRewards,
        }
    }
}

impl PoolKind {
    fn to_bitmask(self) -> u8 {
        match self {
            Self::Joining => MASK_JOINING,
            Self::AutoCompounding => MASK_AUTO,
            Self::ManualRewards => MASK_MANUAL,
            Self::Leaving => MASK_LEAVING,
        }
    }
}

/// Keeps track of which pools of some candidate a delegator is into.
/// This is used in storage to have a quick way to fetch all the delegations of
/// a delegator, and then know exactly which storage entries to look at to get
/// more detailed info.
#[derive(
    RuntimeDebug,
    Default,
    PartialEq,
    Eq,
    Encode,
    Decode,
    Copy,
    Clone,
    TypeInfo,
    Serialize,
    Deserialize,
    MaxEncodedLen,
)]
pub struct DelegatorCandidateSummary(pub(crate) u8);

impl DelegatorCandidateSummary {
    pub fn new() -> Self {
        Self(0)
    }

    fn bit(&self, mask: u8) -> bool {
        self.0 & mask != 0
    }

    fn set_bit(&mut self, mask: u8, value: bool) {
        if value {
            self.0 |= mask
        } else {
            self.0 &= !mask
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn joining(&self) -> bool {
        self.bit(MASK_JOINING)
    }

    pub fn auto(&self) -> bool {
        self.bit(MASK_AUTO)
    }

    pub fn manual(&self) -> bool {
        self.bit(MASK_MANUAL)
    }

    pub fn leaving(&self) -> bool {
        self.bit(MASK_LEAVING)
    }

    pub fn pool(&self, pool: PoolKind) -> bool {
        self.bit(pool.to_bitmask())
    }

    pub fn set_pool(&mut self, pool: PoolKind, value: bool) {
        self.set_bit(pool.to_bitmask(), value)
    }

    pub fn with_bit(mut self, mask: u8, value: bool) -> Self {
        self.set_bit(mask, value);
        self
    }

    pub fn with_joining(self, value: bool) -> Self {
        self.with_bit(MASK_JOINING, value)
    }

    pub fn with_auto(self, value: bool) -> Self {
        self.with_bit(MASK_AUTO, value)
    }

    pub fn with_manual(self, value: bool) -> Self {
        self.with_bit(MASK_MANUAL, value)
    }

    pub fn with_leaving(self, value: bool) -> Self {
        self.with_bit(MASK_LEAVING, value)
    }

    pub fn with_pool(self, pool: PoolKind, value: bool) -> Self {
        self.with_bit(pool.to_bitmask(), value)
    }
}

/// Keeps track of stats that are useful for dApps and block explorers which can
/// be tricky or expensive to gather otherwise.
#[derive(
    RuntimeDebug,
    Default,
    PartialEq,
    Eq,
    Encode,
    Decode,
    Copy,
    Clone,
    TypeInfo,
    Serialize,
    Deserialize,
    MaxEncodedLen,
)]
pub struct CandidateSummary {
    /// Amount of delegators in any pool without duplicates.
    pub delegators: u32,
    /// Amount of joining delegators. A single delegator joining multiple times
    /// only count once.
    pub joining_delegators: u32,
    /// Amount of delegators in the auto compounding pool.
    pub auto_compounding_delegators: u32,
    /// Amount of delegators in the manual rewards pool.
    pub manual_rewards_delegators: u32,
    /// Amount of leaving delegators. A single delegator leaving multiple times
    //// only count once.
    pub leaving_delegators: u32,
}

impl CandidateSummary {
    pub fn pool_delegators_mut(&mut self, pool: PoolKind) -> &mut u32 {
        match pool {
            PoolKind::Joining => &mut self.joining_delegators,
            PoolKind::AutoCompounding => &mut self.auto_compounding_delegators,
            PoolKind::ManualRewards => &mut self.manual_rewards_delegators,
            PoolKind::Leaving => &mut self.leaving_delegators,
        }
    }

    pub fn with_pool(mut self, pool: PoolKind, count: u32) -> Self {
        let pool = self.pool_delegators_mut(pool);
        *pool = count;
        self
    }
}
