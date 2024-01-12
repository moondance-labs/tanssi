use {
    crate::{assignment::Assignment, tests::Test},
    sp_std::collections::btree_map::BTreeMap,
};

#[test]
fn assign_full_old_assigned_priority() {
    let collators = vec![1, 2, 3, 4, 5];
    let container_chains = vec![(1000.into(), 5)];
    let old_assigned = BTreeMap::from_iter(vec![(1000.into(), vec![3, 4])]);

    let new_assigned = Assignment::<Test>::assign_full(collators, container_chains, old_assigned);
    let expected = BTreeMap::from_iter(vec![(1000.into(), vec![3, 4, 1, 2, 5])]);
    assert_eq!(new_assigned, expected);
}

#[test]
fn assign_full_invalid_old_assigned_collators_removed() {
    let collators = vec![1, 2, 3, 4, 5];
    let container_chains = vec![(1000.into(), 5)];
    let old_assigned = BTreeMap::from_iter(vec![(1000.into(), vec![20, 21])]);

    let new_assigned = Assignment::<Test>::assign_full(collators, container_chains, old_assigned);
    let expected = BTreeMap::from_iter(vec![(1000.into(), vec![1, 2, 3, 4, 5])]);
    assert_eq!(new_assigned, expected);
}

#[test]
fn assign_full_invalid_chains_removed() {
    // Mark all collators as already assigned to a chain that does not exist. Should treat them as not assigned.
    let collators = vec![1, 2, 3, 4, 5];
    let container_chains = vec![(1000.into(), 5)];
    let old_assigned = BTreeMap::from_iter(vec![(1001.into(), vec![1, 2, 3, 4, 5])]);

    let new_assigned = Assignment::<Test>::assign_full(collators, container_chains, old_assigned);
    let expected = BTreeMap::from_iter(vec![(1000.into(), vec![1, 2, 3, 4, 5])]);
    assert_eq!(new_assigned, expected);
}

#[test]
fn assign_full_truncates_collators() {
    // Need 2 collators for each chain, when old_assigned has more than 2. Should truncate old_assigned to 2.
    let collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let container_chains = vec![(1000.into(), 2), (2000.into(), 2)];
    let old_assigned = BTreeMap::from_iter(vec![
        (1000.into(), vec![1, 2, 3, 4, 5]),
        (2000.into(), vec![6, 7, 8, 9, 10]),
    ]);

    let new_assigned = Assignment::<Test>::assign_full(collators, container_chains, old_assigned);
    let expected = BTreeMap::from_iter(vec![(1000.into(), vec![1, 2]), (2000.into(), vec![6, 7])]);
    assert_eq!(new_assigned, expected);
}
