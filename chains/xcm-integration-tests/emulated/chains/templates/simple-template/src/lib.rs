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
use {
    container_chain_template_simple_runtime::{
        xcm_config, AssetRate, Balances, ForeignAssets, ForeignAssetsCreator, ParachainInfo,
        ParachainSystem, PolkadotXcm, System, XcmpQueue,
    },
    emulated_integration_tests_common::{
        impl_assert_events_helpers_for_parachain, xcm_emulator::decl_test_parachains,
    },
    tanssi_emulated_integration_tests_common::TestDigestProvider,
};

decl_test_parachains! {
    pub struct SimpleTemplate {
        genesis = genesis::genesis(),
        on_init = (),
        runtime = container_chain_template_simple_runtime,
        core = {
            XcmpMessageHandler: XcmpQueue,
            LocationToAccountId: xcm_config::LocationToAccountId,
            ParachainInfo: ParachainInfo,
            MessageOrigin: cumulus_primitives_core::AggregateMessageOrigin,
            DigestProvider: TestDigestProvider<container_chain_template_simple_runtime::Runtime, Self::Network>,
        },
        pallets = {
            System: System,
            Balances: Balances,
            ParachainSystem: ParachainSystem,
            PolkadotXcm: PolkadotXcm,
            ForeignAssets:  ForeignAssets,
            AssetRate:  AssetRate,
            ForeignAssetsCreator: ForeignAssetsCreator,
        }
    }
}

impl_assert_events_helpers_for_parachain!(SimpleTemplate);
