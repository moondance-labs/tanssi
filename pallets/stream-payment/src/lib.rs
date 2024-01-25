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

//! A pallet to create payment streams, where users can setup recurrent payment at some rate per
//! unit of time. The pallet aims to be configurable and usage agnostic:
//!
//! - Runtime configure which assets are supported by providing an `AssetId` type and a type
//! implementing the `Assets` trait which only requires function needed by the pallet (increase
//! deposit when creating or refilling a stream, decrease deposit when closing a stream, and
//! transferring a deposit when the stream payment is performed). Both types allows to easily add
//! new supported assets in the future while being retro-compatible. The pallet make few assumptions
//! about how the funds are deposited (thanks to the custom trait), which should allow to easily
//! support assets from various pallets/sources.
//! - Runtime configure which unit of time is supported to express the rate of payment. Units of
//! time should be monotonically increasing. Users can then choose which unit of time they want to
//! use.
//!
//! The pallet provides the following calls:
//! - `open_stream(target, time_unit, asset_id, rate, initial_deposit)`: The origin creates a stream
//! towards a target (payee), with given time unit, asset and rate. A deposit is made, which is able
//! to pay for `initial_deposit / rate`. Streams are indexed using a `StreamId` which is returned
//! with an event.
//! - `update_stream(stream_id)`: can be called by anyone to update a stream, performing the payment
//! for the elapsed time since the last update. All other calls implicitly call `update_stream`,
//! such that at any point in time you're guaranteed you'll be able to redeem the payment for the
//! elapsed time; which allow to call it only when the funds are needed without fear of non-payment.
//! - `close_stream(stream_id)`: only callable by the source or target of the stream. It pays for
//! the elapsed time then refund the remaining deposit to the source.
//! - `refill_stream(stream_id, increase)`: Increase the deposit in the stream. It first pays with
//! what is left before increasing the deposit, which means a source will not retro-actively pay for
//! a drained stream. A target that provides services in exchange for payment should suspend the
//! service as soon as updating the stream would make it drain, and should resume services once the
//! stream is refilled.
//! - `request_change(stream_id, kind, new_config)`: Allows to request changing the config of the
//! stream. `kind` states if the change is a mere suggestion or is mandatory, in which case there is
//! a provided deadline at which point payments will no longer occur. Requests that don't change the
//! time unit or asset id and change the rate at a disadvantage for the caller is applied
//! immediately. An existing request can be overritten by both parties if it was a suggestion, while
//! only by the previous requester if it was mandatory. A nonce is increased to prevent to prevent
//! one to frontrunner the acceptation of a request with another request.
//! - `accept_requested_change(stream_id, request_nonce)`: Accept the change for this stream id and
//! request nonce. If one want to refuse a change they can either leave it as is (which will do
//! nothing if the request is a suggestion, or stop payment when reaching the deadline if mandatory)
//! or close the stream with `close_stream`.
//!
//! For UIs the pallet provides the following storages:
//! - `Streams: StreamId => Stream`: stream data indexed by stream id.
//! - `LookupStreamsWithSource: AccountId => StreamId => ()`: allows to list allow the streams with
//! a given source by iterating over all storage keys with the key prefix corresponding to the
//! account.
//! - `LookupStreamsWithTarget: AccountId => StreamId => ()`: same but for the target.
//! Those last 2 storages are solely for UIs to list incoming and outgoing streams. Key prefix is
//! used to reduce the POV cost that would require a single Vec of StreamId.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use {
    core::cmp::min,
    frame_support::{
        dispatch::DispatchErrorWithPostInfo,
        pallet,
        pallet_prelude::*,
        storage::types::{StorageDoubleMap, StorageMap},
        traits::tokens::Balance,
        Blake2_128Concat,
    },
    frame_system::pallet_prelude::*,
    parity_scale_codec::{FullCodec, MaxEncodedLen},
    scale_info::TypeInfo,
    sp_runtime::traits::{AtLeast32BitUnsigned, CheckedAdd, CheckedSub, One, Saturating, Zero},
    sp_std::{fmt::Debug, marker::PhantomData},
};

pub use pallet::*;

/// Type able to provide the current time for given unit.
/// For each unit the returned number should monotonically increase and not
/// overflow.
pub trait TimeProvider<Unit, Number> {
    fn now(unit: &Unit) -> Option<Number>;
}

/// Interactions the pallet needs with assets.
pub trait Assets<AccountId, AssetId, Balance> {
    /// Transfer assets deposited by an account to another account.
    /// Those assets should not be considered deposited in the target account.
    fn transfer_deposit(
        asset_id: AssetId,
        from: &AccountId,
        to: &AccountId,
        amount: Balance,
    ) -> DispatchResult;

    /// Increase the deposit for an account and asset id. Should fail if account doesn't have
    /// enough of that asset. Funds should be safe and not slashable.
    fn increase_deposit(asset_id: AssetId, account: &AccountId, amount: Balance) -> DispatchResult;

    /// Decrease the deposit for an account and asset id. Should fail on underflow.
    fn decrease_deposit(asset_id: AssetId, account: &AccountId, amount: Balance) -> DispatchResult;
}

#[pallet(dev_mode)]
pub mod pallet {
    use super::*;

    /// Pooled Staking pallet.
    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Overarching event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Type used to represent stream ids. Should be large enough to not overflow.
        type StreamId: AtLeast32BitUnsigned
            + Default
            + Debug
            + Copy
            + Clone
            + FullCodec
            + TypeInfo
            + MaxEncodedLen;

        /// The balance type, which is also the type representing time (as this
        /// pallet will do math with both time and balances to compute how
        /// much should be paid).
        type Balance: Balance;

        /// Type representing an asset id, a identifier allowing distinguishing assets.
        type AssetId: Debug + Clone + FullCodec + TypeInfo + MaxEncodedLen + PartialEq + Eq;

        /// Provide interaction with assets.
        type Assets: Assets<Self::AccountId, Self::AssetId, Self::Balance>;

        /// Represents which units of time can be used. Designed to be an enum
        /// with a variant for each kind of time source/scale supported.
        type TimeUnit: Debug + Clone + FullCodec + TypeInfo + MaxEncodedLen + Eq;

        /// Provide the current time in given unit.
        type TimeProvider: TimeProvider<Self::TimeUnit, Self::Balance>;
    }

    type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    type AssetIdOf<T> = <T as Config>::AssetId;

    pub type RequestNonce = u32;

    /// A stream payment from source to target.
    /// Stores the last time the stream was updated, which allows to compute
    /// elapsed time and perform payment.
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    #[derive(RuntimeDebug, PartialEq, Eq, Encode, Decode, Clone, TypeInfo)]
    pub struct Stream<AccountId, Unit, AssetId, Balance> {
        /// Payer, source of the stream.
        pub source: AccountId,
        /// Payee, target of the stream.
        pub target: AccountId,
        /// Steam config (time unit, asset id, rate)
        pub config: StreamConfig<Unit, AssetId, Balance>,
        /// How much is deposited to fund this stream.
        pub deposit: Balance,
        /// Last time the stream was updated in `config.time_unit`.
        pub last_time_updated: Balance,
        /// Nonce for requests. This prevents a request to make a first request
        /// then change it to another request to frontrun the other party
        /// accepting.
        pub request_nonce: RequestNonce,
        /// A pending change request if any.
        pub pending_request: Option<ChangeRequest<Unit, AssetId, Balance>>,
    }

    impl<AccountId: PartialEq, Unit, AssetId, Balance> Stream<AccountId, Unit, AssetId, Balance> {
        pub fn account_to_party(&self, account: AccountId) -> Option<Party> {
            match account {
                a if a == self.source => Some(Party::Source),
                a if a == self.target => Some(Party::Target),
                _ => None,
            }
        }
    }

    /// Stream configuration.
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    #[derive(RuntimeDebug, PartialEq, Eq, Encode, Decode, Copy, Clone, TypeInfo)]
    pub struct StreamConfig<Unit, AssetId, Balance> {
        /// Unit in which time is measured using a `TimeProvider`.
        pub time_unit: Unit,
        /// Asset used for payment.
        pub asset_id: AssetId,
        /// Amount of asset / unit.
        pub rate: Balance,
    }

    /// Origin of a change request.
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    #[derive(RuntimeDebug, PartialEq, Eq, Encode, Decode, Copy, Clone, TypeInfo)]
    pub enum Party {
        Source,
        Target,
    }

    /// Kind of change requested.
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    #[derive(RuntimeDebug, PartialEq, Eq, Encode, Decode, Copy, Clone, TypeInfo)]
    pub enum ChangeKind<Time> {
        /// The requested change is a suggestion, and the other party doesn't
        /// need to accept it.
        Suggestion,
        /// The requested change is mandatory, and the other party must either
        /// accept the change or close the stream. Reaching the deadline will
        /// close the stream too.
        Mandatory { deadline: Time },
    }

    /// A request to change a stream config.
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    #[derive(RuntimeDebug, PartialEq, Eq, Encode, Decode, Clone, TypeInfo)]
    pub struct ChangeRequest<Unit, AssetId, Balance> {
        requester: Party,
        kind: ChangeKind<Balance>,
        new_config: StreamConfig<Unit, AssetId, Balance>,
    }

    pub type StreamOf<T> =
        Stream<AccountIdOf<T>, <T as Config>::TimeUnit, AssetIdOf<T>, <T as Config>::Balance>;

    pub type StreamConfigOf<T> =
        StreamConfig<<T as Config>::TimeUnit, AssetIdOf<T>, <T as Config>::Balance>;

    pub type ChangeRequestOf<T> =
        ChangeRequest<<T as Config>::TimeUnit, AssetIdOf<T>, <T as Config>::Balance>;

    /// Store the next available stream id.
    #[pallet::storage]
    pub type NextStreamId<T: Config> = StorageValue<Value = T::StreamId, QueryKind = ValueQuery>;

    /// Store each stream indexed by an Id.
    #[pallet::storage]
    pub type Streams<T: Config> = StorageMap<
        Hasher = Blake2_128Concat,
        Key = T::StreamId,
        Value = StreamOf<T>,
        QueryKind = OptionQuery,
    >;

    /// Lookup for all streams with given source.
    /// To avoid maintaining a growing list of stream ids, they are stored in
    /// the form of an entry (AccountId, StreamId). If such entry exists then
    /// this AccountId is a source in StreamId. One can iterate over all storage
    /// keys starting with the AccountId to find all StreamIds.
    #[pallet::storage]
    pub type LookupStreamsWithSource<T: Config> = StorageDoubleMap<
        Key1 = AccountIdOf<T>,
        Hasher1 = Blake2_128Concat,
        Key2 = T::StreamId,
        Hasher2 = Blake2_128Concat,
        Value = (),
        QueryKind = OptionQuery,
    >;

    /// Lookup for all streams with given target.
    /// To avoid maintaining a growing list of stream ids, they are stored in
    /// the form of an entry (AccountId, StreamId). If such entry exists then
    /// this AccountId is a target in StreamId. One can iterate over all storage
    /// keys starting with the AccountId to find all StreamIds.
    #[pallet::storage]
    pub type LookupStreamsWithTarget<T: Config> = StorageDoubleMap<
        Key1 = AccountIdOf<T>,
        Hasher1 = Blake2_128Concat,
        Key2 = T::StreamId,
        Hasher2 = Blake2_128Concat,
        Value = (),
        QueryKind = OptionQuery,
    >;

    #[pallet::error]
    pub enum Error<T> {
        UnknownStreamId,
        StreamIdOverflow,
        UnauthorizedOrigin,
        CantBeBothSourceAndTarget,
        CantFetchCurrentTime,
        TimeMustBeIncreasing,
        CurrencyOverflow,
        SourceCantDecreaseRate,
        TargetCantIncreaseRate,
        CantOverrideMandatoryChange,
        NoPendingRequest,
        CantAcceptOwnRequest,
        WrongRequestNonce,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        StreamOpened {
            stream_id: T::StreamId,
        },
        StreamClosed {
            stream_id: T::StreamId,
            refunded: T::Balance,
        },
        StreamPayment {
            stream_id: T::StreamId,
            source: AccountIdOf<T>,
            target: AccountIdOf<T>,
            amount: T::Balance,
            drained: bool,
        },
        StreamRefilled {
            stream_id: T::StreamId,
            increase: T::Balance,
            new_deposit: T::Balance,
        },
        StreamConfigChanged {
            stream_id: T::StreamId,
            old_config: StreamConfigOf<T>,
            new_config: StreamConfigOf<T>,
        },
        StreamConfigChangeRequested {
            stream_id: T::StreamId,
            request_nonce: RequestNonce,
            old_config: StreamConfigOf<T>,
            new_config: StreamConfigOf<T>,
        },
    }

    /// Freeze reason to use if needed.
    #[pallet::composite_enum]
    pub enum FreezeReason {
        StreamPayment,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        pub fn open_stream(
            origin: OriginFor<T>,
            target: AccountIdOf<T>,
            config: StreamConfigOf<T>,
            initial_deposit: T::Balance,
        ) -> DispatchResultWithPostInfo {
            let origin = ensure_signed(origin)?;

            let _stream_id = Self::open_stream_returns_id(origin, target, config, initial_deposit)?;

            Ok(().into())
        }

        #[pallet::call_index(1)]
        pub fn close_stream(
            origin: OriginFor<T>,
            stream_id: T::StreamId,
        ) -> DispatchResultWithPostInfo {
            let origin = ensure_signed(origin)?;
            let mut stream = Streams::<T>::get(stream_id).ok_or(Error::<T>::UnknownStreamId)?;

            // Only source or target can close a stream.
            ensure!(
                origin == stream.source || origin == stream.target,
                Error::<T>::UnauthorizedOrigin
            );

            // Update stream before closing it to ensure fair payment.
            Self::perform_stream_payment(stream_id, &mut stream)?;

            // Unfreeze funds left in the stream.
            T::Assets::decrease_deposit(
                stream.config.asset_id.clone(),
                &stream.source,
                stream.deposit,
            )?;

            // Remove stream from storage.
            Streams::<T>::remove(stream_id);
            LookupStreamsWithSource::<T>::remove(stream.source, stream_id);
            LookupStreamsWithTarget::<T>::remove(stream.target, stream_id);

            // Emit event.
            Pallet::<T>::deposit_event(Event::<T>::StreamClosed {
                stream_id,
                refunded: stream.deposit,
            });

            Ok(().into())
        }

        #[pallet::call_index(2)]
        pub fn update_stream(
            origin: OriginFor<T>,
            stream_id: T::StreamId,
        ) -> DispatchResultWithPostInfo {
            // No problem with anyone updating any stream.
            let _ = ensure_signed(origin)?;

            let mut stream = Streams::<T>::get(stream_id).ok_or(Error::<T>::UnknownStreamId)?;
            Self::perform_stream_payment(stream_id, &mut stream)?;
            Streams::<T>::insert(stream_id, stream);

            Ok(().into())
        }

        #[pallet::call_index(3)]
        pub fn refill_stream(
            origin: OriginFor<T>,
            stream_id: T::StreamId,
            increase: T::Balance,
        ) -> DispatchResultWithPostInfo {
            let origin = ensure_signed(origin)?;
            let mut stream = Streams::<T>::get(stream_id).ok_or(Error::<T>::UnknownStreamId)?;

            // Only source can refill stream
            ensure!(origin == stream.source, Error::<T>::UnauthorizedOrigin);

            // Source will not pay for drained stream retroactively, so we perform payment with
            // what is left first.
            Self::perform_stream_payment(stream_id, &mut stream)?;

            // Increase deposit.
            T::Assets::increase_deposit(stream.config.asset_id.clone(), &origin, increase)?;
            stream.deposit = stream
                .deposit
                .checked_add(&increase)
                .ok_or(Error::<T>::CurrencyOverflow)?;

            // Emit event.
            Pallet::<T>::deposit_event(Event::<T>::StreamRefilled {
                stream_id,
                increase,
                new_deposit: stream.deposit,
            });

            // Update stream info in storage.
            Streams::<T>::insert(stream_id, stream);

            Ok(().into())
        }

        #[pallet::call_index(4)]
        pub fn request_change(
            origin: OriginFor<T>,
            stream_id: T::StreamId,
            kind: ChangeKind<T::Balance>,
            new_config: StreamConfigOf<T>,
        ) -> DispatchResultWithPostInfo {
            let origin = ensure_signed(origin)?;
            let mut stream = Streams::<T>::get(stream_id).ok_or(Error::<T>::UnknownStreamId)?;

            let requester = stream
                .account_to_party(origin)
                .ok_or(Error::<T>::UnauthorizedOrigin)?;

            if stream.config == new_config {
                return Ok(().into());
            }

            // If asset id and time unit are the same, we allow to make the change
            // immediatly if the origin is at a disadvantage.
            'immediate: {
                if new_config.time_unit != stream.config.time_unit
                    || new_config.asset_id != stream.config.asset_id
                {
                    break 'immediate;
                }

                if requester == Party::Source && new_config.rate < stream.config.rate {
                    break 'immediate;
                }

                if requester == Party::Target && new_config.rate > stream.config.rate {
                    break 'immediate;
                }

                // Perform pending payment before changing config.
                Self::perform_stream_payment(stream_id, &mut stream)?;

                // Emit event.
                Pallet::<T>::deposit_event(Event::<T>::StreamConfigChanged {
                    stream_id,
                    old_config: stream.config.clone(),
                    new_config: new_config.clone(),
                });

                // Update storage.
                stream.config = new_config.clone();
                Streams::<T>::insert(stream_id, stream);

                return Ok(().into());
            }

            // If there is already a mandatory change request, only the origin
            // of this request can change it.
            if let Some(ChangeRequest {
                kind: ChangeKind::Mandatory { .. },
                requester: pending_requester,
                ..
            }) = &stream.pending_request
            {
                ensure!(
                    &requester == pending_requester,
                    Error::<T>::CantOverrideMandatoryChange
                );
            }

            stream.request_nonce = stream.request_nonce.wrapping_add(1);
            stream.pending_request = Some(ChangeRequest {
                requester,
                kind,
                new_config: new_config.clone(),
            });

            // Emit event.
            Pallet::<T>::deposit_event(Event::<T>::StreamConfigChangeRequested {
                stream_id,
                request_nonce: stream.request_nonce,
                old_config: stream.config.clone(),
                new_config,
            });

            // Update storage.
            Streams::<T>::insert(stream_id, stream);

            Ok(().into())
        }

        #[pallet::call_index(5)]
        pub fn accept_requested_change(
            origin: OriginFor<T>,
            stream_id: T::StreamId,
            request_nonce: RequestNonce,
        ) -> DispatchResultWithPostInfo {
            let origin = ensure_signed(origin)?;
            let mut stream = Streams::<T>::get(stream_id).ok_or(Error::<T>::UnknownStreamId)?;

            let accepter = stream
                .account_to_party(origin)
                .ok_or(Error::<T>::UnauthorizedOrigin)?;

            let Some(request) = stream.pending_request.take() else {
                return Err(Error::<T>::NoPendingRequest.into());
            };

            ensure!(
                request_nonce == stream.request_nonce,
                Error::<T>::WrongRequestNonce
            );
            ensure!(
                accepter != request.requester,
                Error::<T>::CantAcceptOwnRequest
            );

            // Perform pending payment before changing config.
            Self::perform_stream_payment(stream_id, &mut stream)?;

            todo!()
        }
    }

    impl<T: Config> Pallet<T> {
        /// Try to open a stream and returns its id.
        /// Prefers calling this function from other pallets instead of `open_stream` as the
        /// latter can't return the id.
        pub fn open_stream_returns_id(
            origin: AccountIdOf<T>,
            target: AccountIdOf<T>,
            config: StreamConfigOf<T>,
            initial_deposit: T::Balance,
        ) -> Result<T::StreamId, DispatchErrorWithPostInfo> {
            ensure!(origin != target, Error::<T>::CantBeBothSourceAndTarget);

            // Generate a new stream id.
            let stream_id = NextStreamId::<T>::get();
            let next_stream_id = stream_id
                .checked_add(&One::one())
                .ok_or(Error::<T>::StreamIdOverflow)?;
            NextStreamId::<T>::set(next_stream_id);

            // Freeze initial deposit.
            T::Assets::increase_deposit(config.asset_id.clone(), &origin, initial_deposit)?;

            // Create stream data.
            let now =
                T::TimeProvider::now(&config.time_unit).ok_or(Error::<T>::CantFetchCurrentTime)?;
            let stream = Stream {
                source: origin.clone(),
                target: target.clone(),
                config,
                deposit: initial_deposit,
                last_time_updated: now,
                request_nonce: 0,
                pending_request: None,
            };

            // Insert stream in storage.
            Streams::<T>::insert(stream_id, stream);
            LookupStreamsWithSource::<T>::insert(origin, stream_id, ());
            LookupStreamsWithTarget::<T>::insert(target, stream_id, ());

            // Emit event.
            Pallet::<T>::deposit_event(Event::<T>::StreamOpened { stream_id });

            Ok(stream_id)
        }

        /// Behavior:
        /// A stream payment consist of a locked deposit, a rate per unit of time and the
        /// last time the stream was updated. When updating the stream, **at most**
        /// `elapsed_time * rate` is unlocked from the source account and transfered to the target
        /// account. If this amount is greater than the left deposit, the stream is considered
        /// drained **but not closed**. The source can come back later and refill the stream,
        /// however there will be no retroactive payment for the time spent as drained.
        /// If the stream payment is used to rent a service, the target should pause the service
        /// while the stream is drained, and resume it once it is refilled.
        fn perform_stream_payment(
            stream_id: T::StreamId,
            stream: &mut StreamOf<T>,
        ) -> DispatchResultWithPostInfo {
            let mut now = T::TimeProvider::now(&stream.config.time_unit)
                .ok_or(Error::<T>::CantFetchCurrentTime)?;

            // We want to uupdate `stream.last_time_updated` to `now` as soon
            // as possible to avoid forgetting to do it. We copy the old value
            // for payment computation.
            let last_time_updated = stream.last_time_updated;
            stream.last_time_updated = now;

            // Take into account mandatory change request deadline. Note that
            // while it'll perform payment up to deadline,
            // `stream.last_time_updated` is still the "real now" to avoid
            // retroactive payment in case the deadline changes.
            if let Some(ChangeRequest {
                kind: ChangeKind::Mandatory { deadline },
                ..
            }) = &stream.pending_request
            {
                now = min(now, *deadline);
            }

            // Dont perform payment
            if now == last_time_updated {
                return Ok(().into());
            }

            // If deposit is zero the stream is fully drained and there is nothing to transfer.
            if stream.deposit.is_zero() {
                return Ok(().into());
            }

            let delta = now
                .checked_sub(&last_time_updated)
                .ok_or(Error::<T>::TimeMustBeIncreasing)?;

            // We compute the amount due to the target according to the rate, which may be
            // lowered if the stream deposit is lower.
            // Saturating is fine as it'll be clamped to the source deposit. It is also safer as
            // considering it an error can make a stream un-updatable if too much time has passed
            // without updates.
            let mut payment = delta.saturating_mul(stream.config.rate);

            // We compute the new amount of locked funds. If it underflows it
            // means that there is more to pay that what is left, in which case
            // we pay all that is left.
            let (new_locked, drained) = match stream.deposit.checked_sub(&payment) {
                Some(v) if v.is_zero() => (v, true),
                Some(v) => (v, false),
                None => {
                    payment = stream.deposit;
                    (Zero::zero(), true)
                }
            };

            if payment.is_zero() {
                return Ok(().into());
            }

            // Transfer from the source to target.
            T::Assets::transfer_deposit(
                stream.config.asset_id.clone(),
                &stream.source,
                &stream.target,
                payment,
            )?;

            // Update stream info.
            stream.deposit = new_locked;

            // Emit event.
            Pallet::<T>::deposit_event(Event::<T>::StreamPayment {
                stream_id,
                source: stream.source.clone(),
                target: stream.target.clone(),
                amount: payment,
                drained,
            });

            Ok(().into())
        }
    }
}
