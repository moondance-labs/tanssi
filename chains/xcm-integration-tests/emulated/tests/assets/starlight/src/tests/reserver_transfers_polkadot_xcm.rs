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
    frame_support::{
        assert_ok,
        weights::{Weight, WeightToFee},
    },
    simple_template_emulated_chain::{genesis::PARA_ID, SimpleTemplateParaPallet},
    sp_runtime::FixedU128,
    starlight_emulated_chain::StarlightRelayPallet,
    starlight_system_emulated_network::{
        SimpleTemplateEmptyReceiver, SimpleTemplatePara as SimpleTemplate,
        StarlightRelay as Starlight, StarlightSender,
    },
    xcm::{
        latest::prelude::{Junctions::*, *},
        VersionedLocation,
    },
    xcm_emulator::{assert_expected_events, bx, Chain, TestExt},
};

#[allow(unused_assignments)]
#[test]
fn transfer_assets_from_starlight_to_one_of_its_parachains() {
    // Starlight origin (sender)
    let starlight_alice_origin =
        <Starlight as Chain>::RuntimeOrigin::signed(StarlightSender::get());

    // Destination location from the starlight relay viewpoint
    let simple_template_dest: VersionedLocation = Location {
        parents: 0,
        interior: X1([Parachain(PARA_ID)].into()),
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

    let amount_to_send: starlight_runtime::Balance =
        starlight_runtime::ExistentialDeposit::get() * 1000;

    // Asset located in Starlight relay
    let assets: Assets = (Here, amount_to_send).into();
    let fee_asset_item = 0;
    let starlight_token_asset_id = 1u16;

    // Register the asset first in simple template chain
    SimpleTemplate::execute_with(|| {
        let root_origin = <SimpleTemplate as Chain>::RuntimeOrigin::root();

        // Create foreign asset from relay chain
        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                Location {
                    parents: 1,
                    interior: Here,
                },
                starlight_token_asset_id,
                SimpleTemplateEmptyReceiver::get(),
                true,
                1
            )
        );

        // Create asset rate
        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::AssetRate::create(
                root_origin,
                bx!(1),
                FixedU128::from_u32(1)
            )
        );
    });

    // Send XCM transfer from Starlight
    Starlight::execute_with(|| {
        assert_ok!(
            <Starlight as StarlightRelayPallet>::XcmPallet::limited_reserve_transfer_assets(
                starlight_alice_origin,
                bx!(simple_template_dest),
                bx!(simple_template_beneficiary),
                bx!(assets.into()),
                fee_asset_item,
                WeightLimit::Unlimited,
            )
        );
    });

    // Verify token reception in Simple Template chain
    SimpleTemplate::execute_with(|| {
        type RuntimeEvent = <SimpleTemplate as Chain>::RuntimeEvent;
        let mut outcome_weight = Weight::default();

        // Check message processing
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

        // Calculate native balance based on weight
        let native_balance =
            container_chain_template_simple_runtime::WeightToFee::weight_to_fee(&outcome_weight);

        // Verify receiver's balance
        assert_eq!(
            <ForeignAssets as frame_support::traits::fungibles::Inspect<_>>::balance(
                starlight_token_asset_id,
                &SimpleTemplateEmptyReceiver::get(),
            ),
            amount_to_send - native_balance
        );
    });
}
