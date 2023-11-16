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
    crate::weights::WeightInfo,
    cumulus_primitives_core::ParaId,
    frame_support::{
        pallet_prelude::*,
        sp_runtime::{traits::Zero, Saturating},
        traits::{tokens::ExistenceRequirement, Currency, WithdrawReasons},
    },
    frame_system::pallet_prelude::*,
    scale_info::prelude::vec::Vec,
    tp_traits::{AuthorNotingHook, BlockNumber},
};

#[cfg(any(test, feature = "runtime-benchmarks"))]
mod benchmarks;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
pub mod weights;

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
        type MaxCreditsStored: Get<BlockNumberFor<Self>>;

        type WeightInfo: WeightInfo;
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
        CreditsSet {
            para_id: ParaId,
            credits: BlockNumberFor<T>,
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
        #[pallet::weight(T::WeightInfo::purchase_credits())]
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

        /// Set the number of block production credits for this para_id without paying for them.
        /// Can only be called by root.
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::set_credits())]
        pub fn set_credits(
            origin: OriginFor<T>,
            para_id: ParaId,
            credits: BlockNumberFor<T>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            if credits.is_zero() {
                BlockProductionCredits::<T>::remove(para_id);
            } else {
                BlockProductionCredits::<T>::insert(para_id, credits);
            }

            Self::deposit_event(Event::<T>::CreditsSet { para_id, credits });

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
        pub para_id_credits: Vec<(ParaId, BlockNumberFor<T>)>,
    }

    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                para_id_credits: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            for (para_id, credits) in &self.para_id_credits {
                BlockProductionCredits::<T>::insert(para_id, credits);
            }
        }
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

pub struct ChargeForBlockCredit<Runtime>(PhantomData<Runtime>);
impl<T: Config> OnChargeForBlockCredit<T> for ChargeForBlockCredit<T> {
    fn charge_credits(
        payer: &T::AccountId,
        _para_id: &ParaId,
        _credits: BlockNumberFor<T>,
        fee: BalanceOf<T>,
    ) -> Result<(), crate::Error<T>> {
        use frame_support::traits::tokens::imbalance::Imbalance;

        let result = T::Currency::withdraw(
            payer,
            fee,
            WithdrawReasons::FEE,
            ExistenceRequirement::AllowDeath,
        );
        let imbalance = result.map_err(|_| crate::Error::InsufficientFundsToPurchaseCredits)?;

        if imbalance.peek() != fee {
            panic!("withdrawn balance incorrect");
        }

        Ok(())
    }
}

/// Returns the cost for a given block credit at the current time. This can be a complex operation,
/// so it also returns the weight it consumes. (TODO: or just rely on benchmarking)
pub trait ProvideBlockProductionCost<T: Config> {
    fn block_cost(para_id: &ParaId) -> (BalanceOf<T>, Weight);
}

impl<T: Config> AuthorNotingHook<T::AccountId> for Pallet<T> {
    // This hook is called when pallet_author_noting sees that the block number of a container chain has increased.
    // Currently we always charge 1 credit, even if a container chain produced more that 1 block in between tanssi
    // blocks.
    fn on_container_author_noted(
        _author: &T::AccountId,
        _block_number: BlockNumber,
        para_id: ParaId,
    ) -> Weight {
        let total_weight = T::DbWeight::get().reads_writes(1, 1);

        if let Err(e) = Pallet::<T>::burn_credit_for_para(&para_id) {
            log::warn!(
                "Failed to burn credits for container chain {}: {:?}",
                u32::from(para_id),
                e
            );
        }

        total_weight
    }
}
