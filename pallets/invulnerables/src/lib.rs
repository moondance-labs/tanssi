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

//! Invulnerables pallet.
//!
//! A pallet to manage invulnerable collators in a parachain.
//!
//! ## Terminology
//!
//! - Collator: A parachain block producer.
//! - Invulnerable: An account appointed by governance and guaranteed to be in the collator set.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use {
    core::marker::PhantomData,
    sp_runtime::{traits::Convert, TokenError},
};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;

#[frame_support::pallet]
pub mod pallet {
    pub use crate::weights::WeightInfo;

    #[cfg(feature = "runtime-benchmarks")]
    use frame_support::traits::Currency;

    use {
        frame_support::{
            dispatch::DispatchResultWithPostInfo,
            pallet_prelude::*,
            traits::{EnsureOrigin, ValidatorRegistration},
            BoundedVec, DefaultNoBound,
        },
        frame_system::pallet_prelude::*,
        pallet_session::SessionManager,
        sp_runtime::traits::Convert,
        sp_staking::SessionIndex,
        sp_std::vec::Vec,
    };

    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

    /// A convertor from collators id. Since this pallet does not have stash/controller, this is
    /// just identity.
    pub struct IdentityCollator;
    impl<T> sp_runtime::traits::Convert<T, Option<T>> for IdentityCollator {
        fn convert(t: T) -> Option<T> {
            Some(t)
        }
    }

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Origin that can dictate updating parameters of this pallet.
        type UpdateOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        /// Maximum number of invulnerables.
        #[pallet::constant]
        type MaxInvulnerables: Get<u32>;

        /// A stable ID for a collator.
        type CollatorId: Member + Parameter + MaybeSerializeDeserialize + MaxEncodedLen + Ord;

        /// A conversion from account ID to collator ID.
        ///
        /// Its cost must be at most one storage read.
        type CollatorIdOf: Convert<Self::AccountId, Option<Self::CollatorId>>;

        /// Validate a user is registered
        type CollatorRegistration: ValidatorRegistration<Self::CollatorId>;

        /// The weight information of this pallet.
        type WeightInfo: WeightInfo;

        #[cfg(feature = "runtime-benchmarks")]
        type Currency: Currency<Self::AccountId>
            + frame_support::traits::fungible::Balanced<Self::AccountId>;
    }

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    /// The invulnerable, permissioned collators. This list must be sorted.
    #[pallet::storage]
    #[pallet::getter(fn invulnerables)]
    pub type Invulnerables<T: Config> =
        StorageValue<_, BoundedVec<T::CollatorId, T::MaxInvulnerables>, ValueQuery>;

    #[pallet::genesis_config]
    #[derive(DefaultNoBound)]
    pub struct GenesisConfig<T: Config> {
        pub invulnerables: Vec<T::CollatorId>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            let duplicate_invulnerables = self
                .invulnerables
                .iter()
                .collect::<sp_std::collections::btree_set::BTreeSet<_>>();
            assert!(
                duplicate_invulnerables.len() == self.invulnerables.len(),
                "duplicate invulnerables in genesis."
            );

            let bounded_invulnerables =
                BoundedVec::<_, T::MaxInvulnerables>::try_from(self.invulnerables.clone())
                    .expect("genesis invulnerables are more than T::MaxInvulnerables");

            <Invulnerables<T>>::put(bounded_invulnerables);
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// New Invulnerables were set.
        NewInvulnerables { invulnerables: Vec<T::CollatorId> },
        /// A new Invulnerable was added.
        InvulnerableAdded { account_id: T::AccountId },
        /// An Invulnerable was removed.
        InvulnerableRemoved { account_id: T::AccountId },
        /// An account was unable to be added to the Invulnerables because they did not have keys
        /// registered. Other Invulnerables may have been set.
        InvalidInvulnerableSkipped { account_id: T::AccountId },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// There are too many Invulnerables.
        TooManyInvulnerables,
        /// Account is already an Invulnerable.
        AlreadyInvulnerable,
        /// Account is not an Invulnerable.
        NotInvulnerable,
        /// Account does not have keys registered
        NoKeysRegistered,
        /// Unable to derive collator id from account id
        UnableToDeriveCollatorId,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Add a new account `who` to the list of `Invulnerables` collators.
        ///
        /// The origin for this call must be the `UpdateOrigin`.
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::add_invulnerable(
			T::MaxInvulnerables::get().saturating_sub(1),
		))]
        pub fn add_invulnerable(
            origin: OriginFor<T>,
            who: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            T::UpdateOrigin::ensure_origin(origin)?;
            // don't let one unprepared collator ruin things for everyone.
            let maybe_collator_id = T::CollatorIdOf::convert(who.clone())
                .filter(T::CollatorRegistration::is_registered);

            let collator_id = maybe_collator_id.ok_or(Error::<T>::NoKeysRegistered)?;

            <Invulnerables<T>>::try_mutate(|invulnerables| -> DispatchResult {
                if invulnerables.contains(&collator_id) {
                    Err(Error::<T>::AlreadyInvulnerable)?;
                }
                invulnerables
                    .try_push(collator_id.clone())
                    .map_err(|_| Error::<T>::TooManyInvulnerables)?;
                Ok(())
            })?;

            Self::deposit_event(Event::InvulnerableAdded { account_id: who });

            let weight_used = T::WeightInfo::add_invulnerable(
                Invulnerables::<T>::decode_len()
                    .unwrap_or_default()
                    .try_into()
                    .unwrap_or(T::MaxInvulnerables::get().saturating_sub(1)),
            );

            Ok(Some(weight_used).into())
        }

        /// Remove an account `who` from the list of `Invulnerables` collators. `Invulnerables` must
        /// be sorted.
        ///
        /// The origin for this call must be the `UpdateOrigin`.
        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::remove_invulnerable(T::MaxInvulnerables::get()))]
        pub fn remove_invulnerable(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
            T::UpdateOrigin::ensure_origin(origin)?;

            let collator_id = T::CollatorIdOf::convert(who.clone())
                .ok_or(Error::<T>::UnableToDeriveCollatorId)?;

            <Invulnerables<T>>::try_mutate(|invulnerables| -> DispatchResult {
                let pos = invulnerables
                    .iter()
                    .position(|x| x == &collator_id)
                    .ok_or(Error::<T>::NotInvulnerable)?;
                invulnerables.remove(pos);
                Ok(())
            })?;

            Self::deposit_event(Event::InvulnerableRemoved { account_id: who });
            Ok(())
        }
    }

    /// Play the role of the session manager.
    impl<T: Config> SessionManager<T::CollatorId> for Pallet<T> {
        fn new_session(index: SessionIndex) -> Option<Vec<T::CollatorId>> {
            log::info!(
                "assembling new invulnerable collators for new session {} at #{:?}",
                index,
                <frame_system::Pallet<T>>::block_number(),
            );

            let invulnerables = Self::invulnerables().to_vec();
            frame_system::Pallet::<T>::register_extra_weight_unchecked(
                T::WeightInfo::new_session(invulnerables.len() as u32),
                DispatchClass::Mandatory,
            );
            Some(invulnerables)
        }
        fn start_session(_: SessionIndex) {
            // we don't care.
        }
        fn end_session(_: SessionIndex) {
            // we don't care.
        }
    }
}

/// If the rewarded account is an Invulnerable, distribute the entire reward
/// amount to them. Otherwise use the `Fallback` distribution.
pub struct InvulnerableRewardDistribution<Runtime, Currency, Fallback>(
    PhantomData<(Runtime, Currency, Fallback)>,
);

use {frame_support::pallet_prelude::Weight, sp_runtime::traits::Get};

type CreditOf<Runtime, Currency> =
    frame_support::traits::fungible::Credit<<Runtime as frame_system::Config>::AccountId, Currency>;
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

impl<Runtime, Currency, Fallback>
    tp_traits::DistributeRewards<AccountIdOf<Runtime>, CreditOf<Runtime, Currency>>
    for InvulnerableRewardDistribution<Runtime, Currency, Fallback>
where
    Runtime: frame_system::Config + Config,
    Fallback: tp_traits::DistributeRewards<AccountIdOf<Runtime>, CreditOf<Runtime, Currency>>,
    Currency: frame_support::traits::fungible::Balanced<AccountIdOf<Runtime>>,
{
    fn distribute_rewards(
        rewarded: AccountIdOf<Runtime>,
        amount: CreditOf<Runtime, Currency>,
    ) -> frame_support::pallet_prelude::DispatchResultWithPostInfo {
        let mut total_weight = Weight::zero();
        let collator_id = Runtime::CollatorIdOf::convert(rewarded.clone())
            .ok_or(Error::<Runtime>::UnableToDeriveCollatorId)?;
        // weight to read invulnerables
        total_weight += Runtime::DbWeight::get().reads(1);
        if !Invulnerables::<Runtime>::get().contains(&collator_id) {
            let post_info = Fallback::distribute_rewards(rewarded, amount)?;
            if let Some(weight) = post_info.actual_weight {
                total_weight += weight;
            }
        } else {
            Currency::resolve(&rewarded, amount).map_err(|_| TokenError::NotExpendable)?;
            total_weight +=
                Runtime::WeightInfo::reward_invulnerable(Runtime::MaxInvulnerables::get())
        }
        Ok(Some(total_weight).into())
    }
}
