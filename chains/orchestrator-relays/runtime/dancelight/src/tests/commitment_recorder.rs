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
    frame_support::assert_ok,
    snowbridge_merkle_tree::merkle_root,
    sp_core::H256,
    sp_runtime::traits::Keccak256,
};

#[test]
fn test_unified_commitment_both_v1_and_v2() {
    ExtBuilder::default().build().execute_with(|| {
        let v1_commit = H256::from([1; 32]);
        let v2_commit = H256::from([2; 32]);

        // Record v1 commitment
        OutboundMessageCommitmentRecorder::record_commitment_root(v1_commit);
        // Record v2 commitment
        OutboundMessageCommitmentRecorder::record_commitment_root_v2(v2_commit);

        // Take the root
        let root = OutboundMessageCommitmentRecorder::take_commitment_root();

        // Expected unified root
        let expected_root = merkle_root::<Keccak256, _>([v1_commit, v2_commit].into_iter());

        assert_eq!(root, Some(expected_root));

        // Check event
        let events = frame_system::Pallet::<crate::Runtime>::events();
        let commitment_read_event = events
            .into_iter()
            .find(|e| matches!(e.event, RuntimeEvent::OutboundMessageCommitmentRecorder(pallet_outbound_message_commitment_recorder::Event::CommitmentRootRead { .. })))
            .expect("CommitmentRootRead event should be emitted");
        if let RuntimeEvent::OutboundMessageCommitmentRecorder(pallet_outbound_message_commitment_recorder::Event::CommitmentRootRead { commitment }) = commitment_read_event.event {
            assert_eq!(commitment, expected_root);
        }
    });
}

#[test]
fn test_commitment_only_v1() {
    ExtBuilder::default().build().execute_with(|| {
        let v1_commit = H256::from([1; 32]);

        // Record v1 commitment
        OutboundMessageCommitmentRecorder::record_commitment_root(v1_commit);

        // Take the root
        let root = OutboundMessageCommitmentRecorder::take_commitment_root();

        assert_eq!(root, Some(v1_commit));

        // Check event
        let events = frame_system::Pallet::<crate::Runtime>::events();
        let commitment_read_event = events
            .into_iter()
            .find(|e| matches!(e.event, RuntimeEvent::OutboundMessageCommitmentRecorder(pallet_outbound_message_commitment_recorder::Event::CommitmentRootRead { .. })))
            .expect("CommitmentRootRead event should be emitted");
        if let RuntimeEvent::OutboundMessageCommitmentRecorder(pallet_outbound_message_commitment_recorder::Event::CommitmentRootRead { commitment }) = commitment_read_event.event {
            assert_eq!(commitment, v1_commit);
        }
    });
}

#[test]
fn test_commitment_only_v2() {
    ExtBuilder::default().build().execute_with(|| {
        let v2_commit = H256::from([2; 32]);

        // Record v2 commitment
        OutboundMessageCommitmentRecorder::record_commitment_root_v2(v2_commit);

        // Take the root
        let root = OutboundMessageCommitmentRecorder::take_commitment_root();

        assert_eq!(root, Some(v2_commit));

        // Check event
        let events = frame_system::Pallet::<crate::Runtime>::events();
        let commitment_read_event = events
            .into_iter()
            .find(|e| matches!(e.event, RuntimeEvent::OutboundMessageCommitmentRecorder(pallet_outbound_message_commitment_recorder::Event::CommitmentRootRead { .. })))
            .expect("CommitmentRootRead event should be emitted");
        if let RuntimeEvent::OutboundMessageCommitmentRecorder(pallet_outbound_message_commitment_recorder::Event::CommitmentRootRead { commitment }) = commitment_read_event.event {
            assert_eq!(commitment, v2_commit);
        }
    });
}

#[test]
fn test_commitment_none() {
    ExtBuilder::default().build().execute_with(|| {
        // Take the root without recording any
        let root = OutboundMessageCommitmentRecorder::take_commitment_root();

        assert_eq!(root, None);

        // No event should be emitted
        let events = frame_system::Pallet::<crate::Runtime>::events();
        let has_commitment_read = events.into_iter().any(|e| {
            matches!(
                e.event,
                RuntimeEvent::OutboundMessageCommitmentRecorder(
                    pallet_outbound_message_commitment_recorder::Event::CommitmentRootRead { .. }
                )
            )
        });
        assert!(!has_commitment_read);
    });
}

#[test]
fn test_events_recorded() {
    ExtBuilder::default().build().execute_with(|| {
        let v1_commit = H256::from([1; 32]);
        let v2_commit = H256::from([2; 32]);

        // Record v1
        OutboundMessageCommitmentRecorder::record_commitment_root(v1_commit);
        // Record v2
        OutboundMessageCommitmentRecorder::record_commitment_root_v2(v2_commit);

        // Take
        let _ = OutboundMessageCommitmentRecorder::take_commitment_root();

        // Check events
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

        assert_eq!(recorded_events.len(), 3); // Two recorded, one read
        assert!(matches!(recorded_events[0], pallet_outbound_message_commitment_recorder::Event::NewCommitmentRootRecorded { commitment } if commitment == v1_commit));
        assert!(matches!(recorded_events[1], pallet_outbound_message_commitment_recorder::Event::NewCommitmentRootRecorded { commitment } if commitment == v2_commit));
        assert!(matches!(recorded_events[2], pallet_outbound_message_commitment_recorder::Event::CommitmentRootRead { .. }));
    });
}
