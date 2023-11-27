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
    crate::{mock::*, Config, *},
    frame_support::{pallet_prelude::*, traits::fungible::Inspect},
    sp_runtime::Permill,
};

fn get_balance(who: &AccountId) -> Balance {
    <<Test as Config>::Currency as Inspect<AccountId>>::balance(who)
}

fn get_total_issuance() -> Balance {
    <<Test as Config>::Currency as Inspect<AccountId>>::total_issuance()
}

#[test]
fn test1() {
    new_test_ext().execute_with(|| {
        // TODO
    });
}
