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

//! Commitment recorder pallet.
//!
//! A pallet to record outbound message commitment.
//!

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use {frame_support::pallet_prelude::*, sp_core::H256};

    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        NewCommitmentRootRecorded { commitment: H256 },
        CommitmentRootRead { commitment: H256 },
    }

    /// Message commitment from last block.
    /// This will be set only when there are messages to relay.
    #[pallet::storage]
    pub type RecordedCommitment<T: Config> = StorageValue<_, H256, OptionQuery>;

    impl<T: Config> Pallet<T> {
        pub fn take_commitment_root() -> Option<H256> {
            let maybe_commitment = RecordedCommitment::<T>::take();
            if let Some(commitment) = maybe_commitment {
                Pallet::<T>::deposit_event(Event::<T>::CommitmentRootRead { commitment });
                Some(commitment)
            } else {
                None
            }
        }

        pub fn record_commitment_root(commitment: H256) {
            RecordedCommitment::<T>::put(commitment);
            Pallet::<T>::deposit_event(Event::<T>::NewCommitmentRootRecorded { commitment });
        }
    }
}
