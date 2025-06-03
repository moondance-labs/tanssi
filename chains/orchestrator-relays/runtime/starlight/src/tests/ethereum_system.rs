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
    crate::{tests::common::*, EthereumSystem},
    frame_support::assert_noop,
    pallet_xcm::Origin as XcmOrigin,
    snowbridge_outbound_queue_primitives::OperatingMode,
    sp_core::H160,
    sp_runtime::DispatchError::BadOrigin,
    sp_std::vec,
    xcm::latest::{Junction::Parachain, Location},
};

#[test]
fn test_create_agent_not_allowed() {
    ExtBuilder::default()
        .with_balances(vec![(AccountId::from(ALICE), 210_000 * UNIT)])
        .build()
        .execute_with(|| {
            let sibling_para_location = Location::new(1, [Parachain(2000)]);

            // create_agent is disabled for sibling parachains
            assert_noop!(
                EthereumSystem::create_agent(XcmOrigin::Xcm(sibling_para_location).into()),
                BadOrigin
            );

            let relay_location = Location::new(1, []);

            // create_agent also disabled for relay origin
            assert_noop!(
                EthereumSystem::create_agent(XcmOrigin::Xcm(relay_location).into()),
                BadOrigin
            );

            // create_agent is disabled for signed origins
            assert_noop!(
                EthereumSystem::create_agent(
                    <Runtime as frame_system::Config>::RuntimeOrigin::signed(AccountId::from(
                        ALICE
                    ))
                ),
                BadOrigin
            );

            // create_agent is disabled for root
            assert_noop!(
                EthereumSystem::create_agent(
                    <Runtime as frame_system::Config>::RuntimeOrigin::root()
                ),
                BadOrigin
            );
        })
}

#[test]
fn test_create_channel_not_allowed() {
    ExtBuilder::default()
        .with_balances(vec![(AccountId::from(ALICE), 210_000 * UNIT)])
        .build()
        .execute_with(|| {
            let sibling_para_location = Location::new(1, [Parachain(2000)]);

            // create_channel is disabled for sibling parachains
            assert_noop!(
                EthereumSystem::create_channel(
                    XcmOrigin::Xcm(sibling_para_location).into(),
                    OperatingMode::Normal
                ),
                BadOrigin
            );

            let relay_location = Location::new(1, []);

            // create_channel also disabled for relay origin
            assert_noop!(
                EthereumSystem::create_channel(
                    XcmOrigin::Xcm(relay_location).into(),
                    OperatingMode::Normal
                ),
                BadOrigin
            );

            // create_channel is disabled for signed origins
            assert_noop!(
                EthereumSystem::create_channel(
                    <Runtime as frame_system::Config>::RuntimeOrigin::signed(AccountId::from(
                        ALICE
                    )),
                    OperatingMode::Normal
                ),
                BadOrigin
            );

            // create_channel is disabled for root
            assert_noop!(
                EthereumSystem::create_channel(
                    <Runtime as frame_system::Config>::RuntimeOrigin::root(),
                    OperatingMode::Normal
                ),
                BadOrigin
            );
        })
}

#[test]
fn test_update_channel_not_allowed() {
    ExtBuilder::default()
        .with_balances(vec![(AccountId::from(ALICE), 210_000 * UNIT)])
        .build()
        .execute_with(|| {
            let sibling_para_location = Location::new(1, [Parachain(2000)]);

            // update_channel is disabled for sibling parachains
            assert_noop!(
                EthereumSystem::update_channel(
                    XcmOrigin::Xcm(sibling_para_location).into(),
                    OperatingMode::Normal
                ),
                BadOrigin
            );

            let relay_location = Location::new(1, []);

            // update_channel also disabled for relay origin
            assert_noop!(
                EthereumSystem::update_channel(
                    XcmOrigin::Xcm(relay_location).into(),
                    OperatingMode::Normal
                ),
                BadOrigin
            );

            // update_channel is disabled for signed origins
            assert_noop!(
                EthereumSystem::update_channel(
                    <Runtime as frame_system::Config>::RuntimeOrigin::signed(AccountId::from(
                        ALICE
                    )),
                    OperatingMode::Normal
                ),
                BadOrigin
            );

            // update_channel is disabled for root
            assert_noop!(
                EthereumSystem::update_channel(
                    <Runtime as frame_system::Config>::RuntimeOrigin::root(),
                    OperatingMode::Normal
                ),
                BadOrigin
            );
        })
}

#[test]
fn test_transfer_native_from_agent_not_allowed() {
    ExtBuilder::default()
        .with_balances(vec![(AccountId::from(ALICE), 210_000 * UNIT)])
        .build()
        .execute_with(|| {
            let sibling_para_location = Location::new(1, [Parachain(2000)]);

            // transfer_native_from_agent is disabled for sibling parachains
            assert_noop!(
                EthereumSystem::transfer_native_from_agent(
                    XcmOrigin::Xcm(sibling_para_location).into(),
                    H160::default(),
                    1000u128
                ),
                BadOrigin
            );

            let relay_location = Location::new(1, []);

            // transfer_native_from_agent also disabled for relay origin
            assert_noop!(
                EthereumSystem::transfer_native_from_agent(
                    XcmOrigin::Xcm(relay_location).into(),
                    H160::default(),
                    1000u128
                ),
                BadOrigin
            );

            // transfer_native_from_agent is disabled for signed origins
            assert_noop!(
                EthereumSystem::transfer_native_from_agent(
                    <Runtime as frame_system::Config>::RuntimeOrigin::signed(AccountId::from(
                        ALICE
                    )),
                    H160::default(),
                    1000u128
                ),
                BadOrigin
            );

            // transfer_native_from_agent is disabled for root
            assert_noop!(
                EthereumSystem::transfer_native_from_agent(
                    <Runtime as frame_system::Config>::RuntimeOrigin::root(),
                    H160::default(),
                    1000u128
                ),
                BadOrigin
            );
        })
}
