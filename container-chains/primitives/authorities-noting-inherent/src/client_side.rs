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
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>

use {
    crate::ContainerChainAuthoritiesInherentData,
    cumulus_primitives_core::{relay_chain::HeadData, ParaId},
    cumulus_relay_chain_interface::{PHash, RelayChainInterface},
    parity_scale_codec::Decode,
    tc_orchestrator_chain_interface::OrchestratorChainInterface,
    tp_core::{well_known_keys, Header as OrchestratorHeader},
};

const LOG_TARGET: &str = "parachain-inherent";

/// Collect the relevant relay chain state in form of a proof
/// for putting it into authorities noting inherent
async fn collect_relay_storage_proof(
    relay_chain_interface: &impl RelayChainInterface,
    orchestrator_para_id: ParaId,
    relay_parent: PHash,
) -> Option<sp_state_machine::StorageProof> {
    let mut relevant_keys = Vec::new();
    relevant_keys.push(well_known_keys::para_id_head(orchestrator_para_id));

    relay_chain_interface
        .prove_read(relay_parent, &relevant_keys)
        .await
        .ok()
}

/// Collect the relevant orchestrator chain state in form of a proof
/// for putting it into the authorities noting inherent
async fn collect_orchestrator_storage_proof(
    orchestrator_chain_interface: &impl OrchestratorChainInterface,
    orchestrator_parent: PHash,
) -> Option<sp_state_machine::StorageProof> {
    // We need to fetch the actual session index to build the key for the
    // authorities.
    let session_index = orchestrator_chain_interface
        .get_storage_by_key(orchestrator_parent, well_known_keys::SESSION_INDEX)
        .await
        .ok()??;
    let session_index = u32::decode(&mut session_index.as_slice()).ok()?;

    let mut relevant_keys = Vec::new();
    relevant_keys.push(well_known_keys::SESSION_INDEX.to_vec());
    relevant_keys.push(well_known_keys::authority_assignment_for_session(
        session_index,
    ));

    orchestrator_chain_interface
        .prove_read(orchestrator_parent, &relevant_keys)
        .await
        .ok()
}

impl ContainerChainAuthoritiesInherentData {
    /// Create the [`ContainerChainAuthoritiesInherentData`] at the given `relay_parent`.
    ///
    /// Returns `None` if the creation failed.
    pub async fn create_at(
        relay_parent: PHash,
        relay_chain_interface: &impl RelayChainInterface,
        orchestrator_chain_interface: &impl OrchestratorChainInterface,
        orchestrator_para_id: ParaId,
    ) -> Option<ContainerChainAuthoritiesInherentData> {
        let relay_chain_state =
            collect_relay_storage_proof(relay_chain_interface, orchestrator_para_id, relay_parent)
                .await?;

        let header_orchestrator = relay_chain_interface
            .get_storage_by_key(
                relay_parent,
                &well_known_keys::para_id_head(orchestrator_para_id),
            )
            .await
            .map_err(|e| {
                tracing::error!(
                    target: LOG_TARGET,
                    relay_parent = ?relay_parent,
                    error = ?e,
                    "Cannot obtain the orchestrator para id header."
                )
            })
            .ok()?;

        let header_data_orchestrator = header_orchestrator
            .map(|raw| <HeadData>::decode(&mut &raw[..]))
            .transpose()
            .map_err(|e| {
                tracing::error!(
                    target: LOG_TARGET,
                    error = ?e,
                    "Cannot decode the head data",
                )
            })
            .ok()?
            .unwrap_or_default();

        // We later take the Header decoded
        let orchestrator_header =
            tp_core::Header::decode(&mut header_data_orchestrator.0.as_slice())
                .map_err(|e| {
                    tracing::error!(
                        target: LOG_TARGET,
                        error = ?e,
                        "Cannot decode the head data",
                    )
                })
                .ok()?;

        let orchestrator_chain_state = collect_orchestrator_storage_proof(
            orchestrator_chain_interface,
            orchestrator_header.hash(),
        )
        .await?;

        Some(ContainerChainAuthoritiesInherentData {
            relay_chain_state: relay_chain_state.clone(),
            orchestrator_chain_state,
        })
    }

    pub async fn get_latest_orchestrator_head_info(
        relay_parent: PHash,
        relay_chain_interface: &impl RelayChainInterface,
        orchestrator_para_id: ParaId,
    ) -> Option<OrchestratorHeader> {
        let header_orchestrator = relay_chain_interface
            .get_storage_by_key(
                relay_parent,
                &well_known_keys::para_id_head(orchestrator_para_id),
            )
            .await
            .map_err(|e| {
                tracing::error!(
                    target: LOG_TARGET,
                    relay_parent = ?relay_parent,
                    error = ?e,
                    "Cannot obtain the orchestrator para id header."
                )
            })
            .ok()?;

        let header_data_orchestrator = header_orchestrator
            .map(|raw| <HeadData>::decode(&mut &raw[..]))
            .transpose()
            .map_err(|e| {
                tracing::error!(
                    target: LOG_TARGET,
                    error = ?e,
                    "Cannot decode the head data",
                )
            })
            .ok()?
            .unwrap_or_default();

        // We later take the Header decoded
        let orchestrator_header =
            OrchestratorHeader::decode(&mut header_data_orchestrator.0.as_slice())
                .map_err(|e| {
                    tracing::error!(
                        target: LOG_TARGET,
                        error = ?e,
                        "Cannot decode the head data",
                    )
                })
                .ok()?;

        Some(orchestrator_header)
    }
}

// Implementation of InherentDataProvider
#[async_trait::async_trait]
impl sp_inherents::InherentDataProvider for ContainerChainAuthoritiesInherentData {
    async fn provide_inherent_data(
        &self,
        inherent_data: &mut sp_inherents::InherentData,
    ) -> Result<(), sp_inherents::Error> {
        inherent_data.put_data(crate::INHERENT_IDENTIFIER, &self)
    }

    async fn try_handle_error(
        &self,
        _: &sp_inherents::InherentIdentifier,
        _: &[u8],
    ) -> Option<Result<(), sp_inherents::Error>> {
        None
    }
}
