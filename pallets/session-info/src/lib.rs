//! # Session Info Pallet

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use {frame_support::pallet_prelude::*, frame_system::pallet_prelude::*};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::storage]
    #[pallet::getter(fn session_duration)]
    pub(super) type SessionDuration<T: Config> = StorageValue<_, T::BlockNumber, ValueQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn set_new_session_duration(
            origin: OriginFor<T>,
            duration: T::BlockNumber,
        ) -> DispatchResult {
            ensure_root(origin)?;
            SessionDuration::<T>::put(duration);
            Self::deposit_event(Event::SessionDurationChanged { duration });
            Ok(())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        SessionDurationChanged { duration: T::BlockNumber },
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub duration: T::BlockNumber,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            GenesisConfig {
                duration: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            SessionDuration::<T>::put(&self.duration);
        }
    }

    impl<T: Config> Get<T::BlockNumber> for Pallet<T> {
        fn get() -> T::BlockNumber {
            SessionDuration::<T>::get()
        }
    }
}
