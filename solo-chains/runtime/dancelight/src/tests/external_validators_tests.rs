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
    crate::{
        tests::common::*, EthereumSystem, ExternalValidators, ExternalValidatorsRewards, MaxExternalValidators, RuntimeEvent, SessionKeys, SessionsPerEra, System
    }, frame_support::{assert_ok, traits::fungible::Mutate}, pallet_external_validators::Forcing, parity_scale_codec::Encode,
    snowbridge_core::{Channel, PRIMARY_GOVERNANCE_CHANNEL}, 
    sp_core::H256, sp_io::hashing::twox_64, std::{collections::HashMap, ops::RangeInclusive}, tp_traits::OnEraEnd
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

            ExternalValidators::set_external_validators(external_validators).unwrap();

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

                ExternalValidators::set_external_validators(vec![mock_validator]).unwrap();

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

            ExternalValidators::set_external_validators(vec![mock_validator.clone()]).unwrap();
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
            ExternalValidators::set_external_validators(vec![AccountId::from(ALICE)]).unwrap();

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

                ExternalValidators::set_external_validators(vec![mock_validator]).unwrap();

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

                ExternalValidators::set_external_validators(vec![mock_validator.clone()]).unwrap();
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
                ExternalValidators::set_external_validators(vec![]).unwrap();
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

                ExternalValidators::set_external_validators(vec![mock_validator.clone()]).unwrap();
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

                ExternalValidators::set_external_validators(vec![mock_validator.clone()]).unwrap();
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

                ExternalValidators::set_external_validators(vec![]).unwrap();
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

            ExternalValidators::set_external_validators(vec![mock_validator.clone()]).unwrap();
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

            let channel_id = PRIMARY_GOVERNANCE_CHANNEL.encode();

            // Insert PRIMARY_GOVERNANCE_CHANNEL channel id into storage.
            let mut combined_channel_id_key = Vec::new();
            let hashed_key = twox_64(&channel_id);

            combined_channel_id_key.extend_from_slice(&hashed_key);
            combined_channel_id_key.extend_from_slice(PRIMARY_GOVERNANCE_CHANNEL.as_ref());

            let mut full_storage_key = Vec::new();
            full_storage_key.extend_from_slice(&frame_support::storage::storage_prefix(b"EthereumSystem", b"Channels"));
            full_storage_key.extend_from_slice(&combined_channel_id_key);

            let channel = Channel {
                agent_id: H256::default(),
                para_id: 1000u32.into()
            };

            frame_support::storage::unhashed::put(&full_storage_key, &channel);
            
            // This will call on_era_end for era 0
            run_to_session(sessions_per_era);

            let outbound_msg_queue_event = System::events()
				.iter()
				.filter(|r| match r.event {
					RuntimeEvent::EthereumOutboundQueue(snowbridge_pallet_outbound_queue::Event::MessageQueued { .. }) => true,
					_ => false,
				})
				.count();

			assert_eq!(outbound_msg_queue_event, 1, "MessageQueued event should be emitted");
        });
}
