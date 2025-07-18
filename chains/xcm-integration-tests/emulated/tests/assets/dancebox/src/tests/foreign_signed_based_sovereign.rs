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
    dancebox_runtime::UNIT as DANCE,
    frame_support::{
        assert_ok,
        weights::{Weight, WeightToFee},
    },
    frontier_template_emulated_chain::{
        EthereumEmptyReceiver, EthereumSender, FrontierTemplateParaPallet,
    },
    westend_emulated_chain::WestendRelayPallet,
    westend_system_emulated_network::westend_emulated_chain::westend_runtime::Dmp,
    westend_system_emulated_network::{
        DanceboxEmptyReceiver, DanceboxPara as Dancebox, DanceboxSender,
        FrontierTemplatePara as FrontierTemplate, WestendRelay as Westend, WestendSender,
    },
    xcm::{
        latest::prelude::{Junctions::*, *},
        opaque::latest::WESTEND_GENESIS_HASH,
        VersionedLocation, VersionedXcm,
    },
    xcm_emulator::{assert_expected_events, bx, Chain, Parachain, TestExt},
    xcm_executor::traits::ConvertLocation,
};

#[test]
fn using_signed_based_sovereign_works_in_tanssi() {
    Westend::execute_with(|| {
        Dmp::make_parachain_reachable(Dancebox::para_id());
    });

    // XcmPallet send arguments
    let root_origin = <Westend as Chain>::RuntimeOrigin::root();
    let dancebox_dest: VersionedLocation = Location {
        parents: 0,
        interior: X1([Parachain(2000u32)].into()),
    }
    .into();

    let buy_execution_fee = Asset {
        id: dancebox_runtime::xcm_config::SelfReserve::get().into(),
        fun: Fungible(50 * DANCE),
    };

    let xcm = VersionedXcm::from(Xcm(vec![
        DescendOrigin(X1([AccountId32 {
            network: None,
            id: WestendSender::get().into(),
        }]
        .into())),
        WithdrawAsset(vec![buy_execution_fee.clone()].into()),
        BuyExecution {
            fees: buy_execution_fee.clone(),
            weight_limit: Unlimited,
        },
        DepositAsset {
            assets: Wild(AllCounted(1)),
            beneficiary: AccountId32 {
                network: None,
                id: DanceboxEmptyReceiver::get().into(),
            }
            .into(),
        },
    ]));

    let alice_westend_account_dancebox = xcm_builder::HashedDescription::<
        dancebox_runtime::AccountId,
        xcm_builder::DescribeFamily<xcm_builder::DescribeAllTerminal>,
    >::convert_location(&Location {
        parents: 1,
        interior: X1([AccountId32 {
            network: Some(NetworkId::ByGenesis(WESTEND_GENESIS_HASH)),
            id: WestendSender::get().into(),
        }]
        .into()),
    })
    .unwrap();

    // Send some tokens to the account derived fromt the signed origin
    Dancebox::execute_with(|| {
        let origin = <Dancebox as Chain>::RuntimeOrigin::signed(DanceboxSender::get());

        assert_ok!(
            <Dancebox as DanceboxParaPallet>::Balances::transfer_allow_death(
                origin,
                sp_runtime::MultiAddress::Id(alice_westend_account_dancebox),
                100 * DANCE
            )
        );
    });

    // Send XCM message from Westend
    Westend::execute_with(|| {
        assert_ok!(<Westend as WestendRelayPallet>::XcmPallet::send(
            root_origin,
            bx!(dancebox_dest),
            bx!(xcm),
        ));
    });

    // Send XCM message from Dancebox
    Dancebox::execute_with(|| {
        Dancebox::assert_dmp_queue_complete(None);
        // Assert empty receiver received funds
        assert!(
            <Dancebox as DanceboxParaPallet>::System::account(DanceboxEmptyReceiver::get())
                .data
                .free
                > 0
        );
    });
}

#[test]
fn using_signed_based_sovereign_works_from_tanssi_to_frontier_template() {
    // XcmPallet send arguments
    let alice_origin = <Dancebox as Chain>::RuntimeOrigin::signed(DanceboxSender::get());

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
        // We also need to transfer first sufficient amount to the signed-based sovereign

        let alice_dancebox_account_frontier = xcm_builder::HashedDescription::<
            container_chain_template_frontier_runtime::AccountId,
            xcm_builder::DescribeFamily<xcm_builder::DescribeAllTerminal>,
        >::convert_location(&Location {
            parents: 1,
            interior: X2([
                Parachain(2000u32),
                AccountId32 {
                    network: Some(NetworkId::ByGenesis(WESTEND_GENESIS_HASH)),
                    id: DanceboxSender::get().into(),
                },
            ]
            .into()),
        })
        .unwrap();

        let origin = <FrontierTemplate as Chain>::RuntimeOrigin::signed(EthereumSender::get());
        assert_ok!(
            <FrontierTemplate as FrontierTemplateParaPallet>::Balances::transfer_allow_death(
                origin,
                alice_dancebox_account_frontier,
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
            alice_origin,
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
