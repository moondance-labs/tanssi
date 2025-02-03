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

use super::*;
use hex_literal::hex;

#[test]
fn test_command_encoding() {
    let command = Command::Test(b"Hello, world!".to_vec());

    let expected = hex!(
        "0000000000000000000000000000000000000000000000000000000000000020" // tuple offset
        "0000000000000000000000000000000000000000000000000000000000000020" // bytes offset in tuple
        "000000000000000000000000000000000000000000000000000000000000000d" // bytes size
        "48656C6C6F2C20776F726C642100000000000000000000000000000000000000" // bytes
    );

    assert_eq!(command.abi_encode(), expected);
}

#[test]
fn test_report_rewards_encoding() {
    let command = Command::ReportRewards {
        external_idx: 123_456_789,
        era_index: 42,
        total_points: 123_456_789_012_345,
        tokens_inflated: 987_654_321_098,
        rewards_merkle_root: H256::from(hex!(
            "b6e16d27ac5ab427a7f68900ac5559ce272dc6c37c82b3e052246c82244c50e4"
        )),
        token_id: H256::repeat_byte(0x01),
    };

    let expected = hex!(
        // no tuple offset since all fields have static size
        "00000000000000000000000000000000000000000000000000000000075BCD15" // timestamp
        "000000000000000000000000000000000000000000000000000000000000002A" // era index
        "00000000000000000000000000000000000000000000000000007048860DDF79" // total points
        "000000000000000000000000000000000000000000000000000000E5F4C8F3CA" // total inflated
        "b6e16d27ac5ab427a7f68900ac5559ce272dc6c37c82b3e052246c82244c50e4" // root
        "0101010101010101010101010101010101010101010101010101010101010101" // token_id
    );

    assert_eq!(command.abi_encode(), expected);
}

#[test]
fn test_report_slashes_encoding() {
    pub const ALICE: [u8; 32] = [4u8; 32];
    pub const BOB: [u8; 32] = [5u8; 32];
    pub const CHARLIE: [u8; 32] = [6u8; 32];
    let command = Command::ReportSlashes {
        era_index: 42,
        slashes: vec![
            SlashData {
                encoded_validator_id: sp_runtime::AccountId32::from(ALICE).encode(),
                slash_fraction: 5_000u32,
                external_idx: 500u64,
            },
            SlashData {
                encoded_validator_id: sp_runtime::AccountId32::from(BOB).encode(),
                slash_fraction: 4_000u32,
                external_idx: 400u64,
            },
            SlashData {
                encoded_validator_id: sp_runtime::AccountId32::from(CHARLIE).encode(),
                slash_fraction: 3_000u32,
                external_idx: 300u64,
            },
        ],
    };

    let expected = hex!(
        "0000000000000000000000000000000000000000000000000000000000000020" // offset of era_index
        "000000000000000000000000000000000000000000000000000000000000002A" // era index
        "0000000000000000000000000000000000000000000000000000000000000040" // offset of slashes
        "0000000000000000000000000000000000000000000000000000000000000003" // length of slashes
        "0404040404040404040404040404040404040404040404040404040404040404" // ALICE
        "0000000000000000000000000000000000000000000000000000000000001388" // 5_000u32
        "00000000000000000000000000000000000000000000000000000000000001F4" // 500u64
        "0505050505050505050505050505050505050505050505050505050505050505" // BOB
        "0000000000000000000000000000000000000000000000000000000000000FA0" // 4_000u32
        "0000000000000000000000000000000000000000000000000000000000000190" // 400u64
        "0606060606060606060606060606060606060606060606060606060606060606" // CHARLIE
        "0000000000000000000000000000000000000000000000000000000000000BB8" // 3_000u32
        "000000000000000000000000000000000000000000000000000000000000012C"  // 300u64
    );

    assert_eq!(command.abi_encode(), expected);
}
