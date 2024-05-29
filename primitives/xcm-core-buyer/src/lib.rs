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

use frame_support::pallet_prelude::{Decode, Encode, TypeInfo};
use frame_support::{CloneNoBound, DebugNoBound};
use sp_runtime::RuntimeAppPublic;
use tp_traits::ParaId;

/// Proof that I am a collator, assigned to a para_id, and I can buy a core for that para_id
#[derive(Encode, Decode, CloneNoBound, PartialEq, Eq, DebugNoBound, TypeInfo)]
pub struct BuyCoreCollatorProof<PublicKey>
where
    PublicKey: RuntimeAppPublic + Clone + core::fmt::Debug,
{
    pub nonce: u64,
    pub public_key: PublicKey,
    pub signature: PublicKey::Signature,
}

impl<PublicKey> BuyCoreCollatorProof<PublicKey>
where
    PublicKey: RuntimeAppPublic + Clone + core::fmt::Debug,
{
    pub fn verify_signature(&self, para_id: ParaId) -> bool {
        let payload = (self.nonce, para_id).encode();
        self.public_key.verify(&payload, &self.signature)
    }

    pub fn new(nonce: u64, para_id: ParaId, public_key: PublicKey) -> Option<Self> {
        let payload = (nonce, para_id).encode();
        public_key
            .sign(&payload)
            .map(|signature| BuyCoreCollatorProof {
                nonce,
                public_key,
                signature,
            })
    }
}
