// Copyright 2019-2022 Moondance Labs Ltd.
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
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>.

use {
    crate::mock::*,
    sp_runtime::traits::BlakeTwo256,
    test_relay_sproof_builder::{
        CollatorAssignmentSproofBuilder, HeaderAs, ParaHeaderSproofBuilderItem,
    },
};

#[test]
fn test_authorities_insertion_right_para_id() {
    let mut assignment = CollatorAssignmentSproofBuilder::<u64>::default();
    assignment
        .collator_assignment
        .container_chains
        .insert(ParachainId::get().into(), vec![10u64, 11u64]);

    let (orchestrator_chain_root, orchestrator_chain_state) =
        assignment.into_state_root_and_proof();

    BlockTests::new()
        .with_relay_sproof_builder(move |_, relay_block_num, sproof| match relay_block_num {
            1 => {
                let mut s = ParaHeaderSproofBuilderItem::default();
                s.para_id = OrchestratorParachainId::get().into();
                s.author_id =
                    HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                        parent_hash: Default::default(),
                        number: Default::default(),
                        state_root: orchestrator_chain_root.clone(),
                        extrinsics_root: Default::default(),
                        digest: sp_runtime::generic::Digest { logs: vec![] },
                    });
                sproof.items.push(s);
            }
            _ => unreachable!(),
        })
        .with_orchestrator_storage_proof(orchestrator_chain_state)
        .add(1, || {
            assert_eq!(AuthoritiesNoting::authorities(), vec![10u64, 11u64]);
        });
}

#[test]
fn test_authorities_insertion_wrong_para_id() {
    let mut assignment = CollatorAssignmentSproofBuilder::<u64>::default();
    assignment
        .collator_assignment
        .container_chains
        .insert((ParachainId::get() + 1).into(), vec![10u64, 11u64]);

    let (orchestrator_chain_root, orchestrator_chain_state) =
        assignment.into_state_root_and_proof();

    BlockTests::new()
        .with_relay_sproof_builder(move |_, relay_block_num, sproof| match relay_block_num {
            1 => {
                let mut s = ParaHeaderSproofBuilderItem::default();
                s.para_id = OrchestratorParachainId::get().into();
                s.author_id =
                    HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                        parent_hash: Default::default(),
                        number: Default::default(),
                        state_root: orchestrator_chain_root.clone(),
                        extrinsics_root: Default::default(),
                        digest: sp_runtime::generic::Digest { logs: vec![] },
                    });
                sproof.items.push(s);
            }
            _ => unreachable!(),
        })
        .with_orchestrator_storage_proof(orchestrator_chain_state)
        .add(1, || {
            assert!(AuthoritiesNoting::authorities().is_empty());
        });
}

#[test]
#[should_panic(
    expected = "Orchestrator chain authorities data needs to be present in every block!"
)]
fn test_not_inserting_inherent() {
    let mut assignment = CollatorAssignmentSproofBuilder::<u64>::default();
    assignment
        .collator_assignment
        .container_chains
        .insert(ParachainId::get().into(), vec![10u64, 11u64]);

    let (orchestrator_chain_root, orchestrator_chain_state) =
        assignment.into_state_root_and_proof();

    BlockTests::new()
        .with_relay_sproof_builder(move |_, relay_block_num, sproof| match relay_block_num {
            1 => {
                let mut s = ParaHeaderSproofBuilderItem::default();
                s.para_id = OrchestratorParachainId::get().into();
                s.author_id =
                    HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                        parent_hash: Default::default(),
                        number: Default::default(),
                        state_root: orchestrator_chain_root.clone(),
                        extrinsics_root: Default::default(),
                        digest: sp_runtime::generic::Digest { logs: vec![] },
                    });
                sproof.items.push(s);
            }
            _ => unreachable!(),
        })
        .with_orchestrator_storage_proof(orchestrator_chain_state)
        .skip_inherent_insertion()
        .add(1, || {
            assert!(AuthoritiesNoting::authorities().is_empty());
        });
}
