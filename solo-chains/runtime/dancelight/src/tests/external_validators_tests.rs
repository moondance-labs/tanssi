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

            // session 5 validators: ALICE, BOB
            // session 6 validators: ALICE, BOB, 0x0f
            // session 7 validators: ALICE, BOB, 0x0f
            // session 12 validators: ALICE, BOB, 0x15

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
                println!("session {} validators: {:?}", session, validators);
            }

            todo!();
        });
}
