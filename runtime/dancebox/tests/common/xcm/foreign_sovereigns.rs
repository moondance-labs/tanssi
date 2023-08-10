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
    crate::common::xcm::{
        mocknets::{
            Dancebox, DanceboxPallet, Westend, WestendEmptyReceiver, WestendPallet, WestendSender,
        },
        *,
    },
    frame_support::{
        assert_ok,
        weights::{Weight, WeightToFee},
    },
    parity_scale_codec::Encode,
    sp_core::Get,
    westend_runtime_constants::currency::UNITS as WND,
    xcm::{
        latest::{prelude::*, Error::Trap as TrapError},
        VersionedMultiLocation, VersionedXcm,
    },
    xcm_executor::traits::Convert,
};

#[test]
fn using_sovereign_works_from_tanssi() {
    // XcmPallet send arguments
    let sudo_origin = <Dancebox as Para>::RuntimeOrigin::root();
    let relay_destination: VersionedMultiLocation = MultiLocation::parent().into();

    let buy_execution_fee_amount = westend_runtime_constants::fee::WeightToFee::weight_to_fee(
        &Weight::from_parts(10_000_000_000, 300_000),
    );

    let buy_execution_fee = MultiAsset {
        id: Concrete(MultiLocation::here()),
        fun: Fungible(buy_execution_fee_amount),
    };

    let xcm = VersionedXcm::from(Xcm(vec![
        WithdrawAsset {
            0: vec![buy_execution_fee.clone()].into(),
        },
        BuyExecution {
            fees: buy_execution_fee.clone(),
            weight_limit: Unlimited,
        },
        DepositAsset {
            assets: Wild(AllCounted(1)),
            beneficiary: AccountId32 {
                network: None,
                id: WestendEmptyReceiver::get().into(),
            }
            .into(),
        },
    ]));

    Westend::execute_with(|| {
        // We also need to transfer first sufficient amount to the sovereign
        let sovereign_account =
            westend_runtime::xcm_config::LocationConverter::convert_ref(MultiLocation {
                parents: 0,
                interior: X1(Parachain(<Dancebox as Para>::ParachainInfo::get().into())),
            })
            .unwrap();

        let origin = <Westend as Relay>::RuntimeOrigin::signed(WestendSender::get());
        assert_ok!(<Westend as Relay>::Balances::transfer(
            origin,
            sp_runtime::MultiAddress::Id(sovereign_account),
            100 * WND
        ));
        // Assert empty receiver has 0 funds
        assert_eq!(
            <Westend as Relay>::System::account(WestendEmptyReceiver::get())
                .data
                .free,
            0
        );
    });

    // Send XCM message from Dancebox
    Dancebox::execute_with(|| {
        assert_ok!(<Dancebox as DanceboxPallet>::PolkadotXcm::send(
            sudo_origin,
            bx!(relay_destination),
            bx!(xcm),
        ));

        type RuntimeEvent = <Dancebox as Para>::RuntimeEvent;

        assert_expected_events!(
            Dancebox,
            vec![
                RuntimeEvent::PolkadotXcm(pallet_xcm::Event::Sent { .. }) => {},
            ]
        );
    });

    // Receive XCM message in Relay
    Westend::execute_with(|| {
        type RuntimeEvent = <Westend as Relay>::RuntimeEvent;
        assert_expected_events!(
            Westend,
            vec![
                RuntimeEvent::MessageQueue(
                    pallet_message_queue::Event::Processed {
                        success,
                        ..
                    }) => {
                    success: *success == true,
                },
            ]
        );
        // Assert empty receiver received funds
        assert!(
            <Westend as Relay>::System::account(WestendEmptyReceiver::get())
                .data
                .free
                > 0
        );
    });
}
