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

//! # Data Preservers Pallet
//!
//! This pallet allows container chains to select data preservers.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(any(test, feature = "runtime-benchmarks"))]
mod benchmarks;
pub mod weights;
pub use weights::WeightInfo;

use {
    dp_core::ParaId,
    frame_support::{
        pallet_prelude::*,
        traits::{
            fungible::{Balanced, Inspect},
            EnsureOriginWithArg,
        },
        DefaultNoBound,
    },
    frame_system::pallet_prelude::*,
    sp_runtime::traits::Get,
    sp_std::vec::Vec,
};

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::genesis_config]
    #[derive(DefaultNoBound)]
    pub struct GenesisConfig<T: Config> {
        /// Para ids
        pub para_id_boot_nodes: Vec<(ParaId, Vec<Vec<u8>>)>,
        pub _phantom: PhantomData<T>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            // Sort para ids and detect duplicates, but do it using a vector of
            // references to avoid cloning the boot nodes.
            let mut para_ids: Vec<&_> = self.para_id_boot_nodes.iter().collect();
            para_ids.sort_by(|a, b| a.0.cmp(&b.0));
            para_ids.dedup_by(|a, b| {
                if a.0 == b.0 {
                    panic!("Duplicate para_id: {}", u32::from(a.0));
                } else {
                    false
                }
            });

            for (para_id, boot_nodes) in para_ids {
                let boot_nodes: Vec<_> = boot_nodes
                    .iter()
                    .map(|x| BoundedVec::try_from(x.clone()).expect("boot node url too long"))
                    .collect();
                let boot_nodes = BoundedVec::try_from(boot_nodes).expect("too many boot nodes");
                <BootNodes<T>>::insert(para_id, boot_nodes);
            }
        }
    }

    /// Data preservers pallet.
    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Inspect<Self::AccountId> + Balanced<Self::AccountId>;
        // Who can call set_boot_nodes?
        type SetBootNodesOrigin: EnsureOriginWithArg<Self::RuntimeOrigin, ParaId>;

        #[pallet::constant]
        type MaxBootNodes: Get<u32>;
        #[pallet::constant]
        type MaxBootNodeUrlLen: Get<u32>;

        type WeightInfo: WeightInfo;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// The list of boot_nodes changed.
        BootNodesChanged { para_id: ParaId },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// This container chain does not have any boot nodes
        NoBootNodes,
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
            T::SetBootNodesOrigin::ensure_origin(origin, &para_id)?;

            BootNodes::<T>::insert(para_id, boot_nodes);

            Self::deposit_event(Event::BootNodesChanged { para_id });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Function that will be called when a container chain is deregistered. Cleans up all the storage related to this para_id.
        /// Cannot fail.
        pub fn para_deregistered(para_id: ParaId) {
            BootNodes::<T>::remove(para_id);
        }

        pub fn check_valid_for_collating(para_id: ParaId) -> DispatchResult {
            // To be able to call mark_valid_for_collating, a container chain must have bootnodes
            if Pallet::<T>::boot_nodes(para_id).len() > 0 {
                Ok(())
            } else {
                Err(Error::<T>::NoBootNodes.into())
            }
        }
    }
}
