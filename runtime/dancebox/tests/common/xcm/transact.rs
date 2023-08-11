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
    crate::common::xcm::mocknets::{Dancebox, Westend, WestendPallet},
    frame_support::{
        assert_ok,
        weights::{Weight, WeightToFee},
    },
    parity_scale_codec::Encode,
    xcm::{
        latest::{
            prelude::*,
            Error::{BadOrigin, Barrier},
        },
        VersionedMultiLocation, VersionedXcm,
    },
    xcm_builder::ParentIsPreset,
    xcm_executor::traits::Convert,
};

#[test]
fn transact_sudo_from_relay_hits_barrier_dancebox_without_buy_exec() {
    let call = <Dancebox as Para>::RuntimeCall::Configuration(pallet_configuration::Call::<
        <Dancebox as Para>::Runtime,
    >::set_max_collators {
        new: 50,
    })
    .encode()
    .into();

    // XcmPallet send arguments
    let sudo_origin = <Westend as Relay>::RuntimeOrigin::root();
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
        assert_ok!(<Westend as WestendPallet>::XcmPallet::send(
            sudo_origin,
            bx!(dancebox_para_destination),
            bx!(xcm),
        ));

        type RuntimeEvent = <Westend as Relay>::RuntimeEvent;

        assert_expected_events!(
            Westend,
            vec![
                RuntimeEvent::XcmPallet(pallet_xcm::Event::Sent { .. }) => {},
            ]
        );
    });

    // Receive XCM message in Assets Parachain
    Dancebox::execute_with(|| {
        type RuntimeEvent = <Dancebox as Para>::RuntimeEvent;
        assert_expected_events!(
            Dancebox,
            vec![
                RuntimeEvent::DmpQueue(cumulus_pallet_dmp_queue::Event::ExecutedDownward { outcome, .. }) => {
                    outcome: *outcome == Outcome::Error(Barrier),
                },
            ]
        );
    });
}

#[test]
fn transact_sudo_from_relay_does_not_have_sudo_power() {
    let call = <Dancebox as Para>::RuntimeCall::Configuration(pallet_configuration::Call::<
        <Dancebox as Para>::Runtime,
    >::set_max_collators {
        new: 50,
    })
    .encode()
    .into();

    // XcmPallet send arguments
    let sudo_origin = <Westend as Relay>::RuntimeOrigin::root();
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
        WithdrawAsset {
            0: vec![buy_execution_fee.clone()].into(),
        },
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
        assert_ok!(<Westend as WestendPallet>::XcmPallet::send(
            sudo_origin,
            bx!(dancebox_para_destination),
            bx!(xcm),
        ));

        type RuntimeEvent = <Westend as Relay>::RuntimeEvent;

        assert_expected_events!(
            Westend,
            vec![
                RuntimeEvent::XcmPallet(pallet_xcm::Event::Sent { .. }) => {},
            ]
        );
    });

    // Receive XCM message in Assets Parachain
    Dancebox::execute_with(|| {
        type RuntimeEvent = <Dancebox as Para>::RuntimeEvent;
        assert_expected_events!(
            Dancebox,
            vec![
                RuntimeEvent::DmpQueue(
                    cumulus_pallet_dmp_queue::Event::ExecutedDownward {
                        outcome: Outcome::Incomplete(_w, error), ..
                    }) => {
                    error: *error == BadOrigin,
                },
            ]
        );
    });
}

#[test]
fn transact_sudo_from_relay_has_signed_origin_powers() {
    let call = <Dancebox as Para>::RuntimeCall::System(frame_system::Call::<
        <Dancebox as Para>::Runtime,
    >::remark_with_event {
        remark: b"Test".to_vec(),
    })
    .encode()
    .into();

    // XcmPallet send arguments
    let sudo_origin = <Westend as Relay>::RuntimeOrigin::root();
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
        WithdrawAsset {
            0: vec![buy_execution_fee.clone()].into(),
        },
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
        assert_ok!(<Westend as WestendPallet>::XcmPallet::send(
            sudo_origin,
            bx!(dancebox_para_destination),
            bx!(xcm),
        ));

        type RuntimeEvent = <Westend as Relay>::RuntimeEvent;

        assert_expected_events!(
            Westend,
            vec![
                RuntimeEvent::XcmPallet(pallet_xcm::Event::Sent { .. }) => {},
            ]
        );
    });

    // Receive XCM message in Assets Parachain
    Dancebox::execute_with(|| {
        type RuntimeEvent = <Dancebox as Para>::RuntimeEvent;
        assert_expected_events!(
            Dancebox,
            vec![
                RuntimeEvent::System(
                    frame_system::Event::Remarked {
                        sender,
                        ..
                    }) => {
                    sender: *sender == ParentIsPreset::<dancebox_runtime::AccountId>::convert_ref(MultiLocation::parent()).unwrap(),
                },
            ]
        );
    });
}

#[test]
fn transact_sudo_from_frontier_has_signed_origin_powers() {
    let call = <Dancebox as Para>::RuntimeCall::System(frame_system::Call::<
        <Dancebox as Para>::Runtime,
    >::remark_with_event {
        remark: b"Test".to_vec(),
    })
    .encode()
    .into();

    // XcmPallet send arguments
    let sudo_origin = <FrontierTemplate as Parachain>::RuntimeOrigin::root();
    let dancebox_para_destination: VersionedMultiLocation =
        FrontierTemplate::sibling_location_of(Dancebox::para_id()).into();

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
        WithdrawAsset {
            0: vec![buy_execution_fee.clone()].into(),
        },
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
        assert_ok!(<Westend as WestendPallet>::XcmPallet::send(
            sudo_origin,
            bx!(dancebox_para_destination),
            bx!(xcm),
        ));

        type RuntimeEvent = <Westend as Relay>::RuntimeEvent;

        assert_expected_events!(
            Westend,
            vec![
                RuntimeEvent::XcmPallet(pallet_xcm::Event::Sent { .. }) => {},
            ]
        );
    });

    // Receive XCM message in Assets Parachain
    Dancebox::execute_with(|| {
        type RuntimeEvent = <Dancebox as Para>::RuntimeEvent;
        assert_expected_events!(
            Dancebox,
            vec![
                RuntimeEvent::System(
                    frame_system::Event::Remarked {
                        sender,
                        ..
                    }) => {
                    sender: *sender == ParentIsPreset::<dancebox_runtime::AccountId>::convert_ref(MultiLocation::parent()).unwrap(),
                },
            ]
        );
    });
}
