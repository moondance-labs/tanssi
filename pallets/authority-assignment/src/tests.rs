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
    crate::{mock::*, CollatorContainerChain},
    dp_collator_assignment::AssignedCollators,
    std::collections::BTreeMap,
};

fn assigned_collators_at_session(session_index: u32) -> Option<BTreeMap<String, u32>> {
    let assigned_collators = CollatorContainerChain::<Test>::get(session_index)?;

    let mut h = BTreeMap::new();

    for (para_id, collators) in assigned_collators.container_chains.iter() {
        for collator in collators.iter() {
            h.insert(collator.clone(), u32::from(*para_id));
        }
    }

    for collator in assigned_collators.orchestrator_chain {
        h.insert(collator, 999);
    }

    Some(h)
}

#[test]
fn assign_collators_genesis() {
    new_test_ext().execute_with(|| {
        MockData::mutate(|m| {
            m.next_collator_assignment = AssignedCollators {
                orchestrator_chain: vec![1, 2, 3, 4, 5],
                container_chains: BTreeMap::from_iter(vec![
                    (1001.into(), vec![6, 7]),
                    (1002.into(), vec![8, 9]),
                ]),
            };

            m.nimbus_map = BTreeMap::from_iter(
                vec![
                    (1, "nmbs1"),
                    (2, "nmbs2"),
                    (3, "nmbs3"),
                    (4, "nmbs4"),
                    (5, "nmbs5"),
                    (6, "nmbs6"),
                    (7, "nmbs7"),
                    (8, "nmbs8"),
                    (9, "nmbs9"),
                ]
                .into_iter()
                .map(|(id, nimbus_id)| (id, nimbus_id.to_string())),
            );
        });

        run_to_block(1);

        let expected_collators: Option<BTreeMap<String, u32>> = Some(BTreeMap::from_iter(
            vec![
                ("nmbs1", 999),
                ("nmbs2", 999),
                ("nmbs3", 999),
                ("nmbs4", 999),
                ("nmbs5", 999),
                ("nmbs6", 1001),
                ("nmbs7", 1001),
                ("nmbs8", 1002),
                ("nmbs9", 1002),
            ]
            .into_iter()
            .map(|(nimbus_id, para_id)| (nimbus_id.to_string(), para_id)),
        ));

        assert_eq!(assigned_collators_at_session(0), expected_collators);
        assert_eq!(assigned_collators_at_session(1), expected_collators);
        assert_eq!(assigned_collators_at_session(2), None);
    });
}

#[test]
fn assign_collators_session_one() {
    new_test_ext().execute_with(|| {
        MockData::mutate(|m| {
            m.next_collator_assignment = AssignedCollators {
                orchestrator_chain: vec![1, 2, 3, 4, 5],
                container_chains: BTreeMap::from_iter(vec![
                    (1001.into(), vec![6, 7]),
                    (1002.into(), vec![8, 9]),
                ]),
            };

            m.nimbus_map = BTreeMap::from_iter(
                vec![
                    (1, "nmbs1"),
                    (2, "nmbs2"),
                    (3, "nmbs3"),
                    (4, "nmbs4"),
                    (5, "nmbs5"),
                    (6, "nmbs6"),
                    (7, "nmbs7"),
                    (8, "nmbs8"),
                    (9, "nmbs9"),
                ]
                .into_iter()
                .map(|(id, nimbus_id)| (id, nimbus_id.to_string())),
            );
        });

        run_to_block(1);

        let expected_collators: Option<BTreeMap<String, u32>> = Some(BTreeMap::from_iter(
            vec![
                ("nmbs1", 999),
                ("nmbs2", 999),
                ("nmbs3", 999),
                ("nmbs4", 999),
                ("nmbs5", 999),
                ("nmbs6", 1001),
                ("nmbs7", 1001),
                ("nmbs8", 1002),
                ("nmbs9", 1002),
            ]
            .into_iter()
            .map(|(nimbus_id, para_id)| (nimbus_id.to_string(), para_id)),
        ));

        assert_eq!(assigned_collators_at_session(0), expected_collators);
        assert_eq!(assigned_collators_at_session(1), expected_collators);
        assert_eq!(assigned_collators_at_session(2), None);

        run_to_session(1);

        assert_eq!(assigned_collators_at_session(0), None);
        assert_eq!(assigned_collators_at_session(1), expected_collators);
        assert_eq!(assigned_collators_at_session(2), expected_collators);
        assert_eq!(assigned_collators_at_session(3), None);
    });
}

#[test]
fn assign_collators_change_nimbus_key() {
    new_test_ext().execute_with(|| {
        MockData::mutate(|m| {
            m.next_collator_assignment = AssignedCollators {
                orchestrator_chain: vec![1, 2, 3, 4, 5],
                container_chains: BTreeMap::from_iter(vec![
                    (1001.into(), vec![6, 7]),
                    (1002.into(), vec![8, 9]),
                ]),
            };

            m.nimbus_map = BTreeMap::from_iter(
                vec![
                    (1, "nmbs1"),
                    (2, "nmbs2"),
                    (3, "nmbs3"),
                    (4, "nmbs4"),
                    (5, "nmbs5"),
                    (6, "nmbs6"),
                    (7, "nmbs7"),
                    (8, "nmbs8"),
                    (9, "nmbs9"),
                ]
                .into_iter()
                .map(|(id, nimbus_id)| (id, nimbus_id.to_string())),
            );
        });

        run_to_block(1);

        let expected_collators: Option<BTreeMap<String, u32>> = Some(BTreeMap::from_iter(
            vec![
                ("nmbs1", 999),
                ("nmbs2", 999),
                ("nmbs3", 999),
                ("nmbs4", 999),
                ("nmbs5", 999),
                ("nmbs6", 1001),
                ("nmbs7", 1001),
                ("nmbs8", 1002),
                ("nmbs9", 1002),
            ]
            .into_iter()
            .map(|(nimbus_id, para_id)| (nimbus_id.to_string(), para_id)),
        ));

        assert_eq!(assigned_collators_at_session(0), expected_collators);
        assert_eq!(assigned_collators_at_session(1), expected_collators);
        assert_eq!(assigned_collators_at_session(2), None);

        MockData::mutate(|m| {
            // Change key for collator 1
            m.nimbus_map.insert(1, "nmbs1-changed".to_string());
        });

        run_to_session(1);

        let expected_collators_at_2: Option<BTreeMap<String, u32>> = Some(BTreeMap::from_iter(
            vec![
                ("nmbs1-changed", 999),
                ("nmbs2", 999),
                ("nmbs3", 999),
                ("nmbs4", 999),
                ("nmbs5", 999),
                ("nmbs6", 1001),
                ("nmbs7", 1001),
                ("nmbs8", 1002),
                ("nmbs9", 1002),
            ]
            .into_iter()
            .map(|(nimbus_id, para_id)| (nimbus_id.to_string(), para_id)),
        ));

        // Collators in session 2 use the new keys, but collators in session 1 use the old keys
        assert_eq!(assigned_collators_at_session(0), None);
        assert_eq!(assigned_collators_at_session(1), expected_collators);
        assert_eq!(assigned_collators_at_session(2), expected_collators_at_2);
        assert_eq!(assigned_collators_at_session(3), None);
    });
}

#[test]
fn assign_collators_remove_collator() {
    new_test_ext().execute_with(|| {
        MockData::mutate(|m| {
            m.next_collator_assignment = AssignedCollators {
                orchestrator_chain: vec![1, 2, 3, 4, 5],
                container_chains: BTreeMap::from_iter(vec![
                    (1001.into(), vec![6, 7]),
                    (1002.into(), vec![8, 9]),
                ]),
            };

            m.nimbus_map = BTreeMap::from_iter(
                vec![
                    (1, "nmbs1"),
                    (2, "nmbs2"),
                    (3, "nmbs3"),
                    (4, "nmbs4"),
                    (5, "nmbs5"),
                    (6, "nmbs6"),
                    (7, "nmbs7"),
                    (8, "nmbs8"),
                    (9, "nmbs9"),
                ]
                .into_iter()
                .map(|(id, nimbus_id)| (id, nimbus_id.to_string())),
            );
        });

        run_to_block(1);

        let expected_collators: Option<BTreeMap<String, u32>> = Some(BTreeMap::from_iter(
            vec![
                ("nmbs1", 999),
                ("nmbs2", 999),
                ("nmbs3", 999),
                ("nmbs4", 999),
                ("nmbs5", 999),
                ("nmbs6", 1001),
                ("nmbs7", 1001),
                ("nmbs8", 1002),
                ("nmbs9", 1002),
            ]
            .into_iter()
            .map(|(nimbus_id, para_id)| (nimbus_id.to_string(), para_id)),
        ));

        assert_eq!(assigned_collators_at_session(0), expected_collators);
        assert_eq!(assigned_collators_at_session(1), expected_collators);
        assert_eq!(assigned_collators_at_session(2), None);

        MockData::mutate(|m| {
            // Remove key for collator 9
            m.nimbus_map.remove(&9);
            // And remove collator 9 from assignment
            let collators_1002 = m
                .next_collator_assignment
                .container_chains
                .get_mut(&1002.into())
                .unwrap();
            assert_eq!(collators_1002.pop(), Some(9));
        });

        run_to_session(1);

        let expected_collators_at_2: Option<BTreeMap<String, u32>> = Some(BTreeMap::from_iter(
            vec![
                ("nmbs1", 999),
                ("nmbs2", 999),
                ("nmbs3", 999),
                ("nmbs4", 999),
                ("nmbs5", 999),
                ("nmbs6", 1001),
                ("nmbs7", 1001),
                ("nmbs8", 1002),
            ]
            .into_iter()
            .map(|(nimbus_id, para_id)| (nimbus_id.to_string(), para_id)),
        ));

        // Collators in session 2 use the new keys, but collators in session 1 use the old keys
        assert_eq!(assigned_collators_at_session(0), None);
        assert_eq!(assigned_collators_at_session(1), expected_collators);
        assert_eq!(assigned_collators_at_session(2), expected_collators_at_2);
        assert_eq!(assigned_collators_at_session(3), None);
    });
}

#[test]
fn assign_collators_insert_collator() {
    new_test_ext().execute_with(|| {
        MockData::mutate(|m| {
            m.next_collator_assignment = AssignedCollators {
                orchestrator_chain: vec![1, 2, 3, 4, 5],
                container_chains: BTreeMap::from_iter(vec![
                    (1001.into(), vec![6, 7]),
                    (1002.into(), vec![8, 9]),
                ]),
            };

            m.nimbus_map = BTreeMap::from_iter(
                vec![
                    (1, "nmbs1"),
                    (2, "nmbs2"),
                    (3, "nmbs3"),
                    (4, "nmbs4"),
                    (5, "nmbs5"),
                    (6, "nmbs6"),
                    (7, "nmbs7"),
                    (8, "nmbs8"),
                    (9, "nmbs9"),
                ]
                .into_iter()
                .map(|(id, nimbus_id)| (id, nimbus_id.to_string())),
            );
        });

        run_to_block(1);

        let expected_collators: Option<BTreeMap<String, u32>> = Some(BTreeMap::from_iter(
            vec![
                ("nmbs1", 999),
                ("nmbs2", 999),
                ("nmbs3", 999),
                ("nmbs4", 999),
                ("nmbs5", 999),
                ("nmbs6", 1001),
                ("nmbs7", 1001),
                ("nmbs8", 1002),
                ("nmbs9", 1002),
            ]
            .into_iter()
            .map(|(nimbus_id, para_id)| (nimbus_id.to_string(), para_id)),
        ));

        assert_eq!(assigned_collators_at_session(0), expected_collators);
        assert_eq!(assigned_collators_at_session(1), expected_collators);
        assert_eq!(assigned_collators_at_session(2), None);

        MockData::mutate(|m| {
            m.nimbus_map.insert(10, "nmbs10".to_string());
            m.next_collator_assignment.orchestrator_chain.push(10);
        });

        run_to_session(1);

        let expected_collators_at_2: Option<BTreeMap<String, u32>> = Some(BTreeMap::from_iter(
            vec![
                ("nmbs1", 999),
                ("nmbs2", 999),
                ("nmbs3", 999),
                ("nmbs4", 999),
                ("nmbs5", 999),
                ("nmbs6", 1001),
                ("nmbs7", 1001),
                ("nmbs8", 1002),
                ("nmbs9", 1002),
                ("nmbs10", 999),
            ]
            .into_iter()
            .map(|(nimbus_id, para_id)| (nimbus_id.to_string(), para_id)),
        ));

        // Collators in session 2 use the new keys, but collators in session 1 use the old keys
        assert_eq!(assigned_collators_at_session(0), None);
        assert_eq!(assigned_collators_at_session(1), expected_collators);
        assert_eq!(assigned_collators_at_session(2), expected_collators_at_2);
        assert_eq!(assigned_collators_at_session(3), None);
    });
}
