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

//! # Registrar Pallet
//!
//! This pallet is in charge of registering containerChains (identified by their Id)
//! that have to be served by the orchestrator chain. Parachains registrations and de-
//! registrations are not immediately applied, but rather they take T::SessionDelay sessions
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

#[cfg(any(test, feature = "runtime-benchmarks"))]
mod benchmark_blob;
#[cfg(any(test, feature = "runtime-benchmarks"))]
mod benchmarks;
pub mod weights;
pub use weights::WeightInfo;

pub use pallet::*;

use {
    cumulus_primitives_core::relay_chain::HeadData,
    dp_chain_state_snapshot::GenericStateProof,
    dp_container_chain_genesis_data::ContainerChainGenesisData,
    dp_core::{well_known_keys::REGISTRAR_PARAS_INDEX, ParaInfo},
    frame_support::{
        pallet_prelude::*,
        traits::{
            fungible::{Inspect, InspectHold, Mutate, MutateHold},
            tokens::{Fortitude, Precision, Restriction},
            EnsureOriginWithArg,
        },
        DefaultNoBound, Hashable, LOG_TARGET,
    },
    frame_system::pallet_prelude::*,
    parity_scale_codec::{Decode, Encode},
    sp_core::H256,
    sp_runtime::{
        traits::{AtLeast32BitUnsigned, Verify},
        Saturating,
    },
    sp_std::{collections::btree_set::BTreeSet, prelude::*},
    tp_traits::{
        GetCurrentContainerChains, GetSessionContainerChains, GetSessionIndex, ParaId,
        ParathreadParams as ParathreadParamsTy, RegistrarHandler, RelayStorageRootProvider,
        SessionContainerChains, SlotFrequency,
    },
};

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::genesis_config]
    #[derive(DefaultNoBound)]
    pub struct GenesisConfig<T: Config> {
        /// Para ids
        pub para_ids: Vec<(
            ParaId,
            ContainerChainGenesisData,
            Option<ParathreadParamsTy>,
        )>,
        #[serde(skip)]
        pub phantom: PhantomData<T>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            // Sort para ids and detect duplicates, but do it using a vector of
            // references to avoid cloning the genesis data, which may be big.
            let mut para_ids: Vec<&_> = self.para_ids.iter().collect();
            para_ids.sort_by(|a, b| a.0.cmp(&b.0));
            para_ids.dedup_by(|a, b| {
                if a.0 == b.0 {
                    panic!("Duplicate para_id: {}", u32::from(a.0));
                } else {
                    false
                }
            });

            let mut bounded_para_ids = BoundedVec::default();

            for (para_id, genesis_data, parathread_params) in para_ids {
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

                if let Some(parathread_params) = parathread_params {
                    <ParathreadParams<T>>::insert(para_id, parathread_params);
                }
            }

            <RegisteredParaIds<T>>::put(bounded_para_ids);
        }
    }

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Origin that is allowed to call maintenance extrinsics for container owner
        type RegistrarOrigin: EnsureOriginWithArg<Self::RuntimeOrigin, ParaId>;

        /// Origin that is allowed to call mark_valid_for_collating
        type MarkValidForCollatingOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        /// Max length of para id list
        #[pallet::constant]
        type MaxLengthParaIds: Get<u32>;

        /// Max length of encoded genesis data
        #[pallet::constant]
        type MaxGenesisDataSize: Get<u32>;

        type RegisterWithRelayProofOrigin: EnsureOrigin<
            Self::RuntimeOrigin,
            Success = Self::AccountId,
        >;

        type RelayStorageRootProvider: RelayStorageRootProvider;

        type SessionIndex: parity_scale_codec::FullCodec + TypeInfo + Copy + AtLeast32BitUnsigned;

        #[pallet::constant]
        type SessionDelay: Get<Self::SessionIndex>;

        type CurrentSessionIndex: GetSessionIndex<Self::SessionIndex>;

        type Currency: Mutate<Self::AccountId>
            + MutateHold<Self::AccountId, Reason = Self::RuntimeHoldReason>;

        type RuntimeHoldReason: From<HoldReason>;

        type RegistrarHooks: RegistrarHooks;

        /// External manager that takes care of executing specific operations
        /// when register-like functions of this pallet are called.
        ///
        /// Mostly used when we are in a relay-chain configuration context (Dancelight)
        /// to also register, deregister and upgrading paraIds in polkadot's
        /// paras_registrar pallet.
        type InnerRegistrar: RegistrarHandler<Self::AccountId>;

        type WeightInfo: WeightInfo;

        #[pallet::constant]
        type DataDepositPerByte: Get<<Self::Currency as Inspect<Self::AccountId>>::Balance>;
    }

    #[pallet::storage]
    pub type RegisteredParaIds<T: Config> =
        StorageValue<_, BoundedVec<ParaId, T::MaxLengthParaIds>, ValueQuery>;

    #[pallet::storage]
    #[pallet::unbounded]
    pub type PendingParaIds<T: Config> = StorageValue<
        _,
        Vec<(T::SessionIndex, BoundedVec<ParaId, T::MaxLengthParaIds>)>,
        ValueQuery,
    >;

    #[pallet::storage]
    // TODO: this is not unbounded because we check the encoded size in register
    #[pallet::unbounded]
    pub type ParaGenesisData<T: Config> =
        StorageMap<_, Blake2_128Concat, ParaId, ContainerChainGenesisData, OptionQuery>;

    #[pallet::storage]
    pub type PendingVerification<T: Config> =
        StorageMap<_, Blake2_128Concat, ParaId, (), OptionQuery>;

    #[pallet::storage]
    pub type Paused<T: Config> =
        StorageValue<_, BoundedVec<ParaId, T::MaxLengthParaIds>, ValueQuery>;

    #[pallet::storage]
    #[pallet::unbounded]
    pub type PendingPaused<T: Config> = StorageValue<
        _,
        Vec<(T::SessionIndex, BoundedVec<ParaId, T::MaxLengthParaIds>)>,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::unbounded]
    pub type PendingToRemove<T: Config> = StorageValue<
        _,
        Vec<(T::SessionIndex, BoundedVec<ParaId, T::MaxLengthParaIds>)>,
        ValueQuery,
    >;

    #[pallet::storage]
    pub type ParathreadParams<T: Config> =
        StorageMap<_, Blake2_128Concat, ParaId, ParathreadParamsTy, OptionQuery>;

    #[pallet::storage]
    #[pallet::unbounded]
    pub type PendingParathreadParams<T: Config> = StorageValue<
        _,
        Vec<(
            T::SessionIndex,
            BoundedVec<(ParaId, ParathreadParamsTy), T::MaxLengthParaIds>,
        )>,
        ValueQuery,
    >;

    /// This storage aims to act as a 'buffer' for paraIds that must be deregistered at the
    /// end of the block execution by calling 'T::InnerRegistrar::deregister()' implementation.
    ///
    /// We need this buffer because when we are using this pallet on a relay-chain environment
    /// like Dancelight (where 'T::InnerRegistrar' implementation is usually the
    /// 'paras_registrar' pallet) we need to deregister (via 'paras_registrar::deregister')
    /// the same paraIds we have in 'PendingToRemove<T>', and we need to do this deregistration
    /// process inside 'on_finalize' hook.
    ///
    /// It can be the case that some paraIds need to be downgraded to a parathread before
    /// deregistering on 'paras_registrar'. This process usually takes 2 sessions,
    /// and the actual downgrade happens when the block finalizes.
    ///
    /// Therefore, if we tried to perform this relay deregistration process at the beginning
    /// of the session/block inside ('on_initialize') initializer_on_new_session() as we do
    /// for this pallet, it would fail due to the downgrade process could have not taken
    /// place yet.
    #[pallet::storage]
    pub type BufferedParasToDeregister<T: Config> =
        StorageValue<_, BoundedVec<ParaId, T::MaxLengthParaIds>, ValueQuery>;

    pub type DepositBalanceOf<T> =
        <<T as Config>::Currency as Inspect<<T as frame_system::Config>::AccountId>>::Balance;

    #[derive(
        Default, Clone, Encode, Decode, RuntimeDebug, PartialEq, scale_info::TypeInfo, MaxEncodedLen,
    )]
    #[scale_info(skip_type_params(T))]
    pub struct DepositInfo<T: Config> {
        pub creator: T::AccountId,
        pub deposit: DepositBalanceOf<T>,
    }

    /// Registrar deposits, a mapping from paraId to a struct
    /// holding the creator (from which the deposit was reserved) and
    /// the deposit amount
    #[pallet::storage]
    pub type RegistrarDeposit<T: Config> = StorageMap<_, Blake2_128Concat, ParaId, DepositInfo<T>>;

    #[pallet::storage]
    pub type ParaManager<T: Config> =
        StorageMap<_, Blake2_128Concat, ParaId, T::AccountId, OptionQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new para id has been registered. [para_id]
        ParaIdRegistered { para_id: ParaId },
        /// A para id has been deregistered. [para_id]
        ParaIdDeregistered { para_id: ParaId },
        /// A new para id is now valid for collating. [para_id]
        ParaIdValidForCollating { para_id: ParaId },
        /// A para id has been paused from collating.
        ParaIdPaused { para_id: ParaId },
        /// A para id has been unpaused.
        ParaIdUnpaused { para_id: ParaId },
        /// Parathread params changed
        ParathreadParamsChanged { para_id: ParaId },
        /// Para manager has changed
        ParaManagerChanged {
            para_id: ParaId,
            manager_address: T::AccountId,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Attempted to register a ParaId that was already registered
        ParaIdAlreadyRegistered,
        /// Attempted to deregister a ParaId that is not registered
        ParaIdNotRegistered,
        /// Attempted to deregister a ParaId that is already being deregistered
        ParaIdAlreadyDeregistered,
        /// Attempted to pause a ParaId that was already paused
        ParaIdAlreadyPaused,
        /// Attempted to unpause a ParaId that was not paused
        ParaIdNotPaused,
        /// The bounded list of ParaIds has reached its limit
        ParaIdListFull,
        /// Attempted to register a ParaId with a genesis data size greater than the limit
        GenesisDataTooBig,
        /// Tried to mark_valid_for_collating a ParaId that is not in PendingVerification
        ParaIdNotInPendingVerification,
        /// Tried to register a ParaId with an account that did not have enough balance for the deposit
        NotSufficientDeposit,
        /// Tried to change parathread params for a para id that is not a registered parathread
        NotAParathread,
        /// Attempted to execute an extrinsic meant only for the para creator
        NotParaCreator,
        /// The relay storage root for the corresponding block number could not be retrieved
        RelayStorageRootNotFound,
        /// The provided relay storage proof is not valid
        InvalidRelayStorageProof,
        /// The provided signature from the parachain manager in the relay is not valid
        InvalidRelayManagerSignature,
        /// Tried to deregister a parachain that was not deregistered from the relay chain
        ParaStillExistsInRelay,
        /// Tried to register a paraId in a relay context without specifying a proper HeadData.
        HeadDataNecessary,
        /// Tried to register a paraId in a relay context without specifying a wasm chain code.
        WasmCodeNecessary,
    }

    #[pallet::composite_enum]
    pub enum HoldReason {
        RegistrarDeposit,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
            let mut weight = Weight::zero().saturating_add(T::DbWeight::get().reads_writes(1, 1));

            let buffered_paras = BufferedParasToDeregister::<T>::take();

            for para_id in buffered_paras {
                weight.saturating_accrue(T::InnerRegistrar::deregister_weight());
                // Deregister (in the relay context) each paraId present inside the buffer
                T::InnerRegistrar::deregister(para_id);
            }
            weight
        }

        #[cfg(feature = "try-runtime")]
        fn try_state(_n: BlockNumberFor<T>) -> Result<(), sp_runtime::TryRuntimeError> {
            use {scale_info::prelude::format, sp_std::collections::btree_set::BTreeSet};
            // A para id can only be in 1 of [`RegisteredParaIds`, `PendingVerification`, `Paused`]
            // Get all those para ids and check for duplicates
            let mut para_ids: Vec<ParaId> = vec![];
            para_ids.extend(RegisteredParaIds::<T>::get());
            para_ids.extend(PendingVerification::<T>::iter_keys());
            para_ids.extend(Paused::<T>::get());
            para_ids.sort();
            para_ids.dedup_by(|a, b| {
                if a == b {
                    panic!("Duplicate para id: {}", u32::from(*a));
                } else {
                    false
                }
            });

            // All para ids have an entry in `ParaGenesisData`
            for para_id in &para_ids {
                assert!(
                    ParaGenesisData::<T>::contains_key(para_id),
                    "Para id {} missing genesis data",
                    u32::from(*para_id)
                );
            }

            // All entries in `RegistrarDeposit` and `ParaGenesisData` are in one of the other lists
            let mut para_id_set = BTreeSet::from_iter(para_ids.iter().cloned());
            // Also add the Pending lists here
            para_id_set.extend(
                PendingParaIds::<T>::get()
                    .into_iter()
                    .flat_map(|(_session_index, x)| x),
            );
            para_id_set.extend(
                PendingPaused::<T>::get()
                    .into_iter()
                    .flat_map(|(_session_index, x)| x),
            );
            para_id_set.extend(
                PendingToRemove::<T>::get()
                    .into_iter()
                    .flat_map(|(_session_index, x)| x),
            );
            let entries: Vec<_> = RegistrarDeposit::<T>::iter().map(|(k, _v)| k).collect();
            for para_id in entries {
                assert!(
                    para_id_set.contains(&para_id),
                    "Found RegistrarDeposit for unknown para id: {}",
                    u32::from(para_id)
                );
            }
            let entries: Vec<_> = ParaGenesisData::<T>::iter().map(|(k, _v)| k).collect();
            for para_id in entries {
                assert!(
                    para_id_set.contains(&para_id),
                    "Found ParaGenesisData for unknown para id: {}",
                    u32::from(para_id)
                );
            }

            // Sorted storage items are sorted
            fn assert_is_sorted_and_unique<T: Ord>(x: &[T], name: &str) {
                assert!(
                    x.windows(2).all(|w| w[0] < w[1]),
                    "sorted list not sorted or not unique: {}",
                    name,
                );
            }
            assert_is_sorted_and_unique(&RegisteredParaIds::<T>::get(), "RegisteredParaIds");
            assert_is_sorted_and_unique(&Paused::<T>::get(), "Paused");
            for (i, (_session_index, x)) in PendingParaIds::<T>::get().into_iter().enumerate() {
                assert_is_sorted_and_unique(&x, &format!("PendingParaIds[{}]", i));
            }
            for (i, (_session_index, x)) in PendingPaused::<T>::get().into_iter().enumerate() {
                assert_is_sorted_and_unique(&x, &format!("PendingPaused[{}]", i));
            }
            for (i, (_session_index, x)) in PendingToRemove::<T>::get().into_iter().enumerate() {
                assert_is_sorted_and_unique(&x, &format!("PendingToRemove[{}]", i));
            }

            // Pending storage items are sorted and session index is unique
            let pending: Vec<_> = PendingParaIds::<T>::get()
                .into_iter()
                .map(|(session_index, _x)| session_index)
                .collect();
            assert_is_sorted_and_unique(&pending, "PendingParaIds");
            let pending: Vec<_> = PendingPaused::<T>::get()
                .into_iter()
                .map(|(session_index, _x)| session_index)
                .collect();
            assert_is_sorted_and_unique(&pending, "PendingPaused");
            let pending: Vec<_> = PendingToRemove::<T>::get()
                .into_iter()
                .map(|(session_index, _x)| session_index)
                .collect();
            assert_is_sorted_and_unique(&pending, "PendingToRemove");

            Ok(())
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register container-chain
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::register(genesis_data.encoded_size() as u32, genesis_data.storage.len() as u32))]
        pub fn register(
            origin: OriginFor<T>,
            para_id: ParaId,
            genesis_data: ContainerChainGenesisData,
            head_data: Option<HeadData>,
        ) -> DispatchResult {
            let account = ensure_signed(origin)?;
            Self::do_register(account, para_id, genesis_data, head_data)?;
            Self::deposit_event(Event::ParaIdRegistered { para_id });

            Ok(())
        }

        /// Deregister container-chain.
        ///
        /// If a container-chain is registered but not marked as valid_for_collating, this will remove it
        /// from `PendingVerification` as well.
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::deregister_immediate(
        ).max(T::WeightInfo::deregister_scheduled(
        )))]
        pub fn deregister(origin: OriginFor<T>, para_id: ParaId) -> DispatchResult {
            T::RegistrarOrigin::ensure_origin(origin, &para_id)?;

            Self::do_deregister(para_id)?;

            Ok(())
        }

        /// Mark container-chain valid for collating
        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::mark_valid_for_collating())]
        pub fn mark_valid_for_collating(origin: OriginFor<T>, para_id: ParaId) -> DispatchResult {
            T::MarkValidForCollatingOrigin::ensure_origin(origin)?;

            Self::do_mark_valid_for_collating(para_id)?;

            Ok(())
        }

        /// Pause container-chain from collating. Does not remove its boot nodes nor its genesis config.
        /// Only container-chains that have been marked as valid_for_collating can be paused.
        #[pallet::call_index(4)]
        #[pallet::weight(T::WeightInfo::pause_container_chain())]
        pub fn pause_container_chain(origin: OriginFor<T>, para_id: ParaId) -> DispatchResult {
            T::RegistrarOrigin::ensure_origin(origin, &para_id)?;

            Self::schedule_paused_parachain_change(|para_ids, paused| {
                match paused.binary_search(&para_id) {
                    Ok(_) => return Err(Error::<T>::ParaIdAlreadyPaused.into()),
                    Err(index) => {
                        paused
                            .try_insert(index, para_id)
                            .map_err(|_e| Error::<T>::ParaIdListFull)?;
                    }
                }
                match para_ids.binary_search(&para_id) {
                    Ok(index) => {
                        para_ids.remove(index);
                    }
                    // We can only pause para ids that are marked as valid,
                    // otherwise unpausing them later would cause problems
                    Err(_) => return Err(Error::<T>::ParaIdNotRegistered.into()),
                }
                Self::deposit_event(Event::ParaIdPaused { para_id });

                Ok(())
            })?;

            Ok(())
        }

        /// Unpause container-chain.
        /// Only container-chains that have been paused can be unpaused.
        #[pallet::call_index(5)]
        #[pallet::weight(T::WeightInfo::unpause_container_chain())]
        pub fn unpause_container_chain(origin: OriginFor<T>, para_id: ParaId) -> DispatchResult {
            T::RegistrarOrigin::ensure_origin(origin, &para_id)?;

            Self::schedule_paused_parachain_change(|para_ids, paused| {
                match paused.binary_search(&para_id) {
                    Ok(index) => {
                        paused.remove(index);
                    }
                    Err(_) => return Err(Error::<T>::ParaIdNotPaused.into()),
                }
                match para_ids.binary_search(&para_id) {
                    // This Ok is unreachable, a para id cannot be in "RegisteredParaIds" and "Paused" at the same time
                    Ok(_) => return Err(Error::<T>::ParaIdAlreadyRegistered.into()),
                    Err(index) => {
                        para_ids
                            .try_insert(index, para_id)
                            .map_err(|_e| Error::<T>::ParaIdListFull)?;
                    }
                }
                Self::deposit_event(Event::ParaIdUnpaused { para_id });

                Ok(())
            })?;

            Ok(())
        }

        /// Register parathread
        #[pallet::call_index(6)]
        #[pallet::weight(T::WeightInfo::register_parathread(genesis_data.encoded_size() as u32, genesis_data.storage.len() as u32))]
        pub fn register_parathread(
            origin: OriginFor<T>,
            para_id: ParaId,
            slot_frequency: SlotFrequency,
            genesis_data: ContainerChainGenesisData,
            head_data: Option<HeadData>,
        ) -> DispatchResult {
            let account = ensure_signed(origin)?;
            Self::do_register(account, para_id, genesis_data, head_data)?;
            // Insert parathread params
            let params = ParathreadParamsTy { slot_frequency };
            ParathreadParams::<T>::insert(para_id, params);
            Self::deposit_event(Event::ParaIdRegistered { para_id });

            Ok(())
        }

        /// Change parathread params
        #[pallet::call_index(7)]
        #[pallet::weight(T::WeightInfo::set_parathread_params())]
        pub fn set_parathread_params(
            origin: OriginFor<T>,
            para_id: ParaId,
            slot_frequency: SlotFrequency,
        ) -> DispatchResult {
            T::RegistrarOrigin::ensure_origin(origin, &para_id)?;

            Self::schedule_parathread_params_change(para_id, |params| {
                params.slot_frequency = slot_frequency;

                Self::deposit_event(Event::ParathreadParamsChanged { para_id });

                Ok(())
            })?;

            Ok(())
        }

        #[pallet::call_index(8)]
        #[pallet::weight(T::WeightInfo::set_para_manager())]
        pub fn set_para_manager(
            origin: OriginFor<T>,
            para_id: ParaId,
            manager_address: T::AccountId,
        ) -> DispatchResult {
            // Allow root to force set para manager.
            if let Some(origin) = ensure_signed_or_root(origin)? {
                let creator =
                    RegistrarDeposit::<T>::get(para_id).map(|deposit_info| deposit_info.creator);

                ensure!(Some(origin) == creator, Error::<T>::NotParaCreator);
            }

            ParaManager::<T>::insert(para_id, manager_address.clone());

            Self::deposit_event(Event::<T>::ParaManagerChanged {
                para_id,
                manager_address,
            });

            Ok(())
        }

        /// Register parachain or parathread
        #[pallet::call_index(9)]
        #[pallet::weight(T::WeightInfo::register_with_relay_proof(genesis_data.encoded_size() as u32, genesis_data.storage.len() as u32))]
        pub fn register_with_relay_proof(
            origin: OriginFor<T>,
            para_id: ParaId,
            parathread_params: Option<ParathreadParamsTy>,
            relay_proof_block_number: u32,
            relay_storage_proof: sp_trie::StorageProof,
            manager_signature: cumulus_primitives_core::relay_chain::Signature,
            genesis_data: ContainerChainGenesisData,
            head_data: Option<HeadData>,
        ) -> DispatchResult {
            let account = T::RegisterWithRelayProofOrigin::ensure_origin(origin)?;
            let relay_storage_root =
                T::RelayStorageRootProvider::get_relay_storage_root(relay_proof_block_number)
                    .ok_or(Error::<T>::RelayStorageRootNotFound)?;
            let relay_state_proof =
                GenericStateProof::<cumulus_primitives_core::relay_chain::Block>::new(
                    relay_storage_root,
                    relay_storage_proof,
                )
                .map_err(|_| Error::<T>::InvalidRelayStorageProof)?;

            let bytes = para_id.twox_64_concat();
            let key = [REGISTRAR_PARAS_INDEX, bytes.as_slice()].concat();
            let relay_para_info = relay_state_proof
                .read_entry::<ParaInfo<
                    cumulus_primitives_core::relay_chain::AccountId,
                    cumulus_primitives_core::relay_chain::Balance,
                >>(key.as_slice(), None)
                .map_err(|_| Error::<T>::InvalidRelayStorageProof)?;
            let relay_manager = relay_para_info.manager;

            // Verify manager signature
            let signature_msg = Self::relay_signature_msg(para_id, &account, relay_storage_root);
            if !manager_signature.verify(&*signature_msg, &relay_manager) {
                return Err(Error::<T>::InvalidRelayManagerSignature.into());
            }

            Self::do_register(account, para_id, genesis_data, head_data)?;
            // Insert parathread params
            if let Some(parathread_params) = parathread_params {
                ParathreadParams::<T>::insert(para_id, parathread_params);
            }
            Self::deposit_event(Event::ParaIdRegistered { para_id });

            Ok(())
        }

        /// Deregister a parachain that no longer exists in the relay chain. The origin of this
        /// extrinsic will be rewarded with the parachain deposit.
        #[pallet::call_index(10)]
        #[pallet::weight(T::WeightInfo::deregister_with_relay_proof_immediate(
        ).max(T::WeightInfo::deregister_with_relay_proof_scheduled(
        )))]
        pub fn deregister_with_relay_proof(
            origin: OriginFor<T>,
            para_id: ParaId,
            relay_proof_block_number: u32,
            relay_storage_proof: sp_trie::StorageProof,
        ) -> DispatchResult {
            let account = T::RegisterWithRelayProofOrigin::ensure_origin(origin)?;

            let relay_storage_root =
                T::RelayStorageRootProvider::get_relay_storage_root(relay_proof_block_number)
                    .ok_or(Error::<T>::RelayStorageRootNotFound)?;
            let relay_state_proof =
                GenericStateProof::<cumulus_primitives_core::relay_chain::Block>::new(
                    relay_storage_root,
                    relay_storage_proof,
                )
                .map_err(|_| Error::<T>::InvalidRelayStorageProof)?;

            let bytes = para_id.twox_64_concat();
            let key = [REGISTRAR_PARAS_INDEX, bytes.as_slice()].concat();
            // TODO: we don't even need to decode the value, only check if it exists
            // Need to add exists_storage method to dancekit
            let relay_para_info = relay_state_proof
                .read_optional_entry::<ParaInfo<
                    cumulus_primitives_core::relay_chain::AccountId,
                    cumulus_primitives_core::relay_chain::Balance,
                >>(key.as_slice())
                .map_err(|_| Error::<T>::InvalidRelayStorageProof)?;
            if relay_para_info.is_some() {
                return Err(Error::<T>::ParaStillExistsInRelay.into());
            }

            // Take the deposit immediately and give it to origin account
            if let Some(asset_info) = RegistrarDeposit::<T>::take(para_id) {
                // Slash deposit from parachain creator
                // TODO: error handling
                let _ = T::Currency::transfer_on_hold(
                    &HoldReason::RegistrarDeposit.into(),
                    &asset_info.creator,
                    &account,
                    asset_info.deposit,
                    Precision::Exact,
                    Restriction::Free,
                    Fortitude::Force,
                );
            }

            Self::do_deregister(para_id)?;

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
        pub fn is_para_manager(para_id: &ParaId, account: &T::AccountId) -> bool {
            // This check will only pass if both are true:
            // * The para_id has a deposit in pallet_registrar
            // * The signed_account is the para manager (or creator if None)
            if let Some(manager) = ParaManager::<T>::get(para_id) {
                manager == *account
            } else {
                RegistrarDeposit::<T>::get(para_id)
                    .map(|deposit_info| deposit_info.creator)
                    .as_ref()
                    == Some(account)
            }
        }

        #[cfg(feature = "runtime-benchmarks")]
        pub fn benchmarks_get_or_create_para_manager(para_id: &ParaId) -> T::AccountId {
            use {
                frame_benchmarking::account,
                frame_support::{assert_ok, dispatch::RawOrigin},
            };

            let mut storage = BoundedVec::try_from(vec![]).unwrap();
            storage
                .try_push((b":code".to_vec(), vec![1; 10]).into())
                .unwrap();
            let genesis_data = ContainerChainGenesisData {
                storage,
                name: Default::default(),
                id: Default::default(),
                fork_id: Default::default(),
                extensions: Default::default(),
                properties: Default::default(),
            };

            // Return container chain manager, or register container chain as ALICE if it does not exist
            if !ParaGenesisData::<T>::contains_key(para_id) {
                // Register as a new user

                /// Create a funded user.
                /// Used for generating the necessary amount for registering
                fn create_funded_user<T: Config>(
                    string: &'static str,
                    n: u32,
                    total: DepositBalanceOf<T>,
                ) -> (T::AccountId, DepositBalanceOf<T>) {
                    const SEED: u32 = 0;
                    let user = account(string, n, SEED);
                    assert_ok!(T::Currency::mint_into(&user, total));
                    (user, total)
                }

                let deposit = Self::get_genesis_cost(genesis_data.encoded_size());
                let new_balance = T::Currency::minimum_balance()
                    .saturating_mul(10_000_000u32.into())
                    .saturating_add(deposit);
                let account = create_funded_user::<T>("caller", 1000, new_balance).0;
                T::InnerRegistrar::prepare_chain_registration(*para_id, account.clone());
                let origin = RawOrigin::Signed(account);

                assert_ok!(Self::register(
                    origin.into(),
                    *para_id,
                    genesis_data.clone(),
                    T::InnerRegistrar::bench_head_data(),
                ));
            }

            let deposit_info = RegistrarDeposit::<T>::get(para_id).expect("Cannot return signed origin for a container chain that was registered by root. Try using a different para id");

            let deposit = Self::get_genesis_cost(genesis_data.encoded_size());
            // Fund deposit creator, just in case it is not a new account
            let new_balance = (T::Currency::minimum_balance().saturating_add(deposit))
                .saturating_mul(2u32.into());
            assert_ok!(T::Currency::mint_into(&deposit_info.creator, new_balance));

            deposit_info.creator
        }

        pub fn get_genesis_cost(size: usize) -> <T::Currency as Inspect<T::AccountId>>::Balance {
            T::DataDepositPerByte::get().saturating_mul((size as u32).into())
        }

        fn do_register(
            account: T::AccountId,
            para_id: ParaId,
            genesis_data: ContainerChainGenesisData,
            head_data: Option<HeadData>,
        ) -> DispatchResult {
            // The actual registration takes place 2 sessions after the call to
            // `mark_valid_for_collating`, but the genesis data is inserted now.
            // This is because collators should be able to start syncing the new container chain
            // before the first block is mined. However, we could store the genesis data in a
            // different key, like PendingParaGenesisData.
            // TODO: for benchmarks, this call to .encoded_size is O(n) with respect to the number
            // of key-values in `genesis_data.storage`, even if those key-values are empty. And we
            // won't detect that the size is too big until after iterating over all of them, so the
            // limit in that case would be the transaction size.
            let genesis_data_size = genesis_data.encoded_size();

            let deposit = Self::get_genesis_cost(genesis_data_size);
            // Verify we can hold
            if !T::Currency::can_hold(&HoldReason::RegistrarDeposit.into(), &account, deposit) {
                return Err(Error::<T>::NotSufficientDeposit.into());
            }

            // Check if the para id is already registered by looking at the genesis data
            if ParaGenesisData::<T>::contains_key(para_id) {
                return Err(Error::<T>::ParaIdAlreadyRegistered.into());
            }

            // Check if the para id is already in PendingVerification (unreachable)
            let is_pending_verification = PendingVerification::<T>::take(para_id).is_some();
            if is_pending_verification {
                return Err(Error::<T>::ParaIdAlreadyRegistered.into());
            }

            // Insert para id into PendingVerification
            PendingVerification::<T>::insert(para_id, ());

            if genesis_data_size > T::MaxGenesisDataSize::get() as usize {
                return Err(Error::<T>::GenesisDataTooBig.into());
            }

            // Hold the deposit, we verified we can do this
            T::Currency::hold(&HoldReason::RegistrarDeposit.into(), &account, deposit)?;

            // Register the paraId also in the relay context (if any).
            T::InnerRegistrar::register(
                account.clone(),
                para_id,
                &genesis_data.storage,
                head_data,
            )?;

            // Update DepositInfo
            RegistrarDeposit::<T>::insert(
                para_id,
                DepositInfo {
                    creator: account.clone(),
                    deposit,
                },
            );
            ParaGenesisData::<T>::insert(para_id, genesis_data);

            ParaManager::<T>::insert(para_id, account);

            Ok(())
        }

        fn do_deregister(para_id: ParaId) -> DispatchResult {
            // Check if the para id is in "PendingVerification".
            // This is a special case because then we can remove it immediately, instead of waiting 2 sessions.
            let is_pending_verification = PendingVerification::<T>::take(para_id).is_some();
            if is_pending_verification {
                Self::deposit_event(Event::ParaIdDeregistered { para_id });
                // Cleanup immediately
                Self::cleanup_deregistered_para_id(para_id);
                BufferedParasToDeregister::<T>::try_mutate(|v| v.try_push(para_id)).map_err(
                    |_e| {
                        DispatchError::Other(
                            "Failed to add paraId to deregistration list: buffer is full",
                        )
                    },
                )?;
            } else {
                Self::schedule_paused_parachain_change(|para_ids, paused| {
                    // We have to find out where, in the sorted vec the para id is, if anywhere.

                    match para_ids.binary_search(&para_id) {
                        Ok(index) => {
                            para_ids.remove(index);
                        }
                        Err(_) => {
                            // If the para id is not registered, it may be paused. In that case, remove it from there
                            match paused.binary_search(&para_id) {
                                Ok(index) => {
                                    paused.remove(index);
                                }
                                Err(_) => {
                                    return Err(Error::<T>::ParaIdNotRegistered.into());
                                }
                            }
                        }
                    }

                    Ok(())
                })?;
                // Mark this para id for cleanup later
                Self::schedule_parachain_cleanup(para_id)?;

                // If we have InnerRegistrar set to a relay context (like Dancelight),
                // we first need to downgrade the paraId (if it was a parachain before)
                // and convert it to a parathread before deregistering it. Otherwise
                // the deregistration process will fail in the scheduled session.
                //
                // We only downgrade if the paraId is a parachain in the context of
                // this pallet.
                if ParathreadParams::<T>::get(para_id).is_none() {
                    T::InnerRegistrar::schedule_para_downgrade(para_id)?;
                }

                Self::deposit_event(Event::ParaIdDeregistered { para_id });
            }

            Ok(())
        }

        fn do_mark_valid_for_collating(para_id: ParaId) -> DispatchResult {
            let is_pending_verification = PendingVerification::<T>::take(para_id).is_some();
            if !is_pending_verification {
                return Err(Error::<T>::ParaIdNotInPendingVerification.into());
            }

            Self::schedule_parachain_change(|para_ids| {
                // We don't want to add duplicate para ids, so we check whether the potential new
                // para id is already present in the list. Because the list is always ordered, we can
                // leverage the binary search which makes this check O(log n).

                match para_ids.binary_search(&para_id) {
                    // This Ok is unreachable
                    Ok(_) => return Err(Error::<T>::ParaIdAlreadyRegistered.into()),
                    Err(index) => {
                        para_ids
                            .try_insert(index, para_id)
                            .map_err(|_e| Error::<T>::ParaIdListFull)?;
                    }
                }

                Ok(())
            })?;

            T::RegistrarHooks::check_valid_for_collating(para_id)?;

            Self::deposit_event(Event::ParaIdValidForCollating { para_id });

            T::RegistrarHooks::para_marked_valid_for_collating(para_id);

            // If we execute mark_valid_for_collating, we automatically upgrade
            // the paraId to a parachain (in the relay context) at the end of the execution.
            //
            // We only upgrade if the paraId is a parachain in the context of
            // this pallet.
            if ParathreadParams::<T>::get(para_id).is_none() {
                T::InnerRegistrar::schedule_para_upgrade(para_id)?;
            }

            Ok(())
        }

        /// Relay parachain manager signature message. Includes:
        /// * para_id, in case the manager has more than 1 para in the relay
        /// * accountid in tanssi, to ensure that the creator role is assigned to the desired account
        /// * relay_storage_root, to make the signature network-specific, and also make it expire
        ///     when the relay storage root expires.
        pub fn relay_signature_msg(
            para_id: ParaId,
            tanssi_account: &T::AccountId,
            relay_storage_root: H256,
        ) -> Vec<u8> {
            (para_id, tanssi_account, relay_storage_root).encode()
        }

        fn schedule_parachain_change(
            updater: impl FnOnce(&mut BoundedVec<ParaId, T::MaxLengthParaIds>) -> DispatchResult,
        ) -> DispatchResult {
            let mut pending_paras = PendingParaIds::<T>::get();
            // First, we need to decide what we should use as the base paras.
            let mut base_paras = pending_paras
                .last()
                .map(|(_, paras)| paras.clone())
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

        fn schedule_paused_parachain_change(
            updater: impl FnOnce(
                &mut BoundedVec<ParaId, T::MaxLengthParaIds>,
                &mut BoundedVec<ParaId, T::MaxLengthParaIds>,
            ) -> DispatchResult,
        ) -> DispatchResult {
            let mut pending_paras = PendingParaIds::<T>::get();
            let mut pending_paused = PendingPaused::<T>::get();
            // First, we need to decide what we should use as the base paras.
            let mut base_paras = pending_paras
                .last()
                .map(|(_, paras)| paras.clone())
                .unwrap_or_else(Self::registered_para_ids);
            let mut base_paused = pending_paused
                .last()
                .map(|(_, paras)| paras.clone())
                .unwrap_or_else(Self::paused);
            let old_base_paras = base_paras.clone();
            let old_base_paused = base_paused.clone();

            updater(&mut base_paras, &mut base_paused)?;

            if base_paras != old_base_paras {
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
            }

            if base_paused != old_base_paused {
                let new_paused = base_paused;
                let scheduled_session = Self::scheduled_session();

                if let Some(&mut (_, ref mut paras)) = pending_paused
                    .iter_mut()
                    .find(|&&mut (apply_at_session, _)| apply_at_session >= scheduled_session)
                {
                    *paras = new_paused;
                } else {
                    // We are scheduling a new parachains change for the scheduled session.
                    pending_paused.push((scheduled_session, new_paused));
                }

                <PendingPaused<T>>::put(pending_paused);
            }

            Ok(())
        }

        fn schedule_parathread_params_change(
            para_id: ParaId,
            updater: impl FnOnce(&mut ParathreadParamsTy) -> DispatchResult,
        ) -> DispatchResult {
            // Check that the para id is a parathread by reading the old params
            let params = match ParathreadParams::<T>::get(para_id) {
                Some(x) => x,
                None => {
                    return Err(Error::<T>::NotAParathread.into());
                }
            };

            let mut pending_params = PendingParathreadParams::<T>::get();
            // First, we need to decide what we should use as the base params.
            let mut base_params = pending_params
                .last()
                .and_then(|(_, para_id_params)| {
                    match para_id_params
                        .binary_search_by_key(&para_id, |(para_id, _params)| *para_id)
                    {
                        Ok(idx) => {
                            let (_para_id, params) = &para_id_params[idx];
                            Some(params.clone())
                        }
                        Err(_idx) => None,
                    }
                })
                .unwrap_or(params);

            updater(&mut base_params)?;
            let new_params = base_params;

            let scheduled_session = Self::scheduled_session();

            if let Some(&mut (_, ref mut para_id_params)) = pending_params
                .iter_mut()
                .find(|&&mut (apply_at_session, _)| apply_at_session >= scheduled_session)
            {
                match para_id_params.binary_search_by_key(&para_id, |(para_id, _params)| *para_id) {
                    Ok(idx) => {
                        let (_para_id, params) = &mut para_id_params[idx];
                        *params = new_params;
                    }
                    Err(idx) => {
                        para_id_params
                            .try_insert(idx, (para_id, new_params))
                            .map_err(|_e| Error::<T>::ParaIdListFull)?;
                    }
                }
            } else {
                // We are scheduling a new parathread params change for the scheduled session.
                pending_params.push((
                    scheduled_session,
                    BoundedVec::truncate_from(vec![(para_id, new_params)]),
                ));
            }

            <PendingParathreadParams<T>>::put(pending_params);

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

            let new_paras = if !pending_paras.is_empty() {
                let (mut past_and_present, future) = pending_paras
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
                    <PendingParaIds<T>>::put(future);
                }

                new_paras
            } else {
                // pending_paras.is_empty, so parachain list did not change
                None
            };

            let pending_paused = <PendingPaused<T>>::get();
            if !pending_paused.is_empty() {
                let (mut past_and_present, future) = pending_paused
                    .into_iter()
                    .partition::<Vec<_>, _>(|&(apply_at_session, _)| {
                        apply_at_session <= *session_index
                    });

                if past_and_present.len() > 1 {
                    // This should never happen since we schedule parachain changes only into the future
                    // sessions and this handler called for each session change.
                    log::error!(
                        target: LOG_TARGET,
                        "Skipping applying paused parachain changes scheduled sessions in the past",
                    );
                }

                let new_paused = past_and_present.pop().map(|(_, paras)| paras);
                if let Some(ref new_paused) = new_paused {
                    // Apply the new parachain list.
                    Paused::<T>::put(new_paused);
                    <PendingPaused<T>>::put(future);
                }
            }

            let pending_parathread_params = <PendingParathreadParams<T>>::get();
            if !pending_parathread_params.is_empty() {
                let (mut past_and_present, future) = pending_parathread_params
                    .into_iter()
                    .partition::<Vec<_>, _>(|&(apply_at_session, _)| {
                        apply_at_session <= *session_index
                    });

                if past_and_present.len() > 1 {
                    // This should never happen since we schedule parachain changes only into the future
                    // sessions and this handler called for each session change.
                    log::error!(
                        target: LOG_TARGET,
                        "Skipping applying parathread params changes scheduled sessions in the past",
                    );
                }

                let new_params = past_and_present.pop().map(|(_, params)| params);
                if let Some(ref new_params) = new_params {
                    for (para_id, params) in new_params {
                        <ParathreadParams<T>>::insert(para_id, params);
                    }
                    <PendingParathreadParams<T>>::put(future);
                }
            }

            let pending_to_remove = <PendingToRemove<T>>::get();
            if !pending_to_remove.is_empty() {
                let (past_and_present, future) =
                    pending_to_remove.into_iter().partition::<Vec<_>, _>(
                        |&(apply_at_session, _)| apply_at_session <= *session_index,
                    );

                if !past_and_present.is_empty() {
                    // Unlike `PendingParaIds`, this cannot skip items because we must cleanup all parachains.
                    // But this will only happen if `initializer_on_new_session` is not called for a big range of
                    // sessions, and many parachains are deregistered in the meantime.
                    let mut removed_para_ids = BTreeSet::new();
                    for (_, new_paras) in &past_and_present {
                        for para_id in new_paras {
                            Self::cleanup_deregistered_para_id(*para_id);
                            removed_para_ids.insert(*para_id);
                            if let Err(id) =
                                BufferedParasToDeregister::<T>::try_mutate(|v| v.try_push(*para_id))
                            {
                                log::error!(
                                    target: LOG_TARGET,
                                    "Failed to add paraId {:?} to deregistration list",
                                    id
                                );
                            }
                        }
                    }

                    // Also need to remove PendingParams to avoid setting params for a para id that does not exist
                    let mut pending_parathread_params = <PendingParathreadParams<T>>::get();
                    for (_, new_params) in &mut pending_parathread_params {
                        new_params.retain(|(para_id, _params)| {
                            // Retain para ids that are not in the list of removed para ids
                            !removed_para_ids.contains(para_id)
                        });
                    }
                    <PendingParathreadParams<T>>::put(pending_parathread_params);
                    <PendingToRemove<T>>::put(future);
                }
            }

            SessionChangeOutcome {
                prev_paras,
                new_paras,
            }
        }

        /// Remove all para id storage in this pallet,
        /// and execute para_deregistered hook to clean up other pallets as well
        fn cleanup_deregistered_para_id(para_id: ParaId) {
            ParaGenesisData::<T>::remove(para_id);
            ParathreadParams::<T>::remove(para_id);
            // Get asset creator and deposit amount
            // Deposit may not exist, for example if the para id was registered on genesis
            if let Some(asset_info) = RegistrarDeposit::<T>::take(para_id) {
                // Release hold
                let _ = T::Currency::release(
                    &HoldReason::RegistrarDeposit.into(),
                    &asset_info.creator,
                    asset_info.deposit,
                    Precision::Exact,
                );
            }

            ParaManager::<T>::remove(para_id);

            T::RegistrarHooks::para_deregistered(para_id);
        }

        fn schedule_parachain_cleanup(para_id: ParaId) -> DispatchResult {
            let scheduled_session = Self::scheduled_session();
            let mut pending_paras = PendingToRemove::<T>::get();
            // First, we need to decide what we should use as the base paras.
            let base_paras = match pending_paras
                .binary_search_by_key(&scheduled_session, |(session, _paras)| *session)
            {
                Ok(i) => &mut pending_paras[i].1,
                Err(i) => {
                    pending_paras.insert(i, (scheduled_session, Default::default()));

                    &mut pending_paras[i].1
                }
            };

            // Add the para_id to the entry for the scheduled session.
            match base_paras.binary_search(&para_id) {
                // This Ok is unreachable
                Ok(_) => return Err(Error::<T>::ParaIdAlreadyDeregistered.into()),
                Err(index) => {
                    base_paras
                        .try_insert(index, para_id)
                        .map_err(|_e| Error::<T>::ParaIdListFull)?;
                }
            }

            // Save the updated list of pending parachains for removal.
            <PendingToRemove<T>>::put(pending_paras);

            Ok(())
        }

        pub fn registered_para_ids() -> BoundedVec<ParaId, T::MaxLengthParaIds> {
            RegisteredParaIds::<T>::get()
        }

        pub fn pending_registered_para_ids(
        ) -> Vec<(T::SessionIndex, BoundedVec<ParaId, T::MaxLengthParaIds>)> {
            PendingParaIds::<T>::get()
        }

        pub fn para_genesis_data(para_id: ParaId) -> Option<ContainerChainGenesisData> {
            ParaGenesisData::<T>::get(para_id)
        }

        pub fn pending_verification(para_id: ParaId) -> Option<()> {
            PendingVerification::<T>::get(para_id)
        }

        pub fn paused() -> BoundedVec<ParaId, T::MaxLengthParaIds> {
            Paused::<T>::get()
        }

        pub fn pending_paused() -> Vec<(T::SessionIndex, BoundedVec<ParaId, T::MaxLengthParaIds>)> {
            PendingPaused::<T>::get()
        }

        pub fn pending_to_remove() -> Vec<(T::SessionIndex, BoundedVec<ParaId, T::MaxLengthParaIds>)>
        {
            PendingToRemove::<T>::get()
        }

        pub fn parathread_params(para_id: ParaId) -> Option<ParathreadParamsTy> {
            ParathreadParams::<T>::get(para_id)
        }

        pub fn pending_parathread_params() -> Vec<(
            T::SessionIndex,
            BoundedVec<(ParaId, ParathreadParamsTy), T::MaxLengthParaIds>,
        )> {
            PendingParathreadParams::<T>::get()
        }

        pub fn registrar_deposit(para_id: ParaId) -> Option<DepositInfo<T>> {
            RegistrarDeposit::<T>::get(para_id)
        }
    }

    impl<T: Config> GetCurrentContainerChains for Pallet<T> {
        type MaxContainerChains = T::MaxLengthParaIds;

        fn current_container_chains() -> BoundedVec<ParaId, Self::MaxContainerChains> {
            Self::registered_para_ids()
        }

        #[cfg(feature = "runtime-benchmarks")]
        fn set_current_container_chains(container_chains: &[ParaId]) {
            let paras: BoundedVec<ParaId, T::MaxLengthParaIds> =
                container_chains.to_vec().try_into().unwrap();
            RegisteredParaIds::<T>::put(paras);
        }
    }

    impl<T: Config> GetSessionContainerChains<T::SessionIndex> for Pallet<T> {
        fn session_container_chains(session_index: T::SessionIndex) -> SessionContainerChains {
            let (past_and_present, _) = Pallet::<T>::pending_registered_para_ids()
                .into_iter()
                .partition::<Vec<_>, _>(|&(apply_at_session, _)| apply_at_session <= session_index);

            let paras = if let Some(last) = past_and_present.last() {
                last.1.clone()
            } else {
                Pallet::<T>::registered_para_ids()
            };

            let mut parachains = vec![];
            let mut parathreads = vec![];

            for para_id in paras {
                // TODO: sweet O(n) db reads
                if let Some(parathread_params) = ParathreadParams::<T>::get(para_id) {
                    parathreads.push((para_id, parathread_params));
                } else {
                    parachains.push(para_id);
                }
            }

            SessionContainerChains {
                parachains,
                parathreads,
            }
        }

        #[cfg(feature = "runtime-benchmarks")]
        fn set_session_container_chains(
            _session_index: T::SessionIndex,
            container_chains: &[ParaId],
        ) {
            // TODO: this assumes session_index == current
            let paras: BoundedVec<ParaId, T::MaxLengthParaIds> =
                container_chains.to_vec().try_into().unwrap();
            RegisteredParaIds::<T>::put(paras);
        }
    }
}

pub trait RegistrarHooks {
    fn para_marked_valid_for_collating(_para_id: ParaId) -> Weight {
        Weight::default()
    }
    fn para_deregistered(_para_id: ParaId) -> Weight {
        Weight::default()
    }
    fn check_valid_for_collating(_para_id: ParaId) -> DispatchResult {
        Ok(())
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn benchmarks_ensure_valid_for_collating(_para_id: ParaId) {}
}

impl RegistrarHooks for () {}

pub struct EnsureSignedByManager<T>(sp_std::marker::PhantomData<T>);

impl<T> EnsureOriginWithArg<T::RuntimeOrigin, ParaId> for EnsureSignedByManager<T>
where
    T: Config,
{
    type Success = T::AccountId;

    fn try_origin(
        o: T::RuntimeOrigin,
        para_id: &ParaId,
    ) -> Result<Self::Success, T::RuntimeOrigin> {
        let signed_account =
            <frame_system::EnsureSigned<_> as EnsureOrigin<_>>::try_origin(o.clone())?;

        if !Pallet::<T>::is_para_manager(para_id, &signed_account) {
            return Err(frame_system::RawOrigin::Signed(signed_account).into());
        }

        Ok(signed_account)
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn try_successful_origin(para_id: &ParaId) -> Result<T::RuntimeOrigin, ()> {
        let manager = Pallet::<T>::benchmarks_get_or_create_para_manager(para_id);

        Ok(frame_system::RawOrigin::Signed(manager).into())
    }
}
