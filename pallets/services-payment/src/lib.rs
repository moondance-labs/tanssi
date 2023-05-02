//! # Services Payment pallet
//!
//! This pallet allows for block creation services to be paid for by a
//! containerChain.

#![cfg_attr(not(feature = "std"), no_std)]

use {
    cumulus_primitives_core::ParaId,
    frame_support::{pallet_prelude::*, sp_runtime::Saturating, traits::Currency},
    frame_system::pallet_prelude::*,
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
        type OnChargeForBlockCredit: OnChargeForBlockCredit<Self>;
        /// Currency type for fee payment
        type Currency: Currency<Self::AccountId>;
        /// Provider of a block cost which can adjust from block to block
        type ProvideBlockProductionCost: ProvideBlockProductionCost<Self>;
        /// The maximum number of credits that can be accumulated
        type MaxCreditsStored: Get<u64>;
    }

    #[pallet::error]
    pub enum Error<T> {
        TooManyCredits,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        CreditsPurchased {
            para_id: ParaId,
            payer: T::AccountId,
            fee: BalanceOf<T>, 
            credits_purchased: u64,
            credits_owned: u64,
        }
    }

    #[pallet::storage]
    #[pallet::getter(fn collator_commission)]
    type BlockProductionCredits<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ParaId,
        u64,
        OptionQuery
    >;

    #[pallet::call]
    impl<T: Config> Pallet<T> 
    where
        BalanceOf<T>: From<u64>,
    {
        #[pallet::call_index(0)]
        #[pallet::weight(0)] // TODO
        pub fn purchase_credits(
            origin: OriginFor<T>,
            para_id: ParaId,
            credits: u64
        ) -> DispatchResultWithPostInfo {
            let account = ensure_signed(origin)?;

            let existing_credits = BlockProductionCredits::<T>::get(para_id).unwrap_or(0u64);
            let updated_credits = existing_credits.saturating_add(credits);
            ensure!(
                updated_credits <= T::MaxCreditsStored::get(),
                Error::<T>::TooManyCredits,
            );

            // get the current per-credit cost of a block
            let (block_cost, weight) = T::ProvideBlockProductionCost::block_cost(&para_id);
            let total_fee = block_cost.saturating_mul(credits.into());
            
            T::OnChargeForBlockCredit::charge_credits(&para_id, credits, total_fee)?;

            BlockProductionCredits::<T>::insert(para_id, updated_credits);

            Self::deposit_event(Event::<T>::CreditsPurchased {
                para_id,
                payer: account,
                fee: total_fee,
                credits_purchased: credits,
                credits_owned: updated_credits,
            });

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        // TODO:
        pub fn burn_credit_for_para(para_id: &ParaId) { }
    }
}

/// Balance used by this pallet
pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

/// Handler for fee charging. This will be invoked when fees need to be deducted from the fee
/// account for a given paraId.
pub trait OnChargeForBlockCredit<T: Config> {
    fn charge_credits(
        para_id: &ParaId,
        credits: u64,
        fee: BalanceOf<T>,
    ) -> Result<(), Error<T>>;
}

/// Returns the cost for a given block credit at the current time. This can be a complex operation,
/// so it also returns the weight it consumes. (TODO: or just rely on benchmarking)
pub trait ProvideBlockProductionCost<T: Config> {
    fn block_cost(para_id: &ParaId) -> (BalanceOf<T>, Weight);
}