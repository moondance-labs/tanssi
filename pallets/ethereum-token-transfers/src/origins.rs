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

use {
    super::pallet::Origin, crate::Config, frame_support::pallet_prelude::*,
    sp_runtime::traits::Morph,
};

/// Implementation of the [EnsureOrigin] trait for the [Origin::EthereumTokenTransfers] origin.
pub struct EnsureEthereumTokenTransfersOrigin<T>(core::marker::PhantomData<T>);
impl<T: Config, O: OriginTrait + From<Origin<T>>> EnsureOrigin<O>
    for EnsureEthereumTokenTransfersOrigin<T>
where
    for<'a> &'a O::PalletsOrigin: TryInto<&'a Origin<T>>,
{
    type Success = T::AccountId;

    fn try_origin(o: O) -> Result<Self::Success, O> {
        match o.caller().try_into() {
            Ok(Origin::EthereumTokenTransfers(account_id)) => return Ok(account_id.clone()),
            _ => (),
        }

        Err(o)
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn try_successful_origin() -> Result<O, ()> {
        use frame_support::traits::Get;
        let account = T::BenchmarkHelper::get();
        Ok(O::from(Origin::EthereumTokenTransfers(account)))
    }
}

pub struct ConvertUnitTo<T>(core::marker::PhantomData<T>);

impl<OutcomeType> Morph<()> for ConvertUnitTo<OutcomeType>
where
    OutcomeType: From<xcm::latest::Location>,
{
    type Outcome = OutcomeType;

    fn morph(_: ()) -> OutcomeType {
        xcm::latest::Location::here().into()
    }
}

pub struct ConvertAccountIdTo<AccountId, T, Network>(
    core::marker::PhantomData<(AccountId, T, Network)>
);

impl<T1, AccountId, OutcomeType, Network> Morph<T1> for ConvertAccountIdTo<AccountId, OutcomeType, Network>
where
    AccountId: From<T1> + Into<[u8; 32]>,
    OutcomeType: From<xcm::latest::Junction>,
    Network: Get<Option<xcm::latest::NetworkId>>,
{
    type Outcome = OutcomeType;

    fn morph(account_id: T1) -> OutcomeType {
        let account_id_32: AccountId = AccountId::from(account_id);
        xcm::latest::Junction::AccountId32 {
            network: Network::get(),
            id: account_id_32.into(),
        }
            .into()
    }
}
