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
    crate::PhantomData, parity_scale_codec::Encode, sp_core::blake2_256, xcm::latest::prelude::*,
    xcm_executor::traits::ConvertLocation,
};

pub struct ContainerChainEthereumLocationConverter<AccountId>(PhantomData<AccountId>);

impl<AccountId> ConvertLocation<AccountId> for ContainerChainEthereumLocationConverter<AccountId>
where
    AccountId: From<[u8; 32]> + Clone,
{
    fn convert_location(location: &Location) -> Option<AccountId> {
        match location.unpack() {
            (1, [GlobalConsensus(Ethereum { chain_id })]) => {
                Some(Self::from_chain_id(chain_id).into())
            }
            (2, [GlobalConsensus(Ethereum { chain_id })]) => {
                Some(Self::from_chain_id(chain_id).into())
            }
            _ => None,
        }
    }
}

impl<AccountId> ContainerChainEthereumLocationConverter<AccountId> {
    pub fn from_chain_id(chain_id: &u64) -> [u8; 32] {
        (b"ethereum-chain", chain_id).using_encoded(blake2_256)
    }
    pub fn from_chain_id_with_key(chain_id: &u64, key: [u8; 20]) -> [u8; 32] {
        (b"ethereum-chain", chain_id, key).using_encoded(blake2_256)
    }
}
