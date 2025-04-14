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

pub use xcm_emulator::{bx, TestExt};
use {
    container_chain_template_simple_runtime::AccountId,
    dancebox_emulated_chain::Dancebox,
    frame_support::parameter_types,
    frontier_template_emulated_chain::FrontierTemplate,
    simple_template_emulated_chain::SimpleTemplate,
    sp_keyring::Sr25519Keyring,
    tanssi_emulated_integration_tests_common::accounts::{ALICE, BOB, RANDOM},
    westend_emulated_chain::Westend,
    xcm_emulator::{decl_test_networks, Chain},
};

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
}

parameter_types! {
    // Westend
    pub WestendSender: AccountId = Sr25519Keyring::Alice.to_account_id();
    pub WestendReceiver: AccountId = Sr25519Keyring::Bob.to_account_id();
    pub WestendEmptyReceiver: AccountId = WestendRelay::account_id_of(RANDOM);

    // Dancebox
    pub DanceboxSender: AccountId = AccountId::from(ALICE);
    pub DanceboxReceiver: AccountId = AccountId::from(BOB);
    pub DanceboxEmptyReceiver: AccountId = DanceboxPara::account_id_of(RANDOM);

    // SimpleTemplate
    pub SimpleTemplateSender: AccountId = Sr25519Keyring::Alice.to_account_id();
    pub SimpleTemplateReceiver: AccountId = Sr25519Keyring::Bob.to_account_id();
    pub SimpleTemplateEmptyReceiver: AccountId = SimpleTemplatePara::account_id_of(RANDOM);
}
