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
            Dancebox, DanceboxEmptyReceiver, DanceboxPallet, DanceboxSender, EthereumEmptyReceiver,
            EthereumSender, FrontierTemplate, Westend, WestendEmptyReceiver, WestendPallet,
            WestendSender,
        },
        *,
    },
    container_chain_template_frontier_runtime::UNIT as FRONTIER_DEV,
    dancebox_runtime::UNIT as DANCE,
    frame_support::{
        assert_ok,
        weights::{Weight, WeightToFee},
    },
    westend_runtime_constants::currency::UNITS as WND,
    xcm::{latest::prelude::*, VersionedMultiLocation, VersionedXcm},
    xcm_executor::traits::Convert,
};

#[test]
fn using_signed_based_sovereign_works_in_tanssi() {
    // XcmPallet send arguments
    let alice_origin = <Westend as Relay>::RuntimeOrigin::signed(WestendSender::get());
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
                id: DanceboxEmptyReceiver::get().into(),
            }
            .into(),
        },
    ]));

    let alice_westend_account_dancebox = xcm_builder::HashedDescriptionDescribeFamilyAllTerminal::<
        crate::AccountId,
    >::convert_ref(MultiLocation {
        parents: 1,
        interior: X1(AccountId32 {
            network: Some(NetworkId::Westend),
            id: WestendSender::get().into(),
        }),
    })
    .unwrap();

    // Send some tokens to the account derived fromt the signed origin
    Dancebox::execute_with(|| {
        let origin = <Dancebox as Para>::RuntimeOrigin::signed(DanceboxSender::get());

        assert_ok!(<Dancebox as Para>::Balances::transfer(
            origin,
            sp_runtime::MultiAddress::Id(alice_westend_account_dancebox),
            100 * DANCE
        ));
    });

    // Send XCM message from Westend
    Westend::execute_with(|| {
        assert_ok!(<Westend as WestendPallet>::XcmPallet::send(
            alice_origin,
            bx!(dancebox_dest),
            bx!(xcm),
        ));
    });

    // Send XCM message from Dancebox
    Dancebox::execute_with(|| {
        type RuntimeEvent = <Dancebox as Para>::RuntimeEvent;
        assert_expected_events!(
            Dancebox,
            vec![
                RuntimeEvent::DmpQueue(
                    cumulus_pallet_dmp_queue::Event::ExecutedDownward {
                        outcome, ..
                    }) => {
                    outcome: outcome.clone().ensure_complete().is_ok(),
                },
            ]
        );
        // Assert empty receiver received funds
        assert!(
            <Dancebox as Para>::System::account(DanceboxEmptyReceiver::get())
                .data
                .free
                > 0
        );
    });
}
