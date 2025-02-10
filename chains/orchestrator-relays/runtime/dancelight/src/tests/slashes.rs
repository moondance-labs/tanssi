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
    crate::tests::common::*,
    crate::{
        BondingDuration, EthereumSystem, ExternalValidatorSlashes, ExternalValidators, Grandpa,
        Historical, RuntimeEvent, SessionsPerEra, SlashDeferDuration,
    },
    frame_support::{assert_noop, assert_ok, traits::KeyOwnerProofSystem},
    parity_scale_codec::Encode,
    sp_core::{Pair, H256},
    sp_runtime::Perbill,
    sp_std::vec,
    tp_bridge::Command,
    xcm::latest::prelude::*,
    xcm::VersionedLocation,
};

#[test]
fn invulnerables_cannot_be_slashed_with_babe() {
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
            inject_babe_slash(&AccountId::from(ALICE).to_string());
            let reports = pallet_offences::Reports::<crate::Runtime>::iter().collect::<Vec<_>>();
            assert_eq!(reports.len(), 1);
            assert_eq!(ExternalValidators::current_era().unwrap(), 0);

            let slashes = ExternalValidatorSlashes::slashes(
                ExternalValidators::current_era().unwrap() + SlashDeferDuration::get() + 1,
            );
            assert_eq!(slashes.len(), 0);
        });
}

#[test]
fn invulnerables_cannot_be_slashed_with_grandpa() {
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
            inject_grandpa_slash(&AccountId::from(ALICE).to_string());
            let reports = pallet_offences::Reports::<crate::Runtime>::iter().collect::<Vec<_>>();
            assert_eq!(reports.len(), 1);
            assert_eq!(ExternalValidators::current_era().unwrap(), 0);

            let slashes = ExternalValidatorSlashes::slashes(
                ExternalValidators::current_era().unwrap() + SlashDeferDuration::get() + 1,
            );
            assert_eq!(slashes.len(), 0);
        });
}

#[test]
fn non_invulnerables_can_be_slashed_with_babe() {
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
            assert_ok!(ExternalValidators::remove_whitelisted(
                RuntimeOrigin::root(),
                AccountId::from(ALICE)
            ));

            inject_babe_slash(&AccountId::from(ALICE).to_string());

            let reports = pallet_offences::Reports::<crate::Runtime>::iter().collect::<Vec<_>>();
            assert_eq!(reports.len(), 1);
            assert_eq!(ExternalValidators::current_era().unwrap(), 0);

            let slashes = ExternalValidatorSlashes::slashes(
                ExternalValidators::current_era().unwrap() + SlashDeferDuration::get() + 1,
            );
            assert_eq!(slashes.len(), 1);
            assert_eq!(slashes[0].validator, AccountId::from(ALICE));
            //the formula is (3*offenders/num_validators)^2
            // since we have 1 offender, 2 validators, this makes it a maximum of 1
            assert_eq!(slashes[0].percentage, Perbill::from_percent(100));
        });
}

#[test]
fn non_invulnerables_can_be_slashed_with_grandpa() {
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
            assert_ok!(ExternalValidators::remove_whitelisted(
                RuntimeOrigin::root(),
                AccountId::from(ALICE)
            ));

            inject_grandpa_slash(&AccountId::from(ALICE).to_string());

            let reports = pallet_offences::Reports::<crate::Runtime>::iter().collect::<Vec<_>>();
            assert_eq!(reports.len(), 1);
            assert_eq!(ExternalValidators::current_era().unwrap(), 0);

            let slashes = ExternalValidatorSlashes::slashes(
                ExternalValidators::current_era().unwrap() + SlashDeferDuration::get() + 1,
            );
            assert_eq!(slashes.len(), 1);
            assert_eq!(slashes[0].validator, AccountId::from(ALICE));
            //the formula is (3*offenders/num_validators)^2
            // since we have 1 offender, 2 validators, this makes it a maximum of 1
            assert_eq!(slashes[0].percentage, Perbill::from_percent(100));
        });
}

#[test]
fn test_slashing_percentage_applied_correctly() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_validators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);
            assert_ok!(ExternalValidators::remove_whitelisted(
                RuntimeOrigin::root(),
                AccountId::from(ALICE)
            ));

            inject_babe_slash(&AccountId::from(ALICE).to_string());

            let reports = pallet_offences::Reports::<crate::Runtime>::iter().collect::<Vec<_>>();
            assert_eq!(reports.len(), 1);
            assert_eq!(ExternalValidators::current_era().unwrap(), 0);

            let slashes = ExternalValidatorSlashes::slashes(
                ExternalValidators::current_era().unwrap() + SlashDeferDuration::get() + 1,
            );
            assert_eq!(slashes.len(), 1);
            assert_eq!(slashes[0].validator, AccountId::from(ALICE));
            //the formula is (3*offenders/num_validators)^2
            // since we have 1 offender, 4 validators, this makes it a maximum of 0.75^2=0.5625
            assert_eq!(slashes[0].percentage, Perbill::from_parts(562500000));
        });
}

#[test]
fn test_slashes_are_not_additive_in_percentage() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
            (AccountId::from(EVE), 100_000 * UNIT),
        ])
        .with_validators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
            (AccountId::from(EVE), 100 * UNIT),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);
            assert_ok!(ExternalValidators::remove_whitelisted(
                RuntimeOrigin::root(),
                AccountId::from(ALICE)
            ));

            inject_babe_slash(&AccountId::from(ALICE).to_string());

            inject_grandpa_slash(&AccountId::from(ALICE).to_string());

            let reports = pallet_offences::Reports::<crate::Runtime>::iter().collect::<Vec<_>>();

            // we have 2 reports
            assert_eq!(reports.len(), 2);
            assert_eq!(ExternalValidators::current_era().unwrap(), 0);

            let slashes = ExternalValidatorSlashes::slashes(
                ExternalValidators::current_era().unwrap() + SlashDeferDuration::get() + 1,
            );

            // but a single slash
            assert_eq!(slashes.len(), 1);
            assert_eq!(slashes[0].validator, AccountId::from(ALICE));
            // the formula is (3*offenders/num_validators)^2
            // since we have 1 offender, 5 validators, this makes it 0.36
            // we injected 2 offences BUT THEY ARE NOT ADDITIVE
            assert_eq!(slashes[0].percentage, Perbill::from_parts(360000000));
        });
}
#[test]
fn test_slashes_are_cleaned_after_bonding_period() {
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
            assert_ok!(ExternalValidators::remove_whitelisted(
                RuntimeOrigin::root(),
                AccountId::from(ALICE)
            ));

            inject_babe_slash(&AccountId::from(ALICE).to_string());

            let reports = pallet_offences::Reports::<crate::Runtime>::iter().collect::<Vec<_>>();
            assert_eq!(reports.len(), 1);
            assert_eq!(ExternalValidators::current_era().unwrap(), 0);

            let slashes = ExternalValidatorSlashes::slashes(
                ExternalValidators::current_era().unwrap() + SlashDeferDuration::get() + 1,
            );
            assert_eq!(slashes.len(), 1);
            // The first session in which the era 3 will be pruned is
            // (28+3+1)*sessionsPerEra
            let fist_session_era_3_pruned = (ExternalValidators::current_era().unwrap()
                + SlashDeferDuration::get()
                + 1
                + BondingDuration::get()
                + 1)
                * SessionsPerEra::get();

            let first_era_deferred =
                ExternalValidators::current_era().unwrap() + SlashDeferDuration::get() + 1;

            println!("first era deferred {:?}", first_era_deferred);
            run_to_session(fist_session_era_3_pruned);

            let slashes_after_bonding_period =
                ExternalValidatorSlashes::slashes(first_era_deferred);
            assert_eq!(slashes_after_bonding_period.len(), 0);
        });
}

#[test]
fn test_slashes_can_be_cleared_before_deferred_period_applies() {
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
            assert_ok!(ExternalValidators::remove_whitelisted(
                RuntimeOrigin::root(),
                AccountId::from(ALICE)
            ));

            inject_babe_slash(&AccountId::from(ALICE).to_string());

            let reports = pallet_offences::Reports::<crate::Runtime>::iter().collect::<Vec<_>>();
            assert_eq!(reports.len(), 1);
            assert_eq!(ExternalValidators::current_era().unwrap(), 0);

            let deferred_era =
                ExternalValidators::current_era().unwrap() + SlashDeferDuration::get() + 1;
            let slashes = ExternalValidatorSlashes::slashes(deferred_era);
            assert_eq!(slashes.len(), 1);
            assert_eq!(slashes[0].validator, AccountId::from(ALICE));

            // Now let's clean it up
            assert_ok!(ExternalValidatorSlashes::cancel_deferred_slash(
                RuntimeOrigin::root(),
                deferred_era,
                vec![0]
            ));
            let slashes_after_cancel = ExternalValidatorSlashes::slashes(deferred_era);
            assert_eq!(slashes_after_cancel.len(), 0);
        });
}

#[test]
fn test_slashes_cannot_be_cancelled_after_defer_period() {
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

            run_to_block(2);
            assert_ok!(ExternalValidators::remove_whitelisted(
                RuntimeOrigin::root(),
                AccountId::from(ALICE)
            ));

            inject_babe_slash(&AccountId::from(ALICE).to_string());

            let reports = pallet_offences::Reports::<crate::Runtime>::iter().collect::<Vec<_>>();
            assert_eq!(reports.len(), 1);
            assert_eq!(ExternalValidators::current_era().unwrap(), 0);

            let deferred_era =
                ExternalValidators::current_era().unwrap() + SlashDeferDuration::get() + 1;

            let slashes = ExternalValidatorSlashes::slashes(deferred_era);
            assert_eq!(slashes.len(), 1);
            assert_eq!(slashes[0].validator, AccountId::from(ALICE));

            // The first session in which the era 3 will be deferred is 18
            // 3 sessions per era
            // (externalValidators::current_era().unwrap() + SlashDeferDuration::get() + 1)*SessionsPerEra
            // formula is:

            let first_deferred_session =
                (ExternalValidators::current_era().unwrap() + SlashDeferDuration::get() + 1)
                    * SessionsPerEra::get();
            run_to_session(first_deferred_session);

            assert_eq!(ExternalValidators::current_era().unwrap(), deferred_era);
            // Now let's clean it up
            assert_noop!(
                ExternalValidatorSlashes::cancel_deferred_slash(
                    RuntimeOrigin::root(),
                    deferred_era,
                    vec![0]
                ),
                pallet_external_validator_slashes::Error::<crate::Runtime>::DeferPeriodIsOver
            );
        });
}

#[test]
fn test_slashes_are_sent_to_ethereum() {
    sp_tracing::try_init_simple();
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
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

            run_to_block(2);

            inject_babe_slash(&AccountId::from(ALICE).to_string());

            let reports = pallet_offences::Reports::<crate::Runtime>::iter().collect::<Vec<_>>();
            assert_eq!(reports.len(), 1);
            assert_eq!(ExternalValidators::current_era().unwrap(), 0);

            let deferred_era =
                ExternalValidators::current_era().unwrap() + SlashDeferDuration::get() + 1;

            let slashes = ExternalValidatorSlashes::slashes(deferred_era);
            assert_eq!(slashes.len(), 1);
            assert_eq!(slashes[0].validator, AccountId::from(ALICE));

            let session_in_which_slashes_are_sent =
                (ExternalValidators::current_era().unwrap() + SlashDeferDuration::get() + 1)
                    * SessionsPerEra::get();
            run_to_session(session_in_which_slashes_are_sent);

            let outbound_msg_queue_event = System::events()
                .iter()
                .filter(|r| match r.event {
                    RuntimeEvent::EthereumOutboundQueue(
                        snowbridge_pallet_outbound_queue::Event::MessageQueued { .. },
                    ) => true,
                    _ => false,
                })
                .count();

            // We have two reasons for sending messages:
            // 1, because on_era_end sends rewards
            // 2, because on_era_start sends slashes
            // Both session ends and session starts are done on_initialize of frame-sesssion
            assert_eq!(
                outbound_msg_queue_event, 1,
                "MessageQueued event should be emitted"
            );

            // Slashes start being sent after the era block
            // They are scheduled as unprocessedSlashes
            run_block();

            let outbound_msg_queue_event = System::events()
                .iter()
                .filter(|r| match r.event {
                    RuntimeEvent::EthereumOutboundQueue(
                        snowbridge_pallet_outbound_queue::Event::MessageQueued { .. },
                    ) => true,
                    _ => false,
                })
                .count();

            let mut slashes_command_found: Option<Command> = None;
            let mut message_id_found: Option<H256> = None;
            let ext_validators_slashes_event = System::events()
                .iter()
                .filter(|r| match &r.event {
                    RuntimeEvent::ExternalValidatorSlashes(
                        pallet_external_validator_slashes::Event::SlashesMessageSent {
                            slashes_command,
                            message_id,
                        },
                    ) => {
                        message_id_found = Some(*message_id);
                        slashes_command_found = Some(slashes_command.clone());
                        true
                    }
                    _ => false,
                })
                .count();

            // This one is related to slashes
            assert_eq!(
                outbound_msg_queue_event, 1,
                "MessageQueued event should be emitted"
            );

            assert_eq!(
                ext_validators_slashes_event, 1,
                "SlashesMessageSent event should be emitted"
            );

            let expected_slashes = vec![SlashData {
                encoded_validator_id: AccountId::from(ALICE).encode(),
                slash_fraction: Perbill::from_percent(100).deconstruct(),
                external_idx: 0,
            }];

            let expected_slashes_command = Command::ReportSlashes {
                era_index: 1u32,
                slashes: expected_slashes,
            };

            assert_eq!(
                slashes_command_found.unwrap(),
                expected_slashes_command,
                "Both slashes commands should match!"
            );

            assert_eq!(message_id_found.unwrap(), read_last_entropy().into());

            // EthereumOutboundQueue -> queue_message -> MessageQQueuePallet (queue)
            // MessageQueuePallet on_initialize -> dispatch queue -> process_message -> EthereumOutboundQueue_process_message
            let nonce = snowbridge_pallet_outbound_queue::Nonce::<Runtime>::get(
                snowbridge_core::PRIMARY_GOVERNANCE_CHANNEL,
            );

            // We dispatched 2 already
            assert_eq!(nonce, 2);
        });
}

use frame_support::traits::Get;
use tp_bridge::SlashData;

#[test]
fn test_slashes_are_sent_to_ethereum_accumulatedly() {
    sp_tracing::try_init_simple();
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
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


            run_to_block(2);

            // We can inject arbitraqry slashes for arbitary accounts with root
            let page_limit: u32 = <Runtime as pallet_external_validator_slashes::Config>::QueuedSlashesProcessedPerBlock::get();
            for i in 0..page_limit +1 {
                assert_ok!(ExternalValidatorSlashes::force_inject_slash(
                    RuntimeOrigin::root(),
                    0,
                    AccountId::new(H256::from_low_u64_be(i as u64).to_fixed_bytes()),
                    Perbill::from_percent(75),
                    1
                ));
            }

            let deferred_era = ExternalValidators::current_era().unwrap() + SlashDeferDuration::get() + 1;

            let slashes = ExternalValidatorSlashes::slashes(deferred_era);
            assert_eq!(slashes.len() as u32, page_limit +1);

            let session_in_which_slashes_are_sent =
                (ExternalValidators::current_era().unwrap() + SlashDeferDuration::get() + 1)
                    * SessionsPerEra::get();
            run_to_session(session_in_which_slashes_are_sent);

            let outbound_msg_queue_event = System::events()
                .iter()
                .filter(|r| match r.event {
                    RuntimeEvent::EthereumOutboundQueue(
                        snowbridge_pallet_outbound_queue::Event::MessageQueued { .. },
                    ) => true,
                    _ => false,
                })
                .count();

            // We have two reasons for sending messages:
            // 1, because on_era_end sends rewards
            // 2, because on_era_start sends slashes
            // Both session ends and session starts are done on_initialize of frame-sesssion
            assert_eq!(
                outbound_msg_queue_event, 1,
                "MessageQueued event should be emitted"
            );

            // We still have all slashes as unprocessed
            let unprocessed_slashes = ExternalValidatorSlashes::unreported_slashes();
            assert_eq!(unprocessed_slashes.len() as u32, page_limit +1);


            // Slashes start being sent after the era block
            // They are scheduled as unprocessedSlashes
            run_block();

            let outbound_msg_queue_event = System::events()
                .iter()
                .filter(|r| match r.event {
                    RuntimeEvent::EthereumOutboundQueue(
                        snowbridge_pallet_outbound_queue::Event::MessageQueued { .. },
                    ) => true,
                    _ => false,
                })
                .count();

            let unprocessed_slashes = ExternalValidatorSlashes::unreported_slashes();

            // This one is related to slashes
            assert_eq!(
                outbound_msg_queue_event, 1,
                "MessageQueued event should be emitted"
            );

            // We still should have one pending unprocessed slash, to be sent in the next block
            assert_eq!(unprocessed_slashes.len() as u32, 1);

            run_block();

            let outbound_msg_queue_event = System::events()
                .iter()
                .filter(|r| match r.event {
                    RuntimeEvent::EthereumOutboundQueue(
                        snowbridge_pallet_outbound_queue::Event::MessageQueued { .. },
                    ) => true,
                    _ => false,
                })
                .count();

            let unprocessed_slashes = ExternalValidatorSlashes::unreported_slashes();

            // This one is related to slashes
            assert_eq!(
                outbound_msg_queue_event, 1,
                "MessageQueued event should be emitted"
            );

            // Now we should have 0
            assert_eq!(unprocessed_slashes.len() as u32, 0);

            // EthereumOutboundQueue -> queue_message -> MessageQQueuePallet (queue)
            // MessageQueuePallet on_initialize -> dispatch queue -> process_message -> EthereumOutboundQueue_process_message
            let nonce = snowbridge_pallet_outbound_queue::Nonce::<Runtime>::get(
                snowbridge_core::PRIMARY_GOVERNANCE_CHANNEL,
            );

            // We dispatched 3 already
            // 1 reward + 2 slashes
            assert_eq!(nonce, 3);
        });
}

#[test]
fn test_slashes_are_sent_to_ethereum_accumulate_until_next_era() {
    sp_tracing::try_init_simple();
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
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

            run_to_block(2);

            // We can inject arbitraqry slashes for arbitary accounts with root
            let page_limit: u32 = <Runtime as pallet_external_validator_slashes::Config>::QueuedSlashesProcessedPerBlock::get();

            let blocks_in_era = crate::EpochDurationInBlocks::get() * SessionsPerEra::get();
            let total_slashes_to_inject = blocks_in_era*page_limit +1;
            for i in 0..total_slashes_to_inject {
                assert_ok!(ExternalValidatorSlashes::force_inject_slash(
                    RuntimeOrigin::root(),
                    0,
                    AccountId::new(H256::from_low_u64_be(i as u64).to_fixed_bytes()),
                    Perbill::from_percent(75),
                    1
                ));
            }

            let deferred_era = ExternalValidators::current_era().unwrap() + SlashDeferDuration::get() + 1;

            let slashes = ExternalValidatorSlashes::slashes(deferred_era);
            assert_eq!(slashes.len() as u32, total_slashes_to_inject);

            let session_in_which_slashes_are_sent =
                (ExternalValidators::current_era().unwrap() + SlashDeferDuration::get() + 1)
                    * SessionsPerEra::get();
            run_to_session(session_in_which_slashes_are_sent);

            let outbound_msg_queue_event = System::events()
                .iter()
                .filter(|r| match r.event {
                    RuntimeEvent::EthereumOutboundQueue(
                        snowbridge_pallet_outbound_queue::Event::MessageQueued { .. },
                    ) => true,
                    _ => false,
                })
                .count();

            // We have two reasons for sending messages:
            // 1, because on_era_end sends rewards
            // 2, because on_era_start sends slashes
            // Both session ends and session starts are done on_initialize of frame-sesssion
            assert_eq!(
                outbound_msg_queue_event, 1,
                "MessageQueued event should be emitted"
            );

            // We still have all slashes as unprocessed
            let unprocessed_slashes = ExternalValidatorSlashes::unreported_slashes();
            assert_eq!(unprocessed_slashes.len() as u32, total_slashes_to_inject);

            // Running to the next era, but we should still have unprocessed
            run_to_session((ExternalValidators::current_era().unwrap() +1)*SessionsPerEra::get());

            let unprocessed_slashes = ExternalValidatorSlashes::unreported_slashes();

            // We still should have one pending unprocessed slash, to be sent in the next block
            assert_eq!(unprocessed_slashes.len() as u32, 1);

            // And in this case, we have 2 events
            // the rewards one plus the one where we sent remaining slashes
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
                outbound_msg_queue_event, 2,
                "MessageQueued event should be emitted"
            );
        });
}

fn inject_babe_slash(seed: &str) {
    let babe_key = get_pair_from_seed::<babe_primitives::AuthorityId>(seed);
    let equivocation_proof = generate_babe_equivocation_proof(&babe_key);

    // create the key ownership proof
    let key = (babe_primitives::KEY_TYPE, babe_key.public());
    let key_owner_proof = Historical::prove(key).unwrap();

    // report the equivocation
    assert_ok!(Babe::report_equivocation_unsigned(
        RuntimeOrigin::none(),
        Box::new(equivocation_proof),
        key_owner_proof,
    ));
}

fn inject_grandpa_slash(seed: &str) {
    let grandpa_key = get_pair_from_seed::<grandpa_primitives::AuthorityId>(seed);

    let set_id = Grandpa::current_set_id();

    let equivocation_proof = generate_grandpa_equivocation_proof(
        set_id,
        (1, H256::random(), 1, &grandpa_key),
        (1, H256::random(), 1, &grandpa_key),
    );
    // create the key ownership proof
    let key = (grandpa_primitives::KEY_TYPE, grandpa_key.public());
    let key_owner_proof = Historical::prove(key).unwrap();

    // report the equivocation
    assert_ok!(Grandpa::report_equivocation_unsigned(
        RuntimeOrigin::none(),
        Box::new(equivocation_proof),
        key_owner_proof,
    ));
}
