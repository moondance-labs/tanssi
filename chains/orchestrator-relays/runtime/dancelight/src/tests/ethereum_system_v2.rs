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

#![cfg(test)]

use {
    crate::{tests::common::*, EthereumSystemV2},
    alloc::vec,
    dancelight_runtime_constants::DANCELIGHT_GENESIS_HASH,
    frame_support::{assert_noop, assert_ok, error::BadOrigin},
    xcm::{
        latest::{prelude::*, Junctions::*, Location},
        VersionedLocation,
    },
};

#[test]
fn test_sudo_can_register_ethereum_system_v2() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);
            let token_location: VersionedLocation = Location::here().into();
            let reanchored_location = Location {
                parents: 1,
                interior: X1([GlobalConsensus(NetworkId::ByGenesis(
                    DANCELIGHT_GENESIS_HASH,
                ))]
                .into()),
            };

            // Even though the register is done through ethereum systemv2, the storage
            // is from v1
            assert_ok!(EthereumSystemV2::register_token(
                root_origin(),
                Box::new(token_location.clone()),
                Box::new(token_location),
                snowbridge_core::AssetMetadata {
                    name: "dance".as_bytes().to_vec().try_into().unwrap(),
                    symbol: "dance".as_bytes().to_vec().try_into().unwrap(),
                    decimals: 12,
                }
            ));

            let received_token_id =
                snowbridge_pallet_system::NativeToForeignId::<Runtime>::get(&reanchored_location);
            assert!(received_token_id.is_some());
            assert_eq!(
                snowbridge_pallet_system::ForeignToNativeId::<Runtime>::get(
                    received_token_id.unwrap()
                ),
                Some(reanchored_location)
            );
        });
}
#[test]
fn nobody_else_can_register_ethereum_v2() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);
            let token_location: VersionedLocation = Location::here().into();

            // Even though the register is done through ethereum systemv2, the storage
            // is from v1
            assert_noop!(
                EthereumSystemV2::register_token(
                    origin_of(ALICE.into()),
                    Box::new(token_location.clone()),
                    Box::new(token_location),
                    snowbridge_core::AssetMetadata {
                        name: "dance".as_bytes().to_vec().try_into().unwrap(),
                        symbol: "dance".as_bytes().to_vec().try_into().unwrap(),
                        decimals: 12,
                    }
                ),
                BadOrigin
            );
        });
}
