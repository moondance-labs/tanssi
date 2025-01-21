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
    sp_std::collections::btree_map::BTreeMap,
    tp_bridge::Command,
    tp_traits::{ActiveEraInfo, OnEraEnd, OnEraStart},
};

#[test]
fn test_set_token_transfer_channel_only_callable_by_root() {
    new_test_ext().execute_with(|| {
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

        assert_ok!(EthereumTokenTransfers::set_token_transfer_channel(
            RuntimeOrigin::root(),
            channel_id,
            agent_id,
            para_id
        ));
    });
}

#[test]
fn test_cannot_register_existing_channel_id() {
    new_test_ext().execute_with(|| {
        let channel_id = ChannelId::new([5u8; 32]);
        let agent_id = AgentId::random();
        let para_id: ParaId = 2000u32.into();

        assert_ok!(EthereumTokenTransfers::set_token_transfer_channel(
            RuntimeOrigin::root(),
            channel_id,
            agent_id,
            para_id
        ));

        assert_noop!(
            EthereumTokenTransfers::set_token_transfer_channel(
                RuntimeOrigin::root(),
                channel_id,
                agent_id,
                2001u32.into()
            ),
            Error::<Test>::ChannelIdAlreadyExists
        );
    });
}

#[test]
fn test_cannot_register_existing_para_id() {
    new_test_ext().execute_with(|| {
        let channel_id = ChannelId::new([5u8; 32]);
        let agent_id = AgentId::random();
        let para_id: ParaId = 2000u32.into();

        assert_ok!(EthereumTokenTransfers::set_token_transfer_channel(
            RuntimeOrigin::root(),
            channel_id,
            agent_id,
            para_id
        ));

        let new_channel_id = ChannelId::new([6u8; 32]);

        assert_noop!(
            EthereumTokenTransfers::set_token_transfer_channel(
                RuntimeOrigin::root(),
                new_channel_id,
                agent_id,
                para_id
            ),
            Error::<Test>::ParaIdAlreadyExists
        );
    });
}
