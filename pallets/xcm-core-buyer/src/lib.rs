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

//! # XCM Core Buyer Pallet
//!
//! This pallet allows collators to buy parathread cores on demand.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(any(test, feature = "runtime-benchmarks"))]
mod benchmarks;
pub mod weights;

use {
    crate::weights::WeightInfo,
    dp_core::ParaId,
    frame_support::{
        pallet_prelude::*,
        traits::fungible::{Balanced, Inspect},
    },
    frame_system::pallet_prelude::*,
    parity_scale_codec::EncodeLike,
    sp_runtime::traits::{AccountIdConversion, Convert, Get},
    sp_std::{vec, vec::Vec},
    staging_xcm::{
        prelude::*,
        v3::{InteriorMultiLocation, MultiAsset, MultiAssets, Xcm},
    },
    tp_traits::ParathreadParams,
};

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Inspect<Self::AccountId> + Balanced<Self::AccountId>;

        type XcmSender: SendXcm;
        /// Get encoded call to buy a core in the relay chain. This will be passed to the XCM
        /// `Transact` instruction.
        type GetPurchaseCoreCall: GetPurchaseCoreCall<Self::RelayChain>;
        /// Get current block number, used in `validate_unsigned`.
        type GetBlockNumber: Get<u32>;
        /// How to convert a `ParaId` into an `AccountId32`. Used to derive the parathread tank
        /// account in `interior_multilocation`.
        type GetParathreadAccountId: Convert<ParaId, [u8; 32]>;
        /// Orchestartor chain `ParaId`. Used in `absolute_multilocation` to convert the
        /// `interior_multilocation` into what the relay chain needs to allow to `DepositAsset`.
        type SelfParaId: Get<ParaId>;
        type RelayChain: Default
            + Encode
            + Decode
            + TypeInfo
            + EncodeLike
            + Clone
            + PartialEq
            + sp_std::fmt::Debug;
        /// Limit how many in-flight XCM requests can be sent to the relay chain in one block.
        #[pallet::constant]
        type MaxParathreads: Get<u32>;
        /// Get the parathread params. Used to verify that the para id is a parathread.
        // TODO: and in the future to restrict the ability to buy a core depending on slot frequency
        type GetParathreadParams: GetParathreadParams;
        /// Get a list of collators assigned to this parathread. Used to verify the collator proof.
        type GetAssignedCollators: GetParathreadCollators<Self::AccountId>;
        /// A configuration for base priority of unsigned transactions.
        ///
        /// This is exposed so that it can be tuned for particular runtime, when
        /// multiple pallets send unsigned transactions.
        #[pallet::constant]
        type UnsignedPriority: Get<TransactionPriority>;

        type WeightInfo: WeightInfo;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// An XCM message to buy a core for this parathread has been sent to the relay chain.
        BuyCoreXcmSent { para_id: ParaId },
    }

    #[pallet::error]
    pub enum Error<T> {
        InvalidProof,
        ErrorValidatingXCM,
        ErrorDeliveringXCM,
        /// An order for this para id already exists
        OrderAlreadyExists,
        /// The para id is not a parathread
        NotAParathread,
        /// There are too many in-flight orders, buying cores will not work until some of those
        /// orders finish.
        InFlightLimitReached,
        /// There are no collators assigned to this parathread, so no point in buying a core
        NoAssignedCollators,
        /// This collator is not assigned to this parathread
        CollatorNotAssigned,
        /// The `XcmWeights` storage has not been set. This must have been set by root with the
        /// value of the relay chain xcm call weight and extrinsic weight
        XcmWeightStorageNotSet,
        /// Converting a multilocation into a relay relative multilocation failed
        ReanchorFailed,
    }

    /// Proof that I am a collator, assigned to a para_id, and I can buy a core for that para_id
    #[derive(Encode, Decode, CloneNoBound, PartialEq, Eq, DebugNoBound, TypeInfo)]
    #[scale_info(skip_type_params(T))]
    pub struct BuyCoreCollatorProof<T: Config> {
        account: T::AccountId,
        // TODO
        _signature: (),
    }

    /// Set of parathreads that have already sent an XCM message to buy a core recently.
    /// Used to avoid 2 collators buying a core at the same time, because it is only possible to buy
    /// 1 core in 1 relay block for the same parathread.
    #[pallet::storage]
    pub type InFlightOrders<T: Config> =
        StorageValue<_, BoundedBTreeSet<ParaId, T::MaxParathreads>, ValueQuery>;

    /// This must be set by root with the value of the relay chain xcm call weight and extrinsic
    /// weight limit. This is a storage item because relay chain weights can change, so we need to
    /// be able to adjust them without doing a runtime upgrade.
    #[pallet::storage]
    pub type RelayXcmWeightConfig<T: Config> =
        StorageValue<_, RelayXcmWeightConfigInner<T>, OptionQuery>;

    #[derive(Encode, Decode, CloneNoBound, PartialEq, Eq, DebugNoBound, TypeInfo)]
    #[scale_info(skip_type_params(T))]
    pub struct RelayXcmWeightConfigInner<T> {
        pub buy_execution_cost: u128,
        pub weight_at_most: Weight,
        pub _phantom: PhantomData<T>,
    }

    /// This must be set by root with the value of the relay chain xcm call weight and extrinsic
    /// weight limit. This is a storage item because relay chain weights can change, so we need to
    /// be able to adjust them without doing a runtime upgrade.
    #[pallet::storage]
    pub type RelayChain<T: Config> = StorageValue<_, T::RelayChain, ValueQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Buy a core for this parathread id.
        /// Collators should call this to indicate that they intend to produce a block, but they
        /// cannot do it because this para id has no available cores.
        /// The purchase is automatic using XCM, and collators do not need to do anything.
        // Note that the collators that will be calling this function are parathread collators, not
        // tanssi collators. So we cannot force them to provide a complex proof, e.g. against relay
        // state.
        #[pallet::call_index(0)]
        // TODO: weight
        #[pallet::weight(T::WeightInfo::force_buy_core(T::MaxParathreads::get()))]
        pub fn buy_core(
            origin: OriginFor<T>,
            para_id: ParaId,
            // since signature verification is done in `validate_unsigned`
            // we can skip doing it here again.
            proof: BuyCoreCollatorProof<T>,
        ) -> DispatchResult {
            // Signature verification is done in `validate_unsigned`.
            // We use `ensure_none` here because this can only be called by collators, and we do not
            // want collators to pay fees.
            ensure_none(origin)?;

            let assigned_collators = T::GetAssignedCollators::get_parathread_collators(para_id);
            if assigned_collators.is_empty() {
                return Err(Error::<T>::NoAssignedCollators.into());
            }

            if !assigned_collators.contains(&proof.account) {
                return Err(Error::<T>::CollatorNotAssigned.into());
            }

            // TODO: implement proof validation
            return Err(Error::<T>::InvalidProof.into());

            //Self::on_collator_instantaneous_core_requested(para_id)
        }

        /// Buy core for para id as root. Does not require any proof, useful in tests.
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::force_buy_core(T::MaxParathreads::get()))]
        pub fn force_buy_core(origin: OriginFor<T>, para_id: ParaId) -> DispatchResult {
            ensure_root(origin)?;

            // Check that at least one collator could buy a core for this parathread.
            // Even though this extrinsic is called `force`, it should only be possible
            // to use it when an equivalent non-force call can be created.
            let assigned_collators = T::GetAssignedCollators::get_parathread_collators(para_id);
            if assigned_collators.is_empty() {
                return Err(Error::<T>::NoAssignedCollators.into());
            }

            Self::on_collator_instantaneous_core_requested(para_id)
        }

        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::set_relay_xcm_weight_config())]
        pub fn set_relay_xcm_weight_config(
            origin: OriginFor<T>,
            xcm_weights: Option<RelayXcmWeightConfigInner<T>>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            if let Some(xcm_weights) = xcm_weights {
                RelayXcmWeightConfig::<T>::put(xcm_weights);
            } else {
                RelayXcmWeightConfig::<T>::kill();
            }

            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::set_relay_chain())]
        pub fn set_relay_chain(
            origin: OriginFor<T>,
            relay_chain: Option<T::RelayChain>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            if let Some(relay_chain) = relay_chain {
                RelayChain::<T>::put(relay_chain);
            } else {
                RelayChain::<T>::kill();
            }

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Returns the interior multilocation for this container chain para id. This is a relative
        /// multilocation that can be used in the `descend_origin` XCM opcode.
        pub fn interior_multilocation(para_id: ParaId) -> InteriorMultiLocation {
            let container_chain_account = T::GetParathreadAccountId::convert(para_id);
            let account_junction = Junction::AccountId32 {
                id: container_chain_account,
                network: None,
            };

            InteriorMultiLocation::X1(account_junction)
        }

        /// Returns a multilocation that can be used in the `deposit_asset` XCM opcode.
        /// The `interior_multilocation` can be obtained using `Self::interior_multilocation`.
        pub fn relay_relative_multilocation(
            interior_multilocation: InteriorMultiLocation,
        ) -> Result<MultiLocation, Error<T>> {
            let relay_chain = MultiLocation::parent();
            let context = Parachain(T::SelfParaId::get().into()).into();
            let mut reanchored: MultiLocation = interior_multilocation.into();
            reanchored
                .reanchor(&relay_chain, context)
                .map_err(|_| Error::<T>::ReanchorFailed)?;

            Ok(reanchored)
        }

        /// Send an XCM message to the relay chain to try to buy a core for this para_id.
        fn on_collator_instantaneous_core_requested(para_id: ParaId) -> DispatchResult {
            let mut in_flight_orders = InFlightOrders::<T>::get();
            if in_flight_orders.contains(&para_id) {
                return Err(Error::<T>::OrderAlreadyExists.into());
            }
            in_flight_orders
                .try_insert(para_id)
                .map_err(|_| Error::<T>::InFlightLimitReached)?;

            // Check that the para id is a parathread
            let parathread_params = T::GetParathreadParams::get_parathread_params(para_id);
            if parathread_params.is_none() {
                return Err(Error::<T>::NotAParathread.into());
            }

            // TODO: also compare the latest slot from pallet_author_noting with parathread_params.slot_frequency

            let xcm_weights_storage =
                RelayXcmWeightConfig::<T>::get().ok_or(Error::<T>::XcmWeightStorageNotSet)?;

            let withdraw_amount = xcm_weights_storage.buy_execution_cost;

            // Use the account derived from the multilocation composed with DescendOrigin
            // Buy on-demand cores
            // Any failure should return everything to the derivative account

            // Don't use utility::as_derivative because that will make the tanssi sovereign account
            // pay for fees, instead use `DescendOrigin` to make the parathread tank account
            // pay for fees.
            // TODO: when coretime is implemented, use coretime instantaneous credits instead of
            // buying on-demand cores at the price defined by the relay
            let origin = OriginKind::SovereignAccount;
            // TODO: max_amount is the max price of a core that this parathread is willing to pay
            // It should be defined in a storage item somewhere, controllable by the container chain
            // manager.
            let max_amount = u128::MAX;
            let call =
                T::GetPurchaseCoreCall::get_encoded(RelayChain::<T>::get(), max_amount, para_id);
            let weight_at_most = xcm_weights_storage.weight_at_most;

            // Assumption: derived account already has DOT
            // The balance should be enough to cover the `Withdraw` needed to `BuyExecution`, plus
            // the price of the core, which can change based on demand.
            let relay_asset_total: MultiAsset = (Here, withdraw_amount).into();
            let refund_asset_filter: MultiAssetFilter =
                MultiAssetFilter::Wild(WildMultiAsset::AllCounted(1));

            let interior_multilocation = Self::interior_multilocation(para_id);
            // The parathread tank account is derived from the tanssi sovereign account and the
            // parathread para id.
            let derived_account = Self::relay_relative_multilocation(interior_multilocation)?;

            // Need to use `builder_unsafe` because safe `builder` does not allow `descend_origin` as first instruction.
            // We use `descend_origin` instead of wrapping the transact call in `utility.as_derivative`
            // because with `descend_origin` the parathread tank account will pay for fees, while
            // `utility.as_derivative` will make the tanssi sovereign account pay for fees.
            let message: Xcm<()> = Xcm::builder_unsafe()
                .descend_origin(interior_multilocation)
                .withdraw_asset(MultiAssets::from(vec![relay_asset_total.clone()]))
                .buy_execution(relay_asset_total, Unlimited)
                // Both in case of error and in case of success, we want to refund the unused weight
                .set_appendix(
                    Xcm::builder_unsafe()
                        .refund_surplus()
                        .deposit_asset(refund_asset_filter, derived_account)
                        .build(),
                )
                .transact(origin, weight_at_most, call.into())
                .build();

            // Send XCM to relay chain
            let relay_chain = MultiLocation::parent();
            // We intentionally do not charge any fees
            let (ticket, _price) =
                T::XcmSender::validate(&mut Some(relay_chain), &mut Some(message))
                    .map_err(|_| Error::<T>::ErrorValidatingXCM)?;
            T::XcmSender::deliver(ticket).map_err(|_| Error::<T>::ErrorDeliveringXCM)?;
            Self::deposit_event(Event::BuyCoreXcmSent { para_id });
            InFlightOrders::<T>::put(in_flight_orders);

            Ok(())
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
            let mut weight = Weight::zero();

            // 1 write in on_finalize
            weight += T::DbWeight::get().writes(1);

            weight
        }

        fn on_finalize(_: BlockNumberFor<T>) {
            // We clear this storage item because we only need it to prevent collators from buying
            // more than one core for the same parathread in the same relay block
            // TODO: with async backing, it is possible that two tanssi blocks are included in the
            // same relay block, so this kill is not correct, should only kill the storage if the
            // relay block number has changed
            // TODO: this allows collators to send N consecutive messages to buy 1 core for the same
            // parathread, as long as the parathread block is not included in pallet_author_noting.
            // So a malicious collator could drain the parathread tank account by buying cores on
            // every tanssi block but not actually producing any block.
            InFlightOrders::<T>::kill();
        }
    }

    #[pallet::validate_unsigned]
    impl<T: Config> ValidateUnsigned for Pallet<T> {
        type Call = Call<T>;

        fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
            if let Call::buy_core { para_id, proof } = call {
                /*
                if <Pallet<T>>::is_online(heartbeat.authority_index) {
                    // we already received a heartbeat for this authority
                    return InvalidTransaction::Stale.into()
                }

                // check if session index from heartbeat is recent
                let current_session = T::ValidatorSet::session_index();
                if heartbeat.session_index != current_session {
                    return InvalidTransaction::Stale.into()
                }

                // verify that the incoming (unverified) pubkey is actually an authority id
                let keys = Keys::<T>::get();
                if keys.len() as u32 != heartbeat.validators_len {
                    return InvalidTransaction::Custom(INVALID_VALIDATORS_LEN).into()
                }
                let authority_id = match keys.get(heartbeat.authority_index as usize) {
                    Some(id) => id,
                    None => return InvalidTransaction::BadProof.into(),
                };

                // check signature (this is expensive so we do it last).
                let signature_valid = heartbeat.using_encoded(|encoded_heartbeat| {
                    authority_id.verify(&encoded_heartbeat, signature)
                });

                if !signature_valid {
                    return InvalidTransaction::BadProof.into()
                }
                */

                // TODO: read session or block number
                let block_number = T::GetBlockNumber::get();

                // TODO: validate proof
                let _ = proof;

                ValidTransaction::with_tag_prefix("XcmCoreBuyer")
                    .priority(T::UnsignedPriority::get())
                    // TODO: tags
                    .and_provides((block_number, para_id))
                    //.and_provides((current_session, authority_id))
                    //.longevity(
                    //    TryInto::<u64>::try_into(
                    //       T::NextSessionRotation::average_session_length() / 2u32.into(),
                    //    )
                    //        .unwrap_or(64_u64),
                    //)
                    .longevity(64)
                    .propagate(true)
                    .build()
            } else {
                InvalidTransaction::Call.into()
            }
        }
    }
}

pub trait GetPurchaseCoreCall<RelayChain> {
    /// Get the encoded call to buy a core for this `para_id`, with this `max_amount`.
    /// Returns the encoded call and its estimated weight.
    fn get_encoded(relay_chain: RelayChain, max_amount: u128, para_id: ParaId) -> Vec<u8>;
}

pub trait GetParathreadCollators<AccountId> {
    fn get_parathread_collators(para_id: ParaId) -> Vec<AccountId>;

    #[cfg(feature = "runtime-benchmarks")]
    fn set_parathread_collators(para_id: ParaId, collators: Vec<AccountId>);
}

pub trait GetParathreadParams {
    fn get_parathread_params(para_id: ParaId) -> Option<ParathreadParams>;

    #[cfg(feature = "runtime-benchmarks")]
    fn set_parathread_params(para_id: ParaId, parathread_params: Option<ParathreadParams>);
}

/// Use `into_account_truncating` to convert a `ParaId` into a `[u8; 32]`.
pub struct ParaIdIntoAccountTruncating;

impl Convert<ParaId, [u8; 32]> for ParaIdIntoAccountTruncating {
    fn convert(para_id: ParaId) -> [u8; 32] {
        // Derive a 32 byte account id for a parathread. Note that this is not the address of
        // the relay chain parathread tank, but that address is derived from this.
        let account: dp_core::AccountId = para_id.into_account_truncating();

        account.into()
    }
}
