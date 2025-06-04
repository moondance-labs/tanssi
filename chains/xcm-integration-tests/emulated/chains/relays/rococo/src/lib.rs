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
use cumulus_primitives_core::relay_chain::runtime_api::runtime_decl_for_parachain_host::ParachainHostV13;
pub use xcm_emulator::TestExt;

decl_test_relay_chains! {
    #[api_version(11)]
    pub struct Rococo {
        genesis = genesis::genesis(),
        on_init = (),
        runtime = rococo_runtime,
        core = {
            SovereignAccountOf: rococo_runtime::xcm_config::LocationConverter,
        },
        pallets = {
            System: rococo_runtime::System,
            Session: rococo_runtime::Session,
            Configuration: rococo_runtime::Configuration,
            Balances: rococo_runtime::Balances,
            Registrar: rococo_runtime::Registrar,
            ParasSudoWrapper: rococo_runtime::ParasSudoWrapper,
            OnDemandAssignmentProvider: rococo_runtime::OnDemandAssignmentProvider,
            XcmPallet: rococo_runtime::XcmPallet,
            Sudo: rococo_runtime::Sudo,
        }
    }
}
