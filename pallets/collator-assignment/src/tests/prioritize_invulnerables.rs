use crate::assignment::Assignment;
use crate::assignment::ContainerChain;
use crate::tests::Test;
use crate::Pallet as CollatorAssignment;
use sp_std::collections::btree_map::BTreeMap;

#[test]
fn invulnerable_priority_0_collators() {
    let collators = vec![];
    let container_chains = vec![];
    let mut old_assigned = BTreeMap::new();

    let num_invulnerables = Assignment::<Test>::prioritize_invulnerables(
        &collators,
        &container_chains,
        &mut old_assigned,
    );
}

#[test]
fn bug_same_invulnerable_selected_twice() {
    let collators = vec![100];
    let container_chains = vec![ContainerChain {
        para_id: 1000.into(),
        min_collators: 2,
        max_collators: 5,
    }];
    let mut old_assigned = BTreeMap::from_iter(vec![(1000.into(), vec![100])]);

    let num_invulnerables = Assignment::<Test>::prioritize_invulnerables(
        &collators,
        &container_chains,
        &mut old_assigned,
    );

    assert_eq!(num_invulnerables, 1);
}

#[test]
fn bug_no_priority() {
    // There are 3 invulnerables, 1 assigned to orchestrator and 2 assigned to a container chain.
    // After `prioritize_invulnerables` the first one from the container should move to orchestrator
    let collators = vec![1, 2, 3, 4, 5, 102, 103, 104];

    let container_chains = vec![
        ContainerChain {
            para_id: 1000.into(),
            min_collators: 2,
            max_collators: 5,
        },
        ContainerChain {
            para_id: 2000.into(),
            min_collators: 2,
            max_collators: 2,
        },
        ContainerChain {
            para_id: 2001.into(),
            min_collators: 2,
            max_collators: 2,
        },
    ];

    let mut old_assigned = BTreeMap::from_iter(vec![
        (1000.into(), vec![101, 104, 1, 2, 3]),
        (2000.into(), vec![4, 5]),
        (2001.into(), vec![102, 103]),
    ]);

    let num_invulnerables = Assignment::<Test>::prioritize_invulnerables(
        &collators,
        &container_chains,
        &mut old_assigned,
    );

    assert_eq!(num_invulnerables, 2);
}
