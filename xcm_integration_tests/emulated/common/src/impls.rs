use {snowbridge_pallet_outbound_queue::CommittedMessage, sp_std::cell::RefCell};

pub fn eth_bridge_sent_msgs() -> Vec<CommittedMessage> {
    ETH_BRIDGE_SENT_MSGS.with(|q| (*q.borrow()).clone())
}

// Store messages sent to ethereum throught the bridge
thread_local! {
    pub static ETH_BRIDGE_SENT_MSGS: RefCell<Vec<CommittedMessage>> = RefCell::new(Vec::new());
}
