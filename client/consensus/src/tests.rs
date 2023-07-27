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

#[cfg(test)]
mod tests {
    use {
        super::*,
        futures::prelude::*,
        nimbus_primitives::{CompatibleDigestItem, NimbusPair, NIMBUS_ENGINE_ID, NIMBUS_KEY_ID},
        parking_lot::Mutex,
        sc_block_builder::BlockBuilderProvider,
        sc_client_api::BlockchainEvents,
        sc_consensus::{BoxJustificationImport, ForkChoiceStrategy},
        sc_consensus_aura::SlotProportion,
        sc_consensus_slots::{BackoffAuthoringOnFinalizedHeadLagging, SimpleSlotWorker},
        sc_keystore::LocalKeystore,
        sc_network_test::{Block as TestBlock, *},
        sp_consensus::{
            BlockOrigin, EnableProofRecording, Environment, NoNetwork as DummyOracle, Proposal,
            Proposer,
        },
        sp_consensus_aura::{inherents::InherentDataProvider, SlotDuration},
        sp_inherents::InherentData,
        sp_keyring::sr25519::Keyring,
        sp_keystore::Keystore,
        sp_runtime::{
            traits::{Block as BlockT, Header as _},
            Digest, DigestItem,
        },
        sp_timestamp::Timestamp,
        std::{sync::Arc, task::Poll, time::Duration},
        substrate_test_runtime_client::TestClient,
    };
    const SLOT_DURATION_MS: u64 = 1000;

    type Error = sp_blockchain::Error;

    struct DummyFactory(Arc<TestClient>);
    struct DummyProposer(u64, Arc<TestClient>);

    #[derive(Clone)]
    pub struct OwnPassThroughVerifier {
        finalized: bool,
    }

    impl OwnPassThroughVerifier {
        /// Create a new instance.
        ///
        /// Every verified block will use `finalized` for the `BlockImportParams`.
        pub fn new(finalized: bool) -> Self {
            Self { finalized }
        }
    }

    #[async_trait::async_trait]
    impl<B: BlockT> sc_consensus::Verifier<B> for OwnPassThroughVerifier {
        async fn verify(
            &mut self,
            mut block: sc_consensus::BlockImportParams<B, ()>,
        ) -> Result<sc_consensus::BlockImportParams<B, ()>, String> {
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
                .ok_or(
                    "Expected one consensus or pre-runtime digest that contains author id bytes",
                )?;

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

    impl Environment<TestBlock> for DummyFactory {
        type Proposer = DummyProposer;
        type CreateProposer = future::Ready<Result<DummyProposer, Error>>;
        type Error = Error;

        fn init(&mut self, parent_header: &<TestBlock as BlockT>::Header) -> Self::CreateProposer {
            future::ready(Ok(DummyProposer(parent_header.number + 1, self.0.clone())))
        }
    }

    impl Proposer<TestBlock> for DummyProposer {
        type Error = Error;
        type Transaction =
            sc_client_api::TransactionFor<substrate_test_runtime_client::Backend, TestBlock>;
        type Proposal =
            future::Ready<Result<Proposal<TestBlock, Self::Transaction, Self::Proof>, Error>>;
        type ProofRecording = EnableProofRecording;
        type Proof = sc_client_api::StorageProof;

        fn propose(
            self,
            _: InherentData,
            digests: Digest,
            _: Duration,
            _: Option<usize>,
        ) -> Self::Proposal {
            let r = self
                .1
                .new_block(digests)
                .unwrap()
                .build()
                .map_err(|e| e.into());

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
        type Verifier = OwnPassThroughVerifier;
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
            OwnPassThroughVerifier::new(true)
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

    #[tokio::test]
    async fn authoring_blocks() {
        let net = AuraTestNet::new(3);

        let peers = &[
            (0, Keyring::Alice),
            (1, Keyring::Bob),
            (2, Keyring::Charlie),
        ];

        let net = Arc::new(Mutex::new(net));
        let mut import_notifications = Vec::new();
        let mut aura_futures = Vec::new();

        let mut keystore_paths = Vec::new();
        for (peer_id, key) in peers {
            let mut net = net.lock();
            let peer = net.peer(*peer_id);
            let client = peer.client().as_client();
            let select_chain = peer.select_chain().expect("full client has a select chain");
            let keystore_path = tempfile::tempdir().expect("Creates keystore path");
            let keystore = Arc::new(
                LocalKeystore::open(keystore_path.path(), None).expect("Creates keystore."),
            );

            keystore
                .sr25519_generate_new(NIMBUS_KEY_ID, Some(&key.to_seed()))
                .expect("Creates authority key");
            keystore_paths.push(keystore_path);

            let environ = DummyFactory(client.clone());
            import_notifications.push(
                client
                    .import_notification_stream()
                    .take_while(|n| {
                        future::ready(!(n.origin != BlockOrigin::Own && n.header.number() < &5))
                    })
                    .for_each(move |_| futures::future::ready(())),
            );

            aura_futures.push(
                start_orchestrator::<nimbus_primitives::NimbusPair, _, _, _, _, _, _, _, _, _, _>(
                    StartAuraParams {
                        slot_duration: SlotDuration::from_millis(SLOT_DURATION_MS),
                        block_import: client.clone(),
                        select_chain,
                        client,
                        proposer_factory: environ,
                        sync_oracle: DummyOracle,
                        justification_sync_link: (),
                        create_inherent_data_providers: |_, _| async {
                            let slot = InherentDataProvider::from_timestamp_and_slot_duration(
                                Timestamp::current(),
                                SlotDuration::from_millis(SLOT_DURATION_MS),
                            );

                            Ok((slot,))
                        },
                        force_authoring: false,
                        backoff_authoring_blocks: Some(
                            BackoffAuthoringOnFinalizedHeadLagging::default(),
                        ),
                        keystore,
                        block_proposal_slot_portion: SlotProportion::new(0.5),
                        max_block_proposal_slot_portion: None,
                        telemetry: None,
                        compatibility_mode: Default::default(),
                    },
                )
                .expect("Starts aura"),
            );
        }

        future::select(
            future::poll_fn(move |cx| {
                net.lock().poll(cx);
                Poll::<()>::Pending
            }),
            future::select(
                future::join_all(aura_futures),
                future::join_all(import_notifications),
            ),
        )
        .await;
    }

    #[tokio::test]
    async fn authoring_blocks_but_producing_candidates_instead_of_calling_on_slot() {
        let net = AuraTestNet::new(3);

        let peers = &[
            (0, Keyring::Alice),
            (1, Keyring::Bob),
            (2, Keyring::Charlie),
        ];

        let net = Arc::new(Mutex::new(net));
        let mut import_notifications = Vec::new();
        let mut aura_futures = Vec::new();

        let mut keystore_paths = Vec::new();
        for (peer_id, key) in peers {
            let mut net = net.lock();
            let peer = net.peer(*peer_id);
            let client = peer.client().as_client();
            let select_chain = peer.select_chain().expect("full client has a select chain");
            let keystore_path = tempfile::tempdir().expect("Creates keystore path");
            let keystore = Arc::new(
                LocalKeystore::open(keystore_path.path(), None).expect("Creates keystore."),
            );

            keystore
                .sr25519_generate_new(NIMBUS_KEY_ID, Some(&key.to_seed()))
                .expect("Creates authority key");
            keystore_paths.push(keystore_path);

            let environ = DummyFactory(client.clone());
            import_notifications.push(
                client
                    .import_notification_stream()
                    .take_while(|n| {
                        future::ready(!(n.origin != BlockOrigin::Own && n.header.number() < &5))
                    })
                    .for_each(move |_| futures::future::ready(())),
            );

            let create_inherent_data_providers = |_, _| async {
                let slot = InherentDataProvider::from_timestamp_and_slot_duration(
                    Timestamp::current(),
                    SlotDuration::from_millis(SLOT_DURATION_MS),
                );

                Ok((slot,))
            };

            let sync_oracle = DummyOracle;
            let slot_duration = SlotDuration::from_millis(SLOT_DURATION_MS);

            let params = crate::BuildOrchestratorAuraConsensusParams {
                proposer_factory: environ,
                create_inherent_data_providers: |_, _| async {
                    let slot = InherentDataProvider::from_timestamp_and_slot_duration(
                        Timestamp::current(),
                        SlotDuration::from_millis(SLOT_DURATION_MS),
                    );

                    Ok((slot,))
                },
                get_authorities_from_orchestrator: move |_block_hash: <TestBlock as BlockT>::Hash,
                (_relay_parent, _validation_data): (H256, PersistedValidationData)| {
                    async move {
                        let aux_data = vec![
                            (Keyring::Alice).public().into(),
                            (Keyring::Bob).public().into(),
                            (Keyring::Charlie).public().into(),
                        ];
                        Ok(aux_data)
                    }
                },
                block_import: client.clone(),
                para_client: client,
                sync_oracle: DummyOracle,
                keystore,
                force_authoring: false,
                backoff_authoring_blocks: Some(
                    BackoffAuthoringOnFinalizedHeadLagging::default(),
                ),
                slot_duration: SlotDuration::from_millis(SLOT_DURATION_MS),
                // We got around 500ms for proposing
                block_proposal_slot_portion: SlotProportion::new(0.5),
                max_block_proposal_slot_portion: None,
                telemetry: None,
            };

            let parachain_block_producer =
                crate::OrchestratorAuraConsensus::build::<NimbusPair, _, _, _, _, _, _>(params);

            aura_futures.push(start_orchestrator_aura_consensus_candidate_producer(
                slot_duration,
                select_chain,
                parachain_block_producer,
                sync_oracle,
                create_inherent_data_providers,
            ));
        }

        future::select(
            future::poll_fn(move |cx| {
                net.lock().poll(cx);
                Poll::<()>::Pending
            }),
            future::select(
                future::join_all(aura_futures),
                future::join_all(import_notifications),
            ),
        )
        .await;
    }

    #[tokio::test]
    async fn current_node_authority_should_claim_slot() {
        let net = AuraTestNet::new(4);

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

        let net = Arc::new(Mutex::new(net));

        let mut net = net.lock();
        let peer = net.peer(3);
        let client = peer.client().as_client();
        let environ = DummyFactory(client.clone());

        let worker = build_orchestrator_aura_worker::<
            nimbus_primitives::NimbusPair,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
        >(BuildOrchestratorAuraWorkerParams {
            client: client.clone(),
            block_import: client,
            proposer_factory: environ,
            keystore: keystore.into(),
            sync_oracle: DummyOracle,
            justification_sync_link: (),
            force_authoring: false,
            backoff_authoring_blocks: Some(BackoffAuthoringOnFinalizedHeadLagging::default()),
            telemetry: None,
            block_proposal_slot_portion: SlotProportion::new(0.5),
            max_block_proposal_slot_portion: None,
            compatibility_mode: Default::default(),
        });

        let head = Header::new(
            1,
            H256::from_low_u64_be(0),
            H256::from_low_u64_be(0),
            Default::default(),
            Default::default(),
        );
        assert!(worker
            .claim_slot(&head, 0.into(), &authorities)
            .await
            .is_none());
        assert!(worker
            .claim_slot(&head, 1.into(), &authorities)
            .await
            .is_none());
        assert!(worker
            .claim_slot(&head, 2.into(), &authorities)
            .await
            .is_none());
        assert!(worker
            .claim_slot(&head, 3.into(), &authorities)
            .await
            .is_some());
        assert!(worker
            .claim_slot(&head, 4.into(), &authorities)
            .await
            .is_none());
        assert!(worker
            .claim_slot(&head, 5.into(), &authorities)
            .await
            .is_none());
        assert!(worker
            .claim_slot(&head, 6.into(), &authorities)
            .await
            .is_none());
        assert!(worker
            .claim_slot(&head, 7.into(), &authorities)
            .await
            .is_some());
    }

    #[tokio::test]
    async fn on_slot_returns_correct_block() {
        let net = AuraTestNet::new(4);

        let keystore_path = tempfile::tempdir().expect("Creates keystore path");
        let keystore = LocalKeystore::open(keystore_path.path(), None).expect("Creates keystore.");
        keystore
            .sr25519_generate_new(NIMBUS_KEY_ID, Some(&Keyring::Alice.to_seed()))
            .expect("Key should be created");

        let net = Arc::new(Mutex::new(net));

        let mut net = net.lock();
        let peer = net.peer(3);
        let client = peer.client().as_client();
        let environ = DummyFactory(client.clone());

        let mut worker = build_orchestrator_aura_worker::<
            nimbus_primitives::NimbusPair,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
        >(BuildOrchestratorAuraWorkerParams {
            client: client.clone(),
            block_import: client.clone(),
            proposer_factory: environ,
            keystore: keystore.into(),
            sync_oracle: DummyOracle,
            justification_sync_link: (),
            force_authoring: false,
            backoff_authoring_blocks: Some(BackoffAuthoringOnFinalizedHeadLagging::default()),
            telemetry: None,
            block_proposal_slot_portion: SlotProportion::new(0.5),
            max_block_proposal_slot_portion: None,
            compatibility_mode: Default::default(),
        });

        let head = client.expect_header(client.info().genesis_hash).unwrap();

        use crate::consensus_orchestrator::TanssiSlotWorker;
        let res = worker
            .tanssi_on_slot(
                SlotInfo {
                    slot: 0.into(),
                    ends_at: std::time::Instant::now() + Duration::from_secs(100),
                    create_inherent_data: Box::new(()),
                    duration: Duration::from_millis(1000),
                    chain_head: head,
                    block_size_limit: None,
                },
                vec![
                    (Keyring::Alice).public().into(),
                    (Keyring::Bob).public().into(),
                    (Keyring::Charlie).public().into(),
                ],
            )
            .await
            .unwrap();

        // The returned block should be imported and we should be able to get its header by now.
        assert!(client.header(res.block.hash()).unwrap().is_some());
    }
}

use {
    crate::{
        consensus_orchestrator::{
            build_orchestrator_aura_worker, BuildOrchestratorAuraWorkerParams,
        },
        AuthorityId, ConsensusError, InherentDataProviderExt,
    },
    futures::Future,
    parity_scale_codec::{Decode, Encode},
    sc_client_api::{backend::AuxStore, BlockOf, HeaderBackend},
    sc_consensus::BlockImport,
    sc_consensus_aura::StartAuraParams,
    sc_consensus_slots::BackoffAuthoringBlocksStrategy,
    sp_api::ProvideRuntimeApi,
    sp_application_crypto::AppPublic,
    sp_consensus::{Environment, Proposer, SelectChain, SyncOracle},
    sp_core::{
        crypto::{ByteArray, Pair},
        H256,
    },
    sp_inherents::CreateInherentDataProviders,
    sp_runtime::traits::{Block as BlockT, Header as HeaderT, Member, NumberFor},
    std::hash::Hash,
};
/// Start the aura worker. The returned future should be run in a futures executor.
pub fn start_orchestrator<P, B, C, SC, I, PF, SO, L, CIDP, BS, Error>(
    StartAuraParams {
        slot_duration,
        client,
        select_chain,
        block_import,
        proposer_factory,
        sync_oracle,
        justification_sync_link,
        create_inherent_data_providers,
        force_authoring,
        backoff_authoring_blocks,
        keystore,
        block_proposal_slot_portion,
        max_block_proposal_slot_portion,
        telemetry,
        compatibility_mode,
    }: StartAuraParams<C, SC, I, PF, SO, L, CIDP, BS, NumberFor<B>>,
) -> Result<impl Future<Output = ()>, ConsensusError>
where
    P: Pair<Public = nimbus_primitives::NimbusId> + Send + Sync,
    P::Public: AppPublic + Hash + Member + Encode + Decode,
    P::Signature: TryFrom<Vec<u8>> + Hash + Member + Encode + Decode,
    B: BlockT,
    C: ProvideRuntimeApi<B> + BlockOf + AuxStore + HeaderBackend<B> + Send + Sync,
    AuthorityId<P>: From<<nimbus_primitives::NimbusPair as sp_application_crypto::Pair>::Public>,
    SC: SelectChain<B>,
    I: BlockImport<B, Transaction = sp_api::TransactionFor<C, B>> + Send + Sync + 'static,
    PF: Environment<B, Error = Error> + Send + Sync + 'static,
    PF::Proposer: Proposer<B, Error = Error, Transaction = sp_api::TransactionFor<C, B>>,
    SO: SyncOracle + Send + Sync + Clone,
    L: sc_consensus::JustificationSyncLink<B>,
    CIDP: CreateInherentDataProviders<B, ()> + Send + 'static,
    CIDP::InherentDataProviders: InherentDataProviderExt + Send,
    BS: BackoffAuthoringBlocksStrategy<NumberFor<B>> + Send + Sync + 'static,
    Error: std::error::Error + Send + From<ConsensusError> + 'static,
{
    let worker = build_orchestrator_aura_worker::<P, _, _, _, _, _, _, _, _>(
        BuildOrchestratorAuraWorkerParams {
            client,
            block_import,
            proposer_factory,
            keystore,
            sync_oracle: sync_oracle.clone(),
            justification_sync_link,
            force_authoring,
            backoff_authoring_blocks,
            telemetry,
            block_proposal_slot_portion,
            max_block_proposal_slot_portion,
            compatibility_mode,
        },
    );
    let get_authorities_from_orchestrator =
        move |_block_hash: <B as BlockT>::Hash,
              (_relay_parent, _validation_data): (H256, PersistedValidationData)| {
            async move {
                let aux_data = vec![
                    (Keyring::Alice).public().into(),
                    (Keyring::Bob).public().into(),
                    (Keyring::Charlie).public().into(),
                ];
                Ok(aux_data)
            }
        };

    Ok(start_tanssi_slot_worker(
        slot_duration,
        select_chain,
        worker,
        sync_oracle,
        create_inherent_data_providers,
        get_authorities_from_orchestrator,
    ))
}

use {
    crate::consensus_orchestrator::RetrieveAuthoritiesFromOrchestrator,
    cumulus_primitives_core::{relay_chain::Hash as PHash, PersistedValidationData},
};

use {
    crate::LOG_TARGET,
    futures_timer::Delay,
    sc_consensus_slots::SlotInfo,
    sp_consensus_slots::{Slot, SlotDuration},
    std::time::Duration,
};
/// A stream that returns every time there is a new slot.
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

use {
    cumulus_client_consensus_common::ParachainConsensus, nimbus_primitives::NimbusId,
    sp_keyring::sr25519::Keyring,
};
/// Start a new slot worker.
///
/// Every time a new slot is triggered, `worker.on_slot` is called and the future it returns is
/// polled until completion, unless we are major syncing.
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

/// Start a new slot worker.
///
/// Every time a new slot is triggered, `worker.on_slot` is called and the future it returns is
/// polled until completion, unless we are major syncing.
pub async fn start_tanssi_slot_worker<B, C, W, SO, CIDP, GOH>(
    slot_duration: SlotDuration,
    client: C,
    mut worker: W,
    sync_oracle: SO,
    create_inherent_data_providers: CIDP,
    get_authorities_from_orchestrator: GOH,
) where
    B: BlockT,
    C: SelectChain<B>,
    W: crate::consensus_orchestrator::TanssiSlotWorker<B>,
    SO: SyncOracle + Send,
    CIDP: CreateInherentDataProviders<B, ()> + Send + 'static,
    CIDP::InherentDataProviders: InherentDataProviderExt + Send,
    GOH: RetrieveAuthoritiesFromOrchestrator<B, (PHash, PersistedValidationData), W::AuxData>,
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

        let authorities = get_authorities_from_orchestrator
            .retrieve_authorities_from_orchestrator(
                Default::default(),
                (Default::default(), Default::default()),
            )
            .await
            .unwrap();

        let _ = worker.tanssi_on_slot(slot_info, authorities).await;
    }
}
