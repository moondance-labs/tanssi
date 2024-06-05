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

//! # Data Preservers Pallet
//!
//! This pallet allows container chains to select data preservers.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(any(test, feature = "runtime-benchmarks"))]
mod benchmarks;

pub mod weights;
pub use weights::WeightInfo;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use {
    core::fmt::Debug,
    dp_core::ParaId,
    frame_support::{
        dispatch::DispatchErrorWithPostInfo,
        pallet_prelude::*,
        traits::{
            fungible::{Balanced, Inspect, MutateHold},
            tokens::Precision,
            EnsureOriginWithArg,
        },
    },
    frame_system::pallet_prelude::*,
    parity_scale_codec::FullCodec,
    sp_runtime::{
        traits::{CheckedAdd, CheckedMul, CheckedSub, Get, One, Zero},
        ArithmeticError,
    },
};

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[cfg(feature = "std")]
    pub trait SerdeIfStd: serde::Serialize + serde::de::DeserializeOwned {}
    #[cfg(feature = "std")]
    impl<T: serde::Serialize + serde::de::DeserializeOwned> SerdeIfStd for T {}

    #[cfg(not(feature = "std"))]
    pub trait SerdeIfStd {}
    #[cfg(not(feature = "std"))]
    impl<T> SerdeIfStd for T {}

    /// Balance used by this pallet
    pub type BalanceOf<T> =
        <<T as Config>::Currency as Inspect<<T as frame_system::Config>::AccountId>>::Balance;

    pub type ProviderRequestOf<T> = <<T as Config>::AssignmentPayment as AssignmentPayment<
        <T as frame_system::Config>::AccountId,
    >>::ProviderRequest;

    pub type AssignerParameterOf<T> = <<T as Config>::AssignmentPayment as AssignmentPayment<
        <T as frame_system::Config>::AccountId,
    >>::AssignerParameter;

    pub type AssignmentWitnessOf<T> = <<T as Config>::AssignmentPayment as AssignmentPayment<
        <T as frame_system::Config>::AccountId,
    >>::AssignmentWitness;

    /// Data preserver profile.
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    #[derive(
        RuntimeDebugNoBound, PartialEqNoBound, EqNoBound, Encode, Decode, CloneNoBound, TypeInfo,
    )]
    #[scale_info(skip_type_params(T))]
    pub struct Profile<T: Config> {
        pub url: BoundedVec<u8, T::MaxNodeUrlLen>,
        pub para_ids: ParaIdsFilter<T>,
        pub mode: ProfileMode,
        pub assignment_request: ProviderRequestOf<T>,
    }

    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    #[derive(
        RuntimeDebugNoBound, PartialEqNoBound, EqNoBound, Encode, Decode, CloneNoBound, TypeInfo,
    )]
    #[scale_info(skip_type_params(T))]
    pub enum ParaIdsFilter<T: Config> {
        AnyParaId,
        Whitelist(BoundedVec<ParaId, T::MaxParaIdsVecLen>),
        Blacklist(BoundedVec<ParaId, T::MaxParaIdsVecLen>),
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

    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    #[derive(RuntimeDebug, PartialEq, Eq, Encode, Decode, Clone, TypeInfo)]
    pub enum ProfileMode {
        Bootnode,
        Rpc { supports_ethereum_rpcs: bool },
    }

    /// Profile with additional data:
    /// - the account id which created (and manage) the profile
    /// - the amount deposited to register the profile
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    #[derive(
        RuntimeDebugNoBound, PartialEqNoBound, EqNoBound, Encode, Decode, CloneNoBound, TypeInfo,
    )]
    #[scale_info(skip_type_params(T))]
    pub struct RegisteredProfile<T: Config> {
        pub account: T::AccountId,
        pub deposit: BalanceOf<T>,
        pub profile: Profile<T>,
        /// There can be at most 1 assignment per profile.
        pub assignment: Option<(ParaId, AssignmentWitnessOf<T>)>,
    }

    /// Computes the deposit cost of a profile.
    pub trait ProfileDeposit<Profile, Balance> {
        fn profile_deposit(profile: &Profile) -> Result<Balance, DispatchErrorWithPostInfo>;
    }

    /// Implementation of `ProfileDeposit` based on the size of the SCALE-encoding.
    pub struct BytesProfileDeposit<BaseCost, ByteCost>(PhantomData<(BaseCost, ByteCost)>);

    impl<Profile, Balance, BaseCost, ByteCost> ProfileDeposit<Profile, Balance>
        for BytesProfileDeposit<BaseCost, ByteCost>
    where
        BaseCost: Get<Balance>,
        ByteCost: Get<Balance>,
        Profile: Encode,
        Balance: TryFrom<usize> + CheckedAdd + CheckedMul,
    {
        fn profile_deposit(profile: &Profile) -> Result<Balance, DispatchErrorWithPostInfo> {
            let base = BaseCost::get();
            let byte = ByteCost::get();
            let size: Balance = profile
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

    /// Allows to process various kinds of payment options for assignments.
    pub trait AssignmentPayment<AccountId> {
        /// Providers requests which kind of payment it accepts.
        type ProviderRequest: FullCodec + TypeInfo + Copy + Clone + Debug + Eq + SerdeIfStd;
        /// Extra parameter the assigner provides.
        type AssignerParameter: FullCodec + TypeInfo + Copy + Clone + Debug + Eq + SerdeIfStd;
        /// Represents the succesful outcome of the assignment.
        type AssignmentWitness: FullCodec + TypeInfo + Copy + Clone + Debug + Eq + SerdeIfStd;

        fn try_start_assignment(
            assigner: AccountId,
            provider: AccountId,
            request: &Self::ProviderRequest,
            extra: Self::AssignerParameter,
        ) -> Result<Self::AssignmentWitness, DispatchErrorWithPostInfo>;

        fn try_stop_assignment(
            assigner: AccountId,
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

    /// Data preservers pallet.
    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type RuntimeHoldReason: From<HoldReason>;

        type Currency: Inspect<Self::AccountId>
            + Balanced<Self::AccountId>
            + MutateHold<Self::AccountId, Reason = Self::RuntimeHoldReason>;

        type ProfileId: Default
            + FullCodec
            + TypeInfo
            + Copy
            + Clone
            + Debug
            + Eq
            + CheckedAdd
            + One
            + Ord;

        // Who can call set_boot_nodes?
        type SetBootNodesOrigin: EnsureOriginWithArg<Self::RuntimeOrigin, ParaId>;

        // Who can call start_assignment/stop_assignment?
        type AssignmentOrigin: EnsureOriginWithArg<
            Self::RuntimeOrigin,
            ParaId,
            Success = Self::AccountId,
        >;

        // Who can call force_X?
        type ForceSetProfileOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        #[pallet::constant]
        type MaxAssignmentsPerParaId: Get<u32>;
        #[pallet::constant]
        type MaxNodeUrlLen: Get<u32>;
        #[pallet::constant]
        type MaxParaIdsVecLen: Get<u32>;

        /// How much must be deposited to register a profile.
        type ProfileDeposit: ProfileDeposit<Profile<Self>, BalanceOf<Self>>;

        type AssignmentPayment: AssignmentPayment<Self::AccountId>;

        type WeightInfo: WeightInfo;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// The list of boot_nodes changed.
        BootNodesChanged { para_id: ParaId },
        ProfileCreated {
            account: T::AccountId,
            profile_id: T::ProfileId,
            deposit: BalanceOf<T>,
        },
        ProfileUpdated {
            profile_id: T::ProfileId,
            old_deposit: BalanceOf<T>,
            new_deposit: BalanceOf<T>,
        },
        ProfileDeleted {
            profile_id: T::ProfileId,
            released_deposit: BalanceOf<T>,
        },
        AssignmentStarted {
            profile_id: T::ProfileId,
            para_id: ParaId,
        },
        AssignmentStopped {
            profile_id: T::ProfileId,
            para_id: ParaId,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// This container chain does not have any boot nodes
        NoBootNodes,

        UnknownProfileId,
        NextProfileIdShouldBeAvailable,

        /// Made for `AssignmentPayment` implementors to report a mismatch between
        /// `ProviderRequest` and `AssignerParameter`.
        AssignmentPaymentRequestParameterMismatch,

        ProfileAlreadyAssigned,
        ProfileNotAssigned,
        ProfileIsNotElligibleForParaId,
        WrongParaId,
        MaxAssignmentsPerParaIdReached,
    }

    #[pallet::composite_enum]
    pub enum HoldReason {
        ProfileDeposit,
    }

    #[deprecated]
    #[pallet::storage]
    pub type BootNodes<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ParaId,
        BoundedVec<BoundedVec<u8, T::MaxNodeUrlLen>, T::MaxAssignmentsPerParaId>,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn profiles)]
    pub type Profiles<T: Config> =
        StorageMap<_, Blake2_128Concat, T::ProfileId, RegisteredProfile<T>, OptionQuery>;

    #[pallet::storage]
    pub type NextProfileId<T: Config> = StorageValue<_, T::ProfileId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn assignments)]
    pub type Assignments<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ParaId,
        BoundedVec<T::ProfileId, T::MaxAssignmentsPerParaId>,
        ValueQuery,
    >;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::create_profile(
            profile.url.len() as u32,
            profile.para_ids.len() as u32,
        ))]
        pub fn create_profile(
            origin: OriginFor<T>,
            profile: Profile<T>,
        ) -> DispatchResultWithPostInfo {
            let account = ensure_signed(origin)?;

            let deposit = T::ProfileDeposit::profile_deposit(&profile)?;
            T::Currency::hold(&HoldReason::ProfileDeposit.into(), &account, deposit)?;

            let id = NextProfileId::<T>::get();
            NextProfileId::<T>::set(
                id.checked_add(&One::one())
                    .ok_or(ArithmeticError::Overflow)?,
            );

            ensure!(
                !Profiles::<T>::contains_key(id),
                Error::<T>::NextProfileIdShouldBeAvailable
            );

            Profiles::<T>::insert(
                id,
                RegisteredProfile {
                    account: account.clone(),
                    deposit,
                    profile,
                    assignment: None,
                },
            );

            Self::deposit_event(Event::ProfileCreated {
                account,
                profile_id: id,
                deposit,
            });

            Ok(().into())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::update_profile(
            profile.url.len() as u32,
            profile.para_ids.len() as u32,
        ))]
        pub fn update_profile(
            origin: OriginFor<T>,
            profile_id: T::ProfileId,
            profile: Profile<T>,
        ) -> DispatchResultWithPostInfo {
            let account = ensure_signed(origin)?;

            let Some(existing_profile) = Profiles::<T>::get(profile_id) else {
                Err(Error::<T>::UnknownProfileId)?
            };

            ensure!(
                existing_profile.account == account,
                sp_runtime::DispatchError::BadOrigin,
            );

            // Update deposit
            let new_deposit = T::ProfileDeposit::profile_deposit(&profile)?;

            if let Some(diff) = new_deposit.checked_sub(&existing_profile.deposit) {
                T::Currency::hold(
                    &HoldReason::ProfileDeposit.into(),
                    &existing_profile.account,
                    diff,
                )?;
            } else if let Some(diff) = existing_profile.deposit.checked_sub(&new_deposit) {
                T::Currency::release(
                    &HoldReason::ProfileDeposit.into(),
                    &existing_profile.account,
                    diff,
                    Precision::Exact,
                )?;
            }

            Profiles::<T>::insert(
                profile_id,
                RegisteredProfile {
                    deposit: new_deposit,
                    profile,
                    ..existing_profile
                },
            );

            Self::deposit_event(Event::ProfileUpdated {
                profile_id,
                old_deposit: existing_profile.deposit,
                new_deposit,
            });

            Ok(().into())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::delete_profile())]
        pub fn delete_profile(
            origin: OriginFor<T>,
            profile_id: T::ProfileId,
        ) -> DispatchResultWithPostInfo {
            let account = ensure_signed(origin)?;

            let Some(profile) = Profiles::<T>::get(profile_id) else {
                Err(Error::<T>::UnknownProfileId)?
            };

            ensure!(
                profile.account == account,
                sp_runtime::DispatchError::BadOrigin,
            );

            T::Currency::release(
                &HoldReason::ProfileDeposit.into(),
                &profile.account,
                profile.deposit,
                Precision::Exact,
            )?;

            Profiles::<T>::remove(profile_id);

            Self::deposit_event(Event::ProfileDeleted {
                profile_id,
                released_deposit: profile.deposit,
            });

            Ok(().into())
        }

        #[pallet::call_index(4)]
        #[pallet::weight(T::WeightInfo::force_create_profile(
            profile.url.len() as u32,
            profile.para_ids.len() as u32,
        ))]
        pub fn force_create_profile(
            origin: OriginFor<T>,
            profile: Profile<T>,
            for_account: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            T::ForceSetProfileOrigin::ensure_origin(origin)?;

            let id = NextProfileId::<T>::get();
            NextProfileId::<T>::set(
                id.checked_add(&One::one())
                    .ok_or(ArithmeticError::Overflow)?,
            );

            ensure!(
                !Profiles::<T>::contains_key(id),
                Error::<T>::NextProfileIdShouldBeAvailable
            );

            Profiles::<T>::insert(
                id,
                RegisteredProfile {
                    account: for_account.clone(),
                    deposit: Zero::zero(),
                    profile,
                    assignment: None,
                },
            );

            Self::deposit_event(Event::ProfileCreated {
                account: for_account,
                profile_id: id,
                deposit: Zero::zero(),
            });

            Ok(().into())
        }

        #[pallet::call_index(5)]
        #[pallet::weight(T::WeightInfo::force_update_profile(
            profile.url.len() as u32,
            profile.para_ids.len() as u32,
        ))]
        pub fn force_update_profile(
            origin: OriginFor<T>,
            profile_id: T::ProfileId,
            profile: Profile<T>,
        ) -> DispatchResultWithPostInfo {
            T::ForceSetProfileOrigin::ensure_origin(origin)?;

            let Some(existing_profile) = Profiles::<T>::get(profile_id) else {
                Err(Error::<T>::UnknownProfileId)?
            };

            // We release the previous deposit
            T::Currency::release(
                &HoldReason::ProfileDeposit.into(),
                &existing_profile.account,
                existing_profile.deposit,
                Precision::Exact,
            )?;

            Profiles::<T>::insert(
                profile_id,
                RegisteredProfile {
                    account: existing_profile.account,
                    deposit: Zero::zero(),
                    profile,
                    assignment: None,
                },
            );

            Self::deposit_event(Event::ProfileUpdated {
                profile_id,
                old_deposit: existing_profile.deposit,
                new_deposit: Zero::zero(),
            });

            Ok(().into())
        }

        #[pallet::call_index(6)]
        #[pallet::weight(T::WeightInfo::force_delete_profile())]
        pub fn force_delete_profile(
            origin: OriginFor<T>,
            profile_id: T::ProfileId,
        ) -> DispatchResultWithPostInfo {
            T::ForceSetProfileOrigin::ensure_origin(origin)?;

            let Some(profile) = Profiles::<T>::get(profile_id) else {
                Err(Error::<T>::UnknownProfileId)?
            };

            T::Currency::release(
                &HoldReason::ProfileDeposit.into(),
                &profile.account,
                profile.deposit,
                Precision::Exact,
            )?;

            Profiles::<T>::remove(profile_id);

            Self::deposit_event(Event::ProfileDeleted {
                profile_id,
                released_deposit: profile.deposit,
            });

            Ok(().into())
        }

        #[pallet::call_index(7)]
        #[pallet::weight(0)]
        pub fn start_assignment(
            origin: OriginFor<T>,
            profile_id: T::ProfileId,
            para_id: ParaId,
            assigner_param: AssignerParameterOf<T>,
        ) -> DispatchResultWithPostInfo {
            let assigner = T::AssignmentOrigin::ensure_origin(origin, &para_id)?;

            let mut profile = Profiles::<T>::get(profile_id).ok_or(Error::<T>::UnknownProfileId)?;

            if profile.assignment.is_some() {
                Err(Error::<T>::ProfileAlreadyAssigned)?
            }

            if !profile.profile.para_ids.can_assign(&para_id) {
                Err(Error::<T>::ProfileIsNotElligibleForParaId)?
            }

            // Add profile id to BoundedVec early in case bound is reached
            {
                let mut assignments = Assignments::<T>::get(para_id);
                let Err(position) = assignments.binary_search(&profile_id) else {
                    Err(Error::<T>::ProfileAlreadyAssigned)?
                };

                assignments
                    .try_insert(position, profile_id)
                    .map_err(|_| Error::<T>::MaxAssignmentsPerParaIdReached)?;

                Assignments::<T>::insert(para_id, assignments);
            }

            // TODO: Check Ethereum RPC?

            let witness = T::AssignmentPayment::try_start_assignment(
                assigner,
                profile.account.clone(),
                &profile.profile.assignment_request,
                assigner_param,
            )?;

            profile.assignment = Some((para_id, witness));
            Profiles::<T>::insert(profile_id, profile);

            Self::deposit_event(Event::AssignmentStarted {
                profile_id,
                para_id,
            });

            Ok(().into())
        }

        #[pallet::call_index(8)]
        #[pallet::weight(0)]
        pub fn stop_assignment(
            origin: OriginFor<T>,
            profile_id: T::ProfileId,
            para_id: ParaId,
        ) -> DispatchResultWithPostInfo {
            let assigner = T::AssignmentOrigin::ensure_origin(origin, &para_id)?;

            let mut profile = Profiles::<T>::get(profile_id).ok_or(Error::<T>::UnknownProfileId)?;

            let Some((assignment_para_id, assignment_witness)) = profile.assignment.take() else {
                Err(Error::<T>::ProfileNotAssigned)?
            };

            if assignment_para_id != para_id {
                Err(Error::<T>::WrongParaId)?
            }

            T::AssignmentPayment::try_stop_assignment(
                assigner,
                profile.account.clone(),
                assignment_witness,
            )?;

            Profiles::<T>::insert(profile_id, profile);

            {
                let mut assignments = Assignments::<T>::get(para_id);
                let Ok(position) = assignments.binary_search(&profile_id) else {
                    Err(Error::<T>::ProfileNotAssigned)?
                };

                assignments.remove(position);

                Assignments::<T>::insert(para_id, assignments);
            }

            Self::deposit_event(Event::AssignmentStopped {
                profile_id,
                para_id,
            });

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn assignments_profiles(para_id: ParaId) -> impl Iterator<Item = Profile<T>> {
            Assignments::<T>::get(para_id)
                .into_iter()
                .filter_map(Profiles::<T>::get)
                .map(|profile| profile.profile)
        }

        /// Function that will be called when a container chain is deregistered. Cleans up all the storage related to this para_id.
        /// Cannot fail.
        pub fn para_deregistered(para_id: ParaId) {
            Assignments::<T>::remove(para_id);
        }

        pub fn check_valid_for_collating(para_id: ParaId) -> DispatchResult {
            if Self::assignments_profiles(para_id)
                .filter(|profile| profile.mode == ProfileMode::Bootnode)
                .count()
                == 0
            {
                Err(Error::<T>::NoBootNodes)?
            }

            Ok(())
        }
    }
}
