#![cfg_attr(not(feature = "std"), no_std)]

pub use cumulus_primitives_core::relay_chain::Slot;
pub use cumulus_primitives_core::ParaId;
use sp_std::vec::Vec;

/// Get the current list of container chains parachain ids.
pub trait GetCurrentContainerChains {
    fn current_container_chains() -> Vec<ParaId>;
}

/// Get the list of container chains parachain ids at giv en
/// session index.
pub trait GetSessionContainerChains<SessionIndex> {
    fn session_container_chains(session_index: SessionIndex) -> Vec<ParaId>;
}

/// Returns author for parachain id based on the provided inherent.
pub trait GetContainerChainAuthor<AccountId> {
    fn author_for_slot(slot: Slot, para_id: ParaId) -> Option<AccountId>;
}

pub trait GetHostConfiguration<SessionIndex> {
    fn orchestrator_chain_collators(session_index: SessionIndex) -> u32;
    fn collators_per_container(session_index: SessionIndex) -> u32;
}

/// Returns current session index.
pub trait GetSessionIndex<SessionIndex> {
    fn session_index() -> SessionIndex;
}
