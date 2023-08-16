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
use {
    super::constants::{
        accounts::{ALICE, BOB, RANDOM},
        frontier_template, simple_template, westend,
    },
    frame_support::{parameter_types, sp_tracing},
};

pub use sp_core::{sr25519, storage::Storage, Get};
use {
    xcm::prelude::*,
    xcm_builder::{ParentIsPreset, SiblingParachainConvertsVia},
    xcm_emulator::{
        decl_test_networks, decl_test_parachains, decl_test_relay_chains, Parachain, RelayChain,
        TestExt,
    },
    xcm_executor::traits::Convert,
};

decl_test_relay_chains! {
    pub struct Westend {
        genesis = westend::genesis(),
        on_init = (),
        runtime = {
            Runtime: westend_runtime::Runtime,
            RuntimeOrigin: westend_runtime::RuntimeOrigin,
            RuntimeCall: westend_runtime::RuntimeCall,
            RuntimeEvent: westend_runtime::RuntimeEvent,
            MessageQueue: westend_runtime::MessageQueue,
            XcmConfig: westend_runtime::xcm_config::XcmConfig,
            SovereignAccountOf: westend_runtime::xcm_config::LocationConverter, //TODO: rename to SovereignAccountOf,
            System: westend_runtime::System,
            Balances: westend_runtime::Balances,
        },
        pallets_extra = {
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
            (ParentIsPreset::<crate::AccountId>::convert_ref(MultiLocation::parent()).unwrap(), 100_000 * crate::UNIT),
            // And to sovereigns
            (
                SiblingParachainConvertsVia::<polkadot_parachain::primitives::Sibling, crate::AccountId>::convert_ref(
                    MultiLocation{ parents: 1, interior: X1(Parachain(2001u32))}
                ).unwrap(), 100_000 * crate::UNIT
            ),
            (
                SiblingParachainConvertsVia::<polkadot_parachain::primitives::Sibling, crate::AccountId>::convert_ref(
                    MultiLocation{ parents: 1, interior: X1(Parachain(2002u32))}
                ).unwrap(), 100_000 * crate::UNIT
            ),


        ])
        .with_safe_xcm_version(3)
        .with_own_para_id(2000u32.into())
        .build_storage(),
        on_init = (),
        runtime = {
            Runtime: dancebox_runtime::Runtime,
            RuntimeOrigin: dancebox_runtime::RuntimeOrigin,
            RuntimeCall: dancebox_runtime::RuntimeCall,
            RuntimeEvent: dancebox_runtime::RuntimeEvent,
            XcmpMessageHandler: dancebox_runtime::XcmpQueue,
            DmpMessageHandler: dancebox_runtime::DmpQueue,
            LocationToAccountId: dancebox_runtime::xcm_config::LocationToAccountId,
            System: dancebox_runtime::System,
            Balances: dancebox_runtime::Balances,
            ParachainSystem: dancebox_runtime::ParachainSystem,
            ParachainInfo: dancebox_runtime::ParachainInfo,
        },
        pallets_extra = {
            PolkadotXcm: dancebox_runtime::PolkadotXcm,
        }
    },
    pub struct FrontierTemplate {
        genesis = frontier_template::genesis(),
        on_init = (),
        runtime = {
            Runtime: container_chain_template_frontier_runtime::Runtime,
            RuntimeOrigin: container_chain_template_frontier_runtime::RuntimeOrigin,
            RuntimeCall: container_chain_template_frontier_runtime::RuntimeCall,
            RuntimeEvent: container_chain_template_frontier_runtime::RuntimeEvent,
            XcmpMessageHandler: container_chain_template_frontier_runtime::XcmpQueue,
            DmpMessageHandler: container_chain_template_frontier_runtime::DmpQueue,
            LocationToAccountId: container_chain_template_frontier_runtime::xcm_config::LocationToAccountId,
            System: container_chain_template_frontier_runtime::System,
            Balances: container_chain_template_frontier_runtime::Balances,
            ParachainSystem: container_chain_template_frontier_runtime::ParachainSystem,
            ParachainInfo: container_chain_template_frontier_runtime::ParachainInfo,
        },
        pallets_extra = {
            PolkadotXcm: container_chain_template_frontier_runtime::PolkadotXcm,
        }
    },
    pub struct SimpleTemplate {
        genesis = simple_template::genesis(),
        on_init = (),
        runtime = {
            Runtime: container_chain_template_simple_runtime::Runtime,
            RuntimeOrigin: container_chain_template_simple_runtime::RuntimeOrigin,
            RuntimeCall: container_chain_template_simple_runtime::RuntimeCall,
            RuntimeEvent: container_chain_template_simple_runtime::RuntimeEvent,
            XcmpMessageHandler: container_chain_template_simple_runtime::XcmpQueue,
            DmpMessageHandler: container_chain_template_simple_runtime::DmpQueue,
            LocationToAccountId: container_chain_template_simple_runtime::xcm_config::LocationToAccountId,
            System: container_chain_template_simple_runtime::System,
            Balances: container_chain_template_simple_runtime::Balances,
            ParachainSystem: container_chain_template_simple_runtime::ParachainSystem,
            ParachainInfo: container_chain_template_simple_runtime::ParachainInfo,
        },
        pallets_extra = {
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
