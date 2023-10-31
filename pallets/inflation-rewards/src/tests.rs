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
    crate::{mock::*, Config, *},
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
        let container_author_balance_2 = get_balance(&container_author_2);

        let total_supply_0 = get_total_issuance();
        <Pallet<Test> as Hooks<u64>>::on_initialize(1);
        let total_supply_1 = get_total_issuance();

        let new_supply_1 = total_supply_1 - total_supply_0;

        // Note container author
        let registered_para_ids = <Test as Config>::ContainerChains::current_container_chains();
        <Pallet<Test> as AuthorNotingHook<AccountId>>::on_container_author_noted(
            &container_author,
            1,
            registered_para_ids[0],
        );

        // Author should be rewarded immediately
        assert_eq!(
            get_balance(&container_author),
            // 70% rewards for 2 chains, so 35% per chain
            container_author_balance + (Permill::from_percent(35) * new_supply_1),
        );

        <Pallet<Test> as Hooks<u64>>::on_initialize(2);
        let total_supply_2 = get_total_issuance();
        let new_supply_2 = total_supply_2 - total_supply_1;

        // Note next container author
        <Pallet<Test> as AuthorNotingHook<AccountId>>::on_container_author_noted(
            &container_author_2,
            2,
            registered_para_ids[0],
        );

        // Author should be rewarded immediately
        assert_eq!(
            get_balance(&container_author_2),
            // 70% rewards for 2 chains, so 35% per chain
            container_author_balance_2 + (Permill::from_percent(35) * new_supply_2),
        );
    });
}

#[test]
fn test_cannot_reward_twice_in_same_tanssi_block() {
    new_test_ext().execute_with(|| {
        let container_author = 2;
        let container_author_balance = get_balance(&container_author);

        let total_supply_0 = get_total_issuance();
        <Pallet<Test> as Hooks<u64>>::on_initialize(1);
        let total_supply_1 = get_total_issuance();

        let new_supply_1 = total_supply_1 - total_supply_0;

        // Note container author
        let registered_para_ids = <Test as Config>::ContainerChains::current_container_chains();
        <Pallet<Test> as AuthorNotingHook<AccountId>>::on_container_author_noted(
            &container_author,
            1,
            registered_para_ids[0],
        );

        // Regardless if we inject a new block, we cannot reward twice the same paraId
        <Pallet<Test> as AuthorNotingHook<AccountId>>::on_container_author_noted(
            &container_author,
            2,
            registered_para_ids[0],
        );

        // Author should be rewarded only once
        assert_eq!(
            get_balance(&container_author),
            // 70% rewards for 2 chains, so 35% per chain
            container_author_balance + (Permill::from_percent(35) * new_supply_1),
        );
    });
}

#[test]
fn test_non_claimed_rewards_go_to_on_unbalanced() {
    new_test_ext().execute_with(|| {
        let container_author = 2;
        let container_author_balance = get_balance(&container_author);

        <Pallet<Test> as Hooks<u64>>::on_initialize(1);
        let on_unbalanced_account = get_balance(&OnUnbalancedInflationAccount::get());

        let total_supply_1 = get_total_issuance();

        // We initilize the next block without claiming rewards for the container
        // author should have not been rewarded and the onUNbalanced hook should kick in
        // we use block 2 because it has reminder
        <Pallet<Test> as Hooks<u64>>::on_initialize(2);

        let total_supply_2 = get_total_issuance();

        let new_supply_2 = total_supply_2 - total_supply_1;

        // OnUnbalancedInflationAccount::get() should be rewarded with the non-claimed
        // rewards
        // The onUnbalanedInflationAccount should have:
        // the non-reward portion ((Permill::from_percent(30) * new_supply_1))
        // the reminder ((Permill::from_percent(70) * suppl7 % number of container chains))
        // the non-claimed rewards
        let staking_rewards = Permill::from_percent(70) * new_supply_2;
        let non_staking_rewards = new_supply_2 - staking_rewards;
        // (orchestrator plus container);
        let reminder = staking_rewards % 2;

        assert_eq!(
            get_balance(&OnUnbalancedInflationAccount::get()),
            // 70% rewards for 2 chains, so 35% per chain
            on_unbalanced_account
                + non_staking_rewards
                + reminder
                + (Permill::from_percent(35) * new_supply_2),
        );

        // and the author is not rewarded
        assert_eq!(
            get_balance(&container_author),
            // 70% rewards for 2 chains, so 35% per chain
            container_author_balance,
        );
    });
}
