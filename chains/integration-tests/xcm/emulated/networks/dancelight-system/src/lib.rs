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
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>.

pub use xcm_emulator::{bx, TestExt};
use {
    container_chain_template_simple_runtime::AccountId,
    dancelight_emulated_chain::Dancelight,
    frame_support::parameter_types,
    frontier_template_emulated_chain::FrontierTemplate,
    simple_template_emulated_chain::SimpleTemplate,
    sp_keyring::Sr25519Keyring,
    tanssi_emulated_integration_tests_common::accounts::{ALICE, BOB, RANDOM},
    xcm_emulator::{
        decl_test_networks, Bridge, BridgeLaneId, BridgeMessage, BridgeMessageDispatchError,
        BridgeMessageHandler, Chain, Network,
    },
};

pub struct DancelightEthMockBridge;
impl Bridge for DancelightEthMockBridge {
    type Source = DancelightRelay;
    type Target = ();
    type Handler = DancelightEthMockBridgeHandler;

    fn init() {
        <DancelightRelay as Chain>::Network::init();
    }
}

decl_test_networks! {
    pub struct DancelightMockNet {
        relay_chain = Dancelight,
        parachains = vec![
            FrontierTemplate,
            SimpleTemplate,
        ],
        bridge = DancelightEthMockBridge
    }
}

parameter_types! {
    // Dancelight
    pub DancelightSender: AccountId = Sr25519Keyring::Alice.to_account_id();
    pub DancelightReceiver: AccountId = Sr25519Keyring::Bob.to_account_id();
    pub DancelightEmptyReceiver: AccountId = DancelightRelay::account_id_of(RANDOM);

    // SimpleTemplate
    pub SimpleTemplateSender: AccountId = AccountId::from(ALICE);
    pub SimpleTemplateReceiver: AccountId = AccountId::from(BOB);
    pub SimpleTemplateEmptyReceiver: AccountId = SimpleTemplatePara::account_id_of(RANDOM);
}

pub struct DancelightEthMockBridgeHandler;
impl BridgeMessageHandler for DancelightEthMockBridgeHandler {
    fn get_source_outbound_messages() -> Vec<BridgeMessage> {
        // Get messages from the outbound queue
        let msgs = DancelightRelay::ext_wrapper(|| {
            snowbridge_pallet_outbound_queue::Messages::<<DancelightRelay as Chain>::Runtime>::get()
        });

        // Store messages in our static mock buffer
        tanssi_emulated_integration_tests_common::impls::ETH_BRIDGE_SENT_MSGS.with(|sent_msgs| {
            sent_msgs.borrow_mut().extend(msgs.clone());
        });

        // TODO: We don't check the dispatches messages from the bridge so it's fine to return default value here
        Default::default()
    }

    fn dispatch_target_inbound_message(
        _message: BridgeMessage,
    ) -> Result<(), BridgeMessageDispatchError> {
        unimplemented!("dispatch_target_inbound_message")
    }

    fn notify_source_message_delivery(_lane_id: BridgeLaneId) {
        unimplemented!("notify_source_message_delivery")
    }
}
