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
    crate::{tests::common::*, RuntimeCall},
    frame_support::assert_noop,
    sp_runtime::traits::Dispatchable,
};

#[test]
fn test_disabled_some_extrinsics_container_chain_management() {
    ExtBuilder::default().build().execute_with(|| {
        run_to_block(2);

        assert_noop!(
            RuntimeCall::ServicesPayment(pallet_services_payment::Call::set_given_free_credits {
                para_id: 2000.into(),
                given_free_credits: true,
            })
            .dispatch(<Runtime as frame_system::Config>::RuntimeOrigin::signed(
                AccountId::from(ALICE)
            )),
            frame_system::Error::<Runtime>::CallFiltered
        );

        assert_noop!(
            RuntimeCall::StreamPayment(pallet_stream_payment::Call::cancel_change_request {
                stream_id: 0u32.into(),
            })
            .dispatch(<Runtime as frame_system::Config>::RuntimeOrigin::signed(
                AccountId::from(ALICE)
            )),
            frame_system::Error::<Runtime>::CallFiltered
        );

        assert_noop!(
            RuntimeCall::DataPreservers(pallet_data_preservers::Call::delete_profile {
                profile_id: 0u32.into(),
            })
            .dispatch(<Runtime as frame_system::Config>::RuntimeOrigin::signed(
                AccountId::from(ALICE)
            )),
            frame_system::Error::<Runtime>::CallFiltered
        );
    });
}

#[test]
fn test_disabled_some_extrinsics_democracy() {
    ExtBuilder::default().build().execute_with(|| {
        run_to_block(2);

        assert_noop!(
            RuntimeCall::Treasury(pallet_treasury::Call::payout { index: 0u32 }).dispatch(
                <Runtime as frame_system::Config>::RuntimeOrigin::signed(AccountId::from(ALICE))
            ),
            frame_system::Error::<Runtime>::CallFiltered
        );

        assert_noop!(
            RuntimeCall::ConvictionVoting(pallet_conviction_voting::Call::undelegate {
                class: 0u16,
            })
            .dispatch(<Runtime as frame_system::Config>::RuntimeOrigin::signed(
                AccountId::from(ALICE)
            )),
            frame_system::Error::<Runtime>::CallFiltered
        );

        assert_noop!(
            RuntimeCall::Referenda(pallet_referenda::Call::place_decision_deposit { index: 0u32 })
                .dispatch(<Runtime as frame_system::Config>::RuntimeOrigin::signed(
                    AccountId::from(ALICE)
                )),
            frame_system::Error::<Runtime>::CallFiltered
        );

        assert_noop!(
            RuntimeCall::FellowshipCollective(pallet_ranked_collective::Call::add_member {
                who: sp_runtime::MultiAddress::Id(sp_runtime::AccountId32::from(BOB))
            })
            .dispatch(<Runtime as frame_system::Config>::RuntimeOrigin::signed(
                AccountId::from(ALICE)
            )),
            frame_system::Error::<Runtime>::CallFiltered
        );

        assert_noop!(
            RuntimeCall::FellowshipReferenda(pallet_referenda::Call::<
                Runtime,
                pallet_referenda::Instance2,
            >::cancel {
                index: 0u32
            })
            .dispatch(<Runtime as frame_system::Config>::RuntimeOrigin::signed(
                AccountId::from(ALICE)
            )),
            frame_system::Error::<Runtime>::CallFiltered
        );

        assert_noop!(
            RuntimeCall::Whitelist(pallet_whitelist::Call::whitelist_call {
                call_hash: Default::default()
            })
            .dispatch(<Runtime as frame_system::Config>::RuntimeOrigin::signed(
                AccountId::from(ALICE)
            )),
            frame_system::Error::<Runtime>::CallFiltered
        );

        assert_noop!(
            RuntimeCall::Preimage(pallet_preimage::Call::note_preimage { bytes: vec![] }).dispatch(
                <Runtime as frame_system::Config>::RuntimeOrigin::signed(AccountId::from(ALICE))
            ),
            frame_system::Error::<Runtime>::CallFiltered
        );
    });
}

#[test]
fn test_disabled_some_extrinsics_xcm() {
    ExtBuilder::default().build().execute_with(|| {
        run_to_block(2);

        assert_noop!(
            RuntimeCall::Hrmp(runtime_parachains::hrmp::Call::force_process_hrmp_open {
                channels: 0u32
            })
            .dispatch(<Runtime as frame_system::Config>::RuntimeOrigin::signed(
                AccountId::from(ALICE)
            )),
            frame_system::Error::<Runtime>::CallFiltered
        );

        assert_noop!(
            RuntimeCall::MessageQueue(pallet_message_queue::Call::reap_page {
                message_origin: crate::AggregateMessageOrigin::Ump(
                    runtime_parachains::inclusion::UmpQueueId::Para(1u32.into())
                ),
                page_index: 0u32
            })
            .dispatch(<Runtime as frame_system::Config>::RuntimeOrigin::signed(
                AccountId::from(ALICE)
            )),
            frame_system::Error::<Runtime>::CallFiltered
        );

        assert_noop!(
            RuntimeCall::AssetRate(pallet_asset_rate::Call::create {
                asset_kind: Box::new(()),
                rate: 0.into()
            })
            .dispatch(<Runtime as frame_system::Config>::RuntimeOrigin::signed(
                AccountId::from(ALICE)
            )),
            frame_system::Error::<Runtime>::CallFiltered
        );

        assert_noop!(
            RuntimeCall::XcmPallet(pallet_xcm::Call::force_default_xcm_version {
                maybe_xcm_version: None
            })
            .dispatch(<Runtime as frame_system::Config>::RuntimeOrigin::signed(
                AccountId::from(ALICE)
            )),
            frame_system::Error::<Runtime>::CallFiltered
        );
    });
}

#[test]
fn test_disabled_some_extrinsics_container_registrar() {
    ExtBuilder::default().build().execute_with(|| {
        run_to_block(2);

        assert_noop!(
            RuntimeCall::ContainerRegistrar(pallet_registrar::Call::deregister {
                para_id: 2000.into()
            })
            .dispatch(<Runtime as frame_system::Config>::RuntimeOrigin::signed(
                AccountId::from(ALICE)
            )),
            frame_system::Error::<Runtime>::CallFiltered
        );

        assert_noop!(
            RuntimeCall::OnDemandAssignmentProvider(
                runtime_parachains::on_demand::Call::place_order_allow_death {
                    para_id: 2000.into(),
                    max_amount: 0
                }
            )
            .dispatch(<Runtime as frame_system::Config>::RuntimeOrigin::signed(
                AccountId::from(ALICE)
            )),
            frame_system::Error::<Runtime>::CallFiltered
        );

        assert_noop!(
            RuntimeCall::Registrar(runtime_common::paras_registrar::Call::deregister {
                id: 2000.into()
            })
            .dispatch(<Runtime as frame_system::Config>::RuntimeOrigin::signed(
                AccountId::from(ALICE)
            )),
            frame_system::Error::<Runtime>::CallFiltered
        );
    });
}
