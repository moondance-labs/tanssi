use std::collections::HashMap;

use crate::{mock::*, Store};

fn assigned_collators() -> HashMap<u64, u32> {
    <CollatorAssignment as Store>::CollatorParachain::iter().collect()
}

#[test]
fn assign_initial_collators() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.moondance_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            m.parachains = vec![1001, 1002]
        });

        assert_eq!(assigned_collators(), HashMap::new(),);
        run_to_block(6);

        assert_eq!(
            assigned_collators(),
            HashMap::from_iter(vec![
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
            m.moondance_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            m.parachains = vec![1001, 1002]
        });

        assert_eq!(assigned_collators(), HashMap::new(),);
        run_to_block(6);

        assert_eq!(
            assigned_collators(),
            HashMap::from_iter(vec![
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
        run_to_block(11);

        assert_eq!(
            assigned_collators(),
            HashMap::from_iter(vec![
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
fn assign_collators_after_one_leaves_moondance() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.moondance_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            m.parachains = vec![1001, 1002]
        });

        assert_eq!(assigned_collators(), HashMap::new(),);
        run_to_block(6);

        assert_eq!(
            assigned_collators(),
            HashMap::from_iter(vec![
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

        run_to_block(11);

        assert_eq!(
            assigned_collators(),
            HashMap::from_iter(vec![
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
fn assign_collators_if_config_moondance_collators_increases() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.moondance_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
            m.parachains = vec![1001, 1002]
        });
        assert_eq!(assigned_collators(), HashMap::new(),);
        run_to_block(6);

        assert_eq!(
            assigned_collators(),
            HashMap::from_iter(vec![
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
            // Add 3 new collators to moondance
            m.moondance_collators = 8;
        });

        run_to_block(11);

        assert_eq!(
            assigned_collators(),
            HashMap::from_iter(vec![
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
fn assign_collators_if_config_moondance_collators_decreases() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.moondance_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
            m.parachains = vec![1001, 1002]
        });
        assert_eq!(assigned_collators(), HashMap::new(),);
        run_to_block(6);

        assert_eq!(
            assigned_collators(),
            HashMap::from_iter(vec![
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
            // Remove 3 collators from moondance
            m.moondance_collators = 2;
        });

        run_to_block(11);

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
            m.moondance_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
            m.parachains = vec![1001, 1002]
        });

        assert_eq!(assigned_collators(), HashMap::new(),);
        run_to_block(6);

        assert_eq!(
            assigned_collators(),
            HashMap::from_iter(vec![
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
            // Add 2 new collators to each parachain
            m.collators_per_container = 4;
        });

        run_to_block(11);

        assert_eq!(
            assigned_collators(),
            HashMap::from_iter(vec![
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
fn assign_collators_if_parachain_is_removed() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.moondance_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
            m.parachains = vec![1001, 1002]
        });
        assert_eq!(assigned_collators(), HashMap::new(),);
        run_to_block(6);

        assert_eq!(
            assigned_collators(),
            HashMap::from_iter(vec![
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
            // Remove 1 parachain
            m.parachains = vec![1001 /*1002*/];
        });

        run_to_block(11);

        assert_eq!(
            assigned_collators(),
            HashMap::from_iter(vec![
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
fn assign_collators_if_parachain_is_added() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            m.collators_per_container = 2;
            m.moondance_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
            m.parachains = vec![1001, 1002]
        });
        assert_eq!(assigned_collators(), HashMap::new(),);
        run_to_block(6);

        assert_eq!(
            assigned_collators(),
            HashMap::from_iter(vec![
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
            // Add 1 new parachain
            m.parachains = vec![1001, 1002, 1003];
        });

        run_to_block(11);

        assert_eq!(
            assigned_collators(),
            HashMap::from_iter(vec![
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
            m.moondance_collators = 5;

            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
            m.parachains = vec![1001, 1002]
        });
        assert_eq!(assigned_collators(), HashMap::new(),);
        run_to_block(6);

        assert_eq!(
            assigned_collators(),
            HashMap::from_iter(vec![
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

        run_to_block(11);
        assert_eq!(assigned_collators(), HashMap::from_iter(vec![]));
    });
}
