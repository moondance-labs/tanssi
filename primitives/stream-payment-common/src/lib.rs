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

use core::marker::PhantomData;
use frame_support::traits::{
    fungible::{InspectHold, Mutate, MutateHold},
    tokens::{Precision, Preservation},
};
use frame_support::{Deserialize, Serialize};
use pallet_stream_payment::HoldReason;
use parity_scale_codec::MaxEncodedLen;
use primitives::{AccountId, Balance, BlockNumber};
use sp_runtime::traits::{Block, Header};
use tp_traits::{apply, derive_storage_traits};

tp_traits::alias!(
    pub trait RuntimeConfigs:
        frame_system::Config<AccountId = AccountId, Block: Block<Header: Header<Number = BlockNumber>>> +
        pallet_balances::Config<Balance = Balance, RuntimeHoldReason: From<HoldReason>> +
        pallet_timestamp::Config<Moment = u64>
);

#[apply(derive_storage_traits)]
#[derive(Copy, Serialize, Deserialize, MaxEncodedLen)]
pub enum AssetId {
    Native,
    // TODO: Support more assets like foreign assets
}

pub struct AssetsManager<Runtime>(PhantomData<Runtime>);
impl<Runtime> pallet_stream_payment::AssetsManager<AccountId, AssetId, Balance>
    for AssetsManager<Runtime>
where
    Runtime: RuntimeConfigs,
{
    fn transfer_deposit(
        asset_id: &AssetId,
        from: &AccountId,
        to: &AccountId,
        amount: Balance,
    ) -> frame_support::pallet_prelude::DispatchResult {
        match asset_id {
            AssetId::Native => {
                // We remove the hold before transfering.
                Self::decrease_deposit(asset_id, from, amount)?;
                pallet_balances::Pallet::<Runtime>::transfer(
                    from,
                    to,
                    amount,
                    Preservation::Preserve,
                )
                .map(|_| ())
            }
        }
    }

    fn increase_deposit(
        asset_id: &AssetId,
        account: &AccountId,
        amount: Balance,
    ) -> frame_support::pallet_prelude::DispatchResult {
        match asset_id {
            AssetId::Native => pallet_balances::Pallet::<Runtime>::hold(
                &HoldReason::StreamPayment.into(),
                account,
                amount,
            ),
        }
    }

    fn decrease_deposit(
        asset_id: &AssetId,
        account: &AccountId,
        amount: Balance,
    ) -> frame_support::pallet_prelude::DispatchResult {
        match asset_id {
            AssetId::Native => pallet_balances::Pallet::<Runtime>::release(
                &HoldReason::StreamPayment.into(),
                account,
                amount,
                Precision::Exact,
            )
            .map(|_| ()),
        }
    }

    fn get_deposit(asset_id: &AssetId, account: &AccountId) -> Balance {
        match asset_id {
            AssetId::Native => pallet_balances::Pallet::<Runtime>::balance_on_hold(
                &HoldReason::StreamPayment.into(),
                account,
            ),
        }
    }

    /// Benchmarks: should return the asset id which has the worst performance when interacting
    /// with it.
    #[cfg(feature = "runtime-benchmarks")]
    fn bench_worst_case_asset_id() -> AssetId {
        AssetId::Native
    }

    /// Benchmarks: should return the another asset id which has the worst performance when interacting
    /// with it afther `bench_worst_case_asset_id`. This is to benchmark the worst case when changing config
    /// from one asset to another.
    #[cfg(feature = "runtime-benchmarks")]
    fn bench_worst_case_asset_id2() -> AssetId {
        AssetId::Native
    }

    /// Benchmarks: should set the balance for the asset id returned by `bench_worst_case_asset_id`.
    #[cfg(feature = "runtime-benchmarks")]
    fn bench_set_balance(asset_id: &AssetId, account: &AccountId, amount: Balance) {
        use frame_support::traits::fungible::Mutate;

        // only one asset id
        let AssetId::Native = asset_id;

        pallet_balances::Pallet::<Runtime>::set_balance(account, amount);
    }
}

#[apply(derive_storage_traits)]
#[derive(Copy, Serialize, Deserialize, MaxEncodedLen)]
pub enum TimeUnit {
    BlockNumber,
    Timestamp,
    // TODO: Container chains/relay block number?
}

pub struct TimeProvider<Runtime>(PhantomData<Runtime>);
impl<Runtime> pallet_stream_payment::TimeProvider<TimeUnit, Balance> for TimeProvider<Runtime>
where
    Runtime: RuntimeConfigs,
{
    fn now(unit: &TimeUnit) -> Option<Balance> {
        match *unit {
            TimeUnit::BlockNumber => Some(frame_system::Pallet::<Runtime>::block_number().into()),
            TimeUnit::Timestamp => Some(pallet_timestamp::Pallet::<Runtime>::get().into()),
        }
    }

    /// Benchmarks: should return the time unit which has the worst performance calling
    /// `TimeProvider::now(unit)` with.
    #[cfg(feature = "runtime-benchmarks")]
    fn bench_worst_case_time_unit() -> TimeUnit {
        // Both BlockNumber and Timestamp cost the same (1 db read), but overriding timestamp
        // doesn't work well in benches, while block number works fine.
        TimeUnit::BlockNumber
    }

    /// Benchmarks: sets the "now" time for time unit returned by `worst_case_time_unit`.
    #[cfg(feature = "runtime-benchmarks")]
    fn bench_set_now(instant: Balance) {
        frame_system::Pallet::<Runtime>::set_block_number(instant as u32)
    }
}
