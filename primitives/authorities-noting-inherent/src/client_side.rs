use crate::ContainerChainAuthoritiesInherentData;
use cumulus_primitives_core::ParaId;
use cumulus_primitives_core::PersistedValidationData;
use cumulus_relay_chain_interface::PHash;
use cumulus_relay_chain_interface::RelayChainInterface;
use tc_tanssi_chain_interface::TanssiChainInterface;

use cumulus_primitives_core::relay_chain::BlakeTwo256;
use cumulus_primitives_core::relay_chain::BlockNumber;
use cumulus_primitives_core::relay_chain::HeadData;
use parity_scale_codec::Decode;
use tp_core::well_known_keys::para_id_head;
use tp_core::well_known_keys::COLLATOR_ASSIGNMENT_INDEX;

const LOG_TARGET: &str = "parachain-inherent";

/// Collect the relevant relay chain state in form of a proof
/// for putting it into the author
/// noting inherent.
async fn collect_relay_storage_proof(
    relay_chain_interface: &impl RelayChainInterface,
    orchestrator_para_id: ParaId,
    relay_parent: PHash,
) -> Option<sp_state_machine::StorageProof> {
    let mut relevant_keys = Vec::new();
    relevant_keys.push(para_id_head(orchestrator_para_id));

    relay_chain_interface
        .prove_read(relay_parent, &relevant_keys)
        .await
        .ok()
}

async fn collect_tanssi_storage_proof(
    orchestrator_chain_interface: &impl TanssiChainInterface,
    tanssi_parent: PHash,
) -> Option<sp_state_machine::StorageProof> {
    let mut relevant_keys = Vec::new();
    relevant_keys.push(COLLATOR_ASSIGNMENT_INDEX.to_vec());

    orchestrator_chain_interface
        .prove_read(tanssi_parent, &relevant_keys)
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
        orchestrator_chain_interface: &impl TanssiChainInterface,
        validation_data: &PersistedValidationData,
        orchestrator_para_id: ParaId,
    ) -> Option<ContainerChainAuthoritiesInherentData> {
        let relay_chain_state = collect_relay_storage_proof(
            relay_chain_interface,
            orchestrator_para_id.clone(),
            relay_parent,
        )
        .await?;

        let header_orchestrator = relay_chain_interface
            .get_storage_by_key(relay_parent, &para_id_head(orchestrator_para_id))
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
        let orchestrator_header = sp_runtime::generic::Header::<BlockNumber, BlakeTwo256>::decode(
            &mut header_data_orchestrator.0.as_slice(),
        )
        .map_err(|e| {
            tracing::error!(
                target: LOG_TARGET,
                error = ?e,
                "Cannot decode the head data",
            )
        })
        .ok()?;

        // TODO: not sure if this is the hash we should use, it looks to me as if point to the previous block
        // For now we keep it like this, but something that we should look
        let orchestrator_chain_state = collect_tanssi_storage_proof(
            orchestrator_chain_interface,
            orchestrator_header.hash(),
        )
        .await?;

        Some(ContainerChainAuthoritiesInherentData {
            validation_data: validation_data.clone(),
            relay_chain_state: relay_chain_state.clone(),
            orchestrator_chain_state: orchestrator_chain_state,
        })
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
