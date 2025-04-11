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
    snowbridge_core::BasicOperatingMode::Halted,
    sp_core::H160,
    sp_runtime::traits::Dispatchable,
};

#[test]
fn test_disabled_some_extrinsics_for_balances() {
    ExtBuilder::default().build().execute_with(|| {
        run_to_block(2);

        assert_noop!(
            RuntimeCall::Balances(pallet_balances::Call::transfer_allow_death {
                dest: AccountId::from(BOB).into(),
                value: 12345,
            })
            .dispatch(<Runtime as frame_system::Config>::RuntimeOrigin::signed(
                AccountId::from(ALICE)
            )),
            frame_system::Error::<Runtime>::CallFiltered
        );

        assert_noop!(
            RuntimeCall::Balances(pallet_balances::Call::force_transfer {
                source: AccountId::from(ALICE).into(),
                dest: AccountId::from(BOB).into(),
                value: 12345,
            })
            .dispatch(<Runtime as frame_system::Config>::RuntimeOrigin::signed(
                AccountId::from(ALICE)
            )),
            frame_system::Error::<Runtime>::CallFiltered
        );
    });
}

#[test]
fn test_disabled_some_extrinsics_for_bridges() {
    ExtBuilder::default().build().execute_with(|| {
        run_to_block(2);

        assert_noop!(
            RuntimeCall::EthereumSystem(snowbridge_pallet_system::Call::create_agent {}).dispatch(
                <Runtime as frame_system::Config>::RuntimeOrigin::signed(AccountId::from(ALICE))
            ),
            frame_system::Error::<Runtime>::CallFiltered
        );

        assert_noop!(
            RuntimeCall::EthereumOutboundQueue(
                snowbridge_pallet_outbound_queue::Call::set_operating_mode { mode: Halted }
            )
            .dispatch(<Runtime as frame_system::Config>::RuntimeOrigin::signed(
                AccountId::from(ALICE)
            )),
            frame_system::Error::<Runtime>::CallFiltered
        );

        assert_noop!(
            RuntimeCall::EthereumInboundQueue(
                snowbridge_pallet_inbound_queue::Call::set_operating_mode { mode: Halted }
            )
            .dispatch(<Runtime as frame_system::Config>::RuntimeOrigin::signed(
                AccountId::from(ALICE)
            )),
            frame_system::Error::<Runtime>::CallFiltered
        );

        assert_noop!(
            RuntimeCall::EthereumTokenTransfers(
                pallet_ethereum_token_transfers::Call::transfer_native_token {
                    amount: 12345,
                    recipient: H160::random(),
                }
            )
            .dispatch(<Runtime as frame_system::Config>::RuntimeOrigin::signed(
                AccountId::from(ALICE)
            )),
            frame_system::Error::<Runtime>::CallFiltered
        );

        assert_noop!(
            RuntimeCall::EthereumBeaconClient(
                snowbridge_pallet_ethereum_client::Call::set_operating_mode { mode: Halted }
            )
            .dispatch(<Runtime as frame_system::Config>::RuntimeOrigin::signed(
                AccountId::from(ALICE)
            )),
            frame_system::Error::<Runtime>::CallFiltered
        );
    });
}

#[test]
fn test_disabled_some_extrinsics_for_pooled_staking() {
    ExtBuilder::default().build().execute_with(|| {
        run_to_block(2);

        assert_noop!(
            RuntimeCall::PooledStaking(pallet_pooled_staking::Call::update_candidate_position {
                candidates: vec![]
            })
            .dispatch(<Runtime as frame_system::Config>::RuntimeOrigin::signed(
                AccountId::from(ALICE)
            )),
            frame_system::Error::<Runtime>::CallFiltered
        );
    });
}
