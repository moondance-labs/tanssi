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
    container_chain_template_frontier_runtime::currency::UNIT as FRONTIER_DEV,
    dancebox_emulated_chain::DanceboxParaPallet,
    frame_support::{
        assert_ok,
        weights::{Weight, WeightToFee},
    },
    frontier_template_emulated_chain::{
        EthereumEmptyReceiver, EthereumSender, FrontierTemplateParaPallet,
    },
    westend_emulated_chain::WestendRelayPallet,
    westend_runtime_constants::currency::UNITS as WND,
    westend_system_emulated_network::{
        DanceboxPara as Dancebox, FrontierTemplatePara as FrontierTemplate, WestendEmptyReceiver,
        WestendRelay as Westend, WestendSender,
    },
    xcm::{
        latest::prelude::{Junctions::X1, *},
        VersionedLocation, VersionedXcm,
    },
    xcm_emulator::{assert_expected_events, bx, Chain, TestExt},
    xcm_executor::traits::ConvertLocation,
};

#[test]
fn using_sovereign_works_from_tanssi() {
    // XcmPallet send arguments
    let sudo_origin = <Dancebox as Chain>::RuntimeOrigin::root();
    let relay_destination: VersionedLocation = Location::parent().into();

    let buy_execution_fee_amount = westend_runtime_constants::fee::WeightToFee::weight_to_fee(
        &Weight::from_parts(10_000_000_000, 300_000),
    );

    let buy_execution_fee = Asset {
        id: Location::here().into(),
        fun: Fungible(buy_execution_fee_amount),
    };

    let xcm = VersionedXcm::from(Xcm(vec![
        WithdrawAsset(vec![buy_execution_fee.clone()].into()),
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
            westend_runtime::xcm_config::LocationConverter::convert_location(&Location {
                parents: 0,
                interior: X1([Parachain(2000u32)].into()),
            })
            .unwrap();

        let origin = <Westend as Chain>::RuntimeOrigin::signed(WestendSender::get());
        assert_ok!(
            <Westend as WestendRelayPallet>::Balances::transfer_allow_death(
                origin,
                sp_runtime::MultiAddress::Id(sovereign_account),
                100 * WND
            )
        );
        // Assert empty receiver has 0 funds
        assert_eq!(
            <Westend as Chain>::System::account(WestendEmptyReceiver::get())
                .data
                .free,
            0
        );
    });

    // Send XCM message from Dancebox
    Dancebox::execute_with(|| {
        assert_ok!(<Dancebox as DanceboxParaPallet>::PolkadotXcm::send(
            sudo_origin,
            bx!(relay_destination),
            bx!(xcm),
        ));

        type RuntimeEvent = <Dancebox as Chain>::RuntimeEvent;

        assert_expected_events!(
            Dancebox,
            vec![
                RuntimeEvent::PolkadotXcm(pallet_xcm::Event::Sent { .. }) => {},
            ]
        );
    });

    // Receive XCM message in Relay
    Westend::execute_with(|| {
        type RuntimeEvent = <Westend as Chain>::RuntimeEvent;
        assert_expected_events!(
            Westend,
            vec![
                RuntimeEvent::MessageQueue(
                    pallet_message_queue::Event::Processed {
                        success,
                        ..
                    }) => {
                    success: *success,
                },
            ]
        );
        // Assert empty receiver received funds
        assert!(
            <Westend as Chain>::System::account(WestendEmptyReceiver::get())
                .data
                .free
                > 0
        );
    });
}

#[test]
fn using_sovereign_works_from_tanssi_frontier_template() {
    // XcmPallet send arguments
    let sudo_origin = <Dancebox as Chain>::RuntimeOrigin::root();
    let frontier_destination: VersionedLocation = Location {
        parents: 1,
        interior: X1([Parachain(2001)].into()),
    }
    .into();

    let buy_execution_fee_amount =
        container_chain_template_frontier_runtime::WeightToFee::weight_to_fee(&Weight::from_parts(
            10_000_000_000,
            300_000,
        ));

    let buy_execution_fee = Asset {
        id: container_chain_template_frontier_runtime::xcm_config::SelfReserve::get().into(),
        fun: Fungible(buy_execution_fee_amount),
    };

    let xcm = VersionedXcm::from(Xcm(vec![
        WithdrawAsset(vec![buy_execution_fee.clone()].into()),
        BuyExecution {
            fees: buy_execution_fee.clone(),
            weight_limit: Unlimited,
        },
        DepositAsset {
            assets: Wild(AllCounted(1)),
            beneficiary: AccountKey20 {
                network: None,
                key: EthereumEmptyReceiver::get().into(),
            }
            .into(),
        },
    ]));

    FrontierTemplate::execute_with(|| {
        // We also need to transfer first sufficient amount to the sovereign
        let sovereign_account =
            container_chain_template_frontier_runtime::xcm_config::LocationToAccountId::convert_location(&Location {
                parents: 1,
                interior: X1([Parachain(2000u32)].into()),
            })
            .unwrap();

        let origin = <FrontierTemplate as Chain>::RuntimeOrigin::signed(EthereumSender::get());
        assert_ok!(
            <FrontierTemplate as FrontierTemplateParaPallet>::Balances::transfer_allow_death(
                origin,
                sovereign_account,
                100 * FRONTIER_DEV
            )
        );
        // Assert empty receiver has 0 funds
        assert_eq!(
            <FrontierTemplate as FrontierTemplateParaPallet>::System::account(
                EthereumEmptyReceiver::get()
            )
            .data
            .free,
            0
        );
    });

    // Send XCM message from Dancebox
    Dancebox::execute_with(|| {
        assert_ok!(<Dancebox as DanceboxParaPallet>::PolkadotXcm::send(
            sudo_origin,
            bx!(frontier_destination),
            bx!(xcm),
        ));

        type RuntimeEvent = <Dancebox as Chain>::RuntimeEvent;

        assert_expected_events!(
            Dancebox,
            vec![
                RuntimeEvent::PolkadotXcm(pallet_xcm::Event::Sent { .. }) => {},
                RuntimeEvent::XcmpQueue(cumulus_pallet_xcmp_queue::Event::XcmpMessageSent { .. }) => {},
            ]
        );
    });

    FrontierTemplate::execute_with(|| {
        FrontierTemplate::assert_xcmp_queue_success(None);
        // Assert empty receiver received funds
        assert!(
            <FrontierTemplate as FrontierTemplateParaPallet>::System::account(
                EthereumEmptyReceiver::get()
            )
            .data
            .free
                > 0
        );
    });
}
