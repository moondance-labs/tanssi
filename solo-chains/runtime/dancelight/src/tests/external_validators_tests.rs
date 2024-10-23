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

use crate::ExternalValidators;
use frame_support::traits::fungible::Mutate;
use std::collections::HashMap;
use {crate::tests::common::*, frame_support::assert_ok, sp_std::vec};

#[test]
fn validators_only_change_once_per_era() {
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

            let mut session_validators = HashMap::new();

            for session in 1u32..=13 {
                let mock_validator = AccountId::from([10u8 + session as u8; 32]);
                let mock_keys = get_authority_keys_from_seed(&mock_validator.to_string(), None);

                assert_ok!(Balances::mint_into(&mock_validator, 10_000 * UNIT));
                assert_ok!(Session::set_keys(
                    origin_of(mock_validator.clone()),
                    crate::SessionKeys {
                        babe: mock_keys.babe.clone(),
                        grandpa: mock_keys.grandpa.clone(),
                        para_validator: mock_keys.para_validator.clone(),
                        para_assignment: mock_keys.para_assignment.clone(),
                        authority_discovery: mock_keys.authority_discovery.clone(),
                        beefy: mock_keys.beefy.clone(),
                        nimbus: mock_keys.nimbus.clone(),
                    },
                    vec![]
                ));

                ExternalValidators::set_external_validators(vec![mock_validator]).unwrap();

                run_to_session(session);
                let validators = Session::validators();
                session_validators.insert(session, validators);
            }

            // 1 era = 6 sessions
            // session_range => validators
            // [0, 5] => Alice, Bob
            // [6, 11] => Alice, Bob, 0x0f
            // [12, ..] => Alice, Bob, 0x15
            assert_eq!(
                session_validators[&5],
                vec![AccountId::from(ALICE), AccountId::from(BOB)]
            );
            assert_eq!(
                session_validators[&6],
                vec![
                    AccountId::from(ALICE),
                    AccountId::from(BOB),
                    AccountId::from([0x0f; 32])
                ]
            );
            // TODO: if compiling with fast-runtime, this line will fail because 1 era = 3 sessions, so instead of
            // validator 0x0f you will see 0x12
            assert_eq!(
                session_validators[&11],
                vec![
                    AccountId::from(ALICE),
                    AccountId::from(BOB),
                    AccountId::from([0x0f; 32])
                ]
            );
            assert_eq!(
                session_validators[&12],
                vec![
                    AccountId::from(ALICE),
                    AccountId::from(BOB),
                    AccountId::from([0x15; 32])
                ]
            );
        });
}
