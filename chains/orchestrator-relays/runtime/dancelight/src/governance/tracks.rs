// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot. If not, see <http://www.gnu.org/licenses/>.

//! Track configurations for governance.

use super::*;
use sp_runtime::str_array as s;
const fn percent(x: u128) -> sp_arithmetic::FixedI64 {
    sp_arithmetic::FixedI64::from_rational(x, 100)
}
use pallet_referenda::Curve;
const APP_ROOT: Curve = Curve::make_reciprocal(4, 28, percent(80), percent(50), percent(100));
const SUP_ROOT: Curve = Curve::make_linear(28, 28, percent(20), percent(50));
const TRACKS_DATA: [pallet_referenda::Track<u16, Balance, BlockNumber>; 1] =
    [pallet_referenda::Track {
        id: 0,
        info: pallet_referenda::TrackInfo {
            name: s("root"),
            max_deciding: 1,
            decision_deposit: 500 * GRAND,
            prepare_period: 8 * MINUTES,
            decision_period: 20 * MINUTES,
            confirm_period: 12 * MINUTES,
            min_enactment_period: 5 * MINUTES,
            min_approval: APP_ROOT,
            min_support: SUP_ROOT,
        },
    }];

pub struct TracksInfo;
impl pallet_referenda::TracksInfo<Balance, BlockNumber> for TracksInfo {
    type Id = u16;
    type RuntimeOrigin = <RuntimeOrigin as frame_support::traits::OriginTrait>::PalletsOrigin;
    fn tracks(
    ) -> impl Iterator<Item = Cow<'static, pallet_referenda::Track<Self::Id, Balance, BlockNumber>>>
    {
        TRACKS_DATA.iter().map(Cow::Borrowed)
    }
    fn track_for(id: &Self::RuntimeOrigin) -> Result<Self::Id, ()> {
        if let Ok(system_origin) = frame_system::RawOrigin::try_from(id.clone()) {
            match system_origin {
                frame_system::RawOrigin::Root => Ok(0),
                _ => Err(()),
            }
        } else {
            Err(())
        }
    }
}
