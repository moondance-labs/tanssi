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

pub mod alias;

pub use {
    alias::*,
    cumulus_primitives_core::{
        relay_chain::{BlockNumber, HeadData, Slot, ValidationCode},
        ParaId,
    },
    dp_chain_state_snapshot::{GenericStateProof, ReadEntryErr},
    dp_container_chain_genesis_data::ContainerChainGenesisDataItem,
};
use {
    core::marker::PhantomData,
    frame_support::{
        dispatch::DispatchErrorWithPostInfo,
        pallet_prelude::{Decode, DispatchResultWithPostInfo, Encode, Get, MaxEncodedLen, Weight},
        BoundedVec,
    },
    scale_info::TypeInfo,
    serde::{Deserialize, Serialize},
    sp_core::H256,
    sp_runtime::{
        app_crypto::sp_core,
        traits::{CheckedAdd, CheckedMul},
        ArithmeticError, DispatchResult, Perbill, RuntimeDebug,
    },
    sp_std::{collections::btree_set::BTreeSet, vec::Vec},
};

// Separate import as rustfmt wrongly change it to `sp_std::vec::self`, which is the module instead
// of the macro.
use sp_std::vec;

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
/// tanssi slot time, 6 seconds.
// TODO: this is currently ignored
#[derive(
    Clone,
    Debug,
    Encode,
    Decode,
    scale_info::TypeInfo,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    MaxEncodedLen,
)]
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

impl SlotFrequency {
    pub fn should_parathread_buy_core(
        &self,
        current_slot: Slot,
        max_slot_required_to_complete_purchase: Slot,
        last_block_slot: Slot,
    ) -> bool {
        current_slot
            >= last_block_slot
                .saturating_add(Slot::from(u64::from(self.min)))
                .saturating_sub(max_slot_required_to_complete_purchase)
    }

    pub fn should_parathread_author_block(
        &self,
        current_slot: Slot,
        last_block_slot: Slot,
    ) -> bool {
        current_slot >= last_block_slot.saturating_add(Slot::from(u64::from(self.min)))
    }
}

impl Default for SlotFrequency {
    fn default() -> Self {
        Self { min: 1, max: 1 }
    }
}

#[derive(
    Clone,
    Debug,
    Encode,
    Decode,
    scale_info::TypeInfo,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    MaxEncodedLen,
)]
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
    fn target_container_chain_fullness(session_index: SessionIndex) -> Perbill;
    fn max_parachain_cores_percentage(session_index: SessionIndex) -> Option<Perbill>;
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

impl<SessionIndex> ShouldRotateAllCollators<SessionIndex> for () {
    fn should_rotate_all_collators(_session_index: SessionIndex) -> bool {
        false
    }
}

/// Helper trait for pallet_collator_assignment to be able to give priority to invulnerables
pub trait RemoveInvulnerables<AccountId> {
    /// Remove the first n invulnerables from the list of collators. The order should be respected.
    fn remove_invulnerables(
        collators: &mut Vec<AccountId>,
        num_invulnerables: usize,
    ) -> Vec<AccountId>;
}

impl<AccountId: Clone> RemoveInvulnerables<AccountId> for () {
    fn remove_invulnerables(
        _collators: &mut Vec<AccountId>,
        _num_invulnerables: usize,
    ) -> Vec<AccountId> {
        // Default impl: no collators are invulnerables
        vec![]
    }
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

impl RemoveParaIdsWithNoCredits for () {
    fn remove_para_ids_with_no_credits(
        _para_ids: &mut Vec<ParaId>,
        _currently_assigned: &BTreeSet<ParaId>,
    ) {
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn make_valid_para_ids(_para_ids: &[ParaId]) {}
}

pub trait RelayStorageRootProvider {
    fn get_relay_storage_root(relay_block_number: u32) -> Option<H256>;

    #[cfg(feature = "runtime-benchmarks")]
    fn set_relay_storage_root(relay_block_number: u32, storage_root: Option<H256>);
}

impl RelayStorageRootProvider for () {
    fn get_relay_storage_root(_relay_block_number: u32) -> Option<H256> {
        None
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn set_relay_storage_root(_relay_block_number: u32, _storage_root: Option<H256>) {}
}

/// Information extracted from the latest container chain header
#[derive(
    Default,
    Clone,
    Encode,
    Decode,
    PartialEq,
    sp_core::RuntimeDebug,
    scale_info::TypeInfo,
    MaxEncodedLen,
    Serialize,
    Deserialize,
)]
pub struct ContainerChainBlockInfo<AccountId> {
    pub block_number: BlockNumber,
    pub author: AccountId,
    pub latest_slot_number: Slot,
}

pub trait LatestAuthorInfoFetcher<AccountId> {
    fn get_latest_author_info(para_id: ParaId) -> Option<ContainerChainBlockInfo<AccountId>>;
}

pub trait StorageDeposit<Data, Balance> {
    fn compute_deposit(data: &Data) -> Result<Balance, DispatchErrorWithPostInfo>;
}

pub struct BytesDeposit<BaseCost, ByteCost>(PhantomData<(BaseCost, ByteCost)>);
impl<Data, Balance, BaseCost, ByteCost> StorageDeposit<Data, Balance>
    for BytesDeposit<BaseCost, ByteCost>
where
    Data: Encode,
    Balance: TryFrom<usize> + CheckedAdd + CheckedMul,
    BaseCost: Get<Balance>,
    ByteCost: Get<Balance>,
{
    fn compute_deposit(data: &Data) -> Result<Balance, DispatchErrorWithPostInfo> {
        let base = BaseCost::get();
        let byte = ByteCost::get();
        let size: Balance = data
            .encoded_size()
            .try_into()
            .map_err(|_| ArithmeticError::Overflow)?;

        let deposit = byte
            .checked_mul(&size)
            .ok_or(ArithmeticError::Overflow)?
            .checked_add(&base)
            .ok_or(ArithmeticError::Overflow)?;

        Ok(deposit)
    }
}

/// Trait to abstract away relay storage proofs, and allow the same logic to work on both parachains and solochains.
/// Parachains should use relay storage proofs, while solochains should read from storage directly.
pub trait GenericStorageReader {
    fn read_entry<T: Decode>(&self, key: &[u8], fallback: Option<T>) -> Result<T, ReadEntryErr>;
}

impl GenericStorageReader for GenericStateProof<cumulus_primitives_core::relay_chain::Block> {
    fn read_entry<T: Decode>(&self, key: &[u8], fallback: Option<T>) -> Result<T, ReadEntryErr> {
        GenericStateProof::read_entry(self, key, fallback)
    }
}

/// Solo chain impl, read directly from storage
pub struct NativeStorageReader;
impl GenericStorageReader for NativeStorageReader {
    fn read_entry<T: Decode>(&self, key: &[u8], fallback: Option<T>) -> Result<T, ReadEntryErr> {
        match frame_support::storage::unhashed::get(key).or(fallback) {
            Some(x) => Ok(x),
            None => Err(ReadEntryErr::Absent),
        }
    }
}

/// Trait to handle registrar-related operations in a relay-chain context.
/// Mostly used to wire Tanssi's and Polkadot's registrars, for them to
/// work together in a solo-chain environment.
pub trait RegistrarHandler<AccountId> {
    fn register(
        who: AccountId,
        id: ParaId,
        genesis_storage: &[ContainerChainGenesisDataItem],
        head_data: Option<HeadData>,
    ) -> DispatchResult;

    fn schedule_para_upgrade(id: ParaId) -> DispatchResult;
    fn schedule_para_downgrade(id: ParaId) -> DispatchResult;
    fn deregister(id: ParaId);
    fn deregister_weight() -> Weight;

    #[cfg(feature = "runtime-benchmarks")]
    fn bench_head_data() -> Option<HeadData> {
        None
    }
    #[cfg(feature = "runtime-benchmarks")]
    fn add_trusted_validation_code(_code: Vec<u8>) {}
    #[cfg(feature = "runtime-benchmarks")]
    fn registrar_new_session(_session: u32) {}
}

impl<AccountId> RegistrarHandler<AccountId> for () {
    fn register(
        _who: AccountId,
        _id: ParaId,
        _genesis_storage: &[ContainerChainGenesisDataItem],
        _head_data: Option<HeadData>,
    ) -> DispatchResult {
        Ok(())
    }

    fn schedule_para_upgrade(_id: ParaId) -> DispatchResult {
        Ok(())
    }

    fn schedule_para_downgrade(_id: ParaId) -> DispatchResult {
        Ok(())
    }

    fn deregister(_id: ParaId) {}

    fn deregister_weight() -> Weight {
        Weight::default()
    }
}

/// Trait to retrieve the orchestrator block author (if any).
/// In a relay-chain context we will return None.
pub trait MaybeSelfChainBlockAuthor<AccountId> {
    fn get_block_author() -> Option<AccountId>;
}

impl<AccountId> MaybeSelfChainBlockAuthor<AccountId> for () {
    fn get_block_author() -> Option<AccountId> {
        None
    }
}

/// Information regarding the active era (era in used in session).
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct ActiveEraInfo {
    /// Index of era.
    pub index: EraIndex,
    /// Moment of start expressed as millisecond from `$UNIX_EPOCH`.
    ///
    /// Start can be none if start hasn't been set for the era yet,
    /// Start is set on the first on_finalize of the era to guarantee usage of `Time`.
    pub start: Option<u64>,
}

/// Counter for the number of eras that have passed.
pub type EraIndex = u32;

pub trait EraIndexProvider {
    fn active_era() -> ActiveEraInfo;
    fn era_to_session_start(era_index: EraIndex) -> Option<u32>;
}

pub trait ValidatorProvider<ValidatorId> {
    fn validators() -> Vec<ValidatorId>;
}

pub trait InvulnerablesProvider<ValidatorId> {
    fn invulnerables() -> Vec<ValidatorId>;
}

pub trait OnEraStart {
    fn on_era_start(_era_index: EraIndex, _session_start: u32) {}
}

#[impl_trait_for_tuples::impl_for_tuples(5)]
impl OnEraStart for Tuple {
    fn on_era_start(era_index: EraIndex, session_start: u32) {
        for_tuples!( #( Tuple::on_era_start(era_index, session_start); )* );
    }
}

pub trait OnEraEnd {
    fn on_era_end(_era_index: EraIndex) {}
}

#[impl_trait_for_tuples::impl_for_tuples(5)]
impl OnEraEnd for Tuple {
    fn on_era_end(era_index: EraIndex) {
        for_tuples!( #( Tuple::on_era_end(era_index); )* );
    }
}
