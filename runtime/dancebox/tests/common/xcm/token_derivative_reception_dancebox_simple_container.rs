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
            SimpleTemplatePara as SimpleTemplate, SimpleTemplateParaPallet, SimpleTemplateReceiver,
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
fn receive_tokens_from_tanssi_to_simple_template() {
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
            id: SimpleTemplateReceiver::get().into(),
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
                SimpleTemplateReceiver::get(),
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
            <Dancebox as DanceboxParaPallet>::PolkadotXcm::limited_reserve_transfer_assets(
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
                &SimpleTemplateReceiver::get(),
            ),
            amount_to_send - native_balance
        );
    });
}
