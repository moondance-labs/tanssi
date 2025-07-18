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

use {
    frame_support::{
        assert_ok,
        weights::{Weight, WeightToFee},
    },
    westend_emulated_chain::WestendRelayPallet,
    westend_system_emulated_network::westend_emulated_chain::westend_runtime::Dmp,
    westend_system_emulated_network::{DanceboxPara as Dancebox, WestendRelay as Westend},
    xcm::{latest::prelude::*, VersionedLocation, VersionedXcm},
    xcm_emulator::{
        assert_expected_events, bx, Chain, Parachain as Para, RelayChain as Relay, TestExt,
    },
};

#[test]
fn trapping_asserts_works_with_polkadot_xcm() {
    Westend::execute_with(|| {
        Dmp::make_parachain_reachable(Dancebox::para_id());
    });

    // XcmPallet send arguments
    let sudo_origin = <Westend as Chain>::RuntimeOrigin::root();
    let dancebox_para_destination: VersionedLocation =
        Westend::child_location_of(Dancebox::para_id()).into();

    let buy_execution_fee_amount =
        dancebox_runtime::WeightToFee::weight_to_fee(&Weight::from_parts(10_000_000_000, 300_000));

    let buy_execution_fee = Asset {
        id: dancebox_runtime::xcm_config::SelfReserve::get().into(),
        fun: Fungible(buy_execution_fee_amount),
    };

    let xcm = VersionedXcm::from(Xcm(vec![
        WithdrawAsset(vec![buy_execution_fee.clone()].into()),
        BuyExecution {
            fees: buy_execution_fee.clone(),
            weight_limit: Unlimited,
        },
        Trap(0),
    ]));

    // Send XCM message from Relay Chain
    Westend::execute_with(|| {
        assert_ok!(<Westend as WestendRelayPallet>::XcmPallet::send(
            sudo_origin,
            bx!(dancebox_para_destination),
            bx!(xcm),
        ));

        type RuntimeEvent = <Westend as Chain>::RuntimeEvent;

        assert_expected_events!(
            Westend,
            vec![
                RuntimeEvent::XcmPallet(pallet_xcm::Event::Sent { .. }) => {},
            ]
        );
    });

    // Receive XCM message in Assets Parachain
    Dancebox::execute_with(|| {
        type RuntimeEvent = <Dancebox as Chain>::RuntimeEvent;
        Dancebox::assert_dmp_queue_incomplete(None);
        assert_expected_events!(
            Dancebox,
            vec![
                RuntimeEvent::PolkadotXcm(
                    pallet_xcm::Event::AssetsTrapped{origin, ..}) => {
                        origin: *origin == Location::parent(),
                },
            ]
        );
    });
}
