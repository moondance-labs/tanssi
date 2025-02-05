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

use sp_core::H256;
use {
    crate::{
        tests::common::*, EthereumSystem, ExternalValidators, ExternalValidatorsRewards,
        MaxExternalValidators, RewardTokenLocation, RuntimeEvent, SessionKeys, SessionsPerEra,
        System,
    },
    frame_support::{assert_ok, traits::fungible::Mutate},
    pallet_external_validators::Forcing,
    sp_runtime::traits::MaybeEquivalence,
    std::{collections::HashMap, ops::RangeInclusive},
    tp_bridge::Command,
    xcm::latest::prelude::*,
    xcm::VersionedLocation,
};

fn assert_validators_do_not_change(
    validators: &HashMap<u32, Vec<AccountId>>,
    session_range: RangeInclusive<u32>,
) {
    let first_validators = &validators[session_range.start()];
    for session in session_range {
        let current_validators = &validators[&session];
        assert_eq!(
            current_validators, first_validators,
            "Validators have changed in session {}",
            session
        );
    }
}

fn active_era_session_start() -> u32 {
    let active_era = pallet_external_validators::ActiveEra::<Runtime>::get()
        .map(|x| x.index)
        .unwrap_or(0);

    pallet_external_validators::ErasStartSessionIndex::<Runtime>::get(active_era).unwrap()
}

fn active_era_index() -> u32 {
    ExternalValidators::active_era().map(|x| x.index).unwrap()
}

#[test]
fn whitelisted_validators_priority() {
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

            // SessionsPerEra depends on fast-runtime feature, this test should pass regardless
            let sessions_per_era = SessionsPerEra::get();

            let mut external_validators = vec![];
            let max_validators = MaxExternalValidators::get();
            // Try to insert 105 mock validators (max is 100, so last 5 will not be in the pallet)
            for i in 0..(max_validators + 5) {
                let mock_validator = AccountId::from([0x10 + i as u8; 32]);
                let mock_keys = get_authority_keys_from_seed(&mock_validator.to_string(), None);

                assert_ok!(Balances::mint_into(&mock_validator, 10_000 * UNIT));
                assert_ok!(Session::set_keys(
                    origin_of(mock_validator.clone()),
                    SessionKeys {
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
                external_validators.push(mock_validator);
            }

            ExternalValidators::set_external_validators_inner(external_validators, 1).unwrap();

            run_to_session(sessions_per_era);
            let validators = Session::validators();

            // 2 whitelisted validators (Alice, Bob), and 100 external validators
            assert_eq!(validators.len(), 2 + max_validators as usize);
            assert_eq!(
                &validators[..2],
                &[AccountId::from(ALICE), AccountId::from(BOB)]
            );
        });
}

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

            // SessionsPerEra depends on fast-runtime feature, this test should pass regardless
            let sessions_per_era = SessionsPerEra::get();
            let mut session_validators = HashMap::new();

            for session in 1u32..=(sessions_per_era * 2 + 1) {
                // For every session:
                // * Create a new validator account, MockValidatorN, mint balance, and insert keys
                // * Set this validator as the only "external validator"
                // * Run to session start
                // * Pallet session wants to know validators for the next session (session + 1), so:
                // * If the next session is in the same era as the previous session: validators do not change
                // * If the next session is in a new era: new validators will be set to [Alice, Bob, MockValidatorN]
                // and stored as QueuedKeys in pallet session.
                // So validators for session N will be [Alice, Bob, MockValidator(N-1)]
                let mock_validator = AccountId::from([0x10 + session as u8; 32]);
                let mock_keys = get_authority_keys_from_seed(&mock_validator.to_string(), None);

                assert_ok!(Balances::mint_into(&mock_validator, 10_000 * UNIT));
                assert_ok!(Session::set_keys(
                    origin_of(mock_validator.clone()),
                    SessionKeys {
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

                ExternalValidators::set_external_validators_inner(vec![mock_validator], 1).unwrap();

                run_to_session(session);
                let validators = Session::validators();
                session_validators.insert(session, validators);
            }

            // Example with 1 era = 6 sessions
            // session_range => validators
            // [0, 5] => Alice, Bob
            // [6, 11] => Alice, Bob, 0x15
            // [12, ..] => Alice, Bob, 0x1b
            assert_eq!(
                session_validators[&1],
                vec![AccountId::from(ALICE), AccountId::from(BOB)]
            );
            assert_eq!(
                session_validators[&sessions_per_era],
                vec![
                    AccountId::from(ALICE),
                    AccountId::from(BOB),
                    AccountId::from([0x10 + sessions_per_era as u8 - 1; 32])
                ]
            );
            assert_eq!(
                session_validators[&(sessions_per_era * 2)],
                vec![
                    AccountId::from(ALICE),
                    AccountId::from(BOB),
                    AccountId::from([0x10 + sessions_per_era as u8 * 2 - 1; 32])
                ]
            );
            // Also, validators cannot change inside an era
            assert_validators_do_not_change(&session_validators, 1..=(sessions_per_era - 1));
            assert_validators_do_not_change(
                &session_validators,
                sessions_per_era..=(sessions_per_era * 2 - 1),
            );
            assert_validators_do_not_change(
                &session_validators,
                (sessions_per_era * 2)..=(sessions_per_era * 2 + 1),
            );
        });
}

#[test]
fn external_validators_can_be_disabled() {
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

            // SessionsPerEra depends on fast-runtime feature, this test should pass regardless
            let sessions_per_era = SessionsPerEra::get();

            let mock_validator = AccountId::from([0x10; 32]);
            let mock_keys = get_authority_keys_from_seed(&mock_validator.to_string(), None);

            assert_ok!(Balances::mint_into(&mock_validator, 10_000 * UNIT));
            assert_ok!(Session::set_keys(
                origin_of(mock_validator.clone()),
                SessionKeys {
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

            ExternalValidators::set_external_validators_inner(vec![mock_validator.clone()], 1)
                .unwrap();
            assert_ok!(ExternalValidators::skip_external_validators(
                root_origin(),
                true
            ));

            run_to_session(sessions_per_era);
            let validators = Session::validators();

            // Only whitelisted validators get selected
            assert_eq!(
                validators,
                vec![AccountId::from(ALICE), AccountId::from(BOB)]
            );

            // Enable external validators for next session
            assert_ok!(ExternalValidators::skip_external_validators(
                root_origin(),
                false
            ));

            run_to_session(2 * sessions_per_era);
            let validators = Session::validators();
            assert_eq!(
                validators,
                vec![AccountId::from(ALICE), AccountId::from(BOB), mock_validator]
            );
        });
}

#[test]
fn no_duplicate_validators() {
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

            // SessionsPerEra depends on fast-runtime feature, this test should pass regardless
            let sessions_per_era = SessionsPerEra::get();

            // Alice is both a whitelisted validator and an external validator
            ExternalValidators::set_external_validators_inner(vec![AccountId::from(ALICE)], 1)
                .unwrap();

            run_to_session(sessions_per_era);
            let validators = Session::validators();

            // 2 whitelisted validators (Alice, Bob), Alice does not appear twice
            assert_eq!(
                validators,
                vec![AccountId::from(ALICE), AccountId::from(BOB)]
            );
        });
}

#[test]
fn default_era_changes() {
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

            // SessionsPerEra depends on fast-runtime feature, this test should pass regardless
            let sessions_per_era = SessionsPerEra::get();

            if sessions_per_era != 6 {
                log::error!("Ignoring test default_era_changes because it doesn't work with sessions_per_era={}. Compile without fast-runtime feature and try again.", sessions_per_era);
                return;
            }

            let mut data = vec![];
            let mut prev_validators = Session::validators();

            for session in 1u32..=(sessions_per_era * 2 + 1) {
                // For every session:
                // * Create a new validator account, MockValidatorN, mint balance, and insert keys
                // * Set this validator as the only "external validator"
                // * Run to session start
                // * Pallet session wants to know validators for the next session (session + 1), so:
                // * If the next session is in the same era as the previous session: validators do not change
                // * If the next session is in a new era: new validators will be set to [Alice, Bob, MockValidatorN]
                // and stored as QueuedKeys in pallet session.
                // So validators for session N will be [Alice, Bob, MockValidator(N-1)]
                let mock_validator = AccountId::from([0x10 + session as u8; 32]);
                let mock_keys = get_authority_keys_from_seed(&mock_validator.to_string(), None);

                assert_ok!(Balances::mint_into(&mock_validator, 10_000 * UNIT));
                assert_ok!(Session::set_keys(
                        origin_of(mock_validator.clone()),
                        SessionKeys {
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

                ExternalValidators::set_external_validators_inner(vec![mock_validator], 1).unwrap();

                run_to_session(session);
                let validators = Session::validators();
                let validators_changed = validators != prev_validators;
                prev_validators = validators;
                data.push((
                    session,
                    ExternalValidators::current_era().unwrap(),
                    active_era_index(),
                    active_era_session_start(),
                    validators_changed,
                ));
            }

            // (session, current_era, active_era, active_era_session_start, new_validators)
            let expected = vec![
                (1, 0, 0, 0, false),
                (2, 0, 0, 0, false),
                (3, 0, 0, 0, false),
                (4, 0, 0, 0, false),
                (5, 1, 0, 0, false),
                (6, 1, 1, 6, true),
                (7, 1, 1, 6, false),
                (8, 1, 1, 6, false),
                (9, 1, 1, 6, false),
                (10, 1, 1, 6, false),
                (11, 2, 1, 6, false),
                (12, 2, 2, 12, true),
                (13, 2, 2, 12, false),
            ];

            assert_eq!(data, expected);
        });
}

#[test]
fn babe_session_works() {
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
            run_to_session(2);

            let session = Session::current_index();

            // If pallet_external_validators returns empty validators, pallet_session will skip some
            // sessions and the reported session will be 7 instead of 2
            assert_eq!(session, 2);
        });
}

mod force_eras {
    use super::*;

    #[test]
    fn force_new_era() {
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

                // SessionsPerEra depends on fast-runtime feature, this test should pass regardless
                let sessions_per_era = SessionsPerEra::get();

                let mock_validator = AccountId::from([0x10; 32]);
                let mock_keys = get_authority_keys_from_seed(&mock_validator.to_string(), None);

                assert_ok!(Balances::mint_into(&mock_validator, 10_000 * UNIT));
                assert_ok!(Session::set_keys(
                    origin_of(mock_validator.clone()),
                    SessionKeys {
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

                ExternalValidators::set_external_validators_inner(vec![mock_validator.clone()], 1)
                    .unwrap();
                assert_eq!(ExternalValidators::current_era(), Some(0));
                assert_ok!(ExternalValidators::force_era(
                    root_origin(),
                    Forcing::ForceNew
                ));
                // Still era 1, until next session
                assert_eq!(ExternalValidators::current_era(), Some(0));
                assert_eq!(Session::current_index(), 0);

                run_to_session(1);
                assert_eq!(Session::current_index(), 1);
                // Era changes in session 1, but validators will change in session 2
                assert_eq!(ExternalValidators::current_era(), Some(1));
                let validators = Session::validators();
                assert_eq!(
                    validators,
                    vec![AccountId::from(ALICE), AccountId::from(BOB),]
                );

                run_to_session(2);
                // Validators change now
                assert_eq!(ExternalValidators::current_era(), Some(1));
                let validators = Session::validators();
                assert_eq!(
                    validators,
                    vec![
                        AccountId::from(ALICE),
                        AccountId::from(BOB),
                        mock_validator.clone()
                    ]
                );

                // Change external validators again
                ExternalValidators::set_external_validators_inner(vec![], 1).unwrap();
                run_to_session(1 + sessions_per_era - 1);
                // Validators will not change until `sessions_per_era` sessions later
                // With sessions_per_era=6, era will change in session 7, validators will change in
                // session 8, this is session 6
                assert_eq!(ExternalValidators::current_era(), Some(1));
                let validators = Session::validators();
                assert_eq!(
                    validators,
                    vec![
                        AccountId::from(ALICE),
                        AccountId::from(BOB),
                        mock_validator.clone()
                    ]
                );

                run_to_session(1 + sessions_per_era);
                // This is session 7, new era but not new validators
                assert_eq!(ExternalValidators::current_era(), Some(2));
                assert_eq!(active_era_index(), 1);
                let validators = Session::validators();
                assert_eq!(
                    validators,
                    vec![AccountId::from(ALICE), AccountId::from(BOB), mock_validator]
                );

                run_to_session(1 + sessions_per_era + 1);
                // This is session 8, validators will change now
                assert_eq!(ExternalValidators::current_era(), Some(2));
                assert_eq!(active_era_index(), 2);
                let validators = Session::validators();
                assert_eq!(
                    validators,
                    vec![AccountId::from(ALICE), AccountId::from(BOB)]
                );
            });
    }

    #[test]
    fn force_no_eras() {
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

                // SessionsPerEra depends on fast-runtime feature, this test should pass regardless
                let sessions_per_era = SessionsPerEra::get();

                let mock_validator = AccountId::from([0x10; 32]);
                let mock_keys = get_authority_keys_from_seed(&mock_validator.to_string(), None);

                assert_ok!(Balances::mint_into(&mock_validator, 10_000 * UNIT));
                assert_ok!(Session::set_keys(
                    origin_of(mock_validator.clone()),
                    SessionKeys {
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

                ExternalValidators::set_external_validators_inner(vec![mock_validator.clone()], 1)
                    .unwrap();
                // Validators will never change
                assert_eq!(ExternalValidators::current_era(), Some(0));
                assert_ok!(ExternalValidators::force_era(
                    root_origin(),
                    Forcing::ForceNone
                ));

                run_to_session(sessions_per_era);
                assert_eq!(ExternalValidators::current_era(), Some(0));
                let validators = Session::validators();
                assert_eq!(
                    validators,
                    vec![AccountId::from(ALICE), AccountId::from(BOB)]
                );
            });
    }

    #[test]
    fn force_new_era_always() {
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

                let mock_validator = AccountId::from([0x10; 32]);
                let mock_keys = get_authority_keys_from_seed(&mock_validator.to_string(), None);

                assert_ok!(Balances::mint_into(&mock_validator, 10_000 * UNIT));
                assert_ok!(Session::set_keys(
                    origin_of(mock_validator.clone()),
                    SessionKeys {
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

                ExternalValidators::set_external_validators_inner(vec![mock_validator.clone()], 1)
                    .unwrap();
                // Validators will change on every session
                assert_eq!(ExternalValidators::current_era(), Some(0));
                assert_ok!(ExternalValidators::force_era(
                    root_origin(),
                    Forcing::ForceAlways
                ));

                run_to_session(2);
                assert_eq!(ExternalValidators::current_era(), Some(2));
                let validators = Session::validators();
                assert_eq!(
                    validators,
                    vec![
                        AccountId::from(ALICE),
                        AccountId::from(BOB),
                        mock_validator.clone()
                    ]
                );

                ExternalValidators::set_external_validators_inner(vec![], 1).unwrap();
                run_to_session(4);
                assert_eq!(ExternalValidators::current_era(), Some(4));
                let validators = Session::validators();
                assert_eq!(
                    validators,
                    vec![AccountId::from(ALICE), AccountId::from(BOB)]
                );
            });
    }
}

#[test]
fn external_validators_manual_reward_points() {
    use {crate::ValidatorIndex, runtime_parachains::inclusion::RewardValidators};

    ExtBuilder::default()
        .with_balances(vec![
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            // SessionsPerEra depends on fast-runtime feature, this test should pass regardless
            let sessions_per_era = SessionsPerEra::get();

            let mock_validator = AccountId::from([0x10; 32]);
            let mock_keys = get_authority_keys_from_seed(&mock_validator.to_string(), None);

            assert_ok!(Balances::mint_into(&mock_validator, 10_000 * UNIT));
            assert_ok!(Session::set_keys(
                origin_of(mock_validator.clone()),
                SessionKeys {
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

            ExternalValidators::set_external_validators_inner(vec![mock_validator.clone()], 1)
                .unwrap();
            assert_ok!(ExternalValidators::skip_external_validators(
                root_origin(),
                true
            ));

            run_to_session(sessions_per_era);
            let validators = Session::validators();

            // Only whitelisted validators get selected
            assert_eq!(
                validators,
                vec![AccountId::from(ALICE), AccountId::from(BOB)]
            );

            assert!(
                pallet_external_validators_rewards::RewardPointsForEra::<Runtime>::iter().count()
                    == 0
            );

            crate::RewardValidators::reward_backing(vec![ValidatorIndex(0)]);

            assert!(
                pallet_external_validators_rewards::RewardPointsForEra::<Runtime>::iter().count()
                    == 1
            );
        });
}

#[test]
fn external_validators_rewards_sends_message_on_era_end() {
    ExtBuilder::default()
        .with_balances(vec![
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .with_validators(vec![])
        .with_external_validators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .build()
        .execute_with(|| {
            let token_location: VersionedLocation = Location::here().into();

            assert_ok!(EthereumSystem::register_token(
                root_origin(),
                Box::new(token_location),
                snowbridge_core::AssetMetadata {
                    name: "dance".as_bytes().to_vec().try_into().unwrap(),
                    symbol: "dance".as_bytes().to_vec().try_into().unwrap(),
                    decimals: 12,
                }
            ));

            // SessionsPerEra depends on fast-runtime feature, this test should pass regardless
            let sessions_per_era = SessionsPerEra::get();

            // This will call on_era_end for era 0
            run_to_session(sessions_per_era);

            let outbound_msg_queue_event = System::events()
                .iter()
                .filter(|r| match r.event {
                    RuntimeEvent::EthereumOutboundQueue(
                        snowbridge_pallet_outbound_queue::Event::MessageQueued { .. },
                    ) => true,
                    _ => false,
                })
                .count();

            assert_eq!(
                outbound_msg_queue_event, 1,
                "MessageQueued event should be emitted"
            );
        });
}

#[test]
fn external_validators_rewards_merkle_proofs() {
    use {crate::ValidatorIndex, runtime_parachains::inclusion::RewardValidators};

    ExtBuilder::default()
        .with_balances(vec![
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            // SessionsPerEra depends on fast-runtime feature, this test should pass regardless
            let sessions_per_era = SessionsPerEra::get();

            assert_ok!(ExternalValidators::skip_external_validators(
                root_origin(),
                false
            ));
            assert_eq!(
                ExternalValidators::whitelisted_validators(),
                vec![AccountId::from(ALICE), AccountId::from(BOB)]
            );
            assert_ok!(ExternalValidators::remove_whitelisted(
                root_origin(),
                AccountId::from(ALICE)
            ));
            assert_ok!(ExternalValidators::remove_whitelisted(
                root_origin(),
                AccountId::from(BOB)
            ));

            assert_ok!(ExternalValidators::set_external_validators_inner(
                vec![AccountId::from(CHARLIE), AccountId::from(DAVE)],
                1
            ));

            // Register CHARLIE and DAVE session keys
            let charlie_keys =
                get_authority_keys_from_seed(&AccountId::from(CHARLIE).to_string(), None);
            let dave_keys = get_authority_keys_from_seed(&AccountId::from(DAVE).to_string(), None);
            assert_ok!(Session::set_keys(
                origin_of(CHARLIE.into()),
                crate::SessionKeys {
                    babe: charlie_keys.babe.clone(),
                    grandpa: charlie_keys.grandpa.clone(),
                    para_validator: charlie_keys.para_validator.clone(),
                    para_assignment: charlie_keys.para_assignment.clone(),
                    authority_discovery: charlie_keys.authority_discovery.clone(),
                    beefy: charlie_keys.beefy.clone(),
                    nimbus: charlie_keys.nimbus.clone(),
                },
                vec![]
            ));
            assert_ok!(Session::set_keys(
                origin_of(DAVE.into()),
                crate::SessionKeys {
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

            run_to_session(sessions_per_era);
            run_block();
            let validators = Session::validators();

            // Only external validators get selected
            assert_eq!(
                validators,
                vec![AccountId::from(CHARLIE), AccountId::from(DAVE)]
            );

            // Reward all validators in era 1
            crate::RewardValidators::reward_backing(vec![ValidatorIndex(0)]);
            crate::RewardValidators::reward_backing(vec![ValidatorIndex(1)]);

            assert_eq!(
                pallet_external_validators_rewards::RewardPointsForEra::<Runtime>::iter().count(),
                1
            );

            let (_era_index, era_rewards) =
                pallet_external_validators_rewards::RewardPointsForEra::<Runtime>::iter()
                    .next()
                    .unwrap();
            assert!(era_rewards.total > 0);

            println!("era_rewards: {era_rewards:?}");

            let charlie_merkle_proof = ExternalValidatorsRewards::generate_rewards_merkle_proof(
                AccountId::from(CHARLIE),
                1u32,
            );
            let is_charlie_merkle_proof_valid =
                ExternalValidatorsRewards::verify_rewards_merkle_proof(
                    charlie_merkle_proof.unwrap(),
                );

            let dave_merkle_proof = ExternalValidatorsRewards::generate_rewards_merkle_proof(
                AccountId::from(DAVE),
                1u32,
            );
            let is_dave_merkle_proof_valid =
                ExternalValidatorsRewards::verify_rewards_merkle_proof(dave_merkle_proof.unwrap());

            assert!(is_charlie_merkle_proof_valid);
            assert!(is_dave_merkle_proof_valid);

            // Let's check invalid proofs now.
            // Alice is not a validator anymore.
            let alice_merkle_proof = ExternalValidatorsRewards::generate_rewards_merkle_proof(
                AccountId::from(ALICE),
                1u32,
            );

            let charlie_invalid_merkle_proof =
                ExternalValidatorsRewards::generate_rewards_merkle_proof(
                    AccountId::from(CHARLIE),
                    0u32,
                );

            let dave_invalid_merkle_proof =
                ExternalValidatorsRewards::generate_rewards_merkle_proof(
                    AccountId::from(DAVE),
                    2u32,
                );

            // Alice is not present in the validator set, so no merkle proof for her.
            assert!(alice_merkle_proof.is_none());

            // Charlie wasn't rewarded for era 0.
            assert!(charlie_invalid_merkle_proof.is_none());

            // Proof for a future era should also be invalid.
            assert!(dave_invalid_merkle_proof.is_none());
        });
}

#[test]
fn external_validators_whitelisted_never_rewarded() {
    use {crate::ValidatorIndex, runtime_parachains::inclusion::RewardValidators};

    ExtBuilder::default()
        .with_balances(vec![
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            // * Test starts with whitelisted validators: Alice, Bob
            // * Remove Alice and Bob, add Charlie and Dave as external validators
            // * Wait until next era change.
            //   Before that, validators will be Alice and Bob, and they will never get rewards
            //   because whitelisted validators don't get rewards.
            //   After the era change, Alice and Bob also won't get rewards because they no longer
            //   are validators. Charlie and Dave will start getting rewards.
            // * This test ensures that Alice never gets any rewards because of edge cases during
            //   era changes.

            // SessionsPerEra depends on fast-runtime feature, this test should pass regardless
            let sessions_per_era = SessionsPerEra::get();

            assert_ok!(ExternalValidators::skip_external_validators(
                root_origin(),
                false
            ));
            assert_eq!(
                ExternalValidators::whitelisted_validators(),
                vec![AccountId::from(ALICE), AccountId::from(BOB)]
            );
            assert_ok!(ExternalValidators::remove_whitelisted(
                root_origin(),
                AccountId::from(ALICE)
            ));
            assert_ok!(ExternalValidators::remove_whitelisted(
                root_origin(),
                AccountId::from(BOB)
            ));

            // Register CHARLIE and DAVE session keys
            let charlie_keys =
                get_authority_keys_from_seed(&AccountId::from(CHARLIE).to_string(), None);
            let dave_keys = get_authority_keys_from_seed(&AccountId::from(DAVE).to_string(), None);
            assert_ok!(Session::set_keys(
                origin_of(CHARLIE.into()),
                crate::SessionKeys {
                    babe: charlie_keys.babe.clone(),
                    grandpa: charlie_keys.grandpa.clone(),
                    para_validator: charlie_keys.para_validator.clone(),
                    para_assignment: charlie_keys.para_assignment.clone(),
                    authority_discovery: charlie_keys.authority_discovery.clone(),
                    beefy: charlie_keys.beefy.clone(),
                    nimbus: charlie_keys.nimbus.clone(),
                },
                vec![]
            ));
            assert_ok!(Session::set_keys(
                origin_of(DAVE.into()),
                crate::SessionKeys {
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

            // Add Charlie as a whitelisted validator, and Dave as an external validator.
            // Check that Dave is rewarded and Charlie is not.
            assert_ok!(ExternalValidators::add_whitelisted(
                root_origin(),
                AccountId::from(CHARLIE)
            ));
            assert_ok!(ExternalValidators::set_external_validators_inner(
                vec![AccountId::from(DAVE)],
                1
            ));

            // Reward validators in every session
            for session in 1..(sessions_per_era + 1) {
                run_to_block(session_to_block(session));
                run_block();
                let validators = Session::validators();

                // Reward all validators
                crate::RewardValidators::reward_backing(vec![ValidatorIndex(0)]);
                crate::RewardValidators::reward_backing(vec![ValidatorIndex(1)]);

                if session < sessions_per_era {
                    // session < 6
                    assert_eq!(
                        validators,
                        vec![AccountId::from(ALICE), AccountId::from(BOB)]
                    );

                    let (_era_index, era_rewards) =
                        pallet_external_validators_rewards::RewardPointsForEra::<Runtime>::iter()
                            .next()
                            .unwrap();
                    assert_eq!(era_rewards.total, 0);
                } else {
                    // session >= 6
                    assert_eq!(
                        validators,
                        vec![AccountId::from(CHARLIE), AccountId::from(DAVE)]
                    );

                    let (_era_index, era_rewards) =
                        pallet_external_validators_rewards::RewardPointsForEra::<Runtime>::iter()
                            .next()
                            .unwrap();
                    assert!(era_rewards.total > 0);

                    let dave_merkle_proof =
                        ExternalValidatorsRewards::generate_rewards_merkle_proof(
                            AccountId::from(DAVE),
                            1u32,
                        );
                    let is_dave_merkle_proof_valid =
                        ExternalValidatorsRewards::verify_rewards_merkle_proof(
                            dave_merkle_proof.unwrap(),
                        );

                    assert!(is_dave_merkle_proof_valid);
                }

                // Alice never gets rewards, for any era
                let alice_merkle_proof = ExternalValidatorsRewards::generate_rewards_merkle_proof(
                    AccountId::from(ALICE),
                    0u32,
                );
                assert!(alice_merkle_proof.is_none());
                let alice_merkle_proof = ExternalValidatorsRewards::generate_rewards_merkle_proof(
                    AccountId::from(ALICE),
                    1u32,
                );
                assert!(alice_merkle_proof.is_none());
            }
        });
}

#[test]
fn external_validators_rewards_test_command_integrity() {
    use {crate::ValidatorIndex, runtime_parachains::inclusion::RewardValidators};

    ExtBuilder::default()
        .with_balances(vec![
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            let token_location: VersionedLocation = Location::here().into();

            assert_ok!(EthereumSystem::register_token(
                root_origin(),
                Box::new(token_location.clone()),
                snowbridge_core::AssetMetadata {
                    name: "dance".as_bytes().to_vec().try_into().unwrap(),
                    symbol: "dance".as_bytes().to_vec().try_into().unwrap(),
                    decimals: 12,
                }
            ));

            let token_id = EthereumSystem::convert_back(&RewardTokenLocation::get()).unwrap();

            // SessionsPerEra depends on fast-runtime feature, this test should pass regardless
            let sessions_per_era = SessionsPerEra::get();

            assert_ok!(ExternalValidators::skip_external_validators(
                root_origin(),
                false
            ));
            assert_eq!(
                ExternalValidators::whitelisted_validators(),
                vec![AccountId::from(ALICE), AccountId::from(BOB)]
            );
            assert_ok!(ExternalValidators::remove_whitelisted(
                root_origin(),
                AccountId::from(ALICE)
            ));
            assert_ok!(ExternalValidators::remove_whitelisted(
                root_origin(),
                AccountId::from(BOB)
            ));

            assert_ok!(ExternalValidators::set_external_validators_inner(
                vec![AccountId::from(CHARLIE), AccountId::from(DAVE)],
                1
            ));

            // Register CHARLIE and DAVE session keys
            let charlie_keys =
                get_authority_keys_from_seed(&AccountId::from(CHARLIE).to_string(), None);
            let dave_keys = get_authority_keys_from_seed(&AccountId::from(DAVE).to_string(), None);
            assert_ok!(Session::set_keys(
                origin_of(CHARLIE.into()),
                crate::SessionKeys {
                    babe: charlie_keys.babe.clone(),
                    grandpa: charlie_keys.grandpa.clone(),
                    para_validator: charlie_keys.para_validator.clone(),
                    para_assignment: charlie_keys.para_assignment.clone(),
                    authority_discovery: charlie_keys.authority_discovery.clone(),
                    beefy: charlie_keys.beefy.clone(),
                    nimbus: charlie_keys.nimbus.clone(),
                },
                vec![]
            ));
            assert_ok!(Session::set_keys(
                origin_of(DAVE.into()),
                crate::SessionKeys {
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

            run_to_session(sessions_per_era);
            run_block();
            let validators = Session::validators();

            // Only whitelisted validators get selected
            assert_eq!(
                validators,
                vec![AccountId::from(CHARLIE), AccountId::from(DAVE)]
            );

            // Validators are automatically rewarded.
            assert_eq!(
                pallet_external_validators_rewards::RewardPointsForEra::<Runtime>::iter().count(),
                1
            );

            let expected_inflation =
                <Runtime as pallet_external_validators_rewards::Config>::EraInflationProvider::get(
                );

            // This will call on_era_end for era 1
            run_to_session(sessions_per_era * 2);

            let mut rewards_command_found: Option<Command> = None;
            let mut message_id_found: Option<H256> = None;
            let ext_validators_rewards_event = System::events()
                .iter()
                .filter(|r| match &r.event {
                    RuntimeEvent::ExternalValidatorsRewards(
                        pallet_external_validators_rewards::Event::RewardsMessageSent {
                            rewards_command,
                            message_id,
                        },
                    ) => {
                        message_id_found = Some(*message_id);
                        rewards_command_found = Some(rewards_command.clone());
                        true
                    }
                    _ => false,
                })
                .count();

            let rewards_utils = ExternalValidatorsRewards::generate_era_rewards_utils(1, None);

            let blocks_per_session: u128 = Babe::current_epoch().duration.into();
            let points_per_block = 20;
            let expected_total_points =
                (sessions_per_era as u128) * blocks_per_session * points_per_block;

            let expected_rewards_command = Command::ReportRewards {
                external_idx: 1u64,
                era_index: 1u32,
                total_points: expected_total_points,
                tokens_inflated: expected_inflation,
                rewards_merkle_root: rewards_utils.unwrap().rewards_merkle_root,
                token_id,
            };

            assert_eq!(
                ext_validators_rewards_event, 1,
                "RewardsMessageSent event should be emitted"
            );
            assert_eq!(
                rewards_command_found.unwrap(),
                expected_rewards_command,
                "Both rewards commands should match!"
            );
            assert_eq!(message_id_found.unwrap(), read_last_entropy().into());
        });
}

#[test]
fn external_validators_rewards_are_minted_in_sovereign_account() {
    ExtBuilder::default()
        .with_balances(vec![
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .with_validators(
            vec![]
        )
        .with_external_validators(
            vec![
                (AccountId::from(ALICE), 210 * UNIT),
                (AccountId::from(BOB), 100 * UNIT),
            ]
        )
        .build()
        .execute_with(|| {
            let token_location: VersionedLocation = Location::here()
            .into();

            assert_ok!(EthereumSystem::register_token(root_origin(), Box::new(token_location), snowbridge_core::AssetMetadata {
                name: "dance".as_bytes().to_vec().try_into().unwrap(),
                symbol: "dance".as_bytes().to_vec().try_into().unwrap(),
                decimals: 12,
		    }));

            // SessionsPerEra depends on fast-runtime feature, this test should pass regardless
            let sessions_per_era = SessionsPerEra::get();

            let sovereign_acount = <Runtime as pallet_external_validators_rewards::Config>::RewardsEthereumSovereignAccount::get();

            let balance_before = System::account(sovereign_acount.clone())
                .data
                .free;

            let expected_inflation = <Runtime as pallet_external_validators_rewards::Config>::EraInflationProvider::get();

            // This will call on_era_end for era 0
            run_to_session(sessions_per_era);

            let balance_after = System::account(sovereign_acount)
                .data
                .free;

            assert_eq!(
                balance_after - balance_before,
                expected_inflation,
                "Inflation should be minted in Ethereum Sovereign Account"
            );
        });
}
