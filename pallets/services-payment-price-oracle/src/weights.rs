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

// TODO: Generate

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_services_payment_price_oracle.
pub trait WeightInfo {
    fn set_token_price() -> Weight;
}

/// Weights for pallet_services_payment_price_oracle using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    /// Storage: `ServicesPaymentPriceOracle::TokenPriceUsd` (r:0 w:1)
    /// Proof: `ServicesPaymentPriceOracle::TokenPriceUsd` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
    fn set_token_price() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 4_000_000 picoseconds.
        Weight::from_parts(5_000_000, 0)
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
}

impl WeightInfo for () {
    /// Storage: `ServicesPaymentPriceOracle::TokenPriceUsd` (r:0 w:1)
    /// Proof: `ServicesPaymentPriceOracle::TokenPriceUsd` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
    fn set_token_price() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 4_000_000 picoseconds.
        Weight::from_parts(5_000_000, 0)
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }
}
