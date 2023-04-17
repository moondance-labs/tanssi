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
use frame_support::traits::Get;
use frame_support::Hashable;
use parity_scale_codec::Decode;
use sp_consensus_aura::inherents::InherentType;
use sp_inherents::{InherentIdentifier, IsFatalError};
use sp_runtime::traits::Hash as HashT;
use sp_std::prelude::*;
use tp_authorities_noting_inherent::INHERENT_IDENTIFIER;
use tp_chain_state_snapshot::*;
use tp_collator_assignment::AssignedCollators;
use tp_core::well_known_keys::COLLATOR_ASSIGNMENT_INDEX;
use tp_core::well_known_keys::PARAS_HEADS_INDEX;
use sp_runtime::RuntimeString;
use parity_scale_codec::Encode;

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
    use super::*;
    use frame_support::dispatch::PostDispatchInfo;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type OrchestratorParaId: Get<ParaId>;
        type SelfParaId: Get<ParaId>;
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
        NoAuthoritiesFound,
    }

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight((0, DispatchClass::Mandatory))]
        // TODO: This weight should be corrected.
        pub fn set_latest_authorities_data(
            origin: OriginFor<T>,
            data: tp_authorities_noting_inherent::ContainerChainAuthoritiesInherentData,
        ) -> DispatchResultWithPostInfo {
            let total_weight = Weight::zero();
            ensure_none(origin)?;
            let tp_authorities_noting_inherent::ContainerChainAuthoritiesInherentData {
                validation_data: vfp,
                relay_chain_state,
                orchestrator_chain_state,
            } = data;

            let para_id = T::OrchestratorParaId::get();
            let relay_state_proof = RelayChainHeaderStateProof::new(
                vfp.relay_parent_storage_root,
                relay_chain_state.clone(),
            )
            .expect("Invalid relay chain state proof");

            let orchestrator_root =
                Self::fetch_orchestrator_header_from_relay_proof(&relay_state_proof, para_id)
                    .expect("qed");

            let orchestrator_state_proof = RelayChainHeaderStateProof::new(
                orchestrator_root,
                orchestrator_chain_state.clone(),
            )
            .expect("Invalid orchestrator chain state proof");

            match Self::fetch_authorities_from_orchestrator_proof(
                &orchestrator_state_proof,
                T::SelfParaId::get(),
            ) {
                Ok(authorities) => Authorities::<T>::put(authorities),
                Err(e) => Authorities::<T>::kill(),
            }

            Ok(PostDispatchInfo {
                actual_weight: Some(total_weight),
                pays_fee: Pays::No,
            })
        }

        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn set_authorities(
            origin: OriginFor<T>,
            authorities: Vec<T::AccountId>,
        ) -> DispatchResult {
            ensure_root(origin)?;
            Authorities::<T>::put(&authorities);
            Self::deposit_event(Event::AuthoritiesInserted { authorities });
            Ok(())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Auhtorities inserted
        AuthoritiesInserted { authorities: Vec<T::AccountId> },
    }

    #[pallet::storage]
    #[pallet::getter(fn authorities)]
    pub(super) type Authorities<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    #[pallet::inherent]
    impl<T: Config> ProvideInherent for Pallet<T> {
        type Call = Call<T>;
		type Error = InherentError;
        // TODO, what should we put here
        const INHERENT_IDENTIFIER: InherentIdentifier = INHERENT_IDENTIFIER;

        fn is_inherent_required(_: &InherentData) -> Result<Option<Self::Error>, Self::Error> {
			// Return Ok(Some(_)) unconditionally because this inherent is required in every block
			Ok(Some(InherentError::Other(
				sp_runtime::RuntimeString::Borrowed(
					"Inherent required",
				),
			)))
		}

        fn create_inherent(data: &InherentData) -> Option<Self::Call> {
            let data: tp_authorities_noting_inherent::ContainerChainAuthoritiesInherentData = data
                .get_data(&INHERENT_IDENTIFIER)
                .ok()
                .flatten()
                .expect("there is not data to be posted; qed");

            Some(Call::set_latest_authorities_data { data })
        }

        fn is_inherent(call: &Self::Call) -> bool {
            matches!(call, Call::set_latest_authorities_data { .. })
        }
    }
}

impl<T: Config> Pallet<T> {
    /// Fetch author slot from a proof of header
    fn fetch_orchestrator_header_from_relay_proof(
        relay_state_proof: &RelayChainHeaderStateProof,
        para_id: ParaId,
    ) -> Result<<BlakeTwo256 as HashT>::Output, Error<T>> {
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
        let orchestrator_chain_header =
            sp_runtime::generic::Header::<BlockNumber, BlakeTwo256>::decode(
                &mut head_data.0.as_slice(),
            )
            .map_err(|_| Error::<T>::FailedDecodingHeader)?
            .clone();

        let orchestrator_chain_storage_root = orchestrator_chain_header.state_root;

        Ok(orchestrator_chain_storage_root)
    }

    /// Fetch author slot from a proof of header
    fn fetch_authorities_from_orchestrator_proof(
        orchestrator_state_proof: &RelayChainHeaderStateProof,
        para_id: ParaId,
    ) -> Result<Vec<T::AccountId>, Error<T>> {
        let assignmnet = orchestrator_state_proof
            .read_entry::<AssignedCollators<T::AccountId>>(COLLATOR_ASSIGNMENT_INDEX, None)
            .map_err(|e| match e {
                ReadEntryErr::Proof => panic!("Invalid proof provided for para head key"),
                _ => Error::<T>::FailedReading,
            })?;

        let authorities = assignmnet
            .container_chains
            .get(&para_id.into())
            .ok_or(Error::<T>::NoAuthoritiesFound)?;
        Ok(authorities.clone())
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