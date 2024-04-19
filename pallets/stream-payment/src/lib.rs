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

#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use weights::WeightInfo;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use {
    core::cmp::min,
    frame_support::{
        dispatch::DispatchErrorWithPostInfo,
        pallet,
        pallet_prelude::*,
        storage::types::{StorageDoubleMap, StorageMap},
        traits::{
            fungible::{Inspect, MutateHold},
            tokens::{Balance, Precision},
        },
        Blake2_128Concat,
    },
    frame_system::pallet_prelude::*,
    parity_scale_codec::{FullCodec, MaxEncodedLen},
    scale_info::TypeInfo,
    sp_runtime::{
        traits::{AtLeast32BitUnsigned, CheckedAdd, CheckedSub, One, Saturating, Zero},
        ArithmeticError,
    },
    sp_std::{fmt::Debug, marker::PhantomData},
};

pub use pallet::*;

/// Type able to provide the current time for given unit.
/// For each unit the returned number should monotonically increase and not
/// overflow.
pub trait TimeProvider<Unit, Number> {
    fn now(unit: &Unit) -> Option<Number>;

    /// Benchmarks: should return the time unit which has the worst performance calling
    /// `TimeProvider::now(unit)` with.
    #[cfg(feature = "runtime-benchmarks")]
    fn bench_worst_case_time_unit() -> Unit;

    /// Benchmarks: sets the "now" time for time unit returned by `bench_worst_case_time_unit`.
    #[cfg(feature = "runtime-benchmarks")]
    fn bench_set_now(instant: Number);
}

/// Interactions the pallet needs with assets.
pub trait Assets<AccountId, AssetId, Balance> {
    /// Transfer assets deposited by an account to another account.
    /// Those assets should not be considered deposited in the target account.
    fn transfer_deposit(
        asset_id: &AssetId,
        from: &AccountId,
        to: &AccountId,
        amount: Balance,
    ) -> DispatchResult;

    /// Increase the deposit for an account and asset id. Should fail if account doesn't have
    /// enough of that asset. Funds should be safe and not slashable.
    fn increase_deposit(asset_id: &AssetId, account: &AccountId, amount: Balance)
        -> DispatchResult;

    /// Decrease the deposit for an account and asset id. Should fail on underflow.
    fn decrease_deposit(asset_id: &AssetId, account: &AccountId, amount: Balance)
        -> DispatchResult;

    /// Return the deposit for given asset and account.
    fn get_deposit(asset_id: &AssetId, account: &AccountId) -> Balance;

    /// Benchmarks: should return the asset id which has the worst performance when interacting
    /// with it.
    #[cfg(feature = "runtime-benchmarks")]
    fn bench_worst_case_asset_id() -> AssetId;

    /// Benchmarks: should return the another asset id which has the worst performance when interacting
    /// with it afther `bench_worst_case_asset_id`. This is to benchmark the worst case when changing config
    /// from one asset to another. If there is only one asset id it is fine to return it in both
    /// `bench_worst_case_asset_id` and `bench_worst_case_asset_id2`.
    #[cfg(feature = "runtime-benchmarks")]
    fn bench_worst_case_asset_id2() -> AssetId;

    /// Benchmarks: should set the balance.
    #[cfg(feature = "runtime-benchmarks")]
    fn bench_set_balance(asset_id: &AssetId, account: &AccountId, amount: Balance);
}

#[pallet]
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

        /// Currency for the opening balance hold for the storage used by the Stream.
        /// NOT to be confused with Assets.
        type Currency: Inspect<Self::AccountId, Balance = Self::Balance>
            + MutateHold<Self::AccountId, Reason = Self::RuntimeHoldReason>;

        type RuntimeHoldReason: From<HoldReason>;

        #[pallet::constant]
        type OpenStreamHoldAmount: Get<Self::Balance>;

        /// Represents which units of time can be used. Designed to be an enum
        /// with a variant for each kind of time source/scale supported.
        type TimeUnit: Debug + Clone + FullCodec + TypeInfo + MaxEncodedLen + Eq;

        /// Provide the current time in given unit.
        type TimeProvider: TimeProvider<Self::TimeUnit, Self::Balance>;

        type WeightInfo: weights::WeightInfo;
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
        /// One-time opening deposit. Will be released on close.
        pub opening_deposit: Balance,
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

    impl Party {
        pub fn inverse(self) -> Self {
            match self {
                Party::Source => Party::Target,
                Party::Target => Party::Source,
            }
        }
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

    /// Describe how the deposit should change.
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    #[derive(RuntimeDebug, PartialEq, Eq, Encode, Decode, Copy, Clone, TypeInfo)]
    pub enum DepositChange<Balance> {
        /// Increase deposit by given amount.
        Increase(Balance),
        /// Decrease deposit by given amount.
        Decrease(Balance),
        /// Set deposit to given amount.
        Absolute(Balance),
    }

    /// A request to change a stream config.
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    #[derive(RuntimeDebug, PartialEq, Eq, Encode, Decode, Clone, TypeInfo)]
    pub struct ChangeRequest<Unit, AssetId, Balance> {
        pub requester: Party,
        pub kind: ChangeKind<Balance>,
        pub new_config: StreamConfig<Unit, AssetId, Balance>,
        pub deposit_change: Option<DepositChange<Balance>>,
    }

    pub type StreamOf<T> =
        Stream<AccountIdOf<T>, <T as Config>::TimeUnit, AssetIdOf<T>, <T as Config>::Balance>;

    pub type StreamConfigOf<T> =
        StreamConfig<<T as Config>::TimeUnit, AssetIdOf<T>, <T as Config>::Balance>;

    pub type ChangeRequestOf<T> =
        ChangeRequest<<T as Config>::TimeUnit, AssetIdOf<T>, <T as Config>::Balance>;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct StreamPaymentStatus<Balance> {
        pub payment: Balance,
        pub deposit_left: Balance,
        /// Whenever the stream is stalled, which can occur either when no funds are left or
        /// if the time is past a mandatory request deadline.
        pub stalled: bool,
    }

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
    #[derive(Clone, PartialEq, Eq)]
    pub enum Error<T> {
        UnknownStreamId,
        StreamIdOverflow,
        UnauthorizedOrigin,
        CantBeBothSourceAndTarget,
        CantFetchCurrentTime,
        SourceCantDecreaseRate,
        TargetCantIncreaseRate,
        CantOverrideMandatoryChange,
        NoPendingRequest,
        CantAcceptOwnRequest,
        CanOnlyCancelOwnRequest,
        WrongRequestNonce,
        ChangingAssetRequiresAbsoluteDepositChange,
        TargetCantChangeDeposit,
        ImmediateDepositChangeRequiresSameAssetId,
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
            stalled: bool,
        },
        StreamConfigChangeRequested {
            stream_id: T::StreamId,
            request_nonce: RequestNonce,
            requester: Party,
            old_config: StreamConfigOf<T>,
            new_config: StreamConfigOf<T>,
        },
        StreamConfigChanged {
            stream_id: T::StreamId,
            old_config: StreamConfigOf<T>,
            new_config: StreamConfigOf<T>,
            deposit_change: Option<DepositChange<T::Balance>>,
        },
    }

    /// Freeze reason to use if needed.
    #[pallet::composite_enum]
    pub enum FreezeReason {
        StreamPayment,
    }

    /// Hold reason to use if needed.
    #[pallet::composite_enum]
    pub enum HoldReason {
        StreamPayment,
        StreamOpened,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a payment stream from the origin to the target with provided config
        /// and initial deposit (in the asset defined in the config).
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::open_stream())]
        pub fn open_stream(
            origin: OriginFor<T>,
            target: AccountIdOf<T>,
            config: StreamConfigOf<T>,
            initial_deposit: T::Balance,
        ) -> DispatchResultWithPostInfo {
            let origin = ensure_signed(origin)?;
            let opening_deposit = T::OpenStreamHoldAmount::get();

            let _stream_id = Self::open_stream_returns_id(
                origin,
                target,
                config,
                initial_deposit,
                opening_deposit,
            )?;

            Ok(().into())
        }

        /// Close a given stream in which the origin is involved. It performs the pending payment
        /// before closing the stream.
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::close_stream())]
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
            T::Assets::decrease_deposit(&stream.config.asset_id, &stream.source, stream.deposit)?;

            // Release opening deposit
            if stream.opening_deposit > 0u32.into() {
                T::Currency::release(
                    &HoldReason::StreamOpened.into(),
                    &stream.source,
                    stream.opening_deposit,
                    Precision::Exact,
                )?;
            }

            // Remove stream from storage.
            Streams::<T>::remove(stream_id);
            LookupStreamsWithSource::<T>::remove(stream.source, stream_id);
            LookupStreamsWithTarget::<T>::remove(stream.target, stream_id);

            // Emit event.
            Pallet::<T>::deposit_event(Event::<T>::StreamClosed {
                stream_id,
                refunded: stream.deposit.saturating_add(stream.opening_deposit),
            });

            Ok(().into())
        }

        /// Perform the pending payment of a stream. Anyone can call this.
        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::perform_payment())]
        pub fn perform_payment(
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

        /// Requests a change to a stream config or deposit.
        ///
        /// If the new config don't change the time unit and asset id, the change will be applied
        /// immediately if it is at the desadvantage of the caller. Otherwise, the request is stored
        /// in the stream and will have to be approved by the other party.
        ///
        /// This call accepts a deposit change, which can only be provided by the source of the
        /// stream. An absolute change is required when changing asset id, as the current deposit
        /// will be released and a new deposit is required in the new asset.
        #[pallet::call_index(3)]
        #[pallet::weight(
            T::WeightInfo::request_change_immediate()
            .max(T::WeightInfo::request_change_delayed())
        )]
        pub fn request_change(
            origin: OriginFor<T>,
            stream_id: T::StreamId,
            kind: ChangeKind<T::Balance>,
            new_config: StreamConfigOf<T>,
            deposit_change: Option<DepositChange<T::Balance>>,
        ) -> DispatchResultWithPostInfo {
            let origin = ensure_signed(origin)?;
            let mut stream = Streams::<T>::get(stream_id).ok_or(Error::<T>::UnknownStreamId)?;

            let requester = stream
                .account_to_party(origin)
                .ok_or(Error::<T>::UnauthorizedOrigin)?;

            ensure!(
                requester == Party::Source || deposit_change.is_none(),
                Error::<T>::TargetCantChangeDeposit
            );

            if stream.config == new_config && deposit_change.is_none() {
                return Ok(().into());
            }

            // If asset id and time unit are the same, we allow to make the change
            // immediatly if the origin is at a disadvantage.
            // We allow this even if there is already a pending request.
            if Self::maybe_immediate_change(
                stream_id,
                &mut stream,
                &new_config,
                deposit_change,
                requester,
            )? {
                return Ok(().into());
            }

            // If the source is requesting a change of asset, they must provide an absolute change.
            if requester == Party::Source
                && new_config.asset_id != stream.config.asset_id
                && !matches!(deposit_change, Some(DepositChange::Absolute(_)))
            {
                Err(Error::<T>::ChangingAssetRequiresAbsoluteDepositChange)?;
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
                deposit_change,
            });

            // Emit event.
            Pallet::<T>::deposit_event(Event::<T>::StreamConfigChangeRequested {
                stream_id,
                request_nonce: stream.request_nonce,
                requester,
                old_config: stream.config.clone(),
                new_config,
            });

            // Update storage.
            Streams::<T>::insert(stream_id, stream);

            Ok(().into())
        }

        /// Accepts a change requested before by the other party. Takes a nonce to prevent
        /// frontrunning attacks. If the target made a request, the source is able to change their
        /// deposit.
        #[pallet::call_index(4)]
        #[pallet::weight(T::WeightInfo::accept_requested_change())]
        pub fn accept_requested_change(
            origin: OriginFor<T>,
            stream_id: T::StreamId,
            request_nonce: RequestNonce,
            deposit_change: Option<DepositChange<T::Balance>>,
        ) -> DispatchResultWithPostInfo {
            let origin = ensure_signed(origin)?;
            let mut stream = Streams::<T>::get(stream_id).ok_or(Error::<T>::UnknownStreamId)?;

            let accepter = stream
                .account_to_party(origin)
                .ok_or(Error::<T>::UnauthorizedOrigin)?;

            let Some(request) = stream.pending_request.clone() else {
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

            ensure!(
                accepter == Party::Source || deposit_change.is_none(),
                Error::<T>::TargetCantChangeDeposit
            );

            // Perform pending payment before changing config.
            Self::perform_stream_payment(stream_id, &mut stream)?;

            // Apply change.
            let deposit_change = deposit_change.or(request.deposit_change);
            match (
                stream.config.asset_id == request.new_config.asset_id,
                deposit_change,
            ) {
                // Same asset and a change, we apply it like in `change_deposit` call.
                (true, Some(change)) => {
                    Self::apply_deposit_change(&mut stream, change)?;
                }
                // Same asset and no change, no problem.
                (true, None) => (),
                // Change in asset with absolute new amount
                (false, Some(DepositChange::Absolute(amount))) => {
                    // Release deposit in old asset.
                    T::Assets::decrease_deposit(
                        &stream.config.asset_id,
                        &stream.source,
                        stream.deposit,
                    )?;

                    // Make deposit in new asset.
                    T::Assets::increase_deposit(
                        &request.new_config.asset_id,
                        &stream.source,
                        amount,
                    )?;
                    stream.deposit = amount;
                }
                // It doesn't make sense to change asset while not providing an absolute new
                // amount.
                (false, _) => Err(Error::<T>::ChangingAssetRequiresAbsoluteDepositChange)?,
            }

            // If time unit changes we need to update `last_time_updated` to be in the
            // new unit.
            if stream.config.time_unit != request.new_config.time_unit {
                stream.last_time_updated = T::TimeProvider::now(&request.new_config.time_unit)
                    .ok_or(Error::<T>::CantFetchCurrentTime)?;
            }

            // Event
            Pallet::<T>::deposit_event(Event::<T>::StreamConfigChanged {
                stream_id,
                old_config: stream.config,
                new_config: request.new_config.clone(),
                deposit_change,
            });

            // Update config in storage.
            stream.config = request.new_config;
            stream.pending_request = None;
            Streams::<T>::insert(stream_id, stream);

            Ok(().into())
        }

        #[pallet::call_index(5)]
        #[pallet::weight(T::WeightInfo::cancel_change_request())]
        pub fn cancel_change_request(
            origin: OriginFor<T>,
            stream_id: T::StreamId,
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
                accepter == request.requester,
                Error::<T>::CanOnlyCancelOwnRequest
            );

            // Update storage.
            // Pending request is removed by calling `.take()`.
            Streams::<T>::insert(stream_id, stream);

            Ok(().into())
        }

        /// Allows immediately changing the deposit for a stream, which is simpler than
        /// calling `request_change` with the proper parameters.
        /// The call takes an asset id to ensure it has not changed (by an accepted request) before
        /// the call is included in a block, in which case the unit is no longer the same and quantities
        /// will not have the same scale/value.
        #[pallet::call_index(6)]
        #[pallet::weight(T::WeightInfo::immediately_change_deposit())]
        pub fn immediately_change_deposit(
            origin: OriginFor<T>,
            stream_id: T::StreamId,
            asset_id: T::AssetId,
            change: DepositChange<T::Balance>,
        ) -> DispatchResultWithPostInfo {
            let origin = ensure_signed(origin)?;
            let mut stream = Streams::<T>::get(stream_id).ok_or(Error::<T>::UnknownStreamId)?;

            ensure!(stream.source == origin, Error::<T>::UnauthorizedOrigin);
            ensure!(
                stream.config.asset_id == asset_id,
                Error::<T>::ImmediateDepositChangeRequiresSameAssetId
            );

            // Perform pending payment before changing deposit.
            Self::perform_stream_payment(stream_id, &mut stream)?;

            // Apply change.
            Self::apply_deposit_change(&mut stream, change)?;

            // Event
            Pallet::<T>::deposit_event(Event::<T>::StreamConfigChanged {
                stream_id,
                old_config: stream.config.clone(),
                new_config: stream.config.clone(),
                deposit_change: Some(change),
            });

            // Update stream in storage.
            Streams::<T>::insert(stream_id, stream);

            Ok(().into())
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
            opening_deposit: T::Balance,
        ) -> Result<T::StreamId, DispatchErrorWithPostInfo> {
            ensure!(origin != target, Error::<T>::CantBeBothSourceAndTarget);

            // Generate a new stream id.
            let stream_id = NextStreamId::<T>::get();
            let next_stream_id = stream_id
                .checked_add(&One::one())
                .ok_or(Error::<T>::StreamIdOverflow)?;
            NextStreamId::<T>::set(next_stream_id);

            // Hold opening deposit for the storage used by Stream
            if opening_deposit > 0u32.into() {
                T::Currency::hold(&HoldReason::StreamOpened.into(), &origin, opening_deposit)?;
            }

            // Freeze initial deposit.
            T::Assets::increase_deposit(&config.asset_id, &origin, initial_deposit)?;

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
                opening_deposit,
            };

            // Insert stream in storage.
            Streams::<T>::insert(stream_id, stream);
            LookupStreamsWithSource::<T>::insert(origin, stream_id, ());
            LookupStreamsWithTarget::<T>::insert(target, stream_id, ());

            // Emit event.
            Pallet::<T>::deposit_event(Event::<T>::StreamOpened { stream_id });

            Ok(stream_id)
        }

        /// Get the stream payment current status, telling how much payment is
        /// pending, how much deposit will be left and whenever the stream is stalled.
        /// The stream is considered stalled if no funds are left or if the provided
        /// time is past a mandatory request deadline. If the provided `now` is `None`
        /// then the current time will be fetched. Being able to provide a custom `now`
        /// allows to check the status in the future.
        pub fn stream_payment_status(
            stream_id: T::StreamId,
            now: Option<T::Balance>,
        ) -> Result<StreamPaymentStatus<T::Balance>, Error<T>> {
            let stream = Streams::<T>::get(stream_id).ok_or(Error::<T>::UnknownStreamId)?;
            let now = match now {
                Some(v) => v,
                None => T::TimeProvider::now(&stream.config.time_unit)
                    .ok_or(Error::<T>::CantFetchCurrentTime)?,
            };

            let last_time_updated = stream.last_time_updated;
            Self::stream_payment_status_by_ref(&stream, last_time_updated, now)
        }

        fn stream_payment_status_by_ref(
            stream: &StreamOf<T>,
            last_time_updated: T::Balance,
            mut now: T::Balance,
        ) -> Result<StreamPaymentStatus<T::Balance>, Error<T>> {
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

            // If deposit is zero the stream is fully drained and there is nothing to transfer.
            if stream.deposit.is_zero() {
                return Ok(StreamPaymentStatus {
                    payment: 0u32.into(),
                    deposit_left: stream.deposit,
                    stalled: true,
                });
            }

            // Dont perform payment if now is before or equal to `last_time_updated`.
            // It can be before due to the deadline adjustment.
            let Some(delta) = now.checked_sub(&last_time_updated) else {
                return Ok(StreamPaymentStatus {
                    payment: 0u32.into(),
                    deposit_left: stream.deposit,
                    stalled: true,
                });
            };

            // We compute the amount due to the target according to the rate, which may be
            // lowered if the stream deposit is lower.
            // Saturating is fine as it'll be clamped to the source deposit. It is also safer as
            // considering it an error can make a stream un-updatable if too much time has passed
            // without updates.
            let mut payment = delta.saturating_mul(stream.config.rate);

            // We compute the new amount of locked funds. If it underflows it
            // means that there is more to pay that what is left, in which case
            // we pay all that is left.
            let (deposit_left, stalled) = match stream.deposit.checked_sub(&payment) {
                Some(v) if v.is_zero() => (v, true),
                Some(v) => (v, false),
                None => {
                    payment = stream.deposit;
                    (Zero::zero(), true)
                }
            };

            Ok(StreamPaymentStatus {
                payment,
                deposit_left,
                stalled,
            })
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
        ) -> Result<T::Balance, DispatchErrorWithPostInfo> {
            let now = T::TimeProvider::now(&stream.config.time_unit)
                .ok_or(Error::<T>::CantFetchCurrentTime)?;

            // We want to update `stream.last_time_updated` to `now` as soon
            // as possible to avoid forgetting to do it. We copy the old value
            // for payment computation.
            let last_time_updated = stream.last_time_updated;
            stream.last_time_updated = now;

            let StreamPaymentStatus {
                payment,
                deposit_left,
                stalled,
            } = Self::stream_payment_status_by_ref(stream, last_time_updated, now)?;

            if payment.is_zero() {
                return Ok(0u32.into());
            }

            // Transfer from the source to target.
            T::Assets::transfer_deposit(
                &stream.config.asset_id,
                &stream.source,
                &stream.target,
                payment,
            )?;

            // Update stream info.
            stream.deposit = deposit_left;

            // Emit event.
            Pallet::<T>::deposit_event(Event::<T>::StreamPayment {
                stream_id,
                source: stream.source.clone(),
                target: stream.target.clone(),
                amount: payment,
                stalled,
            });

            Ok(payment)
        }

        fn apply_deposit_change(
            stream: &mut StreamOf<T>,
            change: DepositChange<T::Balance>,
        ) -> DispatchResultWithPostInfo {
            match change {
                DepositChange::Absolute(amount) => {
                    if let Some(increase) = amount.checked_sub(&stream.deposit) {
                        T::Assets::increase_deposit(
                            &stream.config.asset_id,
                            &stream.source,
                            increase,
                        )?;
                    } else if let Some(decrease) = stream.deposit.checked_sub(&amount) {
                        T::Assets::decrease_deposit(
                            &stream.config.asset_id,
                            &stream.source,
                            decrease,
                        )?;
                    }
                    stream.deposit = amount;
                }
                DepositChange::Increase(increase) => {
                    stream.deposit = stream
                        .deposit
                        .checked_add(&increase)
                        .ok_or(ArithmeticError::Overflow)?;
                    T::Assets::increase_deposit(&stream.config.asset_id, &stream.source, increase)?;
                }
                DepositChange::Decrease(decrease) => {
                    stream.deposit = stream
                        .deposit
                        .checked_sub(&decrease)
                        .ok_or(ArithmeticError::Underflow)?;
                    T::Assets::decrease_deposit(&stream.config.asset_id, &stream.source, decrease)?;
                }
            }

            Ok(().into())
        }

        /// Tries to apply a possibly immediate change. Return if that change was immediate and
        /// applied or not.
        ///
        /// If asset id and time unit are the same, we allow to make the change
        /// immediatly if the origin is at a disadvantage.
        /// We allow this even if there is already a pending request.
        fn maybe_immediate_change(
            stream_id: T::StreamId,
            stream: &mut StreamOf<T>,
            new_config: &StreamConfigOf<T>,
            deposit_change: Option<DepositChange<T::Balance>>,
            requester: Party,
        ) -> Result<bool, DispatchErrorWithPostInfo> {
            if new_config.time_unit != stream.config.time_unit
                || new_config.asset_id != stream.config.asset_id
            {
                return Ok(false);
            }

            if requester == Party::Source && new_config.rate < stream.config.rate {
                return Ok(false);
            }

            if requester == Party::Target && new_config.rate > stream.config.rate {
                return Ok(false);
            }

            // Perform pending payment before changing config.
            Self::perform_stream_payment(stream_id, stream)?;

            // We apply the requested deposit change.
            if let Some(change) = deposit_change {
                Self::apply_deposit_change(stream, change)?;
            }

            // Emit event.
            Pallet::<T>::deposit_event(Event::<T>::StreamConfigChanged {
                stream_id,
                old_config: stream.config.clone(),
                new_config: new_config.clone(),
                deposit_change,
            });

            // Update storage.
            stream.config = new_config.clone();
            Streams::<T>::insert(stream_id, stream);

            Ok(true)
        }
    }
}
