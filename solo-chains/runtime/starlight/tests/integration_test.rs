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
    common::*, frame_support::assert_ok, sp_std::vec,
    starlight_runtime_constants::currency::EXISTENTIAL_DEPOSIT,
};

mod common;

const UNIT: Balance = 1_000_000_000_000_000_000;

#[test]
fn genesis_balances() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            assert_eq!(
                Balances::usable_balance(AccountId::from(ALICE)) + EXISTENTIAL_DEPOSIT,
                210_000 * UNIT,
            );
            assert_eq!(
                Balances::usable_balance(AccountId::from(BOB)) + EXISTENTIAL_DEPOSIT,
                100_000 * UNIT,
            );
        });
}

#[test]
fn test_configuration_on_session_change() {
    ExtBuilder::default().build().execute_with(|| {
        assert_eq!(CollatorConfiguration::config().max_collators, 100);
        assert_eq!(
            CollatorConfiguration::config().min_orchestrator_collators,
            2
        );
        assert_eq!(CollatorConfiguration::config().collators_per_container, 2);

        assert_ok!(
            CollatorConfiguration::set_max_collators(root_origin(), 50),
            ()
        );
        run_to_session(1u32);

        assert_ok!(
            CollatorConfiguration::set_min_orchestrator_collators(root_origin(), 20),
            ()
        );
        assert_eq!(CollatorConfiguration::config().max_collators, 100);
        assert_eq!(
            CollatorConfiguration::config().min_orchestrator_collators,
            2
        );
        assert_eq!(CollatorConfiguration::config().collators_per_container, 2);

        run_to_session(2u32);
        assert_ok!(
            CollatorConfiguration::set_collators_per_container(root_origin(), 10),
            ()
        );
        assert_eq!(CollatorConfiguration::config().max_collators, 50);
        assert_eq!(
            CollatorConfiguration::config().min_orchestrator_collators,
            2
        );
        assert_eq!(CollatorConfiguration::config().collators_per_container, 2);

        run_to_session(3u32);

        assert_eq!(CollatorConfiguration::config().max_collators, 50);
        assert_eq!(
            CollatorConfiguration::config().min_orchestrator_collators,
            20
        );
        assert_eq!(CollatorConfiguration::config().collators_per_container, 2);

        run_to_session(4u32);

        assert_eq!(CollatorConfiguration::config().max_collators, 50);
        assert_eq!(
            CollatorConfiguration::config().min_orchestrator_collators,
            20
        );
        assert_eq!(CollatorConfiguration::config().collators_per_container, 10);
    });
}
