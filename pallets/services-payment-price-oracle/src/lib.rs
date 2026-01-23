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

//! # Services Payment Price Oracle Pallet
//!
//! This pallet stores the token price in USD and provides functions
//! to calculate block production and collator assignment costs based on
//! a fixed monthly USD cost while preserving the ratio between the two services.
//!
//! ## Overview
//!
//! The pallet allows authorized accounts (via sudo) to set the current
//! STAR|TANSSI/USD price. It calculates costs such that:
//! 1. The total monthly cost equals `FixedMonthlyServicesCostUsd`
//! 2. The ratio between block_cost and session_cost is preserved from the
//!    reference values (`ReferenceBlockCost` and `ReferenceSessionCost`)
//!
//! ## Cost Calculation
//!
//! Given reference costs (e.g., 0.03 STAR|TANSSI/block and 50 STAR|TANSSI/session):
//! - Total reference monthly cost = (ref_block_cost * blocks_per_month) + (ref_session_cost * sessions_per_month)
//! - Scale factor = (monthly_cost_usd / token_price) / total_reference_monthly_cost
//! - Actual block_cost = ref_block_cost * scale_factor
//! - Actual session_cost = ref_session_cost * scale_factor
//!
//! This ensures the ratio is preserved while hitting the target monthly USD cost.

#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use weights::WeightInfo;

pub use pallet::*;

use {
    frame_support::pallet_prelude::*,
    frame_system::pallet_prelude::*,
    sp_runtime::{traits::Zero, FixedPointNumber, FixedU128},
};

/// Number of decimals for USD amounts (6 decimals, so $1 = 1_000_000)
pub const USD_DECIMALS: u32 = 6;
/// Seconds per month: 60 sec * 60 min * 24 hours * 30 days
pub const SECONDS_PER_MONTH: u128 = 60 * 60 * 24 * 30;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config<RuntimeEvent: From<Event<Self>>> {
        /// Origin that can set the token price (should be sudo).
        type SetPriceOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        /// Fixed monthly services cost in USD (with USD_DECIMALS precision).
        /// For example, $2000 = 2_000_000_000 (2000 * 10^6)
        #[pallet::constant]
        type FixedMonthlyServicesCostUsd: Get<u128>;

        /// Block time in milliseconds.
        #[pallet::constant]
        type BlockTimeMs: Get<u64>;

        /// Session/Epoch duration in blocks.
        #[pallet::constant]
        type SessionDurationBlocks: Get<u32>;

        /// Token decimals (e.g., 12 for STAR|TANSSI).
        #[pallet::constant]
        type TokenDecimals: Get<u32>;

        /// Reference block production cost in token base units.
        /// This is used to maintain the ratio between block and session costs.
        /// Example: 0.03 STAR|TANSSI = 30_000_000_000 (with 12 decimals)
        #[pallet::constant]
        type ReferenceBlockCost: Get<u128>;

        /// Reference collator assignment cost per session in token base units.
        /// This is used to maintain the ratio between block and session costs.
        /// Example: 50 STAR|TANSSI = 50_000_000_000_000 (with 12 decimals)
        #[pallet::constant]
        type ReferenceSessionCost: Get<u128>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The price cannot be zero.
        PriceCannotBeZero,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Token price has been updated.
        PriceUpdated {
            /// The new price in USD (FixedU128 format).
            new_price: FixedU128,
        },
    }

    /// The current STAR|TANSSI/USD price stored as FixedU128.
    /// Represents how many USD one STAR|TANSSI token is worth.
    #[pallet::storage]
    #[pallet::getter(fn token_price_usd)]
    pub type TokenPriceUsd<T: Config> = StorageValue<_, FixedU128, OptionQuery>;

    #[pallet::genesis_config]
    #[derive(frame_support::DefaultNoBound)]
    pub struct GenesisConfig<T: Config> {
        /// Initial token price in USD (as FixedU128 inner value).
        /// If None, price will not be set at genesis.
        pub initial_price: Option<u128>,
        #[serde(skip)]
        pub _config: core::marker::PhantomData<T>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            if let Some(price) = self.initial_price {
                let fixed_price = FixedU128::from_inner(price);
                if !fixed_price.is_zero() {
                    TokenPriceUsd::<T>::put(fixed_price);
                }
            }
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Set the STAR|TANSSI token price in USD.
        ///
        /// The price is represented as a FixedU128 where the inner value
        /// represents the price with 18 decimal places.
        ///
        /// For example:
        /// - $1.00 = 1_000_000_000_000_000_000 (1 * 10^18)
        /// - $0.50 = 500_000_000_000_000_000 (0.5 * 10^18)
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::set_token_price())]
        pub fn set_token_price(origin: OriginFor<T>, price: FixedU128) -> DispatchResult {
            T::SetPriceOrigin::ensure_origin(origin)?;

            ensure!(!price.is_zero(), Error::<T>::PriceCannotBeZero);

            TokenPriceUsd::<T>::put(price);

            Self::deposit_event(Event::PriceUpdated { new_price: price });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Get the current token price, or None if not set.
        pub fn get_token_price() -> Option<FixedU128> {
            TokenPriceUsd::<T>::get()
        }

        /// Calculate the number of blocks per month based on block time.
        pub fn blocks_per_month() -> u128 {
            let block_time_ms = T::BlockTimeMs::get() as u128;
            if block_time_ms == 0 {
                return 0;
            }
            (SECONDS_PER_MONTH * 1000) / block_time_ms
        }

        /// Calculate the number of sessions per month.
        pub fn sessions_per_month() -> u128 {
            let session_duration = T::SessionDurationBlocks::get() as u128;
            if session_duration == 0 {
                return 0;
            }
            Self::blocks_per_month() / session_duration
        }

        /// Get one token unit based on token decimals.
        pub fn one_token() -> u128 {
            10u128.saturating_pow(T::TokenDecimals::get())
        }

        /// Calculate the total reference monthly cost in tokens.
        /// This is: (ref_block_cost * blocks_per_month) + (ref_session_cost * sessions_per_month)
        fn total_reference_monthly_cost() -> u128 {
            let blocks = Self::blocks_per_month();
            let sessions = Self::sessions_per_month();
            let ref_block_cost = T::ReferenceBlockCost::get();
            let ref_session_cost = T::ReferenceSessionCost::get();

            ref_block_cost
                .saturating_mul(blocks)
                .saturating_add(ref_session_cost.saturating_mul(sessions))
        }

        /// Calculate the scale factor to apply to reference costs.
        ///
        /// scale_factor = (monthly_cost_usd / token_price_usd) / total_reference_monthly_cost
        ///
        /// Returns the scale factor as FixedU128, or None if calculation fails.
        fn calculate_scale_factor() -> Option<FixedU128> {
            let price = Self::get_token_price()?;
            let monthly_cost_usd = T::FixedMonthlyServicesCostUsd::get();
            let total_ref_cost = Self::total_reference_monthly_cost();

            if total_ref_cost == 0 || price.is_zero() {
                return None;
            }

            // Convert monthly_cost_usd to tokens
            // monthly_tokens = monthly_cost_usd / price
            let monthly_tokens_in_usd = Self::usd_to_tokens(monthly_cost_usd, price)?;

            // scale_factor = monthly_tokens / total_ref_cost
            // We use FixedU128 for precision
            let scale = FixedU128::checked_from_rational(monthly_tokens_in_usd, total_ref_cost)?;

            Some(scale)
        }

        /// Calculate the block production cost in tokens.
        ///
        /// block_cost = reference_block_cost * scale_factor
        ///
        /// This preserves the ratio while targeting the monthly USD cost.
        /// Returns None if price is not set or calculations overflow.
        pub fn calculate_block_production_cost() -> Option<u128> {
            let scale_factor = Self::calculate_scale_factor()?;
            let ref_block_cost = T::ReferenceBlockCost::get();

            // block_cost = ref_block_cost * scale_factor
            let cost = scale_factor.saturating_mul_int(ref_block_cost);

            // Ensure we don't return 0 if there's a valid price
            if cost == 0 && !scale_factor.is_zero() {
                Some(1) // Minimum cost of 1 base unit
            } else {
                Some(cost)
            }
        }

        /// Calculate the collator assignment cost in tokens per session.
        ///
        /// session_cost = reference_session_cost * scale_factor
        ///
        /// This preserves the ratio while targeting the monthly USD cost.
        /// Returns None if price is not set or calculations overflow.
        pub fn calculate_collator_assignment_cost() -> Option<u128> {
            let scale_factor = Self::calculate_scale_factor()?;
            let ref_session_cost = T::ReferenceSessionCost::get();

            // session_cost = ref_session_cost * scale_factor
            let cost = scale_factor.saturating_mul_int(ref_session_cost);

            // Ensure we don't return 0 if there's a valid price
            if cost == 0 && !scale_factor.is_zero() {
                Some(1) // Minimum cost of 1 base unit
            } else {
                Some(cost)
            }
        }

        /// Convert USD amount (with USD_DECIMALS precision) to tokens.
        fn usd_to_tokens(usd_amount: u128, price: FixedU128) -> Option<u128> {
            if price.is_zero() {
                return None;
            }

            let one_token = Self::one_token();

            // usd_amount is in USD with USD_DECIMALS (6) precision
            // price is FixedU128 (18 decimals) representing USD per token
            // We want: tokens = usd_amount_in_dollars / price_per_token
            //
            // tokens = (usd_amount / 10^USD_DECIMALS) / price * one_token
            // tokens = usd_amount * one_token / (10^USD_DECIMALS * price)

            let usd_scaled = FixedU128::from_inner(
                usd_amount
                    .checked_mul(FixedU128::DIV)?
                    .checked_div(10u128.pow(USD_DECIMALS))?,
            );

            let tokens_fixed = usd_scaled.checked_div(&price)?;

            // Convert from FixedU128 to token amount
            tokens_fixed
                .into_inner()
                .checked_mul(one_token)?
                .checked_div(FixedU128::DIV)
        }
    }
}
