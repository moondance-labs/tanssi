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
        type MaxInvulnerables: Get<u32>;

        /// A stable ID for a collator.
        type CollatorId: Member + Parameter;

        /// A conversion from account ID to collator ID.
        ///
        /// Its cost must be at most one storage read.
        type CollatorIdOf: Convert<Self::AccountId, Option<Self::CollatorId>>;

        /// Validate a user is registered
        type CollatorRegistration: ValidatorRegistration<Self::CollatorId>;

        /// The weight information of this pallet.
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    /// The invulnerable, permissioned collators. This list must be sorted.
    #[pallet::storage]
    #[pallet::getter(fn invulnerables)]
    pub type Invulnerables<T: Config> =
        StorageValue<_, BoundedVec<T::AccountId, T::MaxInvulnerables>, ValueQuery>;

    #[pallet::genesis_config]
    #[derive(DefaultNoBound)]
    pub struct GenesisConfig<T: Config> {
        pub invulnerables: Vec<T::AccountId>,
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
        NewInvulnerables { invulnerables: Vec<T::AccountId> },
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
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Set the list of invulnerable (fixed) collators.
        ///
        /// Must be called by the `UpdateOrigin`.
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::set_invulnerables(new.len() as u32))]
        pub fn set_invulnerables(origin: OriginFor<T>, new: Vec<T::AccountId>) -> DispatchResult {
            T::UpdateOrigin::ensure_origin(origin)?;

            // Will need to check the length again when putting into a bounded vec, but this
            // prevents the iterator from having too many elements.
            ensure!(
                new.len() as u32 <= T::MaxInvulnerables::get(),
                Error::<T>::TooManyInvulnerables
            );

            let mut new_with_keys = Vec::new();

            // check if the invulnerables have associated validator keys before they are set
            for account_id in &new {
                // don't let one unprepared collator ruin things for everyone.
                let collator_id = T::CollatorIdOf::convert(account_id.clone());
                let is_valid =
                    collator_id.map_or(false, |key| T::CollatorRegistration::is_registered(&key));
                if is_valid {
                    new_with_keys.push(account_id.clone());
                } else {
                    Self::deposit_event(Event::InvalidInvulnerableSkipped {
                        account_id: account_id.clone(),
                    });
                }
            }

            // should never fail since `new` must be equal to or shorter than `TooManyInvulnerables`
            let bounded_invulnerables =
                BoundedVec::<_, T::MaxInvulnerables>::try_from(new_with_keys)
                    .map_err(|_| Error::<T>::TooManyInvulnerables)?;

            <Invulnerables<T>>::put(&bounded_invulnerables);
            Self::deposit_event(Event::NewInvulnerables {
                invulnerables: bounded_invulnerables.to_vec(),
            });

            Ok(())
        }

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

            <Invulnerables<T>>::try_mutate(|invulnerables| -> DispatchResult {
                if invulnerables.contains(&who) {
                    Err(Error::<T>::AlreadyInvulnerable)?;
                }
                invulnerables
                    .try_push(who.clone())
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

            <Invulnerables<T>>::try_mutate(|invulnerables| -> DispatchResult {
                let pos = invulnerables
                    .iter()
                    .position(|x| x == &who)
                    .ok_or(Error::<T>::NotInvulnerable)?;
                invulnerables.remove(pos);
                Ok(())
            })?;

            Self::deposit_event(Event::InvulnerableRemoved { account_id: who });
            Ok(())
        }
    }

    /// Play the role of the session manager.
    impl<T: Config> SessionManager<T::AccountId> for Pallet<T> {
        fn new_session(index: SessionIndex) -> Option<Vec<T::AccountId>> {
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
