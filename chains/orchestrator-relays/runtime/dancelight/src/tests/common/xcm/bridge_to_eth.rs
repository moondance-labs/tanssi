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
    crate::tests::common::xcm::{
        mocknets::{DancelightRelay as Dancelight, DancelightRelayPallet},
        *,
    },
    frame_support::assert_ok,
    parity_scale_codec::Encode,
    sp_core::H256,
    tp_bridge::Command,
    xcm_emulator::Chain,
};

#[test]
fn send_msg_to_eth() {
    let root_origin = <Dancelight as Chain>::RuntimeOrigin::root();
    let nonce: H256 = sp_core::blake2_256(b"nonce").into();
    let msg_size = 32; // For simplicity since we are hashing the nonce in 32bytes

    // Send message to eth
    Dancelight::execute_with(|| {
        assert_ok!(
            <Dancelight as DancelightRelayPallet>::ExternalValidatorSlashes::root_test_send_msg_to_eth(
                root_origin,
                nonce,
                1,
                msg_size
            )
        );
    });

    // Force process bridge messages
    mocknets::force_process_bridge();

    // xcm command generated by root_test_send_msg_to_eth
    let payload = sp_core::blake2_256((nonce, 0).encode().as_ref()).to_vec();
    let command = Command::Test(payload);

    // msg sent in bridge
    let msgs = mocknets::eth_bridge_sent_msgs();
    let sent_message = msgs.first().unwrap();

    assert_eq!(
        sent_message.channel_id,
        snowbridge_core::PRIMARY_GOVERNANCE_CHANNEL
    );
    assert_eq!(sent_message.command, command.index());
    assert_eq!(sent_message.params, command.abi_encode());
}