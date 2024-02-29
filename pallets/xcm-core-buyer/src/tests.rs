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
    crate::{mock::*, *},
    frame_support::{assert_noop, assert_ok},
    sp_runtime::traits::BadOrigin,
};

const ALICE: u64 = 1;

#[test]
fn root_origin_can_force_send_xcm() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            System::set_block_number(1);
            let para_id = 3333.into();

            assert_ok!(XcmCoreBuyer::force_buy_coretime(
                RuntimeOrigin::root(),
                para_id,
            ));

            assert_eq!(events(), vec![Event::CoretimeXcmSent { para_id }]);
        });
}

#[test]
fn signed_origin_cannot_force_send_xcm() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            System::set_block_number(1);
            let para_id = 3333.into();

            assert_noop!(
                XcmCoreBuyer::force_buy_coretime(RuntimeOrigin::signed(ALICE), para_id),
                BadOrigin
            );
        });
}
