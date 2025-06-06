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
    dancebox_emulated_chain::DanceboxParaPallet,
    frame_support::{
        assert_ok,
        traits::{tokens::ConversionToAssetBalance, PalletInfoAccess},
        weights::{Weight, WeightToFee},
    },
    frontier_template_emulated_chain::{EthereumReceiver, FrontierTemplateParaPallet},
    sp_runtime::FixedU128,
    westend_system_emulated_network::{
        DanceboxPara as Dancebox, DanceboxSender, FrontierTemplatePara as FrontierTemplate,
    },
    xcm::{
        latest::prelude::{Junctions::*, *},
        VersionedLocation,
    },
    xcm_emulator::{assert_expected_events, bx, Chain, TestExt},
};

#[allow(unused_assignments)]
#[test]
fn receive_tokens_from_tanssi_to_frontier_template() {
    // XcmPallet reserve transfer arguments
    let alice_origin = <Dancebox as Chain>::RuntimeOrigin::signed(DanceboxSender::get());

    // Parents 1 this time
    let frontier_template_dest: VersionedLocation = Location {
        parents: 1,
        interior: X1([Parachain(2001u32)].into()),
    }
    .into();

    let frontier_template_beneficiary: VersionedLocation = Location {
        parents: 0,
        interior: X1([AccountKey20 {
            network: None,
            key: EthereumReceiver::get().into(),
        }]
        .into()),
    }
    .into();

    let amount_to_send: dancebox_runtime::Balance =
        dancebox_runtime::ExistentialDeposit::get() * 1000;

    let dancebox_pallet_info_junction = PalletInstance(
        <<Dancebox as DanceboxParaPallet>::Balances as PalletInfoAccess>::index() as u8,
    );
    let assets: Assets = (X1([dancebox_pallet_info_junction].into()), amount_to_send).into();
    let fee_asset_item = 0;
    let dancebox_token_asset_id = 1u16;

    // Register the asset first
    FrontierTemplate::execute_with(|| {
        let root_origin = <FrontierTemplate as Chain>::RuntimeOrigin::root();

        assert_ok!(
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                Location {
                    parents: 1,
                    interior: X2([Parachain(2000), dancebox_pallet_info_junction].into())
                },
                dancebox_token_asset_id,
                EthereumReceiver::get(),
                true,
                1
            )
        );

        assert_ok!(
            <FrontierTemplate as FrontierTemplateParaPallet>::AssetRate::create(
                root_origin,
                bx!(1),
                FixedU128::from_u32(1_000_000u32)
            )
        );
    });

    // Send XCM message from Dancebox
    Dancebox::execute_with(|| {
        assert_ok!(
            <Dancebox as DanceboxParaPallet>::PolkadotXcm::limited_reserve_transfer_assets(
                alice_origin,
                bx!(frontier_template_dest),
                bx!(frontier_template_beneficiary),
                bx!(assets.into()),
                fee_asset_item,
                WeightLimit::Unlimited,
            )
        );
    });
    // We should have received the tokens
    FrontierTemplate::execute_with(|| {
        type RuntimeEvent = <FrontierTemplate as Chain>::RuntimeEvent;
        let mut outcome_weight = Weight::default();
        assert_expected_events!(
            FrontierTemplate,
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

        type ForeignAssets = <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssets;

        // We should have charged an amount of tokens that is identical to the weight spent
        let native_balance =
            container_chain_template_frontier_runtime::WeightToFee::weight_to_fee(&outcome_weight);

        // We need to convert this to asset-balance charged.
        let asset_balance = <<FrontierTemplate as FrontierTemplateParaPallet>::AssetRate as ConversionToAssetBalance<_,_,_>>::to_asset_balance(
            native_balance,
            1
        ).unwrap();

        // Assert empty receiver received funds
        assert_eq!(
            <ForeignAssets as frame_support::traits::fungibles::Inspect<_>>::balance(
                dancebox_token_asset_id,
                &EthereumReceiver::get(),
            ),
            amount_to_send - asset_balance
        );
    });
}
