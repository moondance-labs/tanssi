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

#![cfg_attr(not(feature = "std"), no_std)]
use frame_system::ensure_root;
pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::storage]
    pub type OffchainWorkerTestEnabled<T> = StorageValue<_, bool, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub _phantom_data: PhantomData<T>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            <OffchainWorkerTestEnabled<T>>::put(&false);
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        /// Offchain worker entry point.
        ///
        /// By implementing `fn offchain_worker` you declare a new offchain worker.
        /// This function will be called when the node is fully synced and a new best block is
        /// successfully imported.
        /// Note that it's not guaranteed for offchain workers to run on EVERY block, there might
        /// be cases where some blocks are skipped, or for some the worker runs twice (re-orgs),
        /// so the code should be able to handle that.
        fn offchain_worker(_block_number: BlockNumberFor<T>) {
            log::info!("Entering off-chain worker.");
            // The entry point of your code called by off-chain worker
            Self::emit_offchain_event();
        }
    }
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Switch on or off the offchain worker
        ///
        /// Only root (or specified authority account) should be able to switch
        /// the off-chain worker on and off to avoid enabling it by default in production
        #[pallet::call_index(0)]
        #[pallet::weight({0})]
        pub fn switch_offchain_worker(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let _ = ensure_root(origin)?;

            OffchainWorkerTestEnabled::<T>::put(!OffchainWorkerTestEnabled::<T>::get());
            Ok(().into())
        }
    }

    /// Events for the pallet.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Simple offchain event
        SimpleOffchainEvent,
    }
}

impl<T: Config> Pallet<T> {
    /// Send simple unsigned transaction
    fn emit_offchain_event() {
        if OffchainWorkerTestEnabled::<T>::get() {
            Self::deposit_event(Event::SimpleOffchainEvent);
        }
    }
}
