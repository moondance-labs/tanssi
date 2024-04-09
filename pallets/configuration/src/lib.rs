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

//! # Configuration Pallet
//!
//! This pallet stores the configuration for an orchestration-collator assignation chain. In
//! particular stores:
//!
//!    - How many collators are taken.
//!    - How many of those collators should be serving the orchestrator chain
//!    - Howe many of those collators should be serving the containerChains
//!
//! All configuration changes are protected behind the root origin
//! CHanges to the configuration are not immeditaly applied, but rather we wait
//! T::SessionDelay to apply these changes

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
pub mod weights;

pub use weights::WeightInfo;

#[cfg(any(test, feature = "runtime-benchmarks"))]
mod benchmarks;

pub use pallet::*;
use {
    frame_support::pallet_prelude::*,
    frame_system::pallet_prelude::*,
    serde::{Deserialize, Serialize},
    sp_runtime::{traits::AtLeast32BitUnsigned, Perbill, RuntimeAppPublic, Saturating},
    sp_std::prelude::*,
    tp_traits::GetSessionIndex,
};

const LOG_TARGET: &str = "pallet_configuration";

/// All configuration of the runtime with respect to parachains and parathreads.
#[derive(
    Clone,
    Encode,
    Decode,
    PartialEq,
    sp_core::RuntimeDebug,
    scale_info::TypeInfo,
    Serialize,
    Deserialize,
)]
pub struct HostConfiguration {
    /// Maximum number of collators, in total, including orchestrator and containers
    pub max_collators: u32,
    /// Minimum number of collators to be assigned to orchestrator chain
    pub min_orchestrator_collators: u32,
    /// Maximum number of collators to be assigned to orchestrator chain after all the container chains have been
    /// assigned collators.
    pub max_orchestrator_collators: u32,
    /// How many collators to assign to one container chain
    pub collators_per_container: u32,
    /// Rotate all collators once every n sessions. If this value is 0 means that there is no rotation
    pub full_rotation_period: u32,
    /// How many collators to assign to one parathread
    // TODO: for now we only support 1 collator per parathread because using Aura for consensus conflicts with
    // the idea of being able to create blocks every n slots: if there are 2 collators and we create blocks
    // every 2 slots, 1 collator will create all the blocks.
    pub collators_per_parathread: u32,
    /// How many parathreads can be assigned to one collator
    pub parathreads_per_collator: u32,
    /// Ratio of collators that we expect to be assigned to container chains. Affects fees.
    pub target_container_chain_fullness: Perbill,
}

impl Default for HostConfiguration {
    fn default() -> Self {
        Self {
            max_collators: 100u32,
            min_orchestrator_collators: 2u32,
            max_orchestrator_collators: 5u32,
            collators_per_container: 2u32,
            full_rotation_period: 24u32,
            collators_per_parathread: 1,
            parathreads_per_collator: 1,
            target_container_chain_fullness: Perbill::from_percent(80),
        }
    }
}

/// Enumerates the possible inconsistencies of `HostConfiguration`.
#[derive(Debug)]
pub enum InconsistentError {
    /// `max_orchestrator_collators` is lower than `min_orchestrator_collators`
    MaxCollatorsLowerThanMinCollators,
    /// `min_orchestrator_collators` must be at least 1
    MinOrchestratorCollatorsTooLow,
    /// `max_collators` must be at least 1
    MaxCollatorsTooLow,
    /// Tried to modify an unimplemented parameter
    UnimplementedParameter,
}

impl HostConfiguration {
    /// Checks that this instance is consistent with the requirements on each individual member.
    ///
    /// # Errors
    ///
    /// This function returns an error if the configuration is inconsistent.
    pub fn check_consistency(&self) -> Result<(), InconsistentError> {
        if self.max_collators < 1 {
            return Err(InconsistentError::MaxCollatorsTooLow);
        }
        if self.min_orchestrator_collators < 1 {
            return Err(InconsistentError::MinOrchestratorCollatorsTooLow);
        }
        if self.max_orchestrator_collators < self.min_orchestrator_collators {
            return Err(InconsistentError::MaxCollatorsLowerThanMinCollators);
        }
        if self.parathreads_per_collator != 1 {
            return Err(InconsistentError::UnimplementedParameter);
        }
        if self.max_collators < self.min_orchestrator_collators {
            return Err(InconsistentError::MaxCollatorsLowerThanMinCollators);
        }
        Ok(())
    }

    /// Checks that this instance is consistent with the requirements on each individual member.
    ///
    /// # Panics
    ///
    /// This function panics if the configuration is inconsistent.
    pub fn panic_if_not_consistent(&self) {
        if let Err(err) = self.check_consistency() {
            panic!("Host configuration is inconsistent: {:?}", err);
        }
    }
}

#[frame_support::pallet]
pub mod pallet {
    use tp_traits::GetHostConfiguration;

    use super::*;

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type SessionIndex: parity_scale_codec::FullCodec + TypeInfo + Copy + AtLeast32BitUnsigned;

        // `SESSION_DELAY` is used to delay any changes to Paras registration or configurations.
        // Wait until the session index is 2 larger then the current index to apply any changes,
        // which guarantees that at least one full session has passed before any changes are applied.
        #[pallet::constant]
        type SessionDelay: Get<Self::SessionIndex>;

        type CurrentSessionIndex: GetSessionIndex<Self::SessionIndex>;

        /// The identifier type for an authority.
        type AuthorityId: Member
            + Parameter
            + RuntimeAppPublic
            + MaybeSerializeDeserialize
            + MaxEncodedLen;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The new value for a configuration parameter is invalid.
        InvalidNewValue,
    }

    /// The active configuration for the current session.
    #[pallet::storage]
    #[pallet::getter(fn config)]
    pub(crate) type ActiveConfig<T: Config> = StorageValue<_, HostConfiguration, ValueQuery>;

    /// Pending configuration changes.
    ///
    /// This is a list of configuration changes, each with a session index at which it should
    /// be applied.
    ///
    /// The list is sorted ascending by session index. Also, this list can only contain at most
    /// 2 items: for the next session and for the `scheduled_session`.
    #[pallet::storage]
    #[pallet::getter(fn pending_configs)]
    pub(crate) type PendingConfigs<T: Config> =
        StorageValue<_, Vec<(T::SessionIndex, HostConfiguration)>, ValueQuery>;

    /// If this is set, then the configuration setters will bypass the consistency checks. This
    /// is meant to be used only as the last resort.
    #[pallet::storage]
    pub(crate) type BypassConsistencyCheck<T: Config> = StorageValue<_, bool, ValueQuery>;

    #[pallet::genesis_config]
    #[derive(frame_support::DefaultNoBound)]
    pub struct GenesisConfig<T: Config> {
        pub config: HostConfiguration,
        #[serde(skip)]
        pub _config: sp_std::marker::PhantomData<T>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            self.config.panic_if_not_consistent();
            ActiveConfig::<T>::put(&self.config);
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight((
			T::WeightInfo::set_config_with_u32(),
			DispatchClass::Operational,
		))]
        pub fn set_max_collators(origin: OriginFor<T>, new: u32) -> DispatchResult {
            ensure_root(origin)?;
            Self::schedule_config_update(|config| {
                config.max_collators = new;
            })
        }

        #[pallet::call_index(1)]
        #[pallet::weight((
			T::WeightInfo::set_config_with_u32(),
			DispatchClass::Operational,
		))]
        pub fn set_min_orchestrator_collators(origin: OriginFor<T>, new: u32) -> DispatchResult {
            ensure_root(origin)?;
            Self::schedule_config_update(|config| {
                if config.max_orchestrator_collators < new {
                    config.max_orchestrator_collators = new;
                }
                config.min_orchestrator_collators = new;
            })
        }

        #[pallet::call_index(2)]
        #[pallet::weight((
			T::WeightInfo::set_config_with_u32(),
			DispatchClass::Operational,
		))]
        pub fn set_max_orchestrator_collators(origin: OriginFor<T>, new: u32) -> DispatchResult {
            ensure_root(origin)?;
            Self::schedule_config_update(|config| {
                if config.min_orchestrator_collators > new {
                    config.min_orchestrator_collators = new;
                }
                config.max_orchestrator_collators = new;
            })
        }

        #[pallet::call_index(3)]
        #[pallet::weight((
			T::WeightInfo::set_config_with_u32(),
			DispatchClass::Operational,
		))]
        pub fn set_collators_per_container(origin: OriginFor<T>, new: u32) -> DispatchResult {
            ensure_root(origin)?;
            Self::schedule_config_update(|config| {
                config.collators_per_container = new;
            })
        }

        #[pallet::call_index(4)]
        #[pallet::weight((
			T::WeightInfo::set_config_with_u32(),
			DispatchClass::Operational,
		))]
        pub fn set_full_rotation_period(origin: OriginFor<T>, new: u32) -> DispatchResult {
            ensure_root(origin)?;
            Self::schedule_config_update(|config| {
                config.full_rotation_period = new;
            })
        }

        #[pallet::call_index(5)]
        #[pallet::weight((
        T::WeightInfo::set_config_with_u32(),
        DispatchClass::Operational,
        ))]
        pub fn set_collators_per_parathread(origin: OriginFor<T>, new: u32) -> DispatchResult {
            ensure_root(origin)?;
            Self::schedule_config_update(|config| {
                config.collators_per_parathread = new;
            })
        }

        #[pallet::call_index(6)]
        #[pallet::weight((
        T::WeightInfo::set_config_with_u32(),
        DispatchClass::Operational,
        ))]
        pub fn set_parathreads_per_collator(origin: OriginFor<T>, new: u32) -> DispatchResult {
            ensure_root(origin)?;
            Self::schedule_config_update(|config| {
                config.parathreads_per_collator = new;
            })
        }

        #[pallet::call_index(7)]
        #[pallet::weight((
        T::WeightInfo::set_config_with_u32(),
        DispatchClass::Operational,
        ))]
        pub fn set_target_container_chain_fullness(
            origin: OriginFor<T>,
            new: Perbill,
        ) -> DispatchResult {
            ensure_root(origin)?;
            Self::schedule_config_update(|config| {
                config.target_container_chain_fullness = new;
            })
        }

        /// Setting this to true will disable consistency checks for the configuration setters.
        /// Use with caution.
        #[pallet::call_index(44)]
        #[pallet::weight((
			T::DbWeight::get().writes(1),
			DispatchClass::Operational,
		))]
        pub fn set_bypass_consistency_check(origin: OriginFor<T>, new: bool) -> DispatchResult {
            ensure_root(origin)?;
            BypassConsistencyCheck::<T>::put(new);
            Ok(())
        }
    }

    /// A struct that holds the configuration that was active before the session change and optionally
    /// a configuration that became active after the session change.
    pub struct SessionChangeOutcome {
        /// Previously active configuration.
        pub prev_config: HostConfiguration,
        /// If new configuration was applied during the session change, this is the new configuration.
        pub new_config: Option<HostConfiguration>,
    }

    impl<T: Config> Pallet<T> {
        /// Called by the initializer to note that a new session has started.
        ///
        /// Returns the configuration that was actual before the session change and the configuration
        /// that became active after the session change. If there were no scheduled changes, both will
        /// be the same.
        pub fn initializer_on_new_session(session_index: &T::SessionIndex) -> SessionChangeOutcome {
            let pending_configs = <PendingConfigs<T>>::get();
            let prev_config = ActiveConfig::<T>::get();

            // No pending configuration changes, so we're done.
            if pending_configs.is_empty() {
                return SessionChangeOutcome {
                    prev_config,
                    new_config: None,
                };
            }

            // We partition those configs scheduled for the present
            // and those for the future
            let (mut past_and_present, future) = pending_configs
                .into_iter()
                .partition::<Vec<_>, _>(|&(apply_at_session, _)| {
                    apply_at_session <= *session_index
                });

            if past_and_present.len() > 1 {
                // This should never happen since we schedule configuration changes only into the future
                // sessions and this handler called for each session change.
                log::error!(
                    target: LOG_TARGET,
                    "Skipping applying configuration changes scheduled sessions in the past",
                );
            }

            let new_config = past_and_present.pop().map(|(_, config)| config);
            if let Some(ref new_config) = new_config {
                // Apply the new configuration.
                ActiveConfig::<T>::put(new_config);
            }

            // We insert future as PendingConfig
            <PendingConfigs<T>>::put(future);

            SessionChangeOutcome {
                prev_config,
                new_config,
            }
        }

        /// Return the session index that should be used for any future scheduled changes.
        fn scheduled_session() -> T::SessionIndex {
            T::CurrentSessionIndex::session_index().saturating_add(T::SessionDelay::get())
        }

        /// Forcibly set the active config. This should be used with extreme care, and typically
        /// only when enabling parachains runtime pallets for the first time on a chain which has
        /// been running without them.
        pub fn force_set_active_config(config: HostConfiguration) {
            ActiveConfig::<T>::set(config);
        }

        /// This function should be used to update members of the configuration.
        ///
        /// This function is used to update the configuration in a way that is safe. It will check the
        /// resulting configuration and ensure that the update is valid. If the update is invalid, it
        /// will check if the previous configuration was valid. If it was invalid, we proceed with
        /// updating the configuration, giving a chance to recover from such a condition.
        ///
        /// The actual configuration change take place after a couple of sessions have passed. In case
        /// this function is called more than once in a session, then the pending configuration change
        /// will be updated and the changes will be applied at once.
        // NOTE: Explicitly tell rustc not to inline this because otherwise heuristics note the incoming
        // closure making it's attractive to inline. However, in this case, we will end up with lots of
        // duplicated code (making this function to show up in the top of heaviest functions) only for
        // the sake of essentially avoiding an indirect call. Doesn't worth it.
        #[inline(never)]
        fn schedule_config_update(updater: impl FnOnce(&mut HostConfiguration)) -> DispatchResult {
            let mut pending_configs = <PendingConfigs<T>>::get();

            // 1. pending_configs = []
            //    No pending configuration changes.
            //
            //    That means we should use the active config as the base configuration. We will insert
            //    the new pending configuration as (cur+2, new_config) into the list.
            //
            // 2. pending_configs = [(cur+2, X)]
            //    There is a configuration that is pending for the scheduled session.
            //
            //    We will use X as the base configuration. We can update the pending configuration X
            //    directly.
            //
            // 3. pending_configs = [(cur+1, X)]
            //    There is a pending configuration scheduled and it will be applied in the next session.
            //
            //    We will use X as the base configuration. We need to schedule a new configuration change
            //    for the `scheduled_session` and use X as the base for the new configuration.
            //
            // 4. pending_configs = [(cur+1, X), (cur+2, Y)]
            //    There is a pending configuration change in the next session and for the scheduled
            //    session. Due to case №3, we can be sure that Y is based on top of X. This means we
            //    can use Y as the base configuration and update Y directly.
            //
            // There cannot be (cur, X) because those are applied in the session change handler for the
            // current session.

            // First, we need to decide what we should use as the base configuration.
            let mut base_config = pending_configs
                .last()
                .map(|(_, config)| config.clone())
                .unwrap_or_else(Self::config);
            let base_config_consistent = base_config.check_consistency().is_ok();

            // Now, we need to decide what the new configuration should be.
            // We also move the `base_config` to `new_config` to empahsize that the base config was
            // destroyed by the `updater`.
            updater(&mut base_config);
            let new_config = base_config;

            if BypassConsistencyCheck::<T>::get() {
                // This will emit a warning each configuration update if the consistency check is
                // bypassed. This is an attempt to make sure the bypass is not accidentally left on.
                log::warn!(
                    target: LOG_TARGET,
                    "Bypassing the consistency check for the configuration change!",
                );
            } else if let Err(e) = new_config.check_consistency() {
                if base_config_consistent {
                    // Base configuration is consistent and the new configuration is inconsistent.
                    // This means that the value set by the `updater` is invalid and we can return
                    // it as an error.
                    log::warn!(
                        target: LOG_TARGET,
                        "Configuration change rejected due to invalid configuration: {:?}",
                        e,
                    );
                    return Err(Error::<T>::InvalidNewValue.into());
                } else {
                    // The configuration was already broken, so we can as well proceed with the update.
                    // You cannot break something that is already broken.
                    //
                    // That will allow to call several functions and ultimately return the configuration
                    // into consistent state.
                    log::warn!(
                        target: LOG_TARGET,
                        "The new configuration is broken but the old is broken as well. Proceeding",
                    );
                }
            }

            let scheduled_session = Self::scheduled_session();

            if let Some(&mut (_, ref mut config)) = pending_configs
                .iter_mut()
                .find(|&&mut (apply_at_session, _)| apply_at_session >= scheduled_session)
            {
                *config = new_config;
            } else {
                // We are scheduling a new configuration change for the scheduled session.
                pending_configs.push((scheduled_session, new_config));
            }

            <PendingConfigs<T>>::put(pending_configs);

            Ok(())
        }
    }

    impl<T: Config> GetHostConfiguration<T::SessionIndex> for Pallet<T> {
        fn max_collators(session_index: T::SessionIndex) -> u32 {
            let (past_and_present, _) = Pallet::<T>::pending_configs()
                .into_iter()
                .partition::<Vec<_>, _>(|&(apply_at_session, _)| apply_at_session <= session_index);

            let config = if let Some(last) = past_and_present.last() {
                last.1.clone()
            } else {
                Pallet::<T>::config()
            };
            config.max_collators
        }

        fn collators_per_container(session_index: T::SessionIndex) -> u32 {
            let (past_and_present, _) = Pallet::<T>::pending_configs()
                .into_iter()
                .partition::<Vec<_>, _>(|&(apply_at_session, _)| apply_at_session <= session_index);

            let config = if let Some(last) = past_and_present.last() {
                last.1.clone()
            } else {
                Pallet::<T>::config()
            };
            config.collators_per_container
        }

        fn collators_per_parathread(session_index: T::SessionIndex) -> u32 {
            let (past_and_present, _) = Pallet::<T>::pending_configs()
                .into_iter()
                .partition::<Vec<_>, _>(|&(apply_at_session, _)| apply_at_session <= session_index);

            let config = if let Some(last) = past_and_present.last() {
                last.1.clone()
            } else {
                Pallet::<T>::config()
            };
            config.collators_per_parathread
        }

        fn min_collators_for_orchestrator(session_index: T::SessionIndex) -> u32 {
            let (past_and_present, _) = Pallet::<T>::pending_configs()
                .into_iter()
                .partition::<Vec<_>, _>(|&(apply_at_session, _)| apply_at_session <= session_index);

            let config = if let Some(last) = past_and_present.last() {
                last.1.clone()
            } else {
                Pallet::<T>::config()
            };
            config.min_orchestrator_collators
        }

        fn max_collators_for_orchestrator(session_index: T::SessionIndex) -> u32 {
            let (past_and_present, _) = Pallet::<T>::pending_configs()
                .into_iter()
                .partition::<Vec<_>, _>(|&(apply_at_session, _)| apply_at_session <= session_index);

            let config = if let Some(last) = past_and_present.last() {
                last.1.clone()
            } else {
                Pallet::<T>::config()
            };
            config.max_orchestrator_collators
        }
    }
}
