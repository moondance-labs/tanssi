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

use {snowbridge_pallet_outbound_queue::CommittedMessage, sp_std::cell::RefCell};

pub fn eth_bridge_sent_msgs() -> Vec<CommittedMessage> {
    ETH_BRIDGE_SENT_MSGS.with(|q| (*q.borrow()).clone())
}

// Store messages sent to ethereum throught the bridge
thread_local! {
    pub static ETH_BRIDGE_SENT_MSGS: RefCell<Vec<CommittedMessage>> = RefCell::new(Vec::new());
}
