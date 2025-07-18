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
    sp_runtime::{traits::Header, DigestItem, DispatchResult},
    sp_std::borrow::Cow,
    sp_std::vec::Vec,
    tp_author_noting_inherent::INHERENT_IDENTIFIER,
    tp_traits::{
        AuthorNotingHook, AuthorNotingInfo, ContainerChainBlockInfo, ForSession, GenericStateProof,
        GenericStorageReader, GetContainerChainAuthor, GetContainerChainsWithCollators,
        LatestAuthorInfoFetcher, NativeStorageReader, ReadEntryErr,
    },
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

        type ContainerChains: GetContainerChainsWithCollators<Self::AccountId>;

        type SlotBeacon: SlotBeacon;

        type ContainerChainAuthor: GetContainerChainAuthor<Self::AccountId>;

        /// An entry-point for higher-level logic to react to containers chains authoring.
        ///
        /// Typically, this can be a hook to reward block authors.
        type AuthorNotingHook: AuthorNotingHook<Self::AccountId>;

        type RelayOrPara: RelayOrPara;

        /// Max length of para id list, should be the same value as in other pallets.
        #[pallet::constant]
        type MaxContainerChains: Get<u32>;

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

            weight.saturating_accrue(T::DbWeight::get().writes(1));

            // The read onfinalizes
            weight.saturating_accrue(T::DbWeight::get().reads(1));

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
        #[pallet::weight((T::WeightInfo::set_latest_author_data(T::MaxContainerChains::get()), DispatchClass::Mandatory))]
        #[allow(clippy::useless_conversion)]
        pub fn set_latest_author_data(
            origin: OriginFor<T>,
            data: InherentDataOf<T>,
        ) -> DispatchResultWithPostInfo {
            ensure_none(origin)?;

            assert!(
                !<DidSetContainerAuthorData<T>>::exists(),
                "DidSetContainerAuthorData must be updated only once in a block",
            );

            let container_chains_to_check: Vec<_> =
                T::ContainerChains::container_chains_with_collators(ForSession::Current)
                    .into_iter()
                    .filter_map(|(para_id, collators)| (!collators.is_empty()).then_some(para_id))
                    .collect();
            let mut total_weight =
                T::WeightInfo::set_latest_author_data(container_chains_to_check.len() as u32);

            // We do this first to make sure we don't do 2 reads (parachains and relay state)
            // when we have no containers registered
            // Essentially one can pass an empty proof if no container-chains are registered
            if !container_chains_to_check.is_empty() {
                let storage_reader = T::RelayOrPara::create_storage_reader(data);

                let parent_tanssi_slot = u64::from(T::SlotBeacon::slot()).into();
                let mut infos = Vec::with_capacity(container_chains_to_check.len());

                for para_id in container_chains_to_check {
                    match Self::fetch_block_info_from_proof(
                        &storage_reader,
                        para_id,
                        parent_tanssi_slot,
                    ) {
                        Ok(block_info) => {
                            LatestAuthor::<T>::mutate(
                                para_id,
                                |maybe_old_block_info: &mut Option<
                                    ContainerChainBlockInfo<T::AccountId>,
                                >| {
                                    // No block number is the same as the last block number being 0:
                                    // the first block created by collators is block number 1.
                                    let old_block_number = maybe_old_block_info
                                        .as_ref()
                                        .map(|old_block_info| old_block_info.block_number)
                                        .unwrap_or(0);
                                    // We only reward author if the block increases
                                    // If there is no previous block, we should reward the author of the first block
                                    if block_info.block_number > old_block_number {
                                        let bi = block_info.clone();
                                        let info = AuthorNotingInfo {
                                            author: block_info.author,
                                            block_number: block_info.block_number,
                                            para_id,
                                        };
                                        infos.push(info);
                                        *maybe_old_block_info = Some(bi);
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

                total_weight
                    .saturating_accrue(T::AuthorNotingHook::on_container_authors_noted(&infos));
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
    pub(super) type LatestAuthor<T: Config> =
        StorageMap<_, Blake2_128Concat, ParaId, ContainerChainBlockInfo<T::AccountId>, OptionQuery>;

    /// Was the containerAuthorData set?
    #[pallet::storage]
    pub type DidSetContainerAuthorData<T: Config> = StorageValue<_, bool, ValueQuery>;

    #[pallet::inherent]
    impl<T: Config> ProvideInherent for Pallet<T> {
        type Call = Call<T>;
        type Error = InherentError;
        // TODO, what should we put here
        const INHERENT_IDENTIFIER: InherentIdentifier =
            tp_author_noting_inherent::INHERENT_IDENTIFIER;

        fn is_inherent_required(_: &InherentData) -> Result<Option<Self::Error>, Self::Error> {
            // Return Ok(Some(_)) unconditionally because this inherent is required in every block
            Ok(Some(InherentError::Other(Cow::from(
                "Pallet Author Noting Inherent required",
            ))))
        }

        fn create_inherent(data: &InherentData) -> Option<Self::Call> {
            let data = T::RelayOrPara::create_inherent_arg(data);

            Some(Call::set_latest_author_data { data })
        }

        fn is_inherent(call: &Self::Call) -> bool {
            matches!(call, Call::set_latest_author_data { .. })
        }
    }
}

impl<T: Config> Pallet<T> {
    /// Fetch author and block number from a proof of header
    fn fetch_block_info_from_proof<S: GenericStorageReader>(
        relay_state_proof: &S,
        para_id: ParaId,
        tanssi_slot: Slot,
    ) -> Result<ContainerChainBlockInfo<T::AccountId>, Error<T>> {
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
    ) -> Result<ContainerChainBlockInfo<T::AccountId>, Error<T>> {
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

    pub fn latest_author(para_id: ParaId) -> Option<ContainerChainBlockInfo<T::AccountId>> {
        LatestAuthor::<T>::get(para_id)
    }
}

#[derive(Encode)]
#[cfg_attr(feature = "std", derive(Debug, Decode))]
pub enum InherentError {
    Other(Cow<'static, str>),
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

impl<T: Config> LatestAuthorInfoFetcher<T::AccountId> for Pallet<T> {
    fn get_latest_author_info(para_id: ParaId) -> Option<ContainerChainBlockInfo<T::AccountId>> {
        LatestAuthor::<T>::get(para_id)
    }
}

/// This pallet has slightly different behavior when used in a parachain vs when used in a relay chain
/// (solochain). The main difference is:
/// In relay mode, we don't need a storage proof, so the inherent doesn't need any input argument,
/// and instead of reading from a storage proof we read from storage directly.
pub trait RelayOrPara {
    type InherentArg: TypeInfo + Clone + PartialEq + Encode + Decode + core::fmt::Debug;
    type GenericStorageReader: GenericStorageReader;

    fn create_inherent_arg(data: &InherentData) -> Self::InherentArg;
    fn create_storage_reader(data: Self::InherentArg) -> Self::GenericStorageReader;

    #[cfg(feature = "runtime-benchmarks")]
    fn set_current_relay_chain_state(state: cumulus_pallet_parachain_system::RelayChainState);
}

pub type InherentDataOf<T> = <<T as Config>::RelayOrPara as RelayOrPara>::InherentArg;

pub struct RelayMode;
pub struct ParaMode<RCSP: RelaychainStateProvider>(PhantomData<RCSP>);

impl RelayOrPara for RelayMode {
    type InherentArg = ();
    type GenericStorageReader = NativeStorageReader;

    fn create_inherent_arg(_data: &InherentData) -> Self::InherentArg {
        // This ignores the inherent data entirely, so it is compatible with clients that don't add our inherent
    }

    fn create_storage_reader(_data: Self::InherentArg) -> Self::GenericStorageReader {
        NativeStorageReader
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn set_current_relay_chain_state(_state: cumulus_pallet_parachain_system::RelayChainState) {}
}

impl<RCSP: RelaychainStateProvider> RelayOrPara for ParaMode<RCSP> {
    type InherentArg = tp_author_noting_inherent::OwnParachainInherentData;
    type GenericStorageReader = GenericStateProof<cumulus_primitives_core::relay_chain::Block>;

    fn create_inherent_arg(data: &InherentData) -> Self::InherentArg {
        data.get_data(&INHERENT_IDENTIFIER)
            .ok()
            .flatten()
            .expect("there is not data to be posted; qed")
    }

    fn create_storage_reader(data: Self::InherentArg) -> Self::GenericStorageReader {
        let tp_author_noting_inherent::OwnParachainInherentData {
            relay_storage_proof,
        } = data;

        let relay_chain_state = RCSP::current_relay_chain_state();
        let relay_storage_root = relay_chain_state.state_root;

        GenericStateProof::new(relay_storage_root, relay_storage_proof)
            .expect("Invalid relay chain state proof")
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn set_current_relay_chain_state(state: cumulus_pallet_parachain_system::RelayChainState) {
        RCSP::set_current_relay_chain_state(state)
    }
}
