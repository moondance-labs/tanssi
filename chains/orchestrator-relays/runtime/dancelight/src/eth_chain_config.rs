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

use hex_literal::hex;
use snowbridge_beacon_primitives::{Fork, ForkVersions};

enum BuildEnv {
    Prod,
    Benchmark,
    TestLike,
}

const fn current_env() -> BuildEnv {
    if cfg!(feature = "runtime-benchmarks") {
        BuildEnv::Benchmark
    } else if cfg!(any(feature = "std", feature = "fast-runtime", test)) {
        BuildEnv::TestLike
    } else {
        BuildEnv::Prod
    }
}

// Very stupid, but benchmarks are written assuming a fork epoch,
// and test vectors assuming another one
// We need allow dead code here because for regular builds this variable is not used
// This variable is only used in test, fast-runtime or runtime-benchmarks
pub const ELECTRA_TEST_FORK_EPOCH: u64 = match current_env() {
    BuildEnv::Benchmark => 80000000000,
    _ => 0,
};

// TODO: change this when parity has new tests matching fulu
pub const FULU_TEST_FORK_EPOCH: u64 = match current_env() {
    BuildEnv::Benchmark => 80000000001,
    _ => 5000000,
};

// For tests, benchmarks and fast-runtime configurations we use the mocked fork versions
pub const fn fork_versions() -> ForkVersions {
    match current_env() {
        BuildEnv::Prod => ForkVersions {
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
            fulu: Fork {
                version: hex!("90000075"),
                epoch: 272640, // https://notes.ethereum.org/@bbusa/fusaka-bpo-timeline
            },
        },
        _ => ForkVersions {
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
            fulu: Fork {
                version: [6, 0, 0, 0],
                epoch: FULU_TEST_FORK_EPOCH,
            },
        },
    }
}
