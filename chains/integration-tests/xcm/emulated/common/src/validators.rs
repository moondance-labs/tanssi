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
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>.

use {
    crate::get_authority_keys_from_seed_no_beefy,
    babe_primitives::AuthorityId as BabeId,
    cumulus_primitives_core::relay_chain::{
        AccountId, AssignmentId, AuthorityDiscoveryId, ValidatorId,
    },
    sc_consensus_grandpa::AuthorityId as GrandpaId,
};

pub fn initial_authorities() -> Vec<(
    AccountId,
    AccountId,
    BabeId,
    GrandpaId,
    ValidatorId,
    AssignmentId,
    AuthorityDiscoveryId,
)> {
    vec![get_authority_keys_from_seed_no_beefy("Alice")]
}
