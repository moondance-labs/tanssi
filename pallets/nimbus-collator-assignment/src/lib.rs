//! # Nimbus Collator Assignment Pallet

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use {
    frame_support::pallet_prelude::*,
    sp_runtime::{
        traits::{AtLeast32BitUnsigned, One, Zero},
        Saturating,
    },
    sp_std::{prelude::*, vec, collections::btree_map::BTreeMap},
    tp_collator_assignment::AssignedCollators,
    tp_traits::{
        GetHostConfiguration, GetSessionContainerChains,
    },
};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type SessionIndex: parity_scale_codec::FullCodec + TypeInfo + Copy + AtLeast32BitUnsigned;
        // `SESSION_DELAY` is used to delay any changes to Paras registration or configurations.
        // Wait until the session index is 2 larger then the current index to apply any changes,
        // which guarantees that at least one full session has passed before any changes are applied.
        type HostConfiguration: GetHostConfiguration<Self::SessionIndex>;
        type ContainerChains: GetSessionContainerChains<Self::SessionIndex>;
        type NimbusId: parity_scale_codec::FullCodec + TypeInfo + Clone;
    }

    #[pallet::storage]
    #[pallet::getter(fn collator_container_chain)]
    pub(crate) type CollatorContainerChain<T: Config> =
        StorageMap<_, Blake2_128Concat, T::SessionIndex, AssignedCollators<T::NimbusId>, OptionQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    /// A struct that holds the assignment that is active after the session change and optionally
    /// the assignment that becomes active after the next session change.
    pub struct SessionChangeOutcome<T: Config> {
        /// New active assignment.
        pub active_assignment: AssignedCollators<T::NimbusId>,
        /// Next session active assignment.
        pub next_assignment: AssignedCollators<T::NimbusId>,
    }

    impl<T: Config> Pallet<T> {
        /// Assign new collators
        /// collators should be queued collators
        pub fn assign_collators(
            current_session_index: &T::SessionIndex,
            id_map: &BTreeMap<T::AccountId, T::NimbusId>,
            current_collator_assignment: &AssignedCollators<T::AccountId>,
            next_collator_assignment: &AssignedCollators<T::AccountId>,
        ) -> SessionChangeOutcome<T> {
            let current_nimbus_assignment = current_collator_assignment.map(|account_id| id_map[account_id].clone());
            let next_nimbus_assignment = next_collator_assignment.map(|account_id| id_map[account_id].clone());

            // Only applies to session index 0
            if current_session_index == &T::SessionIndex::zero() {
                CollatorContainerChain::<T>::insert(current_session_index, next_nimbus_assignment.clone());
                CollatorContainerChain::<T>::insert(current_session_index.saturating_add(T::SessionIndex::one()), next_nimbus_assignment.clone());

                return SessionChangeOutcome {
                    active_assignment: next_nimbus_assignment.clone(),
                    next_assignment: next_nimbus_assignment,
                };
            }

            // TODO: if we skip sessions we will miss some removals
            CollatorContainerChain::<T>::remove(current_session_index.saturating_sub(T::SessionIndex::one()));
            CollatorContainerChain::<T>::insert(current_session_index, current_nimbus_assignment.clone());
            CollatorContainerChain::<T>::insert(current_session_index.saturating_add(T::SessionIndex::one()), next_nimbus_assignment.clone());

            SessionChangeOutcome {
                active_assignment: current_nimbus_assignment,
                next_assignment: next_nimbus_assignment,
            }
        }

        pub fn initializer_on_new_session(
            current_session_index: &T::SessionIndex,
            id_map: &BTreeMap<T::AccountId, T::NimbusId>,
            current_collator_assignment: &AssignedCollators<T::AccountId>,
            next_collator_assignment: &AssignedCollators<T::AccountId>,
        ) -> SessionChangeOutcome<T> {
            Self::assign_collators(current_session_index, id_map, current_collator_assignment, next_collator_assignment)
        }
    }
}
