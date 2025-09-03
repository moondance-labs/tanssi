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
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>

pub use sp_core::Get;
use {
    emulated_integration_tests_common::{
        impl_assert_events_helpers_for_parachain, xcm_emulator::decl_test_parachains,
    },
    frame_support::sp_runtime::DispatchResult,
    parity_scale_codec::Encode,
    sp_consensus_aura::AURA_ENGINE_ID,
    sp_runtime::generic::DigestItem,
    tanssi_emulated_integration_tests_common::TestDigestProvider,
    xcm_emulator::AdditionalInherentCode,
    xcm_emulator::OnInitialize,
    xcm_emulator::Parachain,
};

mod genesis;

pub struct OrchestratorAdditionalInherentCode;
impl AdditionalInherentCode for OrchestratorAdditionalInherentCode {
    fn on_new_block() -> DispatchResult {
        pallet_author_noting::DidSetContainerAuthorData::<dancebox_runtime::Runtime>::put(true);
        pallet_author_inherent::InherentIncluded::<dancebox_runtime::Runtime>::put(true);
        Ok(())
    }
}

decl_test_parachains! {
    // Parachains
    pub struct Dancebox {
        genesis = genesis::genesis(),
        on_init = {
            dancebox_runtime::System::deposit_log(DigestItem::PreRuntime(AURA_ENGINE_ID, 0u64.encode()));
        },
        runtime = dancebox_runtime,
        core = {
            XcmpMessageHandler: dancebox_runtime::XcmpQueue,
            LocationToAccountId: dancebox_runtime::xcm_config::LocationToAccountId,
            ParachainInfo: dancebox_runtime::ParachainInfo,
            MessageOrigin: cumulus_primitives_core::AggregateMessageOrigin,
            DigestProvider: TestDigestProvider<dancebox_runtime::Runtime, Self::Network>,
            AdditionalInherentCode: OrchestratorAdditionalInherentCode,
        },
        pallets = {
            System: dancebox_runtime::System,
            Balances: dancebox_runtime::Balances,
            ParachainSystem: dancebox_runtime::ParachainSystem,
            PolkadotXcm: dancebox_runtime::PolkadotXcm,
            ForeignAssets:  dancebox_runtime::ForeignAssets,
            AssetRate:  dancebox_runtime::AssetRate,
            ForeignAssetsCreator: dancebox_runtime::ForeignAssetsCreator,
        }
    }
}

impl_assert_events_helpers_for_parachain!(Dancebox);
