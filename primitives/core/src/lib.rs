// Copyright 2019-2022 Moondance Labs Ltd.
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

#![cfg_attr(not(feature = "std"), no_std)]

/// A declarations of storage keys where an external observer can find some interesting data.
pub mod well_known_keys {

    use {
        cumulus_primitives_core::ParaId, sp_core::Encode, sp_io::hashing::twox_64, sp_std::vec::Vec,
    };

    // They key to retrieve the para heads
    pub const PARAS_HEADS_INDEX: &[u8] =
        &hex_literal::hex!["cd710b30bd2eab0352ddcc26417aa1941b3c252fcb29d88eff4f3de5de4476c3"];

    // Retrieves the full key representing the para->heads and the paraId
    pub fn para_id_head(para_id: ParaId) -> Vec<u8> {
        para_id.using_encoded(|para_id: &[u8]| {
            PARAS_HEADS_INDEX
                .iter()
                .chain(twox_64(para_id).iter())
                .chain(para_id.iter())
                .cloned()
                .collect()
        })
    }

    pub const COLLATOR_ASSIGNMENT_INDEX: &[u8] =
        &hex_literal::hex!["4a97b7c32fd2bcd103026654b3408079170f16afec7d161bc6acec3964492a0c"];
}
