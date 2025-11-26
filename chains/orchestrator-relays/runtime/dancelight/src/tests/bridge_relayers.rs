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
    crate::{
        bridge_to_ethereum_config::BridgeReward, filter_events, tests::common::*, BridgeRelayers,
        RuntimeEvent, SnowbridgeFeesAccount,
    },
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
                BridgeRelayers::claim_rewards(relayer_origin, BridgeReward::SnowbridgeRewardInbound),
                pallet_bridge_relayers::Error::<Runtime>::NoRewardForRelayer,
            );
        });
}

#[test]
fn relayer_can_claim_reward() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
            (SnowbridgeFeesAccount::get(), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);

            let relayer_origin =
                <Runtime as frame_system::Config>::RuntimeOrigin::signed(AccountId::from(BOB));

            assert_ok!(BridgeRelayers::register(relayer_origin.clone(), 100));
            let reward_params = BridgeReward::SnowbridgeRewardInbound;

            pallet_bridge_relayers::RelayerRewards::<Runtime>::insert(
                AccountId::from(BOB),
                reward_params.clone(),
                100,
            );
            assert_ok!(BridgeRelayers::claim_rewards(
                RuntimeOrigin::signed(AccountId::from(BOB)),
                reward_params,
            ));

            assert_eq!(
                filter_events!(RuntimeEvent::BridgeRelayers(
                    pallet_bridge_relayers::Event::RewardPaid { .. },
                ))
                .count(),
                1,
                "RewardPaid event should be emitted!"
            );
        });
}

#[test]
fn relayer_register_doesnt_withdraw_from_rewards_account() {
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

            let balance_before = System::account(AccountId::from(BOB)).data.free;

            let relayer_origin =
                <Runtime as frame_system::Config>::RuntimeOrigin::signed(AccountId::from(BOB));

            assert_ok!(BridgeRelayers::register(relayer_origin.clone(), 100));

            assert_eq!(
                filter_events!(RuntimeEvent::BridgeRelayers(
                    pallet_bridge_relayers::Event::RegistrationUpdated { .. },
                ))
                .count(),
                1,
                "BridgeRelayers event should be emitted!"
            );

            let balance_after = System::account(AccountId::from(BOB)).data.free;

            assert_eq!(balance_before, balance_after);
        });
}

#[test]
fn relayer_deregister_is_working() {
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

            let balance_before = System::account(AccountId::from(BOB)).data.free;

            let relayer_origin =
                <Runtime as frame_system::Config>::RuntimeOrigin::signed(AccountId::from(BOB));

            let blocks_for_registration_to_be_active = 100;
            assert_ok!(BridgeRelayers::register(
                relayer_origin.clone(),
                blocks_for_registration_to_be_active.clone()
            ));

            assert_eq!(
                filter_events!(RuntimeEvent::BridgeRelayers(
                    pallet_bridge_relayers::Event::RegistrationUpdated { .. },
                ))
                .count(),
                1,
                "BridgeRelayers event should be emitted!"
            );

            // Skip some blocks to make sure registration is not active anymore
            run_to_block(blocks_for_registration_to_be_active + 5);

            assert_ok!(BridgeRelayers::deregister(relayer_origin.clone()));

            assert_eq!(
                filter_events!(RuntimeEvent::BridgeRelayers(
                    pallet_bridge_relayers::Event::Deregistered { .. },
                ))
                .count(),
                1,
                "Deregistered event should be emitted!"
            );

            let balance_after = System::account(AccountId::from(BOB)).data.free;

            assert_eq!(balance_before, balance_after);
        });
}