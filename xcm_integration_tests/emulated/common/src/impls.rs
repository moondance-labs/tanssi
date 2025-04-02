use {
    dancelight_runtime_constants::currency::UNITS as UNIT,
    emulated_integration_tests_common::xcm_emulator::decl_test_parachains,
    frame_system::Pallet as SystemPallet,
    snowbridge_pallet_outbound_queue::CommittedMessage,
    sp_std::cell::RefCell,
    xcm_emulator::{
        BridgeLaneId, BridgeMessage, BridgeMessageDispatchError, BridgeMessageHandler, Chain,
    },
};

pub fn eth_bridge_sent_msgs() -> Vec<CommittedMessage> {
    ETH_BRIDGE_SENT_MSGS.with(|q| (*q.borrow()).clone())
}

// Store messages sent to ethereum throught the bridge
thread_local! {
    pub static ETH_BRIDGE_SENT_MSGS: RefCell<Vec<CommittedMessage>> = RefCell::new(Vec::new());
}
