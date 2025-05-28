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

//! Crate containing various data structure used by both xcm-core-buyer pallet
//! as well as the client.

#![cfg_attr(not(feature = "std"), no_std)]

use {
    frame_support::{
        pallet_prelude::{Decode, DecodeWithMemTracking, Encode, TypeInfo},
        CloneNoBound, DebugNoBound,
    },
    sp_runtime::{app_crypto::AppCrypto, RuntimeAppPublic},
    sp_std::vec::Vec,
    tp_traits::ParaId,
};

#[cfg(feature = "std")]
use sp_keystore::{Keystore, KeystorePtr};

/// Proof that I am a collator, assigned to a para_id, and I can buy a core for that para_id
#[derive(
    Encode, Decode, CloneNoBound, PartialEq, Eq, DebugNoBound, TypeInfo, DecodeWithMemTracking,
)]
pub struct BuyCoreCollatorProof<PublicKey>
where
    PublicKey: RuntimeAppPublic + Clone + core::fmt::Debug,
{
    pub nonce: u64,
    pub public_key: PublicKey,
    pub signature: PublicKey::Signature,
}

#[cfg(feature = "std")]
#[derive(Debug)]
pub enum BuyCollatorProofCreationError {
    SignatureDecodingError(parity_scale_codec::Error),
    KeyStoreError(sp_keystore::Error),
}

#[cfg(feature = "std")]
impl From<parity_scale_codec::Error> for BuyCollatorProofCreationError {
    fn from(error: parity_scale_codec::Error) -> Self {
        BuyCollatorProofCreationError::SignatureDecodingError(error)
    }
}

#[cfg(feature = "std")]
impl From<sp_keystore::Error> for BuyCollatorProofCreationError {
    fn from(error: sp_keystore::Error) -> Self {
        BuyCollatorProofCreationError::KeyStoreError(error)
    }
}

impl<PublicKey> BuyCoreCollatorProof<PublicKey>
where
    PublicKey: AppCrypto + RuntimeAppPublic + Clone + core::fmt::Debug,
{
    pub fn prepare_payload(nonce: u64, para_id: ParaId) -> Vec<u8> {
        (nonce, para_id).encode()
    }

    pub fn verify_signature(&self, para_id: ParaId) -> bool {
        let payload = (self.nonce, para_id).encode();
        self.public_key.verify(&payload, &self.signature)
    }

    pub fn new(nonce: u64, para_id: ParaId, public_key: PublicKey) -> Option<Self> {
        let payload = Self::prepare_payload(nonce, para_id);
        public_key
            .sign(&payload)
            .map(|signature| BuyCoreCollatorProof {
                nonce,
                public_key,
                signature,
            })
    }

    #[cfg(feature = "std")]
    pub fn new_with_keystore(
        nonce: u64,
        para_id: ParaId,
        public_key: PublicKey,
        keystore: &KeystorePtr,
    ) -> Result<Option<Self>, BuyCollatorProofCreationError> {
        let payload = Self::prepare_payload(nonce, para_id);

        Ok(Keystore::sign_with(
            keystore,
            <PublicKey as AppCrypto>::ID,
            <PublicKey as AppCrypto>::CRYPTO_ID,
            &public_key.to_raw_vec(),
            payload.as_ref(),
        )?
        .map(|signature| Decode::decode(&mut signature.as_ref()))
        .transpose()?
        .map(|signature| BuyCoreCollatorProof {
            nonce,
            public_key,
            signature,
        }))
    }
}
