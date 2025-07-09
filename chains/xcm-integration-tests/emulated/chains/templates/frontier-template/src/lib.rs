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
    container_chain_template_frontier_runtime::{
        xcm_config::LocationToAccountId, AccountId, AssetRate, Balances, ForeignAssets,
        ForeignAssetsCreator, ParachainInfo, ParachainSystem, PolkadotXcm, System, XcmpQueue,
    },
    emulated_integration_tests_common::{
        impl_assert_events_helpers_for_parachain, xcm_emulator::decl_test_parachains,
    },
    frame_support::parameter_types,
    frame_support::sp_runtime::DispatchResult,
    tanssi_emulated_integration_tests_common::TestDigestProvider,
    xcm_emulator::AdditionalInherentCode,
    xcm_emulator::OnInitialize,
    xcm_emulator::Parachain,
};

pub struct TemplateAdditionalInherentCode;
impl AdditionalInherentCode for TemplateAdditionalInherentCode {
    fn on_new_block() -> DispatchResult {
        pallet_cc_authorities_noting::DidSetOrchestratorAuthorityData::<
            container_chain_template_frontier_runtime::Runtime,
        >::put(true);
        pallet_author_inherent::InherentIncluded::<
            container_chain_template_frontier_runtime::Runtime,
        >::put(true);
        Ok(())
    }
}

decl_test_parachains! {
    // Dancelight parachains
    pub struct FrontierTemplate {
        genesis = genesis::genesis(),
        on_init = (),
        runtime = container_chain_template_frontier_runtime,
        core = {
            XcmpMessageHandler: XcmpQueue,
            LocationToAccountId: LocationToAccountId,
            ParachainInfo: ParachainInfo,
            MessageOrigin: cumulus_primitives_core::AggregateMessageOrigin,
            DigestProvider: TestDigestProvider<container_chain_template_frontier_runtime::Runtime, Self::Network>,
            AdditionalInherentCode: TemplateAdditionalInherentCode,
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
    },
}

parameter_types! {
    pub EthereumSender: AccountId = genesis::pre_funded_accounts()[0];
    pub EthereumReceiver: AccountId = genesis::pre_funded_accounts()[1];
    pub EthereumEmptyReceiver: AccountId = [1u8; 20].into();
}

impl_assert_events_helpers_for_parachain!(FrontierTemplate);
