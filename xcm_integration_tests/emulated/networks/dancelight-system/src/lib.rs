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
use xcm_emulator::{decl_test_parachains, decl_test_relay_chains};
use {
    dancelight_runtime_constants::currency::UNITS as UNIT,
    dancelight_runtime_test_utils::ExtBuilder,
    // dancelight_emulated_chain::Dancelight,
    frame_support::parameter_types,
    tanssi_emulated_integration_tests_common,
    xcm_emulator::{
        decl_test_networks, Bridge, BridgeLaneId, BridgeMessage, BridgeMessageDispatchError,
        BridgeMessageHandler, Chain, Network,
    },
};

decl_test_relay_chains! {
    #[api_version(11)]
    pub struct Dancelight {
        genesis = ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (dancelight_runtime::AccountId::from(tanssi_emulated_integration_tests_common::accounts::ALICE), 210_000 * UNIT),
            (dancelight_runtime::AccountId::from(tanssi_emulated_integration_tests_common::accounts::BOB), 100_000 * UNIT),
        ])
        .with_relay_config(runtime_parachains::configuration::HostConfiguration {
            max_downward_message_size: 1024 * 1024,
            ..Default::default()
        })
        .with_safe_xcm_version(3)
        .build_storage(),
        on_init = (),
        runtime = dancelight_runtime,
        core = {
            SovereignAccountOf: dancelight_runtime::xcm_config::LocationConverter,
        },
        pallets = {
            System: dancelight_runtime::System,
            Session: dancelight_runtime::Session,
            Configuration: dancelight_runtime::Configuration,
            Balances: dancelight_runtime::Balances,
            Registrar: dancelight_runtime::Registrar,
            ParasSudoWrapper: dancelight_runtime::ParasSudoWrapper,
            OnDemandAssignmentProvider: dancelight_runtime::OnDemandAssignmentProvider,
            XcmPallet: dancelight_runtime::XcmPallet,
            Sudo: dancelight_runtime::Sudo,
            MessageQueue: dancelight_runtime::MessageQueue,
            ExternalValidatorSlashes: dancelight_runtime::ExternalValidatorSlashes,
            EthereumOutboundQueue: dancelight_runtime::EthereumOutboundQueue,
            EthereumInboundQueue: dancelight_runtime::EthereumInboundQueue,
            EthereumSystem: dancelight_runtime::EthereumSystem,
            ExternalValidators: dancelight_runtime::ExternalValidators,
        }
    }
}

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
