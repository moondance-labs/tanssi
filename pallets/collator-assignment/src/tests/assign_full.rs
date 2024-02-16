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
    crate::{
        assignment::{Assignment, AssignmentError},
        tests::Test,
    },
    sp_std::collections::btree_map::BTreeMap,
};

#[test]
fn assign_full_old_assigned_priority() {
    // Collators in old_assigned will be selected before other collators
    let collators = vec![1, 2, 3, 4, 5];
    let container_chains = vec![(1000.into(), 5)];
    let old_assigned = BTreeMap::from_iter(vec![(1000.into(), vec![3, 4])]);

    let new_assigned =
        Assignment::<Test>::assign_full(collators, container_chains, old_assigned).unwrap();
    let expected = BTreeMap::from_iter(vec![(1000.into(), vec![3, 4, 1, 2, 5])]);
    assert_eq!(new_assigned, expected);
}

#[test]
fn assign_full_invalid_old_assigned_collators_removed() {
    // If the collators in old_assigned are no longer collators, they are not assigned
    let collators = vec![1, 2, 3, 4, 5];
    let container_chains = vec![(1000.into(), 5)];
    let old_assigned = BTreeMap::from_iter(vec![(1000.into(), vec![20, 21])]);

    let new_assigned =
        Assignment::<Test>::assign_full(collators, container_chains, old_assigned).unwrap();
    let expected = BTreeMap::from_iter(vec![(1000.into(), vec![1, 2, 3, 4, 5])]);
    assert_eq!(new_assigned, expected);
}

#[test]
fn assign_full_invalid_chains_removed() {
    // Mark all collators as already assigned to a chain that does not exist. Should treat them as not assigned.
    let collators = vec![1, 2, 3, 4, 5];
    let container_chains = vec![(1000.into(), 5)];
    let old_assigned = BTreeMap::from_iter(vec![(1001.into(), vec![1, 2, 3, 4, 5])]);

    let new_assigned =
        Assignment::<Test>::assign_full(collators, container_chains, old_assigned).unwrap();
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

    let new_assigned =
        Assignment::<Test>::assign_full(collators, container_chains, old_assigned).unwrap();
    let expected = BTreeMap::from_iter(vec![(1000.into(), vec![1, 2]), (2000.into(), vec![6, 7])]);
    assert_eq!(new_assigned, expected);
}

#[test]
fn assign_full_old_assigned_error_if_not_enough_collators() {
    // Need 4 collators, only have 2, and all 2 were assigned to the second chain. If the function did not panic, we
    // would have 0 collators assigned to the first chain, which is supposed to have priority.
    let collators = vec![1, 2];
    let container_chains = vec![(1000.into(), 2), (2000.into(), 2)];
    let old_assigned = BTreeMap::from_iter(vec![(2000.into(), vec![1, 2])]);
    let new_assigned = Assignment::<Test>::assign_full(collators, container_chains, old_assigned);
    assert_eq!(
        new_assigned.unwrap_err(),
        AssignmentError::NotEnoughCollators
    );
}
