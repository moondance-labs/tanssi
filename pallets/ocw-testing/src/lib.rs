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
use frame_system::{
    self as system, ensure_none, ensure_root, offchain::SubmitTransaction,
    pallet_prelude::BlockNumberFor,
};
use sp_runtime::transaction_validity::{InvalidTransaction, TransactionValidity, ValidTransaction};

pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config:
        frame_system::offchain::SendTransactionTypes<Call<Self>> + frame_system::Config
    {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Number of blocks of cooldown after unsigned transaction is included.
        ///
        /// This ensures that we only accept unsigned transactions once, every `UnsignedInterval`
        /// blocks.
        #[pallet::constant]
        type UnsignedInterval: Get<BlockNumberFor<Self>>;
    }

    #[pallet::storage]
    pub(super) type OffchainWorkerTestEnabled<T> = StorageValue<_, bool, ValueQuery>;

    /// Defines the block when next unsigned transaction will be accepted.
    ///
    /// To prevent spam of unsigned (and unpaid!) transactions on the network,
    /// we only allow one transaction every `T::UnsignedInterval` blocks.
    /// This storage entry defines when new transaction is going to be accepted.
    #[pallet::storage]
    pub(super) type NextUnsignedAt<T: Config> = StorageValue<_, BlockNumberFor<T>, ValueQuery>;

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
        fn offchain_worker(block_number: BlockNumberFor<T>) {
            log::info!("Entering off-chain worker.");
            // The entry point of your code called by off-chain worker
            let res = Self::send_raw_unsigned_transaction(block_number);
            if let Err(e) = res {
                log::error!("Error: {}", e);
            }
        }
    }
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Switches on or off the offchain worker
        ///
        /// Only root (or specified authority account) should be able to switch
        /// the off-chain worker on and off to avoid enabling it by default in production
        #[pallet::call_index(0)]
        #[pallet::weight(T::DbWeight::get().write)]
        pub fn set_offchain_worker(
            origin: OriginFor<T>,
            is_testing_enabled: bool,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            OffchainWorkerTestEnabled::<T>::put(is_testing_enabled);
            Ok(().into())
        }

        /// Submits unsigned transaction that emits an event
        ///
        /// Can be triggered only by an offchain worker
        #[pallet::call_index(1)]
        #[pallet::weight(T::DbWeight::get().write)]
        pub fn submit_event_unsigned(
            origin: OriginFor<T>,
            _block_number: BlockNumberFor<T>,
        ) -> DispatchResultWithPostInfo {
            // This ensures that the function can only be called via unsigned transaction.
            ensure_none(origin)?;

            // Increment the block number at which we expect next unsigned transaction.
            let current_block = <frame_system::Pallet<T>>::block_number();

            // Emits offchain event
            Self::deposit_event(Event::SimpleOffchainEvent);

            <NextUnsignedAt<T>>::put(current_block + T::UnsignedInterval::get());
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

    #[pallet::validate_unsigned]
    impl<T: Config> ValidateUnsigned for Pallet<T> {
        type Call = Call<T>;

        /// Validate unsigned call to this module.
        ///
        /// By default unsigned transactions are disallowed, but implementing the validator
        /// here we make sure that some particular calls (the ones produced by offchain worker)
        /// are being whitelisted and marked as valid.
        fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
            if let Call::submit_event_unsigned { block_number } = call {
                Self::validate_transaction_parameters(block_number)
            } else {
                InvalidTransaction::Call.into()
            }
        }
    }
}

impl<T: Config> Pallet<T> {
    /// A helper function to sign payload and send an unsigned transaction
    fn send_raw_unsigned_transaction(block_number: BlockNumberFor<T>) -> Result<(), &'static str> {
        // Make sure offchain worker testing is enabled
        let is_offchain_worker_enabled = OffchainWorkerTestEnabled::<T>::get();
        if !is_offchain_worker_enabled {
            return Err("Offchain worker is not enabled");
        }
        // Make sure transaction can be sent
        let next_unsigned_at = NextUnsignedAt::<T>::get();
        if next_unsigned_at > block_number {
            return Err("Too early to send unsigned transaction");
        }

        let call = Call::submit_event_unsigned { block_number };

        SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into())
            .map_err(|()| "Unable to submit unsigned transaction.")?;

        Ok(())
    }

    fn validate_transaction_parameters(block_number: &BlockNumberFor<T>) -> TransactionValidity {
        // Make sure offchain worker testing is enabled
        let is_offchain_worker_enabled = OffchainWorkerTestEnabled::<T>::get();
        if !is_offchain_worker_enabled {
            return InvalidTransaction::Call.into();
        }
        // Now let's check if the transaction has any chance to succeed.
        let next_unsigned_at = NextUnsignedAt::<T>::get();
        if &next_unsigned_at > block_number {
            return InvalidTransaction::Stale.into();
        }
        // Let's make sure to reject transactions from the future.
        let current_block = <system::Pallet<T>>::block_number();
        if &current_block < block_number {
            return InvalidTransaction::Future.into();
        }
        ValidTransaction::with_tag_prefix("ExampleOffchainWorker")
            // We set base priority to 2**20 and hope it's included before any other
            // transactions in the pool. Next we tweak the priority depending on how much
            // it differs from the current average. (the more it differs the more priority it
            // has).
            .priority(2u64.pow(20))
            // This transaction does not require anything else to go before into the pool.
            // In theory we could require `previous_unsigned_at` transaction to go first,
            // but it's not necessary in our case.
            //.and_requires()
            // We set the `provides` tag to be the same as `next_unsigned_at`. This makes
            // sure only one transaction produced after `next_unsigned_at` will ever
            // get to the transaction pool and will end up in the block.
            // We can still have multiple transactions compete for the same "spot",
            // and the one with higher priority will replace other one in the pool.
            .and_provides(next_unsigned_at)
            // The transaction is only valid for next 5 blocks. After that it's
            // going to be revalidated by the pool.
            .longevity(6)
            // It's fine to propagate that transaction to other peers, which means it can be
            // created even by nodes that don't produce blocks.
            // Note that sometimes it's better to keep it for yourself (if you are the block
            // producer), since for instance in some schemes others may copy your solution and
            // claim a reward.
            .propagate(true)
            .build()
    }
}
