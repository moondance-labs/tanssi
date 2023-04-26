//! # Services Payment pallet
//!
//! This pallet allows for block creation services to be paid for by a
//! containerChain.

#![cfg_attr(not(feature = "std"), no_std)]

use {
    cumulus_primitives_core::ParaId,
    frame_support::{pallet_prelude::*, traits::Currency},
};

/*
#[cfg(test)]
mod mock;

#[cfg(test)]
mod test;
*/

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Handler for fees
        type OnChargeForServices: OnChargeForServices<Self>;
        /// Produce an AccountId unique to a given ParaId (such as through hashing)
        type ParaIdToAccountId: ParaIdToAccountId<Self>;
        /// Currency type for fee payment
        type Currency: Currency<Self::AccountId>;
    }

    #[pallet::error]
    pub enum Error<T> {
        NoPaymentMade,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        FeePaid {
            para_id: ParaId,
            para_account_id: T::AccountId,
            fee: BalanceOf<T>, 
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
    }

    impl<T: Config> Pallet<T> {
        pub fn charge_fee_for_para(para_id: ParaId, fee: BalanceOf<T>) -> Result<(), Error<T>> {
            let para_account_id = T::ParaIdToAccountId::derive_account_id(para_id);
            T::OnChargeForServices::withdraw_fee(&para_account_id, &para_id, fee)?;

            Self::deposit_event(Event::<T>::FeePaid {
                para_id,
                para_account_id,
                fee,
            });

            Ok(())
        }
    }
}

/// Balance used by this pallet
pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

/// Handler for fee charging. This will be invoked when fees need to be deducted from the fee
/// account for a given paraId.
pub trait OnChargeForServices<T: Config> {
    fn withdraw_fee(
        para_account: &T::AccountId,
        para_id: &ParaId,
        fee: BalanceOf<T>,
    ) -> Result<(), Error<T>>;
}

/// Produce an AccountId from the given ParaId. This should have the following properties:
/// * deterministic (always result in the same output for the same input)
/// * collision-resistant (no two ParaIds should produce the same AccountId with any significant likelihood)
/// * cryptographically secure (it should not be feasible to guess a matching private key)
pub trait ParaIdToAccountId<T: Config> {
    fn derive_account_id(para_id: ParaId) -> T::AccountId;
}
