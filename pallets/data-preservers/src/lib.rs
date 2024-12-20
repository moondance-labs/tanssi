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

mod types;

pub use {pallet::*, types::*};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(any(test, feature = "runtime-benchmarks"))]
mod benchmarks;

#[cfg(feature = "runtime-benchmarks")]
pub use benchmarks::ArgumentFactory;

pub mod weights;
pub use weights::WeightInfo;

use {
    core::fmt::Debug,
    dp_core::ParaId,
    frame_support::{
        dispatch::DispatchErrorWithPostInfo,
        pallet_prelude::*,
        traits::{
            fungible::{Balanced, Inspect, MutateHold},
            tokens::Precision,
            EitherOfDiverse, EnsureOriginWithArg,
        },
        DefaultNoBound,
    },
    frame_system::{pallet_prelude::*, EnsureRoot, EnsureSigned},
    parity_scale_codec::FullCodec,
    sp_runtime::{
        traits::{CheckedAdd, CheckedSub, Get, One, Zero},
        ArithmeticError, Either,
    },
    sp_std::vec::Vec,
    tp_traits::StorageDeposit,
};

#[frame_support::pallet]
pub mod pallet {
    use super::*;

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

    #[pallet::genesis_config]
    #[derive(DefaultNoBound)]
    pub struct GenesisConfig<T: Config> {
        pub bootnodes: Vec<(
            // ParaId the profile will be assigned to
            ParaId,
            // Owner of the profile
            T::AccountId,
            // URL of the bootnode
            Vec<u8>,
            // Assignment request
            ProviderRequestOf<T>,
            // Assignment witness (try_start_assignment is skipped)
            AssignmentWitnessOf<T>,
        )>,
        #[serde(skip)]
        pub _phantom: PhantomData<T>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            for (para_id, profile_owner, url, request, witness) in self.bootnodes.clone() {
                let profile = Profile {
                    url: url.try_into().expect("should fit in BoundedVec"),
                    para_ids: ParaIdsFilter::Whitelist({
                        let mut set = BoundedBTreeSet::new();
                        set.try_insert(para_id).expect("to fit in BoundedBTreeSet");
                        set
                    }),
                    mode: ProfileMode::Bootnode,
                    assignment_request: request,
                };

                let profile_id = NextProfileId::<T>::get();
                Pallet::<T>::do_create_profile(profile, profile_owner, Zero::zero())
                    .expect("to create profile");
                Pallet::<T>::do_start_assignment(profile_id, para_id, |_| Ok(witness))
                    .expect("to start assignment");
            }
        }
    }

    /// Data preservers pallet.
    #[pallet::pallet]
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
            + Ord
            + MaxEncodedLen;

        // Who can call start_assignment/stop_assignment?
        type AssignmentOrigin: EnsureOriginWithArg<
            Self::RuntimeOrigin,
            ParaId,
            Success = Self::AccountId,
        >;

        // Who can call force_X?
        type ForceSetProfileOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        #[pallet::constant]
        type MaxAssignmentsPerParaId: Get<u32> + Clone;
        #[pallet::constant]
        type MaxNodeUrlLen: Get<u32> + Clone;
        #[pallet::constant]
        type MaxParaIdsVecLen: Get<u32> + Clone;

        /// How much must be deposited to register a profile.
        type ProfileDeposit: StorageDeposit<Profile<Self>, BalanceOf<Self>>;

        type AssignmentPayment: AssignmentPayment<Self::AccountId>;

        type WeightInfo: WeightInfo;

        /// Helper type for benchmarks.
        #[cfg(feature = "runtime-benchmarks")]
        type BenchmarkHelper: ArgumentFactory<ParaId>;
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
        CantDeleteAssignedProfile,
    }

    #[pallet::composite_enum]
    pub enum HoldReason {
        ProfileDeposit,
    }

    #[pallet::storage]
    pub type Profiles<T: Config> =
        StorageMap<_, Blake2_128Concat, T::ProfileId, RegisteredProfile<T>, OptionQuery>;

    #[pallet::storage]
    pub type NextProfileId<T: Config> = StorageValue<_, T::ProfileId, ValueQuery>;

    #[pallet::storage]
    pub type Assignments<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ParaId,
        BoundedBTreeSet<T::ProfileId, T::MaxAssignmentsPerParaId>,
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

            let deposit = T::ProfileDeposit::compute_deposit(&profile)?;
            T::Currency::hold(&HoldReason::ProfileDeposit.into(), &account, deposit)?;

            Self::do_create_profile(profile, account, deposit)
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

            let new_deposit = T::ProfileDeposit::compute_deposit(&profile)?;

            Self::do_update_profile(profile_id, profile, |existing_profile| {
                ensure!(
                    existing_profile.account == account,
                    sp_runtime::DispatchError::BadOrigin,
                );

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

                Ok(new_deposit)
            })
        }

        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::delete_profile())]
        pub fn delete_profile(
            origin: OriginFor<T>,
            profile_id: T::ProfileId,
        ) -> DispatchResultWithPostInfo {
            let account = ensure_signed(origin)?;

            Self::do_delete_profile(profile_id, |profile| {
                ensure!(
                    profile.account == account,
                    sp_runtime::DispatchError::BadOrigin,
                );

                Ok(().into())
            })
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

            Self::do_create_profile(profile, for_account, Zero::zero())
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

            Self::do_update_profile(profile_id, profile, |existing_profile| {
                // We release the previous deposit
                T::Currency::release(
                    &HoldReason::ProfileDeposit.into(),
                    &existing_profile.account,
                    existing_profile.deposit,
                    Precision::Exact,
                )?;

                // New deposit is zero since its forced
                Ok(Zero::zero())
            })
        }

        #[pallet::call_index(6)]
        #[pallet::weight(T::WeightInfo::force_delete_profile())]
        pub fn force_delete_profile(
            origin: OriginFor<T>,
            profile_id: T::ProfileId,
        ) -> DispatchResultWithPostInfo {
            T::ForceSetProfileOrigin::ensure_origin(origin)?;

            Self::do_delete_profile(profile_id, |_| Ok(().into()))
        }

        #[pallet::call_index(7)]
        #[pallet::weight(T::WeightInfo::start_assignment())]
        pub fn start_assignment(
            origin: OriginFor<T>,
            profile_id: T::ProfileId,
            para_id: ParaId,
            assigner_param: AssignerParameterOf<T>,
        ) -> DispatchResultWithPostInfo {
            let assigner = T::AssignmentOrigin::ensure_origin(origin, &para_id)?;

            Self::do_start_assignment(profile_id, para_id, |profile| {
                T::AssignmentPayment::try_start_assignment(
                    assigner,
                    profile.account.clone(),
                    &profile.profile.assignment_request,
                    assigner_param,
                )
            })
        }

        #[pallet::call_index(8)]
        #[pallet::weight(T::WeightInfo::stop_assignment())]
        pub fn stop_assignment(
            origin: OriginFor<T>,
            profile_id: T::ProfileId,
            para_id: ParaId,
        ) -> DispatchResultWithPostInfo {
            let caller = EitherOfDiverse::<
                // root or para manager can call without being the owner
                EitherOfDiverse<T::AssignmentOrigin, EnsureRoot<T::AccountId>>,
                // otherwise it can be a simple signed account but it will require
                // cheking if it is the owner of the profile
                EnsureSigned<T::AccountId>,
            >::ensure_origin(origin, &para_id)?;

            let mut profile = Profiles::<T>::get(profile_id).ok_or(Error::<T>::UnknownProfileId)?;

            match caller {
                // root or para id manager is allowed to call
                Either::Left(_) => (),
                // signed, must be profile owner
                Either::Right(account) => ensure!(
                    profile.account == account,
                    sp_runtime::DispatchError::BadOrigin
                ),
            }

            let Some((assignment_para_id, assignment_witness)) = profile.assignment.take() else {
                Err(Error::<T>::ProfileNotAssigned)?
            };

            if assignment_para_id != para_id {
                Err(Error::<T>::WrongParaId)?
            }

            T::AssignmentPayment::try_stop_assignment(profile.account.clone(), assignment_witness)?;

            Profiles::<T>::insert(profile_id, profile);

            {
                let mut assignments = Assignments::<T>::get(para_id);
                assignments.remove(&profile_id);
                Assignments::<T>::insert(para_id, assignments);
            }

            Self::deposit_event(Event::AssignmentStopped {
                profile_id,
                para_id,
            });

            Ok(().into())
        }

        #[pallet::call_index(9)]
        #[pallet::weight(T::WeightInfo::force_start_assignment())]
        pub fn force_start_assignment(
            origin: OriginFor<T>,
            profile_id: T::ProfileId,
            para_id: ParaId,
            assignment_witness: AssignmentWitnessOf<T>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            Self::do_start_assignment(profile_id, para_id, |_profile| Ok(assignment_witness))
        }
    }

    impl<T: Config> Pallet<T> {
        fn do_create_profile(
            profile: Profile<T>,
            account: T::AccountId,
            deposit: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
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

        fn do_update_profile(
            profile_id: T::ProfileId,
            new_profile: Profile<T>,
            update_deposit: impl FnOnce(
                &RegisteredProfile<T>,
            ) -> Result<BalanceOf<T>, DispatchErrorWithPostInfo>,
        ) -> DispatchResultWithPostInfo {
            let Some(existing_profile) = Profiles::<T>::get(profile_id) else {
                Err(Error::<T>::UnknownProfileId)?
            };

            let new_deposit = update_deposit(&existing_profile)?;

            Profiles::<T>::insert(
                profile_id,
                RegisteredProfile {
                    deposit: new_deposit,
                    profile: new_profile,
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

        fn do_delete_profile(
            profile_id: T::ProfileId,
            profile_owner_check: impl FnOnce(&RegisteredProfile<T>) -> DispatchResultWithPostInfo,
        ) -> DispatchResultWithPostInfo {
            let Some(profile) = Profiles::<T>::get(profile_id) else {
                Err(Error::<T>::UnknownProfileId)?
            };

            ensure!(
                profile.assignment.is_none(),
                Error::<T>::CantDeleteAssignedProfile,
            );

            profile_owner_check(&profile)?;

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

        fn do_start_assignment(
            profile_id: T::ProfileId,
            para_id: ParaId,
            witness_producer: impl FnOnce(
                &RegisteredProfile<T>,
            )
                -> Result<AssignmentWitnessOf<T>, DispatchErrorWithPostInfo>,
        ) -> DispatchResultWithPostInfo {
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

                assignments
                    .try_insert(profile_id)
                    .map_err(|_| Error::<T>::MaxAssignmentsPerParaIdReached)?;

                Assignments::<T>::insert(para_id, assignments);
            }

            let witness = witness_producer(&profile)?;

            profile.assignment = Some((para_id, witness));
            Profiles::<T>::insert(profile_id, profile);

            Self::deposit_event(Event::AssignmentStarted {
                profile_id,
                para_id,
            });

            Ok(().into())
        }

        pub fn assignments_profiles(para_id: ParaId) -> impl Iterator<Item = Profile<T>> {
            Assignments::<T>::get(para_id)
                .into_iter()
                .filter_map(Profiles::<T>::get)
                .map(|profile| profile.profile)
        }

        /// Function that will be called when a container chain is deregistered. Cleans up all the
        /// storage related to this para_id.
        /// Cannot fail.
        pub fn para_deregistered(para_id: ParaId) {
            Assignments::<T>::remove(para_id);
        }

        pub fn check_valid_for_collating(para_id: ParaId) -> DispatchResult {
            if !Self::assignments_profiles(para_id)
                .any(|profile| profile.mode == ProfileMode::Bootnode)
            {
                Err(Error::<T>::NoBootNodes)?
            }

            Ok(())
        }
    }
}
