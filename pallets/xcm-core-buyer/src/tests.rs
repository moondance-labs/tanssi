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
    crate::{mock::*, *},
    frame_support::{assert_noop, assert_ok},
    sp_runtime::traits::BadOrigin,
};

#[test]
fn root_origin_can_force_buy_xcm() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            run_to_block(1);
            let para_id = 3333.into();

            assert_ok!(XcmCoreBuyer::force_buy_core(RuntimeOrigin::root(), para_id,));

            assert_eq!(events(), vec![Event::BuyCoreXcmSent { para_id }]);
        });
}

#[test]
fn signed_origin_cannot_force_buy_xcm() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            run_to_block(1);
            let para_id = 3333.into();

            assert_noop!(
                XcmCoreBuyer::force_buy_core(RuntimeOrigin::signed(ALICE), para_id),
                BadOrigin
            );
        });
}

#[test]
fn force_buy_two_messages_in_one_block_fails() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            run_to_block(1);
            let para_id = 3333.into();

            assert_ok!(XcmCoreBuyer::force_buy_core(RuntimeOrigin::root(), para_id,));

            assert_eq!(events(), vec![Event::BuyCoreXcmSent { para_id }]);

            assert_noop!(
                XcmCoreBuyer::force_buy_core(RuntimeOrigin::root(), para_id),
                Error::<Test>::OrderAlreadyExists
            );
        });
}

#[test]
fn force_buy_two_messages_in_two_consecutive_blocks_works() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            run_to_block(1);
            let para_id = 3333.into();

            assert_ok!(XcmCoreBuyer::force_buy_core(RuntimeOrigin::root(), para_id,));

            assert_eq!(events(), vec![Event::BuyCoreXcmSent { para_id }]);

            run_to_block(2);

            assert_ok!(XcmCoreBuyer::force_buy_core(RuntimeOrigin::root(), para_id,));

            assert_eq!(events(), vec![Event::BuyCoreXcmSent { para_id }]);
        });
}

#[test]
fn cannot_force_buy_invalid_para_id() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            run_to_block(1);
            let para_id = 2000.into();

            MockData::mutate(|m| {
                // Mock para_id 2000 as a container chain with collators, but not a parathread
                m.container_chain_collators.insert(2000.into(), vec![ALICE]);
            });

            assert_noop!(
                XcmCoreBuyer::force_buy_core(RuntimeOrigin::root(), para_id),
                Error::<Test>::NotAParathread
            );
        });
}

#[test]
fn cannot_force_buy_para_id_with_no_collators() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            run_to_block(1);
            let para_id = 3333.into();
            MockData::mutate(|m| {
                m.container_chain_collators = Default::default();
            });

            assert_noop!(
                XcmCoreBuyer::force_buy_core(RuntimeOrigin::root(), para_id),
                Error::<Test>::NoAssignedCollators
            );
        });
}
