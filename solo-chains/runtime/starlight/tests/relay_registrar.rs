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

mod common;
use {
    crate::common::*,
    frame_support::{assert_noop, assert_ok},
    runtime_common::paras_registrar,
    sp_std::vec,
    starlight_runtime::{Paras, Registrar},
};

const UNIT: Balance = 1_000_000_000_000_000_000;

#[test]
fn registrar_needs_a_reserved_para_id() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_next_free_para_id(2000u32.into())
        .build()
        .execute_with(|| {
            run_to_block(2);
            assert_noop!(
                Registrar::register(
                    origin_of(ALICE.into()),
                    100u32.into(),
                    vec![].into(),
                    vec![].into()
                ),
                paras_registrar::Error::<Runtime>::NotReserved
            );

            // After a reservation, we can register
            let next_para_id = paras_registrar::NextFreeParaId::<Runtime>::get();

            assert_ok!(Registrar::reserve(origin_of(ALICE.into())));

            assert_noop!(
                Registrar::register(
                    origin_of(ALICE.into()),
                    next_para_id,
                    vec![].into(),
                    vec![].into()
                ),
                paras_registrar::Error::<Runtime>::InvalidCode
            );

            let validation_code: cumulus_primitives_core::relay_chain::ValidationCode =
                vec![1u8; cumulus_primitives_core::relay_chain::MIN_CODE_SIZE as usize].into();
            assert_ok!(Registrar::register(
                origin_of(ALICE.into()),
                next_para_id,
                vec![].into(),
                validation_code.clone()
            ));

            assert!(Paras::lifecycle(next_para_id)
                .expect("para should be onboarding")
                .is_onboarding());
            // Two sessions later the para should be a parathread
            // But only if the pvf is accepted! which we havent done
            run_to_session(2);
            assert!(Paras::lifecycle(next_para_id)
                .expect("para should be onboarding")
                .is_onboarding());

            // Now let's accept the pvf, so that after 2 sesssions we have the chain onboarded
            assert_ok!(Paras::add_trusted_validation_code(
                root_origin(),
                validation_code
            ));
            run_to_session(4);

            // PVF accepted and the para should be a parathread
            assert!(Paras::lifecycle(next_para_id)
                .expect("para should be parathread")
                .is_parathread());
        });
}
