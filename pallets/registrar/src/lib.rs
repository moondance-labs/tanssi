//! # Registrar Pallet
//!
//! This pallet is in charge of registering containerChains (identified by their Id)
//! that have to be served by the orchestrator chain. Parachains registrations and de-
//! registrations are not immediatly applied, but rather they take T::SessionDelay sessions
//! to be applied.
//!
//! Registered container chains are stored in the PendingParaIds storage item until the session
//! in which they can be onboarded arrives, in which case they are added to the RegisteredParaIds
//! storage item.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use {
        frame_support::{pallet_prelude::*, LOG_TARGET},
        frame_system::pallet_prelude::*,
        sp_runtime::{traits::AtLeast32BitUnsigned, Saturating},
        sp_std::prelude::*,
        tp_traits::{
            GetCurrentContainerChains, GetSessionContainerChains, GetSessionIndex, ParaId,
        },
    };

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::genesis_config]
    #[derive(Default)]
    pub struct GenesisConfig {
        /// Para ids
        pub para_ids: Vec<(ParaId, ContainerChainGenesisData)>,
    }

    // TODO: move this to tanssi primitives
    // TODO: improve serialization of storage field
    // Currently it looks like this:
    /*
    "storage": [
        {
          "key": "0x0d715f2646c8f85767b5d2764bb2782604a74d81251e398fd8a0a4d55023bb3f"
          "value": "0xd1070000"
        },
        {
          "key": "0x0d715f2646c8f85767b5d2764bb278264e7b9012096b41c4eb3aaf947f6ea429"
          "value": "0x0000"
        }
    ]
     */
    // Ideally it would be:
    /*
    "storage": {
        "0x0d715f2646c8f85767b5d2764bb2782604a74d81251e398fd8a0a4d55023bb3f": "0xd1070000",
        "0x0d715f2646c8f85767b5d2764bb278264e7b9012096b41c4eb3aaf947f6ea429": "0x0000"
    }
     */
    // This is just so it looks nicer on polkadot.js, the functionality is the same
    // The original approach of using `storage: BTreeMap<Vec<u8>, Vec<u8>>` looks very bad
    // in polkadot.js, because `Vec<u8>` is serialized as `[12, 51, 124]` instead of hex.
    // That's why we use `serde(with = "sp_core::bytes")` everywhere, to convert it to hex.
    #[cfg_attr(feature = "std", derive(serde::Deserialize, serde::Serialize))]
    #[derive(
        Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, scale_info::TypeInfo,
    )]
    pub struct ContainerChainGenesisData {
        pub storage: Vec<ContainerChainGenesisDataItem>,
        // TODO: make all these Vec<u8> bounded
        #[cfg_attr(feature = "std", serde(with = "sp_core::bytes"))]
        pub name: Vec<u8>,
        #[cfg_attr(feature = "std", serde(with = "sp_core::bytes"))]
        pub id: Vec<u8>,
        pub fork_id: Option<Vec<u8>>,
        #[cfg_attr(feature = "std", serde(with = "sp_core::bytes"))]
        pub extensions: Vec<u8>,
        pub properties: TokenMetadata,
    }

    // TODO: turn this into a Config type parameter
    // The problem with that is that it forces ContainerChainGenesisData to be generic,
    // and the automatically derived traits force the generic parameter to implement those traits.
    // The errors are like "MaxLengthTokenSymbol does not implement Debug".
    // The solution is to either implement all the traits manually, or use a helper crate like
    // derivative, although that does not seem to support deriving the substrate traits.
    pub struct MaxLengthTokenSymbol;

    impl Get<u32> for MaxLengthTokenSymbol {
        fn get() -> u32 {
            255
        }
    }

    #[cfg_attr(feature = "std", derive(serde::Deserialize, serde::Serialize))]
    #[derive(
        Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, scale_info::TypeInfo,
    )]
    pub struct TokenMetadata {
        pub token_symbol: BoundedVec<u8, MaxLengthTokenSymbol>,
        pub ss58_format: u32,
        pub token_decimals: u32,
    }

    impl Default for TokenMetadata {
        fn default() -> Self {
            // Default values from polkadot.js
            Self {
                token_symbol: BoundedVec::truncate_from(b"UNIT".to_vec()),
                ss58_format: 42,
                token_decimals: 12,
            }
        }
    }

    #[cfg_attr(feature = "std", derive(serde::Deserialize, serde::Serialize))]
    #[derive(
        Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, scale_info::TypeInfo,
    )]
    pub struct ContainerChainGenesisDataItem {
        #[cfg_attr(feature = "std", serde(with = "sp_core::bytes"))]
        pub key: Vec<u8>,
        #[cfg_attr(feature = "std", serde(with = "sp_core::bytes"))]
        pub value: Vec<u8>,
    }

    impl From<(Vec<u8>, Vec<u8>)> for ContainerChainGenesisDataItem {
        fn from(x: (Vec<u8>, Vec<u8>)) -> Self {
            Self {
                key: x.0,
                value: x.1,
            }
        }
    }

    impl From<ContainerChainGenesisDataItem> for (Vec<u8>, Vec<u8>) {
        fn from(x: ContainerChainGenesisDataItem) -> Self {
            (x.key, x.value)
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig {
        fn build(&self) {
            // Sort para ids and detect duplicates, but do it using a vector of
            // references to avoid cloning the genesis data, which may be big.
            let mut para_ids: Vec<&_> = self.para_ids.iter().collect();
            para_ids.sort();
            para_ids.dedup_by(|a, b| {
                if a.0 == b.0 {
                    panic!("Duplicate para_id: {}", a.0);
                } else {
                    false
                }
            });

            let mut bounded_para_ids = BoundedVec::truncate_from(vec![]);

            for (para_id, genesis_data) in para_ids {
                bounded_para_ids
                    .try_push(*para_id)
                    .expect("too many para ids in genesis: bounded vec full");

                let genesis_data_size = genesis_data.encoded_size();
                if genesis_data_size > T::MaxGenesisDataSize::get() as usize {
                    panic!(
                        "genesis data for para_id {:?} is too large: {} bytes (limit is {})",
                        u32::from(*para_id),
                        genesis_data_size,
                        T::MaxGenesisDataSize::get()
                    );
                }
                <ParaGenesisData<T>>::insert(para_id, genesis_data);
            }

            <RegisteredParaIds<T>>::put(bounded_para_ids);
        }
    }

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Origin that is allowed to call register and deregister
        type RegistrarOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        /// Max length of para id list
        #[pallet::constant]
        type MaxLengthParaIds: Get<u32>;

        /// Max length of encoded genesis data
        #[pallet::constant]
        type MaxGenesisDataSize: Get<u32>;

        type SessionIndex: parity_scale_codec::FullCodec + TypeInfo + Copy + AtLeast32BitUnsigned;

        type SessionDelay: Get<Self::SessionIndex>;

        type CurrentSessionIndex: GetSessionIndex<Self::SessionIndex>;
    }

    #[pallet::storage]
    #[pallet::getter(fn registered_para_ids)]
    pub type RegisteredParaIds<T: Config> =
        StorageValue<_, BoundedVec<ParaId, T::MaxLengthParaIds>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn pending_registered_para_ids)]
    pub type PendingParaIds<T: Config> = StorageValue<
        _,
        Vec<(T::SessionIndex, BoundedVec<ParaId, T::MaxLengthParaIds>)>,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn para_genesis_data)]
    pub type ParaGenesisData<T: Config> =
        StorageMap<_, Blake2_128Concat, ParaId, ContainerChainGenesisData, OptionQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new para id has been registered. [para_id]
        ParaIdRegistered { para_id: ParaId },
        /// A para id has been deregistered. [para_id]
        ParaIdDeregistered { para_id: ParaId },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Attempted to register a ParaId that was already registered
        ParaIdAlreadyRegistered,
        /// Attempted to deregister a ParaId that is not registered
        ParaIdNotRegistered,
        /// The bounded list of ParaIds has reached its limit
        ParaIdListFull,
        /// Attempted to register a ParaId with a genesis data size greater than the limit
        GenesisDataTooBig,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register parachain
        #[pallet::call_index(0)]
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,2).ref_time())]
        pub fn register(
            origin: OriginFor<T>,
            para_id: ParaId,
            genesis_data: ContainerChainGenesisData,
        ) -> DispatchResult {
            T::RegistrarOrigin::ensure_origin(origin)?;
            Self::schedule_parachain_change(|para_ids| {
                // We don't want to add duplicate para ids, so we check whether the potential new
                // para id is already present in the list. Because the list is always ordered, we can
                // leverage the binary search which makes this check O(log n).
                let result = match para_ids.binary_search(&para_id) {
                    Ok(_) => Err(Error::<T>::ParaIdAlreadyRegistered.into()),
                    Err(index) => {
                        para_ids
                            .try_insert(index, para_id)
                            .map_err(|_e| Error::<T>::ParaIdListFull)?;

                        Ok(())
                    }
                };
                result
            })?;

            // TODO: while the registration takes place on the next session, the genesis data
            // is inserted immediately. This is because collators should be able to start syncing
            // the new container chain before the first block is mined. However, we could store
            // the genesis data in another key, like PendingParaGenesisData.
            // TODO: for benchmarks, this call to .encoded_size is O(n) with respect to the number
            // of key-values in `genesis_data.storage`, even if those key-values are empty. And we
            // won't detect that the size is too big until after iterating over all of them, so the
            // limit in that case would be the transaction size.
            let genesis_data_size = genesis_data.encoded_size();
            if genesis_data_size > T::MaxGenesisDataSize::get() as usize {
                return Err(Error::<T>::GenesisDataTooBig.into());
            }
            ParaGenesisData::<T>::insert(para_id, genesis_data);

            Self::deposit_event(Event::ParaIdRegistered { para_id });
            Ok(())
        }

        /// Deregister parachain
        #[pallet::call_index(1)]
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
        pub fn deregister(origin: OriginFor<T>, para_id: ParaId) -> DispatchResult {
            T::RegistrarOrigin::ensure_origin(origin)?;

            Self::schedule_parachain_change(|para_ids| {
                // We have to find out where, in the sorted vec the para id is, if anywhere.
                let result = match para_ids.binary_search(&para_id) {
                    Ok(index) => {
                        para_ids.remove(index);
                        Ok(())
                    }
                    Err(_) => Err(Error::<T>::ParaIdNotRegistered.into()),
                };
                result
            })?;
            Self::deposit_event(Event::ParaIdDeregistered { para_id });

            // TODO: while the deregistration takes place on the next session, the genesis data
            // is deleted immediately. This will cause problems since any new collators that want
            // to join now will not be able to sync this parachain
            ParaGenesisData::<T>::remove(para_id);

            Ok(())
        }
    }

    pub struct SessionChangeOutcome<T: Config> {
        /// Previously active parachains.
        pub prev_paras: BoundedVec<ParaId, T::MaxLengthParaIds>,
        /// If new parachains have been applied in the new session, this is the new  list.
        pub new_paras: Option<BoundedVec<ParaId, T::MaxLengthParaIds>>,
    }

    impl<T: Config> Pallet<T> {
        #[inline(never)]
        fn schedule_parachain_change(
            updater: impl FnOnce(&mut BoundedVec<ParaId, T::MaxLengthParaIds>) -> DispatchResult,
        ) -> DispatchResult {
            let mut pending_paras = PendingParaIds::<T>::get();
            // First, we need to decide what we should use as the base paras.
            let mut base_paras = pending_paras
                .last()
                .map(|&(_, ref paras)| paras.clone())
                .unwrap_or_else(Self::registered_para_ids);

            updater(&mut base_paras)?;
            let new_paras = base_paras;

            let scheduled_session = Self::scheduled_session();

            if let Some(&mut (_, ref mut paras)) = pending_paras
                .iter_mut()
                .find(|&&mut (apply_at_session, _)| apply_at_session >= scheduled_session)
            {
                *paras = new_paras;
            } else {
                // We are scheduling a new parachains change for the scheduled session.
                pending_paras.push((scheduled_session, new_paras));
            }

            <PendingParaIds<T>>::put(pending_paras);

            Ok(())
        }

        /// Return the session index that should be used for any future scheduled changes.
        fn scheduled_session() -> T::SessionIndex {
            T::CurrentSessionIndex::session_index().saturating_add(T::SessionDelay::get())
        }

        /// Called by the initializer to note that a new session has started.
        ///
        /// Returns the parachain list that was actual before the session change and the parachain list
        /// that became active after the session change. If there were no scheduled changes, both will
        /// be the same.
        pub fn initializer_on_new_session(
            session_index: &T::SessionIndex,
        ) -> SessionChangeOutcome<T> {
            let pending_paras = <PendingParaIds<T>>::get();
            let prev_paras = RegisteredParaIds::<T>::get();

            // No pending parachain changes, so we're done.
            if pending_paras.is_empty() {
                return SessionChangeOutcome {
                    prev_paras,
                    new_paras: None,
                };
            }

            let (mut past_and_present, future) =
                pending_paras
                    .into_iter()
                    .partition::<Vec<_>, _>(|&(apply_at_session, _)| {
                        apply_at_session <= *session_index
                    });

            if past_and_present.len() > 1 {
                // This should never happen since we schedule parachain changes only into the future
                // sessions and this handler called for each session change.
                log::error!(
                    target: LOG_TARGET,
                    "Skipping applying parachain changes scheduled sessions in the past",
                );
            }

            let new_paras = past_and_present.pop().map(|(_, paras)| paras);
            if let Some(ref new_paras) = new_paras {
                // Apply the new parachain list.
                RegisteredParaIds::<T>::put(new_paras);
            }

            <PendingParaIds<T>>::put(future);

            SessionChangeOutcome {
                prev_paras,
                new_paras,
            }
        }
    }

    impl<T: Config> GetCurrentContainerChains for Pallet<T> {
        fn current_container_chains() -> Vec<ParaId> {
            Self::registered_para_ids()
                .into_iter()
                .map(|x| x.into())
                .collect()
        }
    }

    impl<T: Config> GetSessionContainerChains<T::SessionIndex> for Pallet<T> {
        fn session_container_chains(session_index: T::SessionIndex) -> Vec<ParaId> {
            let (past_and_present, _) = Pallet::<T>::pending_registered_para_ids()
                .into_iter()
                .partition::<Vec<_>, _>(|&(apply_at_session, _)| apply_at_session <= session_index);

            let paras = if let Some(last) = past_and_present.last() {
                last.1.clone()
            } else {
                Pallet::<T>::registered_para_ids()
            };

            paras.into_iter().map(|x| ParaId::from(x)).collect()
        }
    }
}
