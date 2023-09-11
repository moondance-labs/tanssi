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
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]

use {
    frame_support::RuntimeDebug,
    parity_scale_codec::Codec,
    scale_info::TypeInfo,
    sp_api::{Decode, Encode},
};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(RuntimeDebug, PartialEq, Eq, Encode, Decode, Copy, Clone, TypeInfo)]
pub enum TargetPool {
    AutoCompounding,
    ManualRewards,
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(RuntimeDebug, PartialEq, Eq, Encode, Decode, Copy, Clone, TypeInfo)]
pub enum AllTargetPool {
    Joining,
    AutoCompounding,
    ManualRewards,
    Leaving,
}

impl From<TargetPool> for AllTargetPool {
    fn from(value: TargetPool) -> Self {
        match value {
            TargetPool::AutoCompounding => AllTargetPool::AutoCompounding,
            TargetPool::ManualRewards => AllTargetPool::ManualRewards,
        }
    }
}

sp_api::decl_runtime_apis! {
    pub trait PooledStakingApi<AccountId: Codec, Balance: Codec> {
        fn staked_in_pool(
            candidate: AccountId,
            delegator: AccountId,
            pool: AllTargetPool
        ) -> Option<Balance>;

        fn pending_manual_rewards(
            candidate: AccountId,
            delegator: AccountId
        ) -> Option<Balance>;
    }
}
