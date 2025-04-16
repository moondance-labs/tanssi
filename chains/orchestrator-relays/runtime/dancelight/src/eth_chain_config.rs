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

use snowbridge_beacon_primitives::{Fork, ForkVersions};

#[cfg(feature = "runtime-benchmarks")]
pub const ELECTRA_TEST_FORK_EPOCH: u64 = 80000000000;
#[cfg(not(feature = "runtime-benchmarks"))]
pub const ELECTRA_TEST_FORK_EPOCH: u64 = 0;

/// Fork versions for different build environments.
pub const fn fork_versions() -> ForkVersions {
    #[cfg(any(
        feature = "std",
        feature = "fast-runtime",
        feature = "runtime-benchmarks",
        test
    ))]
    {
        ForkVersions {
            genesis: Fork {
                version: [0, 0, 0, 0],
                epoch: 0,
            },
            altair: Fork {
                version: [1, 0, 0, 0],
                epoch: 0,
            },
            bellatrix: Fork {
                version: [2, 0, 0, 0],
                epoch: 0,
            },
            capella: Fork {
                version: [3, 0, 0, 0],
                epoch: 0,
            },
            deneb: Fork {
                version: [4, 0, 0, 0],
                epoch: 0,
            },
            electra: Fork {
                version: [5, 0, 0, 0],
                epoch: ELECTRA_TEST_FORK_EPOCH,
            },
        }
    }

    #[cfg(not(any(
        feature = "std",
        feature = "fast-runtime",
        feature = "runtime-benchmarks",
        test
    )))]
    {
        use hex_literal::hex;
        ForkVersions {
            genesis: Fork {
                version: hex!("90000069"),
                epoch: 0,
            },
            altair: Fork {
                version: hex!("90000070"),
                epoch: 50,
            },
            bellatrix: Fork {
                version: hex!("90000071"),
                epoch: 100,
            },
            capella: Fork {
                version: hex!("90000072"),
                epoch: 56832,
            },
            deneb: Fork {
                version: hex!("90000073"),
                epoch: 132608,
            },
            electra: Fork {
                version: hex!("90000074"),
                epoch: 222464,
            },
        }
    }
}
