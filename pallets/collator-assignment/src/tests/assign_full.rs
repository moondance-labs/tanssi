use crate::assignment::Assignment;
use crate::tests::Test;
use crate::Pallet as CollatorAssignment;
use sp_std::collections::btree_map::BTreeMap;

#[test]
fn assign_full_0_collators() {
    let collators = vec![];
    let container_chains = vec![];
    let old_assigned = BTreeMap::new();

    let new_assigned = Assignment::<Test>::assign_full(collators, container_chains, old_assigned);
}
