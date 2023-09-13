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

//! Crate containing various traits used by moondance crates allowing to connect pallet
//! with each other or with mocks.

#![cfg_attr(not(feature = "std"), no_std)]

pub use cumulus_primitives_core::{relay_chain::Slot, ParaId};
use sp_std::vec::Vec;

/// Get the current list of container chains parachain ids.
pub trait GetCurrentContainerChains {
    fn current_container_chains() -> Vec<ParaId>;

    #[cfg(feature = "runtime-benchmarks")]
    fn set_current_container_chains(container_chains: &[ParaId]);
}

/// Get the list of container chains parachain ids at given
/// session index.
pub trait GetSessionContainerChains<SessionIndex> {
    fn session_container_chains(session_index: SessionIndex) -> Vec<ParaId>;
    #[cfg(feature = "runtime-benchmarks")]
    fn set_session_container_chains(session_index: SessionIndex, container_chains: &[ParaId]);
}

/// Returns author for a parachain id for the given slot.
pub trait GetContainerChainAuthor<AccountId> {
    fn author_for_slot(slot: Slot, para_id: ParaId) -> Option<AccountId>;
    #[cfg(feature = "runtime-benchmarks")]
    fn set_authors_for_para_id(para_id: ParaId, authors: Vec<AccountId>);
}

/// Returns the host configuration composed of the amount of collators assigned
/// to the orchestrator chain, and how many collators are assigned per container chain.
pub trait GetHostConfiguration<SessionIndex> {
    fn min_collators_for_orchestrator(session_index: SessionIndex) -> u32;
    fn max_collators_for_orchestrator(session_index: SessionIndex) -> u32;
    fn collators_per_container(session_index: SessionIndex) -> u32;
}

/// Returns current session index.
pub trait GetSessionIndex<SessionIndex> {
    fn session_index() -> SessionIndex;
}
