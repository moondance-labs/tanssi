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
        assignment::{Assignment, ChainNumCollators},
        tests::Test,
    },
    sp_std::collections::{btree_map::BTreeMap, btree_set::BTreeSet},
};

#[test]
fn invulnerable_priority_0_collators() {
    let collators = vec![];
    let orchestrator_chain = ChainNumCollators {
        para_id: 1000.into(),
        min_collators: 2,
        max_collators: 5,
    };
    let mut old_assigned = BTreeMap::new();

    let num_invulnerables = Assignment::<Test>::prioritize_invulnerables(
        &collators,
        orchestrator_chain,
        &mut old_assigned,
    );

    assert_eq!(num_invulnerables, 0);
}

#[test]
fn invulnerable_priority_0_invulnerables() {
    let collators = vec![1, 2, 3, 4, 5];
    let orchestrator_chain = ChainNumCollators {
        para_id: 1000.into(),
        min_collators: 2,
        max_collators: 5,
    };
    let mut old_assigned = BTreeMap::from_iter(vec![(1000.into(), vec![1, 2])]);

    let num_invulnerables = Assignment::<Test>::prioritize_invulnerables(
        &collators,
        orchestrator_chain,
        &mut old_assigned,
    );

    assert_eq!(num_invulnerables, 0);
}

#[test]
fn invulnerable_priority_1_invulnerable_orchestrator() {
    let collators = vec![1, 2, 3, 4, 5, 101];
    let orchestrator_chain = ChainNumCollators {
        para_id: 1000.into(),
        min_collators: 2,
        max_collators: 5,
    };
    let mut old_assigned = BTreeMap::from_iter(vec![(1000.into(), vec![101])]);

    let num_invulnerables = Assignment::<Test>::prioritize_invulnerables(
        &collators,
        orchestrator_chain,
        &mut old_assigned,
    );

    assert_eq!(num_invulnerables, 1);
}

#[test]
fn invulnerable_priority_1_invulnerable_not_assigned() {
    let collators = vec![1, 2, 3, 4, 5, 101];
    let orchestrator_chain = ChainNumCollators {
        para_id: 1000.into(),
        min_collators: 2,
        max_collators: 5,
    };
    let mut old_assigned = BTreeMap::new();

    let num_invulnerables = Assignment::<Test>::prioritize_invulnerables(
        &collators,
        orchestrator_chain,
        &mut old_assigned,
    );

    assert_eq!(num_invulnerables, 1);
}

#[test]
fn invulnerable_priority_1_invulnerable_assigned_to_another_chain() {
    let collators = vec![1, 2, 3, 4, 5, 101];
    let orchestrator_chain = ChainNumCollators {
        para_id: 1000.into(),
        min_collators: 2,
        max_collators: 5,
    };
    let mut old_assigned =
        BTreeMap::from_iter(vec![(1000.into(), vec![]), (2000.into(), vec![101])]);

    let num_invulnerables = Assignment::<Test>::prioritize_invulnerables(
        &collators,
        orchestrator_chain,
        &mut old_assigned,
    );

    assert_eq!(num_invulnerables, 1);
}

#[test]
fn bug_same_invulnerable_selected_twice() {
    let collators = vec![100];
    let orchestrator_chain = ChainNumCollators {
        para_id: 1000.into(),
        min_collators: 2,
        max_collators: 5,
    };
    let mut old_assigned = BTreeMap::from_iter(vec![(1000.into(), vec![100])]);

    let num_invulnerables = Assignment::<Test>::prioritize_invulnerables(
        &collators,
        orchestrator_chain,
        &mut old_assigned,
    );

    assert_eq!(num_invulnerables, 1);
}

#[test]
fn bug_not_using_assigned_invulnerables() {
    // There are 3 invulnerables, 1 assigned to orchestrator and 2 assigned to a container chain.
    // After `prioritize_invulnerables` the first one from the container should move to orchestrator
    let collators = vec![1, 2, 3, 4, 5, 102, 103, 104];

    let container_chains = [
        ChainNumCollators {
            para_id: 1000.into(),
            min_collators: 2,
            max_collators: 5,
        },
        ChainNumCollators {
            para_id: 2000.into(),
            min_collators: 2,
            max_collators: 2,
        },
        ChainNumCollators {
            para_id: 2001.into(),
            min_collators: 2,
            max_collators: 2,
        },
    ];
    let orchestrator_chain = container_chains[0];

    let mut old_assigned = BTreeMap::from_iter(vec![
        (1000.into(), vec![101, 104, 1, 2, 3]),
        (2000.into(), vec![4, 5]),
        (2001.into(), vec![102, 103]),
    ]);

    let chains_with_collators_set =
        BTreeSet::from_iter(container_chains.iter().map(|cc| cc.para_id));
    let collators_set = BTreeSet::from_iter(collators.iter().cloned());
    Assignment::<Test>::retain_valid_old_assigned(
        &mut old_assigned,
        &chains_with_collators_set,
        &collators_set,
    );
    let num_invulnerables = Assignment::<Test>::prioritize_invulnerables(
        &collators,
        orchestrator_chain,
        &mut old_assigned,
    );

    assert_eq!(num_invulnerables, 2);
}
