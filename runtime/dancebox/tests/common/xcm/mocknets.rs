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
use cumulus_primitives_core::relay_chain::runtime_api::runtime_decl_for_parachain_host::ParachainHostV6;
pub use sp_core::{sr25519, storage::Storage, Get};
use {
    super::constants::{
        accounts::{ALICE, BOB, RANDOM},
        frontier_template, simple_template, westend,
    },
    frame_support::parameter_types,
};
use {
    staging_xcm::prelude::*,
    staging_xcm_builder::{ParentIsPreset, SiblingParachainConvertsVia},
    staging_xcm_executor::traits::ConvertLocation,
    xcm_emulator::{
        decl_test_networks, decl_test_parachains, decl_test_relay_chains, Chain,
        DefaultMessageProcessor,
    },
};

decl_test_relay_chains! {
    #[api_version(5)]
    pub struct Westend {
        genesis = westend::genesis(),
        on_init = (),
        runtime = westend_runtime,
        core = {
            MessageProcessor: DefaultMessageProcessor<Westend>,
            SovereignAccountOf: westend_runtime::xcm_config::LocationConverter, //TODO: rename to SovereignAccountOf,
        },
        pallets = {
            System: westend_runtime::System,
            Balances: westend_runtime::Balances,
            XcmPallet: westend_runtime::XcmPallet,
            Sudo: westend_runtime::Sudo,
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
        on_init = (),
        runtime = dancebox_runtime,
        core = {
            XcmpMessageHandler: dancebox_runtime::XcmpQueue,
            DmpMessageHandler: dancebox_runtime::DmpQueue,
            LocationToAccountId: dancebox_runtime::xcm_config::LocationToAccountId,
            ParachainInfo: dancebox_runtime::ParachainInfo,
        },
        pallets = {
            System: dancebox_runtime::System,
            Balances: dancebox_runtime::Balances,
            ParachainSystem: dancebox_runtime::ParachainSystem,
            PolkadotXcm: dancebox_runtime::PolkadotXcm,
        }
    },
    pub struct FrontierTemplate {
        genesis = frontier_template::genesis(),
        on_init = (),
        runtime = container_chain_template_frontier_runtime,
        core = {
            XcmpMessageHandler: container_chain_template_frontier_runtime::XcmpQueue,
            DmpMessageHandler: container_chain_template_frontier_runtime::DmpQueue,
            LocationToAccountId: container_chain_template_frontier_runtime::xcm_config::LocationToAccountId,
            ParachainInfo: container_chain_template_frontier_runtime::ParachainInfo,
        },
        pallets = {
            System: container_chain_template_frontier_runtime::System,
            Balances: container_chain_template_frontier_runtime::Balances,
            ParachainSystem: container_chain_template_frontier_runtime::ParachainSystem,
            PolkadotXcm: container_chain_template_frontier_runtime::PolkadotXcm,
        }
    },
    pub struct SimpleTemplate {
        genesis = simple_template::genesis(),
        on_init = (),
        runtime = container_chain_template_simple_runtime,
        core = {
            XcmpMessageHandler: container_chain_template_simple_runtime::XcmpQueue,
            DmpMessageHandler: container_chain_template_simple_runtime::DmpQueue,
            LocationToAccountId: container_chain_template_simple_runtime::xcm_config::LocationToAccountId,
            ParachainInfo: container_chain_template_simple_runtime::ParachainInfo,
        },
        pallets = {
            System: container_chain_template_simple_runtime::System,
            Balances: container_chain_template_simple_runtime::Balances,
            ParachainSystem: container_chain_template_simple_runtime::ParachainSystem,
            PolkadotXcm: container_chain_template_simple_runtime::PolkadotXcm,
        }
    }
}

decl_test_networks! {
    pub struct WestendMockNet {
        relay_chain = Westend,
        parachains = vec![
            Dancebox,
            FrontierTemplate,
            SimpleTemplate,
        ],
        bridge = ()
    }
}

parameter_types! {
    // Westend
    pub WestendSender: cumulus_primitives_core::relay_chain::AccountId = Westend::account_id_of(ALICE);
    pub WestendReceiver: cumulus_primitives_core::relay_chain::AccountId = Westend::account_id_of(BOB);
    pub WestendEmptyReceiver: cumulus_primitives_core::relay_chain::AccountId = Westend::account_id_of(RANDOM);
    // Dancebox
    pub DanceboxSender: dancebox_runtime::AccountId = crate::AccountId::from(crate::ALICE);
    pub DanceboxReceiver: dancebox_runtime::AccountId = crate::AccountId::from(crate::BOB);
    pub DanceboxEmptyReceiver: dancebox_runtime::AccountId = Dancebox::account_id_of(RANDOM);

    // SimpleTemplate
    pub SimpleTemplateSender: container_chain_template_simple_runtime::AccountId = SimpleTemplate::account_id_of(ALICE);
    pub SimpleTemplateReceiver: container_chain_template_simple_runtime::AccountId = SimpleTemplate::account_id_of(BOB);

    pub EthereumSender: container_chain_template_frontier_runtime::AccountId = frontier_template::pre_funded_accounts()[0];
    pub EthereumReceiver: container_chain_template_frontier_runtime::AccountId = frontier_template::pre_funded_accounts()[1];
    pub EthereumEmptyReceiver: container_chain_template_frontier_runtime::AccountId = [1u8; 20].into();
}
