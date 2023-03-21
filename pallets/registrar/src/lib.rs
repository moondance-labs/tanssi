#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
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
    }

    #[pallet::storage]
    #[pallet::getter(fn registered_para_ids)]
    pub type RegisteredParaIds<T: Config> =
        StorageValue<_, BoundedVec<u32, T::MaxLengthParaIds>, ValueQuery>;

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

            let mut para_ids = <RegisteredParaIds<T>>::get();

            // We don't want to add duplicate para ids, so we check whether the potential new
            // para id is already present in the list. Because the list is always ordered, we can
            // leverage the binary search which makes this check O(log n).
            match para_ids.binary_search(&para_id) {
                Ok(_) => Err(Error::<T>::ParaIdAlreadyRegistered.into()),
                Err(index) => {
                    para_ids
                        .try_insert(index, para_id)
                        .map_err(|_e| Error::<T>::ParaIdListFull)?;

                    // Store updated list
                    <RegisteredParaIds<T>>::put(para_ids);

                    // Emit an event.
                    Self::deposit_event(Event::ParaIdRegistered { para_id });

                    Ok(())
                }
            }
        }

        /// Deregister parachain
        #[pallet::call_index(1)]
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
        pub fn deregister(origin: OriginFor<T>, para_id: u32) -> DispatchResult {
            T::RegistrarOrigin::ensure_origin(origin)?;

            let mut para_ids = <RegisteredParaIds<T>>::get();

            // We have to find out where, in the sorted vec the para id is, if anywhere.
            match para_ids.binary_search(&para_id) {
                Ok(index) => {
                    para_ids.remove(index);

                    // Store updated list
                    <RegisteredParaIds<T>>::put(para_ids);

                    // Emit an event.
                    Self::deposit_event(Event::ParaIdDeregistered { para_id });

                    Ok(())
                }
                Err(_) => Err(Error::<T>::ParaIdNotRegistered.into()),
            }
        }
    }
}
