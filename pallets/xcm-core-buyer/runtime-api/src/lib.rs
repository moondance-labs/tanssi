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

//! Runtime API for XCM core buyer pallet

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet_xcm_core_buyer::BuyingError;
use {
    frame_support::sp_runtime,
    sp_consensus_slots::Slot,
    sp_runtime::{traits::Block as BlockT, RuntimeAppPublic},
    sp_std::boxed::Box,
    tp_xcm_core_buyer::BuyCoreCollatorProof,
};

sp_api::decl_runtime_apis! {
    pub trait XCMCoreBuyerApi<BlockNumber, ParaId, CollatorPublicKey> where ParaId: parity_scale_codec::Codec, BlockNumber: parity_scale_codec::Codec, BuyingError<BlockNumber>: parity_scale_codec::Codec, CollatorPublicKey: RuntimeAppPublic + Clone + core::fmt::Debug + parity_scale_codec::Codec,  {
        fn is_core_buying_allowed(para_id: ParaId, collator_public_key: CollatorPublicKey) -> Result<(), BuyingError<BlockNumber>>;
        fn create_buy_core_unsigned_extrinsic(para_id: ParaId, proof: BuyCoreCollatorProof<CollatorPublicKey>) -> Box<<Block as BlockT>::Extrinsic>;
        fn get_buy_core_signature_nonce(para_id: ParaId) -> u64;
        fn get_buy_core_slot_drift() -> Slot;
    }
}
