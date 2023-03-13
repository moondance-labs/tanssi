#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use sp_std::prelude::*;

pub use pallet::*;

#[cfg(test)]
mod tests;

const LOG_TARGET: &str = "pallet_configuration";

/// All configuration of the runtime with respect to parachains and parathreads.
#[derive(Clone, Encode, Decode, PartialEq, sp_core::RuntimeDebug, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct HostConfiguration {
    pub max_collators: u32,
    pub moondance_collators: u32,
    pub collators_per_container: u32,
}

impl Default for HostConfiguration {
    fn default() -> Self {
        Self {
            max_collators: Default::default(),
            moondance_collators: Default::default(),
            collators_per_container: Default::default(),
        }
    }
}

/// Enumerates the possible inconsistencies of `HostConfiguration`.
#[derive(Debug)]
pub enum InconsistentError {
    /// `group_rotation_frequency` is set to zero.
    ZeroGroupRotationFrequency,
}

impl HostConfiguration {
    /// Checks that this instance is consistent with the requirements on each individual member.
    ///
    /// # Errors
    ///
    /// This function returns an error if the configuration is inconsistent.
    pub fn check_consistency(&self) -> Result<(), InconsistentError> {
        // TODO: check for some rules such as values that cannot be zero
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

pub trait WeightInfo {
    fn set_config_with_block_number() -> Weight;
    fn set_config_with_u32() -> Weight;
    fn set_config_with_option_u32() -> Weight;
    fn set_config_with_weight() -> Weight;
    fn set_config_with_balance() -> Weight;
    fn set_hrmp_open_request_ttl() -> Weight;
}

impl WeightInfo for () {
    fn set_config_with_block_number() -> Weight {
        Weight::MAX
    }
    fn set_config_with_u32() -> Weight {
        Weight::MAX
    }
    fn set_config_with_option_u32() -> Weight {
        Weight::MAX
    }
    fn set_config_with_weight() -> Weight {
        Weight::MAX
    }
    fn set_config_with_balance() -> Weight {
        Weight::MAX
    }
    fn set_hrmp_open_request_ttl() -> Weight {
        Weight::MAX
    }
}

pub trait GetSessionIndex<SessionIndex> {
    /// Returns current session index.
    fn session_index() -> SessionIndex;

    /// Returns `Self::session_index().saturating_add(delay)`
    fn scheduled_session(delay: SessionIndex) -> SessionIndex;
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Origin that is allowed to call register and deregister
        type RegistrarOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;

        // `SESSION_DELAY` is used to delay any changes to Paras registration or configurations.
        // Wait until the session index is 2 larger then the current index to apply any changes,
        // which guarantees that at least one full session has passed before any changes are applied.
        type SessionDelay: Get<Self::SessionIndex>;

        type SessionIndex: Default + codec::FullCodec + PartialOrd + TypeInfo + Copy;

        type CurrentSessionIndex: GetSessionIndex<Self::SessionIndex>;
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The new value for a configuration parameter is invalid.
        InvalidNewValue,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ScheduledConfigUpdate,
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
    pub(crate) type PendingConfigs<T: Config> =
        StorageValue<_, Vec<(T::SessionIndex, HostConfiguration)>, ValueQuery>;

    /// If this is set, then the configuration setters will bypass the consistency checks. This
    /// is meant to be used only as the last resort.
    #[pallet::storage]
    pub(crate) type BypassConsistencyCheck<T: Config> = StorageValue<_, bool, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig {
        pub config: HostConfiguration,
    }

    #[cfg(feature = "std")]
    impl Default for GenesisConfig {
        fn default() -> Self {
            GenesisConfig {
                config: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig {
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
        pub fn set_moondance_collators(origin: OriginFor<T>, new: u32) -> DispatchResult {
            ensure_root(origin)?;
            Self::schedule_config_update(|config| {
                config.moondance_collators = new;
            })
        }

        #[pallet::call_index(2)]
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

        /// Setting this to true will disable consistency checks for the configuration setters.
        /// Use with caution.
        #[pallet::call_index(44)]
        #[pallet::weight((
			T::DbWeight::get().writes(1),
			DispatchClass::Operational,
		))]
        pub fn set_bypass_consistency_check(origin: OriginFor<T>, new: bool) -> DispatchResult {
            ensure_root(origin)?;
            <Self as Store>::BypassConsistencyCheck::put(new);
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Return the session index that should be used for any future scheduled changes.
        fn scheduled_session() -> T::SessionIndex {
            T::CurrentSessionIndex::scheduled_session(T::SessionDelay::get())
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
                .map(|&(_, ref config)| config.clone())
                .unwrap_or_else(Self::config);
            let base_config_consistent = base_config.check_consistency().is_ok();

            // Now, we need to decide what the new configuration should be.
            // We also move the `base_config` to `new_config` to empahsize that the base config was
            // destroyed by the `updater`.
            updater(&mut base_config);
            let new_config = base_config;

            if <Self as Store>::BypassConsistencyCheck::get() {
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
}
