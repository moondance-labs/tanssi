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
            DanceboxPara as Dancebox, DanceboxParaPallet, DanceboxReceiver,
            SimpleTemplatePara as SimpleTemplate, SimpleTemplateParaPallet, SimpleTemplateSender,
        },
        *,
    },
    frame_support::{
        assert_ok,
        traits::PalletInfoAccess,
        weights::{Weight, WeightToFee},
    },
    sp_runtime::FixedU128,
    staging_xcm::{latest::prelude::*, VersionedMultiLocation},
    xcm_emulator::Chain,
};

#[allow(unused_assignments)]
#[test]
fn receive_tokens_from_the_container_to_tanssi() {
    // XcmPallet reserve transfer arguments
    let alice_origin =
        <SimpleTemplate as Chain>::RuntimeOrigin::signed(SimpleTemplateSender::get());

    // Parents 1 this time
    let dancebox_dest: VersionedMultiLocation = MultiLocation {
        parents: 1,
        interior: X1(Parachain(2000u32)),
    }
    .into();

    let dancebox_beneficiary: VersionedMultiLocation = MultiLocation {
        parents: 0,
        interior: X1(AccountId32 {
            network: None,
            id: DanceboxReceiver::get().into(),
        }),
    }
    .into();

    let amount_to_send: crate::Balance =
        container_chain_template_simple_runtime::ExistentialDeposit::get() * 1000;

    let simple_template_pallet_info_junction = PalletInstance(
        <<SimpleTemplate as SimpleTemplateParaPallet>::Balances as PalletInfoAccess>::index() as u8,
    );
    let assets: MultiAssets = (X1(simple_template_pallet_info_junction), amount_to_send).into();
    let fee_asset_item = 0;
    let simple_template_token_asset_id = 1u16;

    // Register the asset first
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
                DanceboxReceiver::get(),
                true,
                1
            )
        );

        assert_ok!(<Dancebox as DanceboxParaPallet>::AssetRate::create(
            root_origin,
            bx!(1),
            FixedU128::from_u32(1)
        ));
    });

    // Send XCM message from SimpleTemplate
    SimpleTemplate::execute_with(|| {
        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::PolkadotXcm::limited_reserve_transfer_assets(
                alice_origin,
                bx!(dancebox_dest),
                bx!(dancebox_beneficiary),
                bx!(assets.into()),
                fee_asset_item,
                WeightLimit::Unlimited,
            )
        );
    });
    // We should have received the tokens
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
        let native_balance = dancebox_runtime::WeightToFee::weight_to_fee(&outcome_weight);

        // Assert empty receiver received funds
        assert_eq!(
            <ForeignAssets as frame_support::traits::fungibles::Inspect<_>>::balance(
                simple_template_token_asset_id,
                &DanceboxReceiver::get(),
            ),
            amount_to_send - native_balance
        );
    });
}
