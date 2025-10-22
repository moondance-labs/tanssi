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
    crate::bridge_to_ethereum_config::BridgeReward,
    crate::{tests::common::*, BridgeRelayers},
    alloc::vec,
    frame_support::{assert_noop, assert_ok},
};

#[test]
fn test_register_new_relayer() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);

            let relayer_origin =
                <Runtime as frame_system::Config>::RuntimeOrigin::signed(AccountId::from(BOB));

            assert_ok!(BridgeRelayers::register(relayer_origin.clone(), 100));

            // We expect 0 rewards for just registered relayer
            assert_noop!(
                BridgeRelayers::claim_rewards(relayer_origin, BridgeReward::Snowbridge),
                pallet_bridge_relayers::Error::<Runtime>::NoRewardForRelayer,
            );
        });
}
