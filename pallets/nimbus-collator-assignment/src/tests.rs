use {
    crate::{mock::*, CollatorContainerChain},
    std::collections::BTreeMap,
    tp_collator_assignment::AssignedCollators,
};

fn assigned_collators_at_session(session_index: u32) -> Option<BTreeMap<String, u32>> {
    let assigned_collators = CollatorContainerChain::<Test>::get(&session_index)?;

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

        assert_eq!(assigned_collators_at_session(0), expected_collators,);

        assert_eq!(assigned_collators_at_session(1), expected_collators,);

        assert_eq!(assigned_collators_at_session(2), None,);
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

        assert_eq!(assigned_collators_at_session(0), expected_collators,);

        assert_eq!(assigned_collators_at_session(1), expected_collators,);

        assert_eq!(assigned_collators_at_session(2), None,);

        run_to_session(1);

        assert_eq!(assigned_collators_at_session(0), None,);

        assert_eq!(assigned_collators_at_session(1), expected_collators,);

        assert_eq!(assigned_collators_at_session(2), expected_collators,);

        assert_eq!(assigned_collators_at_session(3), None,);
    });
}
