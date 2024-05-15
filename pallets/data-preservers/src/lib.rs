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
    dp_core::ParaId,
    frame_support::{
        dispatch::DispatchErrorWithPostInfo,
        pallet_prelude::*,
        traits::{
            fungible::{Balanced, Inspect, MutateHold},
            tokens::Precision,
            EnsureOriginWithArg,
        },
        DefaultNoBound,
    },
    frame_system::pallet_prelude::*,
    sp_runtime::{
        traits::{CheckedAdd, CheckedMul, CheckedSub, Get, Zero},
        ArithmeticError,
    },
    sp_std::vec::Vec,
};

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::genesis_config]
    #[derive(DefaultNoBound)]
    pub struct GenesisConfig<T: Config> {
        /// Para ids
        pub para_id_boot_nodes: Vec<(ParaId, Vec<Vec<u8>>)>,
        pub _phantom: PhantomData<T>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            // Sort para ids and detect duplicates, but do it using a vector of
            // references to avoid cloning the boot nodes.
            let mut para_ids: Vec<&_> = self.para_id_boot_nodes.iter().collect();
            para_ids.sort_by(|a, b| a.0.cmp(&b.0));
            para_ids.dedup_by(|a, b| {
                if a.0 == b.0 {
                    panic!("Duplicate para_id: {}", u32::from(a.0));
                } else {
                    false
                }
            });

            for (para_id, boot_nodes) in para_ids {
                let boot_nodes: Vec<_> = boot_nodes
                    .iter()
                    .map(|x| BoundedVec::try_from(x.clone()).expect("boot node url too long"))
                    .collect();
                let boot_nodes = BoundedVec::try_from(boot_nodes).expect("too many boot nodes");
                <BootNodes<T>>::insert(para_id, boot_nodes);
            }
        }
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
        // Who can call set_boot_nodes?
        type SetBootNodesOrigin: EnsureOriginWithArg<Self::RuntimeOrigin, ParaId>;

        type ForceSetProfileOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        #[pallet::constant]
        type MaxBootNodes: Get<u32>;
        #[pallet::constant]
        type MaxBootNodeUrlLen: Get<u32>;
        #[pallet::constant]
        type MaxParaIdsVecLen: Get<u32>;

        /// How much must be deposited to register a profile.
        type ProfileDeposit: ProfileDeposit<Profile<Self>, BalanceOf<Self>>;

        type WeightInfo: WeightInfo;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// The list of boot_nodes changed.
        BootNodesChanged { para_id: ParaId },
        ProfileCreated {
            account: T::AccountId,
            profile_id: ProfileId,
            deposit: BalanceOf<T>,
        },
        ProfileUpdated {
            profile_id: ProfileId,
            old_deposit: BalanceOf<T>,
            new_deposit: BalanceOf<T>,
        },
        ProfileDeleted {
            profile_id: ProfileId,
            released_deposit: BalanceOf<T>,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// This container chain does not have any boot nodes
        NoBootNodes,

        UnknownProfileId,
        NextProfileIdShouldBeAvailable,
    }

    #[pallet::composite_enum]
    pub enum HoldReason {
        ProfileDeposit,
    }

    #[pallet::storage]
    #[pallet::getter(fn boot_nodes)]
    pub type BootNodes<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ParaId,
        BoundedVec<BoundedVec<u8, T::MaxBootNodeUrlLen>, T::MaxBootNodes>,
        ValueQuery,
    >;

    #[pallet::storage]
    pub type Profiles<T: Config> =
        StorageMap<_, Blake2_128Concat, ProfileId, RegisteredProfile<T>, OptionQuery>;

    #[pallet::storage]
    pub type NextProfileId<T: Config> = StorageValue<_, ProfileId, ValueQuery>;

    /// Balance used by this pallet
    pub type BalanceOf<T> =
        <<T as Config>::Currency as Inspect<<T as frame_system::Config>::AccountId>>::Balance;

    pub type ProfileId = u64;

    /// Data preserver profile.
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    #[derive(
        RuntimeDebugNoBound, PartialEqNoBound, EqNoBound, Encode, Decode, CloneNoBound, TypeInfo,
    )]
    #[scale_info(skip_type_params(T))]
    pub struct Profile<T: Config> {
        pub url: BoundedVec<u8, T::MaxBootNodeUrlLen>,
        pub limited_to_para_ids: Option<BoundedVec<ParaId, T::MaxParaIdsVecLen>>,
        pub mode: ProfileMode,
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
    }

    pub trait ProfileDeposit<Profile, Balance> {
        fn profile_deposit(profile: &Profile) -> Result<Balance, DispatchErrorWithPostInfo>;
    }

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

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Set boot_nodes for this para id
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::set_boot_nodes(
            T::MaxBootNodeUrlLen::get(),
            boot_nodes.len() as u32,
        ))]
        pub fn set_boot_nodes(
            origin: OriginFor<T>,
            para_id: ParaId,
            boot_nodes: BoundedVec<BoundedVec<u8, T::MaxBootNodeUrlLen>, T::MaxBootNodes>,
        ) -> DispatchResult {
            T::SetBootNodesOrigin::ensure_origin(origin, &para_id)?;

            BootNodes::<T>::insert(para_id, boot_nodes);

            Self::deposit_event(Event::BootNodesChanged { para_id });

            Ok(())
        }

        #[pallet::call_index(1)]
        // TODO: Benchmark
        #[pallet::weight(0)]
        pub fn create_profile(
            origin: OriginFor<T>,
            profile: Profile<T>,
        ) -> DispatchResultWithPostInfo {
            let account = ensure_signed(origin)?;

            let deposit = T::ProfileDeposit::profile_deposit(&profile)?;
            T::Currency::hold(&HoldReason::ProfileDeposit.into(), &account, deposit)?;

            let id = NextProfileId::<T>::get();
            NextProfileId::<T>::set(id.checked_add(1).ok_or(ArithmeticError::Overflow)?);

            ensure!(
                !Profiles::<T>::contains_key(&id),
                Error::<T>::NextProfileIdShouldBeAvailable
            );

            Profiles::<T>::insert(
                id,
                RegisteredProfile {
                    account: account.clone(),
                    deposit,
                    profile,
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
        // TODO: Benchmark
        #[pallet::weight(0)]
        pub fn update_profile(
            origin: OriginFor<T>,
            profile_id: ProfileId,
            profile: Profile<T>,
        ) -> DispatchResultWithPostInfo {
            let account = ensure_signed(origin)?;

            let Some(existing_profile) = Profiles::<T>::get(&profile_id) else {
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
                    account: existing_profile.account,
                    deposit: new_deposit,
                    profile,
                },
            );

            Self::deposit_event(Event::ProfileUpdated {
                profile_id,
                old_deposit: existing_profile.deposit,
                new_deposit: new_deposit,
            });

            Ok(().into())
        }

        #[pallet::call_index(3)]
        // TODO: Benchmark
        #[pallet::weight(0)]
        pub fn delete_profile(
            origin: OriginFor<T>,
            profile_id: ProfileId,
        ) -> DispatchResultWithPostInfo {
            let account = ensure_signed(origin)?;

            let Some(profile) = Profiles::<T>::get(&profile_id) else {
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

            Profiles::<T>::remove(&profile_id);

            Self::deposit_event(Event::ProfileDeleted {
                profile_id,
                released_deposit: profile.deposit,
            });

            Ok(().into())
        }

        #[pallet::call_index(4)]
        // TODO: Benchmark
        #[pallet::weight(0)]
        pub fn force_create_profile(
            origin: OriginFor<T>,
            profile: Profile<T>,
            for_account: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            T::ForceSetProfileOrigin::ensure_origin(origin)?;

            let id = NextProfileId::<T>::get();
            NextProfileId::<T>::set(id.checked_add(1).ok_or(ArithmeticError::Overflow)?);

            ensure!(
                !Profiles::<T>::contains_key(&id),
                Error::<T>::NextProfileIdShouldBeAvailable
            );

            Profiles::<T>::insert(
                id,
                RegisteredProfile {
                    account: for_account.clone(),
                    deposit: Zero::zero(),
                    profile,
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
        // TODO: Benchmark
        #[pallet::weight(0)]
        pub fn force_update_profile(
            origin: OriginFor<T>,
            profile_id: ProfileId,
            profile: Profile<T>,
        ) -> DispatchResultWithPostInfo {
            T::ForceSetProfileOrigin::ensure_origin(origin)?;

            let Some(existing_profile) = Profiles::<T>::get(&profile_id) else {
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
        // TODO: Benchmark
        #[pallet::weight(0)]
        pub fn force_delete_profile(
            origin: OriginFor<T>,
            profile_id: ProfileId,
        ) -> DispatchResultWithPostInfo {
            T::ForceSetProfileOrigin::ensure_origin(origin)?;

            let Some(profile) = Profiles::<T>::get(&profile_id) else {
                Err(Error::<T>::UnknownProfileId)?
            };

            T::Currency::release(
                &HoldReason::ProfileDeposit.into(),
                &profile.account,
                profile.deposit,
                Precision::Exact,
            )?;

            Profiles::<T>::remove(&profile_id);

            Self::deposit_event(Event::ProfileDeleted {
                profile_id,
                released_deposit: profile.deposit,
            });

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Function that will be called when a container chain is deregistered. Cleans up all the storage related to this para_id.
        /// Cannot fail.
        pub fn para_deregistered(para_id: ParaId) {
            BootNodes::<T>::remove(para_id);
        }

        pub fn check_valid_for_collating(para_id: ParaId) -> DispatchResult {
            // To be able to call mark_valid_for_collating, a container chain must have bootnodes
            if Pallet::<T>::boot_nodes(para_id).len() > 0 {
                Ok(())
            } else {
                Err(Error::<T>::NoBootNodes.into())
            }
        }
    }
}
