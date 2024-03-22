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

//! Runtime API for Stream Payment pallet

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use {
    alloc::string::String,
    parity_scale_codec::{Decode, Encode},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode, scale_info::TypeInfo)]
pub struct StreamPaymentApiStatus<Balance> {
    pub payment: Balance,
    pub deposit_left: Balance,
    /// Whenever the stream is inactive, which can occur either when no funds are left or
    /// if the time is past a mandatory request deadline.
    pub inactive: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, scale_info::TypeInfo)]
pub enum StreamPaymentApiError {
    UnknownStreamId,
    Other(String),
}

sp_api::decl_runtime_apis! {
    pub trait StreamPaymentApi<StreamId, Instant, Balance>
    where
        StreamId: parity_scale_codec::Codec,
        Instant: parity_scale_codec::Codec,
        Balance: parity_scale_codec::Codec,
    {
        /// Get the stream payment current status, telling how much payment is
        /// pending, how much deposit will be left and whenever the stream is inactive.
        /// The stream is considered inactive if no funds are left or if the provided
        /// time is past a mandatory request deadline. If the provided `now` is `None`
        /// then the current time will be fetched. Being able to provide a custom `now`
        /// allows to check the status in the future.
        fn stream_payment_status(
            stream_id: StreamId,
            now: Option<Instant>,
        ) -> Result<StreamPaymentApiStatus<Balance>, StreamPaymentApiError>;
    }
}
