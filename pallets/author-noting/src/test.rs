use {
    crate::mock::*,
    cumulus_primitives_core::ParaId,
    hex_literal::hex,
    parity_scale_codec::Encode,
    sp_consensus_aura::{inherents::InherentType, AURA_ENGINE_ID},
    sp_core::H256,
    sp_runtime::{generic::DigestItem, traits::BlakeTwo256},
    test_relay_sproof_builder::{HeaderAs, ParaHeaderSproofBuilder, ParaHeaderSproofBuilderItem},
};

#[test]
fn test_author_id_insertion() {
    BlockTests::new()
        .with_relay_sproof_builder(|_, relay_block_num, sproof| match relay_block_num {
            1 => {
                let slot: InherentType = 13u64.into();
                let mut s = ParaHeaderSproofBuilderItem::default();
                s.para_id = 1001.into();
                s.author_id =
                    HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                        parent_hash: Default::default(),
                        number: Default::default(),
                        state_root: Default::default(),
                        extrinsics_root: Default::default(),
                        digest: sp_runtime::generic::Digest {
                            logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                        },
                    });
                sproof.items.push(s);
            }
            _ => unreachable!(),
        })
        .add(1, || {
            assert_eq!(AuthorNoting::latest_author(ParaId::from(1001)), Some(13u64));
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
                    let mut s = ParaHeaderSproofBuilderItem::default();
                    s.para_id = 1001.into();
                    s.author_id = HeaderAs::AlreadyEncoded(statemint_data.to_vec());
                    sproof.items.push(s);
                }
                _ => unreachable!(),
            }
        })
        .add(1, || {
            assert_eq!(
                AuthorNoting::latest_author(ParaId::from(1001)),
                // Our mock author fetcher will just note the slot
                Some(140006956u64)
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
                let mut s = ParaHeaderSproofBuilderItem::default();
                s.para_id = 1001.into();
                s.author_id =
                    HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                        parent_hash: Default::default(),
                        number: Default::default(),
                        state_root: Default::default(),
                        extrinsics_root: Default::default(),
                        digest: sp_runtime::generic::Digest {
                            logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                        },
                    });
                sproof.items.push(s);
            }
            2 => {
                let slot: InherentType = 13u64.into();
                let mut s = ParaHeaderSproofBuilderItem::default();
                s.para_id = 1001.into();
                s.author_id =
                    HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                        parent_hash: Default::default(),
                        number: Default::default(),
                        state_root: Default::default(),
                        extrinsics_root: Default::default(),
                        digest: sp_runtime::generic::Digest {
                            logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                        },
                    });
                sproof.items.push(s);

                let slot: InherentType = 14u64.into();
                let mut s = ParaHeaderSproofBuilderItem::default();
                s.para_id = 1002.into();
                s.author_id =
                    HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                        parent_hash: Default::default(),
                        number: Default::default(),
                        state_root: Default::default(),
                        extrinsics_root: Default::default(),
                        digest: sp_runtime::generic::Digest {
                            logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                        },
                    });
                sproof.items.push(s);
            }
            _ => unreachable!(),
        })
        .add(1, || {
            // Writing to this pallet storage will only change the sproofs of the next block,
            // not the ones of the current block
            MockData::mutate(|m| {
                m.container_chains = vec![1001.into(), 1002.into()];
            });
            assert_eq!(AuthorNoting::latest_author(ParaId::from(1001)), Some(10u64));
            assert_eq!(AuthorNoting::latest_author(ParaId::from(1002)), None);
        })
        .add(2, || {
            assert_eq!(AuthorNoting::latest_author(ParaId::from(1001)), Some(13u64));
            assert_eq!(AuthorNoting::latest_author(ParaId::from(1002)), Some(14u64));
        });
}

#[test]
#[should_panic]
fn test_should_panic_with_invalid_proof_root() {
    BlockTests::new()
        .with_relay_sproof_builder(|_, relay_block_num, sproof| match relay_block_num {
            1 => {
                let slot: InherentType = 13u64.into();
                let mut s = ParaHeaderSproofBuilderItem::default();
                s.para_id = 1001.into();
                s.author_id =
                    HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                        parent_hash: Default::default(),
                        number: Default::default(),
                        state_root: Default::default(),
                        extrinsics_root: Default::default(),
                        digest: sp_runtime::generic::Digest {
                            logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                        },
                    });
                sproof.items.push(s);
            }
            _ => unreachable!(),
        })
        // Insert an invalid root, not matching the proof generated
        .with_overriden_state_root(H256::default())
        .add(1, || {
            assert_eq!(AuthorNoting::latest_author(ParaId::from(1001)), Some(13u64));
        });
}

#[test]
#[should_panic]
fn test_should_panic_with_invalid_proof_state() {
    let sproof_builder = ParaHeaderSproofBuilder::default();
    let (_, relay_chain_state) = sproof_builder.into_state_root_and_proof();

    BlockTests::new()
        .with_relay_sproof_builder(|_, relay_block_num, sproof| match relay_block_num {
            1 => {
                let slot: InherentType = 13u64.into();
                let mut s = ParaHeaderSproofBuilderItem::default();
                s.para_id = 1001.into();
                s.author_id =
                    HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                        parent_hash: Default::default(),
                        number: Default::default(),
                        state_root: Default::default(),
                        extrinsics_root: Default::default(),
                        digest: sp_runtime::generic::Digest {
                            logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                        },
                    });
                sproof.items.push(s);
            }
            _ => unreachable!(),
        })
        // Insert an invalid root, not matching the proof generated
        .with_overriden_state_proof(relay_chain_state)
        .add(1, || {
            assert_eq!(AuthorNoting::latest_author(ParaId::from(1001)), Some(13u64));
        });
}
