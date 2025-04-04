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

use xcm_emulator::decl_test_parachains;
pub use xcm_emulator::{bx, TestExt};
use {
    dancelight_emulated_chain::Dancelight,
    // dancelight_emulated_chain::Dancelight,
    frame_support::parameter_types,
    tanssi_emulated_integration_tests_common,
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

decl_test_parachains! {
    pub struct SimpleTemplateDancelight {
        genesis = simple_template_dancelight_emulated_chain::genesis::genesis(),
        on_init = (),
        runtime = container_chain_template_simple_runtime,
        core = {
            XcmpMessageHandler: container_chain_template_simple_runtime::XcmpQueue,
            LocationToAccountId: container_chain_template_simple_runtime::xcm_config::LocationToAccountId,
            ParachainInfo: container_chain_template_simple_runtime::ParachainInfo,
            MessageOrigin: cumulus_primitives_core::AggregateMessageOrigin,
        },
        pallets = {
            System: container_chain_template_simple_runtime::System,
            Balances: container_chain_template_simple_runtime::Balances,
            ParachainSystem: container_chain_template_simple_runtime::ParachainSystem,
            PolkadotXcm: container_chain_template_simple_runtime::PolkadotXcm,
            ForeignAssets:  container_chain_template_simple_runtime::ForeignAssets,
            AssetRate:  container_chain_template_simple_runtime::AssetRate,
            ForeignAssetsCreator: container_chain_template_simple_runtime::ForeignAssetsCreator,
        }
    }
}

decl_test_networks! {
    pub struct DancelightMockNet {
        relay_chain = Dancelight,
        parachains = vec![
            FrontierTemplateDancelight,
            SimpleTemplateDancelight,
        ],
        bridge = DancelightEthMockBridge
    }
}

decl_test_parachains! {
    // Dancelight parachains
    pub struct FrontierTemplateDancelight {
        genesis = frontier_template_dancelight_emulated_chain::genesis::genesis(),
        on_init = (),
        runtime = container_chain_template_frontier_runtime,
        core = {
            XcmpMessageHandler: container_chain_template_frontier_runtime::XcmpQueue,
            LocationToAccountId: container_chain_template_frontier_runtime::xcm_config::LocationToAccountId,
            ParachainInfo: container_chain_template_frontier_runtime::ParachainInfo,
            MessageOrigin: cumulus_primitives_core::AggregateMessageOrigin,
        },
        pallets = {
            System: container_chain_template_frontier_runtime::System,
            Balances: container_chain_template_frontier_runtime::Balances,
            ParachainSystem: container_chain_template_frontier_runtime::ParachainSystem,
            PolkadotXcm: container_chain_template_frontier_runtime::PolkadotXcm,
            ForeignAssets:  container_chain_template_frontier_runtime::ForeignAssets,
            AssetRate:  container_chain_template_frontier_runtime::AssetRate,
            ForeignAssetsCreator: container_chain_template_frontier_runtime::ForeignAssetsCreator,
        }
    },
}

parameter_types! {
    // Dancelight
    pub DancelightSender: dancelight_runtime::AccountId = dancelight_runtime::AccountId::from(tanssi_emulated_integration_tests_common::accounts::ALICE);
    pub DancelightReceiver: dancelight_runtime::AccountId = dancelight_runtime::AccountId::from(tanssi_emulated_integration_tests_common::accounts::BOB);
    pub DancelightEmptyReceiver: dancelight_runtime::AccountId = DancelightRelay::account_id_of(tanssi_emulated_integration_tests_common::accounts::RANDOM);

    // SimpleTemplate
    pub SimpleTemplateDancelightSender: container_chain_template_simple_runtime::AccountId = dancelight_runtime::AccountId::from(tanssi_emulated_integration_tests_common::accounts::ALICE);
    pub SimpleTemplateDancelightReceiver: container_chain_template_simple_runtime::AccountId = dancelight_runtime::AccountId::from(tanssi_emulated_integration_tests_common::accounts::BOB);
    pub SimpleTemplateDancelightEmptyReceiver: container_chain_template_simple_runtime::AccountId = SimpleTemplateDancelightPara::account_id_of(tanssi_emulated_integration_tests_common::accounts::RANDOM);
}

pub struct DancelightEthMockBridgeHandler;
// TODO: consider moving to emulated/common
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
