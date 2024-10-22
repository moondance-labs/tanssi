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

//! ExternalValidators pallet.
//!
//! A pallet to manage external validators for a solochain.
//!
//! ## Terminology
//!
//! - WhitelistedValidators: Fixed validators set by root/governance. Have priority over the external validators.
//! - ExternalValidators: Validators set using storage proofs from another blockchain. Changing them triggers a
//!     new era. Can be disabled by setting `SkipExternalValidators` to true.
//!
//! Validators only change once per era. By default the era changes after a fixed number of sessions, but that can
//! be changed by root/governance to increase era on every session, or to never change.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use {
    frame_support::{dispatch::DispatchClass, pallet_prelude::Weight},
    parity_scale_codec::{Decode, Encode, MaxEncodedLen},
    scale_info::TypeInfo,
    sp_runtime::traits::Get,
    sp_runtime::RuntimeDebug,
    sp_staking::SessionIndex,
    sp_std::vec::Vec,
    tp_traits::{
        ActiveEraInfo, EraIndex, EraIndexProvider, OnEraEnd, OnEraStart, ValidatorProvider,
    },
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
        super::*,
        frame_support::{
            dispatch::DispatchResultWithPostInfo,
            pallet_prelude::*,
            traits::{EnsureOrigin, UnixTime, ValidatorRegistration},
            BoundedVec, DefaultNoBound,
        },
        frame_system::pallet_prelude::*,
        sp_runtime::{traits::Convert, SaturatedConversion},
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

        /// Maximum number of whitelisted validators.
        #[pallet::constant]
        type MaxWhitelistedValidators: Get<u32>;

        /// Maximum number of external validators.
        #[pallet::constant]
        type MaxExternalValidators: Get<u32>;

        /// A stable ID for a validator.
        type ValidatorId: Member
            + Parameter
            + MaybeSerializeDeserialize
            + MaxEncodedLen
            + TryFrom<Self::AccountId>;

        /// A conversion from account ID to validator ID.
        ///
        /// Its cost must be at most one storage read.
        type ValidatorIdOf: Convert<Self::AccountId, Option<Self::ValidatorId>>;

        /// Validate a user is registered
        type ValidatorRegistration: ValidatorRegistration<Self::ValidatorId>;

        /// Time used for computing era duration.
        ///
        /// It is guaranteed to start being called from the first `on_finalize`. Thus value at
        /// genesis is not used.
        type UnixTime: UnixTime;

        /// Number of sessions per era.
        #[pallet::constant]
        type SessionsPerEra: Get<SessionIndex>;

        type OnEraStart: OnEraStart;
        type OnEraEnd: OnEraEnd;

        /// The weight information of this pallet.
        type WeightInfo: WeightInfo;

        #[cfg(feature = "runtime-benchmarks")]
        type Currency: Currency<Self::AccountId>
            + frame_support::traits::fungible::Balanced<Self::AccountId>;
    }

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    /// Fixed validators set by root/governance. Have priority over the external validators.
    #[pallet::storage]
    pub type WhitelistedValidators<T: Config> =
        StorageValue<_, BoundedVec<T::ValidatorId, T::MaxWhitelistedValidators>, ValueQuery>;

    /// Validators set using storage proofs from another blockchain. Ignored if `SkipExternalValidators` is true.
    #[pallet::storage]
    pub type ExternalValidators<T: Config> =
        StorageValue<_, BoundedVec<T::ValidatorId, T::MaxExternalValidators>, ValueQuery>;

    /// Allow to disable external validators.
    #[pallet::storage]
    pub type SkipExternalValidators<T: Config> = StorageValue<_, bool, ValueQuery>;

    /// The active era information, it holds index and start.
    #[pallet::storage]
    pub type ActiveEra<T: Config> = StorageValue<_, ActiveEraInfo>;

    /// Session index at the start of this era. Used to know when to start the next era.
    #[pallet::storage]
    pub type EraSessionStart<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// Mode of era forcing.
    #[pallet::storage]
    pub type ForceEra<T> = StorageValue<_, Forcing, ValueQuery>;

    #[pallet::genesis_config]
    #[derive(DefaultNoBound)]
    pub struct GenesisConfig<T: Config> {
        pub whitelisted_validators: Vec<T::ValidatorId>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            let duplicate_validators = self
                .whitelisted_validators
                .iter()
                // T::ValidatorId does not impl Ord or Hash so we cannot collect into set directly,
                // but we can check for duplicates if we encode them first.
                .map(|x| x.encode())
                .collect::<sp_std::collections::btree_set::BTreeSet<_>>();
            assert!(
                duplicate_validators.len() == self.whitelisted_validators.len(),
                "duplicate validators in genesis."
            );

            let bounded_validators = BoundedVec::<_, T::MaxWhitelistedValidators>::try_from(
                self.whitelisted_validators.clone(),
            )
            .expect("genesis validators are more than T::MaxWhitelistedValidators");

            <WhitelistedValidators<T>>::put(bounded_validators);
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new Invulnerable was added.
        WhitelistedValidatorAdded { account_id: T::AccountId },
        /// An Invulnerable was removed.
        WhitelistedValidatorRemoved { account_id: T::AccountId },
        /// A new era has started.
        NewEra { era: EraIndex },
        /// A new force era mode was set.
        ForceEra { mode: Forcing },
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
        /// Allow to ignore external validators and use only whitelisted ones.
        ///
        /// The origin for this call must be the `UpdateOrigin`.
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::skip_external_validators())]
        pub fn skip_external_validators(origin: OriginFor<T>, skip: bool) -> DispatchResult {
            T::UpdateOrigin::ensure_origin(origin)?;

            <SkipExternalValidators<T>>::put(skip);

            Ok(())
        }

        /// Add a new account `who` to the list of `WhitelistedValidators`.
        ///
        /// The origin for this call must be the `UpdateOrigin`.
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::add_whitelisted(
			T::MaxWhitelistedValidators::get().saturating_sub(1),
		))]
        pub fn add_whitelisted(
            origin: OriginFor<T>,
            who: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            T::UpdateOrigin::ensure_origin(origin)?;
            // don't let one unprepared collator ruin things for everyone.
            let maybe_collator_id = T::ValidatorIdOf::convert(who.clone())
                .filter(T::ValidatorRegistration::is_registered);

            let collator_id = maybe_collator_id.ok_or(Error::<T>::NoKeysRegistered)?;

            <WhitelistedValidators<T>>::try_mutate(|invulnerables| -> DispatchResult {
                if invulnerables.contains(&collator_id) {
                    Err(Error::<T>::AlreadyInvulnerable)?;
                }
                invulnerables
                    .try_push(collator_id.clone())
                    .map_err(|_| Error::<T>::TooManyInvulnerables)?;
                Ok(())
            })?;

            Self::deposit_event(Event::WhitelistedValidatorAdded { account_id: who });

            let weight_used = <T as Config>::WeightInfo::add_whitelisted(
                WhitelistedValidators::<T>::decode_len()
                    .unwrap_or_default()
                    .try_into()
                    .unwrap_or(T::MaxWhitelistedValidators::get().saturating_sub(1)),
            );

            Ok(Some(weight_used).into())
        }

        /// Remove an account `who` from the list of `WhitelistedValidators` collators.
        ///
        /// The origin for this call must be the `UpdateOrigin`.
        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::remove_whitelisted(T::MaxWhitelistedValidators::get()))]
        pub fn remove_whitelisted(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
            T::UpdateOrigin::ensure_origin(origin)?;

            let collator_id = T::ValidatorIdOf::convert(who.clone())
                .ok_or(Error::<T>::UnableToDeriveCollatorId)?;

            <WhitelistedValidators<T>>::try_mutate(|invulnerables| -> DispatchResult {
                let pos = invulnerables
                    .iter()
                    .position(|x| x == &collator_id)
                    .ok_or(Error::<T>::NotInvulnerable)?;
                invulnerables.remove(pos);
                Ok(())
            })?;

            Self::deposit_event(Event::WhitelistedValidatorRemoved { account_id: who });
            Ok(())
        }

        /// Force there to be no new eras indefinitely.
        ///
        /// The dispatch origin must be Root.
        ///
        /// # Warning
        ///
        /// The election process starts multiple blocks before the end of the era.
        /// Thus the election process may be ongoing when this is called. In this case the
        /// election will continue until the next era is triggered.
        ///
        /// ## Complexity
        /// - No arguments.
        /// - Weight: O(1)
        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::force_no_eras())]
        pub fn force_no_eras(origin: OriginFor<T>) -> DispatchResult {
            T::UpdateOrigin::ensure_origin(origin)?;
            Self::set_force_era(Forcing::ForceNone);
            Ok(())
        }

        /// Force there to be a new era at the end of the next session. After this, it will be
        /// reset to normal (non-forced) behaviour.
        ///
        /// The dispatch origin must be Root.
        ///
        /// # Warning
        ///
        /// The election process starts multiple blocks before the end of the era.
        /// If this is called just before a new era is triggered, the election process may not
        /// have enough blocks to get a result.
        ///
        /// ## Complexity
        /// - No arguments.
        /// - Weight: O(1)
        #[pallet::call_index(4)]
        #[pallet::weight(T::WeightInfo::force_new_era())]
        pub fn force_new_era(origin: OriginFor<T>) -> DispatchResult {
            T::UpdateOrigin::ensure_origin(origin)?;
            Self::set_force_era(Forcing::ForceNew);
            Ok(())
        }

        /// Force there to be a new era at the end of sessions indefinitely.
        ///
        /// The dispatch origin must be Root.
        ///
        /// # Warning
        ///
        /// The election process starts multiple blocks before the end of the era.
        /// If this is called just before a new era is triggered, the election process may not
        /// have enough blocks to get a result.
        #[pallet::call_index(5)]
        #[pallet::weight(T::WeightInfo::force_new_era_always())]
        pub fn force_new_era_always(origin: OriginFor<T>) -> DispatchResult {
            T::UpdateOrigin::ensure_origin(origin)?;
            Self::set_force_era(Forcing::ForceAlways);
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn set_external_validators(validators: Vec<T::ValidatorId>) -> DispatchResult {
            // If more validators than max, take the first n
            let validators = BoundedVec::truncate_from(validators);
            <ExternalValidators<T>>::put(validators);

            Ok(())
        }

        pub(crate) fn increase_era(new_index: SessionIndex) {
            // Increase era
            <ActiveEra<T>>::mutate(|q| {
                if q.is_none() {
                    *q = Some(ActiveEraInfo {
                        index: 0,
                        start: None,
                    });
                }

                let q = q.as_mut().unwrap();
                q.index += 1;

                Self::deposit_event(Event::NewEra { era: q.index });

                // Set new active era start in next `on_finalize`. To guarantee usage of `Time`
                q.start = None;
            });
            <EraSessionStart<T>>::put(new_index);
        }

        /// Helper to set a new `ForceEra` mode.
        pub(crate) fn set_force_era(mode: Forcing) {
            log::info!("Setting force era mode {:?}.", mode);
            ForceEra::<T>::put(mode);
            Self::deposit_event(Event::<T>::ForceEra { mode });
        }

        pub fn whitelisted_validators() -> Vec<T::ValidatorId> {
            <WhitelistedValidators<T>>::get().into()
        }

        pub fn current_era() -> Option<u32> {
            <ActiveEra<T>>::get().map(|era_info| era_info.index)
        }

        pub fn validators() -> Vec<T::ValidatorId> {
            let mut validators: Vec<_> = WhitelistedValidators::<T>::get().into();

            if !SkipExternalValidators::<T>::get() {
                validators.extend(ExternalValidators::<T>::get())
            }

            validators
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_now: BlockNumberFor<T>) -> Weight {
            // just return the weight of the on_finalize.
            T::DbWeight::get().reads(1)
        }

        fn on_finalize(_n: BlockNumberFor<T>) {
            // Set the start of the first era.
            if let Some(mut active_era) = <ActiveEra<T>>::get() {
                if active_era.start.is_none() {
                    let now_as_millis_u64 = T::UnixTime::now().as_millis().saturated_into::<u64>();
                    active_era.start = Some(now_as_millis_u64);
                    // This write only ever happens once, we don't include it in the weight in
                    // general
                    ActiveEra::<T>::put(active_era);
                }
            }
            // `on_finalize` weight is tracked in `on_initialize`
        }
    }
}

impl<T: Config> pallet_session::SessionManager<T::ValidatorId> for Pallet<T> {
    fn new_session(new_index: SessionIndex) -> Option<Vec<T::ValidatorId>> {
        if new_index <= 1 {
            return None;
        }

        let new_era = match <ForceEra<T>>::get() {
            Forcing::NotForcing => {
                // If enough sessions have elapsed, start new era
                let start_session = <EraSessionStart<T>>::get();
                let current_session = new_index;

                if current_session.saturating_sub(start_session) >= T::SessionsPerEra::get() {
                    true
                } else {
                    false
                }
            }
            Forcing::ForceNew => {
                // Only force once
                <ForceEra<T>>::put(Forcing::NotForcing);

                true
            }
            Forcing::ForceNone => false,
            Forcing::ForceAlways => true,
        };

        if !new_era {
            // If not starting a new era, keep the previous validators
            return None;
        }

        Self::increase_era(new_index);

        T::OnEraStart::on_era_start();

        let validators: Vec<_> = Self::validators();

        frame_system::Pallet::<T>::register_extra_weight_unchecked(
            T::WeightInfo::new_session(validators.len() as u32),
            DispatchClass::Mandatory,
        );

        Some(validators)
    }

    fn end_session(index: SessionIndex) {
        let new_index = index.saturating_add(1);

        if new_index <= 1 {
            return;
        }

        let new_era = match <ForceEra<T>>::get() {
            Forcing::NotForcing => {
                // If enough sessions have elapsed, start new era
                let start_session = <EraSessionStart<T>>::get();
                let current_session = new_index;

                if current_session.saturating_sub(start_session) >= T::SessionsPerEra::get() {
                    true
                } else {
                    false
                }
            }
            Forcing::ForceNew => true,
            Forcing::ForceNone => false,
            Forcing::ForceAlways => true,
        };

        if !new_era {
            return;
        }

        T::OnEraEnd::on_era_end();
    }

    fn start_session(_start_index: SessionIndex) {}
}

impl<T: Config> pallet_session::historical::SessionManager<T::ValidatorId, ()> for Pallet<T> {
    fn new_session(new_index: SessionIndex) -> Option<Vec<(T::ValidatorId, ())>> {
        <Self as pallet_session::SessionManager<_>>::new_session(new_index)
            .map(|r| r.into_iter().map(|v| (v, Default::default())).collect())
    }

    fn start_session(start_index: SessionIndex) {
        <Self as pallet_session::SessionManager<_>>::start_session(start_index)
    }

    fn end_session(end_index: SessionIndex) {
        <Self as pallet_session::SessionManager<_>>::end_session(end_index)
    }
}

impl<T: Config> EraIndexProvider for Pallet<T> {
    fn active_era() -> ActiveEraInfo {
        <ActiveEra<T>>::get().unwrap_or(ActiveEraInfo {
            index: 0,
            start: None,
        })
    }
}

impl<T: Config> ValidatorProvider<T::ValidatorId> for Pallet<T> {
    fn validators() -> Vec<T::ValidatorId> {
        Self::validators()
    }
}

/// Mode of era-forcing.
#[derive(
    Copy, Clone, PartialEq, Eq, Default, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen,
)]
pub enum Forcing {
    /// Not forcing anything - just let whatever happen.
    #[default]
    NotForcing,
    /// Force a new era, then reset to `NotForcing` as soon as it is done.
    ForceNew,
    /// Avoid a new era indefinitely.
    ForceNone,
    /// Force a new era at the end of all sessions indefinitely.
    ForceAlways,
}
