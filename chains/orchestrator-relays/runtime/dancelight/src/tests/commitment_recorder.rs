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

#![cfg(test)]

use {
    crate::{tests::common::*, OutboundMessageCommitmentRecorder, RuntimeEvent},
    snowbridge_merkle_tree::merkle_root,
    sp_core::H256,
    sp_runtime::traits::Keccak256,
};

#[test]
fn test_record_commitment_root_v1_with_leaves() {
    ExtBuilder::default().build().execute_with(|| {
        let leaves = vec![H256::from([1; 32]), H256::from([2; 32])];
        let commitment = H256::from([10; 32]);

        OutboundMessageCommitmentRecorder::record_commitment_root(commitment, leaves.clone());

        let stored = pallet_outbound_message_commitment_recorder::RecordedCommitment::<Runtime>::get();
        assert_eq!(stored, Some((commitment, leaves)));

        let events = frame_system::Pallet::<crate::Runtime>::events();
        let record_event = events
            .into_iter()
            .find(|e| matches!(e.event, RuntimeEvent::OutboundMessageCommitmentRecorder(pallet_outbound_message_commitment_recorder::Event::NewCommitmentRootRecorded { .. })))
            .expect("NewCommitmentRootRecorded event should be emitted");
        if let RuntimeEvent::OutboundMessageCommitmentRecorder(pallet_outbound_message_commitment_recorder::Event::NewCommitmentRootRecorded { commitment: event_commitment }) = record_event.event {
            assert_eq!(event_commitment, commitment);
        }
    });
}

#[test]
fn test_record_commitment_root_v2_with_leaves() {
    ExtBuilder::default().build().execute_with(|| {
        let leaves = vec![H256::from([3; 32]), H256::from([4; 32]), H256::from([5; 32])];
        let commitment = H256::from([20; 32]);

        OutboundMessageCommitmentRecorder::record_commitment_root_v2(commitment, leaves.clone());

        let stored = pallet_outbound_message_commitment_recorder::RecordedCommitmentV2::<Runtime>::get();
        assert_eq!(stored, Some((commitment, leaves)));

        let events = frame_system::Pallet::<crate::Runtime>::events();
        let record_event = events
            .into_iter()
            .find(|e| matches!(e.event, RuntimeEvent::OutboundMessageCommitmentRecorder(pallet_outbound_message_commitment_recorder::Event::NewCommitmentRootRecorded { .. })))
            .expect("NewCommitmentRootRecorded event should be emitted");
        if let RuntimeEvent::OutboundMessageCommitmentRecorder(pallet_outbound_message_commitment_recorder::Event::NewCommitmentRootRecorded { commitment: event_commitment }) = record_event.event {
            assert_eq!(event_commitment, commitment);
        }
    });
}

#[test]
fn test_record_empty_leaves() {
    ExtBuilder::default().build().execute_with(|| {
        let leaves = vec![];
        let commitment = H256::from([30; 32]);

        OutboundMessageCommitmentRecorder::record_commitment_root(commitment, leaves.clone());

        let stored =
            pallet_outbound_message_commitment_recorder::RecordedCommitment::<Runtime>::get();
        assert_eq!(stored, Some((commitment, leaves)));
    });
}

#[test]
fn test_take_commitment_root_v1_only() {
    ExtBuilder::default().build().execute_with(|| {
        let leaves = vec![H256::from([1; 32])];
        let commitment = H256::from([40; 32]);

        OutboundMessageCommitmentRecorder::record_commitment_root(commitment, leaves);

        let root = OutboundMessageCommitmentRecorder::take_commitment_root();
        assert_eq!(root, Some(commitment));

        assert_eq!(pallet_outbound_message_commitment_recorder::RecordedCommitment::<Runtime>::get(), None);

        let events = frame_system::Pallet::<crate::Runtime>::events();
        let read_event = events
            .into_iter()
            .find(|e| matches!(e.event, RuntimeEvent::OutboundMessageCommitmentRecorder(pallet_outbound_message_commitment_recorder::Event::CommitmentRootRead { .. })))
            .expect("CommitmentRootRead event should be emitted");
        if let RuntimeEvent::OutboundMessageCommitmentRecorder(pallet_outbound_message_commitment_recorder::Event::CommitmentRootRead { commitment: event_commitment }) = read_event.event {
            assert_eq!(event_commitment, commitment);
        }
    });
}

#[test]
fn test_take_commitment_root_v2_only() {
    ExtBuilder::default().build().execute_with(|| {
        let leaves = vec![H256::from([2; 32]), H256::from([3; 32])];
        let commitment = H256::from([50; 32]);

        OutboundMessageCommitmentRecorder::record_commitment_root_v2(commitment, leaves);

        let root = OutboundMessageCommitmentRecorder::take_commitment_root();
        assert_eq!(root, Some(commitment));

        assert_eq!(
            pallet_outbound_message_commitment_recorder::RecordedCommitmentV2::<Runtime>::get(),
            None
        );
    });
}

#[test]
fn test_take_commitment_root_both_v1_and_v2() {
    ExtBuilder::default().build().execute_with(|| {
        let leaves_v1 = vec![H256::from([1; 32]), H256::from([2; 32])];
        let commitment_v1 = H256::from([60; 32]);
        let leaves_v2 = vec![H256::from([3; 32])];
        let commitment_v2 = H256::from([70; 32]);

        OutboundMessageCommitmentRecorder::record_commitment_root(commitment_v1, leaves_v1);
        OutboundMessageCommitmentRecorder::record_commitment_root_v2(
            commitment_v2,
            leaves_v2.clone(),
        );

        let root = OutboundMessageCommitmentRecorder::take_commitment_root();

        let combined_leaves = vec![
            H256::from([1; 32]),
            H256::from([2; 32]),
            H256::from([3; 32]),
        ];
        let expected_root = merkle_root::<Keccak256, _>(combined_leaves.into_iter());
        assert_eq!(root, Some(expected_root));

        assert_eq!(
            pallet_outbound_message_commitment_recorder::RecordedCommitment::<Runtime>::get(),
            None
        );
        assert_eq!(
            pallet_outbound_message_commitment_recorder::RecordedCommitmentV2::<Runtime>::get(),
            None
        );
    });
}

#[test]
fn test_take_commitment_root_none() {
    ExtBuilder::default().build().execute_with(|| {
        let root = OutboundMessageCommitmentRecorder::take_commitment_root();
        assert_eq!(root, None);

        let events = frame_system::Pallet::<crate::Runtime>::events();
        let has_read_event = events.into_iter().any(|e| {
            matches!(
                e.event,
                RuntimeEvent::OutboundMessageCommitmentRecorder(
                    pallet_outbound_message_commitment_recorder::Event::CommitmentRootRead { .. }
                )
            )
        });
        assert!(!has_read_event);
    });
}

#[test]
fn test_prove_message_v1() {
    ExtBuilder::default().build().execute_with(|| {
        let leaves = vec![
            H256::from([1; 32]),
            H256::from([2; 32]),
            H256::from([3; 32]),
        ];
        let commitment = H256::from([100; 32]);
        let expected_root = merkle_root::<Keccak256, _>(leaves.clone().into_iter());

        OutboundMessageCommitmentRecorder::record_commitment_root(commitment, leaves.clone());

        let proof = OutboundMessageCommitmentRecorder::prove_message_v1(0);
        assert!(proof.is_some());
        let proof = proof.unwrap();
        assert_eq!(proof.root, expected_root);
        assert_eq!(proof.leaf, leaves[0]);

        let proof = OutboundMessageCommitmentRecorder::prove_message_v1(1);
        assert!(proof.is_some());
        let proof = proof.unwrap();
        assert_eq!(proof.root, expected_root);
        assert_eq!(proof.leaf, leaves[1]);
    });
}

#[test]
fn test_prove_message_v2_only_v2() {
    ExtBuilder::default().build().execute_with(|| {
        let leaves = vec![H256::from([4; 32]), H256::from([5; 32])];
        let commitment = H256::from([110; 32]);
        let expected_root = merkle_root::<Keccak256, _>(leaves.clone().into_iter());

        OutboundMessageCommitmentRecorder::record_commitment_root_v2(commitment, leaves.clone());

        let proof = OutboundMessageCommitmentRecorder::prove_message_v2(0);
        assert!(proof.is_some());
        let proof = proof.unwrap();
        assert_eq!(proof.root, expected_root);
        assert_eq!(proof.leaf, leaves[0]);
    });
}

#[test]
fn test_prove_message_v2_with_v1_and_v2() {
    ExtBuilder::default().build().execute_with(|| {
        let leaves_v1 = vec![H256::from([1; 32]), H256::from([2; 32])];
        let commitment_v1 = H256::from([120; 32]);
        let leaves_v2 = vec![H256::from([3; 32]), H256::from([4; 32])];
        let commitment_v2 = H256::from([130; 32]);

        OutboundMessageCommitmentRecorder::record_commitment_root(commitment_v1, leaves_v1.clone());
        OutboundMessageCommitmentRecorder::record_commitment_root_v2(
            commitment_v2,
            leaves_v2.clone(),
        );

        let combined_leaves = [leaves_v1, leaves_v2].concat();
        let combined_commitment = merkle_root::<Keccak256, _>(combined_leaves.clone().into_iter());

        let proof = OutboundMessageCommitmentRecorder::prove_message_v2(0);
        assert!(proof.is_some());
        let proof = proof.unwrap();
        assert_eq!(proof.root, combined_commitment);
        assert_eq!(proof.leaf, combined_leaves[2]);

        let proof = OutboundMessageCommitmentRecorder::prove_message_v2(1);
        assert!(proof.is_some());
        let proof = proof.unwrap();
        assert_eq!(proof.root, combined_commitment);
        assert_eq!(proof.leaf, combined_leaves[3]);
    });
}

#[test]
fn test_prove_message_no_commitments() {
    ExtBuilder::default().build().execute_with(|| {
        let proof = OutboundMessageCommitmentRecorder::prove_message_v1(0);
        assert!(proof.is_none());

        let proof = OutboundMessageCommitmentRecorder::prove_message_v2(0);
        assert!(proof.is_none());
    });
}

#[test]
fn test_single_leaf_proofs() {
    ExtBuilder::default().build().execute_with(|| {
        let leaves = vec![H256::from([1; 32])];
        let commitment = H256::from([140; 32]);
        let expected_root = merkle_root::<Keccak256, _>(leaves.clone().into_iter());

        OutboundMessageCommitmentRecorder::record_commitment_root(commitment, leaves.clone());

        let proof = OutboundMessageCommitmentRecorder::prove_message_v1(0);
        assert!(proof.is_some());
        let proof = proof.unwrap();
        assert_eq!(proof.root, expected_root);
        assert_eq!(proof.leaf, leaves[0]);
    });
}

#[test]
fn test_events_combined() {
    ExtBuilder::default().build().execute_with(|| {
        let leaves_v1 = vec![H256::from([1; 32])];
        let commitment_v1 = H256::from([150; 32]);
        let leaves_v2 = vec![H256::from([2; 32])];
        let commitment_v2 = H256::from([160; 32]);

        OutboundMessageCommitmentRecorder::record_commitment_root(commitment_v1, leaves_v1);
        OutboundMessageCommitmentRecorder::record_commitment_root_v2(commitment_v2, leaves_v2);

        let _ = OutboundMessageCommitmentRecorder::take_commitment_root();

        let events = frame_system::Pallet::<crate::Runtime>::events();
        let recorded_events: Vec<_> = events
            .into_iter()
            .filter_map(|e| {
                if let RuntimeEvent::OutboundMessageCommitmentRecorder(event) = e.event {
                    Some(event)
                } else {
                    None
                }
            })
            .collect();

        assert_eq!(recorded_events.len(), 3);
        assert!(matches!(recorded_events[0], pallet_outbound_message_commitment_recorder::Event::NewCommitmentRootRecorded { commitment } if commitment == commitment_v1));
        assert!(matches!(recorded_events[1], pallet_outbound_message_commitment_recorder::Event::NewCommitmentRootRecorded { commitment } if commitment == commitment_v2));
        assert!(matches!(recorded_events[2], pallet_outbound_message_commitment_recorder::Event::CommitmentRootRead { .. }));
    });
}
