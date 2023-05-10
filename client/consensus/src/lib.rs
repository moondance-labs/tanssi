// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! The AuRa consensus algorithm for parachains.    
//!
//! This extends the Substrate provided AuRa consensus implementation to make it compatible for
//! parachains. The main entry points for of this consensus algorithm are [`AuraConsensus::build`]
//! and [`fn@import_queue`].
//!
//! For more information about AuRa, the Substrate crate should be checked.
use cumulus_client_consensus_common::{
    ParachainBlockImportMarker, ParachainCandidate, ParachainConsensus,
};
use cumulus_primitives_core::{relay_chain::Hash as PHash, PersistedValidationData};
use parity_scale_codec::{Codec, Decode, Encode};

use futures::lock::Mutex;
use sc_client_api::{backend::AuxStore, BlockOf};
use sc_consensus::{BlockImport, BlockImportParams, ForkChoiceStrategy, StateAction};
use sc_consensus_aura::{find_pre_digest, CompatibilityMode};
use sc_consensus_slots::{
    BackoffAuthoringBlocksStrategy, SimpleSlotWorker, SlotInfo, StorageChanges,
};

use futures::prelude::*;
use nimbus_primitives::CompatibleDigestItem as NimbusCompatibleDigestItem;
use nimbus_primitives::NimbusId;
use sc_telemetry::TelemetryHandle;
use sp_api::Core;
use sp_api::ProvideRuntimeApi;
use sp_application_crypto::{AppKey, AppPublic};
use sp_blockchain::HeaderBackend;
use sp_consensus::{
    BlockOrigin, EnableProofRecording, Environment, Error as ConsensusError, ProofRecording,
    Proposer, SyncOracle,
};
use sp_consensus_aura::{digests::CompatibleDigestItem, AuraApi, SlotDuration};
use sp_consensus_slots::Slot;
use sp_core::crypto::{ByteArray, Pair, Public};
use sp_inherents::CreateInherentDataProviders;
use sp_keystore::{SyncCryptoStore, SyncCryptoStorePtr};
use sp_runtime::{
    traits::{Block as BlockT, Header as HeaderT, Member, NumberFor},
    DigestItem,
};
use std::{convert::TryFrom, fmt::Debug, hash::Hash, marker::PhantomData, pin::Pin, sync::Arc};

mod manual_seal;

pub use manual_seal::OrchestratorManualSealAuraConsensusDataProvider;
pub use sc_consensus_aura::{slot_duration, AuraVerifier, BuildAuraWorkerParams, SlotProportion};
pub use sc_consensus_slots::InherentDataProviderExt;

const LOG_TARGET: &str = "aura::orchestrator";

/// The implementation of the Orchestrator AURA consensus for parachains.
pub struct OrchestratorAuraConsensus<B, CIDP, W> {
    create_inherent_data_providers: Arc<CIDP>,
    aura_worker: Arc<Mutex<W>>,
    slot_duration: SlotDuration,
    _phantom: PhantomData<B>,
}

impl<B, CIDP, W> Clone for OrchestratorAuraConsensus<B, CIDP, W> {
    fn clone(&self) -> Self {
        Self {
            create_inherent_data_providers: self.create_inherent_data_providers.clone(),
            aura_worker: self.aura_worker.clone(),
            slot_duration: self.slot_duration,
            _phantom: PhantomData,
        }
    }
}

/// Build the Orchestrator aura worker.
///
/// The caller is responsible for running this worker, otherwise it will do nothing.
pub fn build_orchestrator_aura_worker<P, B, C, PF, I, SO, L, BS, Error>(
    BuildAuraWorkerParams {
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
    }: BuildAuraWorkerParams<C, I, PF, SO, L, BS, NumberFor<B>>,
) -> impl sc_consensus_slots::SimpleSlotWorker<
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
    C::Api: AuraApi<B, AuthorityId<P>>,
    PF: Environment<B, Error = Error> + Send + Sync + 'static,
    PF::Proposer: Proposer<B, Error = Error, Transaction = sp_api::TransactionFor<C, B>>,
    P: Pair + Send + Sync,
    P::Public: AppPublic + Public + Member + Encode + Decode + Hash + Into<NimbusId>,
    P::Signature: TryFrom<Vec<u8>> + Hash + Member + Encode + Decode,
    I: BlockImport<B, Transaction = sp_api::TransactionFor<C, B>> + Send + Sync + 'static,
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
pub struct BuildOrchestratorAuraConsensusParams<PF, BI, CIDP, Client, BS, SO> {
    pub proposer_factory: PF,
    pub create_inherent_data_providers: CIDP,
    pub block_import: BI,
    pub para_client: Arc<Client>,
    pub backoff_authoring_blocks: Option<BS>,
    pub sync_oracle: SO,
    pub keystore: SyncCryptoStorePtr,
    pub force_authoring: bool,
    pub slot_duration: SlotDuration,
    pub telemetry: Option<TelemetryHandle>,
    pub block_proposal_slot_portion: SlotProportion,
    pub max_block_proposal_slot_portion: Option<SlotProportion>,
}

impl<B, CIDP> OrchestratorAuraConsensus<B, CIDP, ()>
where
    B: BlockT,
    CIDP: CreateInherentDataProviders<B, (PHash, PersistedValidationData)> + 'static,
    CIDP::InherentDataProviders: InherentDataProviderExt,
{
    /// Create a new boxed instance of AURA consensus.
    pub fn build<P, Client, BI, SO, PF, BS, Error>(
        BuildOrchestratorAuraConsensusParams {
            proposer_factory,
            create_inherent_data_providers,
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
        }: BuildOrchestratorAuraConsensusParams<PF, BI, CIDP, Client, BS, SO>,
    ) -> Box<dyn ParachainConsensus<B>>
    where
        Client:
            ProvideRuntimeApi<B> + BlockOf + AuxStore + HeaderBackend<B> + Send + Sync + 'static,
        Client::Api: AuraApi<B, P::Public>,
        BI: BlockImport<B, Transaction = sp_api::TransactionFor<Client, B>>
            + ParachainBlockImportMarker
            + Send
            + Sync
            + 'static,
        SO: SyncOracle + Send + Sync + Clone + 'static,
        BS: BackoffAuthoringBlocksStrategy<NumberFor<B>> + Send + Sync + 'static,
        PF: Environment<B, Error = Error> + Send + Sync + 'static,
        PF::Proposer: Proposer<
            B,
            Error = Error,
            Transaction = sp_api::TransactionFor<Client, B>,
            ProofRecording = EnableProofRecording,
            Proof = <EnableProofRecording as ProofRecording>::Proof,
        >,
        Error: std::error::Error + Send + From<sp_consensus::Error> + 'static,
        P: Pair + Send + Sync,
        P::Public: AppPublic + Public + Member + Encode + Decode + Hash + Into<NimbusId>,
        P::Signature: TryFrom<Vec<u8>> + Hash + Member + Encode + Decode,
    {
        let worker =
            build_orchestrator_aura_worker::<P, _, _, _, _, _, _, _, _>(BuildAuraWorkerParams {
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
            });

        Box::new(OrchestratorAuraConsensus {
            create_inherent_data_providers: Arc::new(create_inherent_data_providers),
            aura_worker: Arc::new(Mutex::new(worker)),
            slot_duration,
            _phantom: PhantomData,
        })
    }
}

impl<B, CIDP, W> OrchestratorAuraConsensus<B, CIDP, W>
where
    B: BlockT,
    CIDP: CreateInherentDataProviders<B, (PHash, PersistedValidationData)> + 'static,
    CIDP::InherentDataProviders: InherentDataProviderExt,
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
impl<B, CIDP, W> ParachainConsensus<B> for OrchestratorAuraConsensus<B, CIDP, W>
where
    B: BlockT,
    CIDP: CreateInherentDataProviders<B, (PHash, PersistedValidationData)> + Send + Sync + 'static,
    CIDP::InherentDataProviders: InherentDataProviderExt + Send,
    W: SimpleSlotWorker<B> + Send + Sync,
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

        let res = self.aura_worker.lock().await.on_slot(info).await?;

        Some(ParachainCandidate {
            block: res.block,
            proof: res.storage_proof,
        })
    }
}

type AuthorityId<P> = <P as Pair>::Public;

struct OrchestratorAuraWorker<C, E, I, P, SO, L, BS, N> {
    client: Arc<C>,
    block_import: I,
    env: E,
    keystore: SyncCryptoStorePtr,
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
    C::Api: AuraApi<B, AuthorityId<P>>,
    E: Environment<B, Error = Error> + Send + Sync,
    E::Proposer: Proposer<B, Error = Error, Transaction = sp_api::TransactionFor<C, B>>,
    I: BlockImport<B, Transaction = sp_api::TransactionFor<C, B>> + Send + Sync + 'static,
    P: Pair + Send + Sync,
    P::Public: AppPublic + Public + Member + Encode + Decode + Hash + Into<NimbusId>,
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
        "orchestrator_aura"
    }

    fn block_import(&mut self) -> &mut Self::BlockImport {
        &mut self.block_import
    }

    fn aux_data(
        &self,
        header: &B::Header,
        _slot: Slot,
    ) -> Result<Self::AuxData, sp_consensus::Error> {
        authorities(
            self.client.as_ref(),
            header.hash(),
            *header.number() + 1u32.into(),
            &self.compatibility_mode,
        )
    }

    fn authorities_len(&self, epoch_data: &Self::AuxData) -> Option<usize> {
        Some(epoch_data.len())
    }

    async fn claim_slot(
        &self,
        _header: &B::Header,
        slot: Slot,
        epoch_data: &Self::AuxData,
    ) -> Option<Self::Claim> {
        let expected_author = slot_author::<P>(slot, epoch_data);
        expected_author.and_then(|p| {
            if SyncCryptoStore::has_keys(
                &*self.keystore,
                &[(p.to_raw_vec(), sp_application_crypto::key_types::AURA)],
            ) {
                Some(p.clone())
            } else {
                None
            }
        })
    }

    fn pre_digest_data(&self, slot: Slot, claim: &Self::Claim) -> Vec<sp_runtime::DigestItem> {
        vec![
            <DigestItem as CompatibleDigestItem<P::Signature>>::aura_pre_digest(slot),
            // We inject the nimbus digest as well. Crutial to be able to verify signatures
            <DigestItem as NimbusCompatibleDigestItem>::nimbus_pre_digest(claim.clone().into()),
        ]
    }

    async fn block_import_params(
        &self,
        header: B::Header,
        header_hash: &B::Hash,
        body: Vec<B::Extrinsic>,
        storage_changes: StorageChanges<<Self::BlockImport as BlockImport<B>>::Transaction, B>,
        public: Self::Claim,
        _epoch: Self::AuxData,
    ) -> Result<
        sc_consensus::BlockImportParams<B, <Self::BlockImport as BlockImport<B>>::Transaction>,
        sp_consensus::Error,
    > {
        // sign the pre-sealed hash of the block and then
        // add it to a digest item.
        let public_type_pair = public.to_public_crypto_pair();
        let public = public.to_raw_vec();
        log::info!("the ID is {:?}", <AuthorityId<P> as AppKey>::ID);
        let signature = SyncCryptoStore::sign_with(
            &*self.keystore,
            <AuthorityId<P> as AppKey>::ID,
            &public_type_pair,
            header_hash.as_ref(),
        )
        .map_err(|e| sp_consensus::Error::CannotSign(public.clone(), e.to_string()))?
        .ok_or_else(|| {
            sp_consensus::Error::CannotSign(
                public.clone(),
                "Could not find key in keystore.".into(),
            )
        })?;
        let signature = signature
            .clone()
            .try_into()
            .map_err(|_| sp_consensus::Error::InvalidSignature(signature, public))?;

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

fn authorities<A, B, C>(
    client: &C,
    parent_hash: B::Hash,
    context_block_number: NumberFor<B>,
    compatibility_mode: &CompatibilityMode<NumberFor<B>>,
) -> Result<Vec<A>, ConsensusError>
where
    A: Codec + Debug,
    B: BlockT,
    C: ProvideRuntimeApi<B>,
    C::Api: AuraApi<B, A>,
{
    let runtime_api = client.runtime_api();

    match compatibility_mode {
        CompatibilityMode::None => {}
        // Use `initialize_block` until we hit the block that should disable the mode.
        CompatibilityMode::UseInitializeBlock { until } => {
            if *until > context_block_number {
                runtime_api
                    .initialize_block(
                        parent_hash,
                        &B::Header::new(
                            context_block_number,
                            Default::default(),
                            Default::default(),
                            parent_hash,
                            Default::default(),
                        ),
                    )
                    .map_err(|_| sp_consensus::Error::InvalidAuthoritiesSet)?;
            }
        }
    }

    runtime_api
        .authorities(parent_hash)
        .ok()
        .ok_or(sp_consensus::Error::InvalidAuthoritiesSet)
}

/// Get slot author for given block along with authorities.
pub(crate) fn slot_author<P: Pair>(
    slot: Slot,
    authorities: &[AuthorityId<P>],
) -> Option<&AuthorityId<P>> {
    if authorities.is_empty() {
        return None;
    }

    let idx = *slot % (authorities.len() as u64);
    assert!(
        idx <= usize::MAX as u64,
        "It is impossible to have a vector with length beyond the address space; qed",
    );

    let current_author = authorities.get(idx as usize).expect(
        "authorities not empty; index constrained to list length;this is a valid index; qed",
    );

    Some(current_author)
}
