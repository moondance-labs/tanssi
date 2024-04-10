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
        traits::{
            tokens::ExistenceRequirement, Currency, EnsureOriginWithArg, OnUnbalanced,
            WithdrawReasons,
        },
    },
    frame_system::pallet_prelude::*,
    scale_info::prelude::vec::Vec,
    serde::{Deserialize, Serialize},
    sp_io::hashing::blake2_256,
    sp_runtime::{traits::TrailingZeroInput, DispatchError},
    tp_traits::{AuthorNotingHook, BlockNumber, CollatorAssignmentHook, CollatorAssignmentTip},
};

#[cfg(any(test, feature = "runtime-benchmarks"))]
mod benchmarks;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
pub mod weights;
pub use weights::WeightInfo;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Handlers for fees
        type OnChargeForBlock: OnUnbalanced<NegativeImbalanceOf<Self>>;
        type OnChargeForCollatorAssignment: OnUnbalanced<NegativeImbalanceOf<Self>>;
        type OnChargeForCollatorAssignmentTip: OnUnbalanced<NegativeImbalanceOf<Self>>;

        /// Currency type for fee payment
        type Currency: Currency<Self::AccountId>;
        /// Provider of a block cost which can adjust from block to block
        type ProvideBlockProductionCost: ProvideBlockProductionCost<Self>;
        /// Provider of a block cost which can adjust from block to block
        type ProvideCollatorAssignmentCost: ProvideCollatorAssignmentCost<Self>;

        /// The maximum number of block production credits that can be accumulated
        #[pallet::constant]
        type FreeBlockProductionCredits: Get<BlockNumberFor<Self>>;

        /// The maximum number of collator assigment production credits that can be accumulated
        #[pallet::constant]
        type FreeCollatorAssignmentCredits: Get<u32>;
        /// Owner of the container chain, can call some only-owner methods
        type ManagerOrigin: EnsureOriginWithArg<Self::RuntimeOrigin, ParaId>;

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
        BlockProductionCreditBurned {
            para_id: ParaId,
            credits_remaining: BlockNumberFor<T>,
        },
        CollatorAssignmentCreditBurned {
            para_id: ParaId,
            credits_remaining: u32,
        },
        CollatorAssignmentTipCollected {
            para_id: ParaId,
            payer: T::AccountId,
            tip: BalanceOf<T>,
        },
        BlockProductionCreditsSet {
            para_id: ParaId,
            credits: BlockNumberFor<T>,
        },
        RefundAddressUpdated {
            para_id: ParaId,
            refund_address: Option<T::AccountId>,
        },
        MaxCorePriceUpdated {
            para_id: ParaId,
            max_core_price: Option<u128>,
        },
        CollatorAssignmentCreditsSet {
            para_id: ParaId,
            credits: u32,
        },
    }

    #[pallet::storage]
    #[pallet::getter(fn free_block_production_credits)]
    pub type BlockProductionCredits<T: Config> =
        StorageMap<_, Blake2_128Concat, ParaId, BlockNumberFor<T>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn free_collator_assignment_credits)]
    pub type CollatorAssignmentCredits<T: Config> =
        StorageMap<_, Blake2_128Concat, ParaId, u32, OptionQuery>;

    /// List of para ids that have already been given free credits
    #[pallet::storage]
    #[pallet::getter(fn given_free_credits)]
    pub type GivenFreeCredits<T: Config> = StorageMap<_, Blake2_128Concat, ParaId, (), OptionQuery>;

    /// Refund address
    #[pallet::storage]
    #[pallet::getter(fn refund_address)]
    pub type RefundAddress<T: Config> =
        StorageMap<_, Blake2_128Concat, ParaId, T::AccountId, OptionQuery>;

    /// Max core price for parathread in relay chain currency
    #[pallet::storage]
    pub type MaxCorePrice<T: Config> = StorageMap<_, Blake2_128Concat, ParaId, u128, OptionQuery>;

    /// Max tip for collator assignment on congestion
    #[pallet::storage]
    #[pallet::getter(fn max_tip)]
    pub type MaxTip<T: Config> = StorageMap<_, Blake2_128Concat, ParaId, BalanceOf<T>, OptionQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T>
    where
        BlockNumberFor<T>: Into<BalanceOf<T>>,
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
                credit,
            });

            Ok(().into())
        }

        /// Set the number of block production credits for this para_id without paying for them.
        /// Can only be called by root.
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::set_block_production_credits())]
        pub fn set_block_production_credits(
            origin: OriginFor<T>,
            para_id: ParaId,
            free_block_credits: BlockNumberFor<T>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            Self::set_free_block_production_credits(&para_id, free_block_credits);

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

        /// Call index to set the refund address for non-spent tokens
        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::set_refund_address())]
        pub fn set_refund_address(
            origin: OriginFor<T>,
            para_id: ParaId,
            refund_address: Option<T::AccountId>,
        ) -> DispatchResultWithPostInfo {
            T::ManagerOrigin::ensure_origin(origin, &para_id)?;

            if let Some(refund_address) = refund_address.clone() {
                RefundAddress::<T>::insert(para_id, refund_address.clone());
            } else {
                RefundAddress::<T>::remove(para_id);
            }

            Self::deposit_event(Event::<T>::RefundAddressUpdated {
                para_id,
                refund_address,
            });

            Ok(().into())
        }

        /// Set the number of block production credits for this para_id without paying for them.
        /// Can only be called by root.
        #[pallet::call_index(4)]
        #[pallet::weight(T::WeightInfo::set_block_production_credits())]
        pub fn set_collator_assignment_credits(
            origin: OriginFor<T>,
            para_id: ParaId,
            free_collator_assignment_credits: u32,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            Self::set_free_collator_assignment_credits(&para_id, free_collator_assignment_credits);

            Ok(().into())
        }

        /// Max core price for parathread in relay chain currency
        #[pallet::call_index(5)]
        #[pallet::weight(T::WeightInfo::set_max_core_price())]
        pub fn set_max_core_price(
            origin: OriginFor<T>,
            para_id: ParaId,
            max_core_price: Option<u128>,
        ) -> DispatchResultWithPostInfo {
            T::ManagerOrigin::ensure_origin(origin, &para_id)?;

            if let Some(max_core_price) = max_core_price {
                MaxCorePrice::<T>::insert(para_id, max_core_price);
            } else {
                MaxCorePrice::<T>::remove(para_id);
            }

            Self::deposit_event(Event::<T>::MaxCorePriceUpdated {
                para_id,
                max_core_price,
            });

            Ok(().into())
        }

        /// Set the maximum tip a container chain is willing to pay to be assigned a collator on congestion.
        /// Can only be called by container chain manager.
        #[pallet::call_index(6)]
        #[pallet::weight(T::WeightInfo::set_max_tip())]
        pub fn set_max_tip(
            origin: OriginFor<T>,
            para_id: ParaId,
            max_tip: Option<BalanceOf<T>>,
        ) -> DispatchResultWithPostInfo {
            T::ManagerOrigin::ensure_origin(origin, &para_id)?;

            if let Some(max_tip) = max_tip {
                MaxTip::<T>::insert(para_id, max_tip);
            } else {
                MaxTip::<T>::remove(para_id);
            }

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Burn a credit for the given para. Deducts one credit if possible, errors otherwise.
        pub fn burn_block_production_free_credit_for_para(
            para_id: &ParaId,
        ) -> DispatchResultWithPostInfo {
            let existing_credits =
                BlockProductionCredits::<T>::get(para_id).unwrap_or(BlockNumberFor::<T>::zero());

            ensure!(
                existing_credits >= 1u32.into(),
                Error::<T>::InsufficientCredits,
            );

            let updated_credits = existing_credits.saturating_sub(1u32.into());
            BlockProductionCredits::<T>::insert(para_id, updated_credits);

            Self::deposit_event(Event::<T>::BlockProductionCreditBurned {
                para_id: *para_id,
                credits_remaining: updated_credits,
            });

            Ok(().into())
        }

        /// Burn a credit for the given para. Deducts one credit if possible, errors otherwise.
        pub fn burn_collator_assignment_free_credit_for_para(
            para_id: &ParaId,
        ) -> DispatchResultWithPostInfo {
            let existing_credits = CollatorAssignmentCredits::<T>::get(para_id).unwrap_or(0u32);

            ensure!(existing_credits >= 1u32, Error::<T>::InsufficientCredits,);

            let updated_credits = existing_credits.saturating_sub(1u32);
            CollatorAssignmentCredits::<T>::insert(para_id, updated_credits);

            Self::deposit_event(Event::<T>::CollatorAssignmentCreditBurned {
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

            // Set number of credits to FreeBlockProductionCredits
            let block_production_existing_credits =
                BlockProductionCredits::<T>::get(para_id).unwrap_or(BlockNumberFor::<T>::zero());
            let block_production_updated_credits = T::FreeBlockProductionCredits::get();
            // Do not update credits if for some reason this para id had more
            if block_production_existing_credits < block_production_updated_credits {
                Self::set_free_block_production_credits(para_id, block_production_updated_credits);
            }

            // Set number of credits to FreeCollatorAssignmentCredits
            let collator_assignment_existing_credits =
                CollatorAssignmentCredits::<T>::get(para_id).unwrap_or(0u32);
            let collator_assignment_updated_credits = T::FreeCollatorAssignmentCredits::get();

            // Do not update credits if for some reason this para id had more
            if collator_assignment_existing_credits < collator_assignment_updated_credits {
                Self::set_free_collator_assignment_credits(
                    para_id,
                    collator_assignment_updated_credits,
                );
            }

            // We only allow to call this function once per para id, even if it didn't actually
            // receive all the free credits
            GivenFreeCredits::<T>::insert(para_id, ());

            Weight::default()
        }

        pub fn set_free_collator_assignment_credits(
            para_id: &ParaId,
            free_collator_assignment_credits: u32,
        ) {
            if free_collator_assignment_credits.is_zero() {
                CollatorAssignmentCredits::<T>::remove(para_id);
            } else {
                CollatorAssignmentCredits::<T>::insert(para_id, free_collator_assignment_credits);
            }

            Self::deposit_event(Event::<T>::CollatorAssignmentCreditsSet {
                para_id: *para_id,
                credits: free_collator_assignment_credits,
            });
        }

        pub fn set_free_block_production_credits(
            para_id: &ParaId,
            free_collator_block_production_credits: BlockNumberFor<T>,
        ) {
            if free_collator_block_production_credits.is_zero() {
                BlockProductionCredits::<T>::remove(para_id);
            } else {
                BlockProductionCredits::<T>::insert(
                    para_id,
                    free_collator_block_production_credits,
                );
            }

            Self::deposit_event(Event::<T>::BlockProductionCreditsSet {
                para_id: *para_id,
                credits: free_collator_block_production_credits,
            });
        }
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub para_id_credits: Vec<FreeCreditGenesisParams<BlockNumberFor<T>>>,
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
            for para_id_credits in &self.para_id_credits {
                BlockProductionCredits::<T>::insert(
                    para_id_credits.para_id,
                    para_id_credits.block_production_credits,
                );
                CollatorAssignmentCredits::<T>::insert(
                    para_id_credits.para_id,
                    para_id_credits.collator_assignment_credits,
                );
            }
        }
    }
}

// Params to be set in genesis
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo, Serialize, Deserialize)]
pub struct FreeCreditGenesisParams<BlockProductCredits> {
    pub para_id: ParaId,
    pub block_production_credits: BlockProductCredits,
    pub collator_assignment_credits: u32,
}
impl<BlockProductCredits> From<(ParaId, BlockProductCredits, u32)>
    for FreeCreditGenesisParams<BlockProductCredits>
{
    fn from(value: (ParaId, BlockProductCredits, u32)) -> Self {
        Self {
            para_id: value.0,
            block_production_credits: value.1,
            collator_assignment_credits: value.2,
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
        if Pallet::<T>::burn_block_production_free_credit_for_para(&para_id).is_err() {
            let (amount_to_charge, _weight) = T::ProvideBlockProductionCost::block_cost(&para_id);
            match T::Currency::withdraw(
                &Self::parachain_tank(para_id),
                amount_to_charge,
                WithdrawReasons::FEE,
                ExistenceRequirement::KeepAlive,
            ) {
                Err(e) => log::warn!(
                    "Failed to withdraw block production payment for container chain {}: {:?}",
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

impl<T: Config> CollatorAssignmentHook<BalanceOf<T>> for Pallet<T> {
    // is_parathread parameter for future use to apply different logic
    fn on_collators_assigned(
        para_id: ParaId,
        maybe_tip: Option<&BalanceOf<T>>,
        _is_parathread: bool,
    ) -> Result<Weight, DispatchError> {
        // Withdraw assignment fee
        let maybe_assignment_imbalance =
            if Pallet::<T>::burn_collator_assignment_free_credit_for_para(&para_id).is_err() {
                let (amount_to_charge, _weight) =
                    T::ProvideCollatorAssignmentCost::collator_assignment_cost(&para_id);
                Some(T::Currency::withdraw(
                    &Self::parachain_tank(para_id),
                    amount_to_charge,
                    WithdrawReasons::FEE,
                    ExistenceRequirement::KeepAlive,
                )?)
            } else {
                None
            };

        if let Some(&tip) = maybe_tip {
            // Only charge the tip to the paras that had a max tip set
            // (aka were willing to tip for being assigned a collator)
            if MaxTip::<T>::get(para_id).is_some() {
                match T::Currency::withdraw(
                    &Self::parachain_tank(para_id),
                    tip,
                    WithdrawReasons::TIP,
                    ExistenceRequirement::KeepAlive,
                ) {
                    Err(e) => {
                        // Return assignment imbalance to tank on error
                        if let Some(assignment_imbalance) = maybe_assignment_imbalance {
                            T::Currency::resolve_creating(
                                &Self::parachain_tank(para_id),
                                assignment_imbalance,
                            );
                        }
                        return Err(e);
                    }
                    Ok(tip_imbalance) => {
                        Self::deposit_event(Event::<T>::CollatorAssignmentTipCollected {
                            para_id,
                            payer: Self::parachain_tank(para_id),
                            tip,
                        });
                        T::OnChargeForCollatorAssignmentTip::on_unbalanced(tip_imbalance);
                    }
                }
            }
        }

        if let Some(assignment_imbalance) = maybe_assignment_imbalance {
            T::OnChargeForCollatorAssignment::on_unbalanced(assignment_imbalance);
        }

        Ok(T::WeightInfo::on_collators_assigned())
    }
}

impl<T: Config> CollatorAssignmentTip<BalanceOf<T>> for Pallet<T> {
    fn get_para_tip(para_id: ParaId) -> Option<BalanceOf<T>> {
        MaxTip::<T>::get(para_id)
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
                if let Some(address) = RefundAddress::<T>::get(para_id) {
                    T::Currency::resolve_creating(&address, imbalance);
                } else {
                    // Burn for now, we might be able to pass something to do with this
                    drop(imbalance);
                }
            }
        }

        // Clean refund addres
        RefundAddress::<T>::remove(para_id);

        // Clean credits
        BlockProductionCredits::<T>::remove(para_id);
        CollatorAssignmentCredits::<T>::remove(para_id);
        MaxTip::<T>::remove(para_id);
        MaxCorePrice::<T>::remove(para_id);
    }
}
