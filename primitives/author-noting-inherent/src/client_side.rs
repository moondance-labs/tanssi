use crate::{para_id_head, OwnParachainInherentData};
use cumulus_primitives_core::ParaId;
use cumulus_primitives_core::PersistedValidationData;
use cumulus_relay_chain_interface::PHash;
use cumulus_relay_chain_interface::RelayChainInterface;
/// Collect the relevant relay chain state in form of a proof
/// for putting it into the author
/// noting inherent.
async fn collect_relay_storage_proof(
    relay_chain_interface: &impl RelayChainInterface,
    para_id: ParaId,
    relay_parent: PHash,
) -> Option<sp_state_machine::StorageProof> {
    let mut relevant_keys = Vec::new();
    relevant_keys.push(para_id_head(para_id));

    relay_chain_interface
        .prove_read(relay_parent, &relevant_keys)
        .await
        .ok()
}

impl OwnParachainInherentData {
    /// Create the [`OwnParachainInherentData`] at the given `relay_parent`.
    ///
    /// Returns `None` if the creation failed.
    pub async fn create_at(
        relay_parent: PHash,
        relay_chain_interface: &impl RelayChainInterface,
        validation_data: &PersistedValidationData,
        para_id: ParaId,
    ) -> Option<OwnParachainInherentData> {
        let relay_chain_state =
            collect_relay_storage_proof(relay_chain_interface, para_id, relay_parent).await?;

        Some(OwnParachainInherentData {
            validation_data: validation_data.clone(),
            relay_chain_state,
        })
    }
}

// Implementation of InherentDataProvider
#[async_trait::async_trait]
impl sp_inherents::InherentDataProvider for OwnParachainInherentData {
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
