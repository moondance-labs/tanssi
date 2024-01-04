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

#![cfg_attr(not(feature = "std"), no_std)]

use {
    frame_support::{
        pallet,
        pallet_prelude::*,
        storage::types::{StorageDoubleMap, StorageMap},
        traits::{
            fungibles::{self, Mutate as _, MutateFreeze as _},
            tokens::{Balance, Preservation},
        },
        Blake2_128Concat,
    },
    frame_system::pallet_prelude::*,
    parity_scale_codec::{FullCodec, MaxEncodedLen},
    scale_info::TypeInfo,
    sp_runtime::traits::{AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, One, Zero},
    sp_std::{fmt::Debug, marker::PhantomData},
};

/// Type able to provide the current time for given unit.
/// For each unit the returned number should monotonically increase and not
/// overflow.
pub trait TimeProvider<Unit, Number> {
    fn now(unit: &Unit) -> Option<Number>;
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
        /// Type used to represent stream ids. Should be large enough to not overflow.
        type StreamId: AtLeast32BitUnsigned
            + Default
            + Debug
            + Copy
            + Clone
            + FullCodec
            + TypeInfo
            + MaxEncodedLen;

        /// Represents which units of time can be used. Designed to be an enum
        /// with a variant for each kind of time source/scale supported.
        type TimeUnit: Debug + Clone + FullCodec + TypeInfo + MaxEncodedLen + Eq;

        /// The balance type, which is also the type representing time (as this
        /// pallet will do math with both time and balances to compute how
        /// much should be paid).
        type Balance: Balance;

        /// LockId type used by `Currencies`.
        type LockId: From<LockId>;

        /// The currencies type, supporting multiple currencies.
        type Currencies: fungibles::Inspect<Self::AccountId, Balance = Self::Balance>
            + fungibles::InspectFreeze<Self::AccountId, Id = Self::LockId>
            + fungibles::Mutate<Self::AccountId>
            + fungibles::MutateFreeze<Self::AccountId>;

        /// Provide the current time in given unit.
        type TimeProvider: TimeProvider<Self::TimeUnit, Self::Balance>;
    }

    type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    type AssetIdOf<T> = <<T as Config>::Currencies as fungibles::Inspect<AccountIdOf<T>>>::AssetId;

    /// A stream payment from source to target.
    /// Stores the last time the stream was updated, which allows to compute
    /// elapsed time and perform payment.
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    #[derive(RuntimeDebug, PartialEq, Eq, Encode, Decode, Clone, TypeInfo)]
    pub struct Stream<AccountId, Unit, AssetId, Balance> {
        source: AccountId,
        target: AccountId,
        time_unit: Unit,
        asset_id: AssetId,
        rate_per_time_unit: Balance,
        locked_funds: Balance,
        last_time_updated: Balance,
    }

    pub type StreamOf<T> =
        Stream<AccountIdOf<T>, <T as Config>::TimeUnit, AssetIdOf<T>, <T as Config>::Balance>;

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
        CantBeBothSourceAndTarget,
        CantFetchCurrentTime,
        TimeOverflow,
        CurrencyOverflow,
    }

    #[pallet::composite_enum]
    pub enum LockId {
        StreamPayment,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        pub fn open_stream(
            origin: OriginFor<T>,
            target: AccountIdOf<T>,
            time_unit: T::TimeUnit,
            asset_id: AssetIdOf<T>,
            rate_per_time_unit: T::Balance,
            initial_deposit: T::Balance,
        ) -> DispatchResultWithPostInfo {
            let origin = ensure_signed(origin)?;
            ensure!(origin != target, Error::<T>::CantBeBothSourceAndTarget);

            let stream_id = NextStreamId::<T>::get();
            let next_stream_id = stream_id
                .checked_add(&One::one())
                .ok_or(Error::<T>::StreamIdOverflow)?;
            NextStreamId::<T>::set(next_stream_id);

            T::Currencies::increase_frozen(
                asset_id.clone(),
                &LockId::StreamPayment.into(),
                &origin,
                initial_deposit,
            )?;

            let now = T::TimeProvider::now(&time_unit).ok_or(Error::<T>::CantFetchCurrentTime)?;
            let stream = Stream {
                source: origin.clone(),
                target: target.clone(),
                time_unit,
                asset_id,
                rate_per_time_unit,
                locked_funds: initial_deposit,
                last_time_updated: now,
            };

            Streams::<T>::insert(stream_id, stream);
            LookupStreamsWithSource::<T>::insert(origin, stream_id, ());
            LookupStreamsWithTarget::<T>::insert(target, stream_id, ());

            Ok(().into())
        }

        #[pallet::call_index(1)]
        pub fn close_stream(
            _origin: OriginFor<T>,
            _stream_id: T::StreamId,
        ) -> DispatchResultWithPostInfo {
            todo!()
        }

        #[pallet::call_index(2)]
        pub fn update_stream(
            origin: OriginFor<T>,
            stream_id: T::StreamId,
        ) -> DispatchResultWithPostInfo {
            // No problem with anyone updating any stream.
            let _ = ensure_signed(origin)?;

            let mut stream = Streams::<T>::get(stream_id).ok_or(Error::<T>::UnknownStreamId)?;
            Self::perform_stream_payment(&mut stream)?;
            Streams::<T>::insert(stream_id, stream);

            // TODO: Event here or in do_update_stream?

            Ok(().into())
        }

        #[pallet::call_index(3)]
        pub fn refill_stream(
            _origin: OriginFor<T>,
            _stream_id: T::StreamId,
            _new_deposit: T::Balance,
        ) -> DispatchResultWithPostInfo {
            todo!()
        }

        #[pallet::call_index(4)]
        pub fn change_stream_rate(
            _origin: OriginFor<T>,
            _stream_id: T::StreamId,
            _new_rate_per_time_unit: T::Balance,
        ) -> DispatchResultWithPostInfo {
            todo!()
        }
    }

    impl<T: Config> Pallet<T> {
        /// Behavior:
        /// A stream payment consist of a locked deposit, a rate per unit of time and the
        /// last time the stream was updated. When updating the stream, **at most**
        /// `elapsed_time * rate` is unlocked from the source account and transfered to the target
        /// account. If this amount is greater than the left deposit, the stream is considered
        /// drained **but not closed**. The source can come back later and refill the stream,
        /// however there will be no retroactive payment for the time spent as drained.
        /// If the stream payment is used to rent a service, the target should pause the service
        /// while the stream is drained, and resume it once it is refilled.
        fn perform_stream_payment(stream: &mut StreamOf<T>) -> DispatchResultWithPostInfo {
            let now =
                T::TimeProvider::now(&stream.time_unit).ok_or(Error::<T>::CantFetchCurrentTime)?;

            if stream.locked_funds.is_zero() {
                stream.last_time_updated = now;
                return Ok(().into());
            }

            let delta = now
                .checked_sub(&stream.last_time_updated)
                .ok_or(Error::<T>::TimeOverflow)?;
            let mut payment = delta
                .checked_mul(&stream.rate_per_time_unit)
                .ok_or(Error::<T>::CurrencyOverflow)?;

            // We compute the new amount of locked funds. If it underflows it
            // means that there is more to pay that what is left, in which case
            // we pay all that is left.
            let new_locked = match stream.locked_funds.checked_sub(&payment) {
                Some(v) => v,
                None => {
                    payment = stream.locked_funds;
                    Zero::zero()
                }
            };

            T::Currencies::decrease_frozen(
                stream.asset_id.clone(),
                &LockId::StreamPayment.into(),
                &stream.source,
                payment,
            )?;
            T::Currencies::transfer(
                stream.asset_id.clone(),
                &stream.source,
                &stream.target,
                payment,
                Preservation::Preserve,
            )?;

            stream.last_time_updated = now;
            stream.locked_funds = new_locked;

            // TODO: Emit event here?            

            Ok(().into())
        }
    }
}
