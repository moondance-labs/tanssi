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
    common::*,
    container_chain_template_frontier_runtime::EVM,
    frame_support::{assert_noop, assert_ok},
    sp_core::{H256, U256},
    sp_runtime::DispatchError,
    sp_std::vec,
};

mod common;

const UNIT: Balance = 1_000_000_000_000_000_000;

#[test]
fn genesis_balances() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            // Remove ALICE and BOB from collators
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .build()
        .execute_with(|| {
            assert_eq!(
                Balances::usable_balance(AccountId::from(ALICE)),
                210_000 * UNIT,
            );
            assert_eq!(
                Balances::usable_balance(AccountId::from(BOB)),
                100_000 * UNIT,
            );
        });
}

#[test]
fn test_can_call_evm_create_with_root() {
    ExtBuilder::default()
        .with_balances(vec![
            (AccountId::from(ALICE), 100_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            assert_ok!(EVM::create(
                <Runtime as frame_system::Config>::RuntimeOrigin::root(),
                ALICE.into(),
                vec![1u8, 2u8, 3u8],
                U256::zero(),
                1_000_000,
                U256::from(10000000000u128),
                None,
                Some(U256::from(0)),
                Vec::new()
            ));

            assert_ok!(EVM::create2(
                <Runtime as frame_system::Config>::RuntimeOrigin::root(),
                ALICE.into(),
                vec![1u8, 2u8, 3u8],
                H256::from([1u8; 32]),
                U256::zero(),
                1_000_000,
                U256::from(10000000000u128),
                None,
                Some(U256::from(1)),
                Vec::new()
            ));
        });
}

#[test]
fn test_can_call_evm_create_with_allowed_addresses() {
    ExtBuilder::default()
        .with_balances(vec![
            (AccountId::from(ALICE), 100_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            assert_ok!(EVM::create(
                <Runtime as frame_system::Config>::RuntimeOrigin::signed(AccountId::from(ALICE)),
                ALICE.into(),
                vec![1u8, 2u8, 3u8],
                U256::zero(),
                1_000_000,
                U256::from(10000000000u128),
                None,
                Some(U256::from(0)),
                Vec::new()
            ));

            assert_ok!(EVM::create2(
                <Runtime as frame_system::Config>::RuntimeOrigin::signed(AccountId::from(ALICE)),
                ALICE.into(),
                vec![1u8, 2u8, 3u8],
                H256::from([1u8; 32]),
                U256::zero(),
                1_000_000,
                U256::from(10000000000u128),
                None,
                Some(U256::from(1)),
                Vec::new()
            ));

            assert_ok!(EVM::create(
                <Runtime as frame_system::Config>::RuntimeOrigin::signed(AccountId::from(BOB)),
                BOB.into(),
                vec![1u8, 2u8, 3u8],
                U256::zero(),
                1_000_000,
                U256::from(10000000000u128),
                None,
                Some(U256::from(0)),
                Vec::new()
            ));

            assert_ok!(EVM::create2(
                <Runtime as frame_system::Config>::RuntimeOrigin::signed(AccountId::from(BOB)),
                BOB.into(),
                vec![1u8, 2u8, 3u8],
                H256::from([1u8; 32]),
                U256::zero(),
                1_000_000,
                U256::from(10000000000u128),
                None,
                Some(U256::from(1)),
                Vec::new()
            ));
        });
}

#[test]
fn test_cant_call_evm_create_with_not_allowed_address() {
    ExtBuilder::default()
        .with_balances(vec![(AccountId::from(CHARLIE), 100_000 * UNIT)])
        .build()
        .execute_with(|| {
            assert_noop!(
                EVM::create(
                    <Runtime as frame_system::Config>::RuntimeOrigin::signed(AccountId::from(
                        CHARLIE
                    )),
                    CHARLIE.into(),
                    vec![1u8, 2u8, 3u8],
                    U256::zero(),
                    1_000_000,
                    U256::from(10000000000u128),
                    None,
                    Some(U256::from(0)),
                    Vec::new()
                ),
                DispatchError::BadOrigin
            );

            assert_noop!(
                EVM::create2(
                    <Runtime as frame_system::Config>::RuntimeOrigin::signed(AccountId::from(
                        CHARLIE
                    )),
                    CHARLIE.into(),
                    vec![1u8, 2u8, 3u8],
                    H256::from([1u8; 32]),
                    U256::zero(),
                    1_000_000,
                    U256::from(10000000000u128),
                    None,
                    Some(U256::from(1)),
                    Vec::new()
                ),
                DispatchError::BadOrigin
            );
        });
}
