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

use {
    super::*,
    dp_core::ParaId,
    frame_support::{dispatch::DispatchErrorWithPostInfo, pallet_prelude::*},
    serde::{de::DeserializeOwned, Serialize},
    tp_traits::{apply, derive_scale_codec, derive_storage_traits},
};

// Data preserver profile.
#[apply(derive_scale_codec)]
#[derive(RuntimeDebugNoBound, PartialEqNoBound, EqNoBound, CloneNoBound, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct Profile<T: Config> {
    pub url: BoundedVec<u8, T::MaxNodeUrlLen>,
    pub para_ids: ParaIdsFilter<T>,
    pub mode: ProfileMode,
    pub assignment_request: ProviderRequestOf<T>,
}

#[apply(derive_scale_codec)]
#[derive(RuntimeDebugNoBound, PartialEqNoBound, EqNoBound, CloneNoBound, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub enum ParaIdsFilter<T: Config> {
    AnyParaId,
    Whitelist(BoundedBTreeSet<ParaId, T::MaxParaIdsVecLen>),
    Blacklist(BoundedBTreeSet<ParaId, T::MaxParaIdsVecLen>),
}

impl<T: Config> ParaIdsFilter<T> {
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        match self {
            Self::AnyParaId => 0,
            Self::Whitelist(list) | Self::Blacklist(list) => list.len(),
        }
    }

    pub fn can_assign(&self, para_id: &ParaId) -> bool {
        match self {
            ParaIdsFilter::AnyParaId => true,
            ParaIdsFilter::Whitelist(list) => list.contains(para_id),
            ParaIdsFilter::Blacklist(list) => !list.contains(para_id),
        }
    }
}

#[apply(derive_storage_traits)]
#[derive(MaxEncodedLen)]
pub enum ProfileMode {
    Bootnode,
    Rpc { supports_ethereum_rpcs: bool },
}

/// Profile with additional data:
/// - the account id which created (and manage) the profile
/// - the amount deposited to register the profile
#[apply(derive_scale_codec)]
#[derive(RuntimeDebugNoBound, PartialEqNoBound, EqNoBound, CloneNoBound, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct RegisteredProfile<T: Config> {
    pub account: T::AccountId,
    pub deposit: BalanceOf<T>,
    pub profile: Profile<T>,
    /// There can be at most 1 assignment per profile.
    pub assignment: Option<(ParaId, AssignmentWitnessOf<T>)>,
}

/// Allows to process various kinds of payment options for assignments.
pub trait AssignmentProcessor<AccountId> {
    /// Providers requests which kind of payment it accepts.
    type ProviderRequest: tp_traits::StorageTraits + Serialize + DeserializeOwned + MaxEncodedLen;
    /// Extra parameter the assigner provides.
    type AssignerParameter: tp_traits::StorageTraits + Serialize + DeserializeOwned;
    /// Represents the succesful outcome of the assignment.
    type AssignmentWitness: tp_traits::StorageTraits + Serialize + DeserializeOwned + MaxEncodedLen;

    fn try_start_assignment(
        assigner: AccountId,
        provider: AccountId,
        request: &Self::ProviderRequest,
        extra: Self::AssignerParameter,
    ) -> Result<Self::AssignmentWitness, DispatchErrorWithPostInfo>;

    fn try_stop_assignment(
        provider: AccountId,
        witness: Self::AssignmentWitness,
    ) -> Result<(), DispatchErrorWithPostInfo>;

    /// Return the values for a free assignment if it is supported.
    /// This is required to perform automatic migration from old Bootnodes storage.
    fn free_variant_values() -> Option<(
        Self::ProviderRequest,
        Self::AssignerParameter,
        Self::AssignmentWitness,
    )>;

    // The values returned by the following functions should match with each other.
    #[cfg(feature = "runtime-benchmarks")]
    fn benchmark_provider_request() -> Self::ProviderRequest;

    #[cfg(feature = "runtime-benchmarks")]
    fn benchmark_assigner_parameter() -> Self::AssignerParameter;

    #[cfg(feature = "runtime-benchmarks")]
    fn benchmark_assignment_witness() -> Self::AssignmentWitness;
}
