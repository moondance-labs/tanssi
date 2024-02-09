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

use std::borrow::Borrow;

// This tests have been greatly influenced by
// https://github.com/paritytech/substrate/blob/master/client/consensus/aura/src/lib.rs#L832
// Most of the items hereby added are intended to make it work with our current consensus mechanism
use {
    crate::{
        collators::{tanssi_claim_slot, Collator, Params as CollatorParams},
        InherentDataProviderExt, LOG_TARGET,
    },
    async_trait::async_trait,
    cumulus_client_collator::service::CollatorService,
    cumulus_client_consensus_common::ParachainConsensus,
    cumulus_client_consensus_proposer::Proposer as ConsensusProposer,
    cumulus_primitives_core::{relay_chain::BlockId, CollationInfo, CollectCollationInfo},
    cumulus_relay_chain_interface::{
        CommittedCandidateReceipt, OverseerHandle, RelayChainInterface, RelayChainResult,
        StorageValue,
    },
    futures::prelude::*,
    futures_timer::Delay,
    nimbus_primitives::{
        CompatibleDigestItem, NimbusId, NimbusPair, NIMBUS_ENGINE_ID, NIMBUS_KEY_ID,
    },
    parity_scale_codec::alloc::collections::{BTreeMap, BTreeSet},
    parking_lot::Mutex,
    polkadot_core_primitives::{Header as PHeader, InboundDownwardMessage, InboundHrmpMessage},
    polkadot_parachain_primitives::primitives::HeadData,
    polkadot_primitives::{
        Hash as PHash, OccupiedCoreAssumption, PersistedValidationData, ValidatorId,
    },
    sc_block_builder::BlockBuilderProvider,
    sc_client_api::HeaderBackend,
    sc_consensus::{BoxJustificationImport, ForkChoiceStrategy},
    sc_consensus_aura::SlotProportion,
    sc_consensus_slots::{BackoffAuthoringOnFinalizedHeadLagging, SlotInfo},
    sc_keystore::LocalKeystore,
    sc_network_test::{Block as TestBlock, *},
    sp_consensus::{
        EnableProofRecording, Environment, NoNetwork as DummyOracle, Proposal, Proposer,
        SelectChain, SyncOracle,
    },
    sp_consensus_aura::{inherents::InherentDataProvider, SlotDuration},
    sp_consensus_slots::Slot,
    sp_core::{
        crypto::{ByteArray, Pair},
        traits::SpawnNamed,
        H256,
    },
    sp_inherents::{CreateInherentDataProviders, InherentData},
    sp_keyring::sr25519::Keyring,
    sp_keystore::{Keystore, KeystorePtr},
    sp_runtime::{
        traits::{Block as BlockT, Header as _},
        Digest, DigestItem,
    },
    sp_timestamp::Timestamp,
    std::{pin::Pin, sync::Arc, time::Duration},
    substrate_test_runtime_client::TestClient,
};

// Duration of slot time
const SLOT_DURATION_MS: u64 = 1000;

type Error = sp_blockchain::Error;

#[derive(Clone)]
struct DummyFactory(Arc<TestClient>);
// We are going to create API because we need this to test runtime apis
// We use the client normally, but for testing certain runtime-api calls,
// we basically mock the runtime-api calls
use sp_api::{ApiRef, ProvideRuntimeApi};
impl ProvideRuntimeApi<Block> for DummyFactory {
    type Api = MockApi;

    fn runtime_api(&self) -> ApiRef<'_, Self::Api> {
        MockApi.into()
    }
}

use cumulus_primitives_core::ParaId;

struct MockApi;

// This is our MockAPi impl. We need these to test first_eligible_key
sp_api::mock_impl_runtime_apis! {
    impl tp_consensus::TanssiAuthorityAssignmentApi<Block, NimbusId> for MockApi {
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
        fn collect_collation_info(header: &<Block as BlockT>::Header) -> CollationInfo {
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
}

#[derive(Clone)]
struct RelayChain(Arc<TestClient>);

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
        let downward_msg = InboundDownwardMessage{
            sent_at: 10u32,
            msg: vec![1u8,2u8,3u8]
        };
        Ok(vec![downward_msg])
    }

    async fn retrieve_all_inbound_hrmp_channel_contents(
        &self,
        _: ParaId,
        _: PHash,
    ) -> RelayChainResult<BTreeMap<ParaId, Vec<InboundHrmpMessage>>> {
        let mut tree = BTreeMap::new();
        let hrmp_msg = InboundHrmpMessage{
            sent_at: 10u32,
            data: vec![1u8,2u8,3u8]
        };
        let para_id = ParaId::from(2000u32);
        tree.insert(para_id, vec![hrmp_msg]);
        Ok(tree)
    }

    async fn persisted_validation_data(
        &self,
        hash: PHash,
        _: ParaId,
        assumption: OccupiedCoreAssumption,
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

    async fn session_index_for_child(&self, _: PHash) -> RelayChainResult<u32> {
        Ok(0)
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

    async fn header(&self, block_id: BlockId) -> RelayChainResult<Option<PHeader>> {
        unimplemented!("Not needed for test")
    }
}

#[derive(Clone)]
struct DummySpawner(Arc<TestClient>);
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

struct DummyProposer(u64, Arc<TestClient>);

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

// The test Environment
impl Environment<TestBlock> for DummyFactory {
    type Proposer = DummyProposer;
    type CreateProposer = future::Ready<Result<DummyProposer, Error>>;
    type Error = Error;

    fn init(&mut self, parent_header: &<TestBlock as BlockT>::Header) -> Self::CreateProposer {
        future::ready(Ok(DummyProposer(parent_header.number + 1, self.0.clone())))
    }
}

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
        let r = self.1.new_block(digests).unwrap().build();

        //TODO: this is new and we should check if it's needed
        let mut tree = BTreeSet::new();
        tree.insert(vec![1u8, 3u8, 4u8]);
        let proof = sc_client_api::StorageProof::new(tree);

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

/// A stream that returns every time there is a new slot.
/// TODO: this would not be necessary if Slots was public in Substrate
pub(crate) struct Slots<Block, SC, IDP> {
    last_slot: Slot,
    slot_duration: Duration,
    until_next_slot: Option<Delay>,
    create_inherent_data_providers: IDP,
    select_chain: SC,
    _phantom: std::marker::PhantomData<Block>,
}

impl<Block, SC, IDP> Slots<Block, SC, IDP> {
    /// Create a new `Slots` stream.
    pub fn new(
        slot_duration: Duration,
        create_inherent_data_providers: IDP,
        select_chain: SC,
    ) -> Self {
        Slots {
            last_slot: 0.into(),
            slot_duration,
            until_next_slot: None,
            create_inherent_data_providers,
            select_chain,
            _phantom: Default::default(),
        }
    }
}

impl<Block, SC, IDP> Slots<Block, SC, IDP>
where
    Block: BlockT,
    SC: SelectChain<Block>,
    IDP: CreateInherentDataProviders<Block, ()> + 'static,
    IDP::InherentDataProviders: crate::InherentDataProviderExt,
{
    /// Returns a future that fires when the next slot starts.
    pub async fn next_slot(&mut self) -> SlotInfo<Block> {
        loop {
            // Wait for slot timeout
            self.until_next_slot
                .take()
                .unwrap_or_else(|| {
                    // Schedule first timeout.
                    let wait_dur = time_until_next_slot(self.slot_duration);
                    Delay::new(wait_dur)
                })
                .await;

            // Schedule delay for next slot.
            let wait_dur = time_until_next_slot(self.slot_duration);
            self.until_next_slot = Some(Delay::new(wait_dur));

            let chain_head = match self.select_chain.best_chain().await {
                Ok(x) => x,
                Err(e) => {
                    log::warn!(
                        target: LOG_TARGET,
                        "Unable to author block in slot. No best block header: {}",
                        e,
                    );
                    // Let's retry at the next slot.
                    continue;
                }
            };

            let inherent_data_providers = match self
                .create_inherent_data_providers
                .create_inherent_data_providers(chain_head.hash(), ())
                .await
            {
                Ok(x) => x,
                Err(e) => {
                    log::warn!(
                        target: LOG_TARGET,
                        "Unable to author block in slot. Failure creating inherent data provider: {}",
                        e,
                    );
                    // Let's retry at the next slot.
                    continue;
                }
            };

            let slot = inherent_data_providers.slot();

            // Never yield the same slot twice.
            if slot > self.last_slot {
                self.last_slot = slot;

                break SlotInfo::new(
                    slot,
                    Box::new(inherent_data_providers),
                    self.slot_duration,
                    chain_head,
                    None,
                );
            }
        }
    }
}
/// Returns current duration since unix epoch.
pub fn duration_now() -> Duration {
    use std::time::SystemTime;
    let now = SystemTime::now();
    now.duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_else(|e| {
            panic!(
                "Current time {:?} is before unix epoch. Something is wrong: {:?}",
                now, e
            )
        })
}

/// Returns the duration until the next slot from now.
pub fn time_until_next_slot(slot_duration: Duration) -> Duration {
    let now = duration_now().as_millis();

    let next_slot = (now + slot_duration.as_millis()) / slot_duration.as_millis();
    let remaining_millis = next_slot * slot_duration.as_millis() - now;
    Duration::from_millis(remaining_millis as u64)
}

/// Start a new slot worker.
///
/// Every time a new slot is triggered, `parachain_block_producer.produce_candidate`
/// is called and the future it returns is
/// polled until completion, unless we are major syncing.
/// TODO: refactor to use the new Tanssi Aura params
pub async fn start_orchestrator_aura_consensus_candidate_producer<B, C, SO, CIDP>(
    slot_duration: SlotDuration,
    client: C,
    mut parachain_block_producer: Box<dyn ParachainConsensus<B>>,
    sync_oracle: SO,
    create_inherent_data_providers: CIDP,
) where
    B: BlockT,
    C: SelectChain<B>,
    SO: SyncOracle + Send,
    CIDP: CreateInherentDataProviders<B, ()> + Send + 'static,
    CIDP::InherentDataProviders: InherentDataProviderExt + Send,
{
    let mut slots = Slots::new(
        slot_duration.as_duration(),
        create_inherent_data_providers,
        client,
    );

    loop {
        let slot_info = slots.next_slot().await;

        if sync_oracle.is_major_syncing() {
            continue;
        }

        let _ = parachain_block_producer
            .produce_candidate(
                &slot_info.chain_head,
                Default::default(),
                &Default::default(),
            )
            .await;
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

    assert!(
        tanssi_claim_slot::<NimbusPair>(authorities.clone(), 0.into(), false, &keystore_ptr)
            .unwrap()
            .is_none()
    );
    assert!(
        tanssi_claim_slot::<NimbusPair>(authorities.clone(), 1.into(), false, &keystore_ptr)
            .unwrap()
            .is_none()
    );
    assert!(
        tanssi_claim_slot::<NimbusPair>(authorities.clone(), 2.into(), false, &keystore_ptr)
            .unwrap()
            .is_none()
    );
    assert!(
        tanssi_claim_slot::<NimbusPair>(authorities.clone(), 3.into(), false, &keystore_ptr)
            .unwrap()
            .is_some()
    );
    assert!(
        tanssi_claim_slot::<NimbusPair>(authorities.clone(), 4.into(), false, &keystore_ptr)
            .unwrap()
            .is_none()
    );
    assert!(
        tanssi_claim_slot::<NimbusPair>(authorities.clone(), 5.into(), false, &keystore_ptr)
            .unwrap()
            .is_none()
    );
    assert!(
        tanssi_claim_slot::<NimbusPair>(authorities.clone(), 6.into(), false, &keystore_ptr)
            .unwrap()
            .is_none()
    );
    assert!(
        tanssi_claim_slot::<NimbusPair>(authorities.clone(), 7.into(), false, &keystore_ptr)
            .unwrap()
            .is_some()
    );
}

#[tokio::test]
async fn on_slot_returns_correct_block() {
    let net = AuraTestNet::new(4);

    let keystore_path = tempfile::tempdir().expect("Creates keystore path");
    let keystore = LocalKeystore::open(keystore_path.path(), None).expect("Creates keystore.");
    let alice_public = keystore
        .sr25519_generate_new(NIMBUS_KEY_ID, Some(&Keyring::Alice.to_seed()))
        .expect("Key should be created");
    
    let keystore_copy = LocalKeystore::open(keystore_path.path(), None).expect("Copies keystore.");
    keystore_copy
        .sr25519_generate_new(NIMBUS_KEY_ID, Some(&Keyring::Alice.to_seed()))
        .expect("Key should be copied");

    let net = Arc::new(Mutex::new(net));

    let mut net = net.lock();
    let peer = net.peer(3);
    let client = peer.client().as_client();
    let environ = DummyFactory(client.clone());
    let spawner = DummySpawner(client.clone());
    let relay_client = RelayChain(client.clone());

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

    let head = client.expect_header(client.info().genesis_hash).unwrap();

    let slot = InherentDataProvider::from_timestamp_and_slot_duration(
        Timestamp::current(),
        SlotDuration::from_millis(SLOT_DURATION_MS),
    );

    let (parachain_inherent_data, other_inherent_data) = collator.create_inherent_data(
        Default::default(),
        &Default::default(),
        head.clone().hash(),
        None,
    ).await.unwrap();

    let keystore_ptr: KeystorePtr = keystore_copy.into();

    let mut claim = tanssi_claim_slot::<NimbusPair>(vec![alice_public.into()], *slot, false, &keystore_ptr)
    .unwrap().unwrap();

    let res = collator.collate(
            &head,
            &mut claim,
            None,
            (parachain_inherent_data, other_inherent_data),
            Duration::from_millis(500),
            3_500_000 as usize,
        )
        .await.unwrap();

    println!("RES: {:#?}", res.0);

    // The returned block should be imported and we should be able to get its header by now.
    //assert!(client.header(res.block.hash()).unwrap().is_some());
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
    let environ = DummyFactory(client.clone());

    let default_hash = Default::default();

    let authorities = crate::authorities::<_, _, nimbus_primitives::NimbusPair>(
        &environ,
        &default_hash,
        1000u32.into(),
    );

    assert_eq!(authorities, Some(vec![Keyring::Alice.public().into()]));
}
