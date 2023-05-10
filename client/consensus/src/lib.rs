// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! The AuRa consensus algorithm for parachains.    
//!
//! This extends the Substrate provided AuRa consensus implementation to make it compatible for
//! parachains. The main entry points for of this consensus algorithm are [`AuraConsensus::build`]
//! and [`fn@import_queue`].
//!
//! For more information about AuRa, the Substrate crate should be checked.
use {sp_consensus_slots::Slot, sp_core::crypto::Pair};

mod consensus_container;
mod consensus_orchestrator;
mod manual_seal;

pub use {
    consensus_container::*,
    consensus_orchestrator::{BuildOrchestratorAuraConsensusParams, OrchestratorAuraConsensus},
};

pub use {
    manual_seal::OrchestratorManualSealAuraConsensusDataProvider,
    sc_consensus_aura::{slot_duration, AuraVerifier, BuildAuraWorkerParams, SlotProportion},
    sc_consensus_slots::InherentDataProviderExt,
};

type AuthorityId<P> = <P as Pair>::Public;

/// Get slot author for given block along with authorities.
pub(crate) fn slot_author<P: Pair>(
    slot: Slot,
    authorities: &[AuthorityId<P>],
) -> Option<&AuthorityId<P>> {
    if authorities.is_empty() {
        return None;
    }

    let idx = *slot % (authorities.len() as u64);
    assert!(
        idx <= usize::MAX as u64,
        "It is impossible to have a vector with length beyond the address space; qed",
    );

    let current_author = authorities.get(idx as usize).expect(
        "authorities not empty; index constrained to list length;this is a valid index; qed",
    );

    Some(current_author)
}
