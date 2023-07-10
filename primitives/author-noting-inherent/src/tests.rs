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

use crate::tests::dummy::make_validator_and_api;

use {
    cumulus_pallet_parachain_system::RelayChainStateProof,
    cumulus_primitives_core::relay_chain::{BlakeTwo256, BlockNumber},
    hex_literal::hex,
    parity_scale_codec::{Decode, Encode},
    sp_consensus_aura::{inherents::InherentType, AURA_ENGINE_ID},
    sp_runtime::DigestItem,
    test_relay_sproof_builder::{HeaderAs, ParaHeaderSproofBuilder, ParaHeaderSproofBuilderItem},
    tp_core::well_known_keys::para_id_head,
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
    let mut s = ParaHeaderSproofBuilderItem::default();
    s.para_id = 1001.into();
    let header = test_header();
    let header_encoded = header.encode();
    s.author_id = HeaderAs::NonEncoded(header);

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
    let mut s = ParaHeaderSproofBuilderItem::default();
    s.para_id = 1001.into();
    let header = test_header();
    let header_encoded = header.encode();
    s.author_id = HeaderAs::AlreadyEncoded(header_encoded.clone());

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

mod dummy {
    use super::block_announce::BlockAnnounceValidator;
    use super::*;
    use async_trait::async_trait;
    use core::pin::Pin;
    use cumulus_primitives_core::relay_chain::Block;
    use cumulus_primitives_core::relay_chain::SessionIndex;
    use cumulus_primitives_core::relay_chain::ValidatorId;
    use cumulus_primitives_core::InboundHrmpMessage;
    use cumulus_relay_chain_inprocess_interface::{check_block_in_chain, BlockCheckStatus};
    use cumulus_relay_chain_interface::OverseerHandle;
    use cumulus_relay_chain_interface::PHash;
    use cumulus_relay_chain_interface::PHeader;
    use cumulus_relay_chain_interface::ParaId;
    use cumulus_relay_chain_interface::RelayChainError;
    use cumulus_relay_chain_interface::RelayChainInterface;
    use cumulus_relay_chain_interface::RelayChainResult;
    use futures::FutureExt;
    use futures::Stream;
    use futures::StreamExt;
    use parking_lot::Mutex;
    use polkadot_primitives::CandidateCommitments;
    use polkadot_primitives::CandidateDescriptor;
    use polkadot_primitives::CollatorPair;
    use polkadot_primitives::CommittedCandidateReceipt;
    use polkadot_primitives::HeadData;
    use polkadot_primitives::Header;
    use polkadot_primitives::InboundDownwardMessage;
    use polkadot_primitives::OccupiedCoreAssumption;
    use polkadot_primitives::PersistedValidationData;
    use polkadot_primitives::ValidationCodeHash;
    use polkadot_test_client::{
        Client as PClient, ClientBlockImportExt, DefaultTestClientBuilderExt,
        FullBackend as PBackend, InitPolkadotBlockBuilder, TestClientBuilder, TestClientBuilderExt,
    };
    use sc_client_api::Backend;
    use sc_client_api::BlockchainEvents;
    use sc_client_api::HeaderBackend;
    use sp_core::Pair;
    use sp_keyring::Sr25519Keyring;
    use sp_state_machine::StorageValue;
    use std::collections::BTreeMap;
    use std::sync::Arc;
    use std::time::Duration;

    #[derive(Default)]
    struct ApiData {
        validators: Vec<ValidatorId>,
        has_pending_availability: bool,
    }

    #[derive(Clone)]
    pub struct DummyRelayChainInterface {
        data: Arc<Mutex<ApiData>>,
        relay_client: Arc<PClient>,
        relay_backend: Arc<PBackend>,
    }

    impl DummyRelayChainInterface {
        fn new() -> Self {
            let builder = TestClientBuilder::new();
            let relay_backend = builder.backend();

            Self {
                data: Arc::new(Mutex::new(ApiData {
                    validators: vec![Sr25519Keyring::Alice.public().into()],
                    has_pending_availability: false,
                })),
                relay_client: Arc::new(builder.build()),
                relay_backend,
            }
        }
    }

    #[async_trait]
    impl RelayChainInterface for DummyRelayChainInterface {
        async fn validators(&self, _: PHash) -> RelayChainResult<Vec<ValidatorId>> {
            Ok(self.data.lock().validators.clone())
        }

        async fn best_block_hash(&self) -> RelayChainResult<PHash> {
            Ok(self.relay_backend.blockchain().info().best_hash)
        }
        async fn finalized_block_hash(&self) -> RelayChainResult<PHash> {
            Ok(self.relay_backend.blockchain().info().finalized_hash)
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
            Ok(BTreeMap::new())
        }

        async fn persisted_validation_data(
            &self,
            _: PHash,
            _: ParaId,
            _: OccupiedCoreAssumption,
        ) -> RelayChainResult<Option<PersistedValidationData>> {
            Ok(Some(PersistedValidationData {
                parent_head: HeadData(default_header().encode()),
                ..Default::default()
            }))
        }

        async fn candidate_pending_availability(
            &self,
            _: PHash,
            _: ParaId,
        ) -> RelayChainResult<Option<CommittedCandidateReceipt>> {
            if self.data.lock().has_pending_availability {
                Ok(Some(CommittedCandidateReceipt {
                    descriptor: CandidateDescriptor {
                        para_head: polkadot_parachain::primitives::HeadData(
                            default_header().encode(),
                        )
                        .hash(),
                        para_id: 0u32.into(),
                        relay_parent: PHash::random(),
                        collator: CollatorPair::generate().0.public(),
                        persisted_validation_data_hash: PHash::random(),
                        pov_hash: PHash::random(),
                        erasure_root: PHash::random(),
                        signature: sp_core::sr25519::Signature([0u8; 64]).into(),
                        validation_code_hash: ValidationCodeHash::from(PHash::random()),
                    },
                    commitments: CandidateCommitments {
                        upward_messages: Default::default(),
                        horizontal_messages: Default::default(),
                        new_validation_code: None,
                        head_data: HeadData(Vec::new()),
                        processed_downward_messages: 0,
                        hrmp_watermark: 0,
                    },
                }))
            } else {
                Ok(None)
            }
        }

        async fn session_index_for_child(&self, _: PHash) -> RelayChainResult<SessionIndex> {
            Ok(0)
        }

        async fn import_notification_stream(
            &self,
        ) -> RelayChainResult<Pin<Box<dyn Stream<Item = PHeader> + Send>>> {
            Ok(Box::pin(
                self.relay_client
                    .import_notification_stream()
                    .map(|notification| notification.header),
            ))
        }

        async fn finality_notification_stream(
            &self,
        ) -> RelayChainResult<Pin<Box<dyn Stream<Item = PHeader> + Send>>> {
            Ok(Box::pin(
                self.relay_client
                    .finality_notification_stream()
                    .map(|notification| notification.header),
            ))
        }

        async fn is_major_syncing(&self) -> RelayChainResult<bool> {
            Ok(false)
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
            _: PHash,
            _: &Vec<Vec<u8>>,
        ) -> RelayChainResult<sc_client_api::StorageProof> {
            unimplemented!("Not needed for test")
        }

        async fn wait_for_block(&self, hash: PHash) -> RelayChainResult<()> {
            let mut listener = match check_block_in_chain(
                self.relay_backend.clone(),
                self.relay_client.clone(),
                hash,
            )? {
                BlockCheckStatus::InChain => return Ok(()),
                BlockCheckStatus::Unknown(listener) => listener,
            };

            let mut timeout = futures_timer::Delay::new(Duration::from_secs(10)).fuse();

            loop {
                futures::select! {
                    _ = timeout => return Err(RelayChainError::WaitTimeout(hash)),
                    evt = listener.next() => match evt {
                        Some(evt) if evt.hash == hash => return Ok(()),
                        // Not the event we waited on.
                        Some(_) => continue,
                        None => return Err(RelayChainError::ImportListenerClosed(hash)),
                    }
                }
            }
        }

        async fn new_best_notification_stream(
            &self,
        ) -> RelayChainResult<Pin<Box<dyn Stream<Item = PHeader> + Send>>> {
            let notifications_stream = self.relay_client.import_notification_stream().filter_map(
                |notification| async move {
                    if notification.is_new_best {
                        Some(notification.header)
                    } else {
                        None
                    }
                },
            );
            Ok(Box::pin(notifications_stream))
        }
    }

    pub fn make_validator_and_api() -> (
        BlockAnnounceValidator<Block, Arc<DummyRelayChainInterface>>,
        Arc<DummyRelayChainInterface>,
    ) {
        let relay_chain_interface = Arc::new(DummyRelayChainInterface::new());
        (
            BlockAnnounceValidator::new(relay_chain_interface.clone(), ParaId::from(56)),
            relay_chain_interface,
        )
    }

    pub fn default_header() -> Header {
        Header {
            number: 1,
            digest: Default::default(),
            extrinsics_root: Default::default(),
            parent_hash: Default::default(),
            state_root: Default::default(),
        }
    }
}

mod block_announce {
    use super::*;
    use core::marker::PhantomData;
    use core::fmt;
    use core::pin::Pin;
    use cumulus_relay_chain_interface::{ParaId, RelayChainInterface};
    use parity_scale_codec::DecodeAll;
    use polkadot_primitives::{CandidateReceipt, OccupiedCoreAssumption, CompactStatement, UncheckedSigned, HeadData, SigningContext};
    use sp_api::{BlockT, HeaderT};
    use sp_consensus::block_validation::{
        BlockAnnounceValidator as BlockAnnounceValidatorT, Validation,
    };
    use futures::{Future, FutureExt};
    use cumulus_relay_chain_interface::PHash;
    use polkadot_node_primitives::{CollationSecondedSignal, Statement};

    type BoxedError = Box<dyn std::error::Error + Send>;

    #[derive(Debug)]
    struct BlockAnnounceError(String);
    impl std::error::Error for BlockAnnounceError {}

    impl fmt::Display for BlockAnnounceError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.fmt(f)
        }
    }

    /// The data that we attach to a block announcement.
    ///
    /// This will be used to prove that a header belongs to a block that is probably being backed by
    /// the relay chain.
    #[derive(Encode, Debug)]
    pub struct BlockAnnounceData {
        /// The receipt identifying the candidate.
        receipt: CandidateReceipt,
        /// The seconded statement issued by a relay chain validator that approves the candidate.
        statement: UncheckedSigned<CompactStatement>,
        /// The relay parent that was used as context to sign the [`Self::statement`].
        relay_parent: PHash,
    }

    impl Decode for BlockAnnounceData {
        fn decode<I: parity_scale_codec::Input>(input: &mut I) -> Result<Self, parity_scale_codec::Error> {
            let receipt = CandidateReceipt::decode(input)?;
            let statement = UncheckedSigned::<CompactStatement>::decode(input)?;

            let relay_parent = match PHash::decode(input) {
                Ok(p) => p,
                // For being backwards compatible, we support missing relay-chain parent.
                Err(_) => receipt.descriptor.relay_parent,
            };

            Ok(Self {
                receipt,
                statement,
                relay_parent,
            })
        }
    }

    impl BlockAnnounceData {
        /// Validate that the receipt, statement and announced header match.
        ///
        /// This will not check the signature, for this you should use [`BlockAnnounceData::check_signature`].
        fn validate(&self, encoded_header: Vec<u8>) -> Result<(), Validation> {
            let candidate_hash =
                if let CompactStatement::Seconded(h) = self.statement.unchecked_payload() {
                    h
                } else {
                    return Err(Validation::Failure { disconnect: true });
                };

            if *candidate_hash != self.receipt.hash() {
                return Err(Validation::Failure { disconnect: true });
            }

            if HeadData(encoded_header).hash() != self.receipt.descriptor.para_head {
                return Err(Validation::Failure { disconnect: true });
            }

            Ok(())
        }

        /// Check the signature of the statement.
        ///
        /// Returns an `Err(_)` if it failed.
        async fn check_signature<RCInterface>(
            self,
            relay_chain_client: &RCInterface,
        ) -> Result<Validation, BlockAnnounceError>
        where
            RCInterface: RelayChainInterface + 'static,
        {
            let validator_index = self.statement.unchecked_validator_index();

            let session_index = match relay_chain_client
                .session_index_for_child(self.relay_parent)
                .await
            {
                Ok(r) => r,
                Err(e) => return Err(BlockAnnounceError(format!("{:?}", e))),
            };

            let signing_context = SigningContext {
                parent_hash: self.relay_parent,
                session_index,
            };

            // Check that the signer is a legit validator.
            let authorities = match relay_chain_client.validators(self.relay_parent).await {
                Ok(r) => r,
                Err(e) => return Err(BlockAnnounceError(format!("{:?}", e))),
            };
            let signer = match authorities.get(validator_index.0 as usize) {
                Some(r) => r,
                None => {
                    return Ok(Validation::Failure { disconnect: true });
                }
            };

            // Check statement is correctly signed.
            if self
                .statement
                .try_into_checked(&signing_context, signer)
                .is_err()
            {
                return Ok(Validation::Failure { disconnect: true });
            }

            Ok(Validation::Success { is_new_best: true })
        }
    }

    impl TryFrom<&'_ CollationSecondedSignal> for BlockAnnounceData {
        type Error = ();

        fn try_from(signal: &CollationSecondedSignal) -> Result<BlockAnnounceData, ()> {
            let receipt = if let Statement::Seconded(receipt) = signal.statement.payload() {
                receipt.to_plain()
            } else {
                return Err(());
            };

            Ok(BlockAnnounceData {
                receipt,
                statement: signal.statement.convert_payload().into(),
                relay_parent: signal.relay_parent,
            })
        }
    }

    /// Parachain specific block announce validator.
    ///
    /// This block announce validator is required if the parachain is running
    /// with the relay chain provided consensus to make sure each node only
    /// imports a reasonable number of blocks per round. The relay chain provided
    /// consensus doesn't have any authorities and so it could happen that without
    /// this special block announce validator a node would need to import *millions*
    /// of blocks per round, which is clearly not doable.
    ///
    /// To solve this problem, each block announcement is delayed until a collator
    /// has received a [`Statement::Seconded`] for its `PoV`. This message tells the
    /// collator that its `PoV` was validated successfully by a parachain validator and
    /// that it is very likely that this `PoV` will be included in the relay chain. Every
    /// collator that doesn't receive the message for its `PoV` will not announce its block.
    /// For more information on the block announcement, see [`WaitToAnnounce`].
    ///
    /// For each block announcement that is received, the generic block announcement validation
    /// will call this validator and provides the extra data that was attached to the announcement.
    /// We call this extra data `justification`.
    /// It is expected that the attached data is a SCALE encoded [`BlockAnnounceData`]. The
    /// statement is checked to be a [`CompactStatement::Seconded`] and that it is signed by an active
    /// parachain validator.
    ///
    /// If no justification was provided we check if the block announcement is at the tip of the known
    /// chain. If it is at the tip, it is required to provide a justification or otherwise we reject
    /// it. However, if the announcement is for a block below the tip the announcement is accepted
    /// as it probably comes from a node that is currently syncing the chain.
    #[derive(Clone)]
    pub struct BlockAnnounceValidator<Block, RCInterface> {
        phantom: PhantomData<Block>,
        relay_chain_interface: RCInterface,
        para_id: ParaId,
    }

    impl<Block, RCInterface> BlockAnnounceValidator<Block, RCInterface>
    where
        RCInterface: Clone,
    {
        /// Create a new [`BlockAnnounceValidator`].
        pub fn new(relay_chain_interface: RCInterface, para_id: ParaId) -> Self {
            Self {
                phantom: Default::default(),
                relay_chain_interface,
                para_id,
            }
        }
    }

    impl<Block: BlockT, RCInterface> BlockAnnounceValidator<Block, RCInterface>
    where
        RCInterface: RelayChainInterface + Clone,
    {
        /// Get the included block of the given parachain in the relay chain.
        async fn included_block(
            relay_chain_interface: &RCInterface,
            hash: PHash,
            para_id: ParaId,
        ) -> Result<Block::Header, BoxedError> {
            let validation_data = relay_chain_interface
                .persisted_validation_data(hash, para_id, OccupiedCoreAssumption::TimedOut)
                .await
                .map_err(|e| Box::new(BlockAnnounceError(format!("{:?}", e))) as Box<_>)?
                .ok_or_else(|| {
                    Box::new(BlockAnnounceError(
                        "Could not find parachain head in relay chain".into(),
                    )) as Box<_>
                })?;
            let para_head = Block::Header::decode(&mut &validation_data.parent_head.0[..])
                .map_err(|e| {
                    Box::new(BlockAnnounceError(format!(
                        "Failed to decode parachain head: {:?}",
                        e
                    ))) as Box<_>
                })?;

            Ok(para_head)
        }

        /// Get the backed block hash of the given parachain in the relay chain.
        async fn backed_block_hash(
            relay_chain_interface: &RCInterface,
            hash: PHash,
            para_id: ParaId,
        ) -> Result<Option<PHash>, BoxedError> {
            let candidate_receipt = relay_chain_interface
                .candidate_pending_availability(hash, para_id)
                .await
                .map_err(|e| Box::new(BlockAnnounceError(format!("{:?}", e))) as Box<_>)?;

            Ok(candidate_receipt.map(|cr| cr.descriptor.para_head))
        }

        /// Handle a block announcement with empty data (no statement) attached to it.
        async fn handle_empty_block_announce_data(
            &self,
            header: Block::Header,
        ) -> Result<Validation, BoxedError> {
            let relay_chain_interface = self.relay_chain_interface.clone();
            let para_id = self.para_id;

            // Check if block is equal or higher than best (this requires a justification)
            let relay_chain_best_hash = relay_chain_interface
                .best_block_hash()
                .await
                .map_err(|e| Box::new(e) as Box<_>)?;
            let block_number = header.number();

            let best_head =
                Self::included_block(&relay_chain_interface, relay_chain_best_hash, para_id)
                    .await?;
            let known_best_number = best_head.number();
            let backed_block = || async {
                Self::backed_block_hash(&relay_chain_interface, relay_chain_best_hash, para_id)
                    .await
            };

            if best_head == header {
                Ok(Validation::Success { is_new_best: true })
            } else if Some(HeadData(header.encode()).hash()) == backed_block().await? {
                Ok(Validation::Success { is_new_best: true })
            } else if block_number >= known_best_number {
                Ok(Validation::Failure { disconnect: false })
            } else {
                Ok(Validation::Success { is_new_best: false })
            }
        }
    }

    impl<Block: BlockT, RCInterface> BlockAnnounceValidatorT<Block>
        for BlockAnnounceValidator<Block, RCInterface>
    where
        RCInterface: RelayChainInterface + Clone + 'static,
    {
        fn validate(
            &mut self,
            header: &Block::Header,
            data: &[u8],
        ) -> Pin<Box<dyn Future<Output = Result<Validation, BoxedError>> + Send>> {
            let relay_chain_interface = self.relay_chain_interface.clone();
            let data = data.to_vec();
            let header = header.clone();
            let header_encoded = header.encode();
            let block_announce_validator = self.clone();

            async move {
                let relay_chain_is_syncing = relay_chain_interface
                    .is_major_syncing()
                    .await
                    .map_err(|e| {
                    })
                    .unwrap_or(false);

                if relay_chain_is_syncing {
                    return Ok(Validation::Success { is_new_best: false });
                }

                if data.is_empty() {
                    return block_announce_validator
                        .handle_empty_block_announce_data(header)
                        .await;
                }

                let block_announce_data = match BlockAnnounceData::decode_all(&mut data.as_slice())
                {
                    Ok(r) => r,
                    Err(err) => {
                        return Err(Box::new(BlockAnnounceError(format!(
                            "Can not decode the `BlockAnnounceData`: {:?}",
                            err
                        ))) as Box<_>)
                    }
                };

                if let Err(e) = block_announce_data.validate(header_encoded) {
                    return Ok(e);
                }

                let relay_parent = block_announce_data.receipt.descriptor.relay_parent;

                relay_chain_interface
                    .wait_for_block(relay_parent)
                    .await
                    .map_err(|e| Box::new(BlockAnnounceError(e.to_string())) as Box<_>)?;

                block_announce_data
                    .check_signature(&relay_chain_interface)
                    .await
                    .map_err(|e| Box::new(e) as Box<_>)
            }
            .boxed()
        }
    }
}

use crate::tests::dummy::default_header;
use futures::executor::block_on;
use polkadot_primitives::Header;
use polkadot_test_client::sp_consensus::block_validation::Validation;
use sp_consensus::block_validation::BlockAnnounceValidator;
use cumulus_primitives_core::relay_chain::Hash;

#[test]
fn valid_if_no_data_and_less_than_best_known_number() {
	let mut validator = make_validator_and_api().0;
	let header = Header { number: 0, ..default_header() };
	let res = block_on(validator.validate(&header, &[]));

	assert_eq!(
		res.unwrap(),
		Validation::Success { is_new_best: false },
		"validating without data with block number < best known number is always a success",
	);
}

#[test]
fn invalid_if_no_data_exceeds_best_known_number() {
	let mut validator = make_validator_and_api().0;
	let header = Header { number: 1, state_root: Hash::random(), ..default_header() };
	let res = block_on(validator.validate(&header, &[]));

	assert_eq!(
		res.unwrap(),
		Validation::Failure { disconnect: false },
		"validation fails if no justification and block number >= best known number",
	);
}

#[test]
fn valid_if_no_data_and_block_matches_best_known_block() {
	let mut validator = make_validator_and_api().0;
	let res = block_on(validator.validate(&default_header(), &[]));

	assert_eq!(
		res.unwrap(),
		Validation::Success { is_new_best: true },
		"validation is successful when the block hash matches the best known block",
	);
}

/*
#[test]
fn check_statement_is_encoded_correctly() {
	let mut validator = make_validator_and_api().0;
	let header = default_header();
	let res = block_on(validator.validate(&header, &[0x42]))
		.expect_err("Should fail on invalid encoded statement");

	check_error(res, |error| {
		matches!(
			error,
			BlockAnnounceError(x) if x.contains("Can not decode the `BlockAnnounceData`")
		)
	});
}

#[test]
fn block_announce_data_decoding_should_reject_extra_data() {
	let (mut validator, api) = make_validator_and_api();

	let (signal, header) = block_on(make_gossip_message_and_header_using_genesis(api, 1));
	let mut data = BlockAnnounceData::try_from(&signal).unwrap().encode();
	data.push(0x42);

	let res = block_on(validator.validate(&header, &data)).expect_err("Should return an error ");

	check_error(res, |error| {
		matches!(
			error,
			BlockAnnounceError(x) if x.contains("Input buffer has still data left after decoding!")
		)
	});
}

#[derive(Encode, Decode, Debug)]
struct LegacyBlockAnnounceData {
	receipt: CandidateReceipt,
	statement: UncheckedSigned<CompactStatement>,
}

#[test]
fn legacy_block_announce_data_handling() {
	let (_, api) = make_validator_and_api();

	let (signal, _) = block_on(make_gossip_message_and_header_using_genesis(api, 1));

	let receipt = if let Statement::Seconded(receipt) = signal.statement.payload() {
		receipt.to_plain()
	} else {
		panic!("Invalid")
	};

	let legacy = LegacyBlockAnnounceData {
		receipt: receipt.clone(),
		statement: signal.statement.convert_payload().into(),
	};

	let data = legacy.encode();

	let block_data =
		BlockAnnounceData::decode(&mut &data[..]).expect("Decoding works from legacy works");
	assert_eq!(receipt.descriptor.relay_parent, block_data.relay_parent);

	let data = block_data.encode();
	LegacyBlockAnnounceData::decode(&mut &data[..]).expect("Decoding works");
}

#[test]
fn check_signer_is_legit_validator() {
	let (mut validator, api) = make_validator_and_api();

	let (signal, header) = block_on(make_gossip_message_and_header_using_genesis(api, 1));
	let data = BlockAnnounceData::try_from(&signal).unwrap().encode();

	let res = block_on(validator.validate(&header, &data));
	assert_eq!(Validation::Failure { disconnect: true }, res.unwrap());
}

#[test]
fn check_statement_is_correctly_signed() {
	let (mut validator, api) = make_validator_and_api();

	let (signal, header) = block_on(make_gossip_message_and_header_using_genesis(api, 0));

	let mut data = BlockAnnounceData::try_from(&signal).unwrap().encode();

	// The signature comes at the end of the type, so change a bit to make the signature invalid.
	let last = data.len() - 1;
	data[last] = data[last].wrapping_add(1);

	let res = block_on(validator.validate(&header, &data));
	assert_eq!(Validation::Failure { disconnect: true }, res.unwrap());
}

#[tokio::test]
async fn check_statement_seconded() {
	let (mut validator, relay_chain_interface) = make_validator_and_api();
	let header = default_header();
	let relay_parent = H256::from_low_u64_be(1);

	let keystore: KeystorePtr = Arc::new(MemoryKeystore::new());
	let alice_public = Keystore::sr25519_generate_new(
		&*keystore,
		ValidatorId::ID,
		Some(&Sr25519Keyring::Alice.to_seed()),
	)
	.unwrap();
	let session_index = relay_chain_interface.session_index_for_child(relay_parent).await.unwrap();
	let signing_context = SigningContext { parent_hash: relay_parent, session_index };

	let statement = Statement::Valid(Default::default());

	let signed_statement = SignedFullStatement::sign(
		&keystore,
		statement,
		&signing_context,
		0.into(),
		&alice_public.into(),
	)
	.ok()
	.flatten()
	.expect("Signs statement");

	let data = BlockAnnounceData {
		receipt: CandidateReceipt {
			commitments_hash: PHash::random(),
			descriptor: CandidateDescriptor {
				para_head: HeadData(Vec::new()).hash(),
				para_id: 0u32.into(),
				relay_parent: PHash::random(),
				collator: CollatorPair::generate().0.public(),
				persisted_validation_data_hash: PHash::random(),
				pov_hash: PHash::random(),
				erasure_root: PHash::random(),
				signature: sp_core::sr25519::Signature([0u8; 64]).into(),
				validation_code_hash: ValidationCodeHash::from(PHash::random()),
			},
		},
		statement: signed_statement.convert_payload().into(),
		relay_parent,
	}
	.encode();

	let res = block_on(validator.validate(&header, &data));
	assert_eq!(Validation::Failure { disconnect: true }, res.unwrap());
}

#[test]
fn check_header_match_candidate_receipt_header() {
	let (mut validator, api) = make_validator_and_api();

	let (signal, mut header) = block_on(make_gossip_message_and_header_using_genesis(api, 0));
	let data = BlockAnnounceData::try_from(&signal).unwrap().encode();
	header.number = 300;

	let res = block_on(validator.validate(&header, &data));
	assert_eq!(Validation::Failure { disconnect: true }, res.unwrap());
}

/// Test that ensures that we postpone the block announce verification until
/// a relay chain block is imported. This is important for when we receive a
/// block announcement before we have imported the associated relay chain block
/// which can happen on slow nodes or nodes with a slow network connection.
#[test]
fn relay_parent_not_imported_when_block_announce_is_processed() {
	block_on(async move {
		let (mut validator, api) = make_validator_and_api();

		let mut client = api.relay_client.clone();
		let block = client.init_polkadot_block_builder().build().expect("Build new block").block;

		let (signal, header) = make_gossip_message_and_header(api, block.hash(), 0).await;

		let data = BlockAnnounceData::try_from(&signal).unwrap().encode();

		let mut validation = validator.validate(&header, &data);

		// The relay chain block is not available yet, so the first poll should return
		// that the future is still pending.
		assert!(poll!(&mut validation).is_pending());

		client.import(BlockOrigin::Own, block).await.expect("Imports the block");

		assert!(matches!(
			poll!(validation),
			Poll::Ready(Ok(Validation::Success { is_new_best: true }))
		));
	});
}

/// Ensures that when we receive a block announcement without a statement included, while the block
/// is not yet included by the node checking the announcement, but the node is already backed.
#[test]
fn block_announced_without_statement_and_block_only_backed() {
	block_on(async move {
		let (mut validator, api) = make_validator_and_api();
		api.data.lock().has_pending_availability = true;

		let header = default_header();

		let validation = validator.validate(&header, &[]);

		assert!(matches!(validation.await, Ok(Validation::Success { is_new_best: true })));
	});
}
*/