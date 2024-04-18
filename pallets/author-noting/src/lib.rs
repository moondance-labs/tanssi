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

//! # Author Noting Pallet
//!
//! This pallet notes the author of the different containerChains that have registered:
//!
//! The set of container chains is retrieved thanks to the GetContainerChains trait
//! For each containerChain, we inspect the Header stored in the relayChain as
//! a generic header. This is the first requirement for containerChains.
//!
//! The second requirement is that an Aura digest with the slot number for the containerChains
//! needs to exist
//!  
//! Using those two requirements we can select who the author was based on the collators assigned
//! to that containerChain, by simply assigning the slot position.

#![cfg_attr(not(feature = "std"), no_std)]

pub use dp_chain_state_snapshot::*;
use {
    cumulus_pallet_parachain_system::RelaychainStateProvider,
    cumulus_primitives_core::{
        relay_chain::{BlakeTwo256, BlockNumber, HeadData},
        ParaId,
    },
    dp_core::well_known_keys::PARAS_HEADS_INDEX,
    frame_support::{dispatch::PostDispatchInfo, pallet_prelude::*, Hashable},
    frame_system::pallet_prelude::*,
    nimbus_primitives::SlotBeacon,
    parity_scale_codec::{Decode, Encode},
    sp_consensus_aura::{inherents::InherentType, Slot, AURA_ENGINE_ID},
    sp_inherents::{InherentIdentifier, IsFatalError},
    sp_runtime::{traits::Header, DigestItem, DispatchResult, RuntimeString},
    tp_author_noting_inherent::INHERENT_IDENTIFIER,
    tp_traits::{AuthorNotingHook, GetContainerChainAuthor, GetCurrentContainerChains},
};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
pub mod weights;
pub use weights::WeightInfo;

#[cfg(any(test, feature = "runtime-benchmarks"))]
mod benchmarks;
#[cfg(feature = "runtime-benchmarks")]
mod mock_proof;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type ContainerChains: GetCurrentContainerChains;

        type SelfParaId: Get<ParaId>;
        type SlotBeacon: SlotBeacon;

        type ContainerChainAuthor: GetContainerChainAuthor<Self::AccountId>;

        type RelayChainStateProvider: cumulus_pallet_parachain_system::RelaychainStateProvider;

        /// An entry-point for higher-level logic to react to containers chains authoring.
        ///
        /// Typically, this can be a hook to reward block authors.
        type AuthorNotingHook: AuthorNotingHook<Self::AccountId>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The new value for a configuration parameter is invalid.
        FailedReading,
        FailedDecodingHeader,
        AuraDigestFirstItem,
        AsPreRuntimeError,
        NonDecodableSlot,
        AuthorNotFound,
        NonAuraDigest,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
            let mut weight = Weight::zero();

            // We clear this storage item to make sure its always included
            DidSetContainerAuthorData::<T>::kill();

            weight += T::DbWeight::get().writes(1);

            // The read onfinalizes
            weight += T::DbWeight::get().reads(1);

            weight
        }

        fn on_finalize(_: BlockNumberFor<T>) {
            assert!(
                <DidSetContainerAuthorData<T>>::exists(),
                "Container chain author data needs to be present in every block!"
            );
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight((T::WeightInfo::set_latest_author_data(<T::ContainerChains as GetCurrentContainerChains>::MaxContainerChains::get()), DispatchClass::Mandatory))]
        pub fn set_latest_author_data(
            origin: OriginFor<T>,
            data: tp_author_noting_inherent::OwnParachainInherentData,
        ) -> DispatchResultWithPostInfo {
            ensure_none(origin)?;

            assert!(
                !<DidSetContainerAuthorData<T>>::exists(),
                "DidSetContainerAuthorData must be updated only once in a block",
            );

            let registered_para_ids = T::ContainerChains::current_container_chains();
            let mut total_weight =
                T::WeightInfo::set_latest_author_data(registered_para_ids.len() as u32);

            // We do this first to make sure we don't do 2 reads (parachains and relay state)
            // when we have no containers registered
            // Essentially one can pass an empty proof if no container-chains are registered
            if !registered_para_ids.is_empty() {
                let tp_author_noting_inherent::OwnParachainInherentData {
                    relay_storage_proof,
                } = data;

                let relay_chain_state = T::RelayChainStateProvider::current_relay_chain_state();
                let relay_storage_root = relay_chain_state.state_root;
                let relay_storage_rooted_proof =
                    GenericStateProof::new(relay_storage_root, relay_storage_proof)
                        .expect("Invalid relay chain state proof");
                let parent_tanssi_slot = u64::from(T::SlotBeacon::slot()).into();

                // TODO: we should probably fetch all authors-containers first
                // then pass the vector to the hook, this would allow for a better estimation
                for para_id in registered_para_ids {
                    match Self::fetch_block_info_from_proof(
                        &relay_storage_rooted_proof,
                        para_id,
                        parent_tanssi_slot,
                    ) {
                        Ok(block_info) => {
                            LatestAuthor::<T>::mutate(
                                para_id,
                                |maybe_old_block_info: &mut Option<ContainerChainBlockInfo<T>>| {
                                    if let Some(ref mut old_block_info) = maybe_old_block_info {
                                        if block_info.block_number > old_block_info.block_number {
                                            // We only reward author if the block increases
                                            total_weight = total_weight.saturating_add(
                                                T::AuthorNotingHook::on_container_author_noted(
                                                    &block_info.author,
                                                    block_info.block_number,
                                                    para_id,
                                                ),
                                            );
                                            let _ = core::mem::replace(old_block_info, block_info);
                                        }
                                    } else {
                                        // If there is no previous block, we should reward the author of the first block
                                        total_weight = total_weight.saturating_add(
                                            T::AuthorNotingHook::on_container_author_noted(
                                                &block_info.author,
                                                block_info.block_number,
                                                para_id,
                                            ),
                                        );
                                        let _ = core::mem::replace(
                                            maybe_old_block_info,
                                            Some(block_info),
                                        );
                                    }
                                },
                            );
                        }
                        Err(e) => log::warn!(
                            "Author-noting error {:?} found in para {:?}",
                            e,
                            u32::from(para_id)
                        ),
                    }
                }
            }

            // We correctly set the data
            DidSetContainerAuthorData::<T>::put(true);

            Ok(PostDispatchInfo {
                actual_weight: Some(total_weight),
                pays_fee: Pays::No,
            })
        }

        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::set_author())]
        pub fn set_author(
            origin: OriginFor<T>,
            para_id: ParaId,
            block_number: BlockNumber,
            author: T::AccountId,
            latest_slot_number: Slot,
        ) -> DispatchResult {
            ensure_root(origin)?;
            LatestAuthor::<T>::insert(
                para_id,
                ContainerChainBlockInfo {
                    block_number,
                    author: author.clone(),
                    latest_slot_number,
                },
            );
            Self::deposit_event(Event::LatestAuthorChanged {
                para_id,
                block_number,
                new_author: author,
                latest_slot_number,
            });
            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::kill_author_data())]
        pub fn kill_author_data(origin: OriginFor<T>, para_id: ParaId) -> DispatchResult {
            ensure_root(origin)?;
            LatestAuthor::<T>::remove(para_id);
            Self::deposit_event(Event::RemovedAuthorData { para_id });
            Ok(())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Latest author changed
        LatestAuthorChanged {
            para_id: ParaId,
            block_number: BlockNumber,
            new_author: T::AccountId,
            latest_slot_number: Slot,
        },
        /// Removed author data
        RemovedAuthorData { para_id: ParaId },
    }

    #[pallet::storage]
    #[pallet::getter(fn latest_author)]
    pub(super) type LatestAuthor<T: Config> =
        StorageMap<_, Blake2_128Concat, ParaId, ContainerChainBlockInfo<T>, OptionQuery>;

    /// Information extracted from the latest container chain header
    #[derive(
        Clone, Encode, Decode, PartialEq, sp_core::RuntimeDebug, scale_info::TypeInfo, MaxEncodedLen,
    )]
    #[scale_info(skip_type_params(T))]
    pub struct ContainerChainBlockInfo<T: Config> {
        pub block_number: BlockNumber,
        pub author: T::AccountId,
        pub latest_slot_number: Slot,
    }

    /// Was the containerAuthorData set?
    #[pallet::storage]
    pub(super) type DidSetContainerAuthorData<T: Config> = StorageValue<_, bool, ValueQuery>;

    #[pallet::inherent]
    impl<T: Config> ProvideInherent for Pallet<T> {
        type Call = Call<T>;
        type Error = InherentError;
        // TODO, what should we put here
        const INHERENT_IDENTIFIER: InherentIdentifier =
            tp_author_noting_inherent::INHERENT_IDENTIFIER;

        fn is_inherent_required(_: &InherentData) -> Result<Option<Self::Error>, Self::Error> {
            // Return Ok(Some(_)) unconditionally because this inherent is required in every block
            Ok(Some(InherentError::Other(
                sp_runtime::RuntimeString::Borrowed("Pallet Author Noting Inherent required"),
            )))
        }

        fn create_inherent(data: &InherentData) -> Option<Self::Call> {
            let data: tp_author_noting_inherent::OwnParachainInherentData = data
                .get_data(&INHERENT_IDENTIFIER)
                .ok()
                .flatten()
                .expect("there is not data to be posted; qed");

            Some(Call::set_latest_author_data { data })
        }

        fn is_inherent(call: &Self::Call) -> bool {
            matches!(call, Call::set_latest_author_data { .. })
        }
    }
}

impl<T: Config> Pallet<T> {
    /// Fetch author and block number from a proof of header
    fn fetch_block_info_from_proof(
        relay_state_proof: &GenericStateProof<cumulus_primitives_core::relay_chain::Block>,
        para_id: ParaId,
        tanssi_slot: Slot,
    ) -> Result<ContainerChainBlockInfo<T>, Error<T>> {
        let bytes = para_id.twox_64_concat();
        // CONCAT
        let key = [PARAS_HEADS_INDEX, bytes.as_slice()].concat();
        // We might encounter empty vecs
        // We only note if we can decode
        // In this process several errors can occur, but we will only log if such errors happen
        // We first take the HeadData
        // If the readError was that the key was not provided (identified by the Proof error),
        // then panic
        let head_data = relay_state_proof
            .read_entry::<HeadData>(key.as_slice(), None)
            .map_err(|e| match e {
                ReadEntryErr::Proof => panic!("Invalid proof provided for para head key"),
                _ => Error::<T>::FailedReading,
            })?;

        // We later take the Header decoded
        let author_header = sp_runtime::generic::Header::<BlockNumber, BlakeTwo256>::decode(
            &mut head_data.0.as_slice(),
        )
        .map_err(|_| Error::<T>::FailedDecodingHeader)?;

        // Return author from first aura log.
        // If there are no aura logs, it iterates over all the logs, then returns the error from the first element.
        // This is because it is hard to return a `Vec<Error<T>>`.
        let mut first_error = None;
        for aura_digest in author_header.digest().logs() {
            match Self::author_from_log(aura_digest, para_id, &author_header, tanssi_slot) {
                Ok(x) => return Ok(x),
                Err(e) => {
                    if first_error.is_none() {
                        first_error = Some(e);
                    }
                }
            }
        }

        Err(first_error.unwrap_or(Error::<T>::AuraDigestFirstItem))
    }

    /// Get block author from aura digest
    fn author_from_log(
        aura_digest: &DigestItem,
        para_id: ParaId,
        author_header: &sp_runtime::generic::Header<BlockNumber, BlakeTwo256>,
        tanssi_slot: Slot,
    ) -> Result<ContainerChainBlockInfo<T>, Error<T>> {
        // We decode the digest as pre-runtime digest
        let (id, mut data) = aura_digest
            .as_pre_runtime()
            .ok_or(Error::<T>::AsPreRuntimeError)?;

        // Match against the Aura digest
        if id == AURA_ENGINE_ID {
            // DecodeSlot
            let slot = InherentType::decode(&mut data).map_err(|_| Error::<T>::NonDecodableSlot)?;

            // Fetch Author
            let author = T::ContainerChainAuthor::author_for_slot(slot, para_id)
                .ok_or(Error::<T>::AuthorNotFound)?;

            Ok(ContainerChainBlockInfo {
                block_number: author_header.number,
                author,
                // We store the slot number of the current tanssi block to have a time-based notion
                // of when the last block of a container chain was included.
                // Note that this is not the slot of the container chain block, and it does not
                // indicate when that block was created, but when it was included in tanssi.
                latest_slot_number: tanssi_slot,
            })
        } else {
            Err(Error::<T>::NonAuraDigest)
        }
    }
}

#[derive(Encode)]
#[cfg_attr(feature = "std", derive(Debug, Decode))]
pub enum InherentError {
    Other(RuntimeString),
}

impl IsFatalError for InherentError {
    fn is_fatal_error(&self) -> bool {
        match *self {
            InherentError::Other(_) => true,
        }
    }
}

impl InherentError {
    /// Try to create an instance ouf of the given identifier and data.
    #[cfg(feature = "std")]
    pub fn try_from(id: &InherentIdentifier, data: &[u8]) -> Option<Self> {
        if id == &INHERENT_IDENTIFIER {
            <InherentError as parity_scale_codec::Decode>::decode(&mut &data[..]).ok()
        } else {
            None
        }
    }
}
