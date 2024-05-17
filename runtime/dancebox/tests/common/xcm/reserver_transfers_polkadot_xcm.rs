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
            SimpleTemplateEmptyReceiver, SimpleTemplatePara as SimpleTemplate,
            SimpleTemplateParaPallet, SimpleTemplateSender, WestendRelay as Westend,
            WestendRelayPallet, WestendSender,
        },
        *,
    },
    frame_support::{
        assert_noop, assert_ok,
        traits::PalletInfoAccess,
        weights::{Weight, WeightToFee},
    },
    sp_runtime::FixedU128,
    staging_xcm::{latest::prelude::*, VersionedMultiLocation},
    xcm_emulator::Chain,
};

#[allow(unused_assignments)]
#[test]
fn transfer_assets_single_asset_fee_and_asset_reserves() {
    // XcmPallet reserve transfer arguments
    let alice_origin = <Dancebox as Chain>::RuntimeOrigin::signed(DanceboxSender::get());

    // Parents 1 this time
    let simple_template_dest: VersionedMultiLocation = MultiLocation {
        parents: 1,
        interior: X1(Parachain(2002u32)),
    }
    .into();

    let simple_template_beneficiary: VersionedMultiLocation = MultiLocation {
        parents: 0,
        interior: X1(AccountId32 {
            network: None,
            id: SimpleTemplateEmptyReceiver::get().into(),
        }),
    }
    .into();

    let amount_to_send: crate::Balance = dancebox_runtime::ExistentialDeposit::get() * 1000;

    let dancebox_pallet_info_junction = PalletInstance(
        <<Dancebox as DanceboxParaPallet>::Balances as PalletInfoAccess>::index() as u8,
    );
    let assets: MultiAssets = (X1(dancebox_pallet_info_junction), amount_to_send).into();
    let fee_asset_item = 0;
    let dancebox_token_asset_id = 1u16;

    // Register the asset first
    SimpleTemplate::execute_with(|| {
        let root_origin = <SimpleTemplate as Chain>::RuntimeOrigin::root();

        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                MultiLocation {
                    parents: 1,
                    interior: X2(Parachain(2000), dancebox_pallet_info_junction)
                },
                dancebox_token_asset_id,
                SimpleTemplateEmptyReceiver::get(),
                true,
                1
            )
        );

        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::AssetRate::create(
                root_origin,
                bx!(1),
                FixedU128::from_u32(1)
            )
        );
    });

    // Send XCM message from Dancebox
    Dancebox::execute_with(|| {
        assert_ok!(
            <Dancebox as DanceboxParaPallet>::PolkadotXcm::transfer_assets(
                alice_origin,
                bx!(simple_template_dest),
                bx!(simple_template_beneficiary),
                bx!(assets.into()),
                fee_asset_item,
                WeightLimit::Unlimited,
            )
        );
    });
    // We should have received the tokens
    SimpleTemplate::execute_with(|| {
        type RuntimeEvent = <SimpleTemplate as Chain>::RuntimeEvent;
        let mut outcome_weight = Weight::default();
        assert_expected_events!(
            SimpleTemplate,
            vec![
                RuntimeEvent::MessageQueue(
                    pallet_message_queue::Event::Processed {
                        success: true,
                        weight_used,
                        ..
                    }) => {
                        weight_used: {
                            outcome_weight = *weight_used;
                            weight_used.all_gte(Weight::from_parts(0,0))
                        },
                    },
            ]
        );
        type ForeignAssets = <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssets;

        // We should have charged an amount of tokens that is identical to the weight spent
        let native_balance =
            container_chain_template_simple_runtime::WeightToFee::weight_to_fee(&outcome_weight);

        // Assert empty receiver received funds
        assert_eq!(
            <ForeignAssets as frame_support::traits::fungibles::Inspect<_>>::balance(
                dancebox_token_asset_id,
                &SimpleTemplateEmptyReceiver::get(),
            ),
            amount_to_send - native_balance
        );
    });
}

#[allow(unused_assignments)]
#[test]
fn transfer_assets_relay_tanssi() {
    // XcmPallet reserve transfer arguments
    let alice_dancebox_origin = <Dancebox as Chain>::RuntimeOrigin::signed(DanceboxSender::get());
    let alice_relay_origin = <Westend as Chain>::RuntimeOrigin::signed(WestendSender::get());

    // Parents 1 this time
    let simple_template_dest: VersionedMultiLocation = MultiLocation {
        parents: 1,
        interior: X1(Parachain(2002u32)),
    }
    .into();

    let simple_template_beneficiary: VersionedMultiLocation = MultiLocation {
        parents: 0,
        interior: X1(AccountId32 {
            network: None,
            id: SimpleTemplateEmptyReceiver::get().into(),
        }),
    }
    .into();

    let dancebox_dest: VersionedMultiLocation = MultiLocation {
        parents: 0,
        interior: X1(Parachain(2000u32)),
    }
    .into();

    let dancebox_beneficiary: VersionedMultiLocation = MultiLocation {
        parents: 0,
        interior: X1(AccountId32 {
            network: None,
            id: DanceboxSender::get().into(),
        }),
    }
    .into();

    let dancebox_amount_to_send: crate::Balance =
        dancebox_runtime::ExistentialDeposit::get() * 1000;

    let dancebox_pallet_info_junction = PalletInstance(
        <<Dancebox as DanceboxParaPallet>::Balances as PalletInfoAccess>::index() as u8,
    );
    let dancebox_assets = (X1(dancebox_pallet_info_junction), dancebox_amount_to_send);
    let relay_amount_to_send: crate::Balance = westend_runtime::ExistentialDeposit::get() * 1000;

    let relay_assets: MultiAssets = (Here, relay_amount_to_send).into();

    let fee_asset_item = 0;
    let dancebox_token_asset_id = 1u16;
    let westend_token_asset_id = 2u16;

    // Register the assets first
    SimpleTemplate::execute_with(|| {
        let root_origin = <SimpleTemplate as Chain>::RuntimeOrigin::root();

        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                MultiLocation {
                    parents: 1,
                    interior: X2(Parachain(2000), dancebox_pallet_info_junction)
                },
                dancebox_token_asset_id,
                SimpleTemplateEmptyReceiver::get(),
                true,
                1
            )
        );

        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::AssetRate::create(
                root_origin.clone(),
                bx!(1),
                FixedU128::from_u32(1)
            )
        );

        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                MultiLocation::parent(),
                westend_token_asset_id,
                SimpleTemplateEmptyReceiver::get(),
                true,
                1
            )
        );

        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::AssetRate::create(
                root_origin,
                bx!(westend_token_asset_id),
                FixedU128::from_u32(1)
            )
        );
    });

    // Register the relay asset first
    Dancebox::execute_with(|| {
        let root_origin = <Dancebox as Chain>::RuntimeOrigin::root();

        assert_ok!(
            <Dancebox as DanceboxParaPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                MultiLocation::parent(),
                westend_token_asset_id,
                DanceboxSender::get(),
                true,
                1
            )
        );

        assert_ok!(<Dancebox as DanceboxParaPallet>::AssetRate::create(
            root_origin,
            bx!(westend_token_asset_id),
            FixedU128::from_u32(1)
        ));
    });

    // Relay sends to dancebox first
    Westend::execute_with(|| {
        assert_ok!(
            <Westend as WestendRelayPallet>::XcmPallet::limited_reserve_transfer_assets(
                alice_relay_origin,
                bx!(dancebox_dest),
                bx!(dancebox_beneficiary),
                bx!(relay_assets.into()),
                fee_asset_item,
                WeightLimit::Unlimited,
            )
        );
    });

    // We should have received the tokens
    let mut native_balance = 0u128;
    Dancebox::execute_with(|| {
        type RuntimeEvent = <Dancebox as Chain>::RuntimeEvent;
        let mut outcome_weight = Weight::default();
        assert_expected_events!(
            Dancebox,
            vec![
                RuntimeEvent::MessageQueue(
                    pallet_message_queue::Event::Processed {
                        success: true,
                        weight_used,
                        ..
                    }) => {
                        weight_used: {
                            outcome_weight = *weight_used;
                            weight_used.all_gte(Weight::from_parts(0,0))
                        },
                    },
            ]
        );
        type ForeignAssets = <Dancebox as DanceboxParaPallet>::ForeignAssets;

        // We should have charged an amount of tokens that is identical to the weight spent
        native_balance = dancebox_runtime::WeightToFee::weight_to_fee(&outcome_weight);

        // Assert empty receiver received funds
        assert_eq!(
            <ForeignAssets as frame_support::traits::fungibles::Inspect<_>>::balance(
                westend_token_asset_id,
                &DanceboxSender::get(),
            ),
            relay_amount_to_send - native_balance
        );
    });

    let relay_tokens_to_send_simple_template = (relay_amount_to_send - native_balance) / 2;
    // We just send half of the DOT received
    let combined_assets: MultiAssets = vec![
        dancebox_assets.into(),
        (
            MultiLocation::parent(),
            relay_tokens_to_send_simple_template,
        )
            .into(),
    ]
    .into();

    // Now we need to send both to simple template
    // Send XCM message from Dancebox
    // Let's try to use dot as the fee
    // This should not work as we are trying to send two assets
    // with different XCM paths (one goes to the relay, the other one does not)
    Dancebox::execute_with(|| {
        assert_noop!(
            <Dancebox as DanceboxParaPallet>::PolkadotXcm::transfer_assets(
                alice_dancebox_origin.clone(),
                bx!(simple_template_dest.clone()),
                bx!(simple_template_beneficiary.clone()),
                bx!(combined_assets.clone().into()),
                1,
                WeightLimit::Unlimited,
            ),
            pallet_xcm::Error::<dancebox_runtime::Runtime>::InvalidAssetUnsupportedReserve
        );
    });
}

#[allow(unused_assignments)]
#[test]
fn transfer_assets_container_token_tanssi() {
    // XcmPallet reserve transfer arguments
    let alice_dancebox_origin = <Dancebox as Chain>::RuntimeOrigin::signed(DanceboxSender::get());
    let alice_simple_template_origin =
        <SimpleTemplate as Chain>::RuntimeOrigin::signed(SimpleTemplateSender::get());

    // Parents 1 this time
    let simple_template_dest: VersionedMultiLocation = MultiLocation {
        parents: 1,
        interior: X1(Parachain(2002u32)),
    }
    .into();

    let simple_template_beneficiary: VersionedMultiLocation = MultiLocation {
        parents: 0,
        interior: X1(AccountId32 {
            network: None,
            id: SimpleTemplateEmptyReceiver::get().into(),
        }),
    }
    .into();

    let dancebox_dest: VersionedMultiLocation = MultiLocation {
        parents: 1,
        interior: X1(Parachain(2000u32)),
    }
    .into();

    let dancebox_beneficiary: VersionedMultiLocation = MultiLocation {
        parents: 0,
        interior: X1(AccountId32 {
            network: None,
            id: DanceboxSender::get().into(),
        }),
    }
    .into();

    let dancebox_amount_to_send: crate::Balance =
        dancebox_runtime::ExistentialDeposit::get() * 1000;

    let dancebox_pallet_info_junction = PalletInstance(
        <<Dancebox as DanceboxParaPallet>::Balances as PalletInfoAccess>::index() as u8,
    );
    let dancebox_assets = (X1(dancebox_pallet_info_junction), dancebox_amount_to_send);
    let simple_template_amount_to_send: crate::Balance =
        container_chain_template_simple_runtime::ExistentialDeposit::get() * 1000;

    let simple_template_pallet_info_junction = PalletInstance(
        <<SimpleTemplate as SimpleTemplateParaPallet>::Balances as PalletInfoAccess>::index() as u8,
    );

    let simple_template_assets: MultiAssets = (
        simple_template_pallet_info_junction,
        simple_template_amount_to_send,
    )
        .into();

    let fee_asset_item = 0;
    let dancebox_token_asset_id = 1u16;
    let simple_template_token_asset_id = 2u16;

    // Register the assets first
    SimpleTemplate::execute_with(|| {
        let root_origin = <SimpleTemplate as Chain>::RuntimeOrigin::root();

        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                MultiLocation {
                    parents: 1,
                    interior: X2(Parachain(2000), dancebox_pallet_info_junction)
                },
                dancebox_token_asset_id,
                SimpleTemplateEmptyReceiver::get(),
                true,
                1
            )
        );

        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::AssetRate::create(
                root_origin.clone(),
                bx!(1),
                FixedU128::from_u32(1)
            )
        );
    });

    // Register the simple template asset first
    Dancebox::execute_with(|| {
        let root_origin = <Dancebox as Chain>::RuntimeOrigin::root();

        assert_ok!(
            <Dancebox as DanceboxParaPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                MultiLocation {
                    parents: 1,
                    interior: X2(Parachain(2002), simple_template_pallet_info_junction)
                },
                simple_template_token_asset_id,
                DanceboxSender::get(),
                true,
                1
            )
        );

        assert_ok!(<Dancebox as DanceboxParaPallet>::AssetRate::create(
            root_origin,
            bx!(simple_template_token_asset_id),
            FixedU128::from_u32(1)
        ));
    });

    // Simple Template sends to dancebox first
    SimpleTemplate::execute_with(|| {
        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::PolkadotXcm::limited_reserve_transfer_assets(
                alice_simple_template_origin,
                bx!(dancebox_dest),
                bx!(dancebox_beneficiary),
                bx!(simple_template_assets.into()),
                fee_asset_item,
                WeightLimit::Unlimited,
            )
        );
    });

    // We should have received the tokens
    let mut native_balance = 0u128;
    Dancebox::execute_with(|| {
        type RuntimeEvent = <Dancebox as Chain>::RuntimeEvent;
        let mut outcome_weight = Weight::default();
        assert_expected_events!(
            Dancebox,
            vec![
                RuntimeEvent::MessageQueue(
                    pallet_message_queue::Event::Processed {
                        success: true,
                        weight_used,
                        ..
                    }) => {
                        weight_used: {
                            outcome_weight = *weight_used;
                            weight_used.all_gte(Weight::from_parts(0,0))
                        },
                    },
            ]
        );
        type ForeignAssets = <Dancebox as DanceboxParaPallet>::ForeignAssets;

        // We should have charged an amount of tokens that is identical to the weight spent
        native_balance = dancebox_runtime::WeightToFee::weight_to_fee(&outcome_weight);

        // Assert empty receiver received funds
        assert_eq!(
            <ForeignAssets as frame_support::traits::fungibles::Inspect<_>>::balance(
                simple_template_token_asset_id,
                &DanceboxSender::get(),
            ),
            simple_template_amount_to_send - native_balance
        );
    });

    let simple_template_tokens_to_send_simple_template =
        (simple_template_amount_to_send - native_balance) / 2;
    // We just send half of the DOT received
    let combined_assets: MultiAssets = vec![
        dancebox_assets.into(),
        (
            MultiLocation {
                parents: 1,
                interior: X2(Parachain(2002), simple_template_pallet_info_junction),
            },
            simple_template_tokens_to_send_simple_template,
        )
            .into(),
    ]
    .into();

    // Now we need to send both to simple template
    // Send XCM message from Dancebox
    // Let's try to use dot as the fee
    // This should work as we are trying to send two assets that follow the same XCM path
    Dancebox::execute_with(|| {
        assert_ok!(
            <Dancebox as DanceboxParaPallet>::PolkadotXcm::transfer_assets(
                alice_dancebox_origin.clone(),
                bx!(simple_template_dest.clone()),
                bx!(simple_template_beneficiary.clone()),
                bx!(combined_assets.clone().into()),
                1,
                WeightLimit::Unlimited,
            )
        );
    });

    // Let's assert we received them
    SimpleTemplate::execute_with(|| {
        type RuntimeEvent = <SimpleTemplate as Chain>::RuntimeEvent;
        let mut outcome_weight = Weight::default();
        assert_expected_events!(
            SimpleTemplate,
            vec![
                RuntimeEvent::MessageQueue(
                    pallet_message_queue::Event::Processed {
                        success: true,
                        weight_used,
                        ..
                    }) => {
                        weight_used: {
                            outcome_weight = *weight_used;
                            weight_used.all_gte(Weight::from_parts(0,0))
                        },
                    },
            ]
        );
        type ForeignAssets = <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssets;

        // We should have charged an amount of tokens that is identical to the weight spent
        let charged_tokens =
            container_chain_template_simple_runtime::WeightToFee::weight_to_fee(&outcome_weight);

        // Assert empty receiver received funds for dancebox asset
        assert_eq!(
            <ForeignAssets as frame_support::traits::fungibles::Inspect<_>>::balance(
                dancebox_token_asset_id,
                &SimpleTemplateEmptyReceiver::get(),
            ),
            dancebox_amount_to_send
        );

        // Assert empty receiver received funds for native asset
        assert_eq!(
            <SimpleTemplate as SimpleTemplateParaPallet>::System::account(
                SimpleTemplateEmptyReceiver::get()
            )
            .data
            .free,
            simple_template_tokens_to_send_simple_template - charged_tokens
        );
    });
}

#[allow(unused_assignments)]
#[test]
fn transfer_asset_relay_token_across_tanssi_container() {
    // XcmPallet reserve transfer arguments
    let alice_dancebox_origin = <Dancebox as Chain>::RuntimeOrigin::signed(DanceboxSender::get());
    let alice_relay_origin = <Westend as Chain>::RuntimeOrigin::signed(WestendSender::get());

    // Parents 1 this time
    let simple_template_dest: VersionedMultiLocation = MultiLocation {
        parents: 1,
        interior: X1(Parachain(2002u32)),
    }
    .into();

    let simple_template_beneficiary: VersionedMultiLocation = MultiLocation {
        parents: 0,
        interior: X1(AccountId32 {
            network: None,
            id: SimpleTemplateEmptyReceiver::get().into(),
        }),
    }
    .into();

    let dancebox_dest: VersionedMultiLocation = MultiLocation {
        parents: 0,
        interior: X1(Parachain(2000u32)),
    }
    .into();

    let dancebox_beneficiary: VersionedMultiLocation = MultiLocation {
        parents: 0,
        interior: X1(AccountId32 {
            network: None,
            id: DanceboxSender::get().into(),
        }),
    }
    .into();

    let relay_amount_to_send: crate::Balance = westend_runtime::ExistentialDeposit::get() * 1000;

    let relay_assets: MultiAssets = (Here, relay_amount_to_send).into();

    let fee_asset_item = 0;
    let westend_token_asset_id = 1u16;

    // Register the relay asset first
    SimpleTemplate::execute_with(|| {
        let root_origin = <SimpleTemplate as Chain>::RuntimeOrigin::root();

        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                MultiLocation::parent(),
                westend_token_asset_id,
                SimpleTemplateEmptyReceiver::get(),
                true,
                1
            )
        );

        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::AssetRate::create(
                root_origin,
                bx!(westend_token_asset_id),
                FixedU128::from_u32(1)
            )
        );
    });

    // Register the relay asset first
    Dancebox::execute_with(|| {
        let root_origin = <Dancebox as Chain>::RuntimeOrigin::root();

        assert_ok!(
            <Dancebox as DanceboxParaPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                MultiLocation::parent(),
                westend_token_asset_id,
                DanceboxSender::get(),
                true,
                1
            )
        );

        assert_ok!(<Dancebox as DanceboxParaPallet>::AssetRate::create(
            root_origin,
            bx!(westend_token_asset_id),
            FixedU128::from_u32(1)
        ));
    });

    // Relay sends to dancebox first
    Westend::execute_with(|| {
        assert_ok!(
            <Westend as WestendRelayPallet>::XcmPallet::limited_reserve_transfer_assets(
                alice_relay_origin,
                bx!(dancebox_dest),
                bx!(dancebox_beneficiary),
                bx!(relay_assets.into()),
                fee_asset_item,
                WeightLimit::Unlimited,
            )
        );
    });

    // We should have received the tokens
    let mut native_balance = 0u128;
    Dancebox::execute_with(|| {
        type RuntimeEvent = <Dancebox as Chain>::RuntimeEvent;
        let mut outcome_weight = Weight::default();
        assert_expected_events!(
            Dancebox,
            vec![
                RuntimeEvent::MessageQueue(
                    pallet_message_queue::Event::Processed {
                        success: true,
                        weight_used,
                        ..
                    }) => {
                        weight_used: {
                            outcome_weight = *weight_used;
                            weight_used.all_gte(Weight::from_parts(0,0))
                        },
                    },
            ]
        );
        type ForeignAssets = <Dancebox as DanceboxParaPallet>::ForeignAssets;

        // We should have charged an amount of tokens that is identical to the weight spent
        native_balance = dancebox_runtime::WeightToFee::weight_to_fee(&outcome_weight);

        // Assert empty receiver received funds
        assert_eq!(
            <ForeignAssets as frame_support::traits::fungibles::Inspect<_>>::balance(
                westend_token_asset_id,
                &DanceboxSender::get(),
            ),
            relay_amount_to_send - native_balance
        );
    });

    let relay_tokens_to_send_simple_template = (relay_amount_to_send - native_balance) / 2;
    // We just send half of the DOT received
    let relay_assets_to_send: MultiAssets = vec![(
        MultiLocation::parent(),
        relay_tokens_to_send_simple_template,
    )
        .into()]
    .into();

    // Now we need to send the relay asset to simple template
    // Send XCM message from Dancebox
    // Let's try to use dot as the fee
    // This should work as we are trying to send a single asset
    Dancebox::execute_with(|| {
        assert_ok!(
            <Dancebox as DanceboxParaPallet>::PolkadotXcm::transfer_assets(
                alice_dancebox_origin.clone(),
                bx!(simple_template_dest.clone()),
                bx!(simple_template_beneficiary.clone()),
                bx!(relay_assets_to_send.into()),
                0,
                WeightLimit::Unlimited,
            )
        );
    });

    Westend::execute_with(|| {
        type RuntimeEvent = <Westend as Chain>::RuntimeEvent;
        assert_expected_events!(
            Westend,
            vec![
                RuntimeEvent::MessageQueue(
                    pallet_message_queue::Event::Processed {
                        success: true,
                        ..
                    }) => {},
            ]
        );
    });

    // Let's assert we received them
    SimpleTemplate::execute_with(|| {
        type RuntimeEvent = <SimpleTemplate as Chain>::RuntimeEvent;
        let mut outcome_weight = Weight::default();
        assert_expected_events!(
            SimpleTemplate,
            vec![
                RuntimeEvent::MessageQueue(
                    pallet_message_queue::Event::Processed {
                        success: true,
                        weight_used,
                        ..
                    }) => {
                        weight_used: {
                            outcome_weight = *weight_used;
                            weight_used.all_gte(Weight::from_parts(0,0))
                        },
                    },
            ]
        );
        type ForeignAssets = <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssets;

        // We should have charged an amount of tokens that is identical to the weight spent
        let charged_tokens =
            container_chain_template_simple_runtime::WeightToFee::weight_to_fee(&outcome_weight);

        // Substract delivery fees
        // There is no easy way to calculate this, but we know at least they should be more
        // than the base delivery fee
        let westend_base_delivery_fee = westend_runtime::xcm_config::BaseDeliveryFee::get();

        // Assert empty receiver received funds for relay asset
        assert!(
            <ForeignAssets as frame_support::traits::fungibles::Inspect<_>>::balance(
                westend_token_asset_id,
                &SimpleTemplateEmptyReceiver::get(),
            ) < relay_tokens_to_send_simple_template - charged_tokens - westend_base_delivery_fee
        );
    });
}
