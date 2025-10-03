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
    dancebox_runtime::{OriginCaller, PolkadotXcm},
    dancebox_runtime_test_utils::*,
    frame_support::assert_ok,
    xcm::latest::prelude::*,
    xcm::{VersionedAssets, VersionedLocation},
};

#[test]
fn test_dry_run_call_transfer_assets() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets tokens for her tests
            (AccountId::from(ALICE), 1_000_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            sp_tracing::try_init_simple();

            // This must be set because in typescript dev tests the xcm version is already set, while
            // in rust integration tests it is not set and that gives error SendError
            assert_ok!(PolkadotXcm::force_default_xcm_version(
                root_origin(),
                Some(4u32)
            ));

            // Similar to the above test but with different expectations
            let balances_pallet_index = 10u8; // Balances pallet index in dancebox runtime
            let random_receiver: [u8; 32] = [0x22; 32];

            let versioned_beneficiary = VersionedLocation::V4(xcm::v4::Location::new(
                0,
                [xcm::v4::Junction::AccountId32 {
                    network: None,
                    id: random_receiver,
                }],
            ));

            let versioned_assets =
                VersionedAssets::V4(xcm::v4::Assets::from(vec![xcm::v4::Asset {
                    id: xcm::v4::AssetId(xcm::v4::Location::new(
                        0,
                        [xcm::v4::Junction::PalletInstance(balances_pallet_index)],
                    )),
                    fun: xcm::v4::Fungibility::Fungible(100_000_000_000_000u128), // 0.1 UNIT
                }]));

            let dest = VersionedLocation::V4(xcm::v4::Location::new(1, xcm::v4::Junctions::Here));

            let call = RuntimeCall::PolkadotXcm(pallet_xcm::Call::transfer_assets {
                dest: Box::new(dest),
                beneficiary: Box::new(versioned_beneficiary),
                assets: Box::new(versioned_assets),
                fee_asset_item: 0,
                weight_limit: WeightLimit::Unlimited,
            });

            let origin_caller =
                OriginCaller::system(frame_system::RawOrigin::Signed(AccountId::from(ALICE)));
            let xcm_version = 4;

            // This works because we are currently inside block 1, after on_initialize
            /*
            use sp_runtime::traits::Dispatchable;
            use dancebox_runtime::RuntimeOrigin;
            call.dispatch(RuntimeOrigin::signed(AccountId::from(ALICE))).unwrap();
            return;
             */

            // Execute the dry run using PolkadotXcm directly
            let result = PolkadotXcm::dry_run_call::<
                Runtime,
                dancebox_runtime::xcm_config::XcmRouter,
                OriginCaller,
                RuntimeCall,
            >(origin_caller, call, xcm_version);

            // This also works because we are currently inside block 1, after on_initialize
            // The typescript test fails because it runs the dryRunCall after block 0 on_finalize, and
            // something is missing there that it makes the test fail
            match result {
                Ok(dry_run_effects) => {
                    println!("DryRun completed with effects: {:?}", dry_run_effects);
                    assert!(dry_run_effects.execution_result.is_ok());
                }
                Err(e) => {
                    panic!("DryRun API call failed: {:?}", e);
                }
            }
        });
}
