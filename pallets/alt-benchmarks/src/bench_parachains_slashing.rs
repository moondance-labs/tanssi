// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

use alloc::boxed::Box;
use alloc::vec;
use core::marker::PhantomData;
use frame_benchmarking::{account, v2::*};
use frame_support::traits::{KeyOwnerProofSystem, OnFinalize, OnInitialize, ValidatorSet};
use frame_system::{pallet_prelude::BlockNumberFor, RawOrigin};
use parity_scale_codec::Decode;
use polkadot_primitives::{
    slashing::{DisputeProof, DisputesTimeSlot, SlashingOffenceKind},
    CandidateHash, Hash, SessionIndex, ValidatorId, ValidatorIndex, PARACHAIN_KEY_TYPE_ID,
};
use polkadot_runtime_parachains::{disputes::SlashingHandler, initializer};
use sp_runtime::traits::{One, OpaqueKeys};
use sp_runtime::Vec;
use sp_session::MembershipProof;

// Candidate hash of the disputed candidate.
const CANDIDATE_HASH: CandidateHash = CandidateHash(Hash::zero());

const MAX_VALIDATORS: u32 = 100;

pub struct Pallet<T: Config>(PhantomData<T>);
pub trait Config:
    pallet_session::Config
    + pallet_session::historical::Config
    + polkadot_runtime_parachains::disputes::Config
    + polkadot_runtime_parachains::disputes::slashing::Config
    + polkadot_runtime_parachains::shared::Config
    + polkadot_runtime_parachains::initializer::Config
{
    type Validators: Validators<Self::AccountId>;
}

pub trait Validators<AccountId> {
    /// Sets the validators to properly run a benchmark. Should take care of everything that
    /// will make pallet_session use those validators, such as them having a balance.
    fn set_validators(validators: &[AccountId]);
}

fn setup_validator_set<T>(n: u32) -> (SessionIndex, MembershipProof, ValidatorId)
where
    T: Config,
{
    let validators: Vec<T::AccountId> = (0..n).map(|i| account("account id", i, 0)).collect();
    T::Validators::set_validators(&validators);

    // create validators and set random session keys
    for (n, who) in validators.into_iter().enumerate() {
        use rand::{RngCore, SeedableRng};

        let keys = {
            const SESSION_KEY_LEN: usize = 32;
            let key_ids = T::Keys::key_ids();
            let mut keys_len = key_ids.len() * SESSION_KEY_LEN;
            if key_ids.contains(&sp_core::crypto::key_types::BEEFY) {
                // BEEFY key is 33 bytes long, not 32.
                keys_len += 1;
            }
            let mut keys = vec![0u8; keys_len];
            let mut rng = rand_chacha::ChaCha12Rng::seed_from_u64(n as u64);
            rng.fill_bytes(&mut keys);
            keys
        };

        let keys: T::Keys = Decode::decode(&mut &keys[..]).expect("wrong number of session keys?");
        let proof: Vec<u8> = vec![];

        pallet_session::Pallet::<T>::set_keys(RawOrigin::Signed(who).into(), keys, proof)
            .expect("session::set_keys should work");
    }

    pallet_session::Pallet::<T>::on_initialize(BlockNumberFor::<T>::one());
    initializer::Pallet::<T>::on_initialize(BlockNumberFor::<T>::one());

    // skip sessions until the new validator set is enacted
    let mut tries = 0;
    while pallet_session::Pallet::<T>::validators().len() < n as usize {
        tries += 1;
        if tries > 100 {
            panic!("failed to enact new validators (n={n}) after rotating many sessions");
        }

        pallet_session::Pallet::<T>::rotate_session();
    }
    initializer::Pallet::<T>::on_finalize(BlockNumberFor::<T>::one());

    let session_index = polkadot_runtime_parachains::shared::CurrentSessionIndex::<T>::get();
    let session_info = polkadot_runtime_parachains::session_info::Sessions::<T>::get(session_index);
    let session_info = session_info.unwrap();
    let validator_id = session_info
        .validators
        .get(ValidatorIndex::from(0))
        .unwrap()
        .clone();
    let key = (PARACHAIN_KEY_TYPE_ID, validator_id.clone());
    let key_owner_proof = pallet_session::historical::Pallet::<T>::prove(key).unwrap();

    // rotate a session to make sure `key_owner_proof` is historical
    initializer::Pallet::<T>::on_initialize(BlockNumberFor::<T>::one());
    pallet_session::Pallet::<T>::rotate_session();
    initializer::Pallet::<T>::on_finalize(BlockNumberFor::<T>::one());

    let idx = polkadot_runtime_parachains::shared::CurrentSessionIndex::<T>::get();
    assert!(
        idx > session_index,
        "session rotation should work for parachain pallets: {} <= {}",
        idx,
        session_index,
    );

    (session_index, key_owner_proof, validator_id)
}

/// Submits a single `ForInvalid` dispute.
fn setup_dispute<T>(session_index: SessionIndex, validator_id: ValidatorId) -> DisputeProof
where
    T: Config,
{
    let current_session = T::ValidatorSet::session_index();
    assert_ne!(session_index, current_session);

    let validator_index = ValidatorIndex(0);
    let losers = [validator_index].into_iter();
    let backers = losers.clone();

    T::SlashingHandler::punish_for_invalid(session_index, CANDIDATE_HASH, losers, backers);

    let unapplied = <UnappliedSlashes<T>>::get(session_index, CANDIDATE_HASH);
    assert_eq!(unapplied.unwrap().keys.len(), 1);

    dispute_proof(session_index, validator_id, validator_index)
}

/// Creates a `ForInvalid` dispute proof.
fn dispute_proof(
    session_index: SessionIndex,
    validator_id: ValidatorId,
    validator_index: ValidatorIndex,
) -> DisputeProof {
    let kind = SlashingOffenceKind::ForInvalid;
    let time_slot = DisputesTimeSlot::new(session_index, CANDIDATE_HASH);

    DisputeProof {
        time_slot,
        kind,
        validator_index,
        validator_id,
    }
}

#[allow(clippy::multiple_bound_locations)]
#[benchmarks(
    where T: Config<KeyOwnerProof = MembershipProof>
)]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn report_dispute_lost(n: Linear<4, MAX_VALIDATORS>) -> Result<(), BenchmarkError> {
        let (session_index, key_owner_proof, validator_id) = setup_validator_set::<T>(n);
        let dispute_proof = setup_dispute::<T>(session_index, validator_id);

        let result;
        #[block]
        {
            result = polkadot_runtime_parachains::disputes::slashing::Pallet::<T>::report_dispute_lost_unsigned(
                RawOrigin::None.into(),
                Box::new(dispute_proof),
                key_owner_proof,
            );
        }

        assert!(result.is_ok());

        Ok(())
    }
}
