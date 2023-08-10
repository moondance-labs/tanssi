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
        westend,
    },
    frame_support::{parameter_types, sp_tracing},
};

pub use sp_core::{sr25519, storage::Storage, Get};
use {
    crate::{AccountId, Balance},
    xcm::prelude::*,
    xcm_builder::ParentIsPreset,
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
            (AccountId::from(crate::ALICE), 210_000 * crate::UNIT),
            (AccountId::from(crate::BOB), 100_000 * crate::UNIT),
            // Give some balance to the relay chain account
            (ParentIsPreset::<AccountId>::convert_ref(MultiLocation::parent()).unwrap(), 100_000 * crate::UNIT)
        ])
        .with_safe_xcm_version(3).build_storage(),
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
    }
}

decl_test_networks! {
    pub struct WestendMockNet {
        relay_chain = Westend,
        parachains = vec![
            Dancebox,
        ],
    }
}

parameter_types! {
    // Westend
    pub WestendSender: cumulus_primitives_core::relay_chain::AccountId = Westend::account_id_of(ALICE);
    pub WestendReceiver: cumulus_primitives_core::relay_chain::AccountId = Westend::account_id_of(BOB);
    pub WestendEmptyReceiver: cumulus_primitives_core::relay_chain::AccountId = Westend::account_id_of(RANDOM);
    // Dancebox
    pub DanceboxSender: dancebox_runtime::AccountId = Dancebox::account_id_of(ALICE);
    pub DanceboxReceiver: dancebox_runtime::AccountId = Dancebox::account_id_of(BOB);
}
