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

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_support::LOG_TARGET;
    use frame_system::pallet_prelude::*;
    // TODO: move this trait to a common primitives folder
    use pallet_configuration::GetSessionIndex;
    use sp_runtime::traits::AtLeast32BitUnsigned;
    use sp_runtime::Saturating;
    use sp_std::prelude::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::genesis_config]
    #[derive(Default)]
    pub struct GenesisConfig {
        /// Para ids
        pub para_ids: Vec<u32>,
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig {
        fn build(&self) {
            let mut para_ids = self.para_ids.clone();
            para_ids.sort();
            para_ids.dedup_by(|a, b| {
                if a == b {
                    panic!("Duplicate para_id: {}", a);
                } else {
                    false
                }
            });

            <RegisteredParaIds<T>>::put(
                BoundedVec::<_, _>::try_from(para_ids)
                    .expect("too many para ids in genesis: bounded vec full"),
            );
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

        type SessionIndex: parity_scale_codec::FullCodec + TypeInfo + Copy + AtLeast32BitUnsigned;

        type SessionDelay: Get<Self::SessionIndex>;

        type CurrentSessionIndex: pallet_configuration::GetSessionIndex<Self::SessionIndex>;
    }

    #[pallet::storage]
    #[pallet::getter(fn registered_para_ids)]
    pub type RegisteredParaIds<T: Config> =
        StorageValue<_, BoundedVec<u32, T::MaxLengthParaIds>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn pending_registered_para_ids)]
    pub type PendingParaIds<T: Config> =
        StorageValue<_, Vec<(T::SessionIndex, BoundedVec<u32, T::MaxLengthParaIds>)>, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new para id has been registered. [para_id]
        ParaIdRegistered { para_id: u32 },
        /// A para id has been deregistered. [para_id]
        ParaIdDeregistered { para_id: u32 },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Attempted to register a ParaId that was already registered
        ParaIdAlreadyRegistered,
        /// Attempted to deregister a ParaId that is not registered
        ParaIdNotRegistered,
        /// The bounded list of ParaIds has reached its limit
        ParaIdListFull,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register parachain
        #[pallet::call_index(0)]
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
        pub fn register(origin: OriginFor<T>, para_id: u32) -> DispatchResult {
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

            Self::deposit_event(Event::ParaIdRegistered { para_id });
            Ok(())
        }

        /// Deregister parachain
        #[pallet::call_index(1)]
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
        pub fn deregister(origin: OriginFor<T>, para_id: u32) -> DispatchResult {
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

            Ok(())
        }
    }

    pub struct SessionChangeOutcome<T: Config> {
        /// Previously active parachains.
        pub prev_paras: BoundedVec<u32, T::MaxLengthParaIds>,
        /// If new parachains have been applied in the new session, this is the new  list.
        pub new_paras: Option<BoundedVec<u32, T::MaxLengthParaIds>>,
    }

    impl<T: Config> Pallet<T> {
        #[inline(never)]
        fn schedule_parachain_change(
            updater: impl FnOnce(&mut BoundedVec<u32, T::MaxLengthParaIds>) -> DispatchResult,
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
}
