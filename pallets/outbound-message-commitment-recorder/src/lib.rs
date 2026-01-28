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
    use snowbridge_merkle_tree::{merkle_proof, MerkleProof};
    use {
        frame_support::pallet_prelude::*, snowbridge_merkle_tree::merkle_root, sp_core::H256,
        sp_runtime::traits::Hash, sp_runtime::Vec,
    };

    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Hashing: Hash<Output = H256>;
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

    /// Message commitment from last block (v1).
    /// This will be set only when there are messages to relay.
    #[pallet::storage]
    #[pallet::unbounded]
    pub type RecordedCommitment<T: Config> = StorageValue<_, (H256, Vec<H256>), OptionQuery>;

    /// Message commitment from last block (v2).
    /// This will be set only when there are messages to relay.
    #[pallet::storage]
    #[pallet::unbounded]
    pub type RecordedCommitmentV2<T: Config> = StorageValue<_, (H256, Vec<H256>), OptionQuery>;

    impl<T: Config> Pallet<T> {
        fn get_combined_leaves(
            leaves_v1: Option<Vec<H256>>,
            leaves_v2: Option<Vec<H256>>,
        ) -> Option<Vec<H256>> {
            match (leaves_v1, leaves_v2) {
                (None, None) => None,
                (Some(leaves_v1), None) => Some(leaves_v1),
                (None, Some(leaves_v2)) => Some(leaves_v2),
                (Some(mut leaves_v1), Some(mut leaves_v2)) => {
                    leaves_v1.append(&mut leaves_v2);
                    Some(leaves_v1)
                }
            }
        }

        pub fn take_commitment_root() -> Option<H256> {
            let v1 = RecordedCommitment::<T>::take();
            let v2 = RecordedCommitmentV2::<T>::take();
            match (v1, v2) {
                (Some((_root_v1, leaves_v1)), Some((_root_v2, leaves_v2))) => {
                    let combined_leaves =
                        Self::get_combined_leaves(Some(leaves_v1), Some(leaves_v2));
                    let root = merkle_root::<<T as Config>::Hashing, _>(
                        combined_leaves.unwrap_or_default().into_iter(),
                    );
                    Pallet::<T>::deposit_event(Event::<T>::CommitmentRootRead { commitment: root });
                    Some(root)
                }
                (Some((root, _leaves)), None) => {
                    // Only v1, return v1 root directly no need to recompute root
                    Pallet::<T>::deposit_event(Event::<T>::CommitmentRootRead { commitment: root });
                    Some(root)
                }
                (None, Some((root, _leaves))) => {
                    // Only v2, return v2 root directly no need to recompute root
                    Pallet::<T>::deposit_event(Event::<T>::CommitmentRootRead { commitment: root });
                    Some(root)
                }
                (None, None) => None,
            }
        }

        pub fn record_commitment_root(commitment: H256, message_leaves: Vec<H256>) {
            RecordedCommitment::<T>::put((commitment, message_leaves));
            Pallet::<T>::deposit_event(Event::<T>::NewCommitmentRootRecorded { commitment });
        }

        pub fn record_commitment_root_v2(commitment: H256, message_leaves: Vec<H256>) {
            RecordedCommitmentV2::<T>::put((commitment, message_leaves));
            Pallet::<T>::deposit_event(Event::<T>::NewCommitmentRootRecorded { commitment });
        }

        pub fn prove_message_v1(leaf_index: u64) -> Option<MerkleProof> {
            Self::prove_message(leaf_index)
        }

        pub fn prove_message_v2(leaf_index: u64) -> Option<MerkleProof> {
            let v1 = RecordedCommitment::<T>::get();
            if let Some((_, leaves)) = v1 {
                let maybe_combined_index = (leaves.len() as u64).checked_add(leaf_index);
                if let Some(combined_index) = maybe_combined_index {
                    Self::prove_message(combined_index)
                } else {
                    // Overflow
                    None
                }
            } else {
                Self::prove_message(leaf_index)
            }
        }

        fn prove_message(combined_leaf_index: u64) -> Option<MerkleProof> {
            let v1 = RecordedCommitment::<T>::get();
            let v2 = RecordedCommitmentV2::<T>::get();
            match (v1, v2) {
                (None, None) => None,
                (v1_data, v2_data) => {
                    let combined_leaves = Self::get_combined_leaves(
                        v1_data.map(|(_root, leaves)| leaves),
                        v2_data.map(|(_root, leaves)| leaves),
                    );

                    if let Some(combined_leaves) = combined_leaves {
                        let proof = merkle_proof::<<T as Config>::Hashing, _>(
                            combined_leaves.into_iter(),
                            combined_leaf_index,
                        );
                        Some(proof)
                    } else {
                        None
                    }
                }
            }
        }
    }
}
