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

use {primitives::AccountId, sp_std::marker::PhantomData, tp_traits::InvulnerablesHelper};

// Common implementation of the InvulnerablesHelper trait for all chains supporting pallet_invulnerables.
pub struct InvulnerablesFilter<Runtime>(PhantomData<Runtime>);
impl<Runtime: pallet_invulnerables::Config<CollatorId = AccountId>> InvulnerablesHelper<AccountId>
    for InvulnerablesFilter<Runtime>
{
    fn is_invulnerable(account_id: &AccountId) -> bool {
        pallet_invulnerables::Pallet::<Runtime>::invulnerables().contains(account_id)
    }
}
