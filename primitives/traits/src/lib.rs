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

pub use cumulus_primitives_core::{
    relay_chain::{BlockNumber, Slot},
    ParaId,
};
use {
    frame_support::{
        pallet_prelude::{DispatchResultWithPostInfo, Get, Weight},
        BoundedVec,
    },
    sp_std::vec::Vec,
};

/// The author-noting hook to react to container chains authoring.
pub trait AuthorNotingHook<AccountId> {
    /// This hook is called partway through the `set_latest_author_data` inherent in author-noting.
    ///
    /// The hook should never panic and is required to return the weight consumed.
    fn on_container_author_noted(
        author: &AccountId,
        block_number: BlockNumber,
        para_id: ParaId,
    ) -> Weight;
}

#[impl_trait_for_tuples::impl_for_tuples(5)]
impl<AccountId> AuthorNotingHook<AccountId> for Tuple {
    fn on_container_author_noted(a: &AccountId, b: BlockNumber, p: ParaId) -> Weight {
        let mut weight: Weight = Default::default();
        for_tuples!( #( weight.saturating_accrue(Tuple::on_container_author_noted(a, b, p)); )* );
        weight
    }
}

pub trait DistributeRewards<AccountId, Imbalance> {
    fn distribute_rewards(rewarded: AccountId, amount: Imbalance) -> DispatchResultWithPostInfo;
}

impl<AccountId, Imbalance> DistributeRewards<AccountId, Imbalance> for () {
    fn distribute_rewards(_rewarded: AccountId, _amount: Imbalance) -> DispatchResultWithPostInfo {
        Ok(().into())
    }
}

/// Get the current list of container chains parachain ids.
pub trait GetCurrentContainerChains {
    type MaxContainerChains: Get<u32>;

    fn current_container_chains() -> BoundedVec<ParaId, Self::MaxContainerChains>;

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

/// Should pallet_collator_assignment trigger a full rotation on this session?
pub trait ShouldRotateAllCollators<SessionIndex> {
    fn should_rotate_all_collators(session_index: SessionIndex) -> bool;
}

/// Helper trait for pallet_collator_assignment to be able to give priority to invulnerables
pub trait RemoveInvulnerables<AccountId> {
    /// Remove the first n invulnerables from the list of collators. The order should be respected.
    fn remove_invulnerables(
        collators: &mut Vec<AccountId>,
        num_invulnerables: usize,
    ) -> Vec<AccountId>;
}

/// Helper trait for pallet_collator_assignment to be able to not assign collators to container chains with no credits
/// in pallet_services_payment
pub trait RemoveParaIdsWithNoCredits {
    /// Remove para ids with not enough credits. The resulting order will affect priority: the first para id in the list
    /// will be the first one to get collators.
    fn remove_para_ids_with_no_credits(para_ids: &mut Vec<ParaId>);

    /// Make those para ids valid by giving them enough credits, for benchmarking.
    #[cfg(feature = "runtime-benchmarks")]
    fn make_valid_para_ids(para_ids: &[ParaId]);
}
