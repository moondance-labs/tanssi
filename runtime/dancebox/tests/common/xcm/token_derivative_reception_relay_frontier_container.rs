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
    crate::common::xcm::{
        mocknets::{
            EthereumReceiver, FrontierTemplate, FrontierTemplatePallet, Westend, WestendPallet,
            WestendSender,
        },
        *,
    },
    frame_support::{
        assert_ok,
        weights::{Weight, WeightToFee},
    },
    sp_runtime::FixedU128,
    staging_xcm::{latest::prelude::*, VersionedMultiLocation},
    xcm_emulator::Chain,
};

#[allow(unused_assignments)]
#[test]
fn receive_tokens_from_the_relay_to_frontier_template() {
    // XcmPallet reserve transfer arguments
    let alice_origin = <Westend as Chain>::RuntimeOrigin::signed(WestendSender::get());

    let frontier_template_dest: VersionedMultiLocation = MultiLocation {
        parents: 0,
        interior: X1(Parachain(2001u32)),
    }
    .into();

    let frontier_template_beneficiary: VersionedMultiLocation = MultiLocation {
        parents: 0,
        interior: X1(AccountKey20 {
            network: None,
            key: EthereumReceiver::get().into(),
        }),
    }
    .into();

    let amount_to_send: crate::Balance = westend_runtime::ExistentialDeposit::get() * 1000;

    let assets: MultiAssets = (Here, amount_to_send).into();
    let fee_asset_item = 0;
    let westend_token_asset_id = 1u16;

    // Register the asset first
    FrontierTemplate::execute_with(|| {
        let root_origin = <FrontierTemplate as Chain>::RuntimeOrigin::root();

        assert_ok!(
            <FrontierTemplate as FrontierTemplatePallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                MultiLocation::parent(),
                westend_token_asset_id.into(),
                EthereumReceiver::get(),
                true,
                1
            )
        );

        assert_ok!(
            <FrontierTemplate as FrontierTemplatePallet>::AssetRate::create(
                root_origin,
                bx!(1),
                FixedU128::from_u32(1)
            )
        );
    });

    // Send XCM message from Westend
    Westend::execute_with(|| {
        assert_ok!(
            <Westend as WestendPallet>::XcmPallet::limited_reserve_transfer_assets(
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
                RuntimeEvent::DmpQueue(
                    cumulus_pallet_dmp_queue::Event::ExecutedDownward {
                        outcome, ..
                    }) => {
                    outcome: {
                        outcome_weight = outcome.clone().weight_used();
                        outcome.clone().ensure_complete().is_ok()
                    },
                },
            ]
        );
        type ForeignAssets = <FrontierTemplate as FrontierTemplatePallet>::ForeignAssets;

        // We should have charged an amount of tokens that is identical to the weight spent
        let native_balance =
            container_chain_template_frontier_runtime::WeightToFee::weight_to_fee(&outcome_weight);

        // Assert empty receiver received funds
        assert_eq!(
            <ForeignAssets as frame_support::traits::fungibles::Inspect<_>>::balance(
                westend_token_asset_id.into(),
                &EthereumReceiver::get(),
            ),
            amount_to_send - native_balance
        );
    });
}

#[test]
fn cannot_receive_tokens_from_the_relay_if_no_rate_is_assigned_frontier_template() {
    // XcmPallet reserve transfer arguments
    let alice_origin = <Westend as Chain>::RuntimeOrigin::signed(WestendSender::get());

    let frontier_template_dest: VersionedMultiLocation = MultiLocation {
        parents: 0,
        interior: X1(Parachain(2001u32)),
    }
    .into();

    let frontier_template_beneficiary: VersionedMultiLocation = MultiLocation {
        parents: 0,
        interior: X1(AccountKey20 {
            network: None,
            key: EthereumReceiver::get().into(),
        }),
    }
    .into();

    let amount_to_send: crate::Balance = westend_runtime::ExistentialDeposit::get() * 1000;

    let assets: MultiAssets = (Here, amount_to_send).into();
    let fee_asset_item = 0;
    let westend_token_asset_id = 1u16;

    // Register the asset first
    FrontierTemplate::execute_with(|| {
        let root_origin = <FrontierTemplate as Chain>::RuntimeOrigin::root();

        assert_ok!(
            <FrontierTemplate as FrontierTemplatePallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                MultiLocation::parent(),
                westend_token_asset_id.into(),
                EthereumReceiver::get(),
                true,
                1
            )
        );
        // we register the asset but we never rate it
    });

    // Send XCM message from Westend
    Westend::execute_with(|| {
        assert_ok!(
            <Westend as WestendPallet>::XcmPallet::limited_reserve_transfer_assets(
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
        assert_expected_events!(
            FrontierTemplate,
            vec![
                RuntimeEvent::DmpQueue(
                    cumulus_pallet_dmp_queue::Event::ExecutedDownward {
                        outcome, ..
                    }) => {
                    outcome: {
                        outcome.clone().ensure_complete().is_err()
                    },
                },
            ]
        );
        type ForeignAssets = <FrontierTemplate as FrontierTemplatePallet>::ForeignAssets;

        // Assert receiver should not have received funds
        assert_eq!(
            <ForeignAssets as frame_support::traits::fungibles::Inspect<_>>::balance(
                westend_token_asset_id.into(),
                &EthereumReceiver::get(),
            ),
            0
        );
    });
}

#[test]
fn cannot_receive_tokens_from_the_relay_if_no_token_is_registered() {
    // XcmPallet reserve transfer arguments
    let alice_origin = <Westend as Chain>::RuntimeOrigin::signed(WestendSender::get());

    let frontier_template_dest: VersionedMultiLocation = MultiLocation {
        parents: 0,
        interior: X1(Parachain(2001u32)),
    }
    .into();

    let frontier_template_beneficiary: VersionedMultiLocation = MultiLocation {
        parents: 0,
        interior: X1(AccountKey20 {
            network: None,
            key: EthereumReceiver::get().into(),
        }),
    }
    .into();

    let amount_to_send: crate::Balance = westend_runtime::ExistentialDeposit::get() * 1000;

    let assets: MultiAssets = (Here, amount_to_send).into();
    let fee_asset_item = 0;
    let westend_token_asset_id = 1u16;

    // Send XCM message from Westend
    Westend::execute_with(|| {
        assert_ok!(
            <Westend as WestendPallet>::XcmPallet::limited_reserve_transfer_assets(
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
        assert_expected_events!(
            FrontierTemplate,
            vec![
                RuntimeEvent::DmpQueue(
                    cumulus_pallet_dmp_queue::Event::ExecutedDownward {
                        outcome, ..
                    }) => {
                    outcome: {
                        outcome.clone().ensure_complete().is_err()
                    },
                },
            ]
        );
        type ForeignAssets = <FrontierTemplate as FrontierTemplatePallet>::ForeignAssets;

        // Assert receiver should not have received funds
        assert_eq!(
            <ForeignAssets as frame_support::traits::fungibles::Inspect<_>>::balance(
                westend_token_asset_id.into(),
                &EthereumReceiver::get(),
            ),
            0
        );
    });
}
