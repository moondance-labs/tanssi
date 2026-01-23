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

#![cfg(test)]

use {
    crate::{
        tests::common::*, Historical, MaintenanceMode, RuntimeCall, RuntimeEvent, SessionKeys,
    },
    alloc::vec,
    frame_support::{assert_noop, assert_ok, traits::KeyOwnerProofSystem},
    pallet_assets::Instance1,
    sp_application_crypto::Pair,
    sp_runtime::traits::Dispatchable,
};

#[test]
fn maintenance_mode_can_be_set_by_sudo() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_sudo(AccountId::from(ALICE))
        .build()
        .execute_with(|| {
            run_to_block(2);
            // Root should be able to enter maintenance mode
            assert_ok!(MaintenanceMode::enter_maintenance_mode(root_origin(),));
        });
}

#[test]
fn asset_maintenance_mode_cannot_be_set_by_signed_origin() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_sudo(AccountId::from(ALICE))
        .build()
        .execute_with(|| {
            run_to_block(2);
            // Alice should not be able to execute this extrinsic
            assert_noop!(
                MaintenanceMode::enter_maintenance_mode(origin_of(ALICE.into())),
                crate::DispatchError::BadOrigin
            );
        });
}

#[test]
fn test_filtered_calls_maintenance_mode() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_sudo(AccountId::from(ALICE))
        .build()
        .execute_with(|| {
            run_to_block(2);
            assert_ok!(MaintenanceMode::enter_maintenance_mode(root_origin()));

            assert_call_filtered(RuntimeCall::Balances(
                pallet_balances::Call::<Runtime>::transfer_allow_death {
                    dest: AccountId::from(BOB).into(),
                    value: 1 * UNIT,
                },
            ));

            assert_call_filtered(RuntimeCall::Registrar(
                runtime_common::paras_registrar::Call::<Runtime>::reserve {},
            ));

            let alice_keys = get_authority_keys_from_seed(&AccountId::from(ALICE).to_string());
            assert_call_filtered(RuntimeCall::Session(
                pallet_session::Call::<Runtime>::set_keys {
                    keys: SessionKeys {
                        babe: alice_keys.babe.clone(),
                        grandpa: alice_keys.grandpa.clone(),
                        para_validator: alice_keys.para_validator.clone(),
                        para_assignment: alice_keys.para_assignment.clone(),
                        authority_discovery: alice_keys.authority_discovery.clone(),
                        beefy: alice_keys.beefy.clone(),
                        nimbus: alice_keys.nimbus.clone(),
                    },
                    proof: vec![],
                },
            ));
            assert_call_filtered(RuntimeCall::System(frame_system::Call::<Runtime>::remark {
                remark: vec![],
            }));

            assert_call_filtered(RuntimeCall::PooledStaking(pallet_pooled_staking::Call::<
                Runtime,
            >::update_candidate_position {
                candidates: vec![],
            }));
            assert_call_filtered(RuntimeCall::Identity(
                pallet_identity::Call::<Runtime>::clear_identity {},
            ));
            assert_call_filtered(RuntimeCall::XcmPallet(
                pallet_xcm::Call::<Runtime>::remove_all_authorized_aliases {},
            ));
            assert_call_filtered(RuntimeCall::EthereumTokenTransfers(
                pallet_ethereum_token_transfers::Call::<Runtime>::transfer_native_token {
                    amount: 1 * UNIT,
                    recipient: sp_core::H160::random(),
                },
            ));
            assert_call_filtered(RuntimeCall::OnDemandAssignmentProvider(
                runtime_parachains::on_demand::Call::<Runtime>::place_order_allow_death {
                    max_amount: 1 * UNIT,
                    para_id: 1u32.into(),
                },
            ));
            assert_call_filtered(RuntimeCall::ContainerRegistrar(pallet_registrar::Call::<
                Runtime,
            >::deregister {
                para_id: 1u32.into(),
            }));
            assert_call_filtered(RuntimeCall::ServicesPayment(
                pallet_services_payment::Call::<Runtime>::purchase_credits {
                    para_id: 1u32.into(),
                    credit: 100,
                },
            ));
            assert_call_filtered(RuntimeCall::DataPreservers(pallet_data_preservers::Call::<
                Runtime,
            >::delete_profile {
                profile_id: 1u32.into(),
            }));
            assert_call_filtered(RuntimeCall::Hrmp(
                runtime_parachains::hrmp::Call::<Runtime>::hrmp_accept_open_channel {
                    sender: 1u32.into(),
                },
            ));
            assert_call_filtered(RuntimeCall::StreamPayment(pallet_stream_payment::Call::<
                Runtime,
            >::close_stream {
                stream_id: 1u64,
            }));
            assert_call_filtered(RuntimeCall::Treasury(
                pallet_treasury::Call::<Runtime>::check_status { index: 1u32 },
            ));
            assert_call_filtered(RuntimeCall::ForeignAssets(pallet_assets::Call::<
                Runtime,
                Instance1,
            >::touch {
                id: 1u16,
            }));
            assert_call_filtered(RuntimeCall::ForeignAssetsCreator(
                pallet_foreign_asset_creator::Call::<Runtime>::destroy_foreign_asset {
                    asset_id: 1u16,
                },
            ));
        });
}

#[test]
fn test_non_filtered_calls_maintenance_mode() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_sudo(AccountId::from(ALICE))
        .build()
        .execute_with(|| {
            run_to_block(2);
            assert_ok!(MaintenanceMode::enter_maintenance_mode(root_origin()));
            let babe_key = get_pair_from_seed::<babe_primitives::AuthorityId>(
                &AccountId::from(ALICE).to_string(),
            );
            let grandpa_key = get_pair_from_seed::<grandpa_primitives::AuthorityId>(
                &AccountId::from(ALICE).to_string(),
            );

            let babe_equivocation_proof = generate_babe_equivocation_proof(&babe_key);

            let set_id = Grandpa::current_set_id();

            let grandpa_equivocation_proof = generate_grandpa_equivocation_proof(
                set_id,
                (1, sp_core::H256::random(), 1, &grandpa_key),
                (1, sp_core::H256::random(), 1, &grandpa_key),
            );

            let grandpa_key_owner_proof =
                Historical::prove((grandpa_primitives::KEY_TYPE, grandpa_key.public())).unwrap();

            let babe_key_owner_proof =
                Historical::prove((babe_primitives::KEY_TYPE, babe_key.public())).unwrap();

            assert_call_not_filtered(RuntimeCall::Babe(
                pallet_babe::Call::<Runtime>::report_equivocation_unsigned {
                    equivocation_proof: Box::new(babe_equivocation_proof),
                    key_owner_proof: babe_key_owner_proof,
                },
            ));

            assert_call_not_filtered(RuntimeCall::Timestamp(
                pallet_timestamp::Call::<Runtime>::set { now: 1u64 },
            ));

            assert_call_not_filtered(RuntimeCall::CollatorConfiguration(
                pallet_configuration::Call::<Runtime>::set_max_collators { new: 1u32 },
            ));

            assert_call_not_filtered(RuntimeCall::TanssiInvulnerables(
                pallet_invulnerables::Call::<Runtime>::add_invulnerable {
                    who: AccountId::from(ALICE),
                },
            ));

            assert_call_not_filtered(RuntimeCall::AuthorNoting(pallet_author_noting::Call::<
                Runtime,
            >::kill_author_data {
                para_id: 2000u32.into(),
            }));

            assert_call_not_filtered(RuntimeCall::ExternalValidators(
                pallet_external_validators::Call::<Runtime>::skip_external_validators {
                    skip: true,
                },
            ));

            assert_call_not_filtered(RuntimeCall::EthereumInboundQueue(
                snowbridge_pallet_inbound_queue::Call::<Runtime>::set_operating_mode {
                    mode: snowbridge_core::BasicOperatingMode::Normal,
                },
            ));

            assert_call_not_filtered(RuntimeCall::Grandpa(
                pallet_grandpa::Call::<Runtime>::report_equivocation_unsigned {
                    equivocation_proof: Box::new(grandpa_equivocation_proof),
                    key_owner_proof: grandpa_key_owner_proof,
                },
            ));

            assert_call_not_filtered(RuntimeCall::EthereumBeaconClient(
                snowbridge_pallet_ethereum_client::Call::<Runtime>::force_checkpoint {
                    update: Box::new(snowbridge_pallet_ethereum_client::mock::load_checkpoint_update_fixture()),
                },
            ));

            assert_call_not_filtered(RuntimeCall::Sudo(
                pallet_sudo::Call::<Runtime>::sudo {
                    call: Box::new(RuntimeCall::System(frame_system::Call::<Runtime>::remark{
                        remark: vec![]
                    })),
                },
            ));

            assert_call_not_filtered(RuntimeCall::Multisig(
                pallet_multisig::Call::<Runtime>::as_multi_threshold_1 {
                    other_signatories: vec![AccountId::from(BOB)],
                    call: Box::new(RuntimeCall::ExternalValidators(
                        pallet_external_validators::Call::<Runtime>::skip_external_validators {
                            skip: true,
                        },
                    )),
                },
            ));
        });
}

#[test]
fn test_assert_utility_does_not_bypass_filters() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_sudo(AccountId::from(ALICE))
        .build()
        .execute_with(|| {
            run_to_block(2);
            assert_ok!(MaintenanceMode::enter_maintenance_mode(root_origin()));

            // In this case, the call is not filtered
            assert_call_not_filtered(RuntimeCall::Utility(
                pallet_utility::Call::<Runtime>::batch {
                    calls: vec![RuntimeCall::Balances(
                        pallet_balances::Call::<Runtime>::transfer_allow_death {
                            dest: AccountId::from(BOB).into(),
                            value: 1 * UNIT,
                        },
                    )],
                },
            ));

            let batch_interrupt_events = System::events()
                .iter()
                .filter(|r| {
                    matches!(
                        r.event,
                        RuntimeEvent::Utility(pallet_utility::Event::BatchInterrupted { .. },)
                    )
                })
                .count();

            assert_eq!(
                batch_interrupt_events, 1,
                "batchInterrupted event should be emitted!"
            );
        });
}

fn assert_call_filtered(call: RuntimeCall) {
    assert_noop!(
        call.dispatch(<Runtime as frame_system::Config>::RuntimeOrigin::signed(
            AccountId::from(ALICE)
        )),
        frame_system::Error::<Runtime>::CallFiltered
    );
}

fn assert_call_not_filtered(call: RuntimeCall) {
    let res = call
        .clone()
        .dispatch(<Runtime as frame_system::Config>::RuntimeOrigin::signed(
            AccountId::from(ALICE),
        ));

    assert!(
        res != Err(frame_system::Error::<Runtime>::CallFiltered.into()),
        "Call is filtered {:?}",
        call
    );
}
