use {
    super::constants::{
        accounts::{ALICE, BOB, RANDOM},
        frontier_template, simple_template,
    },
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

pub struct DancelightEthMockBridgeHandler;
// TODO: consider moving to emulated/common
impl BridgeMessageHandler for DancelightEthMockBridgeHandler {
    fn get_source_outbound_messages() -> Vec<BridgeMessage> {
        // Get messages from the outbound queue
        let msgs = DancelightRelay::ext_wrapper(|| {
            snowbridge_pallet_outbound_queue::Messages::<<DancelightRelay as Chain>::Runtime>::get()
        });

        // Store messages in our static mock buffer
        ETH_BRIDGE_SENT_MSGS.with(|sent_msgs| {
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
