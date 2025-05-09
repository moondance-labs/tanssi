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

use frame_support::traits::Get;
use xcm::latest::prelude::*;
trait Parse {
    /// Returns the "chain" location part. It could be parent, sibling
    /// parachain, or child parachain.
    fn chain_part(&self) -> Option<Location>;
}

impl Parse for Location {
    fn chain_part(&self) -> Option<Location> {
        match (self.parents, self.first_interior()) {
            // sibling parachain
            (1, Some(Parachain(id))) => Some(Location::new(1, [Parachain(*id)])),
            // parent
            (1, _) => Some(Location::parent()),
            // children parachain
            (0, Some(Parachain(id))) => Some(Location::new(0, [Parachain(*id)])),
            _ => None,
        }
    }
}

pub struct NativeAssetReserve;
impl frame_support::traits::ContainsPair<Asset, Location> for NativeAssetReserve {
    fn contains(asset: &Asset, origin: &Location) -> bool {
        log::trace!(target: "xcm::contains", "NativeAssetReserve asset: {:?}, origin: {:?}", asset, origin);
        let reserve = if asset.id.0.parents == 0
            && !matches!(asset.id.0.first_interior(), Some(Parachain(_)))
        {
            Some(Location::here())
        } else {
            asset.id.0.chain_part()
        };

        if let Some(ref reserve) = reserve {
            if reserve == origin {
                return true;
            }
        }
        false
    }
}

pub struct EthereumAssetReserve<EthereumLocation, EthereumNetwork>(
    sp_std::marker::PhantomData<(EthereumLocation, EthereumNetwork)>,
);
impl<EthereumLocation, EthereumNetwork> frame_support::traits::ContainsPair<Asset, Location>
    for EthereumAssetReserve<EthereumLocation, EthereumNetwork>
where
    EthereumLocation: Get<Location>,
    EthereumNetwork: Get<NetworkId>,
{
    fn contains(asset: &Asset, origin: &Location) -> bool {
        log::trace!(target: "xcm::contains", "EthereumAssetReserve asset: {:?}, origin: {:?}", asset, origin);
        let ethereum_network = EthereumNetwork::get();
        let reserve = if asset.id.0.parents == 1 {
            match asset.id.0.first_interior() {
                Some(GlobalConsensus(network)) if *network == ethereum_network => {
                    Some(EthereumLocation::get())
                }
                _ => None,
            }
        } else {
            None
        };

        // Origin must match the EthereumLocation reserve
        if let Some(ref reserve) = reserve {
            if reserve == origin {
                return true;
            }
        }
        false
    }
}
