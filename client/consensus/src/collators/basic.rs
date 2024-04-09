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

use {
    crate::{
        collators as collator_util, consensus_orchestrator::RetrieveAuthoritiesFromOrchestrator,
        OrchestratorAuraWorkerAuxData,
    },
    cumulus_client_collator::{
        relay_chain_driven::CollationRequest, service::ServiceInterface as CollatorServiceInterface,
    },
    cumulus_client_consensus_proposer::ProposerInterface,
    cumulus_primitives_core::{
        relay_chain::{BlockId as RBlockId, Hash as PHash, OccupiedCoreAssumption},
        PersistedValidationData,
    },
    cumulus_relay_chain_interface::RelayChainInterface,
    futures::{channel::mpsc::Receiver, prelude::*},
    parity_scale_codec::{Codec, Decode},
    polkadot_node_primitives::CollationResult,
    polkadot_overseer::Handle as OverseerHandle,
    polkadot_primitives::{CollatorPair, Id as ParaId},
    sc_client_api::{backend::AuxStore, BlockBackend, BlockOf},
    sc_consensus::BlockImport,
    sc_consensus_slots::InherentDataProviderExt,
    sp_api::ProvideRuntimeApi,
    sp_application_crypto::AppPublic,
    sp_blockchain::HeaderBackend,
    sp_consensus::SyncOracle,
    sp_consensus_aura::SlotDuration,
    sp_core::crypto::Pair,
    sp_inherents::CreateInherentDataProviders,
    sp_keystore::KeystorePtr,
    sp_runtime::traits::{Block as BlockT, Header as HeaderT, Member},
    std::{convert::TryFrom, sync::Arc, time::Duration},
};

/// Parameters for [`run`].
pub struct Params<BI, CIDP, Client, RClient, SO, Proposer, CS, GOH> {
    pub create_inherent_data_providers: CIDP,
    pub get_orchestrator_aux_data: GOH,
    pub block_import: BI,
    pub para_client: Arc<Client>,
    pub relay_client: RClient,
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
    pub collation_request_receiver: Option<Receiver<CollationRequest>>,
}

/// Run tanssi Aura consensus as a relay-chain-driven collator.
pub async fn run<Block, P, BI, CIDP, Client, RClient, SO, Proposer, CS, GOH>(
    params: Params<BI, CIDP, Client, RClient, SO, Proposer, CS, GOH>,
) where
    Block: BlockT + Send,
    Client: ProvideRuntimeApi<Block>
        + BlockOf
        + AuxStore
        + HeaderBackend<Block>
        + BlockBackend<Block>
        + Send
        + Sync
        + 'static,
    RClient: RelayChainInterface + Send + Clone + 'static,
    CIDP: CreateInherentDataProviders<Block, (PHash, PersistedValidationData)>
        + Send
        + 'static
        + Clone,
    CIDP::InherentDataProviders: Send + InherentDataProviderExt,
    BI: BlockImport<Block> + Send + Sync + 'static,
    SO: SyncOracle + Send + Sync + Clone + 'static,
    Proposer: ProposerInterface<Block> + Send + Sync + 'static,
    CS: CollatorServiceInterface<Block> + Send + Sync + 'static,
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
    let mut collation_requests = match params.collation_request_receiver {
        Some(receiver) => receiver,
        None => {
            cumulus_client_collator::relay_chain_driven::init(
                params.collator_key,
                params.para_id,
                params.overseer_handle,
            )
            .await
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

    let mut last_processed_slot = 0;

    while let Some(request) = collation_requests.next().await {
        macro_rules! reject_with_error {
				($err:expr) => {{
					request.complete(None);
					tracing::error!(target: crate::LOG_TARGET, err = ?{ $err });
					continue;
				}};
			}

        macro_rules! try_request {
            ($x:expr) => {{
                match $x {
                    Ok(x) => x,
                    Err(e) => reject_with_error!(e),
                }
            }};
        }

        let validation_data = request.persisted_validation_data();

        let parent_header = try_request!(Block::Header::decode(
            &mut &validation_data.parent_head.0[..]
        ));

        let parent_hash = parent_header.hash();

        // Evaluate whether we can build on top
        // The requirement is that the parent_hash is the last included block in the relay
        let can_build = can_build_upon_included::<Block, _>(
            parent_hash,
            &collator.relay_client,
            params.para_id,
            *request.relay_parent(),
        )
        .await;
        if !can_build {
            continue;
        }

        // Check whether we can build upon this block
        if !collator
            .collator_service()
            .check_block_status(parent_hash, &parent_header)
        {
            continue;
        }

        let relay_parent_header = match params
            .relay_client
            .header(RBlockId::hash(*request.relay_parent()))
            .await
        {
            Err(e) => reject_with_error!(e),
            Ok(None) => continue, // sanity: would be inconsistent to get `None` here
            Ok(Some(h)) => h,
        };

        // Retrieve authorities that are able to produce the block
        let authorities = match params
            .get_orchestrator_aux_data
            .retrieve_authorities_from_orchestrator(
                parent_hash,
                (relay_parent_header.hash(), validation_data.clone()),
            )
            .await
        {
            Err(e) => reject_with_error!(e),
            Ok(h) => h,
        };

        let inherent_providers = match params
            .create_inherent_data_providers
            .create_inherent_data_providers(
                parent_hash,
                (*request.relay_parent(), validation_data.clone()),
            )
            .await
        {
            Err(e) => reject_with_error!(e),
            Ok(h) => h,
        };

        let mut claim = match collator_util::tanssi_claim_slot::<P, Block>(
            authorities,
            &parent_header,
            inherent_providers.slot(),
            params.force_authoring,
            &params.keystore,
        ) {
            Ok(None) => continue,
            Err(e) => reject_with_error!(e),
            Ok(Some(h)) => h,
        };

        // With async backing this function will be called every relay chain block.
        //
        // Most parachains currently run with 12 seconds slots and thus, they would try to
        // produce multiple blocks per slot which very likely would fail on chain. Thus, we have
        // this "hack" to only produce on block per slot.
        //
        // With https://github.com/paritytech/polkadot-sdk/issues/3168 this implementation will be
        // obsolete and also the underlying issue will be fixed.
        if last_processed_slot >= *claim.slot() {
            continue;
        }

        let (parachain_inherent_data, other_inherent_data) = try_request!(
            collator
                .create_inherent_data(*request.relay_parent(), validation_data, parent_hash, None,)
                .await
        );

        let maybe_collation = try_request!(
            collator
                .collate(
                    &parent_header,
                    &mut claim,
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
        );

        if let Some((collation, _, post_hash)) = maybe_collation {
            let result_sender = Some(collator.collator_service().announce_with_barrier(post_hash));
            request.complete(Some(CollationResult {
                collation,
                result_sender,
            }));
        } else {
            request.complete(None);
            tracing::debug!(target: crate::LOG_TARGET, "No block proposal");
        }
        last_processed_slot = *claim.slot();
    }
}

// Checks whether we can build upon the last included block
// Essentially checks that the latest head we are trying to build
// is the one included in the relay
async fn can_build_upon_included<Block: BlockT, RClient>(
    parent_hash: Block::Hash,
    relay_client: &RClient,
    para_id: ParaId,
    relay_parent: PHash,
) -> bool
where
    RClient: RelayChainInterface + Send + Clone + 'static,
{
    let included_header = relay_client
        .persisted_validation_data(relay_parent, para_id, OccupiedCoreAssumption::TimedOut)
        .await;

    if let Ok(Some(included_header)) = included_header {
        let decoded = Block::Header::decode(&mut &included_header.parent_head.0[..]).ok();
        if let Some(decoded_header) = decoded {
            let included_hash = decoded_header.hash();
            if parent_hash == included_hash {
                return true;
            }
        }
    }
    false
}
