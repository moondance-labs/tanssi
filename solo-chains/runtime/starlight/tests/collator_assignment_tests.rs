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

use {
    crate::common::*,
    frame_support::assert_ok,
    sp_std::vec,
    starlight_runtime::{genesis_config_presets::get_aura_id_from_seed, TanssiInvulnerables},
    starlight_runtime_constants::currency::EXISTENTIAL_DEPOSIT,
};

mod common;

const UNIT: Balance = 1_000_000_000_000_000_000;

#[test]
fn test_author_collation_aura_change_of_authorities_on_session() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 2,
            min_orchestrator_collators: 0,
            max_orchestrator_collators: 0,
            collators_per_container: 2,
            ..Default::default()
        })
        .build()
        .execute_with(|| {
            run_to_block(2);
            // We change invulnerables
            // We first need to set the keys
            let alice_keys = get_authority_keys_from_seed(&AccountId::from(ALICE).to_string());
            let bob_keys = get_authority_keys_from_seed(&AccountId::from(BOB).to_string());

            // Set CHARLIE and DAVE keys
            let charlie_keys = get_authority_keys_from_seed(&AccountId::from(CHARLIE).to_string());
            let dave_keys = get_authority_keys_from_seed(&AccountId::from(DAVE).to_string());

            assert_ok!(Session::set_keys(
                origin_of(CHARLIE.into()),
                starlight_runtime::SessionKeys {
                    babe: charlie_keys.2.clone(),
                    grandpa: charlie_keys.3.clone(),
                    para_validator: charlie_keys.4.clone(),
                    para_assignment: charlie_keys.5.clone(),
                    authority_discovery: charlie_keys.6.clone(),
                    beefy: charlie_keys.7.clone(),
                    nimbus: charlie_keys.8.clone(),
                },
                vec![]
            ));

            assert_ok!(Session::set_keys(
                origin_of(DAVE.into()),
                starlight_runtime::SessionKeys {
                    babe: dave_keys.2.clone(),
                    grandpa: dave_keys.3.clone(),
                    para_validator: dave_keys.4.clone(),
                    para_assignment: dave_keys.5.clone(),
                    authority_discovery: dave_keys.6.clone(),
                    beefy: dave_keys.7.clone(),
                    nimbus: dave_keys.8.clone(),
                },
                vec![]
            ));

            // Change invulnerables
            assert_ok!(TanssiInvulnerables::remove_invulnerable(
                root_origin(),
                ALICE.into()
            ));
            assert_ok!(TanssiInvulnerables::remove_invulnerable(
                root_origin(),
                BOB.into()
            ));
            assert_ok!(TanssiInvulnerables::add_invulnerable(
                root_origin(),
                CHARLIE.into()
            ));
            assert_ok!(TanssiInvulnerables::add_invulnerable(
                root_origin(),
                DAVE.into()
            ));

            assert!(
                authorities_for_container(1000u32.into())
                    == Some(vec![alice_keys.8.clone(), bob_keys.8.clone()])
            );

            // SESSION CHANGE. First session. it takes 2 sessions to see the change
            run_to_session(1u32);

            assert!(authorities() == vec![alice_keys.2.clone(), bob_keys.2.clone()]);
            assert!(
                authorities_for_container(1000u32.into())
                    == Some(vec![alice_keys.8.clone(), bob_keys.8.clone()])
            );

            // Invulnerables should have triggered on new session authorities change
            run_to_session(2u32);

            assert!(authorities() == vec![alice_keys.2.clone(), bob_keys.2.clone()]);
            assert!(
                authorities_for_container(1000u32.into())
                    == Some(vec![charlie_keys.8.clone(), dave_keys.8.clone()])
            );
        });
}
