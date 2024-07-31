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

#![allow(clippy::await_holding_lock)]

use cumulus_client_consensus_common::ParachainBlockImportMarker;
// This tests have been greatly influenced by
// https://github.com/paritytech/substrate/blob/master/client/consensus/aura/src/lib.rs#L832
// Most of the items hereby added are intended to make it work with our current consensus mechanism
use crate::collators::ClaimMode;
use tp_traits::SlotFrequency;
use {
    crate::{
        collators::{tanssi_claim_slot, Collator, Params as CollatorParams, lookahead::Params as LookAheadParams},
        OrchestratorAuraWorkerAuxData,
    },
    async_trait::async_trait,
    cumulus_client_collator::service::CollatorService,
    cumulus_client_consensus_common::ValidationCodeHashProvider,
    cumulus_client_consensus_proposer::Proposer as ConsensusProposer,
    cumulus_primitives_core::{relay_chain::BlockId, CollationInfo, CollectCollationInfo, ParaId},
    cumulus_relay_chain_interface::{
        CommittedCandidateReceipt, OverseerHandle, RelayChainInterface, RelayChainResult,
        StorageValue,
    },
    polkadot_node_subsystem_test_helpers::TestSubsystemSender,
    cumulus_test_relay_sproof_builder::RelayStateSproofBuilder,
    futures::prelude::*,
    nimbus_primitives::{
        CompatibleDigestItem, NimbusId, NimbusPair, NIMBUS_ENGINE_ID, NIMBUS_KEY_ID,
    },
    parity_scale_codec::Encode,
    parking_lot::Mutex,
    polkadot_core_primitives::{Header as PHeader, InboundDownwardMessage, InboundHrmpMessage},
    polkadot_parachain_primitives::primitives::HeadData,
    polkadot_primitives::{
        Hash as PHash, OccupiedCoreAssumption, PersistedValidationData, ValidatorId,
    },
    sc_block_builder::BlockBuilderBuilder,
    sc_client_api::HeaderBackend,
    sc_consensus::{BoxJustificationImport, ForkChoiceStrategy},
    sc_keystore::LocalKeystore,
    sc_network_test::{Block as TestBlock, Header as TestHeader, *},
    sp_api::{ApiRef, ProvideRuntimeApi},
    sp_consensus::{EnableProofRecording, Environment, Proposal, Proposer},
    sp_consensus_aura::{inherents::InherentDataProvider, SlotDuration, AURA_ENGINE_ID},
    sp_consensus_slots::Slot,
    sp_core::{
        crypto::{ByteArray, Pair},
        traits::SpawnNamed,
    },
    sp_inherents::InherentData,
    sp_keyring::sr25519::Keyring,
    sp_keystore::{Keystore, KeystorePtr},
    sp_runtime::{
        traits::{Block as BlockT, Header as _},
        Digest, DigestItem,
    },
    sp_timestamp::Timestamp,
    std::{
        collections::{BTreeMap, BTreeSet},
        pin::Pin,
        sync::Arc,
        time::Duration,
    },
    substrate_test_runtime_client::TestClient,
    polkadot_overseer::dummy::dummy_overseer_builder,
    sp_consensus::NoNetwork as DummyOracle,
    substrate_test_runtime_client::TestClientBuilder,
};
use polkadot_node_subsystem::messages::{CollationGenerationMessage, RuntimeApiMessage, RuntimeApiRequest};

// Duration of slot time
const SLOT_DURATION_MS: u64 = 1000;

type Error = sp_blockchain::Error;
type VirtualOverseer =
	polkadot_node_subsystem_test_helpers::TestSubsystemContextHandle<RuntimeApiMessage>;

#[derive(Clone)]
struct DummyFactory(Arc<TestClient>);
// We are going to create API because we need this to test runtime apis
// We use the client normally, but for testing certain runtime-api calls,
// we basically mock the runtime-api calls
impl ProvideRuntimeApi<Block> for DummyFactory {
    type Api = MockApi;

    fn runtime_api(&self) -> ApiRef<'_, Self::Api> {
        MockApi.into()
    }
}

struct MockApi;

// This is our MockAPi impl. We need these to test first_eligible_key
sp_api::mock_impl_runtime_apis! {
    impl dp_consensus::TanssiAuthorityAssignmentApi<Block, NimbusId> for MockApi {
        /// Return the current authorities assigned to a given paraId
        fn para_id_authorities(para_id: ParaId) -> Option<Vec<NimbusId>> {
            // We always return Alice if paraId is 1000
            if para_id == 1000u32.into() {
                Some(vec![Keyring::Alice.public().into()])
            }
            else {
                None
            }
        }
        /// Return the paraId assigned to a given authority
        fn check_para_id_assignment(authority: NimbusId) -> Option<ParaId> {
            if authority == Keyring::Alice.public().into() {
                Some(1000u32.into())
            }
            else {
                None
            }
        }
    }

    impl CollectCollationInfo<Block> for MockApi {
        fn collect_collation_info(_header: &<Block as BlockT>::Header) -> CollationInfo {
            CollationInfo {
                upward_messages: Vec::new(),
                horizontal_messages: Vec::new(),
                new_validation_code: None,
                processed_downward_messages: 0u32,
                hrmp_watermark: 0u32,
                head_data: HeadData(vec![1, 2, 3])
            }
        }

    }

    impl async_backing_primitives::UnincludedSegmentApi<Block> for MockApi {
        fn can_build_upon(
            included_hash: <Block as BlockT>::Hash,
            slot: async_backing_primitives::Slot,
        ) -> bool {
            true
        }
    }

     impl TaggedTransactionQueue::<Block> for MockApi {
        fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
			block_hash: <Block as BlockT>::Hash,
		) -> TransactionValidity {todo!()}

    }

    impl pallet_xcm_core_buyer_runtime_api::XCMCoreBuyerApi<Block, BlockNumber, ParaId, NimbusId> for MockApi {
        fn is_core_buying_allowed(para_id: ParaId, collator_public_key: NimbusId) -> Result<(), BuyingError<BlockNumber>> {
            todo!();
        }

        fn create_buy_core_unsigned_extrinsic(para_id: ParaId, proof: BuyCoreCollatorProof<NimbusId>) -> Box<<Block as BlockT>::Extrinsic> {
            todo!();
        }

        fn get_buy_core_signature_nonce(para_id: ParaId) -> u64 {
            todo!();
        }

        fn get_buy_core_slot_drift() -> Slot {
            todo!();
        }
    }

}
#[derive(Clone)]
struct RelayChain;

#[async_trait]
impl RelayChainInterface for RelayChain {
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
        let downward_msg = InboundDownwardMessage {
            sent_at: 10u32,
            msg: vec![1u8, 2u8, 3u8],
        };
        Ok(vec![downward_msg])
    }

    async fn retrieve_all_inbound_hrmp_channel_contents(
        &self,
        _: ParaId,
        _: PHash,
    ) -> RelayChainResult<BTreeMap<ParaId, Vec<InboundHrmpMessage>>> {
        let mut tree = BTreeMap::new();
        let hrmp_msg = InboundHrmpMessage {
            sent_at: 10u32,
            data: vec![1u8, 2u8, 3u8],
        };
        let para_id = ParaId::from(2000u32);
        tree.insert(para_id, vec![hrmp_msg]);
        Ok(tree)
    }

    async fn persisted_validation_data(
        &self,
        _hash: PHash,
        _: ParaId,
        assumption: OccupiedCoreAssumption,
    ) -> RelayChainResult<Option<PersistedValidationData>> {

        let included_persisted = PersistedValidationData {
            /// The parent head-data.
            parent_head: Default::default(),
            /// The relay-chain block number this is in the context of.
            relay_parent_number: 0,
            /// The relay-chain block storage root this is in the context of.
            relay_parent_storage_root: Default::default(),
            /// The maximum legal size of a POV block, in bytes.
            max_pov_size: 5_000_000u32,
        };

        let non_included_persisted = PersistedValidationData {
            parent_head: PHeader {
                parent_hash: Default::default(),
                number: 1u32,
                /// The state trie merkle root
                state_root: Default::default(),
                /// The merkle root of the extrinsics.
                extrinsics_root: Default::default(),
                /// A chain-specific digest of data useful for light clients or referencing auxiliary data.
                digest: Default::default()
            }.encode().into(),
            /// The relay-chain block number this is in the context of.
            relay_parent_number: 1,
            /// The relay-chain block storage root this is in the context of.
            relay_parent_storage_root: Default::default(),
            /// The maximum legal size of a POV block, in bytes.
            max_pov_size: 5_000_000u32,
        };

        if assumption == OccupiedCoreAssumption::Included {
            Ok(Some(included_persisted))
        }
        else {
            Ok(Some(non_included_persisted))
        }
    }

    async fn candidate_pending_availability(
        &self,
        _: PHash,
        _: ParaId,
    ) -> RelayChainResult<Option<CommittedCandidateReceipt>> {
        unimplemented!("Not needed for test")
    }

    async fn session_index_for_child(&self, _: PHash) -> RelayChainResult<u32> {
        Ok(0)
    }

    async fn import_notification_stream(
        &self,
    ) -> RelayChainResult<Pin<Box<dyn Stream<Item = PHeader> + Send>>> {
        Ok(Box::pin(futures::stream::iter([PHeader {
            parent_hash: Default::default(),
            number: 1u32,
            /// The state trie merkle root
            state_root: Default::default(),
            /// The merkle root of the extrinsics.
            extrinsics_root: Default::default(),
            /// A chain-specific digest of data useful for light clients or referencing auxiliary data.
            digest: Default::default()
        }])))
    }

    async fn finality_notification_stream(
        &self,
    ) -> RelayChainResult<Pin<Box<dyn Stream<Item = PHeader> + Send>>> {
        unimplemented!("Not needed for test")
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
        Ok(None)
    }

    async fn prove_read(
        &self,
        _: PHash,
        _: &Vec<Vec<u8>>,
    ) -> RelayChainResult<sc_client_api::StorageProof> {
        let mut tree = BTreeSet::new();
        tree.insert(vec![1u8, 2u8, 3u8]);
        let proof = sc_client_api::StorageProof::new(tree);
        Ok(proof)
    }

    async fn wait_for_block(&self, _: PHash) -> RelayChainResult<()> {
        Ok(())
    }

    async fn new_best_notification_stream(
        &self,
    ) -> RelayChainResult<Pin<Box<dyn Stream<Item = PHeader> + Send>>> {
        unimplemented!("Not needed for test")
    }

    async fn header(&self, _block_id: BlockId) -> RelayChainResult<Option<PHeader>> {
        Ok(Some(PHeader {
            parent_hash: Default::default(),
            number: 1u32,
            /// The state trie merkle root
            state_root: Default::default(),
            /// The merkle root of the extrinsics.
            extrinsics_root: Default::default(),
            /// A chain-specific digest of data useful for light clients or referencing auxiliary data.
            digest: Default::default()
        }))
    }

    async fn validation_code_hash(
        &self,
        _relay_parent: PHash,
        _para_id: ParaId,
        _occupied_core_assumption: OccupiedCoreAssumption,
    ) -> RelayChainResult<Option<polkadot_primitives::ValidationCodeHash>> {
        unimplemented!("Not needed for test")
    }
}

#[derive(Clone)]
struct DummySpawner;
impl SpawnNamed for DummySpawner {
    fn spawn_blocking(
        &self,
        _name: &'static str,
        _group: Option<&'static str>,
        _future: futures::future::BoxFuture<'static, ()>,
    ) {
    }

    fn spawn(
        &self,
        _name: &'static str,
        _group: Option<&'static str>,
        _future: futures::future::BoxFuture<'static, ()>,
    ) {
    }
}

struct DummyProposer(Arc<TestClient>);

// This is going to be our block verifier
// It will mimic what the Nimbus verifier does, but again, Nimbus verifier is non-public
// It should substract the seal from logs and put it in post_logs
#[derive(Clone)]
pub struct SealExtractorVerfier {
    finalized: bool,
}

impl SealExtractorVerfier {
    /// Create a new instance.
    ///
    /// Every verified block will use `finalized` for the `BlockImportParams`.
    pub fn new(finalized: bool) -> Self {
        Self { finalized }
    }
}

#[async_trait::async_trait]
impl<B: BlockT> sc_consensus::Verifier<B> for SealExtractorVerfier {
    async fn verify(
        &mut self,
        mut block: sc_consensus::BlockImportParams<B>,
    ) -> Result<sc_consensus::BlockImportParams<B>, String> {
        if block.fork_choice.is_none() {
            block.fork_choice = Some(ForkChoiceStrategy::LongestChain);
        };
        //TODO: this could be done by making the nimbus verifier public (it is not)

        // Grab the seal digest. Assume it is last (since it is a seal after-all).
        let seal = block
            .header
            .digest_mut()
            .pop()
            .ok_or("Block should have at least one digest on it")?;

        let signature = seal
            .as_nimbus_seal()
            .ok_or_else(|| String::from("HeaderUnsealed"))?;

        // Grab the author information from either the preruntime digest or the consensus digest
        //TODO use the trait
        let claimed_author = block
            .header
            .digest()
            .logs
            .iter()
            .find_map(|digest| match *digest {
                DigestItem::Consensus(id, ref author_id) if id == NIMBUS_ENGINE_ID => {
                    Some(author_id.clone())
                }
                DigestItem::PreRuntime(id, ref author_id) if id == NIMBUS_ENGINE_ID => {
                    Some(author_id.clone())
                }
                _ => None,
            })
            .ok_or("Expected one consensus or pre-runtime digest that contains author id bytes")?;

        // Verify the signature
        let valid_signature = NimbusPair::verify(
            &signature,
            block.header.hash(),
            &NimbusId::from_slice(&claimed_author)
                .map_err(|_| "Invalid Nimbus ID (wrong length)")?,
        );

        if !valid_signature {
            return Err("Block signature invalid".into());
        }
        block.post_digests.push(seal);

        block.finalized = self.finalized;
        Ok(block)
    }
}

use cumulus_primitives_core::relay_chain::ValidationCodeHash;
use polkadot_node_subsystem::{overseer, OverseerSignal, SpawnedSubsystem, SubsystemError};

struct DummyCodeHashProvider;
impl ValidationCodeHashProvider<PHash> for DummyCodeHashProvider {
    fn code_hash_at(&self, at: PHash) -> Option<ValidationCodeHash> {
        Some(PHash::default().into())
    }
}

// The test Environment
impl Environment<TestBlock> for DummyFactory {
    type Proposer = DummyProposer;
    type CreateProposer = future::Ready<Result<DummyProposer, Error>>;
    type Error = Error;

    fn init(&mut self, _parent_header: &<TestBlock as BlockT>::Header) -> Self::CreateProposer {
        future::ready(Ok(DummyProposer(self.0.clone())))
    }
}

impl HeaderMetadata<Block> for DummyFactory {
    type Error = <Client<Backend> as HeaderMetadata<Block>>::Error;

    fn header_metadata(&self, hash: Hash) -> Result<CachedHeaderMetadata<Block>, Self::Error> {
        self.0.header_metadata(hash)
    }

    fn insert_header_metadata(&self, _hash: Hash, _header_metadata: CachedHeaderMetadata<Block>) {
        todo!()
    }

    fn remove_header_metadata(&self, _hash: Hash) {
        todo!()
    }
}

impl HeaderBackend<Block> for DummyFactory {
    fn header(&self, hash: Hash) -> sc_client_api::blockchain::Result<Option<Header>> {
        self.0.header(hash)
    }

    fn info(&self) -> Info<Block> {
        self.0.info()
    }

    fn status(&self, hash: Hash) -> sc_client_api::blockchain::Result<BlockStatus> {
        self.0.status(hash)
    }

    fn number(&self, hash: Hash) -> sc_client_api::blockchain::Result<Option<BlockNumber>> {
        self.0.number(hash)
    }

    fn hash(&self, number: BlockNumber) -> sc_client_api::blockchain::Result<Option<Hash>> {
        self.0.hash(number)
    }
}

impl BlockchainEvents<Block> for DummyFactory {
    fn import_notification_stream(&self) -> ImportNotifications<Block> {
        unimplemented!()
    }

    fn every_import_notification_stream(&self) -> ImportNotifications<Block> {
        unimplemented!()
    }

    fn finality_notification_stream(&self) -> FinalityNotifications<Block> {
        self.0.finality_notification_stream()
    }

    fn storage_changes_notification_stream(
        &self,
        _filter_keys: Option<&[StorageKey]>,
        _child_filter_keys: Option<&[(StorageKey, Option<Vec<StorageKey>>)]>,
    ) -> sc_client_api::blockchain::Result<StorageEventStream<Hash>> {
        unimplemented!()
    }
}

impl BlockOf for DummyFactory {
    type Type = <TestClient as BlockOf>::Type;
}

impl AuxStore for DummyFactory {
    fn insert_aux<
        'a,
        'b: 'a,
        'c: 'a,
        I: IntoIterator<Item = &'a (&'c [u8], &'c [u8])>,
        D: IntoIterator<Item = &'a &'b [u8]>,
    >(
        &self,
        insert: I,
        delete: D,
    ) -> sp_blockchain::Result<()> {
        self.0.insert_aux(insert, delete)
    }

    fn get_aux(&self, key: &[u8]) -> sp_blockchain::Result<Option<Vec<u8>>> {
       self.0.get_aux(key)
    }
}
impl BlockBackend<Block> for DummyFactory {
    fn block_body(
        &self,
        hash:<Block as BlockT>::Hash,
    ) -> sp_blockchain::Result<Option<Vec<<Block as BlockT>::Extrinsic>>>{
        self.0.block_body(hash)
    }
    fn block_indexed_body(&self, hash:<Block as BlockT>::Hash) -> sp_blockchain::Result<Option<Vec<Vec<u8>>>> {
        self.0.block_indexed_body(hash)
    }
    fn block(&self, hash:<Block as BlockT>::Hash) -> sp_blockchain::Result<Option<SignedBlock<Block>>> {
        self.0.block(hash)
    }
    fn block_status(&self, hash:<Block as BlockT>::Hash) -> sp_blockchain::Result<sp_consensus::BlockStatus> {
        self.0.block_status(hash)
    }
    fn justifications(&self, hash:<Block as BlockT>::Hash) -> sp_blockchain::Result<Option<Justifications>> {
        self.0.justifications(hash)
    }
    fn block_hash(&self, number: NumberFor<Block>) -> sp_blockchain::Result<Option<<Block as BlockT>::Hash>> {
        self.0.block_hash(number)
    }
    fn indexed_transaction(&self, hash:<Block as BlockT>::Hash) -> sp_blockchain::Result<Option<Vec<u8>>> {
        self.0.indexed_transaction(hash)
    }
    fn has_indexed_transaction(&self, hash:<Block as BlockT>::Hash) -> sp_blockchain::Result<bool> {
       self.0.has_indexed_transaction(hash)
    }
    fn requires_full_sync(&self) -> bool {
        self.0.requires_full_sync()
    }
}
impl BlockIdTo<Block> for DummyFactory {
    type Error = <TestClient as BlockIdTo<Block>>::Error;
    /// Convert the given `block_id` to the corresponding block hash.
    fn to_hash(
        &self,
        block_id: &sp_runtime::generic::BlockId<Block>,
    ) -> Result<Option<<Block as BlockT>::Hash>, Self::Error> {
        self.0.to_hash(block_id)
    }

    /// Convert the given `block_id` to the corresponding block number.
    fn to_number(
        &self,
        block_id: &sp_runtime::generic::BlockId<Block>,
    ) -> Result<Option<NumberFor<Block>>, Self::Error> {
        self.0.to_number(block_id)
    }
}
impl ParachainBlockImportMarker for DummyFactory {}

// how to propose the block by Dummy Proposer
impl Proposer<TestBlock> for DummyProposer {
    type Error = Error;
    type Proposal = future::Ready<Result<Proposal<TestBlock, Self::Proof>, Error>>;
    type ProofRecording = EnableProofRecording;
    type Proof = sc_client_api::StorageProof;

    fn propose(
        self,
        _: InherentData,
        digests: Digest,
        _: Duration,
        _: Option<usize>,
    ) -> Self::Proposal {
        let r = BlockBuilderBuilder::new(&*self.0)
            .on_parent_block(self.0.chain_info().best_hash)
            .fetch_parent_block_number(&*self.0)
            .unwrap()
            .with_inherent_digests(digests)
            .build()
            .unwrap()
            .build();
        let (_relay_parent_storage_root, proof) =
            RelayStateSproofBuilder::default().into_state_root_and_proof();

        futures::future::ready(r.map(|b| Proposal {
            block: b.block,
            proof,
            storage_changes: b.storage_changes,
        }))
    }
}

type AuraPeer = Peer<(), PeersClient>;

#[derive(Default)]
pub struct AuraTestNet {
    peers: Vec<AuraPeer>,
}

impl TestNetFactory for AuraTestNet {
    type Verifier = SealExtractorVerfier;
    type PeerData = ();
    type BlockImport = PeersClient;

    fn make_block_import(
        &self,
        client: PeersClient,
    ) -> (
        BlockImportAdapter<Self::BlockImport>,
        Option<BoxJustificationImport<Block>>,
        Self::PeerData,
    ) {
        ((client.as_block_import()), None, ())
    }

    fn make_verifier(&self, _client: PeersClient, _peer_data: &()) -> Self::Verifier {
        SealExtractorVerfier::new(true)
    }

    fn peer(&mut self, i: usize) -> &mut AuraPeer {
        &mut self.peers[i]
    }

    fn peers(&self) -> &Vec<AuraPeer> {
        &self.peers
    }

    fn peers_mut(&mut self) -> &mut Vec<AuraPeer> {
        &mut self.peers
    }

    fn mut_peers<F: FnOnce(&mut Vec<AuraPeer>)>(&mut self, closure: F) {
        closure(&mut self.peers);
    }
}

// Checks node slot claim. Again for different slots, different authorities
// should be able to claim
#[tokio::test]
async fn current_node_authority_should_claim_slot() {
    let mut authorities: Vec<NimbusId> = vec![
        Keyring::Alice.public().into(),
        Keyring::Bob.public().into(),
        Keyring::Charlie.public().into(),
    ];

    let keystore_path = tempfile::tempdir().expect("Creates keystore path");
    let keystore = LocalKeystore::open(keystore_path.path(), None).expect("Creates keystore.");

    let public = keystore
        .sr25519_generate_new(NIMBUS_KEY_ID, None)
        .expect("Key should be created");
    authorities.push(public.into());

    let keystore_ptr: KeystorePtr = keystore.into();
    let mut claimed_slots = vec![];

    for slot in 0..8 {
        let dummy_head = TestHeader {
            parent_hash: Default::default(),
            number: Default::default(),
            state_root: Default::default(),
            extrinsics_root: Default::default(),
            digest: Default::default(),
        };
        let aux_data = OrchestratorAuraWorkerAuxData {
            authorities: authorities.clone(),
            slot_freq: None,
        };
        let claim = tanssi_claim_slot::<NimbusPair, TestBlock>(
            aux_data,
            &dummy_head,
            slot.into(),
            ClaimMode::NormalAuthoring,
            &keystore_ptr,
        );

        if claim.is_some() {
            claimed_slots.push(slot);
        }
    }

    assert_eq!(claimed_slots, vec![3, 7]);
}

#[tokio::test]
async fn claim_slot_respects_min_slot_freq() {
    // There is only 1 authority, but it can only claim every 4 slots
    let mut authorities: Vec<NimbusId> = vec![];
    let min_slot_freq = 4u32;

    let keystore_path = tempfile::tempdir().expect("Creates keystore path");
    let keystore = LocalKeystore::open(keystore_path.path(), None).expect("Creates keystore.");

    let public = keystore
        .sr25519_generate_new(NIMBUS_KEY_ID, None)
        .expect("Key should be created");
    authorities.push(public.into());

    let keystore_ptr: KeystorePtr = keystore.into();

    let mut claimed_slots = vec![];

    for slot in 0..10 {
        let parent_slot: u64 = claimed_slots.last().copied().unwrap_or_default();
        let parent_slot: Slot = parent_slot.into();
        let pre_digest = Digest {
            logs: vec![
                DigestItem::PreRuntime(AURA_ENGINE_ID, parent_slot.encode()),
                //DigestItem::PreRuntime(NIMBUS_ENGINE_ID, authority.encode()),
            ],
        };
        let head = TestHeader {
            parent_hash: Default::default(),
            // If we use number=0 aura ignores the digest
            number: claimed_slots.len() as u64,
            state_root: Default::default(),
            extrinsics_root: Default::default(),
            digest: pre_digest,
        };
        let aux_data = OrchestratorAuraWorkerAuxData {
            authorities: authorities.clone(),
            slot_freq: Some(SlotFrequency {
                min: min_slot_freq,
                max: 0u32,
            }),
        };
        let claim = tanssi_claim_slot::<NimbusPair, TestBlock>(
            aux_data,
            &head,
            slot.into(),
            ClaimMode::NormalAuthoring,
            &keystore_ptr,
        );

        if claim.is_some() {
            claimed_slots.push(slot);
        }
    }

    assert_eq!(claimed_slots, vec![0, 4, 8]);
}

#[tokio::test]
async fn collate_returns_correct_block() {
    let net = AuraTestNet::new(4);

    let keystore_path = tempfile::tempdir().expect("Creates keystore path");
    let keystore = LocalKeystore::open(keystore_path.path(), None).expect("Creates keystore.");
    let alice_public = keystore
        .sr25519_generate_new(NIMBUS_KEY_ID, Some(&Keyring::Alice.to_seed()))
        .expect("Key should be created");

    // Copy of the keystore needed for tanssi_claim_slot()
    let keystore_copy = LocalKeystore::open(keystore_path.path(), None).expect("Copies keystore.");
    keystore_copy
        .sr25519_generate_new(NIMBUS_KEY_ID, Some(&Keyring::Alice.to_seed()))
        .expect("Key should be copied");

    let net = Arc::new(Mutex::new(net));

    let mut net = net.lock();
    let peer = net.peer(3);
    let client = peer.client().as_client();
    let environ = DummyFactory(client.clone());
    let spawner = DummySpawner;
    let relay_client = RelayChain;
    let orchestrator_client = RelayChain;

    // Build the collator
    let mut collator = {
        let params = CollatorParams {
            create_inherent_data_providers: |_, _| async {
                let slot = InherentDataProvider::from_timestamp_and_slot_duration(
                    Timestamp::current(),
                    SlotDuration::from_millis(SLOT_DURATION_MS),
                );

                Ok((slot,))
            },
            block_import: client.clone(),
            relay_client: relay_client.clone(),
            keystore: keystore.into(),
            para_id: 1000.into(),
            proposer: ConsensusProposer::new(environ.clone()),
            collator_service: CollatorService::new(
                client.clone(),
                Arc::new(spawner),
                Arc::new(move |_, _| {}),
                Arc::new(environ),
            ),
        };

        Collator::<Block, NimbusPair, _, _, _, _, _>::new(params)
    };

    let mut head = client.expect_header(client.info().genesis_hash).unwrap();

    // Modify the state root of the genesis header for it to match
    // the one inside propose() function
    let (relay_parent_storage_root, _proof) =
        RelayStateSproofBuilder::default().into_state_root_and_proof();
    head.state_root = relay_parent_storage_root;

    // First we create inherent data
    let (parachain_inherent_data, other_inherent_data) = collator
        .create_inherent_data(
            Default::default(),
            &Default::default(),
            head.clone().hash(),
            None,
        )
        .await
        .unwrap();

    // Params for tanssi_claim_slot()
    let slot = InherentDataProvider::from_timestamp_and_slot_duration(
        Timestamp::current(),
        SlotDuration::from_millis(SLOT_DURATION_MS),
    );
    let keystore_ptr: KeystorePtr = keystore_copy.into();

    let mut claim = tanssi_claim_slot::<NimbusPair, TestBlock>(
        OrchestratorAuraWorkerAuxData {
            authorities: vec![alice_public.into()],
            slot_freq: None,
        },
        &head,
        *slot,
        ClaimMode::NormalAuthoring,
        &keystore_ptr,
    )
    .unwrap();

    // At the end we call collate() function
    let res = collator
        .collate(
            &head,
            &mut claim,
            None,
            (parachain_inherent_data, other_inherent_data),
            Duration::from_millis(500),
            3_500_000usize,
        )
        .await
        .unwrap()
        .unwrap()
        .1;

    // The returned block should be imported and we should be able to get its header by now.
    assert!(client.header(res.header().hash()).unwrap().is_some());
}

// Tests authorities are correctly returned and eligibility is correctly calculated
// thanks to the mocked runtime-apis
#[tokio::test]
async fn authorities_runtime_api_tests() {
    let net = AuraTestNet::new(4);
    let net = Arc::new(Mutex::new(net));

    let mut net = net.lock();
    let peer = net.peer(3);
    let client = peer.client().as_client();
    let environ = DummyFactory(client);

    let default_hash = Default::default();

    let authorities = crate::authorities::<_, _, nimbus_primitives::NimbusPair>(
        &environ,
        &default_hash,
        1000u32.into(),
    );

    assert_eq!(authorities, Some(vec![Keyring::Alice.public().into()]));
}

/// Helper function to generate a crypto pair from seed
fn get_aura_id_from_seed(seed: &str) -> NimbusId {
    sp_core::sr25519::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
        .into()
}

/// Helper function to generate a crypto pair from seed
fn get_pair_from_seed(seed: &str) -> sp_core::sr25519::Pair {
    sp_core::sr25519::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
}

struct MockSupportsParachains;

#[async_trait]
impl polkadot_overseer::HeadSupportsParachains for MockSupportsParachains {
    async fn head_supports_parachains(&self, _head: &Hash) -> bool {
        true
    }
}


use polkadot_primitives::{CollatorPair, CoreState, ScheduledCore};
use sc_client_api::{AuxStore, BlockBackend, BlockchainEvents, BlockOf, FinalityNotifications, ImportNotifications, StorageEventStream, StorageKey};
use sp_blockchain::{BlockStatus, CachedHeaderMetadata, HeaderMetadata, Info};
use sp_runtime::generic::SignedBlock;
use sp_runtime::Justifications;
use sp_runtime::traits::{BlockIdTo, NumberFor};
use sp_runtime::transaction_validity::{TransactionSource, TransactionValidity};
use sp_transaction_pool::runtime_api::TaggedTransactionQueue;
use substrate_test_runtime_client::{Backend, Client};
use substrate_test_runtime_client::runtime::BlockNumber;
use pallet_xcm_core_buyer_runtime_api::BuyingError;
use tp_xcm_core_buyer::BuyCoreCollatorProof;

/// A mocked `runtime-api` subsystem.
#[derive(Clone)]
pub struct MockRuntimeApi(ParaId);

#[overseer::subsystem(RuntimeApi, error=polkadot_node_subsystem::SubsystemError, prefix=self::overseer)]
impl<Context> MockRuntimeApi {
    fn start(self, ctx: Context) -> polkadot_node_subsystem::SpawnedSubsystem {
        let future = self.run(ctx).map(|_| Ok(())).boxed();

        polkadot_node_subsystem::SpawnedSubsystem { name: "test-environment", future }
    }
}

#[overseer::contextbounds(RuntimeApi, prefix = self::overseer)]
impl MockRuntimeApi {
    fn new(para_id: ParaId) -> Self {
        Self(para_id)
    }
    async fn run<Context>(self, mut ctx: Context) {

        loop {
            let msg = polkadot_overseer::SubsystemContext::recv(&mut ctx).await.expect("Overseer never fails us");

            match msg {
                orchestra::FromOrchestra::Signal(signal) =>
                    if signal == OverseerSignal::Conclude {
                        return
                    },
                orchestra::FromOrchestra::Communication { msg } => {

                    match msg {

                        RuntimeApiMessage::Request(
                            block_hash,
                            RuntimeApiRequest::AvailabilityCores(sender),
                        ) => {
                            let _ = sender.send(Ok(vec![CoreState::Scheduled(ScheduledCore {
                                para_id: self.0,
                                collator: None,
                            })]));
                        },
                        // Long term TODO: implement more as needed.
                        message => {
                            unimplemented!("Unexpected runtime-api message: {:?}", message)
                        },
                    }
                },
            }
        }
    }
}

#[tokio::test]
async fn collate_lookahead_returns_correct_block() {
    use tokio_util::sync::CancellationToken;
    use substrate_test_runtime_client::DefaultTestClientBuilderExt;
    let net = AuraTestNet::new(4);

    let keystore_path = tempfile::tempdir().expect("Creates keystore path");
    let keystore = LocalKeystore::open(keystore_path.path(), None).expect("Creates keystore.");
    let alice_public = keystore
        .sr25519_generate_new(NIMBUS_KEY_ID, Some(&Keyring::Alice.to_seed()))
        .expect("Key should be created");

    // Copy of the keystore needed for tanssi_claim_slot()
    let keystore_copy = LocalKeystore::open(keystore_path.path(), None).expect("Copies keystore.");
    keystore_copy
        .sr25519_generate_new(NIMBUS_KEY_ID, Some(&Keyring::Alice.to_seed()))
        .expect("Key should be copied");

    let net = Arc::new(Mutex::new(net));

    let mut net = net.lock();
    let peer = net.peer(3);
    let builder = TestClientBuilder::new();
    let backend = builder.backend();
    let client = peer.client().as_client();
    let environ = DummyFactory(client.clone());
    let spawner = DummySpawner;
    let relay_client = RelayChain;
    let orchestrator_client = RelayChain;
	let spawner = sp_core::testing::TaskExecutor::new();
    let orchestrator_tx_pool = sc_transaction_pool::BasicPool::new_full(
        Default::default(),
        true.into(),
        None,
        spawner.clone(),
        client.clone(),
    );
    let mock_runtime_api = MockRuntimeApi::new(1000u32.into());

    let (overseer, handle) =
    dummy_overseer_builder(spawner.clone(), MockSupportsParachains, None)
        .unwrap()
        .replace_runtime_api(|_|mock_runtime_api)
        .build()
        .unwrap();
	spawner.spawn("overseer", None, overseer.run().then(|_| async {}).boxed());

    // Build the collator
    let params = LookAheadParams {
            create_inherent_data_providers: move |_block_hash, _|  async move {
                let slot = InherentDataProvider::from_timestamp_and_slot_duration(
                    Timestamp::current(),
                    SlotDuration::from_millis(SLOT_DURATION_MS),
                );

                Ok((slot,))
            },
            block_import: client.clone(),
            relay_client: relay_client.clone(),
            keystore: keystore.into(),
            para_id: 1000.into(),
            proposer: ConsensusProposer::new(environ.clone()),
            collator_service: CollatorService::new(
                client.clone(),
                Arc::new(spawner.clone()),
                Arc::new(move |_, _| {}),
                Arc::new(environ.clone()),
            ),
            authoring_duration: Duration::from_millis(500),
            cancellation_token: CancellationToken::new(),
            code_hash_provider: DummyCodeHashProvider,
            collator_key: CollatorPair::generate().0,
            force_authoring: false,
            get_orchestrator_aux_data:  move |_block_hash, _extra | async move {
                let aux_data = OrchestratorAuraWorkerAuxData {
                    authorities: vec![],
                    // This is the orchestrator consensus, it does not have a slot frequency
                    slot_freq: None,
                };

               Ok(aux_data)
            },
            get_current_slot_duration: move |_block_hash| {
                SlotDuration::from_millis(6_000)
            },
			overseer_handle: OverseerHandle::new(handle),
            relay_chain_slot_duration: Duration::from_secs(6),
            para_client: environ.clone().into(),
            sync_oracle: DummyOracle,
            para_backend: backend,
            orchestrator_client: environ.into(),
            orchestrator_slot_duration: SlotDuration::from_millis(SLOT_DURATION_MS),
            orchestrator_tx_pool
        };

    let (fut, exit_notification_receiver) = crate::collators::lookahead::run::<_, Block, NimbusPair, _, _, _, _, _, _, _, _, _, _, _, _, _>(
            params,
    );

    spawner.spawn("tanssi-aura", None, Box::pin(fut));
    // The returned block should be imported and we should be able to get its header by now.
    //assert!(client.header(res.header().hash()).unwrap().is_some());
}