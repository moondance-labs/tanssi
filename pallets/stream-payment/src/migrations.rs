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

use crate::{AccountIdOf, AssetIdOf, Config};
use core::marker::PhantomData;
use frame_support::pallet_prelude::Weight;
use frame_support::traits::Get;
use pallet_migrations::Migration;
use parity_scale_codec::{Decode, Encode};
use sp_std::vec;
use sp_std::vec::Vec;

#[derive(Encode, Decode, Clone)]
pub struct OldStream<AccountId, Unit, AssetId, Balance> {
    pub source: AccountId,
    pub target: AccountId,
    pub config: OldStreamConfig<Unit, AssetId, Balance>,
    pub deposit: Balance,
    pub last_time_updated: Balance,
    pub request_nonce: crate::RequestNonce,
    pub pending_request: Option<OldChangeRequest<Unit, AssetId, Balance>>,
    pub opening_deposit: Balance,
}

type OldStreamOf<T> =
    OldStream<AccountIdOf<T>, <T as Config>::TimeUnit, AssetIdOf<T>, <T as Config>::Balance>;

#[derive(Encode, Decode, Clone)]
pub struct OldStreamConfig<Unit, AssetId, BalanceOrDuration> {
    /// Unit in which time is measured using a `TimeProvider`.
    pub time_unit: Unit,
    /// Asset used for payment.
    pub asset_id: AssetId,
    /// Amount of asset / unit.
    pub rate: BalanceOrDuration,
}

#[derive(Encode, Decode, Clone)]
pub struct OldChangeRequest<Unit, AssetId, Balance> {
    pub requester: crate::Party,
    pub kind: crate::ChangeKind<Balance>,
    pub new_config: OldStreamConfig<Unit, AssetId, Balance>,
    pub deposit_change: Option<crate::DepositChange<Balance>>,
}

pub struct MigrateStreamPaymentNewConfigFields<T>(pub PhantomData<T>);
impl<T> Migration for MigrateStreamPaymentNewConfigFields<T>
where
    T: Config,
{
    fn friendly_name(&self) -> &str {
        "TM_MigrateStreamPaymentNewConfigFields"
    }

    fn migrate(&self, _available_weight: Weight) -> Weight {
        let mut count = 0;
        crate::Streams::<T>::translate(|_key, value: OldStreamOf<T>| {
            count += 1;
            let OldStream {
                source,
                target,
                deposit,
                last_time_updated,
                request_nonce,
                pending_request,
                opening_deposit,
                config:
                    OldStreamConfig {
                        time_unit,
                        asset_id,
                        rate,
                    },
            } = value;

            let pending_request = pending_request.map(
                |OldChangeRequest {
                     requester,
                     kind,
                     new_config:
                         OldStreamConfig {
                             time_unit,
                             asset_id,
                             rate,
                         },
                     deposit_change,
                 }| crate::ChangeRequest {
                    requester,
                    kind,
                    deposit_change,
                    new_config: crate::StreamConfig {
                        time_unit,
                        asset_id,
                        rate,
                        minimum_request_deadline_delay: 0u32.into(),
                        soft_minimum_deposit: 0u32.into(),
                    },
                },
            );

            Some(crate::Stream {
                source,
                target,
                deposit,
                last_time_updated,
                request_nonce,
                pending_request,
                opening_deposit,
                config: crate::StreamConfig {
                    time_unit,
                    asset_id,
                    rate,
                    minimum_request_deadline_delay: 0u32.into(),
                    soft_minimum_deposit: 0u32.into(),
                },
            })
        });

        let db_weights = T::DbWeight::get();
        db_weights.reads_writes(count, count)
    }

    #[cfg(feature = "try-runtime")]
    fn pre_upgrade(&self) -> Result<Vec<u8>, sp_runtime::DispatchError> {
        Ok(vec![])
    }

    #[cfg(feature = "try-runtime")]
    fn post_upgrade(&self, _state: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
        Ok(())
    }
}
