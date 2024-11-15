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
    crate::{tests::common::*, ContainerRegistrar, Paras, Registrar, ServicesPayment},
    cumulus_primitives_core::{relay_chain::HeadData, ParaId},
    frame_support::assert_ok,
    sp_std::vec,
};

#[test]
fn test_can_buy_credits_before_registering_para() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);

            // Try to buy the maximum amount of credits
            let balance_before = System::account(AccountId::from(ALICE)).data.free;
            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                2000.into(),
                block_credits_to_required_balance(u32::MAX, 2000.into())
            ));
            let balance_after = System::account(AccountId::from(ALICE)).data.free;

            // Now parachain tank should have this amount
            let balance_tank = System::account(ServicesPayment::parachain_tank(2000.into()))
                .data
                .free;

            assert_eq!(
                balance_tank,
                block_credits_to_required_balance(u32::MAX, 2000.into())
            );

            let expected_cost = block_credits_to_required_balance(u32::MAX, 2000.into());
            assert_eq!(balance_before - balance_after, expected_cost);
        });
}

#[test]
fn test_can_buy_credits_before_registering_para_and_receive_free_credits() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .build()
        .execute_with(|| {
            run_to_block(2);

            // Try to buy (FreeBlockProductionCredits - 1) credits
            let balance_before = System::account(AccountId::from(ALICE)).data.free;
            assert_ok!(ServicesPayment::purchase_credits(
                origin_of(ALICE.into()),
                2000.into(),
                block_credits_to_required_balance(
                    crate::FreeBlockProductionCredits::get() - 1,
                    2000.into()
                )
            ));
            let balance_after = System::account(AccountId::from(ALICE)).data.free;

            // Now parachain tank should have this amount
            let balance_tank = System::account(ServicesPayment::parachain_tank(2000.into()))
                .data
                .free;

            assert_eq!(
                balance_tank,
                block_credits_to_required_balance(
                    crate::FreeBlockProductionCredits::get() - 1,
                    2000.into()
                )
            );

            let expected_cost = block_credits_to_required_balance(
                crate::FreeBlockProductionCredits::get() - 1,
                2000.into(),
            );
            assert_eq!(balance_before - balance_after, expected_cost);

            // Now register para
            assert_ok!(Registrar::reserve(origin_of(ALICE.into())));
            assert_ok!(ContainerRegistrar::register(
                origin_of(ALICE.into()),
                2000.into(),
                get_genesis_data_with_validation_code().0,
                Some(HeadData(vec![1u8, 2u8, 3u8]))
            ));

            run_to_session(2);

            // We need to accept the validation code, so that the para is onboarded after 2 sessions.
            assert_ok!(Paras::add_trusted_validation_code(
                root_origin(),
                get_genesis_data_with_validation_code().1.into()
            ));
            run_to_session(4);

            set_dummy_boot_node(origin_of(ALICE.into()), 2000.into());

            assert_ok!(ContainerRegistrar::mark_valid_for_collating(
                root_origin(),
                2000.into()
            ));

            // We received free credits, because we cannot have more than FreeBlockProductionCredits
            let credits =
                pallet_services_payment::BlockProductionCredits::<Runtime>::get(ParaId::from(2000))
                    .unwrap_or_default();
            assert_eq!(credits, crate::FreeBlockProductionCredits::get());
        });
}
