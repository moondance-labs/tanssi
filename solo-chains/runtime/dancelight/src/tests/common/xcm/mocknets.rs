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
        frontier_template, rococo, simple_template, westend,
    },
    crate::tests::common::ExtBuilder,
    emulated_integration_tests_common::{
        impl_assert_events_helpers_for_parachain, xcm_emulator::decl_test_parachains,
    },
    frame_support::parameter_types,
    xcm_emulator::{decl_test_networks, decl_test_relay_chains, Chain},
};

decl_test_relay_chains! {
    #[api_version(11)]
    pub struct Westend {
        genesis = westend::genesis(),
        on_init = (),
        runtime = westend_runtime,
        core = {
            SovereignAccountOf: westend_runtime::xcm_config::LocationConverter,
        },
        pallets = {
            System: westend_runtime::System,
            Balances: westend_runtime::Balances,
            XcmPallet: westend_runtime::XcmPallet,
            Sudo: westend_runtime::Sudo,
        }
    },
    #[api_version(11)]
    pub struct Rococo {
        genesis = rococo::genesis(),
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
    },
    #[api_version(11)]
    pub struct Dancelight {
        genesis = ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (crate::AccountId::from(crate::tests::common::ALICE), 210_000 * dancelight_runtime_constants::currency::UNITS),
            (crate::AccountId::from(crate::tests::common::BOB), 100_000 * dancelight_runtime_constants::currency::UNITS),
        ])
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
    pub struct FrontierTemplate {
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
    pub struct SimpleTemplate {
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
    },
    pub struct FrontierTemplateRococo {
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
    pub struct SimpleTemplateRococo {
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

impl_assert_events_helpers_for_parachain!(FrontierTemplate);
impl_assert_events_helpers_for_parachain!(SimpleTemplate);

decl_test_networks! {
    pub struct WestendMockNet {
        relay_chain = Westend,
        parachains = vec![
            FrontierTemplate,
            SimpleTemplate,
        ],
        bridge = ()
    },
    pub struct RococoMockNet {
        relay_chain = Rococo,
        parachains = vec![
            FrontierTemplateRococo,
            SimpleTemplateRococo,
        ],
        bridge = ()
    }
}

parameter_types! {
    // Westend
    pub WestendSender: cumulus_primitives_core::relay_chain::AccountId = WestendRelay::account_id_of(ALICE);
    pub WestendReceiver: cumulus_primitives_core::relay_chain::AccountId = WestendRelay::account_id_of(BOB);
    pub WestendEmptyReceiver: cumulus_primitives_core::relay_chain::AccountId = WestendRelay::account_id_of(RANDOM);

    // Rococo
    pub RococoSender: cumulus_primitives_core::relay_chain::AccountId = RococoRelay::account_id_of(ALICE);
    pub RococoReceiver: cumulus_primitives_core::relay_chain::AccountId = RococoRelay::account_id_of(BOB);
    pub RococoEmptyReceiver: cumulus_primitives_core::relay_chain::AccountId = RococoRelay::account_id_of(RANDOM);

    // Dancelight
    pub DancelightSender: crate::AccountId = crate::AccountId::from(crate::tests::common::ALICE);
    pub DancelightReceiver: crate::AccountId = crate::AccountId::from(crate::tests::common::BOB);
    pub DancelightEmptyReceiver: crate::AccountId = crate::AccountId::from(crate::tests::common::RANDOM);

    // SimpleTemplate
    pub SimpleTemplateSender: container_chain_template_simple_runtime::AccountId = SimpleTemplatePara::account_id_of(ALICE);
    pub SimpleTemplateReceiver: container_chain_template_simple_runtime::AccountId = SimpleTemplatePara::account_id_of(BOB);
    pub SimpleTemplateEmptyReceiver: container_chain_template_simple_runtime::AccountId = SimpleTemplatePara::account_id_of(RANDOM);

    pub EthereumSender: container_chain_template_frontier_runtime::AccountId = frontier_template::pre_funded_accounts()[0];
    pub EthereumReceiver: container_chain_template_frontier_runtime::AccountId = frontier_template::pre_funded_accounts()[1];
    pub EthereumEmptyReceiver: container_chain_template_frontier_runtime::AccountId = [1u8; 20].into();
}
