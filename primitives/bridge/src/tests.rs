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
        timestamp: 123_456_789,
        era_index: 42,
        total_points: 123_456_789_012_345,
        tokens_inflated: 987_654_321_098,
        rewards_merkle_root: H256::from(hex!(
            "b6e16d27ac5ab427a7f68900ac5559ce272dc6c37c82b3e052246c82244c50e4"
        )),
    };

    let expected = hex!(
        // no tuple offset since all fields have static size
        "00000000000000000000000000000000000000000000000000000000075BCD15" // timestamp
        "000000000000000000000000000000000000000000000000000000000000002A" // era index
        "00000000000000000000000000000000000000000000000000007048860DDF79" // total points
        "000000000000000000000000000000000000000000000000000000E5F4C8F3CA" // total inflated
        "b6e16d27ac5ab427a7f68900ac5559ce272dc6c37c82b3e052246c82244c50e4" // root
    );

    assert_eq!(command.abi_encode(), expected);
}
