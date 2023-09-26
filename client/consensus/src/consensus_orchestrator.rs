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

//! The Tanssi AuRa consensus algorithm for orchestrator chain and container chain collators.    
//!
//! It calculates based on the orchestrator-state dictated authorities
//! It is identical to AuraWorker and AuraConsensus, except for the fact that we re-implement
//! the ParachainConsensus trait to access the orchestrator-dicated authorities, and further
//! it implements the TanssiWorker to TanssiOnSlot trait. This trait is
use {
    cumulus_client_consensus_common::{ParachainCandidate, ParachainConsensus},
    cumulus_primitives_core::{relay_chain::Hash as PHash, PersistedValidationData},
    parity_scale_codec::{Decode, Encode},
};

use {
    futures::{lock::Mutex, prelude::*},
    nimbus_primitives::{
        CompatibleDigestItem as NimbusCompatibleDigestItem, NimbusPair, NIMBUS_KEY_ID,
    },
    sc_client_api::{backend::AuxStore, BlockOf},
    sc_consensus::{BlockImport, BlockImportParams, ForkChoiceStrategy, StateAction},
    sc_consensus_aura::{find_pre_digest, CompatibilityMode},
    sc_consensus_slots::{
        BackoffAuthoringBlocksStrategy, SimpleSlotWorker, SlotInfo, SlotResult, StorageChanges,
    },
    sc_telemetry::{telemetry, TelemetryHandle, CONSENSUS_DEBUG, CONSENSUS_INFO, CONSENSUS_WARN},
    sp_api::ProvideRuntimeApi,
    sp_application_crypto::{AppCrypto, AppPublic},
    sp_blockchain::HeaderBackend,
    sp_consensus::{
        BlockOrigin, EnableProofRecording, Environment, ProofRecording, Proposer, SyncOracle,
    },
};

use {
    crate::{slot_author, AuthorityId},
    log::{debug, info, warn},
    sp_consensus_aura::{digests::CompatibleDigestItem, SlotDuration},
    sp_consensus_slots::Slot,
    sp_core::crypto::{ByteArray, Pair, Public},
    sp_inherents::CreateInherentDataProviders,
    sp_keystore::{Keystore, KeystorePtr},
    sp_runtime::{
        traits::{Block as BlockT, Header as HeaderT, Member, NumberFor},
        DigestItem,
    },
    std::{
        convert::TryFrom,
        fmt::Debug,
        hash::Hash,
        marker::PhantomData,
        pin::Pin,
        sync::Arc,
        time::{Duration, Instant},
    },
};
pub use {
    sc_consensus_aura::{slot_duration, AuraVerifier, BuildAuraWorkerParams, SlotProportion},
    sc_consensus_slots::InherentDataProviderExt,
};

const LOG_TARGET: &str = "aura::tanssi";

/// The implementation of the Tanssi AURA consensus for parachains.
pub struct OrchestratorAuraConsensus<B, CIDP, GOH, W> {
    create_inherent_data_providers: Arc<CIDP>,
    get_authorities_from_orchestrator: Arc<GOH>,
    aura_worker: Arc<Mutex<W>>,
    slot_duration: SlotDuration,
    _phantom: PhantomData<B>,
}

impl<B, CIDP, GOH, W> Clone for OrchestratorAuraConsensus<B, CIDP, GOH, W> {
    fn clone(&self) -> Self {
        Self {
            create_inherent_data_providers: self.create_inherent_data_providers.clone(),
            get_authorities_from_orchestrator: self.get_authorities_from_orchestrator.clone(),
            aura_worker: self.aura_worker.clone(),
            slot_duration: self.slot_duration,
            _phantom: PhantomData,
        }
    }
}

/// Build the tanssi aura worker.
///
/// The caller is responsible for running this worker, otherwise it will do nothing.
pub fn build_orchestrator_aura_worker<P, B, C, PF, I, SO, L, BS, Error>(
    BuildOrchestratorAuraWorkerParams {
        client,
        block_import,
        proposer_factory,
        sync_oracle,
        justification_sync_link,
        backoff_authoring_blocks,
        keystore,
        block_proposal_slot_portion,
        max_block_proposal_slot_portion,
        telemetry,
        force_authoring,
        compatibility_mode,
    }: BuildOrchestratorAuraWorkerParams<C, I, PF, SO, L, BS, NumberFor<B>>,
) -> impl TanssiSlotWorker<
    B,
    Proposer = PF::Proposer,
    BlockImport = I,
    SyncOracle = SO,
    JustificationSyncLink = L,
    Claim = P::Public,
    AuxData = Vec<AuthorityId<P>>,
>
where
    B: BlockT,
    C: ProvideRuntimeApi<B> + BlockOf + AuxStore + HeaderBackend<B> + Send + Sync,
    AuthorityId<P>: From<<NimbusPair as sp_application_crypto::Pair>::Public>,
    PF: Environment<B, Error = Error> + Send + Sync + 'static,
    PF::Proposer: Proposer<B, Error = Error>,
    P: Pair + Send + Sync,
    P::Public: AppPublic + Hash + Member + Encode + Decode,
    P::Signature: TryFrom<Vec<u8>> + Hash + Member + Encode + Decode,
    I: BlockImport<B> + Send + Sync + 'static,
    Error: std::error::Error + Send + From<sp_consensus::Error> + 'static,
    SO: SyncOracle + Send + Sync + Clone,
    L: sc_consensus::JustificationSyncLink<B>,
    BS: BackoffAuthoringBlocksStrategy<NumberFor<B>> + Send + Sync + 'static,
{
    OrchestratorAuraWorker {
        client,
        block_import,
        env: proposer_factory,
        keystore,
        sync_oracle,
        justification_sync_link,
        force_authoring,
        backoff_authoring_blocks,
        telemetry,
        block_proposal_slot_portion,
        max_block_proposal_slot_portion,
        compatibility_mode,
        _key_type: PhantomData::<P>,
    }
}

/// Parameters of [`OrchestratorAuraConsensus::build`].
pub struct BuildOrchestratorAuraConsensusParams<PF, BI, GOH, CIDP, Client, BS, SO> {
    pub proposer_factory: PF,
    pub create_inherent_data_providers: CIDP,
    pub get_authorities_from_orchestrator: GOH,
    pub block_import: BI,
    pub para_client: Arc<Client>,
    pub backoff_authoring_blocks: Option<BS>,
    pub sync_oracle: SO,
    pub keystore: KeystorePtr,
    pub force_authoring: bool,
    pub slot_duration: SlotDuration,
    pub telemetry: Option<TelemetryHandle>,
    pub block_proposal_slot_portion: SlotProportion,
    pub max_block_proposal_slot_portion: Option<SlotProportion>,
}

impl<B, CIDP, GOH> OrchestratorAuraConsensus<B, CIDP, GOH, ()>
where
    B: BlockT,
    CIDP: CreateInherentDataProviders<B, (PHash, PersistedValidationData)> + 'static,
    GOH: 'static + Sync + Send,
    CIDP::InherentDataProviders: InherentDataProviderExt,
{
    /// Create a new boxed instance of AURA consensus.
    pub fn build<P, Client, BI, SO, PF, BS, Error>(
        BuildOrchestratorAuraConsensusParams {
            proposer_factory,
            create_inherent_data_providers,
            get_authorities_from_orchestrator,
            block_import,
            para_client,
            backoff_authoring_blocks,
            sync_oracle,
            keystore,
            force_authoring,
            slot_duration,
            telemetry,
            block_proposal_slot_portion,
            max_block_proposal_slot_portion,
        }: BuildOrchestratorAuraConsensusParams<PF, BI, GOH, CIDP, Client, BS, SO>,
    ) -> Box<dyn ParachainConsensus<B>>
    where
        Client:
            ProvideRuntimeApi<B> + BlockOf + AuxStore + HeaderBackend<B> + Send + Sync + 'static,
        AuthorityId<P>: From<<NimbusPair as sp_application_crypto::Pair>::Public>,
        BI: BlockImport<B> + Send + Sync + 'static,
        SO: SyncOracle + Send + Sync + Clone + 'static,
        BS: BackoffAuthoringBlocksStrategy<NumberFor<B>> + Send + Sync + 'static,
        PF: Environment<B, Error = Error> + Send + Sync + 'static,
        PF::Proposer: Proposer<
            B,
            Error = Error,
            ProofRecording = EnableProofRecording,
            Proof = <EnableProofRecording as ProofRecording>::Proof,
        >,
        Error: std::error::Error + Send + From<sp_consensus::Error> + 'static,
        P: Pair + Send + Sync + 'static,
        P::Public: AppPublic + Hash + Member + Encode + Decode,
        P::Signature: TryFrom<Vec<u8>> + Hash + Member + Encode + Decode,
        GOH: RetrieveAuthoritiesFromOrchestrator<
                B,
                (PHash, PersistedValidationData),
                Vec<AuthorityId<P>>,
            > + 'static,
    {
        let worker = build_orchestrator_aura_worker::<P, _, _, _, _, _, _, _, _>(
            BuildOrchestratorAuraWorkerParams {
                client: para_client,
                block_import,
                justification_sync_link: (),
                proposer_factory,
                sync_oracle,
                force_authoring,
                backoff_authoring_blocks,
                keystore,
                telemetry,
                block_proposal_slot_portion,
                max_block_proposal_slot_portion,
                compatibility_mode: sc_consensus_aura::CompatibilityMode::None,
            },
        );

        Box::new(OrchestratorAuraConsensus {
            create_inherent_data_providers: Arc::new(create_inherent_data_providers),
            get_authorities_from_orchestrator: Arc::new(get_authorities_from_orchestrator),
            aura_worker: Arc::new(Mutex::new(worker)),
            slot_duration,
            _phantom: PhantomData,
        })
    }
}

impl<B, CIDP, GOH, W> OrchestratorAuraConsensus<B, CIDP, GOH, W>
where
    B: BlockT,
    CIDP: CreateInherentDataProviders<B, (PHash, PersistedValidationData)> + 'static,
    CIDP::InherentDataProviders: InherentDataProviderExt,
    GOH: RetrieveAuthoritiesFromOrchestrator<B, (PHash, PersistedValidationData), W::AuxData>
        + 'static,
    W: TanssiSlotWorker<B> + Send + Sync,
{
    /// Create the inherent data.
    ///
    /// Returns the created inherent data and the inherent data providers used.
    async fn inherent_data(
        &self,
        parent: B::Hash,
        validation_data: &PersistedValidationData,
        relay_parent: PHash,
    ) -> Option<CIDP::InherentDataProviders> {
        self.create_inherent_data_providers
            .create_inherent_data_providers(parent, (relay_parent, validation_data.clone()))
            .await
            .map_err(|e| {
                tracing::error!(
                    target: LOG_TARGET,
                    error = ?e,
                    "Failed to create inherent data providers.",
                )
            })
            .ok()
    }
}

#[async_trait::async_trait]
impl<B, CIDP, GOH, W> ParachainConsensus<B> for OrchestratorAuraConsensus<B, CIDP, GOH, W>
where
    B: BlockT,
    CIDP: CreateInherentDataProviders<B, (PHash, PersistedValidationData)> + Send + Sync + 'static,
    CIDP::InherentDataProviders: InherentDataProviderExt + Send,
    GOH: RetrieveAuthoritiesFromOrchestrator<B, (PHash, PersistedValidationData), W::AuxData>
        + 'static,
    W: TanssiSlotWorker<B> + Send + Sync,
    W::Proposer: Proposer<B, Proof = <EnableProofRecording as ProofRecording>::Proof>,
{
    async fn produce_candidate(
        &mut self,
        parent: &B::Header,
        relay_parent: PHash,
        validation_data: &PersistedValidationData,
    ) -> Option<ParachainCandidate<B>> {
        let inherent_data_providers = self
            .inherent_data(parent.hash(), validation_data, relay_parent)
            .await?;

        let header = self
            .get_authorities_from_orchestrator
            .retrieve_authorities_from_orchestrator(
                parent.hash(),
                (relay_parent, validation_data.clone()),
            )
            .await
            .map_err(|e| {
                tracing::error!(
                    target: LOG_TARGET,
                    error = ?e,
                    "Failed to get orch head.",
                )
            })
            .ok()?;

        let info = SlotInfo::new(
            inherent_data_providers.slot(),
            Box::new(inherent_data_providers),
            self.slot_duration.as_duration(),
            parent.clone(),
            // Set the block limit to 50% of the maximum PoV size.
            //
            // TODO: If we got benchmarking that includes the proof size,
            // we should be able to use the maximum pov size.
            Some((validation_data.max_pov_size / 2) as usize),
        );

        let res = self
            .aura_worker
            .lock()
            .await
            .tanssi_on_slot(info, header)
            .await?;

        Some(ParachainCandidate {
            block: res.block,
            proof: res.storage_proof,
        })
    }
}

#[allow(dead_code)]
struct OrchestratorAuraWorker<C, E, I, P, SO, L, BS, N> {
    client: Arc<C>,
    block_import: I,
    env: E,
    keystore: KeystorePtr,
    sync_oracle: SO,
    justification_sync_link: L,
    force_authoring: bool,
    backoff_authoring_blocks: Option<BS>,
    block_proposal_slot_portion: SlotProportion,
    max_block_proposal_slot_portion: Option<SlotProportion>,
    telemetry: Option<TelemetryHandle>,
    compatibility_mode: CompatibilityMode<N>,
    _key_type: PhantomData<P>,
}

#[async_trait::async_trait]
impl<B, C, E, I, P, Error, SO, L, BS> sc_consensus_slots::SimpleSlotWorker<B>
    for OrchestratorAuraWorker<C, E, I, P, SO, L, BS, NumberFor<B>>
where
    B: BlockT,
    C: ProvideRuntimeApi<B> + BlockOf + HeaderBackend<B> + Sync,
    AuthorityId<P>: From<<NimbusPair as sp_application_crypto::Pair>::Public>,
    E: Environment<B, Error = Error> + Send + Sync,
    E::Proposer: Proposer<B, Error = Error>,
    I: BlockImport<B> + Send + Sync + 'static,
    P: Pair + Send + Sync,
    P::Public: AppPublic + Public + Member + Encode + Decode + Hash,
    P::Signature: TryFrom<Vec<u8>> + Member + Encode + Decode + Hash + Debug,
    SO: SyncOracle + Send + Clone + Sync,
    L: sc_consensus::JustificationSyncLink<B>,
    BS: BackoffAuthoringBlocksStrategy<NumberFor<B>> + Send + Sync + 'static,
    Error: std::error::Error + Send + From<sp_consensus::Error> + 'static,
{
    type BlockImport = I;
    type SyncOracle = SO;
    type JustificationSyncLink = L;
    type CreateProposer =
        Pin<Box<dyn Future<Output = Result<E::Proposer, sp_consensus::Error>> + Send + 'static>>;
    type Proposer = E::Proposer;
    type Claim = P::Public;
    type AuxData = Vec<AuthorityId<P>>;

    fn logging_target(&self) -> &'static str {
        "tanssi_aura"
    }

    fn block_import(&mut self) -> &mut Self::BlockImport {
        &mut self.block_import
    }

    fn aux_data(
        &self,
        _header: &B::Header,
        _slot: Slot,
    ) -> Result<Self::AuxData, sp_consensus::Error> {
        Ok(Default::default())
    }

    fn authorities_len(&self, epoch_data: &Self::AuxData) -> Option<usize> {
        Some(epoch_data.len())
    }

    async fn claim_slot(
        &mut self,
        _header: &B::Header,
        slot: Slot,
        epoch_data: &Self::AuxData,
    ) -> Option<Self::Claim> {
        let expected_author = slot_author::<P>(slot, epoch_data);
        // if not running with force-authoring, just do the usual slot check
        if !self.force_authoring {
            expected_author.and_then(|p| {
                if Keystore::has_keys(&*self.keystore, &[(p.to_raw_vec(), NIMBUS_KEY_ID)]) {
                    Some(p.clone())
                } else {
                    None
                }
            })
        }
        // if running with force-authoring, as long as you are in the authority set,
        // propose
        else {
            epoch_data
                .iter()
                .find(|key| {
                    Keystore::has_keys(&*self.keystore, &[(key.to_raw_vec(), NIMBUS_KEY_ID)])
                })
                .cloned()
        }
    }

    fn pre_digest_data(&self, slot: Slot, claim: &Self::Claim) -> Vec<sp_runtime::DigestItem> {
        vec![
            <DigestItem as CompatibleDigestItem<P::Signature>>::aura_pre_digest(slot),
            // We inject the nimbus digest as well. Crutial to be able to verify signatures
            <DigestItem as NimbusCompatibleDigestItem>::nimbus_pre_digest(
                // TODO remove this unwrap through trait reqs
                nimbus_primitives::NimbusId::from_slice(claim.as_ref()).unwrap(),
            ),
        ]
    }

    async fn block_import_params(
        &self,
        header: B::Header,
        header_hash: &B::Hash,
        body: Vec<B::Extrinsic>,
        storage_changes: StorageChanges<B>,
        public: Self::Claim,
        _epoch: Self::AuxData,
    ) -> Result<sc_consensus::BlockImportParams<B>, sp_consensus::Error> {
        // sign the pre-sealed hash of the block and then
        // add it to a digest item.
        let signature = Keystore::sign_with(
            &*self.keystore,
            <AuthorityId<P> as AppCrypto>::ID,
            <AuthorityId<P> as AppCrypto>::CRYPTO_ID,
            public.as_slice(),
            header_hash.as_ref(),
        )
        .map_err(|e| sp_consensus::Error::CannotSign(format!("{}. Key: {:?}", e, public)))?
        .ok_or_else(|| {
            sp_consensus::Error::CannotSign(format!(
                "Could not find key in keystore. Key: {:?}",
                public
            ))
        })?;
        let signature = signature
            .clone()
            .try_into()
            .map_err(|_| sp_consensus::Error::InvalidSignature(signature, public.to_raw_vec()))?;

        let signature_digest_item =
            <DigestItem as NimbusCompatibleDigestItem>::nimbus_seal(signature);

        let mut import_block = BlockImportParams::new(BlockOrigin::Own, header);
        import_block.post_digests.push(signature_digest_item);
        import_block.body = Some(body);
        import_block.state_action =
            StateAction::ApplyChanges(sc_consensus::StorageChanges::Changes(storage_changes));
        import_block.fork_choice = Some(ForkChoiceStrategy::LongestChain);

        Ok(import_block)
    }

    fn force_authoring(&self) -> bool {
        self.force_authoring
    }

    fn should_backoff(&self, slot: Slot, chain_head: &B::Header) -> bool {
        if let Some(ref strategy) = self.backoff_authoring_blocks {
            if let Ok(chain_head_slot) = find_pre_digest::<B, P::Signature>(chain_head) {
                return strategy.should_backoff(
                    *chain_head.number(),
                    chain_head_slot,
                    self.client.info().finalized_number,
                    slot,
                    self.logging_target(),
                );
            }
        }
        false
    }

    fn sync_oracle(&mut self) -> &mut Self::SyncOracle {
        &mut self.sync_oracle
    }

    fn justification_sync_link(&mut self) -> &mut Self::JustificationSyncLink {
        &mut self.justification_sync_link
    }

    fn proposer(&mut self, block: &B::Header) -> Self::CreateProposer {
        self.env
            .init(block)
            .map_err(|e| sp_consensus::Error::ClientImport(format!("{:?}", e)))
            .boxed()
    }

    fn telemetry(&self) -> Option<TelemetryHandle> {
        self.telemetry.clone()
    }

    fn proposing_remaining_duration(&self, slot_info: &SlotInfo<B>) -> std::time::Duration {
        let parent_slot = find_pre_digest::<B, P::Signature>(&slot_info.chain_head).ok();

        sc_consensus_slots::proposing_remaining_duration(
            parent_slot,
            slot_info,
            &self.block_proposal_slot_portion,
            self.max_block_proposal_slot_portion.as_ref(),
            sc_consensus_slots::SlotLenienceType::Exponential,
            self.logging_target(),
        )
    }
}

/// Parameters of [`build_aura_worker`].
pub struct BuildOrchestratorAuraWorkerParams<C, I, PF, SO, L, BS, N> {
    /// The client to interact with the chain.
    pub client: Arc<C>,
    /// The block import.
    pub block_import: I,
    /// The proposer factory to build proposer instances.
    pub proposer_factory: PF,
    /// The sync oracle that can give us the current sync status.
    pub sync_oracle: SO,
    /// Hook into the sync module to control the justification sync process.
    pub justification_sync_link: L,
    /// Should we force the authoring of blocks?
    pub force_authoring: bool,
    /// The backoff strategy when we miss slots.
    pub backoff_authoring_blocks: Option<BS>,
    /// The keystore used by the node.
    pub keystore: KeystorePtr,
    /// The proportion of the slot dedicated to proposing.
    ///
    /// The block proposing will be limited to this proportion of the slot from the starting of the
    /// slot. However, the proposing can still take longer when there is some lenience factor
    /// applied, because there were no blocks produced for some slots.
    pub block_proposal_slot_portion: SlotProportion,
    /// The maximum proportion of the slot dedicated to proposing with any lenience factor applied
    /// due to no blocks being produced.
    pub max_block_proposal_slot_portion: Option<SlotProportion>,
    /// Telemetry instance used to report telemetry metrics.
    pub telemetry: Option<TelemetryHandle>,
    /// Compatibility mode that should be used.
    ///
    /// If in doubt, use `Default::default()`.
    pub compatibility_mode: CompatibilityMode<N>,
}

#[async_trait::async_trait]
pub trait RetrieveAuthoritiesFromOrchestrator<Block: BlockT, ExtraArgs, A>: Send + Sync {
    /// Create the inherent data providers at the given `parent` block using the given `extra_args`.
    async fn retrieve_authorities_from_orchestrator(
        &self,
        parent: Block::Hash,
        extra_args: ExtraArgs,
    ) -> Result<A, Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait::async_trait]
impl<F, Block, ExtraArgs, Fut, A> RetrieveAuthoritiesFromOrchestrator<Block, ExtraArgs, A> for F
where
    Block: BlockT,
    F: Fn(Block::Hash, ExtraArgs) -> Fut + Sync + Send,
    Fut: std::future::Future<Output = Result<A, Box<dyn std::error::Error + Send + Sync>>>
        + Send
        + 'static,
    ExtraArgs: Send + 'static,
{
    async fn retrieve_authorities_from_orchestrator(
        &self,
        parent: Block::Hash,
        extra_args: ExtraArgs,
    ) -> Result<A, Box<dyn std::error::Error + Send + Sync>> {
        (*self)(parent, extra_args).await
    }
}

#[async_trait::async_trait]
pub trait TanssiSlotWorker<B: BlockT>: SimpleSlotWorker<B> {
    /// Called when a new slot is triggered.
    ///
    /// Returns a future that resolves to a [`SlotResult`] iff a block was successfully built in
    /// the slot. Otherwise `None` is returned.
    /// Accepts the orchestrator header as an input
    async fn tanssi_on_slot(
        &mut self,
        slot_info: SlotInfo<B>,
        aux_data: Self::AuxData,
    ) -> Option<SlotResult<B, <Self::Proposer as Proposer<B>>::Proof>>;
}

#[async_trait::async_trait]
impl<B, C, E, I, P, Error, SO, L, BS> TanssiSlotWorker<B>
    for OrchestratorAuraWorker<C, E, I, P, SO, L, BS, NumberFor<B>>
where
    B: BlockT,
    C: ProvideRuntimeApi<B> + BlockOf + HeaderBackend<B> + Sync,
    AuthorityId<P>: From<<NimbusPair as sp_application_crypto::Pair>::Public>,
    E: Environment<B, Error = Error> + Send + Sync,
    E::Proposer: Proposer<B, Error = Error>,
    I: BlockImport<B> + Send + Sync + 'static,
    P: Pair + Send + Sync,
    P::Public: AppPublic + Public + Member + Encode + Decode + Hash,
    P::Signature: TryFrom<Vec<u8>> + Member + Encode + Decode + Hash + Debug,
    SO: SyncOracle + Send + Clone + Sync,
    L: sc_consensus::JustificationSyncLink<B>,
    BS: BackoffAuthoringBlocksStrategy<NumberFor<B>> + Send + Sync + 'static,
    Error: std::error::Error + Send + From<sp_consensus::Error> + 'static,
{
    async fn tanssi_on_slot(
        &mut self,
        slot_info: SlotInfo<B>,
        aux_data: Self::AuxData,
    ) -> Option<SlotResult<B, <Self::Proposer as Proposer<B>>::Proof>>
    where
        Self: Sync,
    {
        let slot = slot_info.slot;
        let telemetry = self.telemetry();
        let logging_target = self.logging_target();

        let proposing_remaining_duration = self.proposing_remaining_duration(&slot_info);

        let end_proposing_at = if proposing_remaining_duration == Duration::default() {
            debug!(
                target: logging_target,
                "Skipping proposal slot {} since there's no time left to propose", slot,
            );

            return None;
        } else {
            Instant::now() + proposing_remaining_duration
        };

        self.notify_slot(&slot_info.chain_head, slot, &aux_data);

        let authorities_len = self.authorities_len(&aux_data);

        if !self.force_authoring()
            && self.sync_oracle().is_offline()
            && authorities_len.map(|a| a > 1).unwrap_or(false)
        {
            debug!(
                target: logging_target,
                "Skipping proposal slot. Waiting for the network."
            );
            telemetry!(
                telemetry;
                CONSENSUS_DEBUG;
                "slots.skipping_proposal_slot";
                "authorities_len" => authorities_len,
            );

            return None;
        }

        let claim = self
            .claim_slot(&slot_info.chain_head, slot, &aux_data)
            .await?;

        log::info!("claim valid for slot {:?}", slot);

        if self.should_backoff(slot, &slot_info.chain_head) {
            return None;
        }

        debug!(
            target: logging_target,
            "Starting authorship at slot: {slot}"
        );

        telemetry!(telemetry; CONSENSUS_DEBUG; "slots.starting_authorship"; "slot_num" => slot);

        let proposer = match self.proposer(&slot_info.chain_head).await {
            Ok(p) => p,
            Err(err) => {
                warn!(
                    target: logging_target,
                    "Unable to author block in slot {slot:?}: {err}"
                );

                telemetry!(
                    telemetry;
                    CONSENSUS_WARN;
                    "slots.unable_authoring_block";
                    "slot" => *slot,
                    "err" => ?err
                );

                return None;
            }
        };

        let proposal = self
            .propose(proposer, &claim, slot_info, end_proposing_at)
            .await?;

        let (block, storage_proof) = (proposal.block, proposal.proof);
        let (header, body) = block.deconstruct();
        let header_num = *header.number();
        let header_hash = header.hash();
        let parent_hash = *header.parent_hash();

        let block_import_params = match self
            .block_import_params(
                header,
                &header_hash,
                body.clone(),
                proposal.storage_changes,
                claim,
                aux_data,
            )
            .await
        {
            Ok(bi) => bi,
            Err(err) => {
                warn!(
                    target: logging_target,
                    "Failed to create block import params: {}", err
                );

                return None;
            }
        };

        info!(
            target: logging_target,
            "🔖 Pre-sealed block for proposal at {}. Hash now {:?}, previously {:?}.",
            header_num,
            block_import_params.post_hash(),
            header_hash,
        );

        telemetry!(
            telemetry;
            CONSENSUS_INFO;
            "slots.pre_sealed_block";
            "header_num" => ?header_num,
            "hash_now" => ?block_import_params.post_hash(),
            "hash_previously" => ?header_hash,
        );

        let header = block_import_params.post_header();
        match self.block_import().import_block(block_import_params).await {
            Ok(res) => {
                res.handle_justification(
                    &header.hash(),
                    *header.number(),
                    self.justification_sync_link(),
                );
            }
            Err(err) => {
                warn!(
                    target: logging_target,
                    "Error with block built on {:?}: {}", parent_hash, err,
                );

                telemetry!(
                    telemetry;
                    CONSENSUS_WARN;
                    "slots.err_with_block_built_on";
                    "hash" => ?parent_hash,
                    "err" => ?err,
                );
            }
        }

        Some(SlotResult {
            block: B::new(header, body),
            storage_proof,
        })
    }
}
