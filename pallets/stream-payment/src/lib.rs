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
        traits::{fungibles, tokens::Balance},
        Blake2_128Concat,
    },
    frame_system::pallet_prelude::*,
    parity_scale_codec::{FullCodec, MaxEncodedLen},
    scale_info::TypeInfo,
    sp_std::{fmt::Debug, marker::PhantomData},
};

/// Type able to provide the current time for given unit.
pub trait TimeProvider<Unit, Number> {
    fn now(unit: Unit) -> Option<Number>;
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
        /// Represents which units of time can be used. Designed to be an enum
        /// with a variant for each kind of time source/scale supported.
        type TimeUnit: Debug + Clone + FullCodec + TypeInfo + MaxEncodedLen + Eq;

        /// The balance type, which is also the type representing time (as this
        /// pallet will do math with both time and balances to compute how
        /// much should be paid).
        type Balance: Balance;

        /// The currencies type, supporting multiple currencies.
        type Currencies: fungibles::Inspect<Self::AccountId, Balance = Self::Balance>;

        /// Provide the current time in given unit.
        type TimeProvider: TimeProvider<Self::TimeUnit, Self::Balance>;
    }

    pub type StreamId = u64;
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
    pub type NextStreamId<T: Config> = StorageValue<Value = StreamId, QueryKind = ValueQuery>;

    /// Store each stream indexed by an Id.
    #[pallet::storage]
    pub type Streams<T: Config> = StorageMap<
        Hasher = Blake2_128Concat,
        Key = StreamId,
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
        Key2 = StreamId,
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
        Key2 = StreamId,
        Hasher2 = Blake2_128Concat,
        Value = (),
        QueryKind = OptionQuery,
    >;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        pub fn open_stream(
            _origin: OriginFor<T>,
            _target: AccountIdOf<T>,
            _time_unit: T::TimeUnit,
            _asset_id: AssetIdOf<T>,
            _rate_per_time_unit: T::Balance,
            _initial_deposit: T::Balance,
        ) -> DispatchResultWithPostInfo {
            todo!()
        }

        #[pallet::call_index(1)]
        pub fn close_stream(
            _origin: OriginFor<T>,
            _stream: StreamId,
        ) -> DispatchResultWithPostInfo {
            todo!()
        }

        #[pallet::call_index(2)]
        pub fn update_stream(
            _origin: OriginFor<T>,
            _stream: StreamId,
        ) -> DispatchResultWithPostInfo {
            todo!()
        }

        #[pallet::call_index(3)]
        pub fn refill_stream(
            _origin: OriginFor<T>,
            _stream: StreamId,
            _new_deposit: T::Balance,
        ) -> DispatchResultWithPostInfo {
            todo!()
        }

        #[pallet::call_index(4)]
        pub fn change_stream_rate(
            _origin: OriginFor<T>,
            _stream: StreamId,
            _new_rate_per_time_unit: T::Balance,
        ) -> DispatchResultWithPostInfo {
            todo!()
        }
    }
}
