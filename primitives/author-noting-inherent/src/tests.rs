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
    crate::{tests::mock_relay_chain_impl::MyMockRelayInterface, OwnParachainInherentData},
    cumulus_pallet_parachain_system::RelayChainStateProof,
    cumulus_primitives_core::relay_chain::{BlakeTwo256, BlockNumber},
    dp_core::well_known_keys::para_id_head,
    futures::executor::block_on,
    hex_literal::hex,
    parity_scale_codec::{Decode, Encode},
    sp_consensus_aura::{inherents::InherentType, AURA_ENGINE_ID},
    sp_inherents::InherentDataProvider,
    sp_runtime::DigestItem,
    std::sync::atomic::{AtomicU8, Ordering},
    test_relay_sproof_builder::{HeaderAs, ParaHeaderSproofBuilder, ParaHeaderSproofBuilderItem},
};

#[test]
fn header_decode_collisions() {
    // The hex below is the result of encoding a Header to Vec<u8>, and then encoding that Vec<u8> again.
    // Trying to decode this bytes directly as a Header should always fail, but because of how the
    // SCALE codec works it can sometimes succeed and output garbage.
    let bad_value = hex!("e102ad81ae5c9623edf94e9ca481698383ac8032e13a8a0642407a51987e98a5d5db01010fcbe894fb15e253e2918af5633a040bd379fa5d225685101fa5e8d17843c68de9e6d71f42d894088c1cfb6d4ee9d2bf9abc5254428dcadc4997442007afb6e00806617572612048a659080000000005617572610101dc4e2be503910fb326840244eb65fe21d9a9a8f23414ab909f3baabb991e8855abd5a00f1640ec8df48687f33967887f4a86ae6299693e9baf28b7192722248d");
    let good_value = hex!("e102451c84b3d0383f1d7002fd597c45406bd8d2c0bace9e52bb35a8dbfa805b46c60501888d8570e847209a707668977b5792569e865796a9130e1c37fdb1fd7c6f3b73e87cbecebd0de4abd17b8a80995972d8187ae9998a87d134b807e9b8f5565e2b0806617572612049a6590800000000056175726101010ee968af2eac0ce1223b5618497961064542543a75b72abad1a7d919fc7d8937a4180c242670561c4179e8b83cedde3e80cfc99793b5a35cf020055fc80cb684");
    let bad: Result<sp_runtime::generic::Header<BlockNumber, BlakeTwo256>, _> =
        <_>::decode(&mut &bad_value[..]);
    let good: Result<sp_runtime::generic::Header<BlockNumber, BlakeTwo256>, _> =
        <_>::decode(&mut &good_value[..]);

    assert!(bad.is_err());
    assert!(good.is_ok());

    // But decoding as a Vec<u8> and then as a Header will always work.
    let bad: Result<sp_runtime::generic::Header<BlockNumber, BlakeTwo256>, _> =
        <Vec<u8>>::decode(&mut &bad_value[..]).and_then(|bytes| <_>::decode(&mut &bytes[..]));
    let good: Result<sp_runtime::generic::Header<BlockNumber, BlakeTwo256>, _> =
        <Vec<u8>>::decode(&mut &good_value[..]).and_then(|bytes| <_>::decode(&mut &bytes[..]));

    assert!(bad.is_ok());
    assert!(good.is_ok());
}

fn test_header() -> sp_runtime::generic::Header<u32, BlakeTwo256> {
    let slot: InherentType = 13u64.into();

    sp_runtime::generic::Header::<u32, BlakeTwo256> {
        parent_hash: Default::default(),
        number: Default::default(),
        state_root: Default::default(),
        extrinsics_root: Default::default(),
        digest: sp_runtime::generic::Digest {
            logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
        },
    }
}

#[test]
fn header_double_encode() {
    // The ParaHeaderSproofBuilder should always encode as a Vec<u8>, and then encode that Vec<u8> again.
    let header = test_header();
    let header_encoded = header.encode();
    let s = ParaHeaderSproofBuilderItem {
        para_id: 1001.into(),
        author_id: HeaderAs::NonEncoded(header),
    };

    let mut sb = ParaHeaderSproofBuilder::default();
    sb.items.push(s);
    let (state_root, proof) = sb.into_state_root_and_proof();

    let relay_state_proof = RelayChainStateProof::new(1001.into(), state_root, proof)
        .expect("Invalid relay chain state proof");
    let key = para_id_head(1001.into());
    // If the NonEncoded was not encoded once to Vec, and then again as a Vec, this would fail
    // because we are comparing the "decoded" entry with the encoded header
    let v: Vec<u8> = relay_state_proof.read_entry(&key, None).unwrap();
    assert_eq!(v, header_encoded);
}

#[test]
fn header_double_encode_even_if_already_encoded() {
    // The ParaHeaderSproofBuilder should always encode as a Vec<u8>, and then encode that Vec<u8> again.
    let header = test_header();
    let header_encoded = header.encode();
    let s = ParaHeaderSproofBuilderItem {
        para_id: 1001.into(),
        author_id: HeaderAs::AlreadyEncoded(header_encoded.clone()),
    };

    let mut sb = ParaHeaderSproofBuilder::default();
    sb.items.push(s);
    let (state_root, proof) = sb.into_state_root_and_proof();

    let relay_state_proof = RelayChainStateProof::new(1001.into(), state_root, proof)
        .expect("Invalid relay chain state proof");
    let key = para_id_head(1001.into());
    // If the AlreadyEncoded was not encoded again as a Vec, this would fail
    let v: Vec<u8> = relay_state_proof.read_entry(&key, None).unwrap();
    assert_eq!(v, header_encoded);
}

mod mock_relay_chain_impl {
    use {
        async_trait::async_trait,
        cumulus_primitives_core::{
            relay_chain::{
                BlockId, CommittedCandidateReceipt, OccupiedCoreAssumption, SessionIndex,
            },
            InboundHrmpMessage, ParaId,
        },
        cumulus_relay_chain_interface::{
            OverseerHandle, PHash, PHeader, RelayChainInterface, RelayChainResult,
        },
        futures::Stream,
        polkadot_primitives::{InboundDownwardMessage, PersistedValidationData, ValidatorId},
        sp_state_machine::StorageValue,
        std::{collections::BTreeMap, pin::Pin},
    };

    pub struct MyMockRelayInterface {
        pub prove_read: Box<
            dyn Fn(
                    &MyMockRelayInterface,
                    cumulus_relay_chain_interface::PHash,
                    &Vec<Vec<u8>>,
                )
                    -> cumulus_relay_chain_interface::RelayChainResult<sc_client_api::StorageProof>
                + Send
                + Sync,
        >,
    }

    #[async_trait]
    impl RelayChainInterface for MyMockRelayInterface {
        async fn validators(&self, _: PHash) -> RelayChainResult<Vec<ValidatorId>> {
            unimplemented!("Not needed for test")
        }

        async fn best_block_hash(&self) -> RelayChainResult<PHash> {
            unimplemented!("Not needed for test")
        }
        async fn finalized_block_hash(&self) -> RelayChainResult<PHash> {
            unimplemented!("Not needed for test")
        }

        async fn retrieve_dmq_contents(
            &self,
            _: ParaId,
            _: PHash,
        ) -> RelayChainResult<Vec<InboundDownwardMessage>> {
            unimplemented!("Not needed for test")
        }

        async fn retrieve_all_inbound_hrmp_channel_contents(
            &self,
            _: ParaId,
            _: PHash,
        ) -> RelayChainResult<BTreeMap<ParaId, Vec<InboundHrmpMessage>>> {
            unimplemented!("Not needed for test")
        }

        async fn persisted_validation_data(
            &self,
            _: PHash,
            _: ParaId,
            _: OccupiedCoreAssumption,
        ) -> RelayChainResult<Option<PersistedValidationData>> {
            unimplemented!("Not needed for test")
        }

        async fn candidate_pending_availability(
            &self,
            _: PHash,
            _: ParaId,
        ) -> RelayChainResult<Option<CommittedCandidateReceipt>> {
            unimplemented!("Not needed for test")
        }

        async fn session_index_for_child(&self, _: PHash) -> RelayChainResult<SessionIndex> {
            unimplemented!("Not needed for test")
        }

        async fn import_notification_stream(
            &self,
        ) -> RelayChainResult<Pin<Box<dyn Stream<Item = PHeader> + Send>>> {
            unimplemented!("Not needed for test")
        }

        async fn finality_notification_stream(
            &self,
        ) -> RelayChainResult<Pin<Box<dyn Stream<Item = PHeader> + Send>>> {
            unimplemented!("Not needed for test")
        }

        async fn is_major_syncing(&self) -> RelayChainResult<bool> {
            unimplemented!("Not needed for test")
        }

        fn overseer_handle(&self) -> RelayChainResult<OverseerHandle> {
            unimplemented!("Not needed for test")
        }

        async fn get_storage_by_key(
            &self,
            _: PHash,
            _: &[u8],
        ) -> RelayChainResult<Option<StorageValue>> {
            unimplemented!("Not needed for test")
        }

        async fn prove_read(
            &self,
            relay_parent: PHash,
            keys: &Vec<Vec<u8>>,
        ) -> RelayChainResult<sc_client_api::StorageProof> {
            (self.prove_read)(self, relay_parent, keys)
        }

        async fn wait_for_block(&self, _hash: PHash) -> RelayChainResult<()> {
            unimplemented!("Not needed for test")
        }

        async fn new_best_notification_stream(
            &self,
        ) -> RelayChainResult<Pin<Box<dyn Stream<Item = PHeader> + Send>>> {
            unimplemented!("Not needed for test")
        }

        async fn header(&self, _block_id: BlockId) -> RelayChainResult<Option<PHeader>> {
            unimplemented!("Not needed for test")
        }
    }
}

#[test]
fn create_inherent_with_no_para_ids() {
    let mock_relay_parent = Default::default();
    let mock_proof = sc_client_api::StorageProof::new(vec![]);
    let relay_chain = MyMockRelayInterface {
        prove_read: {
            let mock_proof = mock_proof.clone();
            let call_counter = AtomicU8::new(0);

            Box::new(move |_this, relay_parent, keys| {
                match call_counter.fetch_add(1, Ordering::SeqCst) {
                    0 => {
                        assert_eq!(relay_parent, mock_relay_parent);
                        assert_eq!(keys.len(), 0);
                        Ok(mock_proof.clone())
                    }
                    _ => panic!("Should only be called once"),
                }
            })
        },
    };

    let para_ids = &[];
    let proof = block_on(OwnParachainInherentData::create_at(
        mock_relay_parent,
        &relay_chain,
        para_ids,
    ));

    assert_eq!(
        proof,
        Some(OwnParachainInherentData {
            relay_storage_proof: mock_proof
        })
    );
}

#[test]
fn create_inherent_with_two_para_ids() {
    let mock_relay_parent = Default::default();
    let dummy_node = vec![1, 2, 3];
    let mock_proof = sc_client_api::StorageProof::new(vec![dummy_node]);
    let relay_chain = MyMockRelayInterface {
        prove_read: {
            let mock_proof = mock_proof.clone();
            let call_counter = AtomicU8::new(0);

            Box::new(move |_this, relay_parent, keys| {
                match call_counter.fetch_add(1, Ordering::SeqCst) {
                    0 => {
                        assert_eq!(relay_parent, mock_relay_parent);
                        assert_eq!(keys.len(), 2);
                        // Keys should be different because para ids are different
                        assert_ne!(keys[0], keys[1]);
                        Ok(mock_proof.clone())
                    }
                    _ => panic!("Should only be called once"),
                }
            })
        },
    };

    let para_ids = &[2000.into(), 2001.into()];
    let proof = block_on(OwnParachainInherentData::create_at(
        mock_relay_parent,
        &relay_chain,
        para_ids,
    ));

    assert_eq!(
        proof,
        Some(OwnParachainInherentData {
            relay_storage_proof: mock_proof
        })
    );
}

#[test]
fn test_provide_inherent_data() {
    // Ensure that provide_inherent_data stores the data at the correct key, and the data can be decoded
    let dummy_node = vec![1, 2, 3];
    let relay_chain = MyMockRelayInterface {
        prove_read: Box::new(move |_, _, _| {
            Ok(sc_client_api::StorageProof::new(vec![dummy_node.clone()]))
        }),
    };
    let relay_parent = Default::default();
    let para_ids = &[];
    let proof = block_on(OwnParachainInherentData::create_at(
        relay_parent,
        &relay_chain,
        para_ids,
    ))
    .unwrap();

    let mut inherent_data = sp_inherents::InherentData::new();
    block_on(proof.provide_inherent_data(&mut inherent_data)).unwrap();
    let decoded: OwnParachainInherentData = inherent_data
        .get_data(&crate::INHERENT_IDENTIFIER)
        .unwrap()
        .unwrap();

    assert_eq!(decoded, proof);
}
