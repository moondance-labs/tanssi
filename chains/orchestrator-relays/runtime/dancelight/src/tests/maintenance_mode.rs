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
    crate::{tests::common::*, MaintenanceMode, RuntimeCall, SessionKeys},
    alloc::vec,
    frame_support::{assert_noop, assert_ok},
    pallet_assets::Instance1,
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
            // Alice should be able to execute this extrinsic
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
            // Alice should be able to execute this extrinsic
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
            assert_call_filtered(RuntimeCall::AssetRate(
                pallet_asset_rate::Call::<Runtime>::create {
                    asset_kind: Box::new(()),
                    rate: 1u128.into(),
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
            assert_call_filtered(RuntimeCall::EthereumSystemV2(
                snowbridge_pallet_system_v2::Call::<Runtime>::set_operating_mode {
                    mode: snowbridge_outbound_queue_primitives::OperatingMode::Normal,
                },
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
            assert_call_filtered(RuntimeCall::BridgeRelayers(pallet_bridge_relayers::Call::<
                Runtime,
            >::deregister {}));
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
