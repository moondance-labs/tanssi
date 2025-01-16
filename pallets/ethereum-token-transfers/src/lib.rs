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

//! TODO: pallet's description

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use {
    frame_support::{
        pallet_prelude::*,
        traits::{Defensive, Get, ValidatorSet},
    },
    frame_system::pallet_prelude::*,
    parity_scale_codec::Encode,
    polkadot_primitives::ValidatorIndex,
    runtime_parachains::session_info,
    snowbridge_core::{AgentId, ChannelId, ParaId},
    snowbridge_outbound_queue_merkle_tree::{merkle_proof, merkle_root, verify_proof, MerkleProof},
    sp_core::H256,
    sp_runtime::DispatchResult,
    sp_staking::SessionIndex,
    sp_std::collections::btree_set::BTreeSet,
    sp_std::vec,
    sp_std::vec::Vec,
    tp_bridge::{Command, DeliverMessage, Message, ValidateMessage},
    tp_traits::EthereumSystemChannelManager,
};

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    //pub use crate::weights::WeightInfo;
    /*     use {
        super::*, frame_support::pallet_prelude::*, sp_std::collections::btree_map::BTreeMap,
        tp_traits::EraIndexProvider,
    }; */

    use super::*;

    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

    pub type RewardPoints = u32;
    pub type EraIndex = u32;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Overarching event type.
        // type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Validate a message that will be sent to Ethereum.
        // type ValidateMessage: ValidateMessage;

        /// Send a message to Ethereum. Needs to be validated first.
        /*         type OutboundQueue: DeliverMessage<
            Ticket = <<Self as pallet::Config>::ValidateMessage as ValidateMessage>::Ticket,
        >; */

        type EthereumSystemHandler: EthereumSystemChannelManager;

        // The weight information of this pallet.
        // type WeightInfo: WeightInfo;
    }

    // Events
    // Errors
    #[pallet::error]
    pub enum Error<T> {
        ChannelIdAlreadyExists,
        ParaIdAlreadyExists,
    }

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    // Storage
    #[pallet::storage]
    #[pallet::getter(fn current_channel_id)]
    pub type CurrentChannelId<T: Config> = StorageValue<_, ChannelId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn current_para_id)]
    pub type CurrentParaId<T: Config> = StorageValue<_, ParaId, ValueQuery>;

    // Calls
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // TODO: docs
        // TODO: benchmarking
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::default())]
        pub fn set_token_transfer_channel(
            origin: OriginFor<T>,
            channel_id: ChannelId,
            agent_id: AgentId,
            para_id: ParaId,
        ) -> DispatchResult {
            ensure_root(origin)?;

            let current_channel_id = CurrentChannelId::<T>::get();
            let current_para_id = CurrentParaId::<T>::get();

            if current_channel_id == channel_id {
                return Err(Error::<T>::ChannelIdAlreadyExists.into());
            }

            if current_para_id == para_id {
                return Err(Error::<T>::ParaIdAlreadyExists.into());
            }

            CurrentChannelId::<T>::put(channel_id);
            CurrentParaId::<T>::put(para_id);

            T::EthereumSystemHandler::create_channel(channel_id, agent_id, para_id)?;

            Ok(())
        }
    }
}
