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
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>.

use {
    crate::{mock::*, OrchestratorParaId, ParaId},
    sp_runtime::traits::BlakeTwo256,
    test_relay_sproof_builder::{
        AuthorityAssignmentSproofBuilder, HeaderAs, ParaHeaderSproofBuilderItem,
    },
};

#[test]
fn genesis_config_orchestrator_para_id() {
    new_test_ext().execute_with(|| {
        assert_eq!(OrchestratorParaId::<Test>::get(), 1000u32.into());
    });
}

#[test]
fn genesis_config_orchestrator_para_id_storage_update() {
    new_test_ext().execute_with(|| {
        let new_para_id = ParaId::new(2000);
        OrchestratorParaId::<Test>::put(&new_para_id);
        assert_eq!(OrchestratorParaId::<Test>::get(), new_para_id);
    });
}

#[test]
fn test_authorities_insertion_right_para_id() {
    let mut assignment = AuthorityAssignmentSproofBuilder::<u64>::default();
    assignment
        .authority_assignment
        .container_chains
        .insert(ParachainId::get(), vec![10u64, 11u64]);

    let (orchestrator_chain_root, orchestrator_chain_state) =
        assignment.into_state_root_and_proof();

    BlockTests::new()
        .with_relay_sproof_builder(move |_, relay_block_num, sproof| match relay_block_num {
            1 => {
                let mut s = ParaHeaderSproofBuilderItem::default();
                s.para_id = OrchestratorParachainId::get();
                s.author_id =
                    HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                        parent_hash: Default::default(),
                        number: Default::default(),
                        state_root: orchestrator_chain_root,
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
    let mut assignment = AuthorityAssignmentSproofBuilder::<u64>::default();
    assignment
        .authority_assignment
        .container_chains
        .insert(ParachainId::get() + 1, vec![10u64, 11u64]);

    let (orchestrator_chain_root, orchestrator_chain_state) =
        assignment.into_state_root_and_proof();

    BlockTests::new()
        .with_relay_sproof_builder(move |_, relay_block_num, sproof| match relay_block_num {
            1 => {
                let mut s = ParaHeaderSproofBuilderItem::default();
                s.para_id = OrchestratorParachainId::get();
                s.author_id =
                    HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                        parent_hash: Default::default(),
                        number: Default::default(),
                        state_root: orchestrator_chain_root,
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
    let mut assignment = AuthorityAssignmentSproofBuilder::<u64>::default();
    assignment
        .authority_assignment
        .container_chains
        .insert(ParachainId::get(), vec![10u64, 11u64]);

    let (orchestrator_chain_root, orchestrator_chain_state) =
        assignment.into_state_root_and_proof();

    BlockTests::new()
        .with_relay_sproof_builder(move |_, relay_block_num, sproof| match relay_block_num {
            1 => {
                let mut s = ParaHeaderSproofBuilderItem::default();
                s.para_id = OrchestratorParachainId::get();
                s.author_id =
                    HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                        parent_hash: Default::default(),
                        number: Default::default(),
                        state_root: orchestrator_chain_root,
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

#[test]
#[ignore = "used to generate benchmark data"]
fn encode_proof_for_benchmarks() {
    println!("pub const ENCODED_PROOFS: &[(u32, (&str, &[&str]), (&str, &[&str]))] = &[");

    for x in 0u32..=0 {
        let mut assignment = AuthorityAssignmentSproofBuilder::<u64>::default();
        let mut sproof_builder = test_relay_sproof_builder::ParaHeaderSproofBuilder::default();
        let container_chain_para_id = 200.into();
        let orchestrator_para_id = 1000.into();

        assignment
            .authority_assignment
            .container_chains
            .insert(container_chain_para_id, vec![10u64, 11u64]);

        assignment.session_index = 0; // TODO
        let (root_b, proof_b) = assignment.into_state_root_and_proof();

        let mut s = ParaHeaderSproofBuilderItem::default();
        s.para_id = orchestrator_para_id;
        // TODO: this header can be arbitrarily large, because "digest.logs" is an unbounded vec
        let header = HeaderAs::NonEncoded(tp_core::Header {
            parent_hash: Default::default(),
            number: Default::default(),
            state_root: root_b,
            extrinsics_root: Default::default(),
            digest: sp_runtime::generic::Digest { logs: vec![] },
        });
        s.author_id = header;
        sproof_builder.items.push(s);

        let (root_a, proof_a) = sproof_builder.into_state_root_and_proof();

        println!("({}, (\"{}\", &[", x, hex::encode(root_a),);

        for x in proof_a.iter_nodes() {
            println!("\"{}\",", hex::encode(x));
        }

        println!("]), (");

        println!("\"{}\", &[", hex::encode(root_b),);

        for x in proof_b.iter_nodes() {
            println!("\"{}\",", hex::encode(x));
        }

        println!("])),");
    }

    println!("];")
}
