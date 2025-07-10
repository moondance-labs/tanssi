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

//! Generate mocked values of relay chain storage, for dev tests

use {
    dp_core::{well_known_keys::REGISTRAR_PARAS_INDEX, ParaInfo},
    frame_support::Hashable,
    sp_core::{ed25519, Pair},
    tc_consensus::{Encode, ParaId},
};

pub fn get_mocked_registrar_paras(para_id: ParaId) -> (Vec<u8>, Vec<u8>) {
    let bytes = para_id.twox_64_concat();

    let pairs = get_ed25519_pairs(1);
    //panic!("relay manager private key: {:?}", pairs[0].seed());
    let registrar_paras_key = [REGISTRAR_PARAS_INDEX, bytes.as_slice()].concat();
    let para_info: ParaInfo<
        cumulus_primitives_core::relay_chain::AccountId,
        cumulus_primitives_core::relay_chain::Balance,
    > = ParaInfo {
        manager: pairs[0].public().into(),
        deposit: Default::default(),
        locked: None,
    };

    (registrar_paras_key, para_info.encode())
}

pub fn get_ed25519_pairs(num: u32) -> Vec<ed25519::Pair> {
    let seed: u128 = 12345678901234567890123456789012;
    let mut pairs = Vec::new();
    for i in 0..num {
        pairs.push(ed25519::Pair::from_seed(
            (seed + u128::from(i))
                .to_string()
                .as_bytes()
                .try_into()
                .unwrap(),
        ))
    }
    pairs
}
