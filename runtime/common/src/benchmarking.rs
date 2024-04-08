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

use frame_support::traits::Currency;
use pallet_treasury::ArgumentsFactory;
use sp_std::marker::PhantomData;
pub struct TreasurtBenchmarkHelper<T>(PhantomData<T>);

impl<T> ArgumentsFactory<(), T::AccountId> for TreasurtBenchmarkHelper<T>
where
    T: pallet_treasury::Config,
    T::AccountId: From<[u8; 32]>,
{
    fn create_asset_kind(_seed: u32) -> () {
        ()
    }

    fn create_beneficiary(seed: [u8; 32]) -> T::AccountId {
        let account: T::AccountId = seed.into();
        let balance = T::Currency::minimum_balance();
        let _ = T::Currency::make_free_balance_be(&account, balance);
        account
    }
}
