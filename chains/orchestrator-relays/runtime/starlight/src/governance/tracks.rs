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

const fn percent(x: u128) -> sp_arithmetic::FixedI64 {
    sp_arithmetic::FixedI64::from_rational(x, 100)
}
use pallet_referenda::{Curve, Track};
use runtime_common::prod_or_fast;
use sp_runtime::str_array as s;
const APP_ROOT: Curve = Curve::make_reciprocal(4, 28, percent(80), percent(50), percent(100));
const SUP_ROOT: Curve = Curve::make_linear(28, 28, percent(0), percent(50));
const APP_WHITELISTED_CALLER: Curve =
    Curve::make_reciprocal(16, 28 * 24, percent(96), percent(50), percent(100));
const SUP_WHITELISTED_CALLER: Curve =
    Curve::make_reciprocal(1, 28, percent(20), percent(5), percent(50));

const TRACKS_DATA: [pallet_referenda::Track<u16, Balance, BlockNumber>; 2] = [
    pallet_referenda::Track {
        id: 0,
        info: pallet_referenda::TrackInfo {
            name: s("root"),
            max_deciding: 1,
            decision_deposit: 500 * GRAND,
            prepare_period: prod_or_fast!(1 * DAYS, 8 * MINUTES),
            decision_period: prod_or_fast!(14 * DAYS, 20 * MINUTES),
            confirm_period: prod_or_fast!(1 * DAYS, 12 * MINUTES),
            min_enactment_period: prod_or_fast!(1 * DAYS, 5 * MINUTES),
            min_approval: APP_ROOT,
            min_support: SUP_ROOT,
        },
    },
    pallet_referenda::Track {
        id: 1,
        info: pallet_referenda::TrackInfo {
            name: s("whitelisted_caller"),
            max_deciding: 100,
            decision_deposit: 10 * GRAND,
            prepare_period: 10 * MINUTES,
            decision_period: 14 * DAYS,
            confirm_period: 10 * MINUTES,
            min_enactment_period: 30 * MINUTES,
            min_approval: APP_WHITELISTED_CALLER,
            min_support: SUP_WHITELISTED_CALLER,
        },
    },
];

pub struct TracksInfo;
#[allow(unreachable_patterns)]
// Allow unreachable patterns for potentially future origin tracks
impl pallet_referenda::TracksInfo<Balance, BlockNumber> for TracksInfo {
    type Id = u16;
    type RuntimeOrigin = <RuntimeOrigin as frame_support::traits::OriginTrait>::PalletsOrigin;
    fn tracks() -> impl Iterator<Item = Cow<'static, Track<Self::Id, Balance, BlockNumber, 25>>> {
        TRACKS_DATA.iter().map(Cow::Borrowed)
    }
    fn track_for(id: &Self::RuntimeOrigin) -> Result<Self::Id, ()> {
        if let Ok(system_origin) = frame_system::RawOrigin::try_from(id.clone()) {
            match system_origin {
                frame_system::RawOrigin::Root => Ok(0),
                _ => Err(()),
            }
        } else if let Ok(custom_origin) = origins::Origin::try_from(id.clone()) {
            match custom_origin {
                origins::Origin::WhitelistedCaller => Ok(1),
                _ => Err(()),
            }
        } else {
            Err(())
        }
    }
}
