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
use {
    frame_support::traits::{EnsureOrigin, Get, OriginTrait, TypedGet},
    frame_system::RawOrigin,
    sp_runtime::traits::AccountIdConversion,
};

pub struct EnsureRootOrPalletIdWithSuccess<AccountId, Success, PalletId>(
    core::marker::PhantomData<(AccountId, Success, PalletId)>,
);

impl<O, AccountId, Success, PalletId> EnsureOrigin<O>
    for EnsureRootOrPalletIdWithSuccess<AccountId, Success, PalletId>
where
    O: OriginTrait<AccountId = AccountId>,
    AccountId: Clone + PartialEq + From<sp_runtime::AccountId32>,
    Success: TypedGet,
    PalletId: Get<frame_support::PalletId>,
{
    type Success = Success::Type;

    fn try_origin(o: O) -> Result<Self::Success, O> {
        let pallet_account_32: sp_runtime::AccountId32 = PalletId::get().into_account_truncating();
        let pallet_account: AccountId = AccountId::from(pallet_account_32);

        match o.as_system_ref() {
            Some(RawOrigin::Root) => Ok(Success::get()),
            Some(RawOrigin::Signed(who)) if *who == pallet_account => Ok(Success::get()),
            _ => Err(o),
        }
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn try_successful_origin() -> Result<O, ()> {
        Ok(O::root())
    }
}
