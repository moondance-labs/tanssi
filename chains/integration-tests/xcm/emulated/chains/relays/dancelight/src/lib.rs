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

pub mod genesis;
pub use dancelight_runtime;
use xcm_emulator::decl_test_relay_chains;

decl_test_relay_chains! {
    #[api_version(11)]
    pub struct Dancelight {
        genesis = genesis::genesis(),
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
