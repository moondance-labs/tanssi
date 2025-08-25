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

extern crate alloc;

use {
    alloc::collections::btree_set::BTreeSet,
    frame_support::{parameter_types, traits::Contains},
    xcm::latest::prelude::*,
};

/// The pallet index of the Ethereum inbound queue pallet in the Tanssi runtime.
pub const PARENT_INBOUND_QUEUE_PALLET_INDEX: u8 = 24;

parameter_types! {
    /// Network and location for the Ethereum chain. On Starlight, the Ethereum chain bridged
    /// to is the Ethereum mainnet, with chain ID 1.
    /// <https://chainlist.org/chain/1>
    /// <https://ethereum.org/en/developers/docs/apis/json-rpc/#net_version>
    pub EthereumNetwork: NetworkId = NetworkId::Ethereum { chain_id: 11155111 };

    pub ParentWithEthereumInboundQueueInstance: Location = Location::new(
        1,
        [
            PalletInstance(PARENT_INBOUND_QUEUE_PALLET_INDEX)
        ]
    );

    /// Universal aliases common to frontier and simple templates.
    pub CommonUniversalAliases: BTreeSet<(Location, Junction)> = BTreeSet::from_iter(
        alloc::vec![
            (ParentWithEthereumInboundQueueInstance::get(), GlobalConsensus(EthereumNetwork::get()))
        ]
    );
}

impl Contains<(Location, Junction)> for CommonUniversalAliases {
    fn contains(alias: &(Location, Junction)) -> bool {
        CommonUniversalAliases::get().contains(alias)
    }
}

#[cfg(feature = "runtime-benchmarks")]
pub struct AliasingBenchmarksHelper;

#[cfg(feature = "runtime-benchmarks")]
impl AliasingBenchmarksHelper {
    pub fn prepare_universal_alias() -> Option<(Location, Junction)> {
        let alias = CommonUniversalAliases::get()
            .into_iter()
            .find_map(|(location, junction)| {
                match ParentWithEthereumInboundQueueInstance::get().eq(&location) {
                    true => Some((location, junction)),
                    false => None,
                }
            });
        Some(alias.expect("Tanssi's InboundQueue to container-chain mapping expected here"))
    }
}
