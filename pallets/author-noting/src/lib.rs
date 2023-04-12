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
//!
#![cfg_attr(not(feature = "std"), no_std)]
use cumulus_primitives_core::relay_chain::BlakeTwo256;
use cumulus_primitives_core::relay_chain::BlockNumber;
use cumulus_primitives_core::relay_chain::HeadData;
use cumulus_primitives_core::ParaId;
use frame_support::Hashable;
use parity_scale_codec::Decode;
use sp_consensus_aura::inherents::InherentType;
use sp_consensus_aura::AURA_ENGINE_ID;
use sp_inherents::InherentIdentifier;
use sp_runtime::traits::Header;
use sp_runtime::DispatchResult;
use sp_std::prelude::*;
use tp_author_noting_inherent::INHERENT_IDENTIFIER;
use tp_author_noting_inherent::PARAS_HEADS_INDEX;

mod relay_state_snapshot;
pub use relay_state_snapshot::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod test;

pub use pallet::*;

pub trait GetContainerChains {
    fn container_chains() -> Vec<ParaId>;
}

#[frame_support::pallet]
pub mod pallet {
    use super::{DispatchResult, *};
    use frame_support::dispatch::PostDispatchInfo;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type ContainerChains: GetContainerChains;

        type SelfParaId: Get<ParaId>;

        type AuthorFetcher: GetAuthorFromSlot<Self>;
    }

    pub trait GetAuthorFromSlot<T: Config> {
        /// Returns current session index.
        fn author_from_inherent(inherent: InherentType, para_id: ParaId) -> Option<T::AccountId>;
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

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight((0, DispatchClass::Mandatory))]
        // TODO: This weight should be corrected.
        pub fn set_latest_author_data(
            origin: OriginFor<T>,
            data: tp_author_noting_inherent::OwnParachainInherentData,
        ) -> DispatchResultWithPostInfo {
            let total_weight = Weight::zero();
            ensure_none(origin)?;
            let tp_author_noting_inherent::OwnParachainInherentData {
                validation_data: vfp,
                relay_chain_state,
            } = data;

            let para_ids = T::ContainerChains::container_chains();
            let relay_state_proof = AuthorNotingRelayChainStateProof::new(
                vfp.relay_parent_storage_root,
                relay_chain_state.clone(),
            )
            .expect("Invalid relay chain state proof");

            for para_id in para_ids {
                match Self::fetch_author_slot_from_proof(&relay_state_proof, para_id) {
                    Ok(author) => LatestAuthor::<T>::insert(para_id, author),
                    Err(e) => log::warn!("Author-noting error {:?} found in para {:?}", e, para_id),
                }
            }

            Ok(PostDispatchInfo {
                actual_weight: Some(total_weight),
                pays_fee: Pays::No,
            })
        }

        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn set_author(
            origin: OriginFor<T>,
            para_id: ParaId,
            new: T::AccountId,
        ) -> DispatchResult {
            ensure_root(origin)?;
            LatestAuthor::<T>::insert(para_id, &new);
            Self::deposit_event(Event::LatestAuthorChanged {
                para_id,
                new_author: new,
            });
            Ok(())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Latest author changed
        LatestAuthorChanged {
            para_id: ParaId,
            new_author: T::AccountId,
        },
    }

    #[pallet::storage]
    #[pallet::getter(fn latest_author)]
    pub(super) type LatestAuthor<T: Config> =
        StorageMap<_, Blake2_128Concat, ParaId, T::AccountId, OptionQuery>;

    #[pallet::inherent]
    impl<T: Config> ProvideInherent for Pallet<T> {
        type Call = Call<T>;
        type Error = sp_inherents::MakeFatalError<()>;
        // TODO, what should we put here
        const INHERENT_IDENTIFIER: InherentIdentifier =
            tp_author_noting_inherent::INHERENT_IDENTIFIER;

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
    /// Fetch author slot from a proof of header
    fn fetch_author_slot_from_proof(
        relay_state_proof: &AuthorNotingRelayChainStateProof,
        para_id: ParaId,
    ) -> Result<T::AccountId, Error<T>> {
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
        let mut author_header = sp_runtime::generic::Header::<BlockNumber, BlakeTwo256>::decode(
            &mut head_data.0.as_slice(),
        )
        .map_err(|_| Error::<T>::FailedDecodingHeader)?
        .clone();

        // We take the aura digest as the first item
        // TODO: improve in the future as iteration
        let aura_digest = author_header
            .digest_mut()
            .logs()
            .first()
            .ok_or(Error::<T>::AuraDigestFirstItem)?;

        // We decode the digest as pre-runtime digest
        let (id, mut data) = aura_digest
            .as_pre_runtime()
            .ok_or(Error::<T>::AsPreRuntimeError)?;

        // Match against the Aura digest
        if id == AURA_ENGINE_ID {
            // DecodeSlot
            let slot = InherentType::decode(&mut data).map_err(|_| Error::<T>::NonDecodableSlot)?;

            // Fetch Author
            let author = T::AuthorFetcher::author_from_inherent(slot, para_id)
                .ok_or(Error::<T>::AuthorNotFound)?;

            Ok(author)
        } else {
            Err(Error::<T>::NonAuraDigest)
        }
    }
}
