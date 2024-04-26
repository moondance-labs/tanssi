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
    crate::assert_expected_events,
    crate::common::xcm::{
        mocknets::{
            DanceboxEmptyReceiver, DanceboxPara as Dancebox, DanceboxParaPallet, DanceboxSender,
            EthereumEmptyReceiver, EthereumSender, FrontierTemplatePara as FrontierTemplate,
            FrontierTemplateParaPallet, WestendRelay as Westend, WestendRelayPallet, WestendSender,
        },
        *,
    },
    container_chain_template_frontier_runtime::currency::UNIT as FRONTIER_DEV,
    dancebox_runtime::UNIT as DANCE,
    frame_support::{
        assert_ok,
        weights::{Weight, WeightToFee},
    },
    staging_xcm::{latest::prelude::*, VersionedMultiLocation, VersionedXcm},
    staging_xcm_executor::traits::ConvertLocation,
    xcm_emulator::Chain,
};

#[test]
fn using_signed_based_sovereign_works_in_tanssi() {
    // XcmPallet send arguments
    let root_origin = <Westend as Chain>::RuntimeOrigin::root();
    let dancebox_dest: VersionedMultiLocation = MultiLocation {
        parents: 0,
        interior: X1(Parachain(2000u32)),
    }
    .into();

    let buy_execution_fee = MultiAsset {
        id: Concrete(dancebox_runtime::xcm_config::SelfReserve::get()),
        fun: Fungible(50 * DANCE),
    };

    let xcm = VersionedXcm::from(Xcm(vec![
        DescendOrigin(X1(AccountId32 {
            network: None,
            id: WestendSender::get().into(),
        })),
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

    let alice_westend_account_dancebox = staging_xcm_builder::HashedDescription::<
        crate::AccountId,
        staging_xcm_builder::DescribeFamily<staging_xcm_builder::DescribeAllTerminal>,
    >::convert_location(&MultiLocation {
        parents: 1,
        interior: X1(AccountId32 {
            network: Some(NetworkId::Westend),
            id: WestendSender::get().into(),
        }),
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

    let frontier_destination: VersionedMultiLocation = MultiLocation {
        parents: 1,
        interior: X1(Parachain(2001)),
    }
    .into();

    let buy_execution_fee_amount =
        container_chain_template_frontier_runtime::WeightToFee::weight_to_fee(&Weight::from_parts(
            10_000_000_000,
            300_000,
        ));

    let buy_execution_fee = MultiAsset {
        id: Concrete(container_chain_template_frontier_runtime::xcm_config::SelfReserve::get()),
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

        let alice_dancebox_account_frontier = staging_xcm_builder::HashedDescription::<
            container_chain_template_frontier_runtime::AccountId,
            staging_xcm_builder::DescribeFamily<staging_xcm_builder::DescribeAllTerminal>,
        >::convert_location(&MultiLocation {
            parents: 1,
            interior: X2(
                Parachain(2000u32),
                AccountId32 {
                    network: Some(NetworkId::Westend),
                    id: DanceboxSender::get().into(),
                },
            ),
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
