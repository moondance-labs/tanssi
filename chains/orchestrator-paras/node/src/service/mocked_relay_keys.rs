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
    frame_support::Hashable,
    sp_core::{ed25519, Pair},
    tc_consensus::{Decode, Encode, ParaId},
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

// TODO: import this from dancekit
pub const REGISTRAR_PARAS_INDEX: &[u8] =
    &hex_literal::hex!["3fba98689ebed1138735e0e7a5a790abcd710b30bd2eab0352ddcc26417aa194"];

// Need to copy ParaInfo from
// polkadot-sdk/polkadot/runtime/common/src/paras_registrar/mod.rs
// Because its fields are not public...
// TODO: import this from dancekit
#[derive(Encode, Decode, Clone, PartialEq, Eq, Default)]
pub struct ParaInfo<Account, Balance> {
    /// The account that has placed a deposit for registering this para.
    manager: Account,
    /// The amount reserved by the `manager` account for the registration.
    deposit: Balance,
    /// Whether the para registration should be locked from being controlled by the manager.
    /// None means the lock had not been explicitly set, and should be treated as false.
    locked: Option<bool>,
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
