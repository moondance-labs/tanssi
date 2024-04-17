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
    crate::{mock::*, ContainerChainBlockInfo, Event},
    bounded_collections::bounded_vec,
    cumulus_primitives_core::ParaId,
    frame_support::{
        assert_ok,
        dispatch::GetDispatchInfo,
        inherent::{InherentData, ProvideInherent},
        traits::UnfilteredDispatchable,
    },
    frame_system::RawOrigin,
    hex_literal::hex,
    parity_scale_codec::Encode,
    sp_consensus_aura::{inherents::InherentType, AURA_ENGINE_ID},
    sp_core::H256,
    sp_runtime::{
        generic::DigestItem,
        traits::{BlakeTwo256, HashingFor},
    },
    test_relay_sproof_builder::{HeaderAs, ParaHeaderSproofBuilder, ParaHeaderSproofBuilderItem},
    tp_traits::GetCurrentContainerChains,
};

#[test]
fn test_author_id_insertion() {
    BlockTests::new()
        .with_relay_sproof_builder(|_, relay_block_num, sproof| match relay_block_num {
            1 => {
                let slot: InherentType = 13u64.into();
                let s = ParaHeaderSproofBuilderItem {
                    para_id: 1001.into(),
                    author_id: HeaderAs::NonEncoded(sp_runtime::generic::Header::<
                        u32,
                        BlakeTwo256,
                    > {
                        parent_hash: Default::default(),
                        number: 1,
                        state_root: Default::default(),
                        extrinsics_root: Default::default(),
                        digest: sp_runtime::generic::Digest {
                            logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                        },
                    }),
                };
                sproof.items.push(s);
            }
            _ => unreachable!(),
        })
        .add(1, || {
            assert_eq!(
                AuthorNoting::latest_author(ParaId::from(1001)),
                Some(ContainerChainBlockInfo {
                    block_number: 1,
                    author: 13u64,
                    latest_slot_number: 1u64.into(),
                })
            );
        });
}

#[test]
fn test_author_id_insertion_real_data() {
    BlockTests::new()
        .with_relay_sproof_builder(|_, relay_block_num, sproof| {
            // Statemint data:
            // Block: 3,511,063
            // Slot: 140,006,956
            // RelayHash 0x5ea27df08fe09a82b5e835d4fa67735d0fbdf8d97b9c382f0af7b9c9c92a8545
            let statemint_data = hex!(
                "5d1b54ce2845dedd7f43805849747c44388b7b7cc84dc5083815cc2b58b513145e4cd6000a98bf
                 27921e16366f5a2a388595f87744608684f43ff613026241634390d0c28a9dee52544070b989c71634
                 db54222b86391a75fa37d12544e7022bcd3cd42a080661757261202c56580800000000056175726101
                 018fb36de33276e8d54f77ea0a006ed7ab97b8d0aad00869f7ce6a5709eb1fc3256428b8b2428a2a3e
                 c4fa1c1058ab0e33c5a6b2b5789ab7b3e0accaeccafb4506"
            );

            match relay_block_num {
                1 => {
                    let s = ParaHeaderSproofBuilderItem {
                        para_id: 1001.into(),
                        author_id: HeaderAs::AlreadyEncoded(statemint_data.to_vec()),
                    };
                    sproof.items.push(s);
                }
                _ => unreachable!(),
            }
        })
        .add(1, || {
            assert_eq!(
                AuthorNoting::latest_author(ParaId::from(1001)),
                // Our mock author fetcher will just note the slot
                Some(ContainerChainBlockInfo {
                    block_number: 3511063,
                    author: 140006956,
                    latest_slot_number: 1u64.into()
                })
            );
        });
}

#[test]
fn test_author_id_insertion_many_paras() {
    BlockTests::new()
        .with_relay_sproof_builder(|_, relay_block_num, sproof| match relay_block_num {
            1 => {
                // Since the default parachain list is vec![1001],
                // we must always include a sproof for this para_id
                let slot: InherentType = 10u64.into();
                let s = ParaHeaderSproofBuilderItem {
                    para_id: 1001.into(),
                    author_id: HeaderAs::NonEncoded(sp_runtime::generic::Header::<
                        u32,
                        BlakeTwo256,
                    > {
                        parent_hash: Default::default(),
                        number: 1,
                        state_root: Default::default(),
                        extrinsics_root: Default::default(),
                        digest: sp_runtime::generic::Digest {
                            logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                        },
                    }),
                };
                sproof.items.push(s);
            }
            2 => {
                let slot: InherentType = 13u64.into();
                let s = ParaHeaderSproofBuilderItem {
                    para_id: 1001.into(),
                    author_id: HeaderAs::NonEncoded(sp_runtime::generic::Header::<
                        u32,
                        BlakeTwo256,
                    > {
                        parent_hash: Default::default(),
                        number: 2,
                        state_root: Default::default(),
                        extrinsics_root: Default::default(),
                        digest: sp_runtime::generic::Digest {
                            logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                        },
                    }),
                };
                sproof.items.push(s);

                let slot: InherentType = 14u64.into();
                let s = ParaHeaderSproofBuilderItem {
                    para_id: 1002.into(),
                    author_id: HeaderAs::NonEncoded(sp_runtime::generic::Header::<
                        u32,
                        BlakeTwo256,
                    > {
                        parent_hash: Default::default(),
                        number: 1,
                        state_root: Default::default(),
                        extrinsics_root: Default::default(),
                        digest: sp_runtime::generic::Digest {
                            logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                        },
                    }),
                };
                sproof.items.push(s);
            }
            _ => unreachable!(),
        })
        .add(1, || {
            // Writing to this pallet storage will only change the sproofs of the next block,
            // not the ones of the current block
            MockData::mutate(|m| {
                m.container_chains = bounded_vec![1001.into(), 1002.into()];
            });
            assert_eq!(
                AuthorNoting::latest_author(ParaId::from(1001)),
                Some(ContainerChainBlockInfo {
                    block_number: 1,
                    author: 10u64,
                    latest_slot_number: 1u64.into()
                })
            );
            assert_eq!(AuthorNoting::latest_author(ParaId::from(1002)), None);
        })
        .add(2, || {
            assert_eq!(
                AuthorNoting::latest_author(ParaId::from(1001)),
                Some(ContainerChainBlockInfo {
                    block_number: 2,
                    author: 13u64,
                    latest_slot_number: 2u64.into()
                })
            );
            assert_eq!(
                AuthorNoting::latest_author(ParaId::from(1002)),
                Some(ContainerChainBlockInfo {
                    block_number: 1,
                    author: 14u64,
                    latest_slot_number: 2u64.into()
                })
            );
        });
}

#[test]
#[should_panic(expected = "Invalid relay chain state proof")]
fn test_should_panic_with_invalid_proof_root() {
    BlockTests::new()
        .with_relay_sproof_builder(|_, relay_block_num, sproof| match relay_block_num {
            1 => {
                let slot: InherentType = 13u64.into();
                let s = ParaHeaderSproofBuilderItem {
                    para_id: 1001.into(),
                    author_id: HeaderAs::NonEncoded(sp_runtime::generic::Header::<
                        u32,
                        BlakeTwo256,
                    > {
                        parent_hash: Default::default(),
                        number: 1,
                        state_root: Default::default(),
                        extrinsics_root: Default::default(),
                        digest: sp_runtime::generic::Digest {
                            logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                        },
                    }),
                };
                sproof.items.push(s);
            }
            _ => unreachable!(),
        })
        // Insert an invalid root, not matching the proof generated
        .with_overriden_state_root(H256::default())
        .add(1, || {
            assert_eq!(
                AuthorNoting::latest_author(ParaId::from(1001)),
                Some(ContainerChainBlockInfo {
                    block_number: 1,
                    author: 13u64,
                    latest_slot_number: 0u64.into()
                })
            );
        });
}

#[test]
#[should_panic(expected = "Invalid proof provided for para head key")]
fn test_should_panic_with_invalid_proof_state() {
    let sproof_builder = ParaHeaderSproofBuilder::default();
    let (_, relay_chain_state) = sproof_builder.into_state_root_and_proof();

    BlockTests::new()
        .with_relay_sproof_builder(|_, relay_block_num, sproof| match relay_block_num {
            1 => {
                let slot: InherentType = 13u64.into();
                let s = ParaHeaderSproofBuilderItem {
                    para_id: 1001.into(),
                    author_id: HeaderAs::NonEncoded(sp_runtime::generic::Header::<
                        u32,
                        BlakeTwo256,
                    > {
                        parent_hash: Default::default(),
                        number: 1,
                        state_root: Default::default(),
                        extrinsics_root: Default::default(),
                        digest: sp_runtime::generic::Digest {
                            logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                        },
                    }),
                };
                sproof.items.push(s);
            }
            _ => unreachable!(),
        })
        // Insert a proof, not matching the root generated
        .with_overriden_state_proof(relay_chain_state)
        .add(1, || {
            assert_eq!(
                AuthorNoting::latest_author(ParaId::from(1001)),
                Some(ContainerChainBlockInfo {
                    block_number: 1,
                    author: 13u64,
                    latest_slot_number: 0u64.into()
                })
            );
        });
}

#[test]
#[should_panic(expected = "Invalid proof provided for para head key")]
fn test_should_panic_with_proof_for_not_including_required_para() {
    // Since the default parachain list is vec![1001],
    // we must always include a sproof for this para_id
    let slot: InherentType = 10u64.into();
    let para_id_1001_item = ParaHeaderSproofBuilderItem {
        para_id: 1001.into(),
        author_id: HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
            parent_hash: Default::default(),
            number: Default::default(),
            state_root: Default::default(),
            extrinsics_root: Default::default(),
            digest: sp_runtime::generic::Digest {
                logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
            },
        }),
    };
    let mut proof_item = ParaHeaderSproofBuilder::default();
    proof_item.items.push(para_id_1001_item.clone());

    // However we insert a new para in the state. The idea is that the proof we
    // will pass is for this new paraId, and not 1001. Passing 1001 is required so
    // we should see the node panicking.

    let slot: InherentType = 14u64.into();
    let para_id_1002_item = ParaHeaderSproofBuilderItem {
        para_id: 1002.into(),
        author_id: HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
            parent_hash: Default::default(),
            number: Default::default(),
            state_root: Default::default(),
            extrinsics_root: Default::default(),
            digest: sp_runtime::generic::Digest {
                logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
            },
        }),
    };
    proof_item.items.push(para_id_1002_item.clone());

    // lets get the generated proof here. However we will modify later on the proof we pass to include para id 1002
    let (root, proof) = proof_item.clone().into_state_root_and_proof();
    let db = proof.into_memory_db::<HashingFor<cumulus_primitives_core::relay_chain::Block>>();
    let backend = sp_state_machine::TrieBackendBuilder::new(db, root).build();

    // this should contain both keys (1001, 1002). but we will now generate a proof without one of the keys (1001)
    let mut relevant_keys = proof_item.relevant_keys();
    // remove para 1001
    relevant_keys.remove(0);
    // re-generate the proof only for para 1002
    let proof = sp_state_machine::prove_read(backend, relevant_keys).expect("prove read");

    // We now have a state containing 1001 and 1002 paras, but only 1002 is passed in the proof (when 1001 is required)
    BlockTests::new()
        .with_relay_sproof_builder(move |_, relay_block_num, sproof| match relay_block_num {
            1 => {
                // We guarantee we generate the same DB by constructing the same items
                sproof.items.push(para_id_1001_item.clone());
                sproof.items.push(para_id_1002_item.clone());
            }
            _ => unreachable!(),
        })
        .with_overriden_state_proof(proof)
        .add(1, || {});
}

#[test]
#[should_panic(expected = "Invalid proof provided for para head key")]
fn test_should_panic_with_empty_proof() {
    // Since the default parachain list is vec![1001],
    // we must always include a sproof for this para_id
    let slot: InherentType = 10u64.into();
    let mut para_id_1001_item = ParaHeaderSproofBuilderItem::default();
    let mut proof_item = ParaHeaderSproofBuilder::default();

    para_id_1001_item.para_id = 1001.into();
    para_id_1001_item.author_id =
        HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
            parent_hash: Default::default(),
            number: Default::default(),
            state_root: Default::default(),
            extrinsics_root: Default::default(),
            digest: sp_runtime::generic::Digest {
                logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
            },
        });
    proof_item.items.push(para_id_1001_item.clone());

    // lets get the generated proof here. However we will modify later on the proof to not include anything
    let (root, proof) = proof_item.clone().into_state_root_and_proof();
    let db = proof.into_memory_db::<HashingFor<cumulus_primitives_core::relay_chain::Block>>();
    let backend = sp_state_machine::TrieBackendBuilder::new(db, root).build();

    // Empty relevant keys
    let relevant_keys: Vec<Vec<u8>> = Vec::new();
    // re-generate the proof for nothing
    let proof = sp_state_machine::prove_read(backend, relevant_keys).expect("prove read");

    // We now have a state containing 1001, but an empty proof will be passed
    BlockTests::new()
        .with_relay_sproof_builder(move |_, relay_block_num, sproof| match relay_block_num {
            1 => {
                // We guarantee we generate the same DB by constructing the same items
                sproof.items.push(para_id_1001_item.clone());
            }
            _ => unreachable!(),
        })
        .with_overriden_state_proof(proof)
        .add(1, || {});
}

#[test]
#[should_panic(expected = "Container chain author data needs to be present in every block!")]
fn test_not_inserting_inherent() {
    BlockTests::new()
        .with_relay_sproof_builder(|_, relay_block_num, sproof| match relay_block_num {
            1 => {
                let slot: InherentType = 13u64.into();
                let s = ParaHeaderSproofBuilderItem {
                    para_id: 1001.into(),
                    author_id: HeaderAs::NonEncoded(sp_runtime::generic::Header::<
                        u32,
                        BlakeTwo256,
                    > {
                        parent_hash: Default::default(),
                        number: Default::default(),
                        state_root: Default::default(),
                        extrinsics_root: Default::default(),
                        digest: sp_runtime::generic::Digest {
                            logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                        },
                    }),
                };
                sproof.items.push(s);
            }
            _ => unreachable!(),
        })
        .skip_inherent_insertion()
        .add(1, || {
            assert!(AuthorNoting::latest_author(ParaId::from(1001)).is_none());
        });
}

#[test]
#[ignore = "used to generate benchmark data"]
fn encode_proof_for_benchmarks() {
    println!("pub const ENCODED_PROOFS: &[(u32, &str, &[&str])] = &[");

    for x in 0u32..=100 {
        let mut sproof_builder = ParaHeaderSproofBuilder::default();

        for para_id in 0..x {
            let slot: InherentType = 13u64.into();
            let s = ParaHeaderSproofBuilderItem {
                para_id: para_id.into(),

                // TODO: this header can be arbitrarily large, because "digest.logs" is an unbounded vec
                author_id: HeaderAs::NonEncoded(dp_core::Header {
                    parent_hash: Default::default(),
                    number: Default::default(),
                    state_root: Default::default(),
                    extrinsics_root: Default::default(),
                    digest: sp_runtime::generic::Digest {
                        logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                    },
                }),
            };
            sproof_builder.items.push(s);
        }

        let (root, proof) = sproof_builder.into_state_root_and_proof();

        println!("({}, \"{}\", &[", x, hex::encode(root),);

        for x in proof.iter_nodes() {
            println!("\"{}\",", hex::encode(x));
        }

        println!("]),");
    }

    println!("];")
}

#[test]
fn test_set_author() {
    BlockTests::new()
        .with_relay_sproof_builder(|_, relay_block_num, sproof| match relay_block_num {
            1 => {
                let slot: InherentType = 13u64.into();
                let s = ParaHeaderSproofBuilderItem {
                    para_id: 1001.into(),
                    author_id: HeaderAs::NonEncoded(sp_runtime::generic::Header::<
                        u32,
                        BlakeTwo256,
                    > {
                        parent_hash: Default::default(),
                        number: 1,
                        state_root: Default::default(),
                        extrinsics_root: Default::default(),
                        digest: sp_runtime::generic::Digest {
                            logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                        },
                    }),
                };
                sproof.items.push(s);
            }
            _ => unreachable!(),
        })
        .add(1, || {
            assert_eq!(
                AuthorNoting::latest_author(ParaId::from(1001)),
                Some(ContainerChainBlockInfo {
                    block_number: 1,
                    author: 13u64,
                    latest_slot_number: 1u64.into()
                })
            );
            assert_ok!(AuthorNoting::set_author(
                RuntimeOrigin::root(),
                1001.into(),
                1,
                14u64,
                14u64.into()
            ));
            assert_eq!(
                AuthorNoting::latest_author(ParaId::from(1001)),
                Some(ContainerChainBlockInfo {
                    block_number: 1,
                    author: 14u64,
                    latest_slot_number: 14u64.into()
                })
            );
            System::assert_last_event(
                Event::LatestAuthorChanged {
                    para_id: 1001.into(),
                    block_number: 1,
                    new_author: 14u64,
                    latest_slot_number: 14u64.into(),
                }
                .into(),
            );
        });
}

#[test]
#[should_panic(expected = "DidSetContainerAuthorData must be updated only once in a block")]
fn test_on_initalize_does_not_kill_and_panics() {
    BlockTests::new()
        .skip_author_noting_on_initialize()
        .with_relay_sproof_builder(|_, relay_block_num, sproof| match relay_block_num {
            1 => {
                crate::DidSetContainerAuthorData::<Test>::put(true);
                let slot: InherentType = 13u64.into();
                let s = ParaHeaderSproofBuilderItem {
                    para_id: 1001.into(),
                    author_id: HeaderAs::NonEncoded(sp_runtime::generic::Header::<
                        u32,
                        BlakeTwo256,
                    > {
                        parent_hash: Default::default(),
                        number: Default::default(),
                        state_root: Default::default(),
                        extrinsics_root: Default::default(),
                        digest: sp_runtime::generic::Digest {
                            logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                        },
                    }),
                };

                sproof.items.push(s);
            }
            _ => unreachable!(),
        })
        .add(1, || {});
}

#[test]
fn test_header_non_decodable_does_not_insert() {
    BlockTests::new()
        .with_relay_sproof_builder(|_, relay_block_num, sproof| match relay_block_num {
            1 => {
                let s = ParaHeaderSproofBuilderItem {
                    para_id: 1001.into(),
                    author_id: HeaderAs::AlreadyEncoded(hex!("4321").to_vec()),
                };

                sproof.items.push(s);
            }
            _ => unreachable!(),
        })
        .add(1, || {
            assert_eq!(AuthorNoting::latest_author(ParaId::from(1001)), None);
        });
}

#[test]
fn test_non_aura_digest_does_not_insert_key() {
    BlockTests::new()
        .with_relay_sproof_builder(|_, relay_block_num, sproof| match relay_block_num {
            1 => {
                let slot: InherentType = 13u64.into();
                let s = ParaHeaderSproofBuilderItem {
                    para_id: 1001.into(),
                    author_id: HeaderAs::NonEncoded(sp_runtime::generic::Header::<
                        u32,
                        BlakeTwo256,
                    > {
                        parent_hash: Default::default(),
                        number: Default::default(),
                        state_root: Default::default(),
                        extrinsics_root: Default::default(),
                        // we inject a non-aura digest
                        digest: sp_runtime::generic::Digest {
                            logs: vec![DigestItem::PreRuntime(
                                [b'a', b'a', b'a', b'a'],
                                slot.encode(),
                            )],
                        },
                    }),
                };
                sproof.items.push(s);
            }
            _ => unreachable!(),
        })
        .add(1, || {
            assert_eq!(AuthorNoting::latest_author(ParaId::from(1001)), None);
        });
}

#[test]
fn test_non_decodable_slot_does_not_insert_key() {
    BlockTests::new()
        .with_relay_sproof_builder(|_, relay_block_num, sproof| match relay_block_num {
            1 => {
                let s = ParaHeaderSproofBuilderItem {
                    para_id: 1001.into(),
                    author_id: HeaderAs::NonEncoded(sp_runtime::generic::Header::<
                        u32,
                        BlakeTwo256,
                    > {
                        parent_hash: Default::default(),
                        number: Default::default(),
                        state_root: Default::default(),
                        extrinsics_root: Default::default(),
                        // we inject 1u8 slot, but inherentType is expected so it should not decode
                        digest: sp_runtime::generic::Digest {
                            logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, 1u8.encode())],
                        },
                    }),
                };
                sproof.items.push(s);
            }
            _ => unreachable!(),
        })
        .add(1, || {
            assert_eq!(AuthorNoting::latest_author(ParaId::from(1001)), None);
        });
}

#[test]
fn weights_assigned_to_extrinsics_are_correct() {
    new_test_ext().execute_with(|| {
        assert_eq!(
            crate::Call::<Test>::set_author {
                para_id: 1.into(),
                block_number: 1,
                author: 1u64,
                latest_slot_number: 0u64.into()
            }
            .get_dispatch_info()
            .weight,
            <() as crate::weights::WeightInfo>::set_author()
        );

        let sproof_builder = ParaHeaderSproofBuilder::default();

        let (relay_root, relay_chain_state) = sproof_builder.into_state_root_and_proof();
        frame_support::storage::unhashed::put(MOCK_RELAY_ROOT_KEY, &relay_root);

        let mut inherent_data = InherentData::default();
        let system_inherent_data = tp_author_noting_inherent::OwnParachainInherentData {
            relay_storage_proof: relay_chain_state,
        };
        inherent_data
            .put_data(
                tp_author_noting_inherent::INHERENT_IDENTIFIER,
                &system_inherent_data,
            )
            .expect("failed to put VFP inherent");
        let inherent_weight = AuthorNoting::create_inherent(&inherent_data)
            .expect("got an inherent")
            .dispatch_bypass_filter(RawOrigin::None.into())
            .expect("dispatch succeeded");

        assert_eq!(
            inherent_weight.actual_weight.unwrap(),
            <() as crate::weights::WeightInfo>::set_latest_author_data(
                <Test as crate::Config>::ContainerChains::current_container_chains().len() as u32
            )
        );
    });
}

#[test]
fn test_kill_author_data() {
    BlockTests::new()
        .with_relay_sproof_builder(|_, relay_block_num, sproof| match relay_block_num {
            1 => {
                let slot: InherentType = 13u64.into();
                let s = ParaHeaderSproofBuilderItem {
                    para_id: 1001.into(),
                    author_id: HeaderAs::NonEncoded(sp_runtime::generic::Header::<
                        u32,
                        BlakeTwo256,
                    > {
                        parent_hash: Default::default(),
                        number: 1,
                        state_root: Default::default(),
                        extrinsics_root: Default::default(),
                        digest: sp_runtime::generic::Digest {
                            logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                        },
                    }),
                };
                sproof.items.push(s);
            }
            _ => unreachable!(),
        })
        .add(1, || {
            assert_eq!(
                AuthorNoting::latest_author(ParaId::from(1001)),
                Some(ContainerChainBlockInfo {
                    block_number: 1,
                    author: 13u64,
                    latest_slot_number: 1u64.into()
                })
            );
            assert_ok!(AuthorNoting::kill_author_data(
                RuntimeOrigin::root(),
                1001.into(),
            ));
            assert_eq!(AuthorNoting::latest_author(ParaId::from(1001)), None);
            System::assert_last_event(
                Event::RemovedAuthorData {
                    para_id: 1001.into(),
                }
                .into(),
            );
        });
}

#[test]
fn test_author_id_insertion_not_first_log() {
    BlockTests::new()
        .with_relay_sproof_builder(|_, relay_block_num, sproof| match relay_block_num {
            1 => {
                let slot: InherentType = 13u64.into();
                let s = ParaHeaderSproofBuilderItem {
                    para_id: 1001.into(),
                    author_id: HeaderAs::NonEncoded(sp_runtime::generic::Header::<
                        u32,
                        BlakeTwo256,
                    > {
                        parent_hash: Default::default(),
                        number: 1,
                        state_root: Default::default(),
                        extrinsics_root: Default::default(),
                        digest: sp_runtime::generic::Digest {
                            logs: vec![
                                // Dummy item before aura log
                                DigestItem::PreRuntime([0; 4], vec![]),
                                DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode()),
                            ],
                        },
                    }),
                };
                sproof.items.push(s);
            }
            _ => unreachable!(),
        })
        .add(1, || {
            assert_eq!(
                AuthorNoting::latest_author(ParaId::from(1001)),
                Some(ContainerChainBlockInfo {
                    block_number: 1,
                    author: 13u64,
                    latest_slot_number: 1u64.into()
                })
            );
        });
}
