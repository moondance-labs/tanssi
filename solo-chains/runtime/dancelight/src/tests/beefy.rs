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
    crate::{tests::common::*, AuthorNoting, RewardsPortion},
    beefy_primitives::{
        ecdsa_crypto::{AuthorityId as BeefyId, Signature as BeefySignature},
        ConsensusLog, ValidatorSet, BEEFY_ENGINE_ID,
        known_payloads::MMR_ROOT_ID,
        test_utils::{
            generate_double_voting_proof, generate_fork_voting_proof,
            generate_future_block_voting_proof, Keyring as BeefyKeyring,
        },
        check_double_voting_proof,
        Payload
    },
    cumulus_primitives_core::ParaId,
    frame_support::traits::OnInitialize,
    pallet_beefy::ValidatorSetId,
    parity_scale_codec::{Decode, Encode},
    sp_consensus_aura::AURA_ENGINE_ID,
    sp_runtime::{generic::DigestItem, print, traits::{BlakeTwo256, Keccak256}},
    sp_std::vec,
    test_relay_sproof_builder::{HeaderAs, ParaHeaderSproofBuilder, ParaHeaderSproofBuilderItem},
    tp_traits::ContainerChainBlockInfo,
};

#[test]
fn test_session_change_updates_beefy_authorities_digest() {
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
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            assert_eq!(ValidatorSetId::<Runtime>::get(), 0);

            run_to_session(1);
            assert_eq!(ValidatorSetId::<Runtime>::get(), 1);

            // Get beefy keys for our validators
            let alice_keys =
                get_authority_keys_from_seed(&AccountId::from(ALICE).to_string(), None);
            let bob_keys = get_authority_keys_from_seed(&AccountId::from(BOB).to_string(), None);
            let charlie_keys =
                get_authority_keys_from_seed(&AccountId::from(CHARLIE).to_string(), None);
            let dave_keys = get_authority_keys_from_seed(&AccountId::from(DAVE).to_string(), None);

            let expected_digest = get_beefy_digest(ConsensusLog::AuthoritiesChange(
                ValidatorSet::new(
                    vec![
                        alice_keys.beefy.clone(),
                        bob_keys.beefy.clone(),
                        charlie_keys.beefy.clone(),
                        dave_keys.beefy.clone(),
                    ],
                    1,
                )
                .unwrap(),
            ));

            // Check the expected digest item was placed correctly
            let actual_digest = System::digest().logs[2].clone();
            assert_eq!(expected_digest, actual_digest);

            // Check one more session
            run_to_session(2);
            assert_eq!(ValidatorSetId::<Runtime>::get(), 2);

            let expected_digest = get_beefy_digest(ConsensusLog::AuthoritiesChange(
                ValidatorSet::new(
                    vec![
                        alice_keys.beefy,
                        bob_keys.beefy,
                        charlie_keys.beefy,
                        dave_keys.beefy,
                    ],
                    2,
                )
                .unwrap(),
            ));

            // Check the expected digest item was placed correctly.
            // We should have the new "expected_digest" with the same validators as the
            // previous session, given that we don't have staking yet,
            // thus validator set doesn't change.
            let actual_digest = System::digest().logs[2].clone();
            assert_eq!(expected_digest, actual_digest);
        });
}

#[test]
fn test_valid_and_invalid_double_voting_proofs() {
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
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            let set_id = 3;
            let payload1 = Payload::from_single_entry(MMR_ROOT_ID, vec![42]);
            let payload2 = Payload::from_single_entry(MMR_ROOT_ID, vec![128]);

            // 1st case (invalid proof):
            // Equivocation proof with two votes in the same round for
            // same payload signed by the same key.
            let equivocation_proof = generate_double_voting_proof(
                (1, payload1.clone(), set_id, &BeefyKeyring::Bob),
                (1, payload1.clone(), set_id, &BeefyKeyring::Bob),
            );

            // Previous equivocation proof should be invalid.
            assert!(!check_double_voting_proof::<_, _, Keccak256>(&equivocation_proof));

            // 2nd case (invalid proof):
            // Equivocation proof with two votes in different rounds for
            // different payloads signed by the same key.
            let equivocation_proof = generate_double_voting_proof(
                (1, payload1.clone(), set_id, &BeefyKeyring::Bob),
                (2, payload2.clone(), set_id, &BeefyKeyring::Bob),
            );

            // Previous equivocation proof should be invalid.
            assert!(!check_double_voting_proof::<_, _, Keccak256>(&equivocation_proof));

            // 3rd case (invalid proof): 
            // Equivocation proof with two votes by different authorities.
            let equivocation_proof = generate_double_voting_proof(
                (1, payload1.clone(), set_id, &BeefyKeyring::Alice),
                (1, payload2.clone(), set_id, &BeefyKeyring::Bob),
            );

            // Previous equivocation proof should be invalid.
            assert!(!check_double_voting_proof::<_, _, Keccak256>(&equivocation_proof));

            // 4th case (invalid proof): 
            // Equivocation proof with two votes in different set ids.
            let equivocation_proof = generate_double_voting_proof(
                (1, payload1.clone(), set_id, &BeefyKeyring::Bob),
                (1, payload2.clone(), set_id + 1, &BeefyKeyring::Bob),
            );

            // Previous equivocation proof should be invalid.
            assert!(!check_double_voting_proof::<_, _, Keccak256>(&equivocation_proof));

            // Last case (valid proof):
            // Equivocation proof with two votes in the same round for
            // different payloads signed by the same key.
            let payload2 = Payload::from_single_entry(MMR_ROOT_ID, vec![128]);
            let equivocation_proof = generate_double_voting_proof(
                (1, payload1, set_id, &BeefyKeyring::Bob),
                (1, payload2, set_id, &BeefyKeyring::Bob),
            );

            // Previous equivocation proof should be valid.
            assert!(check_double_voting_proof::<_, _, Keccak256>(&equivocation_proof))
        });
}
