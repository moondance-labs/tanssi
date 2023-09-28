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
    crate::{*, mock::*, Config},
    frame_support::{pallet_prelude::*, traits::fungible::Inspect},
    sp_runtime::Permill,
};

fn get_balance(who: &AccountId) -> Balance {
    <<Test as Config>::Currency as Inspect<AccountId>>::balance(who)
}

fn get_total_issuance() -> Balance {
    <<Test as Config>::Currency as Inspect<AccountId>>::total_issuance()
}

#[test]
fn test_increase_supply() {
    new_test_ext().execute_with(|| {
        let total_supply_0 = get_total_issuance();

        <Pallet<Test> as Hooks<u64>>::on_initialize(1);
        let total_supply_1 = get_total_issuance();
        assert_eq!(
            total_supply_1,
            total_supply_0 + (<Test as Config>::InflationRate::get() * total_supply_0),
        );
        
        <Pallet<Test> as Hooks<u64>>::on_initialize(2);
        let total_supply_2 = get_total_issuance();
        assert_eq!(
            total_supply_2,
            total_supply_1 + (<Test as Config>::InflationRate::get() * total_supply_1),
        );
    });
}

#[test]
fn test_undistributed_rewards() {
    new_test_ext().execute_with(|| {
        let total_supply_0 = get_total_issuance();
        let initial_balance = get_balance(&OnUnbalancedInflationAccount::get());

        <Pallet<Test> as Hooks<u64>>::on_initialize(1);
        let total_supply_1 = get_total_issuance();

        let new_supply = total_supply_1 - total_supply_0;

        // The OnUnbalancedInflationAccount should receive 30% of the new supply
        assert_eq!(
            get_balance(&OnUnbalancedInflationAccount::get()),
            initial_balance + (Permill::from_percent(30) * new_supply),
        );


    });
}

#[test]
fn test_reward_orchestrator_author() {
    new_test_ext().execute_with(|| {
        let author = <Test as Config>::GetSelfChainBlockAuthor::get();
        let author_balance = get_balance(&author);

        let total_supply_0 = get_total_issuance();
        <Pallet<Test> as Hooks<u64>>::on_initialize(1);
        let total_supply_1 = get_total_issuance();

        let new_supply = total_supply_1 - total_supply_0;

        assert_eq!(
            get_balance(&author),
            // 70% rewards for 2 chains, so 35% per chain
            author_balance + (Permill::from_percent(35) * new_supply), 
        );
    });
}

#[test]
fn test_reward_orchestrator_author_less_if_more_chains() {
    new_test_ext().execute_with(|| {
        // Add 2 container chains
        MockData::mutate(|data| {
            data.container_chains.try_push(1002.into()).unwrap();
            data.container_chains.try_push(1003.into()).unwrap();
        });

        let author = <Test as Config>::GetSelfChainBlockAuthor::get();
        let author_balance = get_balance(&author);

        let total_supply_0 = get_total_issuance();
        <Pallet<Test> as Hooks<u64>>::on_initialize(1);
        let total_supply_1 = get_total_issuance();

        let new_supply = total_supply_1 - total_supply_0;

        assert_eq!(
            get_balance(&author),
            // 70% rewards for 3 chains, so 17.5% per chain
            author_balance + (Permill::from_perthousand(175) * new_supply), 
        );
    });
}

#[test]
fn test_reward_container_chain_author() {
    new_test_ext().execute_with(|| {
        let container_author = 2;
        let container_author_2 = 3;
        let container_author_balance = get_balance(&container_author);

        let total_supply_0 = get_total_issuance();
        <Pallet<Test> as Hooks<u64>>::on_initialize(1);
        let total_supply_1 = get_total_issuance();

        let new_supply = total_supply_1 - total_supply_0;

        // Note container author
        let registered_para_ids = <Test as Config>::ContainerChains::current_container_chains();
        <Pallet<Test> as AuthorNotingHook<AccountId>>::on_container_author_noted(
            &container_author,
            1,
            registered_para_ids[0],
        );

        // Note next container author
        <Pallet<Test> as AuthorNotingHook<AccountId>>::on_container_author_noted(
            &container_author_2,
            2,
            registered_para_ids[0],
        );

        assert_eq!(
            get_balance(&container_author),
            // 70% rewards for 2 chains, so 35% per chain
            container_author_balance + (Permill::from_percent(35) * new_supply), 
        );
    });
}

