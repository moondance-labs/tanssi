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
    frame_support::traits::OriginTrait,
    sp_core::Get,
    sp_runtime::traits::TryConvert,
    staging_xcm::latest::{Junction::AccountKey20, MultiLocation, NetworkId},
};

// Convert a local Origin (i.e., a signed 20 byte account Origin)  to a Multilocation
pub struct SignedToAccountKey20<Origin, AccountId, Network>(
    sp_std::marker::PhantomData<(Origin, AccountId, Network)>,
);
impl<Origin, AccountId, Network: Get<NetworkId>> TryConvert<Origin, MultiLocation>
    for SignedToAccountKey20<Origin, AccountId, Network>
where
    Origin: OriginTrait + Clone,
    AccountId: Into<[u8; 20]>,
    Origin::PalletsOrigin: From<frame_system::RawOrigin<AccountId>>
        + TryInto<frame_system::RawOrigin<AccountId>, Error = Origin::PalletsOrigin>,
{
    fn try_convert(o: Origin) -> Result<MultiLocation, Origin> {
        o.try_with_caller(|caller| match caller.try_into() {
            Ok(frame_system::RawOrigin::Signed(who)) => Ok(AccountKey20 {
                key: who.into(),
                network: Some(Network::get()),
            }
            .into()),
            Ok(other) => Err(other.into()),
            Err(other) => Err(other),
        })
    }
}
