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
//!
#![cfg_attr(not(feature = "std"), no_std)]
use cumulus_pallet_parachain_system::RelaychainStateProvider;
use cumulus_primitives_core::relay_chain::BlakeTwo256;
use cumulus_primitives_core::relay_chain::BlockNumber;
use cumulus_primitives_core::relay_chain::HeadData;
use cumulus_primitives_core::ParaId;
use frame_support::traits::Get;
use frame_support::Hashable;
use parity_scale_codec::Decode;
use parity_scale_codec::Encode;
use sp_inherents::{InherentIdentifier, IsFatalError};
use sp_runtime::traits::Hash as HashT;
use sp_runtime::RuntimeString;
use sp_std::prelude::*;
use tp_authorities_noting_inherent::INHERENT_IDENTIFIER;
use tp_chain_state_snapshot::*;
use tp_collator_assignment::AssignedCollators;
use tp_core::well_known_keys::COLLATOR_ASSIGNMENT_INDEX;
use tp_core::well_known_keys::PARAS_HEADS_INDEX;

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

        type RelayChainStateProvider: cumulus_pallet_parachain_system::RelaychainStateProvider;
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
                relay_chain_state: relay_chain_state_proof,
                orchestrator_chain_state: orchestrator_chain_state_proof,
            } = data;

            let relay_storage_root =
                T::RelayChainStateProvider::current_relay_chain_state().state_root;

            let para_id = T::OrchestratorParaId::get();
            let relay_chain_state_proof = RelayChainHeaderStateProof::new(
                relay_storage_root,
                relay_chain_state_proof.clone(),
            )
            .expect("Invalid relay chain state proof");

            // Fetch authorities
            let authorities = {
                let orchestrator_root = Self::fetch_orchestrator_header_from_relay_proof(
                    &relay_chain_state_proof,
                    para_id,
                )?;

                let orchestrator_chain_state_proof = OrchestratorChainHeaderStateProof::new(
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

        // Fetch the orchestrator chain storage root
        let orchestrator_chain_storage_root = orchestrator_chain_header.state_root;

        Ok(orchestrator_chain_storage_root)
    }

    /// Fetch author slot from a proof of header
    fn fetch_authorities_from_orchestrator_proof(
        orchestrator_state_proof: &OrchestratorChainHeaderStateProof,
        para_id: ParaId,
    ) -> Result<Vec<T::AccountId>, Error<T>> {
        // Read the assignment from the orchestrator
        let assignmnet = orchestrator_state_proof
            .read_entry::<AssignedCollators<T::AccountId>>(COLLATOR_ASSIGNMENT_INDEX, None)
            .map_err(|e| match e {
                ReadEntryErr::Proof => panic!("Invalid proof provided for para head key"),
                _ => Error::<T>::FailedReading,
            })?;

        // Read those authorities assigned to this chain
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
