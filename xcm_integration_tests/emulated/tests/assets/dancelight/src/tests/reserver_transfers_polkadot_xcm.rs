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
    // crate::tests::common::xcm::{
    //     mocknets::{
    //         DancelightRelay as Dancelight, DancelightSender,
    //         SimpleTemplateDancelightEmptyReceiver as SimpleTemplateEmptyReceiver,
    //         SimpleTemplateDancelightPara as SimpleTemplateDancelight,
    //     },
    //     *,
    // },
    frame_support::{
        assert_ok,
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

    let amount_to_send: dancelight_runtime::Balance =
        dancelight_runtime::ExistentialDeposit::get() * 1000;

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
