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

use frame_support::{Deserialize, Serialize};
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(any(test, feature = "runtime-benchmarks"))]
mod benchmarks;
pub mod weights;
pub use weights::WeightInfo;

use tp_traits::{AuthorNotingHook, BlockNumber, SlotFrequency};

use {
    dp_core::ParaId,
    frame_support::{
        dispatch::GetDispatchInfo,
        pallet_prelude::*,
        traits::fungible::{Balanced, Inspect},
    },
    frame_system::pallet_prelude::*,
    parity_scale_codec::EncodeLike,
    sp_consensus_aura::Slot,
    sp_runtime::traits::{AccountIdConversion, Convert, Get},
    sp_std::{vec, vec::Vec},
    staging_xcm::{
        latest::{Asset, Assets, InteriorLocation, Response, Xcm},
        prelude::*,
    },
    tp_traits::{LatestAuthorInfoFetcher, ParathreadParams},
    tp_xcm_core_buyer::BuyCoreCollatorProof,
};

pub trait XCMNotifier<T: Config> {
    fn new_notify_query(
        responder: impl Into<Location>,
        notify: impl Into<<T as Config>::RuntimeCall>,
        timeout: BlockNumberFor<T>,
        match_querier: impl Into<Location>,
    ) -> u64;
}

/// Dummy implementation. Should only be used for testing.
impl<T: Config> XCMNotifier<T> for () {
    fn new_notify_query(
        _responder: impl Into<Location>,
        _notify: impl Into<<T as Config>::RuntimeCall>,
        _timeout: BlockNumberFor<T>,
        _match_querier: impl Into<Location>,
    ) -> u64 {
        0
    }
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(RuntimeDebug, PartialEq, Eq, Encode, Decode, Clone, TypeInfo)]
pub struct InFlightCoreBuyingOrder<BN> {
    para_id: ParaId,
    query_id: QueryId,
    ttl: BN,
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub enum BuyingError<BlockNumber> {
    OrderAlreadyExists {
        ttl: BlockNumber,
        current_block_number: BlockNumber,
    },
    BlockProductionPending {
        ttl: BlockNumber,
        current_block_number: BlockNumber,
    },
    NotAParathread,
    NotAllowedToProduceBlockRightNow {
        slot_frequency: SlotFrequency,
        max_slot_earlier_core_buying_permitted: Slot,
        last_block_production_slot: Slot,
    },
}

impl<T: Config> AuthorNotingHook<T::AccountId> for Pallet<T> {
    fn on_container_author_noted(
        _author: &T::AccountId,
        _block_number: BlockNumber,
        para_id: ParaId,
    ) -> Weight {
        PendingBlocks::<T>::remove(para_id);

        T::DbWeight::get().writes(1)
    }
}

#[frame_support::pallet]
pub mod pallet {
    use {
        super::*, nimbus_primitives::SlotBeacon, pallet_xcm::ensure_response,
        sp_runtime::RuntimeAppPublic,
    };

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
        /// How to convert a `ParaId` into an `AccountId32`. Used to derive the parathread tank
        /// account in `interior_multilocation`.
        type GetParathreadAccountId: Convert<ParaId, [u8; 32]>;
        /// The max price that the parathread is willing to pay for a core, in relay chain currency.
        /// If `None`, defaults to `u128::MAX`, the parathread will pay the market price with no
        /// upper bound.
        type GetParathreadMaxCorePrice: GetParathreadMaxCorePrice;
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

        /// Get the parathread params. Used to verify that the para id is a parathread.
        // TODO: and in the future to restrict the ability to buy a core depending on slot frequency
        type GetParathreadParams: GetParathreadParams;
        /// Validate if particular account id and public key pair belongs to a collator and the collator
        /// is selected to collate for particular para id.
        type CheckCollatorValidity: CheckCollatorValidity<Self::AccountId, Self::CollatorPublicKey>;
        /// A configuration for base priority of unsigned transactions.
        ///
        /// This is exposed so that it can be tuned for particular runtime, when
        /// multiple pallets send unsigned transactions.
        #[pallet::constant]
        type UnsignedPriority: Get<TransactionPriority>;

        /// TTL for pending blocks entry, which prevents anyone to submit another core buying xcm.
        #[pallet::constant]
        type PendingBlocksTtl: Get<BlockNumberFor<Self>>;

        /// TTL to be used in xcm's notify query
        #[pallet::constant]
        type CoreBuyingXCMQueryTtl: Get<BlockNumberFor<Self>>;

        /// Additional ttl for in flight orders (total would be CoreBuyingXCMQueryTtl + AdditionalTtlForInflightOrders)
        /// after which the in flight orders can be cleaned up by anyone.
        #[pallet::constant]
        type AdditionalTtlForInflightOrders: Get<BlockNumberFor<Self>>;

        #[pallet::constant]
        type UniversalLocation: Get<InteriorLocation>;

        type RuntimeOrigin: Into<Result<pallet_xcm::Origin, <Self as Config>::RuntimeOrigin>>
            + From<<Self as frame_system::Config>::RuntimeOrigin>;

        /// The overarching call type
        type RuntimeCall: From<Call<Self>> + Encode + GetDispatchInfo;

        /// Outcome notifier implements functionality to enable reporting back the outcome
        type XCMNotifier: XCMNotifier<Self>;

        type LatestAuthorInfoFetcher: LatestAuthorInfoFetcher<Self::AccountId>;

        type SlotBeacon: SlotBeacon;

        /// A PublicKey can be converted into an `AccountId`. This is required in order to verify
        /// the collator signature
        type CollatorPublicKey: Member
            + Parameter
            + RuntimeAppPublic
            + MaybeSerializeDeserialize
            + MaxEncodedLen;

        type WeightInfo: WeightInfo;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// An XCM message to buy a core for this parathread has been sent to the relay chain.
        BuyCoreXcmSent {
            para_id: ParaId,
            transaction_status_query_id: QueryId,
        },
        /// We received response for xcm
        ReceivedBuyCoreXCMResult { para_id: ParaId, response: Response },

        /// We cleaned up expired pending blocks entries.
        CleanedUpExpiredPendingBlocksEntries { para_ids: Vec<ParaId> },

        /// We cleaned up expired in flight orders entries.
        CleanedUpExpiredInFlightOrderEntries { para_ids: Vec<ParaId> },
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
        /// Inverting location from destination point of view failed
        LocationInversionFailed,
        /// Modifying XCM to report the result of XCM failed
        ReportNotifyingSetupFailed,
        /// Unexpected XCM response
        UnexpectedXCMResponse,
        /// Block production is pending for para id with successfully placed order
        BlockProductionPending,
        /// Block production is not allowed for current slot
        NotAllowedToProduceBlockRightNow,
        /// Collator signature nonce is incorrect
        IncorrectCollatorSignatureNonce,
        /// Collator signature is invalid
        InvalidCollatorSignature,
    }

    impl<T: Config> From<BuyingError<BlockNumberFor<T>>> for Error<T> {
        fn from(value: BuyingError<BlockNumberFor<T>>) -> Self {
            match value {
                BuyingError::OrderAlreadyExists { .. } => Error::<T>::OrderAlreadyExists,
                BuyingError::BlockProductionPending { .. } => Error::<T>::BlockProductionPending,
                BuyingError::NotAParathread => Error::<T>::NotAParathread,
                BuyingError::NotAllowedToProduceBlockRightNow { .. } => {
                    Error::<T>::NotAllowedToProduceBlockRightNow
                }
            }
        }
    }

    /// Set of parathreads that have already sent an XCM message to buy a core recently.
    /// Used to avoid 2 collators buying a core at the same time, because it is only possible to buy
    /// 1 core in 1 relay block for the same parathread.
    #[pallet::storage]
    pub type InFlightOrders<T: Config> =
        StorageMap<_, Twox128, ParaId, InFlightCoreBuyingOrder<BlockNumberFor<T>>, OptionQuery>;

    /// Number of pending blocks
    #[pallet::storage]
    pub type PendingBlocks<T: Config> =
        StorageMap<_, Twox128, ParaId, BlockNumberFor<T>, OptionQuery>;

    /// Mapping of QueryId to ParaId
    #[pallet::storage]
    pub type QueryIdToParaId<T: Config> = StorageMap<_, Twox128, QueryId, ParaId, OptionQuery>;

    /// This must be set by root with the value of the relay chain xcm call weight and extrinsic
    /// weight limit. This is a storage item because relay chain weights can change, so we need to
    /// be able to adjust them without doing a runtime upgrade.
    #[pallet::storage]
    pub type RelayXcmWeightConfig<T: Config> =
        StorageValue<_, RelayXcmWeightConfigInner<T>, OptionQuery>;

    /// Collator signature nonce for reply protection
    #[pallet::storage]
    pub type CollatorSignatureNonce<T: Config> = StorageMap<_, Twox128, ParaId, u64, ValueQuery>;

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
        #[pallet::weight(T::WeightInfo::buy_core())]
        pub fn buy_core(
            origin: OriginFor<T>,
            para_id: ParaId,
            // Below parameter are already validated during `validate_unsigned` call
            _collator_account_id: T::AccountId,
            _proof: BuyCoreCollatorProof<T::CollatorPublicKey>,
        ) -> DispatchResult {
            ensure_none(origin)?;

            let current_nonce = CollatorSignatureNonce::<T>::get(para_id);
            CollatorSignatureNonce::<T>::set(para_id, current_nonce + 1);

            Self::on_collator_instantaneous_core_requested(para_id)
        }

        /// Buy core for para id as root. Does not require any proof, useful in tests.
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::force_buy_core())]
        pub fn force_buy_core(origin: OriginFor<T>, para_id: ParaId) -> DispatchResult {
            ensure_root(origin)?;

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

        #[pallet::call_index(4)]
        #[pallet::weight(T::WeightInfo::query_response())]
        pub fn query_response(
            origin: OriginFor<T>,
            query_id: QueryId,
            response: Response,
        ) -> DispatchResult {
            let _responder = ensure_response(<T as Config>::RuntimeOrigin::from(origin))?;

            let maybe_para_id = QueryIdToParaId::<T>::get(query_id);

            let para_id = if let Some(para_id) = maybe_para_id {
                para_id
            } else {
                // Most probably entry was expired or removed in some other way. Let's return early.
                return Ok(());
            };

            QueryIdToParaId::<T>::remove(query_id);
            InFlightOrders::<T>::remove(para_id);

            match response {
                Response::DispatchResult(MaybeErrorCode::Success) => {
                    // Success. Add para id to pending block
                    let now = <frame_system::Pallet<T>>::block_number();
                    let ttl = T::PendingBlocksTtl::get();
                    PendingBlocks::<T>::insert(para_id, now + ttl);
                }
                Response::DispatchResult(_) => {
                    // We do not add paraid to pending block on failure
                }
                _ => {
                    // Unexpected.
                    return Err(Error::<T>::UnexpectedXCMResponse.into());
                }
            }

            Self::deposit_event(Event::ReceivedBuyCoreXCMResult { para_id, response });

            Ok(())
        }

        #[pallet::call_index(5)]
        #[pallet::weight(T::WeightInfo::clean_up_expired_in_flight_orders(expired_pending_blocks_para_id.len() as u32))]
        pub fn clean_up_expired_pending_blocks(
            origin: OriginFor<T>,
            expired_pending_blocks_para_id: Vec<ParaId>,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            let now = frame_system::Pallet::<T>::block_number();
            let mut cleaned_up_para_ids = vec![];

            for para_id in expired_pending_blocks_para_id {
                let maybe_pending_block_ttl = PendingBlocks::<T>::get(para_id);
                if let Some(pending_block_ttl) = maybe_pending_block_ttl {
                    if pending_block_ttl < now {
                        PendingBlocks::<T>::remove(para_id);
                        cleaned_up_para_ids.push(para_id);
                    } else {
                        // Ignore if not expired
                    }
                }
            }

            Self::deposit_event(Event::CleanedUpExpiredPendingBlocksEntries {
                para_ids: cleaned_up_para_ids,
            });

            Ok(())
        }

        #[pallet::call_index(6)]
        #[pallet::weight(T::WeightInfo::clean_up_expired_in_flight_orders(expired_in_flight_orders.len() as u32))]
        pub fn clean_up_expired_in_flight_orders(
            origin: OriginFor<T>,
            expired_in_flight_orders: Vec<ParaId>,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            let now = frame_system::Pallet::<T>::block_number();
            let mut cleaned_up_para_ids = vec![];

            for para_id in expired_in_flight_orders {
                let maybe_in_flight_order = InFlightOrders::<T>::get(para_id);
                if let Some(in_flight_order) = maybe_in_flight_order {
                    if in_flight_order.ttl < now {
                        InFlightOrders::<T>::remove(para_id);
                        QueryIdToParaId::<T>::remove(in_flight_order.query_id);
                        cleaned_up_para_ids.push(para_id);
                    } else {
                        // Ignore if not expired
                    }
                }
            }

            Self::deposit_event(Event::CleanedUpExpiredInFlightOrderEntries {
                para_ids: cleaned_up_para_ids,
            });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Returns the interior multilocation for this container chain para id. This is a relative
        /// multilocation that can be used in the `descend_origin` XCM opcode.
        pub fn interior_multilocation(para_id: ParaId) -> InteriorLocation {
            let container_chain_account = T::GetParathreadAccountId::convert(para_id);
            let account_junction = Junction::AccountId32 {
                id: container_chain_account,
                network: None,
            };

            [account_junction].into()
        }

        /// Returns a multilocation that can be used in the `deposit_asset` XCM opcode.
        /// The `interior_multilocation` can be obtained using `Self::interior_multilocation`.
        pub fn relay_relative_multilocation(
            interior_multilocation: InteriorLocation,
        ) -> Result<Location, Error<T>> {
            let relay_chain = Location::parent();
            let context: InteriorLocation = [Parachain(T::SelfParaId::get().into())].into();
            let mut reanchored: Location = interior_multilocation.into();
            reanchored
                .reanchor(&relay_chain, &context)
                .map_err(|_| Error::<T>::ReanchorFailed)?;

            Ok(reanchored)
        }

        pub fn is_core_buying_allowed(
            para_id: ParaId,
        ) -> Result<(), BuyingError<BlockNumberFor<T>>> {
            // If an in flight order is pending (i.e we did not receive the notification yet) and our
            // record is not expired yet, we should not allow the collator to buy another core.
            let maybe_in_flight_order = InFlightOrders::<T>::get(para_id);
            if let Some(in_flight_order) = maybe_in_flight_order {
                if in_flight_order.ttl < <frame_system::Pallet<T>>::block_number() {
                    InFlightOrders::<T>::remove(para_id);
                } else {
                    return Err(BuyingError::OrderAlreadyExists {
                        ttl: in_flight_order.ttl,
                        current_block_number: <frame_system::Pallet<T>>::block_number(),
                    });
                }
            }

            // If a block production is pending and our record is not expired yet, we should not allow
            // the collator to buy another core yet.
            let maybe_pending_blocks_ttl = PendingBlocks::<T>::get(para_id);
            if let Some(pending_blocks_ttl) = maybe_pending_blocks_ttl {
                if pending_blocks_ttl < <frame_system::Pallet<T>>::block_number() {
                    PendingBlocks::<T>::remove(para_id);
                } else {
                    return Err(BuyingError::BlockProductionPending {
                        ttl: pending_blocks_ttl,
                        current_block_number: <frame_system::Pallet<T>>::block_number(),
                    });
                }
            }

            // Check that the para id is a parathread
            let parathread_params = T::GetParathreadParams::get_parathread_params(para_id)
                .ok_or(BuyingError::NotAParathread)?;

            let maybe_latest_author_info =
                T::LatestAuthorInfoFetcher::get_latest_author_info(para_id);
            if let Some(latest_author_info) = maybe_latest_author_info {
                let current_slot = T::SlotBeacon::slot();
                if !parathread_params.slot_frequency.should_parathread_buy_core(
                    Slot::from(current_slot as u64),
                    Slot::from(2u64),
                    latest_author_info.latest_slot_number,
                ) {
                    // TODO: Take max slots to produce a block from config
                    return Err(BuyingError::NotAllowedToProduceBlockRightNow {
                        slot_frequency: parathread_params.slot_frequency,
                        max_slot_earlier_core_buying_permitted: Slot::from(2u64),
                        last_block_production_slot: latest_author_info.latest_slot_number,
                    });
                }
            }

            Ok(())
        }

        /// Send an XCM message to the relay chain to try to buy a core for this para_id.
        fn on_collator_instantaneous_core_requested(para_id: ParaId) -> DispatchResult {
            Self::is_core_buying_allowed(para_id).map_err(Into::<Error<T>>::into)?;

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
            let max_amount =
                T::GetParathreadMaxCorePrice::get_max_core_price(para_id).unwrap_or(u128::MAX);
            let call =
                T::GetPurchaseCoreCall::get_encoded(RelayChain::<T>::get(), max_amount, para_id);
            let weight_at_most = xcm_weights_storage.weight_at_most;

            // Assumption: derived account already has DOT
            // The balance should be enough to cover the `Withdraw` needed to `BuyExecution`, plus
            // the price of the core, which can change based on demand.
            let relay_asset_total: Asset = (Here, withdraw_amount).into();
            let refund_asset_filter: AssetFilter = AssetFilter::Wild(WildAsset::AllCounted(1));

            let interior_multilocation = Self::interior_multilocation(para_id);
            // The parathread tank account is derived from the tanssi sovereign account and the
            // parathread para id.
            let derived_account =
                Self::relay_relative_multilocation(interior_multilocation.clone())?;

            // Need to use `builder_unsafe` because safe `builder` does not allow `descend_origin` as first instruction.
            // We use `descend_origin` instead of wrapping the transact call in `utility.as_derivative`
            // because with `descend_origin` the parathread tank account will pay for fees, while
            // `utility.as_derivative` will make the tanssi sovereign account pay for fees.

            let notify_call = <T as Config>::RuntimeCall::from(Call::<T>::query_response {
                query_id: 0,
                response: Default::default(),
            });
            let notify_call_weight = notify_call.get_dispatch_info().weight;

            let notify_query_ttl =
                <frame_system::Pallet<T>>::block_number() + T::CoreBuyingXCMQueryTtl::get();

            // Send XCM to relay chain
            let relay_chain = Location::parent();
            let query_id = T::XCMNotifier::new_notify_query(
                relay_chain.clone(),
                notify_call,
                notify_query_ttl,
                interior_multilocation.clone(),
            );

            let message: Xcm<()> = Xcm::builder_unsafe()
                .descend_origin(interior_multilocation.clone())
                .withdraw_asset(Assets::from(vec![relay_asset_total.clone()]))
                .buy_execution(relay_asset_total, Unlimited)
                // Both in case of error and in case of success, we want to refund the unused weight
                .set_appendix(
                    Xcm::builder_unsafe()
                        .report_transact_status(QueryResponseInfo {
                            destination: T::UniversalLocation::get()
                                .invert_target(&relay_chain)
                                .map_err(|_| Error::<T>::LocationInversionFailed)?, // This location from the point of view of destination
                            query_id,
                            max_weight: notify_call_weight,
                        })
                        .refund_surplus()
                        .deposit_asset(refund_asset_filter, derived_account)
                        .build(),
                )
                .transact(origin, weight_at_most, call)
                .build();

            // We intentionally do not charge any fees
            let (ticket, _price) =
                T::XcmSender::validate(&mut Some(relay_chain), &mut Some(message))
                    .map_err(|_| Error::<T>::ErrorValidatingXCM)?;
            T::XcmSender::deliver(ticket).map_err(|_| Error::<T>::ErrorDeliveringXCM)?;
            Self::deposit_event(Event::BuyCoreXcmSent {
                para_id,
                transaction_status_query_id: query_id,
            });

            let in_flight_order_ttl = notify_query_ttl + T::AdditionalTtlForInflightOrders::get();
            InFlightOrders::<T>::insert(
                para_id,
                InFlightCoreBuyingOrder {
                    para_id,
                    query_id,
                    ttl: in_flight_order_ttl,
                },
            );

            QueryIdToParaId::<T>::insert(query_id, para_id);

            Ok(())
        }

        pub fn para_deregistered(para_id: ParaId) {
            // If para is deregistered we need to clean up in flight order, query id mapping
            if let Some(in_flight_order) = InFlightOrders::<T>::take(para_id) {
                InFlightOrders::<T>::remove(para_id);
                QueryIdToParaId::<T>::remove(in_flight_order.query_id);
            }

            // We need to clean the pending block entry if any
            PendingBlocks::<T>::remove(para_id);
        }
    }

    #[pallet::validate_unsigned]
    impl<T: Config> ValidateUnsigned for Pallet<T> {
        type Call = Call<T>;

        fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
            if let Call::buy_core {
                para_id,
                collator_account_id,
                proof,
            } = call
            {
                let block_number = <frame_system::Pallet<T>>::block_number();

                let current_nonce = CollatorSignatureNonce::<T>::get(para_id);
                if proof.nonce != current_nonce {
                    return InvalidTransaction::Call.into();
                }

                let is_valid_collator = T::CheckCollatorValidity::is_valid_collator(
                    *para_id,
                    collator_account_id.clone(),
                    proof.public_key.clone(),
                );
                if !is_valid_collator {
                    return InvalidTransaction::Call.into();
                }

                if !proof.verify_signature(*para_id) {
                    return InvalidTransaction::Call.into();
                }

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

pub trait CheckCollatorValidity<AccountId, PublicKey> {
    fn is_valid_collator(para_id: ParaId, account_id: AccountId, public_key: PublicKey) -> bool;

    #[cfg(feature = "runtime-benchmarks")]
    fn set_valid_collator(para_id: ParaId, account_id: AccountId, public_key: PublicKey);
}

pub trait GetParathreadMaxCorePrice {
    fn get_max_core_price(para_id: ParaId) -> Option<u128>;
}

impl GetParathreadMaxCorePrice for () {
    fn get_max_core_price(_para_id: ParaId) -> Option<u128> {
        None
    }
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
