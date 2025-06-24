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
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::{fungible::Credit, tokens::imbalance::ResolveTo, OnUnbalanced};
use pallet_balances::NegativeImbalance;
#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

pub mod migrations;

#[cfg(feature = "relay")]
pub mod relay;

pub struct DealWithFees<R>(sp_std::marker::PhantomData<R>);
impl<R> OnUnbalanced<Credit<R::AccountId, pallet_balances::Pallet<R>>> for DealWithFees<R>
where
    R: pallet_balances::Config + pallet_treasury::Config + frame_system::Config,
    pallet_treasury::NegativeImbalanceOf<R>: From<NegativeImbalance<R>>,
{
    // this seems to be called for substrate-based transactions
    fn on_unbalanceds(
        mut fees_then_tips: impl Iterator<Item = Credit<R::AccountId, pallet_balances::Pallet<R>>>,
    ) {
        if let Some(fees) = fees_then_tips.next() {
            // 100% of fees & tips goes to the treasury.
            ResolveTo::<pallet_treasury::TreasuryAccountId<R>, pallet_balances::Pallet<R>>::on_unbalanced(fees);

            if let Some(tip) = fees_then_tips.next() {
                ResolveTo::<pallet_treasury::TreasuryAccountId<R>, pallet_balances::Pallet<R>>::on_unbalanced(tip);
            }
        }
    }

    fn on_nonzero_unbalanced(amount: Credit<R::AccountId, pallet_balances::Pallet<R>>) {
        // 100% goes to the treasury
        ResolveTo::<pallet_treasury::TreasuryAccountId<R>, pallet_balances::Pallet<R>>::on_unbalanced(amount);
    }
}
