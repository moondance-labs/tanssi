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
        traits::{tokens::ExistenceRequirement, Currency, OnUnbalanced, WithdrawReasons},
    },
    frame_system::pallet_prelude::*,
    scale_info::prelude::vec::Vec,
    sp_io::hashing::blake2_256,
    sp_runtime::traits::TrailingZeroInput,
    tp_traits::{AuthorNotingHook, BlockNumber, CollatorAssignmentHook},
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
        type OnChargeForBlock: OnUnbalanced<NegativeImbalanceOf<Self>>;

        /// Handler for fees
        type OnChargeForCollatorAssignment: OnUnbalanced<NegativeImbalanceOf<Self>>;

        /// Currency type for fee payment
        type Currency: Currency<Self::AccountId>;
        /// Provider of a block cost which can adjust from block to block
        type ProvideBlockProductionCost: ProvideBlockProductionCost<Self>;
        /// Provider of a block cost which can adjust from block to block
        type ProvideCollatorAssignmentCost: ProvideCollatorAssignmentCost<Self>;

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
            credit: BalanceOf<T>,
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

    /// List of para ids that have already been given free credits
    #[pallet::storage]
    #[pallet::getter(fn given_free_credits)]
    pub type GivenFreeCredits<T: Config> = StorageMap<_, Blake2_128Concat, ParaId, (), OptionQuery>;

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
            credit: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let account = ensure_signed(origin)?;
            let parachain_tank = Self::parachain_tank(para_id);
            T::Currency::transfer(
                &account,
                &parachain_tank,
                credit,
                ExistenceRequirement::KeepAlive,
            )?;

            Self::deposit_event(Event::<T>::CreditsPurchased {
                para_id,
                payer: account,
                credit: credit,
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

        /// Helper to set and cleanup the `GivenFreeCredits` storage.
        /// Can only be called by root.
        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::set_given_free_credits())]
        pub fn set_given_free_credits(
            origin: OriginFor<T>,
            para_id: ParaId,
            given_free_credits: bool,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            if given_free_credits {
                GivenFreeCredits::<T>::insert(para_id, ());
            } else {
                GivenFreeCredits::<T>::remove(para_id);
            }

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

        pub fn give_free_credits(para_id: &ParaId) -> Weight {
            if GivenFreeCredits::<T>::contains_key(para_id) {
                // This para id has already received free credits
                return Weight::default();
            }
            // Set number of credits to MaxCreditsStored
            let existing_credits =
                BlockProductionCredits::<T>::get(para_id).unwrap_or(BlockNumberFor::<T>::zero());
            let updated_credits = T::MaxCreditsStored::get();
            // Do not update credits if for some reason this para id had more
            if existing_credits < updated_credits {
                BlockProductionCredits::<T>::insert(para_id, updated_credits);
                Self::deposit_event(Event::<T>::CreditsSet {
                    para_id: *para_id,
                    credits: updated_credits,
                });
            }

            // We only allow to call this function once per para id, even if it didn't actually
            // receive all the free credits
            GivenFreeCredits::<T>::insert(para_id, ());

            Weight::default()
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

pub type CurrencyOf<T> = <T as Config>::Currency;
/// Type alias to conveniently refer to the `Currency::NegativeImbalance` associated type.
pub type NegativeImbalanceOf<T> =
    <CurrencyOf<T> as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;
/// Handler for fee charging. This will be invoked when fees need to be deducted from the fee
/// account for a given paraId.

/// Returns the cost for a given block credit at the current time. This can be a complex operation,
/// so it also returns the weight it consumes. (TODO: or just rely on benchmarking)
pub trait ProvideBlockProductionCost<T: Config> {
    fn block_cost(para_id: &ParaId) -> (BalanceOf<T>, Weight);
}

/// Returns the cost for a given block credit at the current time. This can be a complex operation,
/// so it also returns the weight it consumes. (TODO: or just rely on benchmarking)
pub trait ProvideCollatorAssignmentCost<T: Config> {
    fn collator_assignment_cost(para_id: &ParaId) -> (BalanceOf<T>, Weight);
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
        if Pallet::<T>::burn_credit_for_para(&para_id).is_err() {
            let (amount_to_charge, _weight) = T::ProvideBlockProductionCost::block_cost(&para_id);
            match T::Currency::withdraw(
                &Self::parachain_tank(para_id),
                amount_to_charge,
                WithdrawReasons::FEE,
                ExistenceRequirement::AllowDeath,
            ) {
                Err(e) => log::warn!(
                    "Failed to withdraw credits for container chain {}: {:?}",
                    u32::from(para_id),
                    e
                ),
                Ok(imbalance) => {
                    T::OnChargeForBlock::on_unbalanced(imbalance);
                }
            }
        }

        T::WeightInfo::on_container_author_noted()
    }
}

impl<T: Config> CollatorAssignmentHook for Pallet<T> {
    fn on_collators_assigned(para_id: ParaId) -> Weight {
        let (amount_to_charge, _weight) =
            T::ProvideCollatorAssignmentCost::collator_assignment_cost(&para_id);
        match T::Currency::withdraw(
            &Self::parachain_tank(para_id),
            amount_to_charge,
            WithdrawReasons::FEE,
            ExistenceRequirement::AllowDeath,
        ) {
            Err(e) => log::warn!(
                "Failed to withdraw credits for container chain {}: {:?}",
                u32::from(para_id),
                e
            ),
            Ok(imbalance) => {
                T::OnChargeForCollatorAssignment::on_unbalanced(imbalance);
            }
        }
        Weight::from_parts(0, 0)
    }
}

impl<T: Config> Pallet<T> {
    /// Derive a derivative account ID from the paraId.
    pub fn parachain_tank(para_id: ParaId) -> T::AccountId {
        let entropy = (b"modlpy/serpayment", para_id).using_encoded(blake2_256);
        Decode::decode(&mut TrailingZeroInput::new(entropy.as_ref()))
            .expect("infinite length input; no invalid inputs for type; qed")
    }

    /// Hook to perform things on deregister
    pub fn para_deregistered(para_id: ParaId) {
        // Drain the para-id account from tokens
        let parachain_tank_balance = T::Currency::total_balance(&Self::parachain_tank(para_id));
        if !parachain_tank_balance.is_zero() {
            if let Ok(imbalance) = T::Currency::withdraw(
                &Self::parachain_tank(para_id),
                parachain_tank_balance,
                WithdrawReasons::FEE,
                ExistenceRequirement::AllowDeath,
            ) {
                // Burn for now, we might be able to pass something to do with this
                drop(imbalance);
            }
        }

        // Clean credits
        BlockProductionCredits::<T>::remove(para_id);
    }
}
