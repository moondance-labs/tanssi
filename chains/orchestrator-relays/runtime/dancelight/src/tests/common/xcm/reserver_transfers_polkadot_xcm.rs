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
    crate::tests::common::xcm::{
        mocknets::{
            DancelightReceiver, DancelightRelay as Dancelight, DancelightSender,
            SimpleTemplateDancelightEmptyReceiver as SimpleTemplateEmptyReceiver,
            SimpleTemplateDancelightPara as SimpleTemplateDancelight,
            SimpleTemplateDancelightSender as SimpleTemplateSender,
        },
        *,
    },
    container_chain_template_simple_runtime::xcm_config::SelfReserve,
    frame_support::{
        assert_ok,
        traits::PalletInfoAccess,
        weights::{Weight, WeightToFee},
    },
    mocknets::{DancelightRelayPallet, SimpleTemplateDancelightParaPallet},
    sp_runtime::FixedU128,
    xcm::{
        latest::prelude::{Junctions::*, *},
        VersionedLocation,
    },
    xcm_emulator::{assert_expected_events, Chain},
};

#[allow(unused_assignments)]
#[test]
fn transfer_assets_from_dancelight_to_one_of_its_parachains() {
    // Dancelight origin (sender)
    let dancelight_alice_origin =
        <Dancelight as Chain>::RuntimeOrigin::signed(DancelightSender::get());

    // Destination location from the dancelight relay viewpoint
    let simple_template_dest: VersionedLocation = Location {
        parents: 0,
        interior: X1([Parachain(constants::simple_template::PARA_ID)].into()),
    }
    .into();

    // Beneficiary location from the simple template parachain viewpoint
    let simple_template_beneficiary: VersionedLocation = Location {
        parents: 0,
        interior: X1([AccountId32 {
            network: None,
            id: SimpleTemplateEmptyReceiver::get().into(),
        }]
        .into()),
    }
    .into();

    let amount_to_send: crate::Balance = crate::ExistentialDeposit::get() * 1000;

    // Asset located in Dancelight relay
    let assets: Assets = (Here, amount_to_send).into();
    let fee_asset_item = 0;
    let dancelight_token_asset_id = 1u16;

    // Register the asset first in simple template chain
    SimpleTemplateDancelight::execute_with(|| {
        let root_origin = <SimpleTemplateDancelight as Chain>::RuntimeOrigin::root();

        // Create foreign asset from relay chain
        assert_ok!(
            <SimpleTemplateDancelight as SimpleTemplateDancelightParaPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                Location {
                    parents: 1,
                    interior: Here,
                },
                dancelight_token_asset_id,
                SimpleTemplateEmptyReceiver::get(),
                true,
                1
            )
        );

        // Create asset rate
        assert_ok!(
            <SimpleTemplateDancelight as SimpleTemplateDancelightParaPallet>::AssetRate::create(
                root_origin,
                bx!(1),
                FixedU128::from_u32(1)
            )
        );
    });

    // Send XCM transfer from Dancelight
    Dancelight::execute_with(|| {
        assert_ok!(
            <Dancelight as DancelightRelayPallet>::XcmPallet::limited_reserve_transfer_assets(
                dancelight_alice_origin,
                bx!(simple_template_dest),
                bx!(simple_template_beneficiary),
                bx!(assets.into()),
                fee_asset_item,
                WeightLimit::Unlimited,
            )
        );
    });

    // Verify token reception in Simple Template chain
    SimpleTemplateDancelight::execute_with(|| {
        type RuntimeEvent = <SimpleTemplateDancelight as Chain>::RuntimeEvent;
        let mut outcome_weight = Weight::default();

        // Check message processing
        assert_expected_events!(
            SimpleTemplateDancelight,
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

        type ForeignAssets =
            <SimpleTemplateDancelight as SimpleTemplateDancelightParaPallet>::ForeignAssets;

        // Calculate native balance based on weight
        let native_balance =
            container_chain_template_simple_runtime::WeightToFee::weight_to_fee(&outcome_weight);

        // Verify receiver's balance
        assert_eq!(
            <ForeignAssets as frame_support::traits::fungibles::Inspect<_>>::balance(
                dancelight_token_asset_id,
                &SimpleTemplateEmptyReceiver::get(),
            ),
            amount_to_send - native_balance
        );
    });
}

#[allow(unused_assignments)]
#[test]
fn transfer_assets_from_parachain_to_dancelight() {
    // Parachain origin (sender)
    let simple_template_origin =
        <SimpleTemplateDancelight as Chain>::RuntimeOrigin::signed(SimpleTemplateSender::get());

    // Destination location from the simple template parachain viewpoint pointing to dancelight relay
    let dancelight_dest: VersionedLocation = Location {
        parents: 1,
        interior: Here,
    }
    .into();

    // Beneficiary location from the dancelight relay viewpoint
    let dancelight_beneficiary: VersionedLocation = Location {
        parents: 0,
        interior: X1([AccountId32 {
            network: None,
            id: DancelightReceiver::get().into(),
        }]
        .into()),
    }
    .into();

    let amount_to_send: crate::Balance = crate::ExistentialDeposit::get() * 1000;

    // Asset located in SimpleTemplate parachain
    let assets: Assets = (SelfReserve::get(), amount_to_send).into();

    let parachain_asset_id = 42u16;
    let fee_asset_item = 0;

    // Register the parachain asset in relay chain
    Dancelight::execute_with(|| {
        let root_origin = <Dancelight as Chain>::RuntimeOrigin::root();

        // Register the parachain asset in relay
        assert_ok!(
            <Dancelight as DancelightRelayPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                Location {
                    parents: 0,
                    interior: X2([
                        Parachain(constants::simple_template::PARA_ID),
                        PalletInstance(
                            <<SimpleTemplateDancelight as SimpleTemplateDancelightParaPallet>::Balances as PalletInfoAccess>::index() as u8,
                        ),
                    ]
                    .into(),),
                },
                parachain_asset_id,
                DancelightReceiver::get(),
                true,
                1
            )
        );

        // Set asset rate in relay
        assert_ok!(<Dancelight as DancelightRelayPallet>::AssetRate::create(
            root_origin,
            Box::new(parachain_asset_id),
            FixedU128::from_u32(1)
        ));
    });

    // Send XCM transfer from SimpleTemplate parachain to Dancelight relay
    SimpleTemplateDancelight::execute_with(|| {
        assert_ok!(
            <SimpleTemplateDancelight as SimpleTemplateDancelightParaPallet>::PolkadotXcm::limited_reserve_transfer_assets(
                simple_template_origin.clone(),
                bx!(dancelight_dest),
                bx!(dancelight_beneficiary),
                bx!(assets.into()),
                fee_asset_item,
                WeightLimit::Unlimited,
            )
        );
    });

    // Verify token reception in Dancelight relay chain
    Dancelight::execute_with(|| {
        type RuntimeEvent = <Dancelight as Chain>::RuntimeEvent;
        let mut outcome_weight = Weight::default();

        // Check message processing
        assert_expected_events!(
            Dancelight,
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

        type ForeignAssets = <Dancelight as DancelightRelayPallet>::ForeignAssets;

        // Calculate native balance based on weight
        let native_balance =
            dancelight_runtime_constants::fee::WeightToFee::weight_to_fee(&outcome_weight);

        println!("native_balance: {}", native_balance); // should be: 12197361

        // Verify receiver's balance
        assert_eq!(
            <ForeignAssets as frame_support::traits::fungibles::Inspect<_>>::balance(
                parachain_asset_id,
                &DancelightReceiver::get(),
            ),
            amount_to_send - native_balance
        );
    });
}
