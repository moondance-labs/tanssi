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

//! A collator for Tanssi Aura that looks ahead of the most recently included parachain block
//! when determining what to build upon.
//!
//! This collator also builds additional blocks when the maximum backlog is not saturated.
//! The size of the backlog is determined by invoking a runtime API. If that runtime API
//! is not supported, this assumes a maximum backlog size of 1.
//!
//! This takes more advantage of asynchronous backing, though not complete advantage.
//! When the backlog is not saturated, this approach lets the backlog temporarily 'catch up'
//! with periods of higher throughput. When the backlog is saturated, we typically
//! fall back to the limited cadence of a single parachain block per relay-chain block.
//!
//! Despite this, the fact that there is a backlog at all allows us to spend more time
//! building the block, as there is some buffer before it can get posted to the relay-chain.
//! The main limitation is block propagation time - i.e. the new blocks created by an author
//! must be propagated to the next author before their turn.

use {
    crate::{
        collators::{self as collator_util, tanssi_claim_slot, ClaimMode, SlotClaim},
        consensus_orchestrator::RetrieveAuthoritiesFromOrchestrator,
        OrchestratorAuraWorkerAuxData,
    },
    async_backing_primitives::UnincludedSegmentApi,
    cumulus_client_collator::service::ServiceInterface as CollatorServiceInterface,
    cumulus_client_consensus_common::{
        self as consensus_common, load_abridged_host_configuration, ParentSearchParams,
    },
    cumulus_client_consensus_proposer::ProposerInterface,
    cumulus_primitives_core::{
        relay_chain::{AsyncBackingParams, CoreIndex, CoreState, Hash as PHash},
        PersistedValidationData,
    },
    cumulus_relay_chain_interface::RelayChainInterface,
    futures::{channel::oneshot, prelude::*},
    nimbus_primitives::NimbusId,
    pallet_xcm_core_buyer_runtime_api::{BuyingError, XCMCoreBuyerApi},
    parity_scale_codec::{Codec, Encode},
    polkadot_node_primitives::SubmitCollationParams,
    polkadot_node_subsystem::messages::{
        CollationGenerationMessage, RuntimeApiMessage, RuntimeApiRequest,
    },
    polkadot_overseer::Handle as OverseerHandle,
    polkadot_primitives::{CollatorPair, Id as ParaId, OccupiedCoreAssumption},
    sc_client_api::{backend::AuxStore, BlockBackend, BlockOf},
    sc_consensus::BlockImport,
    sc_consensus_slots::InherentDataProviderExt,
    sc_transaction_pool_api::TransactionPool,
    sp_api::{ApiError, ProvideRuntimeApi},
    sp_blockchain::{HeaderBackend, HeaderMetadata},
    sp_consensus::SyncOracle,
    sp_consensus_aura::{Slot, SlotDuration},
    sp_core::crypto::Pair,
    sp_inherents::CreateInherentDataProviders,
    sp_keystore::KeystorePtr,
    sp_runtime::{
        traits::{Block as BlockT, BlockIdTo, Header as HeaderT, Member},
        transaction_validity::TransactionSource,
    },
    sp_transaction_pool::runtime_api::TaggedTransactionQueue,
    std::{convert::TryFrom, error::Error, sync::Arc, time::Duration},
    tokio::select,
    tokio_util::sync::CancellationToken,
    tp_xcm_core_buyer::{BuyCollatorProofCreationError, BuyCoreCollatorProof},
};

#[derive(Debug)]
pub enum BuyCoreError<BlockNumber: std::fmt::Debug, PoolError: std::fmt::Debug> {
    NotAParathread,
    UnableToClaimSlot,
    UnableToFindKeyForSigning,
    SlotDriftConversionOverflow,
    ApiError(ApiError),
    BuyingValidationError(BuyingError<BlockNumber>),
    UnableToCreateProof(BuyCollatorProofCreationError),
    TxSubmissionError(PoolError),
}

impl<BlockNumber: std::fmt::Debug, PoolError: std::fmt::Debug>
    BuyCoreError<BlockNumber, PoolError>
{
    fn log_error<Blockhash: std::fmt::Debug>(
        &self,
        slot: Slot,
        para_id: ParaId,
        relay_parent: Blockhash,
    ) {
        match self {
            BuyCoreError::NotAParathread => {
                tracing::trace!(
                    target: crate::LOG_TARGET,
                    ?relay_parent,
                    ?para_id,
                    ?slot,
                    "Para is not a parathread, skipping an attempt to buy core",
                );
            }
            BuyCoreError::UnableToClaimSlot => {
                tracing::trace!(
                    target: crate::LOG_TARGET,
                    ?relay_parent,
                    ?para_id,
                    ?slot,
                    "Unable to claim slot for parathread, skipping attempt to buy the core.",
                );
            }
            BuyCoreError::UnableToFindKeyForSigning => {
                tracing::error!(
                    target: crate::LOG_TARGET,
                    ?relay_parent,
                    ?para_id,
                    ?slot,
                    "Unable to generate buy core proof as unable to find corresponding key",
                );
            }
            BuyCoreError::SlotDriftConversionOverflow => {
                tracing::error!(
                    target: crate::LOG_TARGET,
                    ?relay_parent,
                    ?para_id,
                    ?slot,
                    "Unable to calculate container chain slot drift from orchestrator chain's slot drift",
                );
            }
            BuyCoreError::ApiError(api_error) => {
                tracing::error!(
                    target: crate::LOG_TARGET,
                    ?relay_parent,
                    ?para_id,
                    ?slot,
                    ?api_error,
                    "Unable to call orchestrator runtime api",
                );
            }
            BuyCoreError::BuyingValidationError(buying_error) => {
                tracing::error!(
                    target: crate::LOG_TARGET,
                    ?relay_parent,
                    ?para_id,
                    ?buying_error,
                    ?slot,
                    "Buying core is not allowed right now",
                );
            }
            BuyCoreError::UnableToCreateProof(proof_error) => {
                tracing::error!(
                    target: crate::LOG_TARGET,
                    ?relay_parent,
                    ?para_id,
                    ?slot,
                    ?proof_error,
                    "Unable to generate buy core proof due to an error",
                );
            }
            BuyCoreError::TxSubmissionError(pool_error) => {
                tracing::error!(
                    target: crate::LOG_TARGET,
                    ?relay_parent,
                    ?para_id,
                    ?slot,
                    ?pool_error,
                    "Unable to send buy core unsigned extrinsic through orchestrator tx pool",
                );
            }
        }
    }
}

impl<BlockNumber: std::fmt::Debug, PoolError: std::fmt::Debug> From<BuyingError<BlockNumber>>
    for BuyCoreError<BlockNumber, PoolError>
{
    fn from(buying_error: BuyingError<BlockNumber>) -> Self {
        BuyCoreError::BuyingValidationError(buying_error)
    }
}

impl<BlockNumber: std::fmt::Debug, PoolError: std::fmt::Debug> From<ApiError>
    for BuyCoreError<BlockNumber, PoolError>
{
    fn from(api_error: ApiError) -> Self {
        BuyCoreError::ApiError(api_error)
    }
}

impl<BlockNumber: std::fmt::Debug, PoolError: std::fmt::Debug> From<BuyCollatorProofCreationError>
    for BuyCoreError<BlockNumber, PoolError>
{
    fn from(proof_creation_error: BuyCollatorProofCreationError) -> Self {
        BuyCoreError::UnableToCreateProof(proof_creation_error)
    }
}

pub async fn try_to_buy_core<Block, OBlock, OBlockNumber, P, CIDP, TxPool, OClient>(
    para_id: ParaId,
    aux_data: OrchestratorAuraWorkerAuxData<P>,
    inherent_providers: CIDP::InherentDataProviders,
    keystore: &KeystorePtr,
    orchestrator_client: Arc<OClient>,
    orchestrator_tx_pool: Arc<TxPool>,
    parent_header: <Block as BlockT>::Header,
    orchestrator_slot_duration: SlotDuration,
    container_slot_duration: SlotDuration,
) -> Result<
    <TxPool as TransactionPool>::Hash,
    BuyCoreError<
        <<OBlock as BlockT>::Header as HeaderT>::Number,
        <TxPool as TransactionPool>::Error,
    >,
>
where
    Block: BlockT,
    OBlock: BlockT,
    P: Pair<Public = NimbusId> + Sync + Send + 'static,
    P::Signature: TryFrom<Vec<u8>> + Member + Codec,
    CIDP: CreateInherentDataProviders<Block, (PHash, PersistedValidationData)>
        + Send
        + 'static
        + Clone,
    CIDP::InherentDataProviders: Send + InherentDataProviderExt,
    OClient: ProvideRuntimeApi<OBlock>
        + HeaderMetadata<OBlock, Error = sp_blockchain::Error>
        + HeaderBackend<OBlock>
        + BlockBackend<OBlock>
        + BlockIdTo<OBlock>
        + 'static,
    OClient::Api: TaggedTransactionQueue<OBlock>
        + XCMCoreBuyerApi<OBlock, <<OBlock as BlockT>::Header as HeaderT>::Number, ParaId, NimbusId>,
    TxPool: TransactionPool<Block = OBlock>,
{
    // We do nothing if this is not a parathread
    if aux_data.slot_freq.is_none() {
        return Err(BuyCoreError::NotAParathread);
    }

    let orchestrator_best_hash = orchestrator_client.info().best_hash;
    let orchestrator_runtime_api = orchestrator_client.runtime_api();

    let buy_core_slot_drift = orchestrator_client
        .runtime_api()
        .get_buy_core_slot_drift(orchestrator_best_hash)?;

    // Convert drift in terms of container chain slots for parity between client side calculation and
    // orchestrator runtime calculation
    // We tried to made this calculation as generic as possible so that it can handle
    // arbitrary slot timings as well and won't assume anything.
    // Formula is: (Slot_drift_in_orchestrator_slot * orchestrator_slot_duration) / container_slot_duration
    let buy_core_container_slot_drift = buy_core_slot_drift
        .checked_mul(orchestrator_slot_duration.as_millis())
        .and_then(|intermediate_result| {
            intermediate_result.checked_div(container_slot_duration.as_millis())
        })
        .ok_or(BuyCoreError::SlotDriftConversionOverflow)?;

    let current_container_slot = inherent_providers.slot();

    let slot_claim = tanssi_claim_slot::<P, Block>(
        aux_data,
        &parent_header,
        current_container_slot,
        ClaimMode::ParathreadCoreBuying {
            drift_permitted: buy_core_container_slot_drift.into(),
        },
        keystore,
    )
    .ok_or(BuyCoreError::UnableToClaimSlot)?;

    let pubkey = slot_claim.author_pub;

    orchestrator_runtime_api.is_core_buying_allowed(
        orchestrator_best_hash,
        para_id,
        pubkey.clone(),
    )??;

    let nonce =
        orchestrator_runtime_api.get_buy_core_signature_nonce(orchestrator_best_hash, para_id)?;

    let collator_buy_core_proof =
        BuyCoreCollatorProof::new_with_keystore(nonce, para_id, pubkey, keystore)?
            .ok_or(BuyCoreError::UnableToFindKeyForSigning)?;

    let extrinsic = orchestrator_runtime_api.create_buy_core_unsigned_extrinsic(
        orchestrator_best_hash,
        para_id,
        collator_buy_core_proof,
    )?;

    orchestrator_tx_pool
        .submit_one(orchestrator_best_hash, TransactionSource::Local, *extrinsic)
        .await
        .map_err(BuyCoreError::TxSubmissionError)
}

/// Parameters for [`run`].
pub struct Params<
    GSD,
    BI,
    CIDP,
    Client,
    Backend,
    RClient,
    CHP,
    SO,
    Proposer,
    CS,
    GOH,
    TxPool,
    OClient,
> {
    pub get_current_slot_duration: GSD,
    pub create_inherent_data_providers: CIDP,
    pub get_orchestrator_aux_data: GOH,
    pub block_import: BI,
    pub para_client: Arc<Client>,
    pub para_backend: Arc<Backend>,
    pub relay_client: RClient,
    pub code_hash_provider: CHP,
    pub sync_oracle: SO,
    pub keystore: KeystorePtr,
    pub collator_key: CollatorPair,
    pub para_id: ParaId,
    pub overseer_handle: OverseerHandle,
    pub orchestrator_slot_duration: SlotDuration,
    pub relay_chain_slot_duration: Duration,
    pub proposer: Proposer,
    pub collator_service: CS,
    pub authoring_duration: Duration,
    pub force_authoring: bool,
    pub cancellation_token: CancellationToken,
    pub buy_core_params: BuyCoreParams<TxPool, OClient>,
}

pub enum BuyCoreParams<TxPool, OClient> {
    Orchestrator {
        orchestrator_tx_pool: Arc<TxPool>,
        orchestrator_client: Arc<OClient>,
    },
    Solochain {
        // TODO: relay_tx_pool
    },
}

impl<TxPool, OClient> Clone for BuyCoreParams<TxPool, OClient> {
    fn clone(&self) -> Self {
        match self {
            Self::Orchestrator {
                orchestrator_tx_pool,
                orchestrator_client,
            } => Self::Orchestrator {
                orchestrator_tx_pool: orchestrator_tx_pool.clone(),
                orchestrator_client: orchestrator_client.clone(),
            },
            Self::Solochain {} => Self::Solochain {},
        }
    }
}

impl<TxPool, OClient> BuyCoreParams<TxPool, OClient> {
    pub fn is_solochain(&self) -> bool {
        match self {
            BuyCoreParams::Solochain { .. } => true,
            _ => false,
        }
    }
}

/// Run async-backing-friendly for Tanssi Aura.
pub fn run<
    GSD,
    Block,
    P,
    BI,
    CIDP,
    Client,
    Backend,
    RClient,
    CHP,
    SO,
    Proposer,
    CS,
    GOH,
    TxPool,
    OClient,
    OBlock,
>(
    mut params: Params<
        GSD,
        BI,
        CIDP,
        Client,
        Backend,
        RClient,
        CHP,
        SO,
        Proposer,
        CS,
        GOH,
        TxPool,
        OClient,
    >,
) -> (
    impl Future<Output = ()> + Send + 'static,
    oneshot::Receiver<()>,
)
where
    Block: BlockT,
    Client: ProvideRuntimeApi<Block>
        + BlockOf
        + AuxStore
        + HeaderBackend<Block>
        + BlockBackend<Block>
        + Send
        + Sync
        + 'static,
    Client::Api: UnincludedSegmentApi<Block>,
    Backend: sc_client_api::Backend<Block> + 'static,
    RClient: RelayChainInterface + Clone + 'static,
    CIDP: CreateInherentDataProviders<Block, (PHash, PersistedValidationData)>
        + Send
        + 'static
        + Clone,
    CIDP::InherentDataProviders: Send + InherentDataProviderExt,
    BI: BlockImport<Block> + Send + Sync + 'static,
    SO: SyncOracle + Send + Sync + Clone + 'static,
    Proposer: ProposerInterface<Block> + Send + Sync + 'static,
    CS: CollatorServiceInterface<Block> + Send + Sync + 'static,
    CHP: consensus_common::ValidationCodeHashProvider<Block::Hash> + Send + 'static,
    P: Pair<Public = NimbusId> + Sync + Send + 'static,
    P::Signature: TryFrom<Vec<u8>> + Member + Codec,
    GOH: RetrieveAuthoritiesFromOrchestrator<
            Block,
            (PHash, PersistedValidationData),
            OrchestratorAuraWorkerAuxData<P>,
        >
        + 'static
        + Sync
        + Send,
    OBlock: BlockT,
    OClient: ProvideRuntimeApi<OBlock>
        + HeaderMetadata<OBlock, Error = sp_blockchain::Error>
        + HeaderBackend<OBlock>
        + BlockBackend<OBlock>
        + BlockIdTo<OBlock>
        + 'static,
    OClient::Api: TaggedTransactionQueue<OBlock>
        + XCMCoreBuyerApi<OBlock, <<OBlock as BlockT>::Header as HeaderT>::Number, ParaId, NimbusId>,
    TxPool: TransactionPool<Block = OBlock> + 'static,
    GSD: Fn(<Block as BlockT>::Hash) -> SlotDuration + Send + 'static,
{
    // This is an arbitrary value which is likely guaranteed to exceed any reasonable
    // limit, as it would correspond to 10 non-included blocks.
    //
    // Since we only search for parent blocks which have already been imported,
    // we can guarantee that all imported blocks respect the unincluded segment
    // rules specified by the parachain's runtime and thus will never be too deep.
    const PARENT_SEARCH_DEPTH: usize = 10;

    let (exit_notification_sender, exit_notification_receiver) = oneshot::channel();

    let aura_fut = async move {
        cumulus_client_collator::initialize_collator_subsystems(
            &mut params.overseer_handle,
            params.collator_key,
            params.para_id,
            true,
        )
        .await;

        let mut import_notifications = match params.relay_client.import_notification_stream().await
        {
            Ok(s) => s,
            Err(err) => {
                tracing::error!(
                    target: crate::LOG_TARGET,
                    ?err,
                    "Failed to initialize consensus: no relay chain import notification stream"
                );

                return;
            }
        };

        let mut collator = {
            let params = collator_util::Params {
                create_inherent_data_providers: params.create_inherent_data_providers.clone(),
                block_import: params.block_import,
                relay_client: params.relay_client.clone(),
                keystore: params.keystore.clone(),
                para_id: params.para_id,
                proposer: params.proposer,
                collator_service: params.collator_service,
            };

            collator_util::Collator::<Block, P, _, _, _, _, _>::new(params)
        };

        loop {
            select! {
                maybe_relay_parent_header = import_notifications.next() => {
                    if maybe_relay_parent_header.is_none() {
                        break;
                    }

                    let relay_parent_header = maybe_relay_parent_header.expect("relay_parent_header must exists as we checked for None variant above; qed");
                    let relay_parent = relay_parent_header.hash();

                    let max_pov_size = match params
                        .relay_client
                        .persisted_validation_data(
                            relay_parent,
                            params.para_id,
                            OccupiedCoreAssumption::Included,
                        )
                        .await
                    {
                        Ok(None) => continue,
                        Ok(Some(pvd)) => pvd.max_pov_size,
                        Err(err) => {
                            tracing::error!(target: crate::LOG_TARGET, ?err, "Failed to gather information from relay-client");
                            continue;
                        }
                    };

                    let parent_search_params = ParentSearchParams {
                        relay_parent,
                        para_id: params.para_id,
                        ancestry_lookback: max_ancestry_lookback(relay_parent, &params.relay_client).await,
                        max_depth: PARENT_SEARCH_DEPTH,
                        ignore_alternative_branches: true,
                    };

                    let potential_parents =
                        cumulus_client_consensus_common::find_potential_parents::<Block>(
                            parent_search_params,
                            &*params.para_backend,
                            &params.relay_client,
                        )
                        .await;

                    let mut potential_parents = match potential_parents {
                        Err(e) => {
                            tracing::error!(
                                target: crate::LOG_TARGET,
                                ?relay_parent,
                                err = ?e,
                                "Could not fetch potential parents to build upon"
                            );

                            continue;
                        }
                        Ok(x) => x,
                    };

                    let included_block = match potential_parents.iter().find(|x| x.depth == 0) {
                        None => continue, // also serves as an `is_empty` check.
                        Some(b) => b.hash,
                    };

                    let para_client = &*params.para_client;
                    let keystore = &params.keystore;
                    let can_build_upon = |slot_now, block_hash, aux_data| {
                        can_build_upon::<_, _, P>(
                            slot_now,
                            aux_data,
                            block_hash,
                            included_block,
                            params.force_authoring,
                            para_client,
                            keystore,
                        )
                    };

                    // Sort by depth, ascending, to choose the longest chain.
                    //
                    // If the longest chain has space, build upon that. Otherwise, don't
                    // build at all.
                    potential_parents.sort_by_key(|a| a.depth);
                    let initial_parent = match potential_parents.pop() {
                        None => continue,
                        Some(p) => p,
                    };

                    // Build in a loop until not allowed. Note that the authorities can change
                    // at any block, so we need to re-claim our slot every time.
                    let mut parent_hash = initial_parent.hash;
                    let mut parent_header = initial_parent.header;

                    let core_indices = cores_scheduled_for_para(
                        relay_parent,
                        params.para_id,
                        &mut params.overseer_handle,
                        &params.relay_client,
                    )
                    .await;

                    let overseer_handle = &mut params.overseer_handle;

                    // This needs to change to support elastic scaling, but for continuously
                    // scheduled chains this ensures that the backlog will grow steadily.
                    for n_built in 0..2 {
                        let validation_data = PersistedValidationData {
                            parent_head: parent_header.encode().into(),
                            relay_parent_number: *relay_parent_header.number(),
                            relay_parent_storage_root: *relay_parent_header.state_root(),
                            max_pov_size,
                        };

                        // Retrieve authorities that are able to produce the block
                        let aux_data = match params
                            .get_orchestrator_aux_data
                            .retrieve_authorities_from_orchestrator(
                                parent_hash,
                                (relay_parent_header.hash(), validation_data.clone()),
                            )
                            .await
                        {
                            Err(e) => {
                                tracing::error!(target: crate::LOG_TARGET, ?e);
                                break;
                            }
                            Ok(h) => h,
                        };

                        let inherent_providers = match params
                            .create_inherent_data_providers
                            .create_inherent_data_providers(
                                parent_hash,
                                (relay_parent_header.hash(), validation_data.clone()),
                            )
                            .await
                        {
                            Err(e) => {
                                tracing::error!(target: crate::LOG_TARGET, ?e);
                                break;
                            }
                            Ok(h) => h,
                        };

                        // TODO: Currently we use just the first core here, but for elastic scaling
                        // we iterate and build on all of the cores returned.
                        // More info: https://github.com/paritytech/polkadot-sdk/issues/1829
                        let (is_parachain, core_index) = match (&aux_data.slot_freq, core_indices.first()) {
                            (None, None) => {
                                tracing::warn!(target: crate::LOG_TARGET, para_id = ?params.para_id, "We are parachain and we do not have core allocated, nothing to do");
                                break;
                            }, // We are parachain and we do not have core allocated, nothing to do,
                            (None, Some(core_index)) => {
                                tracing::trace!(target: crate::LOG_TARGET, para_id = ?params.para_id, ?core_index, "We are parachain and we core allocated, let's collate the block");
                                (true, core_index)
                            }, // We are parachain and we have core allocated, let's continue
                            (Some(_slot_frequency), None) => { // We are parathread and core is not allocated. Let's try to buy core
                                tracing::trace!(target: crate::LOG_TARGET, para_id = ?params.para_id, "We are parathread and we do not have core allocated, let's try to buy the core");
                                let slot = inherent_providers.slot();
                                let container_chain_slot_duration = (params.get_current_slot_duration)(parent_header.hash());

                                let buy_core_result = match &params.buy_core_params {
                                    BuyCoreParams::Orchestrator {
                                        orchestrator_client,
                                        orchestrator_tx_pool,
                                    } => {
                                        try_to_buy_core::<_, _, <<OBlock as BlockT>::Header as HeaderT>::Number, _, CIDP, _, _>(params.para_id, aux_data, inherent_providers, &params.keystore, orchestrator_client.clone(), orchestrator_tx_pool.clone(), parent_header, params.orchestrator_slot_duration, container_chain_slot_duration).await
                                    }
                                    BuyCoreParams::Solochain {

                                    } => {
                                        // TODO: implement parathread support for solochain
                                        log::warn!("Unimplemented: cannot buy core for parathread in solochain");
                                        break;
                                    }
                                };
                                match buy_core_result {
                                    Ok(block_hash) => {
                                        tracing::trace!(target: crate::LOG_TARGET, ?block_hash, "Sent unsigned extrinsic to buy the core");
                                    },
                                    Err(buy_core_error) => {
                                        buy_core_error.log_error(slot, params.para_id, relay_parent);
                                    }
                                };
                                break; // No point in continuing as we need to wait for few relay blocks in order for our core to be available.
                            },
                            (Some(_slot_frequency), Some(core_index)) => { // We are parathread and we do have core, let's continue
                                tracing::trace!(target: crate::LOG_TARGET, ?core_index, "We are parathread and we do have core allocated, let's collate the block");
                                (false, core_index)
                            }
                        };

                        let mut slot_claim = match can_build_upon(
                            inherent_providers.slot(),
                            parent_header.clone(),
                            aux_data,
                        )
                        .await
                        {
                            Ok(None) => break,
                            Err(e) => {
                                tracing::error!(target: crate::LOG_TARGET, ?e);
                                break;
                            }
                            Ok(Some(c)) => c,
                        };

                        tracing::debug!(
                            target: crate::LOG_TARGET,
                            ?relay_parent,
                            unincluded_segment_len = initial_parent.depth + n_built,
                            "Slot claimed. Building"
                        );

                        // Build and announce collations recursively until
                        // `can_build_upon` fails or building a collation fails.
                        let (parachain_inherent_data, other_inherent_data) = match collator
                            .create_inherent_data(relay_parent, &validation_data, parent_hash, None)
                            .await
                        {
                            Err(err) => {
                                tracing::error!(target: crate::LOG_TARGET, ?err);
                                break;
                            }
                            Ok(x) => x,
                        };

                        let validation_code_hash = match params.code_hash_provider.code_hash_at(parent_hash)
                        {
                            None => {
                                tracing::error!(target: crate::LOG_TARGET, ?parent_hash, "Could not fetch validation code hash");
                                break;
                            }
                            Some(v) => v,
                        };

                        match collator
                            .collate(
                                &parent_header,
                                &mut slot_claim,
                                None,
                                (parachain_inherent_data, other_inherent_data),
                                params.authoring_duration,
                                // Set the block limit to 50% of the maximum PoV size.
                                //
                                // TODO: If we got benchmarking that includes the proof size,
                                // we should be able to use the maximum pov size.
                                (validation_data.max_pov_size / 2) as usize,
                            )
                            .await
                        {
                            Ok(Some((collation, block_data, new_block_hash))) => {
                                // Here we are assuming that the import logic protects against equivocations
                                // and provides sybil-resistance, as it should.
                                collator
                                    .collator_service()
                                    .announce_block(new_block_hash, None);

                                // Send a submit-collation message to the collation generation subsystem,
                                // which then distributes this to validators.
                                //
                                // Here we are assuming that the leaf is imported, as we've gotten an
                                // import notification.
                                overseer_handle
                                    .send_msg(
                                        CollationGenerationMessage::SubmitCollation(
                                            SubmitCollationParams {
                                                relay_parent,
                                                collation,
                                                parent_head: parent_header.encode().into(),
                                                validation_code_hash,
                                                result_sender: None,
                                                core_index: *core_index
                                            },
                                        ),
                                        "SubmitCollation",
                                    )
                                    .await;

                                parent_hash = new_block_hash;
                                parent_header = block_data.into_header();
                            }
                            Ok(None) => {
                                tracing::debug!(target: crate::LOG_TARGET, "Lookahead collator: No block proposal");
                            }
                            Err(err) => {
                                tracing::error!(target: crate::LOG_TARGET, ?err);
                                break;
                            }
                        }

                        // If it is parathread, no point in async backing as we would have to do
                        // buy core first
                        if !is_parachain {
                            tracing::trace!(target: crate::LOG_TARGET, "Not a parachain so terminated at {:?}", n_built);
                            break;
                        }
                    }
                },
                _ = params.cancellation_token.cancelled() => {
                    log::info!("Stopping lookahead collator");
                    break;
                }
            }
        }

        // Notifying that we have exited
        let _ = exit_notification_sender.send(());
    };

    (aura_fut, exit_notification_receiver)
}

// Checks if we own the slot at the given block and whether there
// is space in the unincluded segment.
async fn can_build_upon<Block: BlockT, Client, P>(
    slot: Slot,
    aux_data: OrchestratorAuraWorkerAuxData<P>,
    parent_header: Block::Header,
    included_block: <Block as BlockT>::Hash,
    force_authoring: bool,
    client: &Client,
    keystore: &KeystorePtr,
) -> Result<Option<SlotClaim<P::Public>>, Box<dyn Error>>
where
    Client: ProvideRuntimeApi<Block>,
    Client::Api: UnincludedSegmentApi<Block>,
    P: Pair + Send + Sync + 'static,
    P::Public: Codec + std::fmt::Debug,
    P::Signature: Codec,
{
    let runtime_api = client.runtime_api();

    let claim_mode = if force_authoring {
        ClaimMode::ForceAuthoring
    } else {
        ClaimMode::NormalAuthoring
    };

    let slot_claim =
        tanssi_claim_slot::<P, Block>(aux_data, &parent_header, slot, claim_mode, keystore);

    // Here we lean on the property that building on an empty unincluded segment must always
    // be legal. Skipping the runtime API query here allows us to seamlessly run this
    // collator against chains which have not yet upgraded their runtime.
    if parent_header.hash() != included_block
        && !runtime_api.can_build_upon(parent_header.hash(), included_block, slot)?
    {
        return Ok(None);
    }

    Ok(slot_claim)
}

/// Reads allowed ancestry length parameter from the relay chain storage at the given relay parent.
///
/// Falls back to 0 in case of an error.
async fn max_ancestry_lookback(
    relay_parent: PHash,
    relay_client: &impl RelayChainInterface,
) -> usize {
    match load_abridged_host_configuration(relay_parent, relay_client).await {
        Ok(Some(config)) => config.async_backing_params.allowed_ancestry_len as usize,
        Ok(None) => {
            tracing::error!(
                target: crate::LOG_TARGET,
                "Active config is missing in relay chain storage",
            );
            0
        }
        Err(err) => {
            tracing::error!(
                target: crate::LOG_TARGET,
                ?err,
                ?relay_parent,
                "Failed to read active config from relay chain client",
            );
            0
        }
    }
}

// Checks if there exists a scheduled core for the para at the provided relay parent.
//
// Falls back to `false` in case of an error.
async fn cores_scheduled_for_para(
    relay_parent: PHash,
    para_id: ParaId,
    overseer_handle: &mut OverseerHandle,
    relay_client: &impl RelayChainInterface,
) -> Vec<CoreIndex> {
    let (tx, rx) = oneshot::channel();
    let request = RuntimeApiRequest::AvailabilityCores(tx);
    overseer_handle
        .send_msg(
            RuntimeApiMessage::Request(relay_parent, request),
            "LookaheadCollator",
        )
        .await;

    let max_candidate_depth = async_backing_params(relay_parent, relay_client)
        .await
        .map(|c| c.max_candidate_depth)
        .unwrap_or(0);

    let cores = match rx.await {
        Ok(Ok(cores)) => cores,
        Ok(Err(error)) => {
            tracing::error!(
                target: crate::LOG_TARGET,
                ?error,
                ?relay_parent,
                "Failed to query availability cores runtime API",
            );
            return Vec::new();
        }
        Err(oneshot::Canceled) => {
            tracing::error!(
                target: crate::LOG_TARGET,
                ?relay_parent,
                "Sender for availability cores runtime request dropped",
            );
            return Vec::new();
        }
    };

    cores
        .iter()
        .enumerate()
        .filter_map(|(index, core)| {
            let core_para_id = match core {
                CoreState::Scheduled(scheduled_core) => Some(scheduled_core.para_id),
                CoreState::Occupied(occupied_core) if max_candidate_depth >= 1 => occupied_core
                    .next_up_on_available
                    .as_ref()
                    .map(|scheduled_core| scheduled_core.para_id),
                CoreState::Free | CoreState::Occupied(_) => None,
            };

            if core_para_id == Some(para_id) {
                Some(CoreIndex(index as u32))
            } else {
                None
            }
        })
        .collect()
}

/// Reads async backing parameters from the relay chain storage at the given relay parent.
async fn async_backing_params(
    relay_parent: PHash,
    relay_client: &impl RelayChainInterface,
) -> Option<AsyncBackingParams> {
    match load_abridged_host_configuration(relay_parent, relay_client).await {
        Ok(Some(config)) => Some(config.async_backing_params),
        Ok(None) => {
            tracing::error!(
                target: crate::LOG_TARGET,
                "Active config is missing in relay chain storage",
            );
            None
        }
        Err(err) => {
            tracing::error!(
                target: crate::LOG_TARGET,
                ?err,
                ?relay_parent,
                "Failed to read active config from relay chain client",
            );
            None
        }
    }
}
