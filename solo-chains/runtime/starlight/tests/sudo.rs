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
    crate::common::*,
    frame_support::{assert_noop, assert_ok},
    sp_std::vec,
    starlight_runtime::Sudo,
};

mod common;
const UNIT: Balance = 1_000_000_000_000_000_000;

#[test]
fn sudo_is_set_to_alice_and_can_be_changed() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_sudo(AccountId::from(ALICE))
        .build()
        .execute_with(|| {
            run_to_block(2);
            // Alice should be able to execute this extrinsic
            assert_ok!(Sudo::sudo(
                origin_of(ALICE.into()),
                Box::new(
                    pallet_sudo::Call::set_key {
                        new: AccountId::from(BOB).into()
                    }
                    .into()
                )
            ));

            // Now Bob should be the sudo account. Trying again with Alice should not work
            assert_noop!(
                Sudo::sudo(
                    origin_of(ALICE.into()),
                    Box::new(
                        pallet_sudo::Call::set_key {
                            new: AccountId::from(BOB).into()
                        }
                        .into()
                    )
                ),
                pallet_sudo::Error::<Runtime>::RequireSudo
            );

            assert_ok!(Sudo::sudo(
                origin_of(BOB.into()),
                Box::new(
                    pallet_sudo::Call::set_key {
                        new: AccountId::from(ALICE).into()
                    }
                    .into()
                )
            ));
        });
}
