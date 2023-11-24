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
    crate::OwnParachainInherentData,
    cumulus_primitives_core::ParaId,
    cumulus_relay_chain_interface::{PHash, RelayChainInterface},
    dp_core::well_known_keys::para_id_head,
};

/// Collect the relevant relay chain state in form of a proof
/// for putting it into the author
/// noting inherent.
async fn collect_relay_storage_proof(
    relay_chain_interface: &impl RelayChainInterface,
    para_ids: &[ParaId],
    relay_parent: PHash,
) -> Option<sp_state_machine::StorageProof> {
    let relevant_keys = para_ids
        .iter()
        .map(|para_id| para_id_head(*para_id))
        .collect();

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
        para_ids: &[ParaId],
    ) -> Option<OwnParachainInherentData> {
        let relay_storage_proof =
            collect_relay_storage_proof(relay_chain_interface, para_ids, relay_parent).await?;

        Some(OwnParachainInherentData {
            relay_storage_proof,
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
