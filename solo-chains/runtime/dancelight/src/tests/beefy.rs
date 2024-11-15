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
    crate::{tests::common::*, Beefy, Historical},
    beefy_primitives::{
        check_double_voting_proof,
        ecdsa_crypto::{
            AuthorityId as BeefyId, Public as BeefyPublic, Signature as BeefySignature,
        },
        known_payloads::MMR_ROOT_ID,
        test_utils::{generate_double_voting_proof, BeefySignerAuthority, Keyring as BeefyKeyring},
        BeefySignatureHasher, Commitment, ConsensusLog, FutureBlockVotingProof, Payload,
        ValidatorSet, ValidatorSetId as ValidatorSetIdType, VoteMessage, BEEFY_ENGINE_ID,
        KEY_TYPE as BEEFY_KEY_TYPE,
    },
    frame_support::{assert_err, assert_ok, traits::KeyOwnerProofSystem},
    pallet_beefy::{Error as BeefyError, GenesisBlock, ValidatorSetId},
    parity_scale_codec::{Decode, Encode},
    sp_application_crypto::{AppCrypto, Pair, RuntimeAppPublic},
    sp_runtime::{traits::Keccak256, DigestItem},
    sp_std::vec,
};

/// Create a new `VoteMessage` from commitment primitives and key pair.
pub fn signed_vote<TPublic: AppCrypto + RuntimeAppPublic<Signature = BeefySignature>>(
    block_number: u32,
    payload: Payload,
    validator_set_id: ValidatorSetIdType,
    key_pair: <TPublic as AppCrypto>::Pair,
) -> VoteMessage<u32, <<TPublic as AppCrypto>::Pair as AppCrypto>::Public, BeefySignature>
where
    <TPublic as AppCrypto>::Pair: BeefySignerAuthority<BeefySignatureHasher>,
    <TPublic as RuntimeAppPublic>::Signature:
        Send + Sync + From<<<TPublic as AppCrypto>::Pair as AppCrypto>::Signature>,
{
    let commitment = Commitment {
        validator_set_id,
        block_number,
        payload,
    };
    let signature: <TPublic as RuntimeAppPublic>::Signature =
        key_pair.sign_with_hasher(&commitment.encode()).into();
    VoteMessage {
        commitment,
        id: key_pair.public(),
        signature,
    }
}

/// Create a new `FutureBlockVotingProof` based on vote.
pub fn generate_future_block_voting_proof(
    vote: (u32, Payload, ValidatorSetIdType),
    key_pair: <BeefyId as AppCrypto>::Pair,
) -> FutureBlockVotingProof<u32, BeefyPublic> {
    let signed_vote = signed_vote::<BeefyId>(vote.0, vote.1, vote.2, key_pair);
    FutureBlockVotingProof { vote: signed_vote }
}

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
            assert!(!check_double_voting_proof::<_, _, Keccak256>(
                &equivocation_proof
            ));

            // 2nd case (invalid proof):
            // Equivocation proof with two votes in different rounds for
            // different payloads signed by the same key.
            let equivocation_proof = generate_double_voting_proof(
                (1, payload1.clone(), set_id, &BeefyKeyring::Bob),
                (2, payload2.clone(), set_id, &BeefyKeyring::Bob),
            );

            // Previous equivocation proof should be invalid.
            assert!(!check_double_voting_proof::<_, _, Keccak256>(
                &equivocation_proof
            ));

            // 3rd case (invalid proof):
            // Equivocation proof with two votes by different authorities.
            let equivocation_proof = generate_double_voting_proof(
                (1, payload1.clone(), set_id, &BeefyKeyring::Alice),
                (1, payload2.clone(), set_id, &BeefyKeyring::Bob),
            );

            // Previous equivocation proof should be invalid.
            assert!(!check_double_voting_proof::<_, _, Keccak256>(
                &equivocation_proof
            ));

            // 4th case (invalid proof):
            // Equivocation proof with two votes in different set ids.
            let equivocation_proof = generate_double_voting_proof(
                (1, payload1.clone(), set_id, &BeefyKeyring::Bob),
                (1, payload2.clone(), set_id + 1, &BeefyKeyring::Bob),
            );

            // Previous equivocation proof should be invalid.
            assert!(!check_double_voting_proof::<_, _, Keccak256>(
                &equivocation_proof
            ));

            // Last case (valid proof):
            // Equivocation proof with two votes in the same round for
            // different payloads signed by the same key.
            let payload2 = Payload::from_single_entry(MMR_ROOT_ID, vec![128]);
            let equivocation_proof = generate_double_voting_proof(
                (1, payload1, set_id, &BeefyKeyring::Bob),
                (1, payload2, set_id, &BeefyKeyring::Bob),
            );

            // Previous equivocation proof should be valid.
            assert!(check_double_voting_proof::<_, _, Keccak256>(
                &equivocation_proof
            ))
        });
}

#[test]
fn test_set_new_genesis() {
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
            run_to_session(1);

            let new_beefy_genesis_delay = 5u32;
            assert_ok!(Beefy::set_new_genesis(
                root_origin(),
                new_beefy_genesis_delay,
            ));

            let expected_new_genesis = System::block_number() + new_beefy_genesis_delay;

            // Check the new genesis was placed correctly.
            assert_eq!(GenesisBlock::<Runtime>::get(), Some(expected_new_genesis));

            // We should not be able to set a genesis < 1
            assert_err!(
                Beefy::set_new_genesis(root_origin(), 0u32,),
                BeefyError::<Runtime>::InvalidConfiguration,
            );
        });
}

#[test]
fn test_report_future_voting_valid_and_invalid_proofs() {
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
            run_to_session(1);

            let block_num = System::block_number();
            let validator_set = Beefy::validator_set().unwrap();
            let authorities = validator_set.validators();
            let set_id = validator_set.id();

            assert_eq!(authorities.len(), 4);
            let equivocation_authority_index = 1;
            let equivocation_key = &authorities[equivocation_authority_index];

            // Create key ownership proof
            let key_owner_proof = Historical::prove((BEEFY_KEY_TYPE, &equivocation_key)).unwrap();

            // Let's generate a key_pair to proof that BOB corresponds to index 1.
            let secret_uri = format!("//{}", &AccountId::from(BOB));
            let key_pair_bob = <BeefyId as AppCrypto>::Pair::from_string(&secret_uri, None)
                .expect("should succeed generating key_pair");

            let payload = Payload::from_single_entry(MMR_ROOT_ID, vec![42]);

            // Build the future block equivocation proof using the generated key_pair.
            let valid_equivocation_proof = generate_future_block_voting_proof(
                (block_num + 100, payload.clone(), set_id),
                key_pair_bob.clone(),
            );

            // Should succeed as BOB is present in validator set.
            assert_ok!(Beefy::report_future_block_voting_unsigned(
                RuntimeOrigin::none(),
                Box::new(valid_equivocation_proof),
                key_owner_proof.clone(),
            ));

            // Let's generate a key_pair of an account that is not present in validator set.
            let secret_uri = format!("//{}", &AccountId::from(FERDIE));
            let key_pair_ferdie = <BeefyId as AppCrypto>::Pair::from_string(&secret_uri, None)
                .expect("should succeed generating invalid key_pair");

            // Build the invalid equivocation proof.
            let invalid_equivocation_proof = generate_future_block_voting_proof(
                (block_num + 100, payload.clone(), set_id),
                key_pair_ferdie,
            );

            // Should fail as FERDIE is not part of the validator set.
            assert_err!(
                Beefy::report_future_block_voting_unsigned(
                    RuntimeOrigin::none(),
                    Box::new(invalid_equivocation_proof),
                    key_owner_proof.clone(),
                ),
                BeefyError::<Runtime>::InvalidKeyOwnershipProof
            );

            let invalid_equivocation_proof =
                generate_future_block_voting_proof((1, payload.clone(), set_id), key_pair_bob);

            // Should fail if the proof targets an old block.
            assert_err!(
                Beefy::report_future_block_voting_unsigned(
                    RuntimeOrigin::none(),
                    Box::new(invalid_equivocation_proof),
                    key_owner_proof,
                ),
                BeefyError::<Runtime>::InvalidFutureBlockVotingProof
            );
        });
}

#[test]
fn test_mmr_digest_updates_after_session_and_single_block() {
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

            let expected_authorities_digest = get_beefy_digest(ConsensusLog::AuthoritiesChange(
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

            // Check that both authorities and MMR digests were correctly placed after session change.
            let actual_authorities_digest = System::digest().logs[2].clone();
            let first_mmr_digest = System::digest().logs[3].clone();

            let first_mmr_digest = match first_mmr_digest.clone() {
                DigestItem::Consensus(id, val) => {
                    if id == BEEFY_ENGINE_ID {
                        match ConsensusLog::<BeefyId>::decode(&mut &val[..]) {
                            Ok(result) => match result {
                                ConsensusLog::AuthoritiesChange(_) => None,
                                ConsensusLog::MmrRoot(m) => Some(m),
                                ConsensusLog::OnDisabled(_) => None,
                            },
                            Err(_) => None,
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            };

            assert!(first_mmr_digest.is_some());
            assert_eq!(expected_authorities_digest, actual_authorities_digest);

            // After running a single block the MMR digest should update again.
            run_block();
            let second_mmr_digest = System::digest().logs[1].clone();

            let second_mmr_digest = match second_mmr_digest.clone() {
                DigestItem::Consensus(id, val) => {
                    if id == BEEFY_ENGINE_ID {
                        match ConsensusLog::<BeefyId>::decode(&mut &val[..]) {
                            Ok(result) => match result {
                                ConsensusLog::AuthoritiesChange(_) => None,
                                ConsensusLog::MmrRoot(m) => Some(m),
                                ConsensusLog::OnDisabled(_) => None,
                            },
                            Err(_) => None,
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            };
            assert!(second_mmr_digest.is_some());
            assert!(first_mmr_digest.unwrap() != second_mmr_digest.unwrap());
        });
}
