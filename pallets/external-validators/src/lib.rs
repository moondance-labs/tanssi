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

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::dispatch::DispatchClass;
pub use pallet::*;
use {
    core::marker::PhantomData,
    sp_runtime::{traits::Convert, TokenError},
    sp_staking::SessionIndex,
    sp_std::vec::Vec,
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
    pub type WhitelistedValidators<T: Config> =
        StorageValue<_, BoundedVec<T::ValidatorId, T::MaxWhitelistedValidators>, ValueQuery>;

    /// The invulnerable, permissioned collators. This list must be sorted.
    #[pallet::storage]
    pub type ExternalValidators<T: Config> =
        StorageValue<_, BoundedVec<T::ValidatorId, T::MaxExternalValidators>, ValueQuery>;

    /// The invulnerable, permissioned collators. This list must be sorted.
    #[pallet::storage]
    pub type SkipExternalValidators<T: Config> = StorageValue<_, bool, ValueQuery>;

    /// The invulnerable, permissioned collators. This list must be sorted.
    #[pallet::storage]
    pub type ActiveEra<T: Config> = StorageValue<_, ActiveEraInfo>;

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
    }

    impl<T: Config> Pallet<T> {
        pub fn set_external_validators(validators: Vec<T::ValidatorId>) -> DispatchResult {
            // If more validators than max, take the first n
            let validators = BoundedVec::truncate_from(validators);
            <ExternalValidators<T>>::put(validators);

            Self::increase_era();

            Ok(())
        }

        fn increase_era() {
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
                // Set new active era start in next `on_finalize`. To guarantee usage of `Time`
                q.start = None;
            });
        }

        pub fn whitelisted_validators() -> Vec<T::ValidatorId> {
            <WhitelistedValidators<T>>::get().into()
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
        let collator_id = Runtime::ValidatorIdOf::convert(rewarded.clone())
            .ok_or(Error::<Runtime>::UnableToDeriveCollatorId)?;
        // weight to read invulnerables
        total_weight += Runtime::DbWeight::get().reads(1);
        if !WhitelistedValidators::<Runtime>::get().contains(&collator_id) {
            let post_info = Fallback::distribute_rewards(rewarded, amount)?;
            if let Some(weight) = post_info.actual_weight {
                total_weight += weight;
            }
        } else {
            Currency::resolve(&rewarded, amount).map_err(|_| TokenError::NotExpendable)?;
            total_weight += Runtime::WeightInfo::reward_validator(
                Runtime::MaxWhitelistedValidators::get() + Runtime::MaxExternalValidators::get(),
            )
        }
        Ok(Some(total_weight).into())
    }
}

impl<T: Config> pallet_session::SessionManager<T::ValidatorId> for Pallet<T> {
    fn new_session(new_index: SessionIndex) -> Option<Vec<T::ValidatorId>> {
        if new_index <= 1 {
            return None;
        }

        let mut validators: Vec<_> = WhitelistedValidators::<T>::get().into();

        if !SkipExternalValidators::<T>::get() {
            validators.extend(ExternalValidators::<T>::get())
        }

        frame_system::Pallet::<T>::register_extra_weight_unchecked(
            T::WeightInfo::new_session(validators.len() as u32),
            DispatchClass::Mandatory,
        );

        Some(validators)
    }

    fn end_session(_: SessionIndex) {}

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
