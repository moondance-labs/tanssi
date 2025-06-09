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
use xcm_emulator::decl_test_relay_chains;

decl_test_relay_chains! {
    #[api_version(11)]
    pub struct Starlight {
        genesis = genesis::genesis(),
        on_init = (),
        runtime = starlight_runtime,
        core = {
            SovereignAccountOf: starlight_runtime::xcm_config::LocationConverter,
        },
        pallets = {
            System: starlight_runtime::System,
            Session: starlight_runtime::Session,
            Configuration: starlight_runtime::Configuration,
            Balances: starlight_runtime::Balances,
            Registrar: starlight_runtime::Registrar,
            ParasSudoWrapper: starlight_runtime::ParasSudoWrapper,
            OnDemandAssignmentProvider: starlight_runtime::OnDemandAssignmentProvider,
            XcmPallet: starlight_runtime::XcmPallet,
            Sudo: starlight_runtime::Sudo,
            MessageQueue: starlight_runtime::MessageQueue,
            ExternalValidatorSlashes: starlight_runtime::ExternalValidatorSlashes,
            EthereumOutboundQueue: starlight_runtime::EthereumOutboundQueue,
            EthereumInboundQueue: starlight_runtime::EthereumInboundQueue,
            EthereumSystem: starlight_runtime::EthereumSystem,
            ExternalValidators: starlight_runtime::ExternalValidators,
        }
    }
}
