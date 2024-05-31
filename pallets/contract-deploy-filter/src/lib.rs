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

//! # Contract deploy filter Pallet
//!
//! A pallet to filter which addresses are allowed to deploy contracts
//! in a chain working within an evm environment.
//!
//! The pallet brings the possibility of filtering the addresses that
//! can deploy contracts in two ways:
//!
//! - Deploying contracts in a direct way -> CREATE calls.
//! - Deploying contracts by calling a function inside a contract which
//!   deploys other contracts internally -> CALL(CREATE) calls.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use {
    frame_support::{pallet_prelude::*, BoundedVec},
    frame_system::{ensure_root, pallet_prelude::*},
    sp_core::{Get, H160},
    sp_std::vec::Vec,
};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {

    use super::*;
    //pub use crate::weights::WeightInfo;

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config /* TODO: should we add pallet_evm::Config? */ {
        /// Overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        #[pallet::constant]
        type MaxAllowedCreate: Get<u32>;

        #[pallet::constant]
        type MaxAllowedCreateInner: Get<u32>;

        // The weight information of this pallet.
        //type WeightInfo: WeightInfo;
    }

    /// Addresses that are allowed to deploy contracts via CREATE calls.
    #[pallet::storage]
    #[pallet::getter(fn allowed_addresses_to_create)]
    pub(crate) type AllowedAddressesToCreate<T: Config> =
        StorageValue<_, BoundedVec<H160, T::MaxAllowedCreate>, ValueQuery>;

    /// Addresses that are allowed to deploy contracts via CALL(CREATE) calls.
    #[pallet::storage]
    #[pallet::getter(fn allowed_addresses_to_create_inner)]
    pub(crate) type AllowedAddressesToCreateInner<T: Config> =
        StorageValue<_, BoundedVec<H160, T::MaxAllowedCreateInner>, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// An address was successfully allowed to deploy contracts via CREATE calls.
        AllowedAddressToCreate { address: H160 },

        /// An address was successfully allowed to deploy contracts via CALL(CREATE) calls.
        AllowedAddressToCreateInner { address: H160 },

        /// An address is not allowed to deploy contracts via CREATE calls anymore.
        RemovedAllowedAddressToCreate { address: H160 },

        /// An address is not allowed to deploy contracts via CALL(CREATE) calls anymore.
        RemovedAllowedAddressToCreateInner { address: H160 },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The address was already allowed to deploy contracts either via CREATE calls
        /// or CALL(CREATE) calls.
        AlreadyAllowed,

        /// MaxAllowedCreate or MaxAllowedCreateInner limit was reached.
        TooManyAllowedAddresses,

        /// An attempt to remove an address from either AllowedAddressesToCreate or
        /// AllowedAddressesToCreateInner that was not present in the list.
        NotPresentInAllowedAddresses,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Allow an address to deploy contracts via CREATE calls.
        #[pallet::call_index(1)]
        //TODO: add proper weight
        /*         #[pallet::weight(T::WeightInfo::allow_address_to_create(
            T::MaxAllowedCreate::get().saturating_sub(1),
        ))] */
        #[pallet::weight(Weight::from_parts(60000u64, 40000u64))]
        pub fn allow_address_to_create(origin: OriginFor<T>, address: H160) -> DispatchResult {
            ensure_root(origin)?;

            <AllowedAddressesToCreate<T>>::try_mutate(|allowed_addresses| -> DispatchResult {
                if allowed_addresses.contains(&address) {
                    Err(Error::<T>::AlreadyAllowed)?;
                }
                allowed_addresses
                    .try_push(address.clone())
                    .map_err(|_| Error::<T>::TooManyAllowedAddresses)?;
                Ok(())
            })?;

            Self::deposit_event(Event::AllowedAddressToCreate { address });

            // TODO: do we need to return the weight used here inside a DispatchResultWithPostInfo?
            Ok(())
        }

        /// Allow an address to deploy contracts via CALL(CREATE) calls.
        #[pallet::call_index(2)]
        //TODO: add proper weight
        /*         #[pallet::weight(T::WeightInfo::allow_address_to_create_inner(
            T::MaxAllowedCreate::get().saturating_sub(1),
        ))] */
        #[pallet::weight(Weight::from_parts(60000u64, 40000u64))]
        pub fn allow_address_to_create_inner(
            origin: OriginFor<T>,
            address: H160,
        ) -> DispatchResult {
            ensure_root(origin)?;

            <AllowedAddressesToCreateInner<T>>::try_mutate(
                |allowed_addresses| -> DispatchResult {
                    if allowed_addresses.contains(&address) {
                        Err(Error::<T>::AlreadyAllowed)?;
                    }
                    allowed_addresses
                        .try_push(address.clone())
                        .map_err(|_| Error::<T>::TooManyAllowedAddresses)?;
                    Ok(())
                },
            )?;

            Self::deposit_event(Event::AllowedAddressToCreateInner { address });

            // TODO: do we need to return the weight used here inside a DispatchResultWithPostInfo?
            Ok(())
        }

        /// Forbid an address to deploy contracts via CREATE calls removing it from the list.
        #[pallet::call_index(3)]
        //TODO: add proper weight
        /*         #[pallet::weight(T::WeightInfo::remove_allowed_address_to_create(
            T::MaxAllowedCreate::get().saturating_sub(1),
        ))] */
        #[pallet::weight(Weight::from_parts(60000u64, 40000u64))]
        pub fn remove_allowed_address_to_create(
            origin: OriginFor<T>,
            address: H160,
        ) -> DispatchResult {
            ensure_root(origin)?;

            <AllowedAddressesToCreate<T>>::try_mutate(|allowed_addresses| -> DispatchResult {
                let pos = allowed_addresses
                    .iter()
                    .position(|x| x == &address)
                    .ok_or(Error::<T>::NotPresentInAllowedAddresses)?;
                allowed_addresses.remove(pos);
                Ok(())
            })?;

            Self::deposit_event(Event::RemovedAllowedAddressToCreate { address });

            // TODO: do we need to return the weight used here inside a DispatchResultWithPostInfo?
            Ok(())
        }

        /// Forbid an address to deploy contracts via CALL(CREATE) calls removing it from the list.
        #[pallet::call_index(4)]
        //TODO: add proper weight
        /*         #[pallet::weight(T::WeightInfo::remove_allowed_address_to_create_inner(
            T::MaxAllowedCreate::get().saturating_sub(1),
        ))] */
        #[pallet::weight(Weight::from_parts(60000u64, 40000u64))]
        pub fn remove_allowed_address_to_create_inner(
            origin: OriginFor<T>,
            address: H160,
        ) -> DispatchResult {
            ensure_root(origin)?;

            <AllowedAddressesToCreateInner<T>>::try_mutate(
                |allowed_addresses| -> DispatchResult {
                    let pos = allowed_addresses
                        .iter()
                        .position(|x| x == &address)
                        .ok_or(Error::<T>::NotPresentInAllowedAddresses)?;
                    allowed_addresses.remove(pos);
                    Ok(())
                },
            )?;

            Self::deposit_event(Event::RemovedAllowedAddressToCreateInner { address });

            // TODO: do we need to return the weight used here inside a DispatchResultWithPostInfo?
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn get_allowed_addresses_to_create() -> Vec<H160> {
            <AllowedAddressesToCreate<T>>::get().into()
        }

        pub fn get_allowed_addresses_to_create_inner() -> Vec<H160> {
            <AllowedAddressesToCreateInner<T>>::get().into()
        }
    }
}
