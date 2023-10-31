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

#[cfg(any(test, feature = "runtime-benchmarks"))]
mod benchmarks;
pub mod weights;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

    use {
        crate::weights::WeightInfo,
        frame_support::{
            pallet_prelude::*,
            traits::{Currency, EitherOfDiverse, ReservableCurrency},
            DefaultNoBound, LOG_TARGET,
        },
        frame_system::{pallet_prelude::*, EnsureSigned},
        sp_runtime::{
            traits::{AtLeast32BitUnsigned, BadOrigin},
            Either, Saturating,
        },
        sp_std::prelude::*,
        tp_container_chain_genesis_data::ContainerChainGenesisData,
        tp_traits::{
            GetCurrentContainerChains, GetSessionContainerChains, GetSessionIndex, ParaId,
        },
    };

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::genesis_config]
    #[derive(DefaultNoBound)]
    pub struct GenesisConfig<T: Config> {
        /// Para ids
        pub para_ids: Vec<(
            ParaId,
            ContainerChainGenesisData<T::MaxLengthTokenSymbol>,
            Vec<Vec<u8>>,
        )>,
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

            for (para_id, genesis_data, boot_nodes) in para_ids {
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
                let boot_nodes: Vec<_> = boot_nodes
                    .iter()
                    .map(|x| BoundedVec::try_from(x.clone()).expect("boot node url too long"))
                    .collect();
                let boot_nodes = BoundedVec::try_from(boot_nodes).expect("too many boot nodes");
                <BootNodes<T>>::insert(para_id, boot_nodes);
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

        type MaxBootNodes: Get<u32>;
        type MaxBootNodeUrlLen: Get<u32>;
        type MaxLengthTokenSymbol: Get<u32>;

        type SessionIndex: parity_scale_codec::FullCodec + TypeInfo + Copy + AtLeast32BitUnsigned;

        #[pallet::constant]
        type SessionDelay: Get<Self::SessionIndex>;

        type CurrentSessionIndex: GetSessionIndex<Self::SessionIndex>;

        type Currency: ReservableCurrency<Self::AccountId>;

        #[pallet::constant]
        type DepositAmount: Get<<Self::Currency as Currency<Self::AccountId>>::Balance>;

        type WeightInfo: WeightInfo;
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
    pub type ParaGenesisData<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ParaId,
        ContainerChainGenesisData<T::MaxLengthTokenSymbol>,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn pending_verification)]
    pub type PendingVerification<T: Config> =
        StorageValue<_, BoundedVec<ParaId, T::MaxLengthParaIds>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn boot_nodes)]
    pub type BootNodes<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ParaId,
        BoundedVec<BoundedVec<u8, T::MaxBootNodeUrlLen>, T::MaxBootNodes>,
        ValueQuery,
    >;

    pub type DepositBalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[derive(Default, Clone, Encode, Decode, RuntimeDebug, PartialEq, scale_info::TypeInfo)]
    #[scale_info(skip_type_params(T))]
    pub struct DepositInfo<T: Config> {
        pub creator: T::AccountId,
        pub deposit: DepositBalanceOf<T>,
    }

    /// Registrar deposits, a mapping from paraId to a struct
    /// holding the creator (from which the deposit was reserved) and
    /// the deposit amount
    #[pallet::storage]
    #[pallet::getter(fn registrar_deposit)]
    pub type RegistrarDeposit<T: Config> = StorageMap<_, Blake2_128Concat, ParaId, DepositInfo<T>>;

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
        /// The list of boot_nodes
        BootNodesChanged { para_id: ParaId },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Attempted to register a ParaId that was already registered
        ParaIdAlreadyRegistered,
        /// Attempted to pause a ParaId that was already in PendingVerification
        ParaIdAlreadyPaused,
        /// Attempted to deregister a ParaId that is not registered
        ParaIdNotRegistered,
        /// The bounded list of ParaIds has reached its limit
        ParaIdListFull,
        /// Attempted to register a ParaId with a genesis data size greater than the limit
        GenesisDataTooBig,
        /// Tried to mark_valid_for_collating a ParaId that is not in PendingVerification
        ParaIdNotInPendingVerification,
        /// Tried to register a ParaId with an account that did not have enough balance for the deposit
        NotSufficientDeposit,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register container-chain
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::register(genesis_data.encoded_size() as u32, T::MaxLengthParaIds::get(), genesis_data.storage.len() as u32))]
        pub fn register(
            origin: OriginFor<T>,
            para_id: ParaId,
            genesis_data: ContainerChainGenesisData<T::MaxLengthTokenSymbol>,
        ) -> DispatchResult {
            let account = ensure_signed(origin)?;
            let deposit = T::DepositAmount::get();

            // Verify we can reserve
            T::Currency::can_reserve(&account, deposit)
                .then_some(true)
                .ok_or(Error::<T>::NotSufficientDeposit)?;

            // Check if the para id is already registered
            let pending_paras = <PendingParaIds<T>>::get();
            let base_paras = pending_paras
                .last()
                .map(|(_, paras)| paras.clone())
                .unwrap_or_else(Self::registered_para_ids);
            if base_paras.binary_search(&para_id).is_ok() {
                return Err(Error::<T>::ParaIdAlreadyRegistered.into());
            }

            // Insert para id into PendingVerification, if it does not exist there
            let mut pending_verification = PendingVerification::<T>::get();
            match pending_verification.binary_search(&para_id) {
                Ok(_) => return Err(Error::<T>::ParaIdAlreadyRegistered.into()),
                Err(index) => {
                    pending_verification
                        .try_insert(index, para_id)
                        .map_err(|_e| Error::<T>::ParaIdListFull)?;
                }
            }

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
            if genesis_data_size > T::MaxGenesisDataSize::get() as usize {
                return Err(Error::<T>::GenesisDataTooBig.into());
            }

            // Reserve the deposit, we verified we can do this
            T::Currency::reserve(&account, deposit)?;

            // Update DepositInfo
            RegistrarDeposit::<T>::insert(
                para_id,
                DepositInfo {
                    creator: account,
                    deposit,
                },
            );
            ParaGenesisData::<T>::insert(para_id, genesis_data);
            PendingVerification::<T>::put(pending_verification);

            Self::deposit_event(Event::ParaIdRegistered { para_id });

            Ok(())
        }

        /// Deregister container-chain.
        ///
        /// If a container-chain is registered but not marked as valid_for_collating, this will remove it
        /// from `PendingVerification` as well.
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::deregister(
            T::MaxGenesisDataSize::get(),
            T::MaxLengthParaIds::get()
        ))]
        pub fn deregister(origin: OriginFor<T>, para_id: ParaId) -> DispatchResult {
            T::RegistrarOrigin::ensure_origin(origin)?;

            Self::schedule_parachain_change(|para_ids| {
                // We have to find out where, in the sorted vec the para id is, if anywhere.

                match para_ids.binary_search(&para_id) {
                    Ok(index) => {
                        para_ids.remove(index);
                        Ok(())
                    }
                    Err(_) => {
                        // If the para id is not registered yet, it may be in "PendingVerification"
                        // In that case, remove it from there
                        let mut para_ids = PendingVerification::<T>::get();

                        match para_ids.binary_search(&para_id) {
                            Ok(index) => {
                                para_ids.remove(index);
                                PendingVerification::<T>::put(para_ids);
                                Ok(())
                            }
                            Err(_) => Err(Error::<T>::ParaIdNotRegistered.into()),
                        }
                    }
                }
            })?;

            // Get asset creator and deposit amount
            // Deposit may not exist, for example if the para id was registered on genesis
            if let Some(asset_info) = RegistrarDeposit::<T>::get(para_id) {
                // Unreserve deposit
                T::Currency::unreserve(&asset_info.creator, asset_info.deposit);

                // Remove asset info
                RegistrarDeposit::<T>::remove(para_id);
            }

            Self::deposit_event(Event::ParaIdDeregistered { para_id });

            // TODO: while the deregistration takes place on the next session, the genesis data
            // is deleted immediately. This will cause problems since any new collators that want
            // to join now will not be able to sync this parachain
            ParaGenesisData::<T>::remove(para_id);
            BootNodes::<T>::remove(para_id);

            Ok(())
        }

        /// Mark container-chain valid for collating
        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::mark_valid_for_collating(T::MaxLengthParaIds::get()))]
        pub fn mark_valid_for_collating(origin: OriginFor<T>, para_id: ParaId) -> DispatchResult {
            T::RegistrarOrigin::ensure_origin(origin)?;

            let mut pending_verification = PendingVerification::<T>::get();

            match pending_verification.binary_search(&para_id) {
                Ok(i) => {
                    pending_verification.remove(i);
                }
                Err(_) => return Err(Error::<T>::ParaIdNotInPendingVerification.into()),
            };

            Self::schedule_parachain_change(|para_ids| {
                // We don't want to add duplicate para ids, so we check whether the potential new
                // para id is already present in the list. Because the list is always ordered, we can
                // leverage the binary search which makes this check O(log n).

                match para_ids.binary_search(&para_id) {
                    Ok(_) => Err(Error::<T>::ParaIdAlreadyRegistered.into()),
                    Err(index) => {
                        para_ids
                            .try_insert(index, para_id)
                            .map_err(|_e| Error::<T>::ParaIdListFull)?;

                        Ok(())
                    }
                }
            })?;

            PendingVerification::<T>::put(pending_verification);

            Self::deposit_event(Event::ParaIdValidForCollating { para_id });

            Ok(())
        }

        /// Set boot_nodes for this para id
        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::set_boot_nodes(
            T::MaxBootNodeUrlLen::get(),
            boot_nodes.len() as u32,
        ))]
        pub fn set_boot_nodes(
            origin: OriginFor<T>,
            para_id: ParaId,
            boot_nodes: BoundedVec<BoundedVec<u8, T::MaxBootNodeUrlLen>, T::MaxBootNodes>,
        ) -> DispatchResult {
            let origin =
                EitherOfDiverse::<T::RegistrarOrigin, EnsureSigned<T::AccountId>>::ensure_origin(
                    origin,
                )?;

            if let Either::Right(signed_account) = origin {
                let deposit_info = RegistrarDeposit::<T>::get(para_id).ok_or(BadOrigin)?;
                if deposit_info.creator != signed_account {
                    Err(BadOrigin)?;
                }
            }

            BootNodes::<T>::insert(para_id, boot_nodes);

            Self::deposit_event(Event::BootNodesChanged { para_id });

            Ok(())
        }

        /// Pause container-chain from collating without removing its boot nodes nor its genesis config
        #[pallet::call_index(4)]
        #[pallet::weight(T::WeightInfo::pause_container_chain(T::MaxLengthParaIds::get()))]
        pub fn pause_container_chain(origin: OriginFor<T>, para_id: ParaId) -> DispatchResult {
            T::RegistrarOrigin::ensure_origin(origin)?;

            let mut pending_verification = PendingVerification::<T>::get();
            match pending_verification.binary_search(&para_id) {
                Ok(_) => return Err(Error::<T>::ParaIdAlreadyPaused.into()),
                Err(index) => {
                    pending_verification
                        .try_insert(index, para_id)
                        .map_err(|_e| Error::<T>::ParaIdListFull)?;
                }
            };

            Self::schedule_parachain_change(|para_ids| match para_ids.binary_search(&para_id) {
                Ok(index) => {
                    para_ids.remove(index);
                    Ok(())
                }
                Err(_) => return Err(Error::<T>::ParaIdNotRegistered.into()),
            })?;

            PendingVerification::<T>::put(pending_verification);

            Self::deposit_event(Event::ParaIdPaused { para_id });

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
        fn session_container_chains(session_index: T::SessionIndex) -> Vec<ParaId> {
            let (past_and_present, _) = Pallet::<T>::pending_registered_para_ids()
                .into_iter()
                .partition::<Vec<_>, _>(|&(apply_at_session, _)| apply_at_session <= session_index);

            let paras = if let Some(last) = past_and_present.last() {
                last.1.clone()
            } else {
                Pallet::<T>::registered_para_ids()
            };

            paras.into_iter().collect()
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
