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
    crate::{mock::*, CollatorContainerChain, Event, PendingCollatorContainerChain},
    dp_collator_assignment::AssignedCollators,
    std::collections::BTreeMap,
};

mod assign_full;
mod prioritize_invulnerables;
mod select_chains;

#[test]
fn assign_initial_collators() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.collators_per_parathread = 2;
            m.min_orchestrator_chain_collators = 5;
            m.max_orchestrator_chain_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            m.container_chains = vec![1001, 1002]
        });

        assert_eq!(assigned_collators(), initial_collators(),);
        run_to_block(6);

        assert_eq!(assigned_collators(), initial_collators(),);
        run_to_block(11);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![
                (1, 1000),
                (2, 1000),
                (3, 1000),
                (4, 1000),
                (5, 1000),
                (6, 1001),
                (7, 1001),
                (8, 1002),
                (9, 1002),
            ]),
        );
    });
}

#[test]
fn assign_collators_after_one_leaves_container() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.collators_per_parathread = 2;
            m.min_orchestrator_chain_collators = 5;
            m.max_orchestrator_chain_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            m.container_chains = vec![1001, 1002]
        });

        assert_eq!(assigned_collators(), initial_collators(),);
        run_to_block(6);

        assert_eq!(assigned_collators(), initial_collators(),);
        run_to_block(11);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![
                (1, 1000),
                (2, 1000),
                (3, 1000),
                (4, 1000),
                (5, 1000),
                (6, 1001),
                (7, 1001),
                (8, 1002),
                (9, 1002),
            ]),
        );

        MockData::mutate(|m| {
            // Remove 6
            m.collators = vec![1, 2, 3, 4, 5, /*6,*/ 7, 8, 9, 10];
        });

        run_to_block(16);
        run_to_block(21);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![
                (1, 1000),
                (2, 1000),
                (3, 1000),
                (4, 1000),
                (5, 1000),
                //(6, 1001),
                (7, 1001),
                (8, 1002),
                (9, 1002),
                // 10 is assigned in place of 6
                (10, 1001),
            ]),
        );
    });
}

#[test]
fn assign_collators_after_one_leaves_orchestrator_chain() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.collators_per_parathread = 2;
            m.min_orchestrator_chain_collators = 5;
            m.max_orchestrator_chain_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            m.container_chains = vec![1001, 1002]
        });

        assert_eq!(assigned_collators(), initial_collators(),);
        run_to_block(11);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![
                (1, 1000),
                (2, 1000),
                (3, 1000),
                (4, 1000),
                (5, 1000),
                (6, 1001),
                (7, 1001),
                (8, 1002),
                (9, 1002),
            ]),
        );

        MockData::mutate(|m| {
            // Remove 4
            m.collators = vec![1, 2, 3, /*4,*/ 5, 6, 7, 8, 9, 10];
        });
        run_to_block(21);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![
                (1, 1000),
                (2, 1000),
                (3, 1000),
                //(4, 1000),
                (5, 1000),
                (6, 1001),
                (7, 1001),
                (8, 1002),
                (9, 1002),
                // 10 is assigned in place of 4
                (10, 1000),
            ]),
        );
    });
}

#[test]
fn assign_collators_if_config_orchestrator_chain_collators_increases() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.collators_per_parathread = 2;
            m.min_orchestrator_chain_collators = 5;
            m.max_orchestrator_chain_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
            m.container_chains = vec![1001, 1002]
        });
        assert_eq!(assigned_collators(), initial_collators(),);
        run_to_block(11);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![
                (1, 1000),
                (2, 1000),
                (3, 1000),
                (4, 1000),
                (5, 1000),
                (6, 1001),
                (7, 1001),
                (8, 1002),
                (9, 1002),
            ]),
        );

        MockData::mutate(|m| {
            // Add 3 new collators to orchestrator_chain
            m.min_orchestrator_chain_collators = 8;
            m.max_orchestrator_chain_collators = 8;
        });

        run_to_block(21);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![
                (1, 1000),
                (2, 1000),
                (3, 1000),
                (4, 1000),
                (5, 1000),
                (6, 1001),
                (7, 1001),
                (8, 1002),
                (9, 1002),
                (10, 1000),
                (11, 1000),
                (12, 1000),
            ]),
        );
    });
}

#[test]
fn assign_collators_if_config_orchestrator_chain_collators_decreases() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.collators_per_parathread = 2;
            m.min_orchestrator_chain_collators = 5;
            m.max_orchestrator_chain_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
            m.container_chains = vec![1001, 1002]
        });
        assert_eq!(assigned_collators(), initial_collators(),);
        run_to_block(11);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![
                (1, 1000),
                (2, 1000),
                (3, 1000),
                (4, 1000),
                (5, 1000),
                (6, 1001),
                (7, 1001),
                (8, 1002),
                (9, 1002),
            ]),
        );

        MockData::mutate(|m| {
            // Remove 3 collators from orchestrator_chain
            m.min_orchestrator_chain_collators = 2;
            m.max_orchestrator_chain_collators = 2;
        });

        run_to_block(21);

        // The removed collators are random so no easy way to test the full list
        assert_eq!(assigned_collators().len(), 6,);
    });
}

#[test]
fn assign_collators_if_config_collators_per_container_increases() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.collators_per_parathread = 2;
            m.min_orchestrator_chain_collators = 5;
            m.max_orchestrator_chain_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
            m.container_chains = vec![1001, 1002]
        });

        assert_eq!(assigned_collators(), initial_collators(),);
        run_to_block(11);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![
                (1, 1000),
                (2, 1000),
                (3, 1000),
                (4, 1000),
                (5, 1000),
                (6, 1001),
                (7, 1001),
                (8, 1002),
                (9, 1002),
            ]),
        );

        MockData::mutate(|m| {
            // Add 2 new collators to each container_chain
            m.collators_per_container = 4;
        });

        run_to_block(21);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![
                (1, 1000),
                (2, 1000),
                (3, 1000),
                (4, 1000),
                (5, 1000),
                (6, 1001),
                (7, 1001),
                (8, 1002),
                (9, 1002),
                (10, 1001),
                (11, 1001),
                (12, 1002),
                (13, 1002),
            ]),
        );
    });
}

#[test]
fn assign_collators_if_container_chain_is_removed() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.collators_per_parathread = 2;
            m.min_orchestrator_chain_collators = 5;
            m.max_orchestrator_chain_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
            m.container_chains = vec![1001, 1002]
        });
        assert_eq!(assigned_collators(), initial_collators(),);
        run_to_block(11);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![
                (1, 1000),
                (2, 1000),
                (3, 1000),
                (4, 1000),
                (5, 1000),
                (6, 1001),
                (7, 1001),
                (8, 1002),
                (9, 1002),
            ]),
        );

        MockData::mutate(|m| {
            // Remove 1 container_chain
            m.container_chains = vec![1001 /*1002*/];
        });

        run_to_block(21);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![
                (1, 1000),
                (2, 1000),
                (3, 1000),
                (4, 1000),
                (5, 1000),
                (6, 1001),
                (7, 1001),
            ]),
        );
    });
}

#[test]
fn assign_collators_if_container_chain_is_added() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.collators_per_parathread = 2;
            m.min_orchestrator_chain_collators = 5;
            m.max_orchestrator_chain_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
            m.container_chains = vec![1001, 1002]
        });
        assert_eq!(assigned_collators(), initial_collators(),);
        run_to_block(11);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![
                (1, 1000),
                (2, 1000),
                (3, 1000),
                (4, 1000),
                (5, 1000),
                (6, 1001),
                (7, 1001),
                (8, 1002),
                (9, 1002),
            ]),
        );

        MockData::mutate(|m| {
            // Add 1 new container_chain
            m.container_chains = vec![1001, 1002, 1003];
        });

        run_to_block(21);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![
                (1, 1000),
                (2, 1000),
                (3, 1000),
                (4, 1000),
                (5, 1000),
                (6, 1001),
                (7, 1001),
                (8, 1002),
                (9, 1002),
                (10, 1003),
                (11, 1003),
            ]),
        );
    });
}

#[test]
fn assign_collators_after_decrease_num_collators() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.collators_per_parathread = 2;
            m.min_orchestrator_chain_collators = 5;
            m.max_orchestrator_chain_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
            m.container_chains = vec![1001, 1002]
        });
        assert_eq!(assigned_collators(), initial_collators(),);
        run_to_block(11);

        let initial_assignment = BTreeMap::from_iter(vec![
            (1, 1000),
            (2, 1000),
            (3, 1000),
            (4, 1000),
            (5, 1000),
            (6, 1001),
            (7, 1001),
            (8, 1002),
            (9, 1002),
        ]);
        assert_eq!(assigned_collators(), initial_assignment,);

        MockData::mutate(|m| {
            m.collators = vec![];
        });

        // Disable logs in this test because it will print:
        //   Error in collator assignment, will keep previous assignment. ZeroCollators
        // But only if this test runs after:
        //   test mock::__construct_runtime_integrity_test::runtime_integrity_tests ... ok
        // Because that test enables logging
        silence_logs(|| {
            run_to_block(21);
        });

        // There are no collators but that would brick the chain, so we keep the old assignment
        assert_eq!(assigned_collators(), initial_assignment);
    });
}

#[test]
fn assign_collators_stay_constant_if_new_collators_can_take_new_chains() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.collators_per_parathread = 2;
            m.min_orchestrator_chain_collators = 2;
            m.max_orchestrator_chain_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
            m.container_chains = vec![];
        });
        assert_eq!(assigned_collators(), initial_collators(),);
        run_to_block(11);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![(1, 1000), (2, 1000), (3, 1000), (4, 1000), (5, 1000),]),
        );

        MockData::mutate(|m| {
            m.container_chains = vec![1001, 1002];
        });
        run_to_block(21);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![
                (1, 1000),
                (2, 1000),
                (3, 1000),
                (4, 1000),
                (5, 1000),
                (6, 1001),
                (7, 1001),
                (8, 1002),
                (9, 1002),
            ]),
        );
    });
}

#[test]
fn assign_collators_move_extra_container_chain_to_orchestrator_chain_if_not_enough_collators() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.collators_per_parathread = 2;
            m.min_orchestrator_chain_collators = 2;
            m.max_orchestrator_chain_collators = 5;

            m.collators = vec![1, 2, 3, 4];
            m.container_chains = vec![];
        });
        assert_eq!(assigned_collators(), initial_collators(),);
        run_to_block(11);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![(1, 1000), (2, 1000), (3, 1000), (4, 1000),]),
        );

        MockData::mutate(|m| {
            m.collators = vec![1, 2, 3, 4, 5];
            m.container_chains = vec![1001, 1002];
        });
        run_to_block(21);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![(1, 1000), (2, 1000), (3, 1000), (4, 1001), (5, 1001),]),
        );
    });
}

#[test]
fn assign_collators_reorganize_container_chains_if_not_enough_collators() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.collators_per_parathread = 2;
            m.min_orchestrator_chain_collators = 2;
            m.max_orchestrator_chain_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
            m.container_chains = vec![1001, 1002, 1003, 1004, 1005];
        });
        assert_eq!(assigned_collators(), initial_collators(),);
        run_to_block(11);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![
                (1, 1000),
                (2, 1000),
                (3, 1001),
                (4, 1001),
                (5, 1002),
                (6, 1002),
                (7, 1003),
                (8, 1003),
                (9, 1004),
                (10, 1004),
                (11, 1005),
                (12, 1005)
            ]),
        );

        MockData::mutate(|m| {
            // Remove collators to leave only 1 per container chain
            m.collators = vec![1, 2, 3, 5, 7, 9, 11];
        });
        run_to_block(21);

        // There are 7 collators in total: 2x2 container chains, plus 3 in the orchestrator chain
        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![
                (1, 1000),
                (2, 1000),
                (3, 1001),
                (5, 1002),
                (7, 1000),
                (9, 1001),
                (11, 1002)
            ]),
        );
    });
}

#[test]
fn assign_collators_set_zero_per_container() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.collators_per_parathread = 2;
            m.min_orchestrator_chain_collators = 2;
            m.max_orchestrator_chain_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
            m.container_chains = vec![1001, 1002, 1003, 1004];
        });
        assert_eq!(assigned_collators(), initial_collators(),);
        run_to_block(11);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![
                (1, 1000),
                (2, 1000),
                (3, 1000),
                (4, 1000),
                (5, 1001),
                (6, 1001),
                (7, 1002),
                (8, 1002),
                (9, 1003),
                (10, 1003),
                (11, 1004),
                (12, 1004),
            ]),
        );

        MockData::mutate(|m| {
            // We don't want to assign collators to container chains anymore
            m.collators_per_container = 0;
        });
        run_to_block(21);

        // There are 5 collators in total: 0x4 container chains, plus 5 in the orchestrator chain
        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![(1, 1000), (2, 1000), (3, 1000), (4, 1000), (5, 1000),]),
        );
    });
}

#[test]
fn assign_collators_rotation() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.collators_per_parathread = 2;
            m.min_orchestrator_chain_collators = 2;
            m.max_orchestrator_chain_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
            m.container_chains = vec![1001, 1002, 1003, 1004];
        });
        assert_eq!(assigned_collators(), initial_collators(),);
        run_to_block(11);

        let initial_assignment = BTreeMap::from_iter(vec![
            (1, 1000),
            (2, 1000),
            (3, 1000),
            (4, 1000),
            (5, 1001),
            (6, 1001),
            (7, 1002),
            (8, 1002),
            (9, 1003),
            (10, 1003),
            (11, 1004),
            (12, 1004),
        ]);

        assert_eq!(assigned_collators(), initial_assignment,);

        MockData::mutate(|m| {
            m.random_seed = [1; 32];
        });

        // The rotation period is every 5 sessions, so the first session with a different assignment
        // will be session 5. Collators are calculated one session in advance, so they will be decided
        // on session 4.
        run_to_block(20);

        assert_eq!(assigned_collators(), initial_assignment,);
        assert_eq!(PendingCollatorContainerChain::<Test>::get(), None,);

        run_to_block(21);
        assert_eq!(assigned_collators(), initial_assignment,);

        assert!(PendingCollatorContainerChain::<Test>::get().is_some(),);

        run_to_block(25);
        assert_eq!(assigned_collators(), initial_assignment,);
        run_to_block(26);

        // Random assignment depends on the seed, shouldn't change unless the algorithm changes
        let shuffled_assignment = BTreeMap::from_iter(vec![
            (1, 1003),
            (2, 1000),
            (3, 1001),
            (4, 1003),
            (5, 1000),
            (6, 1000),
            (7, 1001),
            (8, 1000),
            (9, 1004),
            (10, 1004),
            (11, 1002),
            (12, 1002),
        ]);

        assert_eq!(assigned_collators(), shuffled_assignment);
    });
}

#[test]
fn assign_collators_rotation_container_chains_are_shuffled() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.collators_per_parathread = 2;
            m.min_orchestrator_chain_collators = 2;
            m.max_orchestrator_chain_collators = 5;

            // 4 collators so we can only assign to one container chain
            m.collators = vec![1, 2, 3, 4];
            m.container_chains = vec![1001, 1002];
        });
        assert_eq!(assigned_collators(), initial_collators(),);
        run_to_block(11);

        let initial_assignment =
            BTreeMap::from_iter(vec![(1, 1000), (2, 1000), (3, 1001), (4, 1001)]);

        assert_eq!(assigned_collators(), initial_assignment,);

        MockData::mutate(|m| {
            // Seed chosen manually to see the case where container 1002 is given priority
            m.random_seed = [1; 32];
        });

        run_to_block(26);

        // Random assignment depends on the seed, shouldn't change unless the algorithm changes
        // Test that container chains are shuffled because 1001 does not have priority
        let shuffled_assignment =
            BTreeMap::from_iter(vec![(1, 1002), (2, 1000), (3, 1000), (4, 1002)]);

        assert_eq!(assigned_collators(), shuffled_assignment,);
    });
}

#[test]
fn assign_collators_rotation_parathreads_are_shuffled() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.collators_per_parathread = 2;
            m.min_orchestrator_chain_collators = 2;
            m.max_orchestrator_chain_collators = 5;

            // 4 collators so we can only assign to one parathread
            m.collators = vec![1, 2, 3, 4];
            m.parathreads = vec![3001, 3002];
        });
        assert_eq!(assigned_collators(), initial_collators(),);
        run_to_block(11);

        let initial_assignment =
            BTreeMap::from_iter(vec![(1, 1000), (2, 1000), (3, 3001), (4, 3001)]);

        assert_eq!(assigned_collators(), initial_assignment,);

        MockData::mutate(|m| {
            // Seed chosen manually to see the case where parathread 3002 is given priority
            m.random_seed = [1; 32];
        });

        run_to_block(26);

        // Random assignment depends on the seed, shouldn't change unless the algorithm changes
        // Test that container chains are shuffled because 1001 does not have priority
        let shuffled_assignment =
            BTreeMap::from_iter(vec![(1, 3002), (2, 1000), (3, 1000), (4, 3002)]);

        assert_eq!(assigned_collators(), shuffled_assignment,);
    });
}

#[test]
fn assign_collators_rotation_collators_are_shuffled() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.collators_per_parathread = 2;
            m.min_orchestrator_chain_collators = 2;
            m.max_orchestrator_chain_collators = 5;

            // 10 collators but we only need 9, so 1 collator will not be assigned
            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            m.container_chains = vec![1001, 1002];
        });
        assert_eq!(assigned_collators(), initial_collators(),);
        run_to_block(11);

        let initial_assignment = BTreeMap::from_iter(vec![
            (1, 1000),
            (2, 1000),
            (3, 1000),
            (4, 1000),
            (5, 1000),
            (6, 1001),
            (7, 1001),
            (8, 1002),
            (9, 1002),
        ]);

        assert_eq!(assigned_collators(), initial_assignment,);

        MockData::mutate(|m| {
            m.random_seed = [1; 32];
        });

        run_to_block(26);

        // Random assignment depends on the seed, shouldn't change unless the algorithm changes
        // Test that collators are shuffled and the collators of each container chain are not
        // consecutive in order. For example, if collators 8 and 9 are both assigned to chain 1002,
        // change the random seed until they are on different chains.
        // Collator 10 will never be assigned because of the collator priority.
        let shuffled_assignment = BTreeMap::from_iter(vec![
            (1, 1000),
            (2, 1001),
            (3, 1000),
            (4, 1000),
            (5, 1002),
            (6, 1001),
            (7, 1000),
            (8, 1002),
            (9, 1000),
        ]);

        assert_eq!(assigned_collators(), shuffled_assignment,);
    });
}

#[test]
fn assign_collators_invulnerables_priority_orchestrator() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.collators_per_parathread = 2;
            m.min_orchestrator_chain_collators = 2;
            m.max_orchestrator_chain_collators = 5;

            // 11 collators but we only need 9, so 2 collator will not be assigned
            // id 100 is an invulnerable so it will be assigned to the orchestrator
            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 100];
            m.container_chains = vec![1001, 1002];
        });
        assert_eq!(assigned_collators(), initial_collators(),);
        run_to_block(11);

        let initial_assignment = BTreeMap::from_iter(vec![
            (100, 1000),
            (1, 1000),
            (2, 1000),
            (3, 1000),
            (4, 1000),
            (5, 1001),
            (6, 1001),
            (7, 1002),
            (8, 1002),
        ]);

        assert_eq!(assigned_collators(), initial_assignment,);
    });
}

#[test]
fn assign_collators_invulnerables_priority_orchestrator_reassigned() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.collators_per_parathread = 2;
            m.min_orchestrator_chain_collators = 2;
            m.max_orchestrator_chain_collators = 5;
            // Disable rotation because this test is long
            m.full_rotation_period = Some(0);

            // 10 collators but we only need 9, so 1 collator will not be assigned
            // ids >= 100 are invulnerables so 2 of them will always be assigned to the orchestrator
            m.collators = vec![1, 2, 3, 4, 5, 100, 101, 102, 103, 104];
            m.container_chains = vec![1001, 1002];
        });
        assert_eq!(assigned_collators(), initial_collators(),);
        run_to_block(11);

        let initial_assignment = BTreeMap::from_iter(vec![
            (100, 1000),
            (101, 1000),
            (1, 1000),
            (2, 1000),
            (3, 1000),
            (4, 1001),
            (5, 1001),
            (102, 1002),
            (103, 1002),
        ]);

        assert_eq!(assigned_collators(), initial_assignment,);

        MockData::mutate(|m| {
            // Remove invulnerable from orchestrator, the unassigned invulnerable will take its place
            m.collators = vec![1, 2, 3, 4, 5, 101, 102, 103, 104];
        });

        run_to_block(21);

        let assignment = BTreeMap::from_iter(vec![
            (104, 1000),
            (101, 1000),
            (1, 1000),
            (2, 1000),
            (3, 1000),
            (4, 1001),
            (5, 1001),
            (102, 1002),
            (103, 1002),
        ]);

        assert_eq!(assigned_collators(), assignment,);

        MockData::mutate(|m| {
            // Remove another invulnerable from orchestrator, there are no unassigned invulnerables so the ones in a
            // container chain will move from the container chain to the orchestrator
            m.collators = vec![1, 2, 3, 4, 5, 102, 103, 104];
        });

        run_to_block(31);

        let assignment = BTreeMap::from_iter(vec![
            (104, 1000),
            (102, 1000),
            (1, 1000),
            (2, 1000),
            (3, 1002),
            (4, 1001),
            (5, 1001),
            (103, 1002),
        ]);

        assert_eq!(assigned_collators(), assignment,);
    });
}

#[test]
fn assign_collators_all_invulnerables() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.collators_per_parathread = 2;
            m.min_orchestrator_chain_collators = 2;
            m.max_orchestrator_chain_collators = 5;

            // All collators are invulnerables: this results in the same assignment as if there were not invulnerables
            m.collators = vec![101, 102, 103, 104, 105, 106, 107, 108, 109, 110];
            m.container_chains = vec![1001, 1002];
        });
        assert_eq!(assigned_collators(), initial_collators(),);
        run_to_block(11);

        let initial_assignment = BTreeMap::from_iter(vec![
            (101, 1000),
            (102, 1000),
            (103, 1000),
            (104, 1000),
            (105, 1000),
            (106, 1001),
            (107, 1001),
            (108, 1002),
            (109, 1002),
        ]);

        assert_eq!(assigned_collators(), initial_assignment,);
    });
}

#[test]
fn rotation_events() {
    // Ensure that the NewPendingAssignment event is correct
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.collators_per_parathread = 2;
            m.min_orchestrator_chain_collators = 2;
            m.max_orchestrator_chain_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
            m.container_chains = vec![1001, 1002, 1003, 1004];
        });
        assert_eq!(assigned_collators(), initial_collators(),);

        // Block 1 should emit event, random seed was not set
        System::assert_last_event(
            Event::NewPendingAssignment {
                random_seed: [0; 32],
                full_rotation: false,
                target_session: 1,
            }
            .into(),
        );

        for i in 2..=11 {
            run_to_block(i);
            match i {
                6 | 11 => {
                    System::assert_last_event(
                        Event::NewPendingAssignment {
                            random_seed: [0; 32],
                            full_rotation: false,
                            target_session: (i / 5) as u32 + 1,
                        }
                        .into(),
                    );
                }
                _ => {
                    assert_eq!(
                        System::events(),
                        vec![],
                        "Block #{} should not have any events",
                        i
                    );
                }
            }
        }

        MockData::mutate(|m| {
            m.random_seed = [1; 32];
        });

        // The rotation period is every 5 sessions, so the first session with a different assignment
        // will be session 5. Collators are calculated one session in advance, so they will be decided
        // on session 4, which starts on block 21.
        for i in 12..=51 {
            run_to_block(i);
            match i {
                16 | 26 | 31 | 36 | 41 | 51 => {
                    System::assert_last_event(
                        Event::NewPendingAssignment {
                            random_seed: [1; 32],
                            full_rotation: false,
                            target_session: (i / 5) as u32 + 1,
                        }
                        .into(),
                    );
                }
                21 | 46 => {
                    System::assert_last_event(
                        Event::NewPendingAssignment {
                            random_seed: [1; 32],
                            full_rotation: true,
                            target_session: (i / 5) as u32 + 1,
                        }
                        .into(),
                    );
                }
                _ => {
                    assert_eq!(
                        System::events(),
                        vec![],
                        "Block #{} should not have any events",
                        i
                    );
                }
            }
        }
    });
}

#[test]
fn assign_collators_remove_from_orchestator_when_all_assigned() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.collators_per_parathread = 2;
            m.min_orchestrator_chain_collators = 2;
            m.max_orchestrator_chain_collators = 2;

            m.collators = vec![1, 2];
            m.container_chains = vec![1001];
        });
        assert_eq!(assigned_collators(), initial_collators(),);
        run_to_block(11);

        let initial_assignment = BTreeMap::from_iter(vec![(1, 1000), (2, 1000)]);

        assert_eq!(assigned_collators(), initial_assignment,);

        MockData::mutate(|m| {
            m.collators = vec![1, 2, 3, 4];
        });

        run_to_block(26);

        let assignment = BTreeMap::from_iter(vec![(1, 1000), (2, 1000), (3, 1001), (4, 1001)]);
        assert_eq!(assigned_collators(), assignment,);

        MockData::mutate(|m| {
            m.collators = vec![1, 3, 4];
        });

        run_to_block(36);

        let assignment = BTreeMap::from_iter(vec![(1, 1000), (3, 1000)]);

        assert_eq!(assigned_collators(), assignment,);

        MockData::mutate(|m| {
            m.collators = vec![3, 4];
        });

        run_to_block(46);

        let assignment = BTreeMap::from_iter(vec![(3, 1000), (4, 1000)]);

        assert_eq!(assigned_collators(), assignment,);
    });
}

#[test]
fn collator_assignment_includes_empty_chains() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.collators_per_parathread = 2;
            m.min_orchestrator_chain_collators = 2;
            m.max_orchestrator_chain_collators = 2;

            m.collators = vec![1, 2];
            m.container_chains = vec![2000, 2001, 2002];
            m.parathreads = vec![3000, 3001, 3002]
        });
        assert_eq!(assigned_collators(), initial_collators(),);
        run_to_block(11);

        let assigned_collators = CollatorContainerChain::<Test>::get();
        let expected = AssignedCollators {
            orchestrator_chain: vec![1, 2],
            container_chains: BTreeMap::from_iter(vec![
                (2000.into(), vec![]),
                (2001.into(), vec![]),
                (2002.into(), vec![]),
                (3000.into(), vec![]),
                (3001.into(), vec![]),
                (3002.into(), vec![]),
            ]),
        };
        assert_eq!(assigned_collators, expected);
    });
}

#[test]
fn collator_assignment_remove_parachains_without_credits() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.collators_per_parathread = 2;
            m.min_orchestrator_chain_collators = 2;
            m.max_orchestrator_chain_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7];
            m.container_chains = vec![2000, 5001, 5002];
            m.parathreads = vec![]
        });
        assert_eq!(assigned_collators(), initial_collators(),);
        run_to_block(11);

        let assigned_collators = CollatorContainerChain::<Test>::get();
        let expected = AssignedCollators {
            orchestrator_chain: vec![1, 2, 3, 4, 5],
            container_chains: BTreeMap::from_iter(vec![(2000.into(), vec![6, 7])]),
        };
        assert_eq!(assigned_collators, expected);
    });
}

#[test]
fn collator_assignment_remove_parathreads_without_credits() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.collators_per_parathread = 2;
            m.min_orchestrator_chain_collators = 2;
            m.max_orchestrator_chain_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7];
            m.container_chains = vec![];
            m.parathreads = vec![3000, 5001, 5002]
        });
        assert_eq!(assigned_collators(), initial_collators(),);
        run_to_block(11);

        let assigned_collators = CollatorContainerChain::<Test>::get();
        let expected = AssignedCollators {
            orchestrator_chain: vec![1, 2, 3, 4, 5],
            container_chains: BTreeMap::from_iter(vec![(3000.into(), vec![6, 7])]),
        };
        assert_eq!(assigned_collators, expected);
    });
}

#[test]
fn assign_collators_prioritizing_tip() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.collators_per_parathread = 2;
            m.min_orchestrator_chain_collators = 5;
            m.max_orchestrator_chain_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
            m.container_chains = vec![1001, 1002, 1003, 1004];
            m.apply_tip = false
        });

        run_to_block(11);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![
                (1, 1000),
                (2, 1000),
                (3, 1000),
                (4, 1000),
                (5, 1000),
                (6, 1001),
                (7, 1001),
                (8, 1002),
                (9, 1002),
            ])
        );

        // Enable tip for 1003 and 1004
        MockData::mutate(|m| m.apply_tip = true);

        run_to_block(21);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![
                (1, 1000),
                (2, 1000),
                (3, 1000),
                (4, 1000),
                (5, 1000),
                (6, 1003),
                (7, 1003),
                (8, 1004),
                (9, 1004),
            ]),
        );
    });
}

#[test]
fn on_collators_assigned_hook_failure_removes_para_from_assignment() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.collators_per_parathread = 2;
            m.min_orchestrator_chain_collators = 5;
            m.max_orchestrator_chain_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
            m.container_chains = vec![1001, 1002, 1003, 1004];
            m.assignment_hook_errors = false;
        });
        run_to_block(11);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![
                (1, 1000),
                (2, 1000),
                (3, 1000),
                (4, 1000),
                (5, 1000),
                (6, 1001),
                (7, 1001),
                (8, 1002),
                (9, 1002),
                (10, 1003),
                (11, 1003),
            ]),
        );

        // Para 1001 will fail on_assignment_hook
        MockData::mutate(|m| {
            m.assignment_hook_errors = true;
        });

        run_to_block(21);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![
                (1, 1000),
                (2, 1000),
                (3, 1000),
                (4, 1000),
                (5, 1000),
                (8, 1002),
                (9, 1002),
                (10, 1003),
                (11, 1003),
            ]),
        );
    });
}

#[test]
fn assign_collators_truncates_before_shuffling() {
    // Check that if there are more collators than needed, we only assign the first collators
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            // Need 5 collators in total, 3 for orchestrator and 2 for 1 container chain
            m.collators_per_container = 2;
            m.min_orchestrator_chain_collators = 3;
            m.max_orchestrator_chain_collators = 3;

            // Have 10 collators in total, but only the first 5 will be assigned, in random order
            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            m.container_chains = vec![1001];
            m.random_seed = [1; 32];
        });

        run_to_block(11);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![(1, 1001), (2, 1000), (3, 1000), (4, 1001), (5, 1000),])
        );
    });
}
