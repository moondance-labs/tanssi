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
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>.

//! # Authorities Noting Pallet
//!
//! This pallet notes the authorities assigned to this container-chain in an orchestrator chain
//!
//! First the pallet receives a storage proof of the header of the orchestrator chain
//! Once the storage proof is verified against the relay, the storage root of the orchestrator
//! chain is retrieved from the header
//!  
//! A second storage proof is verified against the storage root of the orchestrator chain. From
//! this the collator-assignation is read, and the authorities assigned to these container-chain
//! are retrieved and stored

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod test;

pub use pallet::*;
use {
    ccp_authorities_noting_inherent::INHERENT_IDENTIFIER,
    cumulus_pallet_parachain_system::RelaychainStateProvider,
    cumulus_primitives_core::{
        relay_chain::{BlakeTwo256, BlockNumber, HeadData},
        ParaId,
    },
    frame_support::{dispatch::PostDispatchInfo, pallet_prelude::*, traits::Get, Hashable},
    frame_system::pallet_prelude::*,
    parity_scale_codec::{Decode, Encode},
    sp_inherents::{InherentIdentifier, IsFatalError},
    sp_runtime::{traits::Hash as HashT, RuntimeString},
    sp_std::{prelude::*},
    tp_chain_state_snapshot::*,
    tp_collator_assignment::AssignedCollators,
    tp_core::well_known_keys,
};

pub trait GetContainerChains {
    fn container_chains() -> Vec<ParaId>;
}

#[frame_support::pallet]
pub mod pallet {
    use parity_scale_codec::FullCodec;

    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type OrchestratorParaId: Get<ParaId>;

        type SelfParaId: Get<ParaId>;

        type RelayChainStateProvider: cumulus_pallet_parachain_system::RelaychainStateProvider;

        type AuthorityId: sp_std::fmt::Debug + PartialEq + Clone + FullCodec + TypeInfo;
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The new value for a configuration parameter is invalid.
        FailedReading,
        FailedDecodingHeader,
        NoAuthoritiesFound,
    }

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_n: T::BlockNumber) -> Weight {
            let mut weight = Weight::zero();

            // We clear this storage item to make sure its always included
            DidSetOrchestratorAuthorityData::<T>::kill();

            weight += T::DbWeight::get().writes(1);

            // The read onfinalizes
            weight += T::DbWeight::get().reads(1);

            weight
        }

        fn on_finalize(_: T::BlockNumber) {
            assert!(
                <DidSetOrchestratorAuthorityData<T>>::exists(),
                "Orchestrator chain authorities data needs to be present in every block!"
            );
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight((0, DispatchClass::Mandatory))]
        // TODO: This weight should be corrected.
        pub fn set_latest_authorities_data(
            origin: OriginFor<T>,
            data: ccp_authorities_noting_inherent::ContainerChainAuthoritiesInherentData,
        ) -> DispatchResultWithPostInfo {
            let total_weight = Weight::zero();
            ensure_none(origin)?;

            assert!(
                !<DidSetOrchestratorAuthorityData<T>>::exists(),
                "DidSetOrchestratorAuthorityData must be updated only once in a block",
            );

            let ccp_authorities_noting_inherent::ContainerChainAuthoritiesInherentData {
                relay_chain_state: relay_chain_state_proof,
                orchestrator_chain_state: orchestrator_chain_state_proof,
            } = data;

            let relay_storage_root =
                T::RelayChainStateProvider::current_relay_chain_state().state_root;

            let para_id = T::OrchestratorParaId::get();
            let relay_chain_state_proof =
                GenericStateProof::new(relay_storage_root, relay_chain_state_proof.clone())
                    .expect("Invalid relay chain state proof");

            // Fetch authorities
            let authorities = {
                let orchestrator_root = Self::fetch_orchestrator_header_from_relay_proof(
                    &relay_chain_state_proof,
                    para_id,
                )?;

                let orchestrator_chain_state_proof = GenericStateProof::new(
                    orchestrator_root,
                    orchestrator_chain_state_proof.clone(),
                )
                .expect("Invalid orchestrator chain state proof");

                Self::fetch_authorities_from_orchestrator_proof(
                    &orchestrator_chain_state_proof,
                    T::SelfParaId::get(),
                )
            };

            match authorities {
                Ok(authorities) => Authorities::<T>::put(authorities),
                Err(e) => {
                    log::warn!("Authorities-noting error {:?}", e);
                    Authorities::<T>::kill();
                }
            }

            DidSetOrchestratorAuthorityData::<T>::put(true);

            Ok(PostDispatchInfo {
                actual_weight: Some(total_weight),
                pays_fee: Pays::No,
            })
        }

        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn set_authorities(
            origin: OriginFor<T>,
            authorities: Vec<T::AuthorityId>,
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
        AuthoritiesInserted {
            authorities: Vec<T::AuthorityId>,
        },
    }

    #[pallet::storage]
    #[pallet::getter(fn authorities)]
    pub(super) type Authorities<T: Config> =
        StorageValue<_, Vec<T::AuthorityId>, ValueQuery>;

    /// Was the containerAuthorData set?
    #[pallet::storage]
    pub(super) type DidSetOrchestratorAuthorityData<T: Config> = StorageValue<_, bool, ValueQuery>;

    #[pallet::inherent]
    impl<T: Config> ProvideInherent for Pallet<T> {
        type Call = Call<T>;
        type Error = InherentError;
        const INHERENT_IDENTIFIER: InherentIdentifier = INHERENT_IDENTIFIER;

        fn is_inherent_required(_: &InherentData) -> Result<Option<Self::Error>, Self::Error> {
            // Return Ok(Some(_)) unconditionally because this inherent is required in every block
            Ok(Some(InherentError::Other(
                sp_runtime::RuntimeString::Borrowed(
                    "Orchestrator Authorities Noting Inherent required",
                ),
            )))
        }

        fn create_inherent(data: &InherentData) -> Option<Self::Call> {
            let data: ccp_authorities_noting_inherent::ContainerChainAuthoritiesInherentData = data
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
    /// TODO: fix me once we have a proper Block type
    fn fetch_orchestrator_header_from_relay_proof(
        relay_state_proof: &GenericStateProof<cumulus_primitives_core::relay_chain::Block>,
        para_id: ParaId,
    ) -> Result<<BlakeTwo256 as HashT>::Output, Error<T>> {
        let bytes = para_id.twox_64_concat();
        // CONCAT
        let key = [well_known_keys::PARAS_HEADS_INDEX, bytes.as_slice()].concat();
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

        // Fetch the orchestrator chain storage root
        let orchestrator_chain_storage_root = orchestrator_chain_header.state_root;

        Ok(orchestrator_chain_storage_root)
    }

    /// Fetch author slot from a proof of header
    fn fetch_authorities_from_orchestrator_proof(
        orchestrator_state_proof: &GenericStateProof<cumulus_primitives_core::relay_chain::Block>,
        para_id: ParaId,
    ) -> Result<Vec<T::AuthorityId>, Error<T>> {
        // Read orchestrator session index
        let session_index = orchestrator_state_proof
            .read_entry::<u32>(well_known_keys::SESSION_INDEX, None)
            .map_err(|e| match e {
                ReadEntryErr::Proof => panic!("Invalid proof: cannot read session index"),
                _ => Error::<T>::FailedReading,
            })?;

        // Read the assignment from the orchestrator
        let assignement = orchestrator_state_proof
            .read_entry::<AssignedCollators<T::AuthorityId>>(
                &well_known_keys::authority_assignment_for_session(session_index),
                None,
            )
            .map_err(|e| match e {
                ReadEntryErr::Proof => panic!("Invalid proof: cannot read assignement"),
                _ => Error::<T>::FailedReading,
            })?;

        // Read those authorities assigned to this chain
        let authorities = assignement
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
