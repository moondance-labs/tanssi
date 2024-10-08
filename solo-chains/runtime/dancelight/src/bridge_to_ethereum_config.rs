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

//! The bridge to ethereum config

pub const SLOTS_PER_EPOCH: u32 = snowbridge_pallet_ethereum_client::config::SLOTS_PER_EPOCH as u32;
use crate::{parameter_types, ConstU32, Runtime, RuntimeEvent};
use snowbridge_beacon_primitives::{Fork, ForkVersions};

parameter_types! {
    pub const ChainForkVersions: ForkVersions = ForkVersions {
        genesis: Fork {
            version: [144, 0, 0, 111], // 0x90000069
            epoch: 0,
        },
        altair: Fork {
            version: [144, 0, 0, 112], // 0x90000070
            epoch: 50,
        },
        bellatrix: Fork {
            version: [144, 0, 0, 113], // 0x90000071
            epoch: 100,
        },
        capella: Fork {
            version: [144, 0, 0, 114], // 0x90000072
            epoch: 56832,
        },
        deneb: Fork {
            version: [144, 0, 0, 115], // 0x90000073
            epoch: 132608,
        },
    };
}

impl snowbridge_pallet_ethereum_client::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ForkVersions = ChainForkVersions;
    type WeightInfo = ();
}
