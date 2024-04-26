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

use crate::common::xcm::*;

use {
    crate::assert_expected_events,
    crate::common::xcm::mocknets::{
        DanceboxPara as Dancebox, FrontierTemplatePara as FrontierTemplate,
        FrontierTemplateParaPallet, SimpleTemplatePara as SimpleTemplate, SimpleTemplateParaPallet,
        WestendRelay as Westend, WestendRelayPallet,
    },
    frame_support::{
        assert_ok,
        weights::{Weight, WeightToFee},
    },
    parity_scale_codec::Encode,
    staging_xcm::{latest::prelude::*, VersionedMultiLocation, VersionedXcm},
    staging_xcm_builder::{ParentIsPreset, SiblingParachainConvertsVia},
    staging_xcm_executor::traits::ConvertLocation,
    xcm_emulator::Chain,
};

#[test]
fn transact_sudo_from_relay_hits_barrier_dancebox_without_buy_exec() {
    let call = <Dancebox as Chain>::RuntimeCall::Configuration(pallet_configuration::Call::<
        <Dancebox as Chain>::Runtime,
    >::set_max_collators {
        new: 50,
    })
    .encode()
    .into();

    // XcmPallet send arguments
    let sudo_origin = <Westend as Chain>::RuntimeOrigin::root();
    let dancebox_para_destination: VersionedMultiLocation =
        Westend::child_location_of(Dancebox::para_id()).into();

    let weight_limit = WeightLimit::Unlimited;
    let require_weight_at_most = Weight::from_parts(1000000000, 200000);
    let origin_kind = OriginKind::Superuser;
    let check_origin = None;

    let xcm = VersionedXcm::from(Xcm(vec![
        UnpaidExecution {
            weight_limit,
            check_origin,
        },
        Transact {
            require_weight_at_most,
            origin_kind,
            call,
        },
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
        Dancebox::assert_dmp_queue_error();
    });
}

#[test]
fn transact_sudo_from_relay_does_not_have_sudo_power() {
    let call = <Dancebox as Chain>::RuntimeCall::Configuration(pallet_configuration::Call::<
        <Dancebox as Chain>::Runtime,
    >::set_max_collators {
        new: 50,
    })
    .encode()
    .into();

    // XcmPallet send arguments
    let sudo_origin = <Westend as Chain>::RuntimeOrigin::root();
    let dancebox_para_destination: VersionedMultiLocation =
        Westend::child_location_of(Dancebox::para_id()).into();

    let require_weight_at_most = Weight::from_parts(1000000000, 200000);
    let origin_kind = OriginKind::Superuser;

    let buy_execution_fee_amount =
        dancebox_runtime::WeightToFee::weight_to_fee(&Weight::from_parts(10_000_000_000, 300_000));

    let buy_execution_fee = MultiAsset {
        id: Concrete(dancebox_runtime::xcm_config::SelfReserve::get()),
        fun: Fungible(buy_execution_fee_amount),
    };

    let xcm = VersionedXcm::from(Xcm(vec![
        WithdrawAsset(vec![buy_execution_fee.clone()].into()),
        BuyExecution {
            fees: buy_execution_fee.clone(),
            weight_limit: Unlimited,
        },
        Transact {
            require_weight_at_most,
            origin_kind,
            call,
        },
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
        Dancebox::assert_dmp_queue_incomplete(None);
    });
}

#[test]
fn transact_sudo_from_relay_has_signed_origin_powers() {
    let call = <Dancebox as Chain>::RuntimeCall::System(frame_system::Call::<
        <Dancebox as Chain>::Runtime,
    >::remark_with_event {
        remark: b"Test".to_vec(),
    })
    .encode()
    .into();

    // XcmPallet send arguments
    let sudo_origin = <Westend as Chain>::RuntimeOrigin::root();
    let dancebox_para_destination: VersionedMultiLocation =
        Westend::child_location_of(Dancebox::para_id()).into();

    let require_weight_at_most = Weight::from_parts(1000000000, 200000);
    let origin_kind = OriginKind::SovereignAccount;

    let buy_execution_fee_amount = dancebox_runtime::WeightToFee::weight_to_fee(
        &Weight::from_parts(10_000_000_000_000, 300_000),
    );

    let buy_execution_fee = MultiAsset {
        id: Concrete(dancebox_runtime::xcm_config::SelfReserve::get()),
        fun: Fungible(buy_execution_fee_amount),
    };

    let xcm = VersionedXcm::from(Xcm(vec![
        WithdrawAsset(vec![buy_execution_fee.clone()].into()),
        BuyExecution {
            fees: buy_execution_fee.clone(),
            weight_limit: Unlimited,
        },
        Transact {
            require_weight_at_most,
            origin_kind,
            call,
        },
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
        assert_expected_events!(
            Dancebox,
            vec![
                RuntimeEvent::System(
                    frame_system::Event::Remarked {
                        sender,
                        ..
                    }) => {
                    sender: *sender == ParentIsPreset::<dancebox_runtime::AccountId>::convert_location(&MultiLocation::parent()).unwrap(),
                },
            ]
        );
    });
}

#[test]
fn transact_sudo_from_frontier_has_signed_origin_powers() {
    let call = <Dancebox as Chain>::RuntimeCall::System(frame_system::Call::<
        <Dancebox as Chain>::Runtime,
    >::remark_with_event {
        remark: b"Test".to_vec(),
    })
    .encode()
    .into();

    // XcmPallet send arguments
    let sudo_origin = <FrontierTemplate as Chain>::RuntimeOrigin::root();
    let dancebox_para_destination: VersionedMultiLocation = MultiLocation {
        parents: 1,
        interior: X1(Parachain(Dancebox::para_id().into())),
    }
    .into();

    let require_weight_at_most = Weight::from_parts(1000000000, 200000);
    let origin_kind = OriginKind::SovereignAccount;

    let buy_execution_fee_amount = dancebox_runtime::WeightToFee::weight_to_fee(
        &Weight::from_parts(10_000_000_000_000, 300_000),
    );

    let buy_execution_fee = MultiAsset {
        id: Concrete(dancebox_runtime::xcm_config::SelfReserve::get()),
        fun: Fungible(buy_execution_fee_amount),
    };

    let xcm = VersionedXcm::from(Xcm(vec![
        WithdrawAsset(vec![buy_execution_fee.clone()].into()),
        BuyExecution {
            fees: buy_execution_fee.clone(),
            weight_limit: Unlimited,
        },
        Transact {
            require_weight_at_most,
            origin_kind,
            call,
        },
    ]));

    // Send XCM message from Frontier Template
    FrontierTemplate::execute_with(|| {
        assert_ok!(
            <FrontierTemplate as FrontierTemplateParaPallet>::PolkadotXcm::send(
                sudo_origin,
                bx!(dancebox_para_destination),
                bx!(xcm),
            )
        );

        type RuntimeEvent = <FrontierTemplate as Chain>::RuntimeEvent;

        assert_expected_events!(
            FrontierTemplate,
            vec![
                RuntimeEvent::PolkadotXcm(pallet_xcm::Event::Sent { .. }) => {},
            ]
        );
    });

    // Receive XCM message in Assets Parachain
    Dancebox::execute_with(|| {
        type RuntimeEvent = <Dancebox as Chain>::RuntimeEvent;
        assert_expected_events!(
            Dancebox,
            vec![
                RuntimeEvent::System(
                    frame_system::Event::Remarked {
                        sender,
                        ..
                    }) => {
                    sender: *sender ==  SiblingParachainConvertsVia::<polkadot_parachain_primitives::primitives::Sibling, crate::AccountId>::convert_location(
                        &MultiLocation{ parents: 1, interior: X1(Parachain(2001u32))}
                    ).unwrap(),
                },
            ]
        );
    });
}

#[test]
fn transact_sudo_from_simple_has_signed_origin_powers() {
    let call = <Dancebox as Chain>::RuntimeCall::System(frame_system::Call::<
        <Dancebox as Chain>::Runtime,
    >::remark_with_event {
        remark: b"Test".to_vec(),
    })
    .encode()
    .into();

    // XcmPallet send arguments
    let sudo_origin = <SimpleTemplate as Chain>::RuntimeOrigin::root();
    let dancebox_para_destination: VersionedMultiLocation = MultiLocation {
        parents: 1,
        interior: X1(Parachain(Dancebox::para_id().into())),
    }
    .into();

    let require_weight_at_most = Weight::from_parts(1000000000, 200000);
    let origin_kind = OriginKind::SovereignAccount;

    let buy_execution_fee_amount = dancebox_runtime::WeightToFee::weight_to_fee(
        &Weight::from_parts(10_000_000_000_000, 300_000),
    );

    let buy_execution_fee = MultiAsset {
        id: Concrete(dancebox_runtime::xcm_config::SelfReserve::get()),
        fun: Fungible(buy_execution_fee_amount),
    };

    let xcm = VersionedXcm::from(Xcm(vec![
        WithdrawAsset(vec![buy_execution_fee.clone()].into()),
        BuyExecution {
            fees: buy_execution_fee.clone(),
            weight_limit: Unlimited,
        },
        Transact {
            require_weight_at_most,
            origin_kind,
            call,
        },
    ]));

    // Send XCM message from Relay Chain
    SimpleTemplate::execute_with(|| {
        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::PolkadotXcm::send(
                sudo_origin,
                bx!(dancebox_para_destination),
                bx!(xcm),
            )
        );

        type RuntimeEvent = <SimpleTemplate as Chain>::RuntimeEvent;

        assert_expected_events!(
            SimpleTemplate,
            vec![
                RuntimeEvent::PolkadotXcm(pallet_xcm::Event::Sent { .. }) => {},
            ]
        );
    });

    // Receive XCM message in Assets Parachain
    Dancebox::execute_with(|| {
        type RuntimeEvent = <Dancebox as Chain>::RuntimeEvent;
        assert_expected_events!(
            Dancebox,
            vec![
                RuntimeEvent::System(
                    frame_system::Event::Remarked {
                        sender,
                        ..
                    }) => {
                    sender: *sender ==  SiblingParachainConvertsVia::<polkadot_parachain_primitives::primitives::Sibling, crate::AccountId>::convert_location(
                        &MultiLocation{ parents: 1, interior: X1(Parachain(2002u32))}
                    ).unwrap(),
                },
            ]
        );
    });
}
