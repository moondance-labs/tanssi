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
    use {
        frame_support::pallet_prelude::*, snowbridge_merkle_tree::merkle_root, sp_core::H256,
        sp_runtime::traits::Keccak256,
    };

    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {}

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        NewCommitmentRootRecorded { commitment: H256 },
        CommitmentRootRead { commitment: H256 },
    }

    /// Message commitment from last block (v1).
    /// This will be set only when there are messages to relay.
    #[pallet::storage]
    pub type RecordedCommitment<T: Config> = StorageValue<_, H256, OptionQuery>;

    /// Message commitment from last block (v2).
    /// This will be set only when there are messages to relay.
    #[pallet::storage]
    pub type RecordedCommitmentV2<T: Config> = StorageValue<_, H256, OptionQuery>;

    impl<T: Config> Pallet<T> {
        pub fn take_commitment_root() -> Option<H256> {
            let v1 = RecordedCommitment::<T>::take();
            let v2 = RecordedCommitmentV2::<T>::take();
            match (v1, v2) {
                (Some(v1_commit), Some(v2_commit)) => {
                    // Both v1 and v2, compute unified merkle root
                    let root = merkle_root::<Keccak256, _>([v1_commit, v2_commit].into_iter());
                    Pallet::<T>::deposit_event(Event::<T>::CommitmentRootRead { commitment: root });
                    Some(root)
                }
                (Some(v1_commit), None) => {
                    // Only v1, return v1
                    Pallet::<T>::deposit_event(Event::<T>::CommitmentRootRead {
                        commitment: v1_commit,
                    });
                    Some(v1_commit)
                }
                (None, Some(v2_commit)) => {
                    // Only v2, return v2
                    Pallet::<T>::deposit_event(Event::<T>::CommitmentRootRead {
                        commitment: v2_commit,
                    });
                    Some(v2_commit)
                }
                (None, None) => None,
            }
        }

        pub fn record_commitment_root(commitment: H256) {
            RecordedCommitment::<T>::put(commitment);
            Pallet::<T>::deposit_event(Event::<T>::NewCommitmentRootRecorded { commitment });
        }

        pub fn record_commitment_root_v2(commitment: H256) {
            RecordedCommitmentV2::<T>::put(commitment);
            Pallet::<T>::deposit_event(Event::<T>::NewCommitmentRootRecorded { commitment });
        }
    }
}
