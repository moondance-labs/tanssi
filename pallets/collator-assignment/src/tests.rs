use std::collections::BTreeMap;

use crate::{mock::*, Store};

fn assigned_collators() -> BTreeMap<u64, u32> {
    let assigned_collators = <CollatorAssignment as Store>::CollatorContainerChain::get();

    let mut h = BTreeMap::new();

    for (para_id, collators) in assigned_collators.container_chains.iter() {
        for collator in collators.iter() {
            h.insert(*collator, *para_id);
        }
    }

    for collator in assigned_collators.orchestrator_chain {
        h.insert(collator, 999);
    }

    h
}

#[test]
fn assign_initial_collators() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.min_orchestrator_chain_collators = 5;
            m.max_orchestrator_chain_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            m.container_chains = vec![1001, 1002]
        });

        assert_eq!(assigned_collators(), BTreeMap::new(),);
        run_to_block(6);

        assert_eq!(assigned_collators(), BTreeMap::new(),);
        run_to_block(11);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![
                (1, 999),
                (2, 999),
                (3, 999),
                (4, 999),
                (5, 999),
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
            m.min_orchestrator_chain_collators = 5;
            m.max_orchestrator_chain_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            m.container_chains = vec![1001, 1002]
        });

        assert_eq!(assigned_collators(), BTreeMap::new(),);
        run_to_block(6);

        assert_eq!(assigned_collators(), BTreeMap::new(),);
        run_to_block(11);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![
                (1, 999),
                (2, 999),
                (3, 999),
                (4, 999),
                (5, 999),
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
                (1, 999),
                (2, 999),
                (3, 999),
                (4, 999),
                (5, 999),
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
            m.min_orchestrator_chain_collators = 5;
            m.max_orchestrator_chain_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            m.container_chains = vec![1001, 1002]
        });

        assert_eq!(assigned_collators(), BTreeMap::new(),);
        run_to_block(11);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![
                (1, 999),
                (2, 999),
                (3, 999),
                (4, 999),
                (5, 999),
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
                (1, 999),
                (2, 999),
                (3, 999),
                //(4, 999),
                (5, 999),
                (6, 1001),
                (7, 1001),
                (8, 1002),
                (9, 1002),
                // 10 is assigned in place of 4
                (10, 999),
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
            m.min_orchestrator_chain_collators = 5;
            m.max_orchestrator_chain_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
            m.container_chains = vec![1001, 1002]
        });
        assert_eq!(assigned_collators(), BTreeMap::new(),);
        run_to_block(11);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![
                (1, 999),
                (2, 999),
                (3, 999),
                (4, 999),
                (5, 999),
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
                (1, 999),
                (2, 999),
                (3, 999),
                (4, 999),
                (5, 999),
                (6, 1001),
                (7, 1001),
                (8, 1002),
                (9, 1002),
                (10, 999),
                (11, 999),
                (12, 999),
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
            m.min_orchestrator_chain_collators = 5;
            m.max_orchestrator_chain_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
            m.container_chains = vec![1001, 1002]
        });
        assert_eq!(assigned_collators(), BTreeMap::new(),);
        run_to_block(11);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![
                (1, 999),
                (2, 999),
                (3, 999),
                (4, 999),
                (5, 999),
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
            m.min_orchestrator_chain_collators = 5;
            m.max_orchestrator_chain_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
            m.container_chains = vec![1001, 1002]
        });

        assert_eq!(assigned_collators(), BTreeMap::new(),);
        run_to_block(11);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![
                (1, 999),
                (2, 999),
                (3, 999),
                (4, 999),
                (5, 999),
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
                (1, 999),
                (2, 999),
                (3, 999),
                (4, 999),
                (5, 999),
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
            m.min_orchestrator_chain_collators = 5;
            m.max_orchestrator_chain_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
            m.container_chains = vec![1001, 1002]
        });
        assert_eq!(assigned_collators(), BTreeMap::new(),);
        run_to_block(11);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![
                (1, 999),
                (2, 999),
                (3, 999),
                (4, 999),
                (5, 999),
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
                (1, 999),
                (2, 999),
                (3, 999),
                (4, 999),
                (5, 999),
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
            m.min_orchestrator_chain_collators = 5;
            m.max_orchestrator_chain_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
            m.container_chains = vec![1001, 1002]
        });
        assert_eq!(assigned_collators(), BTreeMap::new(),);
        run_to_block(11);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![
                (1, 999),
                (2, 999),
                (3, 999),
                (4, 999),
                (5, 999),
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
                (1, 999),
                (2, 999),
                (3, 999),
                (4, 999),
                (5, 999),
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
            m.min_orchestrator_chain_collators = 5;
            m.max_orchestrator_chain_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
            m.container_chains = vec![1001, 1002]
        });
        assert_eq!(assigned_collators(), BTreeMap::new(),);
        run_to_block(11);

        assert_eq!(
            assigned_collators(),
            BTreeMap::from_iter(vec![
                (1, 999),
                (2, 999),
                (3, 999),
                (4, 999),
                (5, 999),
                (6, 1001),
                (7, 1001),
                (8, 1002),
                (9, 1002),
            ]),
        );

        MockData::mutate(|m| {
            m.collators = vec![];
        });

        run_to_block(21);
        assert_eq!(assigned_collators(), BTreeMap::from_iter(vec![]));
    });
}
