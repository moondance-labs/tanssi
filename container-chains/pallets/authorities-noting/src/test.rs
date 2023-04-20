use crate::mock::*;
use parity_scale_codec::Encode;
use sp_consensus_aura::AURA_ENGINE_ID;
use sp_runtime::generic::DigestItem;
use sp_runtime::traits::BlakeTwo256;
use test_relay_sproof_builder::{
    CollatorAssignmentSproofBuilder, HeaderAs, ParaHeaderSproofBuilderItem,
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
                let slot: InherentType = 13u64.into();
                let mut s = ParaHeaderSproofBuilderItem::default();
                s.para_id = OrchestratorParachainId::get().into();
                s.author_id =
                    HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                        parent_hash: Default::default(),
                        number: Default::default(),
                        state_root: orchestrator_chain_root.clone(),
                        extrinsics_root: Default::default(),
                        digest: sp_runtime::generic::Digest {
                            logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                        },
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
                let slot: InherentType = 13u64.into();
                let mut s = ParaHeaderSproofBuilderItem::default();
                s.para_id = OrchestratorParachainId::get().into();
                s.author_id =
                    HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                        parent_hash: Default::default(),
                        number: Default::default(),
                        state_root: orchestrator_chain_root.clone(),
                        extrinsics_root: Default::default(),
                        digest: sp_runtime::generic::Digest {
                            logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                        },
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
