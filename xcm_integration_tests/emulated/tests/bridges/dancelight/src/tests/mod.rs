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

#[cfg(test)]
mod bridge_to_eth;

#[cfg(test)]
mod imports {
    pub use {
        alloy_sol_types::SolEvent,
        dancelight_emulated_chain::DancelightRelayPallet,
        dancelight_runtime::Runtime,
        dancelight_runtime_test_utils::mock_snowbridge_message_proof,
        dancelight_system_emulated_network::DancelightRelay as Dancelight,
        dancelight_system_emulated_network::DancelightSender,
        dancelight_system_emulated_network::SimpleTemplateDancelightPara as DancelightPara,
        frame_support::assert_ok,
        keyring::Sr25519Keyring,
        parity_scale_codec::Encode,
        snowbridge_core::{
            inbound::{Log, Message},
            PRIMARY_GOVERNANCE_CHANNEL,
        },
        snowbridge_router_primitives::inbound::envelope::OutboundMessageAccepted,
        sp_core::H256,
        tp_bridge::{
            symbiotic_message_processor::{
                InboundCommand, Message as SymbioticMessage, Payload, MAGIC_BYTES,
            },
            Command,
        },
        xcm_emulator::{Chain, TestExt},
    };
}
