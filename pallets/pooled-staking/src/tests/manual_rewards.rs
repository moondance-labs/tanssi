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

use crate::PoolsKey;

use super::*;

#[test]
fn first_delegation_init_checkpoint() {
    ExtBuilder::default().build().execute_with(|| {
        // Set counter to simulate past rewards.
        // New delegator should not receive any rewards when joining
        // and their checkpoint should be set to the current counter.
        let counter = 10;
        crate::Pools::<Runtime>::set(
            ACCOUNT_CANDIDATE_1,
            &PoolsKey::ManualRewardsCounter,
            counter,
        );

        let amount = 2 * InitialManualClaimShareValue::get();
        do_full_delegation::<pools::ManualRewards<Runtime>>(
            ACCOUNT_CANDIDATE_1,
            ACCOUNT_DELEGATOR_1,
            amount,
            amount,
            0,
        );

        let checkpoint = crate::Pools::<Runtime>::get(
            ACCOUNT_CANDIDATE_1,
            &PoolsKey::ManualRewardsCheckpoint {
                delegator: ACCOUNT_DELEGATOR_1,
            },
        );
        assert_eq!(checkpoint, counter);
    });
}

#[test]
fn second_delegation_transfer_rewards() {
    ExtBuilder::default().build().execute_with(|| {
        let amount = 2 * InitialManualClaimShareValue::get();
        do_full_delegation::<pools::ManualRewards<Runtime>>(
            ACCOUNT_CANDIDATE_1,
            ACCOUNT_DELEGATOR_1,
            amount,
            amount,
            0,
        );

        // Set counter to simulate rewards.
        let counter = 10;
        crate::Pools::<Runtime>::set(
            ACCOUNT_CANDIDATE_1,
            &PoolsKey::ManualRewardsCounter,
            counter,
        );

        let expected_rewards = 20; // 10 coins (counter) * 2 shares
        do_full_delegation::<pools::ManualRewards<Runtime>>(
            ACCOUNT_CANDIDATE_1,
            ACCOUNT_DELEGATOR_1,
            amount,
            amount,
            expected_rewards,
        );

        let checkpoint = crate::Pools::<Runtime>::get(
            ACCOUNT_CANDIDATE_1,
            &PoolsKey::ManualRewardsCheckpoint {
                delegator: ACCOUNT_DELEGATOR_1,
            },
        );
        assert_eq!(checkpoint, counter);
    });
}

#[test]
fn undelegation_transfer_rewards() {
    ExtBuilder::default().build().execute_with(|| {
        let amount = 2 * InitialManualClaimShareValue::get();
        do_full_delegation::<pools::ManualRewards<Runtime>>(
            ACCOUNT_CANDIDATE_1,
            ACCOUNT_DELEGATOR_1,
            amount,
            amount,
            0,
        );

        // Set counter to simulate rewards.
        let counter = 10;
        crate::Pools::<Runtime>::set(
            ACCOUNT_CANDIDATE_1,
            &PoolsKey::ManualRewardsCounter,
            counter,
        );

        let expected_rewards = 20; // 10 coins (counter) * 2 shares

        let final_amount = 2 * InitialManualClaimShareValue::get();
        let leaving_amount = round_down(final_amount, 3); // test leaving rounding

        do_request_undelegation::<pools::ManualRewards<Runtime>>(
            ACCOUNT_CANDIDATE_1,
            ACCOUNT_DELEGATOR_1,
            final_amount,
            final_amount,
            leaving_amount,
            expected_rewards,
        );

        let checkpoint = crate::Pools::<Runtime>::get(
            ACCOUNT_CANDIDATE_1,
            &PoolsKey::ManualRewardsCheckpoint {
                delegator: ACCOUNT_DELEGATOR_1,
            },
        );
        assert_eq!(checkpoint, counter);
    });
}
