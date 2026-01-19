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

//! Tests for AllowExplicitUnpaidExecutionFrom<Equals<Parent>> barrier on container chains
//!
//! Container chains have the barrier: AllowExplicitUnpaidExecutionFrom<Equals<ParentLocation>>
//! This allows ONLY the parent (relay chain) to send UnpaidExecution messages.
//! User-initiated messages should still be charged fees.

use {
    dancelight_emulated_chain::{dancelight_runtime::Dmp, DancelightRelayPallet},
    dancelight_system_emulated_network::{
        DancelightRelay as Dancelight, DancelightSender, SimpleTemplateEmptyReceiver,
        SimpleTemplatePara as SimpleTemplate,
    },
    frame_support::{assert_ok, traits::fungible::Inspect, weights::Weight},
    simple_template_emulated_chain::{genesis::PARA_ID, SimpleTemplateParaPallet},
    sp_runtime::FixedU128,
    xcm::{latest::prelude::*, VersionedLocation, VersionedXcm},
    xcm_emulator::{
        assert_expected_events, bx, Chain, ConvertLocation, Encode, Parachain, TestExt,
    },
};

/// Test that the parent (relay chain) can send UnpaidExecution messages without being charged fees.
///
/// Verifies the AllowExplicitUnpaidExecutionFrom<Equals<ParentLocation>> barrier allows
/// the parent to execute transactions (system.remark_with_event) without fees.
#[test]
fn parent_can_send_unpaid_execution_without_fees() {
    // Make the parachain reachable
    Dancelight::execute_with(|| {
        Dmp::make_parachain_reachable(SimpleTemplate::para_id());
    });

    // Get parent's sovereign account on the container chain
    let parent_sovereign_on_simple = SimpleTemplate::execute_with(|| {
        container_chain_template_simple_runtime::xcm_config::LocationToAccountId::convert_location(
            &Location::parent(),
        )
        .unwrap()
    });

    // Fund the parent's sovereign account so we can track balance changes
    SimpleTemplate::execute_with(|| {
        let root_origin = <SimpleTemplate as Chain>::RuntimeOrigin::root();
        let initial_balance = 1 * container_chain_template_simple_runtime::UNIT;

        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::Balances::force_set_balance(
                root_origin,
                parent_sovereign_on_simple.clone().into(),
                initial_balance,
            )
        );
    });

    // Get parent sovereign account balance before
    let parent_balance_on_simple_before = SimpleTemplate::execute_with(|| {
        <SimpleTemplate as SimpleTemplateParaPallet>::Balances::balance(&parent_sovereign_on_simple)
    });

    // Send XCM with UnpaidExecution from the parent (Dancelight) to container chain
    Dancelight::execute_with(|| {
        let root_origin = <Dancelight as Chain>::RuntimeOrigin::root();
        let dest: VersionedLocation = Location {
            parents: 0,
            interior: [Parachain(PARA_ID)].into(),
        }
        .into();

        // Create XCM with UnpaidExecution followed by Transact (system.remark_with_event)
        let xcm: VersionedXcm<()> = VersionedXcm::V5(Xcm(vec![
            UnpaidExecution {
                weight_limit: Unlimited,
                check_origin: None,
            },
            Transact {
                origin_kind: OriginKind::SovereignAccount,
                fallback_max_weight: None,
                call: (0u8, 7u8, vec![3u8; 32].encode()).encode().into(), // Dummy call data for system.remark_with_event
            },
        ]));

        assert_ok!(<Dancelight as DancelightRelayPallet>::XcmPallet::send(
            root_origin,
            bx!(dest),
            bx!(xcm)
        ));
    });

    // Verify the XCM was successfully processed on the container chain
    SimpleTemplate::execute_with(|| {
        type RuntimeEvent = <SimpleTemplate as Chain>::RuntimeEvent;

        // Verify message processed successfully and remark event was emitted
        assert_expected_events!(
            SimpleTemplate,
            vec![
                RuntimeEvent::MessageQueue(pallet_message_queue::Event::Processed {
                    success: true,
                    ..
                }) => {},
                RuntimeEvent::System(frame_system::Event::Remarked {..}) => {},
            ]
        );

        // Get parent sovereign account balance after
        let parent_balance_on_simple_after =
            <SimpleTemplate as SimpleTemplateParaPallet>::Balances::balance(
                &parent_sovereign_on_simple,
            );

        // Assert no fees were deducted - balance should be exactly the same
        assert_eq!(
            parent_balance_on_simple_after, parent_balance_on_simple_before,
            "Parent sovereign account should not be charged any fees for UnpaidExecution"
        );
    });
}

/// Test that user-initiated transfer_assets charges fees even with the UnpaidExecution barrier.
///
/// This verifies that normal user transfers still pay fees on both the relay and container chains,
/// ensuring the AllowExplicitUnpaidExecutionFrom barrier doesn't affect regular user operations.
#[test]
fn user_transfer_assets_from_relay_to_container_charges_fees() {
    // Make the parachain reachable
    Dancelight::execute_with(|| {
        Dmp::make_parachain_reachable(SimpleTemplate::para_id());
    });

    let sender = DancelightSender::get();
    let receiver = SimpleTemplateEmptyReceiver::get();
    let amount_to_send: dancelight_runtime::Balance = 10_000_000_000_000_000; // 10 tokens

    // Register relay token on container chain
    let relay_token_asset_id = 1u16;
    SimpleTemplate::execute_with(|| {
        let root_origin = <SimpleTemplate as Chain>::RuntimeOrigin::root();

        // Create foreign asset from relay chain
        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                Location { parents: 1, interior: Here },
                relay_token_asset_id,
                receiver.clone(),
                true,
                1
            )
        );

        // Create asset rate
        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::AssetRate::create(
                root_origin,
                bx!(relay_token_asset_id),
                FixedU128::from_u32(1)
            )
        );
    });

    // Get sender balance before
    let sender_balance_before = Dancelight::execute_with(|| {
        <Dancelight as DancelightRelayPallet>::Balances::balance(&sender)
    });

    // Get receiver balance before
    let receiver_balance_before = SimpleTemplate::execute_with(|| {
        <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssets::balance(
            relay_token_asset_id,
            &receiver,
        )
    });

    // Send transfer_assets from user on relay chain
    Dancelight::execute_with(|| {
        let origin = <Dancelight as Chain>::RuntimeOrigin::signed(sender.clone());
        let dest: VersionedLocation = Location {
            parents: 0,
            interior: [Parachain(PARA_ID)].into(),
        }
        .into();
        let beneficiary: VersionedLocation = Location {
            parents: 0,
            interior: [Junction::AccountId32 {
                network: None,
                id: receiver.clone().into(),
            }]
            .into(),
        }
        .into();

        let assets: Vec<Asset> = vec![(Here, amount_to_send).into()];
        let fee_asset_item = 0;

        assert_ok!(
            <Dancelight as DancelightRelayPallet>::XcmPallet::limited_reserve_transfer_assets(
                origin,
                bx!(dest),
                bx!(beneficiary),
                bx!(assets.into()),
                fee_asset_item,
                WeightLimit::Unlimited,
            )
        );
    });

    // Verify fees were charged on relay chain
    let sender_balance_after = Dancelight::execute_with(|| {
        <Dancelight as DancelightRelayPallet>::Balances::balance(&sender)
    });

    // Sender should have paid: amount + XCM fees
    let total_cost = sender_balance_before - sender_balance_after;
    assert!(
        total_cost > amount_to_send,
        "User should pay XCM fees on relay. Cost: {}, Amount: {}",
        total_cost,
        amount_to_send
    );

    // Verify receiver got tokens (minus DMP fees on container)
    SimpleTemplate::execute_with(|| {
        type RuntimeEvent = <SimpleTemplate as Chain>::RuntimeEvent;

        // Message should be processed successfully
        assert_expected_events!(
            SimpleTemplate,
            vec![
                RuntimeEvent::MessageQueue(pallet_message_queue::Event::Processed {
                    success: true,
                    ..
                }) => {},
            ]
        );

        let receiver_balance_after =
            <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssets::balance(
                relay_token_asset_id,
                &receiver,
            );

        // Receiver should get tokens, but less than full amount due to container-side fees
        assert!(
            receiver_balance_after > receiver_balance_before,
            "Receiver should have received tokens"
        );
        assert!(
            receiver_balance_after < receiver_balance_before + amount_to_send,
            "Receiver should have paid DMP execution fees on container"
        );
    });
}

/// Test that user-initiated XCM with UnpaidExecution is rejected by the barrier.
///
/// Users attempting to bypass fees by including UnpaidExecution in their messages
/// will have their messages rejected at the barrier.
#[test]
fn user_unpaid_execution_with_transact_is_rejected() {
    // Make the parachain reachable
    Dancelight::execute_with(|| {
        Dmp::make_parachain_reachable(SimpleTemplate::para_id());
    });

    let sender = DancelightSender::get();

    // Try to send XCM with UnpaidExecution + Transact(system.remark)
    Dancelight::execute_with(|| {
        let origin = <Dancelight as Chain>::RuntimeOrigin::signed(sender.clone());
        let dest: VersionedLocation = Location {
            parents: 0,
            interior: [Parachain(PARA_ID)].into(),
        }
        .into();

        // Craft XCM trying to execute Transact with UnpaidExecution
        let malicious_xcm: VersionedXcm<()> = VersionedXcm::V5(Xcm(vec![
            UnpaidExecution {
                weight_limit: Unlimited,
                check_origin: None,
            },
            Transact {
                origin_kind: OriginKind::SovereignAccount,
                fallback_max_weight: Some(Weight::from_parts(1_000_000_000, 0)),
                call: vec![0u8; 32].into(), // Dummy call data for system.remark
            },
        ]));

        assert_ok!(<Dancelight as DancelightRelayPallet>::XcmPallet::send(
            origin,
            bx!(dest),
            bx!(malicious_xcm)
        ));
    });

    // Verify the XCM failed due to barrier rejection
    SimpleTemplate::execute_with(|| {
        type RuntimeEvent = <SimpleTemplate as Chain>::RuntimeEvent;

        // Should fail due to barrier rejection - ProcessingFailed
        assert_expected_events!(
            SimpleTemplate,
            vec![
                RuntimeEvent::MessageQueue(pallet_message_queue::Event::ProcessingFailed {
                    ..
                }) => {},
            ]
        );
    });
}

/// Test that users cannot bypass fees using UnpaidExecution + DepositAsset to mint free assets.
///
/// This attempts a malicious XCM with UnpaidExecution to withdraw and deposit assets for free,
/// which should fail at the barrier before any assets are transferred.
#[test]
fn user_unpaid_execution_with_deposit_asset_is_rejected() {
    Dancelight::execute_with(|| {
        Dmp::make_parachain_reachable(SimpleTemplate::para_id());
    });

    let sender = DancelightSender::get();
    let beneficiary = SimpleTemplateEmptyReceiver::get();

    // Get beneficiary balance before
    let beneficiary_balance_before = SimpleTemplate::execute_with(|| {
        <SimpleTemplate as SimpleTemplateParaPallet>::Balances::balance(&beneficiary)
    });

    // Try to send XCM with UnpaidExecution + WithdrawAsset + DepositAsset
    Dancelight::execute_with(|| {
        let origin = <Dancelight as Chain>::RuntimeOrigin::signed(sender.clone());
        let dest: VersionedLocation = Location {
            parents: 0,
            interior: [Parachain(PARA_ID)].into(),
        }
        .into();

        let amount = 5_000_000_000_000_000u128; // 5 tokens

        // Craft malicious XCM trying to deposit assets for free
        let malicious_xcm: VersionedXcm<()> = VersionedXcm::V5(Xcm(vec![
            UnpaidExecution {
                weight_limit: Unlimited,
                check_origin: None,
            },
            WithdrawAsset((Here, amount).into()),
            DepositAsset {
                assets: All.into(),
                beneficiary: Location {
                    parents: 0,
                    interior: [Junction::AccountId32 {
                        network: None,
                        id: beneficiary.clone().into(),
                    }]
                    .into(),
                },
            },
        ]));

        assert_ok!(<Dancelight as DancelightRelayPallet>::XcmPallet::send(
            origin,
            bx!(dest),
            bx!(malicious_xcm)
        ));
    });

    // Verify the XCM failed and no assets were deposited
    SimpleTemplate::execute_with(|| {
        type RuntimeEvent = <SimpleTemplate as Chain>::RuntimeEvent;

        // Should fail due to barrier rejection - ProcessingFailed
        assert_expected_events!(
            SimpleTemplate,
            vec![
                RuntimeEvent::MessageQueue(pallet_message_queue::Event::ProcessingFailed {
                    ..
                }) => {},
            ]
        );

        // Verify beneficiary balance unchanged
        let beneficiary_balance_after =
            <SimpleTemplate as SimpleTemplateParaPallet>::Balances::balance(&beneficiary);
        assert_eq!(
            beneficiary_balance_after, beneficiary_balance_before,
            "Beneficiary should not have received any assets from malicious XCM"
        );
    });
}
