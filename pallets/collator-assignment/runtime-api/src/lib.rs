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

//! Runtime API for CollatorAssignment pallet. Can be used by collators to check
//! which parachain will they be collating, as well as the current assignment of
//! collators to parachains and parachains to collators.

#![cfg_attr(not(feature = "std"), no_std)]

use scale_info::prelude::vec::Vec;

sp_api::decl_runtime_apis! {
    #[api_version(2)]
    pub trait CollatorAssignmentApi<AccountId, ParaId> where
        AccountId: parity_scale_codec::Codec,
        ParaId: parity_scale_codec::Codec,
    {
        /// Returns the list of `ParaId` of registered chains with at least some
        /// collators. This filters out parachains with no assigned collators.
        #[api_version(2)]
        fn parachains_with_some_collators() -> Vec<ParaId>;
    }
}
