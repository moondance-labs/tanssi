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
        collators::{self as collator_util, tanssi_claim_slot, SlotClaim},
        consensus_orchestrator::RetrieveAuthoritiesFromOrchestrator,
        OrchestratorAuraWorkerAuxData,
    },
    async_backing_primitives::UnincludedSegmentApi,
    cumulus_client_collator::service::ServiceInterface as CollatorServiceInterface,
    cumulus_client_consensus_common::{
        self as consensus_common, load_abridged_host_configuration, ParachainBlockImportMarker,
        ParentSearchParams,
    },
    cumulus_client_consensus_proposer::ProposerInterface,
    cumulus_primitives_core::{relay_chain::Hash as PHash, PersistedValidationData},
    cumulus_relay_chain_interface::RelayChainInterface,
    futures::{channel::oneshot, prelude::*},
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
    sp_api::ProvideRuntimeApi,
    sp_application_crypto::AppPublic,
    sp_blockchain::HeaderBackend,
    sp_consensus::SyncOracle,
    sp_consensus_aura::{Slot, SlotDuration},
    sp_core::crypto::Pair,
    sp_inherents::CreateInherentDataProviders,
    sp_keystore::KeystorePtr,
    sp_runtime::traits::{Block as BlockT, Header as HeaderT, Member},
    std::{convert::TryFrom, error::Error, sync::Arc, time::Duration},
    tokio::select,
    tokio_util::sync::CancellationToken,
};

/// Parameters for [`run`].
pub struct Params<BI, CIDP, Client, Backend, RClient, CHP, SO, Proposer, CS, GOH> {
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
    pub slot_duration: SlotDuration,
    pub relay_chain_slot_duration: Duration,
    pub proposer: Proposer,
    pub collator_service: CS,
    pub authoring_duration: Duration,
    pub force_authoring: bool,
    pub cancellation_token: CancellationToken,
}

/// Run async-backing-friendly for Tanssi Aura.
pub fn run<Block, P, BI, CIDP, Client, Backend, RClient, CHP, SO, Proposer, CS, GOH>(
    mut params: Params<BI, CIDP, Client, Backend, RClient, CHP, SO, Proposer, CS, GOH>,
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
    BI: BlockImport<Block> + ParachainBlockImportMarker + Send + Sync + 'static,
    SO: SyncOracle + Send + Sync + Clone + 'static,
    Proposer: ProposerInterface<Block> + Send + Sync + 'static,
    CS: CollatorServiceInterface<Block> + Send + Sync + 'static,
    CHP: consensus_common::ValidationCodeHashProvider<Block::Hash> + Send + 'static,
    P: Pair + Sync + Send + 'static,
    P::Public: AppPublic + Member + Codec,
    P::Signature: TryFrom<Vec<u8>> + Member + Codec,
    GOH: RetrieveAuthoritiesFromOrchestrator<
            Block,
            (PHash, PersistedValidationData),
            OrchestratorAuraWorkerAuxData<P>,
        >
        + 'static
        + Sync
        + Send,
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

                    if !is_para_scheduled(relay_parent, params.para_id, &mut params.overseer_handle).await {
                        tracing::trace!(
                            target: crate::LOG_TARGET,
                            ?relay_parent,
                            ?params.para_id,
                            "Para is not scheduled on any core, skipping import notification",
                        );

                        continue;
                    }

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
    included_block: Block::Hash,
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
    let slot_claim =
        tanssi_claim_slot::<P, Block>(aux_data, &parent_header, slot, force_authoring, keystore);

    // Here we lean on the property that building on an empty unincluded segment must always
    // be legal. Skipping the runtime API query here allows us to seamlessly run this
    // collator against chains which have not yet upgraded their runtime.
    if parent_header.hash() != included_block
        && !runtime_api.can_build_upon(parent_header.hash(), included_block, slot)?
    {
        return Ok(None);
    }

    slot_claim
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
async fn is_para_scheduled(
    relay_parent: PHash,
    para_id: ParaId,
    overseer_handle: &mut OverseerHandle,
) -> bool {
    let (tx, rx) = oneshot::channel();
    let request = RuntimeApiRequest::AvailabilityCores(tx);
    overseer_handle
        .send_msg(
            RuntimeApiMessage::Request(relay_parent, request),
            "LookaheadCollator",
        )
        .await;

    let cores = match rx.await {
        Ok(Ok(cores)) => cores,
        Ok(Err(error)) => {
            tracing::error!(
                target: crate::LOG_TARGET,
                ?error,
                ?relay_parent,
                "Failed to query availability cores runtime API",
            );
            return false;
        }
        Err(oneshot::Canceled) => {
            tracing::error!(
                target: crate::LOG_TARGET,
                ?relay_parent,
                "Sender for availability cores runtime request dropped",
            );
            return false;
        }
    };

    cores.iter().any(|core| core.para_id() == Some(para_id))
}
