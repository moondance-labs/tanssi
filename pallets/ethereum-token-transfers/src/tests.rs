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
    super::*,
    crate::mock::*,
    frame_support::{assert_noop, assert_ok},
    snowbridge_core::{AgentId, ChannelId, ParaId},
    sp_runtime::DispatchError::BadOrigin,
};

#[test]
fn test_set_token_transfer_channel_only_callable_by_root() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let channel_id = ChannelId::new([5u8; 32]);
        let agent_id = AgentId::random();
        let para_id: ParaId = 2000u32.into();

        assert_noop!(
            EthereumTokenTransfers::set_token_transfer_channel(
                RuntimeOrigin::signed(ALICE),
                channel_id,
                agent_id,
                para_id
            ),
            BadOrigin
        );

        assert_eq!(ethereum_system_handler_nonce(), 0);

        assert_ok!(EthereumTokenTransfers::set_token_transfer_channel(
            RuntimeOrigin::root(),
            channel_id,
            agent_id,
            para_id
        ));

        let expected_channel_info = ChannelInfo {
            channel_id,
            para_id,
            agent_id,
        };

        System::assert_last_event(RuntimeEvent::EthereumTokenTransfers(
            crate::Event::ChannelInfoSet {
                channel_info: expected_channel_info,
            },
        ));

        assert_eq!(ethereum_system_handler_nonce(), 1);
    });
}

#[test]
fn test_transfer_native_token_channel_id_not_set() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_eq!(CurrentChannelInfo::<Test>::get(), None);

        assert_noop!(
            EthereumTokenTransfers::transfer_native_token(
                RuntimeOrigin::signed(ALICE),
                10u128,
                H160::default(),
            ),
            Error::<Test>::ChannelInfoNotSet
        );
    });
}

#[test]
fn test_transfer_native_token_succeeds() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let channel_id = ChannelId::new([5u8; 32]);
        let agent_id = AgentId::random();
        let para_id: ParaId = 2000u32.into();

        assert_eq!(ethereum_system_handler_nonce(), 0);

        assert_ok!(EthereumTokenTransfers::set_token_transfer_channel(
            RuntimeOrigin::root(),
            channel_id,
            agent_id,
            para_id
        ));

        // No amount transferred to sovereign yet.
        assert_eq!(
            Balances::free_balance(EthereumSovereignAccount::get()),
            0u128
        );
        assert_eq!(Balances::free_balance(FeesAccount::get()), 0u128);

        let alice_balance_before = Balances::free_balance(ALICE);
        assert_eq!(alice_balance_before, 100u128);

        let expected_channel_info = ChannelInfo {
            channel_id,
            para_id,
            agent_id,
        };

        System::assert_last_event(RuntimeEvent::EthereumTokenTransfers(
            crate::Event::ChannelInfoSet {
                channel_info: expected_channel_info,
            },
        ));

        assert_eq!(ethereum_system_handler_nonce(), 1);
        assert_eq!(sent_ethereum_message_nonce(), 0);

        assert_ok!(EthereumTokenTransfers::transfer_native_token(
            RuntimeOrigin::signed(ALICE),
            10u128,
            H160::default(),
        ));

        let expected_token_id = MockTokenIdConvert::convert_back(&TokenLocation::get());

        System::assert_last_event(RuntimeEvent::EthereumTokenTransfers(
            crate::Event::NativeTokenTransferred {
                message_id: Default::default(),
                channel_id,
                source: ALICE,
                recipient: H160::default(),
                token_id: expected_token_id.unwrap(),
                amount: 10u128,
                fee: 50u128,
            },
        ));

        // Alice balance = balance_before - fee - amount_transferred
        assert_eq!(
            Balances::free_balance(ALICE),
            alice_balance_before - 50u128 - 10u128
        );
        assert_eq!(
            Balances::free_balance(EthereumSovereignAccount::get()),
            10u128
        );
        assert_eq!(Balances::free_balance(FeesAccount::get()), 50u128);

        assert_eq!(sent_ethereum_message_nonce(), 1);
    });
}
