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

//! # Author Noting Pallet
//!
//! This pallet notes the author of the different containerChains that have registered:
//!
//! The set of container chains is retrieved thanks to the GetContainerChains trait
//! For each containerChain, we inspect the Header stored in the relayChain as
//! a generic header. This is the first requirement for containerChains.
//!
//! The second requirement is that an Aura digest with the slot number for the containerChains
//! needs to exist
//!  
//! Using those two requirements we can select who the author was based on the collators assigned
//! to that containerChain, by simply assigning the slot position.

//! # Services Payment pallet
//!
//! This pallet allows for block creation services to be paid for by a
//! containerChain.

#![cfg_attr(not(feature = "std"), no_std)]

use {
    cumulus_primitives_core::ParaId,
    frame_support::{
        pallet_prelude::*,
        sp_runtime::{traits::Zero, Saturating},
        traits::Currency,
    },
    frame_system::pallet_prelude::*,
};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;

#[frame_support::pallet(dev_mode)]
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
        type MaxCreditsStored: Get<BlockNumberFor<Self>>;
    }

    #[pallet::error]
    pub enum Error<T> {
        InsufficientFundsToPurchaseCredits,
        InsufficientCredits,
        CreditPriceTooExpensive,
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
            credits_purchased: BlockNumberFor<T>,
            credits_remaining: BlockNumberFor<T>,
        },
        CreditBurned {
            para_id: ParaId,
            credits_remaining: BlockNumberFor<T>,
        },
    }

    #[pallet::storage]
    #[pallet::getter(fn collator_commission)]
    pub type BlockProductionCredits<T: Config> =
        StorageMap<_, Blake2_128Concat, ParaId, BlockNumberFor<T>, OptionQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T>
    where
        BalanceOf<T>: From<BlockNumberFor<T>>,
    {
        #[pallet::call_index(0)]
        #[pallet::weight(0)] // TODO
        pub fn purchase_credits(
            origin: OriginFor<T>,
            para_id: ParaId,
            credits: BlockNumberFor<T>,
            max_price_per_credit: Option<BalanceOf<T>>,
        ) -> DispatchResultWithPostInfo {
            let account = ensure_signed(origin)?;

            let existing_credits =
                BlockProductionCredits::<T>::get(para_id).unwrap_or(BlockNumberFor::<T>::zero());
            let credits_purchasable = T::MaxCreditsStored::get().saturating_sub(existing_credits);
            let actual_credits_purchased = credits.min(credits_purchasable);

            let updated_credits = existing_credits.saturating_add(actual_credits_purchased);

            // get the current per-credit cost of a block
            let (block_cost, _weight) = T::ProvideBlockProductionCost::block_cost(&para_id);
            if let Some(max_price_per_credit) = max_price_per_credit {
                ensure!(
                    block_cost <= max_price_per_credit,
                    Error::<T>::CreditPriceTooExpensive,
                );
            }

            let total_fee = block_cost.saturating_mul(actual_credits_purchased.into());

            T::OnChargeForBlockCredit::charge_credits(
                &account,
                &para_id,
                actual_credits_purchased,
                total_fee,
            )?;

            BlockProductionCredits::<T>::insert(para_id, updated_credits);

            Self::deposit_event(Event::<T>::CreditsPurchased {
                para_id,
                payer: account,
                fee: total_fee,
                credits_purchased: actual_credits_purchased,
                credits_remaining: updated_credits,
            });

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Burn a credit for the given para. Deducts one credit if possible, errors otherwise.
        pub fn burn_credit_for_para(para_id: &ParaId) -> DispatchResultWithPostInfo {
            let existing_credits =
                BlockProductionCredits::<T>::get(para_id).unwrap_or(BlockNumberFor::<T>::zero());

            ensure!(
                existing_credits >= 1u32.into(),
                Error::<T>::InsufficientCredits,
            );

            let updated_credits = existing_credits.saturating_sub(1u32.into());
            BlockProductionCredits::<T>::insert(para_id, updated_credits);

            Self::deposit_event(Event::<T>::CreditBurned {
                para_id: *para_id,
                credits_remaining: updated_credits,
            });

            Ok(().into())
        }
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        _phantom: PhantomData<T>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                _phantom: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {}
    }
}

/// Balance used by this pallet
pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

/// Handler for fee charging. This will be invoked when fees need to be deducted from the fee
/// account for a given paraId.
pub trait OnChargeForBlockCredit<T: Config> {
    fn charge_credits(
        payer: &T::AccountId,
        para_id: &ParaId,
        credits: BlockNumberFor<T>,
        fee: BalanceOf<T>,
    ) -> Result<(), Error<T>>;
}

/// Returns the cost for a given block credit at the current time. This can be a complex operation,
/// so it also returns the weight it consumes. (TODO: or just rely on benchmarking)
pub trait ProvideBlockProductionCost<T: Config> {
    fn block_cost(para_id: &ParaId) -> (BalanceOf<T>, Weight);
}
