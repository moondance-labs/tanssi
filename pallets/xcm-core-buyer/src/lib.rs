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
    sp_io::hashing::blake2_256,
    sp_runtime::traits::{Convert, Get},
    sp_std::vec,
    sp_std::vec::Vec,
    staging_xcm::prelude::*,
    staging_xcm::v3::{InteriorMultiLocation, MultiAsset, MultiAssets, Xcm},
};

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use sp_runtime::traits::TrailingZeroInput;
    use tp_traits::ParathreadParams;

    /// Data preservers pallet.
    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Inspect<Self::AccountId> + Balanced<Self::AccountId>;

        type XcmBuyExecutionDot: Get<u128>;
        type XcmSender: SendXcm;
        type GetPurchaseCoreCall: GetPurchaseCoreCall;
        type GetBlockNumber: Get<u32>;
        // TODO: use AccountIdConversion trait here?
        type AccountIdToArray32: Convert<Self::AccountId, [u8; 32]>;
        type SelfParaId: Get<ParaId>;
        type MaxParathreads: Get<u32>;
        // TODO: do not abuse Get and Convert traits
        type GetParathreadParams: Convert<ParaId, Option<ParathreadParams>>;
        // TODO: Self::CollatorId?
        type GetAssignedCollators: Convert<ParaId, Vec<Self::AccountId>>;
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
    /// 1 core in 1 relay block.
    #[pallet::storage]
    //pub type InFlightOrders<T: Config> = StorageMap<_, Blake2_128Concat, ParaId, (), OptionQuery>;
    pub type InFlightOrders<T: Config> =
        StorageValue<_, BoundedBTreeSet<ParaId, T::MaxParathreads>, ValueQuery>;

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
        #[pallet::weight(T::WeightInfo::set_boot_nodes(1, 1))]
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

            let assigned_collators = T::GetAssignedCollators::convert(para_id);
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
        // TODO: weight
        #[pallet::weight(T::WeightInfo::set_boot_nodes(1, 1))]
        pub fn force_buy_core(origin: OriginFor<T>, para_id: ParaId) -> DispatchResult {
            ensure_root(origin)?;

            // Check that at least one collator could buy a core for this parathread.
            // Even though this extrinsic is called `force`, it should only be possible
            // to use it when an equivalent non-force call can be created.
            let assigned_collators = T::GetAssignedCollators::convert(para_id);
            if assigned_collators.is_empty() {
                return Err(Error::<T>::NoAssignedCollators.into());
            }

            Self::on_collator_instantaneous_core_requested(para_id)
        }
    }

    impl<T: Config> Pallet<T> {
        /// Derive a derivative account ID from the paraId to use as a DOT tank in the relay chain.
        /// This is not the actual address, the actual address can be computed as a derivative of
        /// the tanssi sovereign account and this address.
        pub fn relay_parachain_tank_id(para_id: ParaId) -> T::AccountId {
            // TODO: we could use the services_payment parachain tank account here, it could be
            // easier to remember that it is the same account, but DANCE tokens are stored in
            // tanssi and DOT tokens are stored in the relay chain
            // TODO: and we could go a step further and set the tank address in tanssi
            // equal to the relay chain, but not sure if that's a good idea
            let entropy = (b"modlpy/buycoretim", para_id).using_encoded(blake2_256);
            Decode::decode(&mut TrailingZeroInput::new(entropy.as_ref()))
                .expect("infinite length input; no invalid inputs for type; qed")
        }

        /// Returns the interior multilocation for this container chain para id. This is a relative
        /// multilocation that can be used in the `descend_origin` XCM opcode.
        pub fn interior_multilocation(para_id: ParaId) -> InteriorMultiLocation {
            /*
            // Not using this method in case another pallet also wants to use a derived account for
            // a different purpose.
            let interior_multilocation =
                InteriorMultiLocation::X1(Junction::Parachain(para_id.into()));
             */
            let container_chain_account = Self::relay_parachain_tank_id(para_id);
            let account_junction = Junction::AccountId32 {
                id: T::AccountIdToArray32::convert(container_chain_account),
                network: None,
            };

            InteriorMultiLocation::X1(account_junction)
        }

        /// Returns a multilocation that can be used in the `deposit_asset` XCM opcode.
        /// The `interior_multilocation` can be obtained using `Self::interior_multilocation`.
        pub fn absolute_multilocation(
            interior_multilocation: InteriorMultiLocation,
        ) -> MultiLocation {
            let mut l = interior_multilocation;
            l.push_front(Junction::Parachain(T::SelfParaId::get().into()))
                .expect("multilocation too long");
            MultiLocation::from(l)
        }

        fn on_collator_instantaneous_core_requested(para_id: ParaId) -> DispatchResult {
            let mut in_flight_orders = InFlightOrders::<T>::get();
            if in_flight_orders.contains(&para_id) {
                return Err(Error::<T>::OrderAlreadyExists.into());
            }
            in_flight_orders
                .try_insert(para_id)
                .map_err(|_| Error::<T>::InFlightLimitReached)?;

            // Check that the para id is a parathread
            let parathread_params = T::GetParathreadParams::convert(para_id);
            if parathread_params.is_none() {
                return Err(Error::<T>::NotAParathread.into());
            }

            // TODO: the origin should have rights to create blocks for para_id
            let withdraw_amount = T::XcmBuyExecutionDot::get();

            // Send xcm to the relay
            // Use a derivative account from the sovereign account based on the paraId
            // Buy on-demand cores
            // Any failure should return everything to the derivative account

            // Don't use utility::as_derivative because that will make the tanssi sovereign account
            // pay for fees, instead use `DescendOrigin` to make the container chain sovereign account
            // pay for fees. The container chain sovereign account is derived from the tanssi sovereign
            // account.
            // TODO: when coretime is implemented, buy core there instead of buying on-demand cores
            let origin = OriginKind::SovereignAccount;
            // TODO: max_amount is the max price of a core that this parathread is willing to pay
            // It should be defined in a storage item somewhere, contrallable by the container chain
            // manager.
            let max_amount = u128::MAX;
            let (call, weight_at_most) = T::GetPurchaseCoreCall::get_encoded(max_amount, para_id);

            // Assumption: derived account already has DOT
            // The balance should be enough to cover
            // TODO: we could make this be part of the proof, so collators cannot call this if the
            // derived account does not have enough balance
            // Although that would not be perfect, the relay state can change in the following block,
            // and the xcm message will be executed in the block n+2, where n is the latest relay
            // block number seen from the tanssi block that included this extrinsic.
            let relay_asset_total: MultiAsset = (Here, withdraw_amount).into();
            let refund_asset_filter: MultiAssetFilter =
                MultiAssetFilter::Wild(WildMultiAsset::AllCounted(1));
            // TODO: need better names for this methods.
            //  interior_multilocation is the one used in DescendOrigin
            //  absolute_multilocation is the one used in DepositAsset
            // They can be easily converted from one another, the difference is that absolute_multilocation
            // has an extra "Parachain" junction in the front, using SelfParaId::get()
            let interior_multilocation = Self::interior_multilocation(para_id);
            let derived_account = Self::absolute_multilocation(interior_multilocation);

            // Need to use `builder_unsafe` because safe `builder` does not allow `descend_origin` as first instruction
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

            // Send to destination chain
            let relay_chain = MultiLocation::parent();
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
            // more than one core in the same relay block
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

pub trait GetPurchaseCoreCall {
    /// Get the encoded call to buy a core for this `para_id`, with this `max_amount`.
    /// Returns the encoded call and its estimated weight.
    fn get_encoded(max_amount: u128, para_id: ParaId) -> (Vec<u8>, Weight);
}
