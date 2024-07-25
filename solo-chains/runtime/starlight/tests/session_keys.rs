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

#![cfg(test)]

use {crate::common::*, frame_support::assert_ok, sp_std::vec};

mod common;
const UNIT: Balance = 1_000_000_000_000_000_000;

#[test]
fn session_key_changes_are_reflected_after_two_sessions() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);
            let alice_keys =
                get_authority_keys_from_seed(&AccountId::from(ALICE).to_string(), None);
            let dave_keys = get_authority_keys_from_seed(&AccountId::from(DAVE).to_string(), None);

            // let's assert that session keys in all pallets are this
            // Babe
            assert!(babe_authorities().contains(&alice_keys.babe.clone()));
            assert!(grandpa_authorities().contains(&alice_keys.grandpa.clone()));
            assert!(!babe_authorities().contains(&dave_keys.babe.clone()));
            assert!(!grandpa_authorities().contains(&dave_keys.grandpa.clone()));

            assert_ok!(Session::set_keys(
                origin_of(ALICE.into()),
                starlight_runtime::SessionKeys {
                    babe: dave_keys.babe.clone(),
                    grandpa: dave_keys.grandpa.clone(),
                    para_validator: dave_keys.para_validator.clone(),
                    para_assignment: dave_keys.para_assignment.clone(),
                    authority_discovery: dave_keys.authority_discovery.clone(),
                    beefy: dave_keys.beefy.clone(),
                    nimbus: dave_keys.nimbus.clone(),
                },
                vec![]
            ));

            // In session one keys are not yet set
            run_to_session(1u32);
            assert!(babe_authorities().contains(&alice_keys.babe.clone()));
            assert!(grandpa_authorities().contains(&alice_keys.grandpa.clone()));
            assert!(!babe_authorities().contains(&dave_keys.babe.clone()));
            assert!(!grandpa_authorities().contains(&dave_keys.grandpa.clone()));

            // In session  2 they should be set
            run_to_session(2u32);

            // While Babe changes are applied immediately (on_initialize)
            // Grandpa changes are applied on-finalize
            // Our tests only stop at on_initialize of the target block,
            // thus we need to create one more block
            assert!(babe_authorities().contains(&dave_keys.babe.clone()));
            assert!(!babe_authorities().contains(&alice_keys.babe.clone()));
            assert!(grandpa_authorities().contains(&alice_keys.grandpa.clone()));
            assert!(!grandpa_authorities().contains(&dave_keys.grandpa.clone()));

            let block_number = System::block_number();
            run_to_block(block_number + 1);
            assert!(babe_authorities().contains(&dave_keys.babe.clone()));
            assert!(!babe_authorities().contains(&alice_keys.babe.clone()));
            assert!(grandpa_authorities().contains(&dave_keys.grandpa.clone()));
            assert!(!grandpa_authorities().contains(&alice_keys.grandpa.clone()));
        });
}
