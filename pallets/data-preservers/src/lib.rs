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

//! # Inflation Rewards Pallet
//!
//! This pallet handle native token inflation and rewards distribution.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(any(test, feature = "runtime-benchmarks"))]
mod benchmarks;
pub mod weights;

use {
    crate::weights::WeightInfo,
    dp_core::{BlockNumber, ParaId},
    frame_support::{
        pallet_prelude::*,
        parameter_types,
        traits::{
            fungible::{Balanced, Credit, Inspect},
            tokens::{Fortitude, Precision, Preservation},
            Imbalance, OnUnbalanced,
        },
    },
    frame_system::pallet_prelude::*,
    sp_runtime::{
        traits::{Get, Saturating},
        Perbill,
    },
    sp_std::vec::Vec,
    tp_traits::{AuthorNotingHook, DistributeRewards, GetCurrentContainerChains},
};

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    pub type BalanceOf<T> =
        <<T as Config>::Currency as Inspect<<T as frame_system::Config>::AccountId>>::Balance;
    pub type CreditOf<T> = Credit<<T as frame_system::Config>::AccountId, <T as Config>::Currency>;

    /// Inflation rewards pallet.
    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(core::marker::PhantomData<T>);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Inspect<Self::AccountId> + Balanced<Self::AccountId>;

        type MaxBootNodes: Get<u32>;
        type MaxBootNodeUrlLen: Get<u32>;

        type WeightInfo: WeightInfo;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// The list of boot_nodes changed.
        BootNodesChanged { para_id: ParaId },
    }

    #[pallet::storage]
    #[pallet::getter(fn boot_nodes)]
    pub type BootNodes<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ParaId,
        BoundedVec<BoundedVec<u8, T::MaxBootNodeUrlLen>, T::MaxBootNodes>,
        ValueQuery,
    >;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Set boot_nodes for this para id
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::set_boot_nodes(
            T::MaxBootNodeUrlLen::get(),
            boot_nodes.len() as u32,
        ))]
        pub fn set_boot_nodes(
            origin: OriginFor<T>,
            para_id: ParaId,
            boot_nodes: BoundedVec<BoundedVec<u8, T::MaxBootNodeUrlLen>, T::MaxBootNodes>,
        ) -> DispatchResult {
            // TODO: extract this logic somewhere and use it instead of ensure_root
            // Something like ensure_container_chain_manager_or_root
            // That will return Either::Left(container_chain_manager_addr), Either::Right for root, or error
            /*
            let origin =
                EitherOfDiverse::<T::RegistrarOrigin, EnsureSigned<T::AccountId>>::ensure_origin(
                    origin,
                )?;

            if let Either::Right(signed_account) = origin {
                let deposit_info = RegistrarDeposit::<T>::get(para_id).ok_or(BadOrigin)?;
                if deposit_info.creator != signed_account {
                    Err(BadOrigin)?;
                }
            }
            */
            ensure_root(origin)?;

            BootNodes::<T>::insert(para_id, boot_nodes);

            Self::deposit_event(Event::BootNodesChanged { para_id });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        fn some_method() -> Weight {
            todo!()
        }
    }
}
