use crate::assignment::Assignment;
use crate::assignment::ContainerChain;
use crate::tests::Test;

#[test]
fn select_chains_not_enough_to_reach_min_container() {
    // 10 collators when the orchestrator needs 2 and the containers need 10 result in no containers having collators
    let container_chains = vec![
        ContainerChain {
            para_id: 1000.into(),
            min_collators: 2,
            max_collators: 5,
        },
        ContainerChain {
            para_id: 2000.into(),
            min_collators: 10,
            max_collators: 10,
        },
        ContainerChain {
            para_id: 2001.into(),
            min_collators: 10,
            max_collators: 10,
        },
    ];
    let new_assigned = Assignment::<Test>::select_chains_with_collators(10, &container_chains);
    assert_eq!(new_assigned, vec![(1000.into(), 5),]);
}

#[test]
fn select_chains_not_enough_to_reach_min_orchestrator() {
    // 1 collator when the orchestrator needs 2 results in 1 collators being assigned to orchestrator
    let container_chains = vec![ContainerChain {
        para_id: 1000.into(),
        min_collators: 2,
        max_collators: 5,
    }];
    let new_assigned = Assignment::<Test>::select_chains_with_collators(1, &container_chains);
    assert_eq!(new_assigned, vec![(1000.into(), 1),]);
}

#[test]
fn select_chains_not_enough_for_all_min() {
    // Need 6 collators to support 3 chains, only have 5. The last chain will be removed and the remaining collator
    // will be assigned to orchestrator.
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
    let new_assigned = Assignment::<Test>::select_chains_with_collators(5, &container_chains);
    assert_eq!(new_assigned, vec![(1000.into(), 3), (2000.into(), 2),]);
}

#[test]
fn select_chains_not_enough_for_all_max() {
    // Need 6 collators to support 3 chains at min, but 15 collators to support them at max.
    // The last chain will be removed and the remaining collator
    let container_chains = vec![
        ContainerChain {
            para_id: 1000.into(),
            min_collators: 2,
            max_collators: 5,
        },
        ContainerChain {
            para_id: 2000.into(),
            min_collators: 2,
            max_collators: 5,
        },
        ContainerChain {
            para_id: 2001.into(),
            min_collators: 2,
            max_collators: 5,
        },
    ];
    let new_assigned = Assignment::<Test>::select_chains_with_collators(7, &container_chains);
    assert_eq!(
        new_assigned,
        vec![(1000.into(), 3), (2000.into(), 2), (2001.into(), 2),]
    );
    let new_assigned = Assignment::<Test>::select_chains_with_collators(10, &container_chains);
    assert_eq!(
        new_assigned,
        vec![(1000.into(), 5), (2000.into(), 3), (2001.into(), 2),]
    );
    let new_assigned = Assignment::<Test>::select_chains_with_collators(13, &container_chains);
    assert_eq!(
        new_assigned,
        vec![(1000.into(), 5), (2000.into(), 5), (2001.into(), 3),]
    );
    let new_assigned = Assignment::<Test>::select_chains_with_collators(15, &container_chains);
    assert_eq!(
        new_assigned,
        vec![(1000.into(), 5), (2000.into(), 5), (2001.into(), 5),]
    );
}

#[test]
fn select_chains_more_than_max() {
    // When the number of collators is greater than the sum of the max, all the chains are assigned max collators
    let container_chains = vec![
        ContainerChain {
            para_id: 1000.into(),
            min_collators: 2,
            max_collators: 5,
        },
        ContainerChain {
            para_id: 2000.into(),
            min_collators: 2,
            max_collators: 5,
        },
        ContainerChain {
            para_id: 2001.into(),
            min_collators: 2,
            max_collators: 5,
        },
    ];
    let new_assigned = Assignment::<Test>::select_chains_with_collators(20, &container_chains);
    assert_eq!(
        new_assigned,
        vec![(1000.into(), 5), (2000.into(), 5), (2001.into(), 5),]
    );
}

#[test]
fn select_chains_not_enough_to_reach_min_container_but_enough_for_parathread() {
    // Chain 2000 has more priority than parathread 3000, but we do not have enough min collators so the container
    // chain gets 0 collator and the parathread gets 1
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
            para_id: 3000.into(),
            min_collators: 1,
            max_collators: 1,
        },
    ];
    let new_assigned = Assignment::<Test>::select_chains_with_collators(3, &container_chains);
    assert_eq!(new_assigned, vec![(1000.into(), 2), (3000.into(), 1)]);
}
