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

//! Runtime API for Services Payment Price Oracle pallet

#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime::FixedU128;

sp_api::decl_runtime_apis! {
    pub trait ServicesPaymentPriceOracleApi<Balance>
    where
        Balance: parity_scale_codec::Codec,
    {
        /// Get the current token price in USD (as FixedU128).
        /// Returns None if price is not set.
        fn token_price_usd() -> Option<FixedU128>;

        /// Get the calculated block production cost in tokens.
        fn block_cost() -> Balance;

        /// Get the calculated collator assignment cost in tokens per session.
        fn collator_assignment_cost() -> Balance;
    }
}
