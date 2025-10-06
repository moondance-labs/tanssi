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
const APP_WHITELISTED: Curve =
    Curve::make_reciprocal(1, 14, percent(96), percent(50), percent(100));
const SUP_WHITELISTED: Curve =
    Curve::make_reciprocal(1, 14 * 24, percent(1), percent(0), percent(2));

const TRACKS_DATA: [pallet_referenda::Track<u16, Balance, BlockNumber>; 2] = [
    pallet_referenda::Track {
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
    },
    pallet_referenda::Track {
        id: 1,
        info: pallet_referenda::TrackInfo {
            name: s("whitelisted_caller"),
            max_deciding: 100,
            decision_deposit: 500 * GRAND,
            prepare_period: 10 * MINUTES,
            decision_period: 14 * DAYS,
            confirm_period: 10 * MINUTES,
            min_enactment_period: 30 * MINUTES,
            min_approval: APP_WHITELISTED,
            min_support: SUP_WHITELISTED,
        },
    },
];

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
        } else if let Ok(custom_origin) = pallet_custom_origins::Origin::try_from(id.clone()) {
            match custom_origin {
                origins::Origin::WhitelistedCaller => Ok(1),
                _ => Err(()),
            }
        } else {
            Err(())
        }
    }
}

#[test]
/// To ensure voters are always locked into their vote
fn vote_locking_always_longer_than_enactment_period() {
    use core::str::from_utf8;
    for track in TRACKS_DATA {
        assert!(
            <Runtime as pallet_conviction_voting::Config>::VoteLockingPeriod::get()
                >= track.info.min_enactment_period,
            "Track {} has enactment period {} < vote locking period {}",
            from_utf8(&track.info.name).expect("Track name is valid UTF-8"),
            track.info.min_enactment_period,
            <Runtime as pallet_conviction_voting::Config>::VoteLockingPeriod::get(),
        );
    }
}
