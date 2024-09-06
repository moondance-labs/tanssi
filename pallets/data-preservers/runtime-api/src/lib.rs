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

//! Runtime API for DataPreservers pallet

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use {
    parity_scale_codec::{Decode, Encode},
    serde::{Deserialize, Serialize},
};

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Encode, Decode, scale_info::TypeInfo, Serialize, Deserialize,
)]
pub enum Assignment<ParaId> {
    /// Profile is not currently assigned.
    NotAssigned,
    /// Profile is activly assigned to this ParaId.
    Active(ParaId),
    /// Profile is assigned to this ParaId but is inactive for some reason.
    /// It may be causes by conditions defined in the assignement configuration,
    /// such as lacking payment.
    Inactive(ParaId),
}

sp_api::decl_runtime_apis! {
    pub trait DataPreserversApi<ProfileId, ParaId>
    where
        ProfileId: parity_scale_codec::Codec,
        ParaId: parity_scale_codec::Codec,
    {
        /// Get the active assignment for this profile id.
        fn get_active_assignment(
            profile_id: ProfileId,
        ) -> Assignment<ParaId>;
    }
}
