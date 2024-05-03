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
        pallet_prelude::{Decode, DispatchResultWithPostInfo, Encode, Get, Weight},
        BoundedVec,
    },
    sp_std::{collections::btree_set::BTreeSet, vec::Vec},
};

/// The collator-assignment hook to react to collators being assigned to container chains.
pub trait CollatorAssignmentHook<Balance> {
    /// This hook is called when collators are assigned to a container
    ///
    /// The hook should never panic and is required to return the weight consumed.
    fn on_collators_assigned(
        para_id: ParaId,
        maybe_tip: Option<&Balance>,
        is_parathread: bool,
    ) -> Result<Weight, sp_runtime::DispatchError>;
}

#[impl_trait_for_tuples::impl_for_tuples(5)]
impl<Balance> CollatorAssignmentHook<Balance> for Tuple {
    fn on_collators_assigned(
        p: ParaId,
        t: Option<&Balance>,
        ip: bool,
    ) -> Result<Weight, sp_runtime::DispatchError> {
        let mut weight: Weight = Default::default();
        for_tuples!( #( weight.saturating_accrue(Tuple::on_collators_assigned(p, t, ip)?); )* );
        Ok(weight)
    }
}

/// Container chains collator assignment tip prioritization on congestion.
/// Tips paras are willing to pay for collator assignment in case of collators demand
/// surpasses the offer.
pub trait CollatorAssignmentTip<Balance> {
    fn get_para_tip(a: ParaId) -> Option<Balance>;
}

impl<Balance> CollatorAssignmentTip<Balance> for () {
    fn get_para_tip(_: ParaId) -> Option<Balance> {
        None
    }
}
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

/// How often should a parathread collator propose blocks. The units are "1 out of n slots", where the slot time is the
/// tanssi slot time, 12 seconds by default.
// TODO: this is currently ignored
#[derive(Clone, Debug, Encode, Decode, scale_info::TypeInfo, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct SlotFrequency {
    /// The parathread will produce at most 1 block every x slots. min=10 means that collators can produce 1 block
    /// every `x >= 10` slots, but they are not enforced to. If collators produce a block after less than 10
    /// slots, they will not be rewarded by tanssi.
    pub min: u32,
    /// The parathread will produce at least 1 block every x slots. max=10 means that collators are forced to
    /// produce 1 block every `x <= 10` slots. Collators can produce a block sooner than that if the `min` allows it, but
    /// waiting more than 10 slots will make them lose the block reward.
    pub max: u32,
}

impl Default for SlotFrequency {
    fn default() -> Self {
        Self { min: 1, max: 1 }
    }
}

#[derive(Clone, Debug, Encode, Decode, scale_info::TypeInfo, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct ParathreadParams {
    pub slot_frequency: SlotFrequency,
}

#[derive(Clone, Debug, Encode, Decode, scale_info::TypeInfo, PartialEq, Eq)]
pub struct SessionContainerChains {
    pub parachains: Vec<ParaId>,
    pub parathreads: Vec<(ParaId, ParathreadParams)>,
}

/// Get the list of container chains parachain ids at given
/// session index.
pub trait GetSessionContainerChains<SessionIndex> {
    fn session_container_chains(session_index: SessionIndex) -> SessionContainerChains;
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
    fn max_collators(session_index: SessionIndex) -> u32;
    fn min_collators_for_orchestrator(session_index: SessionIndex) -> u32;
    fn max_collators_for_orchestrator(session_index: SessionIndex) -> u32;
    fn collators_per_container(session_index: SessionIndex) -> u32;
    fn collators_per_parathread(session_index: SessionIndex) -> u32;
    #[cfg(feature = "runtime-benchmarks")]
    fn set_host_configuration(_session_index: SessionIndex) {}
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
    fn remove_para_ids_with_no_credits(
        para_ids: &mut Vec<ParaId>,
        currently_assigned: &BTreeSet<ParaId>,
    );

    /// Make those para ids valid by giving them enough credits, for benchmarking.
    #[cfg(feature = "runtime-benchmarks")]
    fn make_valid_para_ids(para_ids: &[ParaId]);
}
