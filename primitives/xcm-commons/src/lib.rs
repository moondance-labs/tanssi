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

pub use dp_xcm_reserve::NativeAssetReserve;

/// Filter to ensure an ETH asset is coming from a trusted Ethereum location.
pub struct EthereumAssetReserve<EthereumLocation, EthereumNetwork>(
    core::marker::PhantomData<(EthereumLocation, EthereumNetwork)>,
);
impl<EthereumLocation, EthereumNetwork> frame_support::traits::ContainsPair<Asset, Location>
    for EthereumAssetReserve<EthereumLocation, EthereumNetwork>
where
    EthereumLocation: Get<Location>,
    EthereumNetwork: Get<NetworkId>,
{
    fn contains(asset: &Asset, origin: &Location) -> bool {
        log::trace!(target: "xcm::contains", "EthereumAssetReserve asset: {:?}, origin: {:?}", asset, origin);
        if *origin != EthereumLocation::get() {
            return false;
        }
        matches!((asset.id.0.parents, asset.id.0.first_interior()), (1, Some(GlobalConsensus(network))) if *network == EthereumNetwork::get())
    }
}

/// Filter to ensure an ETH asset is coming from a parent.
pub struct EthereumAssetReserveFromParent<EthereumLocation, EthereumNetwork>(
    core::marker::PhantomData<(EthereumLocation, EthereumNetwork)>,
);
impl<EthereumLocation, EthereumNetwork> frame_support::traits::ContainsPair<Asset, Location>
    for EthereumAssetReserveFromParent<EthereumLocation, EthereumNetwork>
where
    EthereumLocation: Get<Location>,
    EthereumNetwork: Get<NetworkId>,
{
    fn contains(asset: &Asset, origin: &Location) -> bool {
        log::trace!(target: "xcm::contains", "EthereumAssetReserveFromPara asset: {:?}, origin: {:?}, eth_network: {:?}", asset, origin, EthereumLocation::get());
        if *origin == EthereumLocation::get() || *origin == Location::parent() {
            return matches!((asset.id.0.parents, asset.id.0.first_interior()), (2, Some(GlobalConsensus(network))) if *network == EthereumNetwork::get());
        }
        return false;
    }
}

/// Filter to ensure that Ethereum can be recongnized as a reserve for Tanssi asset.
/// Used in containers to allow sending tokens to Ethereum and paying fees with Tanssi.
pub struct EthereumAssetReserveForTanssi<EthereumLocation>(
    core::marker::PhantomData<EthereumLocation>,
);
impl<EthereumLocation> frame_support::traits::ContainsPair<Asset, Location>
    for EthereumAssetReserveForTanssi<EthereumLocation>
where
    EthereumLocation: Get<Location>,
{
    fn contains(asset: &Asset, origin: &Location) -> bool {
        log::trace!(target: "xcm::contains", "EthereumAssetReserveForTanssi asset: {:?}, origin: {:?}, eth_network: {:?}", asset, origin, EthereumLocation::get());
        if *origin == EthereumLocation::get() {
            return matches!((asset.id.0.parents, asset.id.0.first_interior()), (1, None));
        }
        return false;
    }
}

/// Filter to ensure that Ethereum can be recognized as a reserve for container chain assets.
/// Used when assets native to container chains are transferred back from Ethereum.
/// 
/// Note: this is not the usual way of managing container assets, as recognizing Ethereum as 
/// their reserve is not the "correct" way of doing it.
/// 
/// This is an exception for inbound queue V2 processing, where we need to recognize Ethereum 
/// as the reserve for container assets at the moment of placing them in the holding (in Tanssi)
/// via ReserveAssetDeposited instruction.
/// 
/// Otherwise, we don't have any other way of placing inbound container assets in holding, since
/// we cannot extract them from the Ethereum's sovereign account either (as the assets 
/// don't exist in Tanssi).
pub struct EthereumAssetReserveForContainerAssets<EthereumLocation>(
    core::marker::PhantomData<EthereumLocation>,
);
impl<EthereumLocation> frame_support::traits::ContainsPair<Asset, Location>
    for EthereumAssetReserveForContainerAssets<EthereumLocation>
where
    EthereumLocation: Get<Location>,
{
    fn contains(asset: &Asset, origin: &Location) -> bool {
        log::trace!(target: "xcm::contains", "EthereumAssetReserveForContainerAssets asset: {:?}, origin: {:?}, eth_location: {:?}", asset, origin, EthereumLocation::get());
        if *origin == EthereumLocation::get() {
            // Check if the asset has a Parachain junction as its first interior,
            // indicating it's native to a container chain
            return matches!(
                (asset.id.0.parents, asset.id.0.first_interior()),
                (0, Some(Parachain(_)))
            );
        }
        false
    }
}
