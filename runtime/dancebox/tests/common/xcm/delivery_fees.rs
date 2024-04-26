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
            DanceboxPara as Dancebox, DanceboxParaPallet, DanceboxSender,
            EthereumSender as FrontierTemplateSender, FrontierTemplatePara as FrontierTemplate,
            FrontierTemplateParaPallet, SimpleTemplatePara as SimpleTemplate,
            SimpleTemplateParaPallet, SimpleTemplateSender,
        },
        *,
    },
    frame_support::assert_ok,
    frame_support::traits::EnsureOrigin,
    paste::paste,
    staging_xcm::{latest::prelude::*, VersionedMultiLocation, VersionedXcm},
    xcm_emulator::Chain,
};

#[test]
fn ump_delivery_fees_charged_dancebox() {
    let dest = MultiLocation::parent();
    // Send XCM message from Dancebox
    Dancebox::execute_with(|| {
        crate::assert_delivery_fees_test!(Dancebox, dest);
    });
}

#[test]
fn ump_delivery_fees_charged_simple_template() {
    let dest = MultiLocation::parent();

    // Send XCM message from SimpleTemplate
    SimpleTemplate::execute_with(|| {
        crate::assert_delivery_fees_test!(SimpleTemplate, dest);
    });
}

#[test]
fn ump_delivery_fees_charged_frontier_template() {
    let dest = MultiLocation::parent();

    // Send XCM message from FrontierTemplate
    FrontierTemplate::execute_with(|| {
        crate::assert_delivery_fees_test!(FrontierTemplate, dest);
    });
}

#[test]
fn hrmp_delivery_fees_charged_dancebox() {
    let dest = MultiLocation::new(1, X1(Parachain(2001)));
    // Send XCM message from Dancebox
    Dancebox::execute_with(|| {
        crate::assert_delivery_fees_test!(Dancebox, dest);
    });
}

#[test]
fn hrmp_delivery_fees_charged_simple_template() {
    let dest = MultiLocation::new(1, X1(Parachain(2000)));

    // Send XCM message from SimpleTemplate
    SimpleTemplate::execute_with(|| {
        crate::assert_delivery_fees_test!(SimpleTemplate, dest);
    });
}

#[test]
fn hrmp_delivery_fees_charged_frontier_template() {
    let dest = MultiLocation::new(1, X1(Parachain(2000)));

    // Send XCM message from FrontierTemplate
    FrontierTemplate::execute_with(|| {
        crate::assert_delivery_fees_test!(FrontierTemplate, dest);
    });
}

#[macro_export]
macro_rules! assert_delivery_fees_test {
    ( $chain:ident, $dest:ident ) => {
        paste! {
            type RuntimeEvent = <$chain as Chain>::RuntimeEvent;
            let xcm = Xcm(vec![
                RefundSurplus,
            ]);

            let versioned_xcm: VersionedXcm<()> = VersionedXcm::V3(xcm.clone());
            let sender_account =  [<$chain Sender>]::get();

            let balance_sender_before = <$chain as [<$chain ParaPallet>]>::Balances::free_balance(sender_account.clone());

            let origin = <$chain as Chain>::RuntimeOrigin::signed(sender_account.clone());
            let origin_location = <<$chain as Chain>::Runtime as pallet_xcm::Config>::SendXcmOrigin::ensure_origin(origin.clone()).expect("cannot conver origin into junctions");
			let interior: Junctions =
				origin_location.clone().try_into().unwrap();

            let final_xcm: Xcm<()> = Xcm(vec![
                DescendOrigin(interior),
                RefundSurplus,
            ]);
            let dest: VersionedMultiLocation = $dest.into();

            assert_ok!(<$chain as [<$chain ParaPallet>]>::PolkadotXcm::send(
                origin,
                bx!(dest),
                bx!(versioned_xcm)
            ));
            let (_, price) = validate_send::<<<$chain as Chain>::Runtime as pallet_xcm::Config>::XcmRouter>(MultiLocation::parent(), final_xcm.clone()).unwrap();
            let balance_sender_after = <$chain as [<$chain ParaPallet>]>::Balances::free_balance(&sender_account);
            assert!(balance_sender_after < balance_sender_before);
            // assert there is at least an asset
            assert!(!price.is_none());

            assert_expected_events!(
                $chain,
                vec![
                    RuntimeEvent::PolkadotXcm(pallet_xcm::Event::FeesPaid { paying: _, fees }) => {
                        fees: *fees == price,
                    },
                ]
            );
            // We check the first asset, and make sure we at least have charged the fees
            match price.into_inner().first().unwrap().fun {
                Fungible(amount) => assert!(balance_sender_after <= balance_sender_before - amount),
                _ => panic!("Charged amount should be fungible")
            };
        }
    };
}
