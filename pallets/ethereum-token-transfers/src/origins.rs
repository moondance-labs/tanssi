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

use {super::pallet::Origin, frame_support::pallet_prelude::*};

/// Implementation of the [EnsureOrigin] trait for the [Origin::EthereumTokenTransfers] origin.
pub struct EnsureEthereumTokenTransfers;
impl<O: OriginTrait + From<Origin>> EnsureOrigin<O> for EnsureEthereumTokenTransfers
where
    for<'a> &'a O::PalletsOrigin: TryInto<&'a Origin>,
{
    type Success = ();
    fn try_origin(o: O) -> Result<Self::Success, O> {
        match o.caller().try_into() {
            Ok(Origin::EthereumTokenTransfers) => return Ok(()),
            _ => (),
        }

        Err(o)
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn try_successful_origin() -> Result<O, ()> {
        Ok(O::from(Origin::EthereumTokenTransfers))
    }
}

// Probably need to propose upstream
pub struct EitherOfDiverseWithSuccess<L, R, Success>(PhantomData<(L, R, Success)>);

impl<OuterOrigin: Clone, L, R, Success> EnsureOrigin<OuterOrigin>
    for EitherOfDiverseWithSuccess<L, R, Success>
where
    L: EnsureOrigin<OuterOrigin>,
    R: EnsureOrigin<OuterOrigin>,
    Success: TypedGet,
{
    type Success = Success::Type;

    fn try_origin(o: OuterOrigin) -> Result<Self::Success, OuterOrigin> {
        if L::try_origin(o.clone()).is_ok() || R::try_origin(o.clone()).is_ok() {
            Ok(Success::get())
        } else {
            Err(o)
        }
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn try_successful_origin() -> Result<OuterOrigin, ()> {
        L::try_successful_origin().or_else(|_| R::try_successful_origin())
    }
}
