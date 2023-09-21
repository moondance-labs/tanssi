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

pub type PositiveImbalanceOf<T> = <<T as Config>::Currency as frame_support::traits::Currency<
    <T as frame_system::Config>::AccountId,
>>::PositiveImbalance;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::traits::{fungible, tokens::Balance, Currency, OnUnbalanced};
    use sp_runtime::{traits::Get, Perbill};
    use tp_maths::MulDiv;

    /// Inflation rewards pallet.
    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(core::marker::PhantomData<T>);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Currency: Currency<Self::AccountId>
            + fungible::Inspect<Self::AccountId, Balance = Self::Balance>;

        /// Same as Currency::Balance. Must impl `MulDiv` which perform
        /// multiplication followed by division using a bigger type to avoid
        /// overflows.
        type Balance: Balance + MulDiv;

        /// Inflation rate per block (proportion of the total issuance)
        type InflationRate: Get<Perbill>;

        /// Proportion of the new supply dedicated to staking
        type StakingRewardsPart: Get<Perbill>;

        /// What to do with the new suplly not dedicated to staking
        type OnUnbalanced: OnUnbalanced<PositiveImbalanceOf<Self>>;
    }
}
