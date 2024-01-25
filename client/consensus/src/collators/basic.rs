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

use crate::*;
use cumulus_client_collator::{
    relay_chain_driven::CollationRequest, service::ServiceInterface as CollatorServiceInterface,
};
use cumulus_client_consensus_common::ParachainBlockImportMarker;
use cumulus_client_consensus_proposer::ProposerInterface;
use cumulus_primitives_core::{relay_chain::{BlockId as RBlockId, Hash as PHash}, CollectCollationInfo, PersistedValidationData};
use cumulus_relay_chain_interface::RelayChainInterface;
use parity_scale_codec::{Codec, Decode};

use polkadot_node_primitives::CollationResult;
use polkadot_overseer::Handle as OverseerHandle;
use polkadot_primitives::{CollatorPair, Id as ParaId};

use futures::{channel::mpsc::Receiver, prelude::*};
use sc_client_api::{backend::AuxStore, BlockBackend, BlockOf};
use sc_consensus::BlockImport;
use sc_consensus_slots::InherentDataProviderExt;
use sp_api::ProvideRuntimeApi;
use sp_application_crypto::AppPublic;
use sp_blockchain::HeaderBackend;
use sp_consensus::SyncOracle;
use sp_consensus_aura::{AuraApi, SlotDuration};
use sp_core::crypto::Pair;
use sp_inherents::CreateInherentDataProviders;
use sp_keystore::KeystorePtr;
use sp_runtime::traits::{Block as BlockT, Header as HeaderT, Member};
use std::{convert::TryFrom, sync::Arc, time::Duration};

use crate::collators as collator_util;
use crate::{consensus_orchestrator::RetrieveAuthoritiesFromOrchestrator, AuthorityId};

/// Parameters for [`run`].
pub struct Params<BI, CIDP, Client, RClient, SO, Proposer, CS, GOH> {
    pub create_inherent_data_providers: CIDP,
    pub get_authorities_from_orchestrator: GOH,
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

/// Run bare Aura consensus as a relay-chain-driven collator.
pub fn run<Block, P, BI, CIDP, Client, RClient, SO, Proposer, CS, GOH>(
    params: Params<BI, CIDP, Client, RClient, SO, Proposer, CS, GOH>,
) -> impl Future<Output = ()> + Send + 'static
where
    Block: BlockT + Send,
    Client: ProvideRuntimeApi<Block>
        + BlockOf
        + AuxStore
        + HeaderBackend<Block>
        + BlockBackend<Block>
        + Send
        + Sync
        + 'static,
    //TODO: re-check and analyze what to add here.
    //Client::Api: TanssiAuthorityAssignmentApi<Block, P::Public> + CollectCollationInfo<Block>,
    RClient: RelayChainInterface + Send + Clone + 'static,
    CIDP: CreateInherentDataProviders<Block, (PHash, PersistedValidationData)> + Send + 'static + Clone,
    CIDP::InherentDataProviders: Send + InherentDataProviderExt,
    BI: BlockImport<Block> + ParachainBlockImportMarker + Send + Sync + 'static,
    SO: SyncOracle + Send + Sync + Clone + 'static,
    Proposer: ProposerInterface<Block> + Send + Sync + 'static,
    CS: CollatorServiceInterface<Block> + Send + Sync + 'static,
    P: Pair,
    P::Public: AppPublic + Member + Codec,
    P::Signature: TryFrom<Vec<u8>> + Member + Codec,
    GOH: RetrieveAuthoritiesFromOrchestrator<Block,(PHash, PersistedValidationData),Vec<AuthorityId<P>>,> + 'static + Sync + Send,
{
    async move {
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
                //get_authorities_from_orchestrator: params.get_authorities_from_orchestrator,
                block_import: params.block_import,
                relay_client: params.relay_client.clone(),
                keystore: params.keystore.clone(),
                para_id: params.para_id,
                proposer: params.proposer,
                collator_service: params.collator_service,
            };

            collator_util::Collator::<Block, P, _, _, _, _, _>::new(params)
        };

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

            if !collator
                .collator_service()
                .check_block_status(parent_hash.clone(), &parent_header)
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

            let authorities = match params
                .get_authorities_from_orchestrator
                .retrieve_authorities_from_orchestrator(
                    parent_hash,
                    (relay_parent_header.hash(), validation_data.clone()),
                )
                .await
                {
                    Err(e) => reject_with_error!(e),
                    Ok(h) => h,
                };
            
            let inherent_providers = match params.create_inherent_data_providers
                .create_inherent_data_providers(parent_hash.clone(), (*request.relay_parent(), validation_data.clone()))
                .await
                {
                    Err(e) => reject_with_error!(e),
                    Ok(h) => h,
                };

            let claim = match collator_util::tanssi_claim_slot::<P>(
                //&*params.para_client,
                authorities,
                // TODO: check if this is the correct slot to pass here
                inherent_providers.slot(),
                //parent_hash,
                //&relay_parent_header,
                //params.slot_duration,
                //params.relay_chain_slot_duration,
                params.force_authoring,
                &params.keystore,
            )
            .await
            {
                Err(e) => reject_with_error!(e),
                Ok(h) => h,
            };
/*             .map_err(|e| {
                tracing::error!(
                    target: LOG_TARGET,
                    error = ?e,
                    "Failed to get orch head.",
                )
            })
            .ok()?; */


            let (parachain_inherent_data, other_inherent_data) = try_request!(
                collator
                    .create_inherent_data(
                        *request.relay_parent(),
                        &validation_data,
                        parent_hash,
                        None,
                    )
                    .await
            );

            let (collation, _, post_hash) = try_request!(
                collator
                    .collate(
                        &parent_header,
                        &mut claim.expect("Slot claim should exist"),
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

            let result_sender = Some(collator.collator_service().announce_with_barrier(post_hash));
            request.complete(Some(CollationResult {
                collation,
                result_sender,
            }));
        }
    }
}
