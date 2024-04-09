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
    emulated_integration_tests_common::{
        impl_assert_events_helpers_for_parachain, xcm_emulator::decl_test_parachains,
    },
    frame_support::parameter_types,
    parity_scale_codec::Encode,
    sp_consensus_aura::AURA_ENGINE_ID,
    sp_runtime::generic::DigestItem,
    staging_xcm::prelude::*,
    staging_xcm_builder::{ParentIsPreset, SiblingParachainConvertsVia},
    staging_xcm_executor::traits::ConvertLocation,
    xcm_emulator::{decl_test_networks, decl_test_relay_chains, Chain},
};

decl_test_relay_chains! {
    #[api_version(10)]
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
    #[api_version(10)]
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
    }
}

decl_test_parachains! {
    // Parachains
    pub struct Dancebox {
        genesis = crate::ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (crate::AccountId::from(crate::ALICE), 210_000 * crate::UNIT),
            (crate::AccountId::from(crate::BOB), 100_000 * crate::UNIT),
            // Give some balance to the relay chain account
            (ParentIsPreset::<crate::AccountId>::convert_location(&MultiLocation::parent()).unwrap(), 100_000 * crate::UNIT),
            // And to sovereigns
            (
                SiblingParachainConvertsVia::<polkadot_parachain_primitives::primitives::Sibling, crate::AccountId>::convert_location(
                    &MultiLocation{ parents: 1, interior: X1(Parachain(2001u32))}
                ).unwrap(), 100_000 * crate::UNIT
            ),
            (
                SiblingParachainConvertsVia::<polkadot_parachain_primitives::primitives::Sibling, crate::AccountId>::convert_location(
                    &MultiLocation{ parents: 1, interior: X1(Parachain(2002u32))}
                ).unwrap(), 100_000 * crate::UNIT
            ),
        ])
        .with_safe_xcm_version(3)
        .with_own_para_id(2000u32.into())
        .build_storage(),
        on_init = {
            dancebox_runtime::System::deposit_log(DigestItem::PreRuntime(AURA_ENGINE_ID, 0u64.encode()));
        },
        runtime = dancebox_runtime,
        core = {
            XcmpMessageHandler: dancebox_runtime::XcmpQueue,
            LocationToAccountId: dancebox_runtime::xcm_config::LocationToAccountId,
            ParachainInfo: dancebox_runtime::ParachainInfo,
            MessageOrigin: cumulus_primitives_core::AggregateMessageOrigin,
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
    },
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

    // Parachains
    pub struct DanceboxRococo {
        genesis = crate::ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (crate::AccountId::from(crate::ALICE), 210_000 * crate::UNIT),
            (crate::AccountId::from(crate::BOB), 100_000 * crate::UNIT),
            // Give some balance to the relay chain account
            (ParentIsPreset::<crate::AccountId>::convert_location(&MultiLocation::parent()).unwrap(), 100_000 * crate::UNIT),
            // And to sovereigns
            (
                SiblingParachainConvertsVia::<polkadot_parachain_primitives::primitives::Sibling, crate::AccountId>::convert_location(
                    &MultiLocation{ parents: 1, interior: X1(Parachain(2001u32))}
                ).unwrap(), 100_000 * crate::UNIT
            ),
            (
                SiblingParachainConvertsVia::<polkadot_parachain_primitives::primitives::Sibling, crate::AccountId>::convert_location(
                    &MultiLocation{ parents: 1, interior: X1(Parachain(2002u32))}
                ).unwrap(), 100_000 * crate::UNIT
            ),


        ])
        .with_collators(vec![
            (crate::AccountId::from(crate::ALICE), 210 * crate::UNIT),
            (crate::AccountId::from(crate::BOB), 100 * crate::UNIT),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            min_orchestrator_collators: 1,
            max_orchestrator_collators: 1,
            collators_per_container: 1,
            collators_per_parathread: 1,
            full_rotation_period: 0,
            ..Default::default()
        })
        .with_safe_xcm_version(3)
        .with_own_para_id(2000u32.into())
        .build_storage(),
        on_init = {
            dancebox_runtime::System::deposit_log(DigestItem::PreRuntime(AURA_ENGINE_ID, 0u64.encode()));
        },
        runtime = dancebox_runtime,
        core = {
            XcmpMessageHandler: dancebox_runtime::XcmpQueue,
            LocationToAccountId: dancebox_runtime::xcm_config::LocationToAccountId,
            ParachainInfo: dancebox_runtime::ParachainInfo,
            MessageOrigin: cumulus_primitives_core::AggregateMessageOrigin,
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

impl_assert_events_helpers_for_parachain!(Dancebox);
impl_assert_events_helpers_for_parachain!(FrontierTemplate);
impl_assert_events_helpers_for_parachain!(SimpleTemplate);

decl_test_networks! {
    pub struct WestendMockNet {
        relay_chain = Westend,
        parachains = vec![
            Dancebox,
            FrontierTemplate,
            SimpleTemplate,
        ],
        bridge = ()
    },
    pub struct RococoMockNet {
        relay_chain = Rococo,
        parachains = vec![
            DanceboxRococo,
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

    // Dancebox
    pub DanceboxSender: dancebox_runtime::AccountId = crate::AccountId::from(crate::ALICE);
    pub DanceboxReceiver: dancebox_runtime::AccountId = crate::AccountId::from(crate::BOB);
    pub DanceboxEmptyReceiver: dancebox_runtime::AccountId = DanceboxPara::account_id_of(RANDOM);

    // SimpleTemplate
    pub SimpleTemplateSender: container_chain_template_simple_runtime::AccountId = SimpleTemplatePara::account_id_of(ALICE);
    pub SimpleTemplateReceiver: container_chain_template_simple_runtime::AccountId = SimpleTemplatePara::account_id_of(BOB);
    pub SimpleTemplateEmptyReceiver: container_chain_template_simple_runtime::AccountId = SimpleTemplatePara::account_id_of(RANDOM);

    pub EthereumSender: container_chain_template_frontier_runtime::AccountId = frontier_template::pre_funded_accounts()[0];
    pub EthereumReceiver: container_chain_template_frontier_runtime::AccountId = frontier_template::pre_funded_accounts()[1];
    pub EthereumEmptyReceiver: container_chain_template_frontier_runtime::AccountId = [1u8; 20].into();
}
