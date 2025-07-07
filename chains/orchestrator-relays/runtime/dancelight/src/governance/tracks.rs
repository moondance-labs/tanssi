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

const fn percent(x: i32) -> sp_arithmetic::FixedI64 {
    sp_arithmetic::FixedI64::from_rational(x as u128, 100)
}
use pallet_referenda::Curve;
const APP_ROOT: Curve = Curve::make_reciprocal(4, 28, percent(80), percent(50), percent(100));
const SUP_ROOT: Curve = Curve::make_linear(28, 28, percent(0), percent(50));

const TRACKS_DATA: [(u16, pallet_referenda::TrackInfo<Balance, BlockNumber>); 1] = [(
    0,
    pallet_referenda::TrackInfo {
        name: "root",
        max_deciding: 1,
        decision_deposit: 100 * GRAND,
        prepare_period: 8 * MINUTES,
        decision_period: 20 * MINUTES,
        confirm_period: 12 * MINUTES,
        min_enactment_period: 5 * MINUTES,
        min_approval: APP_ROOT,
        min_support: SUP_ROOT,
    },
)];

pub struct TracksInfo;
impl pallet_referenda::TracksInfo<Balance, BlockNumber> for TracksInfo {
    type Id = u16;
    type RuntimeOrigin = <RuntimeOrigin as frame_support::traits::OriginTrait>::PalletsOrigin;
    fn tracks() -> &'static [(Self::Id, pallet_referenda::TrackInfo<Balance, BlockNumber>)] {
        &TRACKS_DATA[..]
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
                // General admin
                origins::Origin::StakingAdmin => Ok(10),
                origins::Origin::Treasurer => Ok(11),
                origins::Origin::LeaseAdmin => Ok(12),
                origins::Origin::FellowshipAdmin => Ok(13),
                origins::Origin::GeneralAdmin => Ok(14),
                origins::Origin::AuctionAdmin => Ok(15),
                // Referendum admins
                origins::Origin::ReferendumCanceller => Ok(20),
                origins::Origin::ReferendumKiller => Ok(21),
                // Limited treasury spenders
                origins::Origin::SmallTipper => Ok(30),
                origins::Origin::BigTipper => Ok(31),
                origins::Origin::SmallSpender => Ok(32),
                origins::Origin::MediumSpender => Ok(33),
                origins::Origin::BigSpender => Ok(34),
                _ => Err(()),
            }
        } else {
            Err(())
        }
    }
}
pallet_referenda::impl_tracksinfo_get!(TracksInfo, Balance, BlockNumber);
