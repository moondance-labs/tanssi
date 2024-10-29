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

use frame_support::traits::KeyOwnerProofSystem;
use sp_core::Pair;
use sp_runtime::Perbill;
use {
    crate::tests::common::*,
    crate::{
        BondingDuration, ExternalValidatorSlashes, ExternalValidators, Grandpa, Historical,
        SessionsPerEra, SlashDeferDuration,
    },
    frame_support::{assert_noop, assert_ok},
    sp_core::H256,
    sp_std::vec,
};

#[test]
fn invulnerables_cannot_be_slashed() {
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
            let reports: Vec<_> = pallet_offences::Reports::<crate::Runtime>::iter()
                .map(|offence| offence)
                .collect();
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

            let reports: Vec<_> = pallet_offences::Reports::<crate::Runtime>::iter()
                .map(|offence| offence)
                .collect();
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

            let reports: Vec<_> = pallet_offences::Reports::<crate::Runtime>::iter()
                .map(|offence| offence)
                .collect();
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

            let reports: Vec<_> = pallet_offences::Reports::<crate::Runtime>::iter()
                .map(|offence| offence)
                .collect();
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

            let reports: Vec<_> = pallet_offences::Reports::<crate::Runtime>::iter()
                .map(|offence| offence)
                .collect();

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

            let reports: Vec<_> = pallet_offences::Reports::<crate::Runtime>::iter()
                .map(|offence| offence)
                .collect();
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

            println!("first session era 3 pruned {:?}", fist_session_era_3_pruned);
            run_to_session(fist_session_era_3_pruned);

            let slashes_after_bonding_period = ExternalValidatorSlashes::slashes(3);
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

            let reports: Vec<_> = pallet_offences::Reports::<crate::Runtime>::iter()
                .map(|offence| offence)
                .collect();
            assert_eq!(reports.len(), 1);
            assert_eq!(ExternalValidators::current_era().unwrap(), 0);

            let slashes = ExternalValidatorSlashes::slashes(
                ExternalValidators::current_era().unwrap() + SlashDeferDuration::get() + 1,
            );
            assert_eq!(slashes.len(), 1);
            assert_eq!(slashes[0].validator, AccountId::from(ALICE));

            // Now let's clean it up
            assert_ok!(ExternalValidatorSlashes::cancel_deferred_slash(
                RuntimeOrigin::root(),
                3,
                vec![0]
            ));
            let slashes_after_cancel = ExternalValidatorSlashes::slashes(3);
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
            run_to_block(2);
            assert_ok!(ExternalValidators::remove_whitelisted(
                RuntimeOrigin::root(),
                AccountId::from(ALICE)
            ));

            inject_babe_slash(&AccountId::from(ALICE).to_string());

            let reports: Vec<_> = pallet_offences::Reports::<crate::Runtime>::iter()
                .map(|offence| offence)
                .collect();
            assert_eq!(reports.len(), 1);
            assert_eq!(ExternalValidators::current_era().unwrap(), 0);

            let slashes = ExternalValidatorSlashes::slashes(
                ExternalValidators::current_era().unwrap() + SlashDeferDuration::get() + 1,
            );
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

            assert_eq!(ExternalValidators::current_era().unwrap(), 3);
            // Now let's clean it up
            assert_noop!(
                ExternalValidatorSlashes::cancel_deferred_slash(RuntimeOrigin::root(), 3, vec![0]),
                pallet_external_validator_slashes::Error::<crate::Runtime>::DeferPeriodIsOver
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
