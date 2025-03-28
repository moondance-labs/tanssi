use {
    // super::constants::{
    //     accounts::{ALICE, BOB, RANDOM},
    //     frontier_template, simple_template,
    // },
    crate::tests::common::ExtBuilder,
    dancelight_runtime_constants::currency::UNITS as UNIT,
    emulated_integration_tests_common::xcm_emulator::decl_test_parachains,
    frame_support::{parameter_types, traits::OnIdle},
    frame_system::Pallet as SystemPallet,
    snowbridge_pallet_outbound_queue::CommittedMessage,
    sp_std::cell::RefCell,
    xcm_emulator::{
        decl_test_networks, decl_test_relay_chains, Bridge, BridgeLaneId, BridgeMessage,
        BridgeMessageDispatchError, BridgeMessageHandler, Chain, Network, Parachain, RelayChain,
        TestExt,
    },
};

// Store messages sent to ethereum throught the bridge
thread_local! {
    pub static ETH_BRIDGE_SENT_MSGS: RefCell<Vec<CommittedMessage>> = RefCell::new(Vec::new());
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
