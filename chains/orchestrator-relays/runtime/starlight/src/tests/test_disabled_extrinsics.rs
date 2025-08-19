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

#![cfg(test)]
use {
    crate::{tests::common::*, RuntimeCall},
    frame_support::assert_noop,
    sp_runtime::traits::Dispatchable,
};

#[test]
fn test_disabled_some_extrinsics() {
    ExtBuilder::default().build().execute_with(|| {
        run_to_block(2);

        assert_noop!(
            RuntimeCall::Treasury(pallet_treasury::Call::payout { index: 0u32 }).dispatch(
                <Runtime as frame_system::Config>::RuntimeOrigin::signed(AccountId::from(ALICE))
            ),
            frame_system::Error::<Runtime>::CallFiltered
        );
    });
}
