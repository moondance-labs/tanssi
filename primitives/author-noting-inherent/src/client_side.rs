use crate::OwnParachainInherentData;
use crate::PARAS_HEADS_INDEX;
use cumulus_primitives_core::ParaId;
use cumulus_primitives_core::PersistedValidationData;
use cumulus_relay_chain_interface::PHash;
use cumulus_relay_chain_interface::RelayChainInterface;
use parity_scale_codec::Encode;
use sp_core::twox_64;

/// Collect the relevant relay chain state in form of a proof for putting it into the validation
/// data inherent.
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

/// The upward message dispatch queue for the given para id.
///
/// The storage entry stores a tuple of two values:
///
/// - `count: u32`, the number of messages currently in the queue for given para,
/// - `total_size: u32`, the total size of all messages in the queue.
pub fn para_id_head(para_id: ParaId) -> Vec<u8> {
    para_id.using_encoded(|para_id: &[u8]| {
        PARAS_HEADS_INDEX
            .iter()
            .chain(twox_64(para_id).iter())
            .chain(para_id.iter())
            .cloned()
            .collect()
    })
}

impl OwnParachainInherentData {
    /// Create the [`ParachainInherentData`] at the given `relay_parent`.
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
