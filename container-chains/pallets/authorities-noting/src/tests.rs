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
    crate::{mock::*, Authorities, Event, OrchestratorParaId, ParaId},
    frame_support::{
        assert_ok,
        dispatch::GetDispatchInfo,
        inherent::{InherentData, ProvideInherent},
        traits::UnfilteredDispatchable,
    },
    frame_system::RawOrigin,
    sp_runtime::traits::BlakeTwo256,
    test_relay_sproof_builder::{
        AuthorityAssignmentSproofBuilder, HeaderAs, ParaHeaderSproofBuilder,
        ParaHeaderSproofBuilderItem,
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

#[test]
fn test_set_authorities() {
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
            let authorities = vec![4, 5];
            assert_ok!(AuthoritiesNoting::set_authorities(
                RuntimeOrigin::root(),
                authorities.clone()
            ));
            assert_eq!(Authorities::<Test>::get(), authorities);
            System::assert_last_event(Event::AuthoritiesInserted { authorities }.into());
        });
}

#[test]
fn test_set_orchestrator_para_id() {
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
            let new_para_id = ParaId::new(2000);
            assert_ok!(AuthoritiesNoting::set_orchestrator_para_id(
                RuntimeOrigin::root(),
                new_para_id
            ));
            assert_eq!(OrchestratorParaId::<Test>::get(), new_para_id);
            System::assert_last_event(
                Event::OrchestratorParachainIdUpdated {
                    new_para_id: 2000.into(),
                }
                .into(),
            );
        });
}

#[test]
fn weights_assigned_to_extrinsics_are_correct() {
    new_test_ext().execute_with(|| {
        assert_eq!(
            crate::Call::<Test>::set_authorities {
                authorities: vec![]
            }
            .get_dispatch_info()
            .weight,
            <() as crate::weights::WeightInfo>::set_authorities(0u32)
        );

        assert_eq!(
            crate::Call::<Test>::set_orchestrator_para_id {
                new_para_id: 1u32.into()
            }
            .get_dispatch_info()
            .weight,
            <() as crate::weights::WeightInfo>::set_orchestrator_para_id()
        );

        let mut assignment = AuthorityAssignmentSproofBuilder::<u64>::default();
        assignment
            .authority_assignment
            .container_chains
            .insert(ParachainId::get(), vec![10u64, 11u64]);

        let (orchestrator_chain_root, orchestrator_chain_state) =
            assignment.into_state_root_and_proof();

        let mut sproof_builder = ParaHeaderSproofBuilder::default();

        let mut s = ParaHeaderSproofBuilderItem::default();
        s.para_id = OrchestratorParachainId::get();
        s.author_id = HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
            parent_hash: Default::default(),
            number: Default::default(),
            state_root: orchestrator_chain_root,
            extrinsics_root: Default::default(),
            digest: sp_runtime::generic::Digest { logs: vec![] },
        });
        sproof_builder.items.push(s);

        let (relay_root, relay_chain_state) = sproof_builder.into_state_root_and_proof();
        frame_support::storage::unhashed::put(MOCK_RELAY_ROOT_KEY, &relay_root);

        let mut inherent_data = InherentData::default();
        let system_inherent_data =
            ccp_authorities_noting_inherent::ContainerChainAuthoritiesInherentData {
                relay_chain_state,
                orchestrator_chain_state: orchestrator_chain_state.clone(),
            };
        inherent_data
            .put_data(
                ccp_authorities_noting_inherent::INHERENT_IDENTIFIER,
                &system_inherent_data,
            )
            .expect("failed to put VFP inherent");
        let inherent_weight = AuthoritiesNoting::create_inherent(&inherent_data)
            .expect("got an inherent")
            .dispatch_bypass_filter(RawOrigin::None.into())
            .expect("dispatch succeeded");

        assert_eq!(
            inherent_weight.actual_weight.unwrap(),
            <() as crate::weights::WeightInfo>::set_latest_authorities_data()
        );
    });
}
