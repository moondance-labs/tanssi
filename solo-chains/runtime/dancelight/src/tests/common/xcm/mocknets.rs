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
    super::constants::{
        accounts::{ALICE, BOB, RANDOM},
        frontier_template, simple_template,
    },
    crate::tests::common::ExtBuilder,
    dancelight_runtime_constants::currency::UNITS as UNIT,
    emulated_integration_tests_common::xcm_emulator::decl_test_parachains,
    frame_support::parameter_types,
    xcm_emulator::{decl_test_networks, decl_test_relay_chains, Chain},
};

decl_test_relay_chains! {
    #[api_version(11)]
    pub struct Dancelight {
        genesis = ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (crate::AccountId::from(crate::tests::common::ALICE), 210_000 * UNIT),
            (crate::AccountId::from(crate::tests::common::BOB), 100_000 * UNIT),
        ])
        .with_relay_config(runtime_parachains::configuration::HostConfiguration {
            max_downward_message_size: 1024 * 1024,
            ..Default::default()
        })
        .with_safe_xcm_version(3)
        .build_storage(),
        on_init = (),
        runtime = crate,
        core = {
            SovereignAccountOf: crate::xcm_config::LocationConverter,
        },
        pallets = {
            System: crate::System,
            Session: crate::Session,
            Configuration: crate::Configuration,
            Balances: crate::Balances,
            Registrar: crate::Registrar,
            ParasSudoWrapper: crate::ParasSudoWrapper,
            OnDemandAssignmentProvider: crate::OnDemandAssignmentProvider,
            XcmPallet: crate::XcmPallet,
            Sudo: crate::Sudo,
        }
    }
}

decl_test_parachains! {
    // Dancelight parachains
    pub struct FrontierTemplateDancelight {
        genesis = frontier_template::genesis(),
        on_init = (),
        runtime = container_chain_template_frontier_runtime,
        core = {
            XcmpMessageHandler: container_chain_template_frontier_runtime::XcmpQueue,
            LocationToAccountId: container_chain_template_frontier_runtime::xcm_config::LocationToAccountId,
            ParachainInfo: container_chain_template_frontier_runtime::ParachainInfo,
            MessageOrigin: cumulus_primitives_core::AggregateMessageOrigin,
        },
        pallets = {
            System: container_chain_template_frontier_runtime::System,
            Balances: container_chain_template_frontier_runtime::Balances,
            ParachainSystem: container_chain_template_frontier_runtime::ParachainSystem,
            PolkadotXcm: container_chain_template_frontier_runtime::PolkadotXcm,
            ForeignAssets:  container_chain_template_frontier_runtime::ForeignAssets,
            AssetRate:  container_chain_template_frontier_runtime::AssetRate,
            ForeignAssetsCreator: container_chain_template_frontier_runtime::ForeignAssetsCreator,
        }
    },
    pub struct SimpleTemplateDancelight {
        genesis = simple_template::genesis(),
        on_init = (),
        runtime = container_chain_template_simple_runtime,
        core = {
            XcmpMessageHandler: container_chain_template_simple_runtime::XcmpQueue,
            LocationToAccountId: container_chain_template_simple_runtime::xcm_config::LocationToAccountId,
            ParachainInfo: container_chain_template_simple_runtime::ParachainInfo,
            MessageOrigin: cumulus_primitives_core::AggregateMessageOrigin,
        },
        pallets = {
            System: container_chain_template_simple_runtime::System,
            Balances: container_chain_template_simple_runtime::Balances,
            ParachainSystem: container_chain_template_simple_runtime::ParachainSystem,
            PolkadotXcm: container_chain_template_simple_runtime::PolkadotXcm,
            ForeignAssets:  container_chain_template_simple_runtime::ForeignAssets,
            AssetRate:  container_chain_template_simple_runtime::AssetRate,
            ForeignAssetsCreator: container_chain_template_simple_runtime::ForeignAssetsCreator,
        }
    }
}

decl_test_networks! {
    pub struct DancelightMockNet {
        relay_chain = Dancelight,
        parachains = vec![
            FrontierTemplateDancelight,
            SimpleTemplateDancelight,
        ],
        bridge = ()
    }
}

parameter_types! {
    // Dancelight
    pub DancelightSender: crate::AccountId = crate::AccountId::from(crate::tests::common::ALICE);
    pub DancelightReceiver: crate::AccountId = crate::AccountId::from(crate::tests::common::BOB);
    pub DancelightEmptyReceiver: crate::AccountId = DancelightRelay::account_id_of(RANDOM);

    // SimpleTemplate
    pub SimpleTemplateDancelightSender: container_chain_template_simple_runtime::AccountId = SimpleTemplateDancelightPara::account_id_of(ALICE);
    pub SimpleTemplateDancelightReceiver: container_chain_template_simple_runtime::AccountId = SimpleTemplateDancelightPara::account_id_of(BOB);
    pub SimpleTemplateDancelightEmptyReceiver: container_chain_template_simple_runtime::AccountId = SimpleTemplateDancelightPara::account_id_of(RANDOM);
}
