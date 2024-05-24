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

#![cfg(feature = "runtime-benchmarks")]

//! Encoded signatures and storage proofs to use in benchmarks, because benchmarks need to run in a
//! no-std environment that doesn't support generating proofs or signatures.

use {
    frame_support::pallet_prelude::{Decode, Encode},
    sp_core::H256,
};

#[derive(Encode, Decode)]
pub struct BenchmarkBlob {
    pub signature_account_u64: cumulus_primitives_core::relay_chain::Signature,
    pub signature_account_32_bytes: cumulus_primitives_core::relay_chain::Signature,
    pub sproof_0: (H256, sp_trie::StorageProof),
    pub sproof_empty: (H256, sp_trie::StorageProof),
}

const ENCODED_BLOB: &[u8] = &[
    0, 9, 177, 241, 214, 77, 65, 110, 230, 156, 74, 146, 155, 254, 39, 188, 91, 117, 252, 8, 90,
    167, 163, 117, 158, 193, 82, 40, 92, 159, 126, 167, 127, 117, 168, 49, 35, 21, 117, 102, 82,
    252, 221, 89, 243, 5, 169, 208, 200, 206, 28, 124, 211, 224, 185, 242, 66, 250, 47, 27, 200,
    31, 48, 141, 6, 0, 118, 30, 162, 34, 163, 78, 91, 130, 87, 18, 148, 219, 145, 6, 214, 15, 236,
    131, 51, 167, 137, 239, 95, 207, 21, 167, 140, 167, 87, 106, 104, 155, 189, 123, 155, 7, 173,
    16, 8, 252, 54, 108, 185, 113, 139, 218, 215, 254, 68, 37, 173, 30, 51, 245, 163, 205, 241,
    202, 134, 131, 97, 41, 182, 15, 144, 103, 146, 229, 52, 207, 5, 82, 255, 36, 219, 55, 41, 38,
    112, 208, 127, 150, 239, 131, 227, 6, 154, 95, 28, 205, 66, 32, 209, 89, 69, 228, 28, 176, 0,
    0, 32, 0, 0, 0, 16, 0, 8, 0, 0, 0, 0, 4, 0, 0, 0, 1, 0, 0, 5, 0, 0, 0, 5, 0, 0, 0, 6, 0, 0, 0,
    6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 196, 47, 140, 97, 41, 216, 22, 207, 81, 195, 116, 188, 127,
    8, 195, 230, 62, 209, 86, 207, 120, 174, 251, 74, 101, 80, 217, 123, 135, 153, 121, 119, 238,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 1, 63, 32, 6, 222, 61, 138, 84, 210, 126,
    68, 169, 213, 206, 24, 150, 24, 242, 45, 180, 180, 157, 149, 50, 13, 144, 33, 153, 76, 133, 15,
    37, 184, 227, 133, 236, 219, 232, 203, 100, 143, 101, 214, 239, 232, 53, 250, 27, 50, 253, 149,
    140, 97, 244, 92, 63, 91, 253, 137, 187, 12, 221, 174, 199, 161, 103, 210, 57, 1, 63, 56, 15,
    186, 152, 104, 158, 190, 209, 19, 135, 53, 224, 231, 165, 167, 144, 171, 205, 113, 11, 48, 189,
    46, 171, 3, 82, 221, 204, 38, 65, 122, 161, 148, 180, 222, 242, 92, 253, 166, 239, 58, 0, 0, 0,
    0, 69, 171, 129, 240, 74, 150, 152, 45, 218, 197, 226, 11, 83, 38, 121, 204, 15, 12, 197, 115,
    153, 57, 69, 212, 166, 142, 68, 212, 239, 160, 6, 119, 200, 95, 12, 230, 120, 121, 157, 62,
    255, 2, 66, 83, 185, 14, 132, 146, 124, 198, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 1, 128, 11, 0, 128, 21, 69, 87, 39, 80,
    189, 63, 244, 210, 244, 71, 86, 224, 172, 98, 5, 171, 30, 40, 194, 211, 222, 117, 47, 81, 150,
    51, 191, 236, 129, 237, 184, 128, 223, 91, 79, 3, 86, 79, 50, 130, 149, 40, 193, 33, 8, 158,
    35, 82, 137, 134, 195, 10, 60, 248, 193, 39, 236, 182, 225, 143, 110, 206, 62, 80, 128, 77,
    174, 114, 63, 107, 0, 39, 241, 147, 41, 150, 41, 130, 189, 148, 216, 5, 24, 47, 167, 29, 117,
    8, 120, 210, 51, 35, 145, 27, 94, 189, 250, 169, 1, 159, 12, 182, 243, 110, 2, 122, 187, 32,
    145, 207, 181, 17, 10, 181, 8, 127, 137, 0, 104, 95, 6, 21, 91, 60, 217, 168, 201, 229, 233,
    162, 63, 213, 220, 19, 165, 237, 32, 0, 0, 0, 0, 0, 0, 0, 0, 104, 95, 8, 49, 108, 191, 143,
    160, 218, 130, 42, 32, 172, 28, 85, 191, 27, 227, 32, 0, 0, 0, 0, 0, 0, 0, 0, 128, 254, 108,
    203, 37, 75, 132, 240, 210, 32, 46, 181, 5, 189, 223, 159, 84, 203, 158, 189, 15, 178, 113,
    144, 114, 233, 46, 229, 124, 29, 161, 216, 9, 190, 247, 74, 173, 54, 61, 117, 185, 65, 198,
    150, 80, 9, 116, 166, 1, 185, 33, 23, 38, 14, 102, 24, 248, 23, 224, 15, 137, 66, 208, 101,
    122, 20, 176, 0, 0, 32, 0, 0, 0, 16, 0, 8, 0, 0, 0, 0, 4, 0, 0, 0, 1, 0, 0, 5, 0, 0, 0, 5, 0,
    0, 0, 6, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 1, 63, 32, 6, 222, 61, 138, 84, 210,
    126, 68, 169, 213, 206, 24, 150, 24, 242, 45, 180, 180, 157, 149, 50, 13, 144, 33, 153, 76,
    133, 15, 37, 184, 227, 133, 236, 219, 232, 203, 100, 143, 101, 214, 239, 232, 53, 250, 27, 50,
    253, 149, 140, 97, 244, 92, 63, 91, 253, 137, 187, 12, 221, 174, 199, 161, 103, 210, 200, 95,
    12, 230, 120, 121, 157, 62, 255, 2, 66, 83, 185, 14, 132, 146, 124, 198, 128, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 1, 128, 3, 0,
    128, 21, 69, 87, 39, 80, 189, 63, 244, 210, 244, 71, 86, 224, 172, 98, 5, 171, 30, 40, 194,
    211, 222, 117, 47, 81, 150, 51, 191, 236, 129, 237, 184, 128, 223, 91, 79, 3, 86, 79, 50, 130,
    149, 40, 193, 33, 8, 158, 35, 82, 137, 134, 195, 10, 60, 248, 193, 39, 236, 182, 225, 143, 110,
    206, 62, 80, 169, 1, 159, 12, 182, 243, 110, 2, 122, 187, 32, 145, 207, 181, 17, 10, 181, 8,
    127, 137, 0, 104, 95, 6, 21, 91, 60, 217, 168, 201, 229, 233, 162, 63, 213, 220, 19, 165, 237,
    32, 0, 0, 0, 0, 0, 0, 0, 0, 104, 95, 8, 49, 108, 191, 143, 160, 218, 130, 42, 32, 172, 28, 85,
    191, 27, 227, 32, 0, 0, 0, 0, 0, 0, 0, 0, 128, 254, 108, 203, 37, 75, 132, 240, 210, 32, 46,
    181, 5, 189, 223, 159, 84, 203, 158, 189, 15, 178, 113, 144, 114, 233, 46, 229, 124, 29, 161,
    216, 9,
];

pub fn benchmark_blob() -> BenchmarkBlob {
    #[allow(const_item_mutation)]
    BenchmarkBlob::decode(&mut ENCODED_BLOB).unwrap()
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            mock::{get_ed25519_pairs, new_test_ext, run_to_block, ParaRegistrar, ALICE},
            ParaInfo, REGISTRAR_PARAS_INDEX,
        },
        cumulus_test_relay_sproof_builder::RelayStateSproofBuilder,
        frame_support::Hashable,
        parity_scale_codec::Encode,
        sp_core::{crypto::AccountId32, Decode, Pair},
        tp_traits::ParaId,
    };

    #[test]
    #[ignore = "used to generate benchmark data"]
    fn benchmark_blob_generate_data() {
        new_test_ext().execute_with(|| {
            run_to_block(1);

            let pairs = get_ed25519_pairs(1);
            let mut sproof = RelayStateSproofBuilder::default();
            let para_id: ParaId = 0.into();
            let bytes = para_id.twox_64_concat();
            let key = [REGISTRAR_PARAS_INDEX, bytes.as_slice()].concat();
            let para_info: ParaInfo<
                cumulus_primitives_core::relay_chain::AccountId,
                cumulus_primitives_core::relay_chain::Balance,
            > = ParaInfo {
                manager: pairs[0].public().into(),
                deposit: Default::default(),
                locked: None,
            };
            sproof.additional_key_values = vec![(key, para_info.encode())];
            let (relay_parent_storage_root, proof) = sproof.into_state_root_and_proof();

            let account = ALICE;
            let relay_storage_root = relay_parent_storage_root;
            let signature_msg =
                ParaRegistrar::relay_signature_msg(para_id, &account, relay_storage_root);
            let signature_account_u64: cumulus_primitives_core::relay_chain::Signature =
                pairs[0].sign(&signature_msg).into();

            // Account generated by
            // create_funded_user::<T>("caller", 0, T::DepositAmount::get());
            let account32 = AccountId32::decode(
                &mut [
                    92u8, 119, 3, 63, 206, 138, 144, 69, 130, 74, 102, 144, 187, 249, 156, 109,
                    178, 105, 80, 47, 10, 141, 29, 42, 0, 133, 66, 213, 105, 10, 7, 73,
                ]
                .as_slice(),
            )
            .unwrap();
            let signature_msg = (para_id, &account32, relay_storage_root).encode();
            let signature_account_32_bytes: cumulus_primitives_core::relay_chain::Signature =
                pairs[0].sign(&signature_msg).into();

            let (empty_relay_parent_storage_root, empty_proof) =
                RelayStateSproofBuilder::default().into_state_root_and_proof();

            let blob = BenchmarkBlob {
                signature_account_u64,
                signature_account_32_bytes,
                sproof_0: (relay_parent_storage_root, proof),
                sproof_empty: (empty_relay_parent_storage_root, empty_proof),
            };

            panic!("{:?}", blob.encode());
        });
    }
}
