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

// This tests have been greatly influenced by
// https://github.com/paritytech/substrate/blob/master/client/consensus/aura/src/lib.rs#L832
// Most of the items hereby added are intended to make it work with our current consensus mechanism
use {
    crate::{InherentDataProviderExt, LOG_TARGET},
    cumulus_client_consensus_common::ParachainConsensus,
    futures::prelude::*,
    futures_timer::Delay,
    nimbus_primitives::{CompatibleDigestItem, NimbusId, NimbusPair, NIMBUS_ENGINE_ID},
    parking_lot::Mutex,
    sc_block_builder::BlockBuilderProvider,
    sc_consensus::{BoxJustificationImport, ForkChoiceStrategy},
    sc_consensus_slots::SlotInfo,
    sc_network_test::{Block as TestBlock, *},
    sp_consensus::{
        EnableProofRecording, Environment, Proposal, Proposer, SelectChain, SyncOracle,
    },
    sp_consensus_aura::SlotDuration,
    sp_consensus_slots::Slot,
    sp_core::crypto::{ByteArray, Pair},
    sp_inherents::{CreateInherentDataProviders, InherentData},
    sp_keyring::sr25519::Keyring,
    sp_runtime::{
        traits::{Block as BlockT, Header as _},
        Digest, DigestItem,
    },
    std::{sync::Arc, time::Duration},
    substrate_test_runtime_client::TestClient,
};

// Duration of slot time
#[allow(unused)]
const SLOT_DURATION_MS: u64 = 1000;

type Error = sp_blockchain::Error;

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

        futures::future::ready(r.map(|b| Proposal {
            block: b.block,
            proof: sc_client_api::StorageProof::empty(),
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
#[allow(unused)]
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
#[allow(unused)]
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
#[allow(unused)]
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
