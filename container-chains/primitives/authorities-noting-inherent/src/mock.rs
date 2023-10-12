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

//! Inherent data provider that supplies mocked author noting data.
//!
//! This is useful when running a node that is not actually backed by any relay chain.
//! For example when running a local node, or running integration tests.
//!
//! We mock a relay chain block number as follows:
//! relay_block_number = offset + relay_blocks_per_para_block * current_para_block
//! To simulate a parachain that starts in relay block 1000 and gets a block in every other relay
//! block, use 1000 and 2
//!
//! para_id: the parachain of which we are gonna mock the headData
//! slots_per_para_block: the number of slots to be applied per parachain block

use {
    crate::ContainerChainAuthoritiesInherentData,
    cumulus_primitives_core::ParaId,
    cumulus_primitives_parachain_inherent::{
        ParachainInherentData, INHERENT_IDENTIFIER as PARACHAIN_SYSTEM_INHERENT_IDENTIFIER,
    },
    nimbus_primitives::NimbusId,
    sp_inherents::{InherentData, InherentDataProvider},
    sp_std::collections::btree_map::BTreeMap,
    test_relay_sproof_builder::{
        AuthorityAssignmentSproofBuilder, HeaderAs, ParaHeaderSproofBuilder,
        ParaHeaderSproofBuilderItem,
    },
    tp_collator_assignment::AssignedCollators,
};

pub struct MockAuthoritiesNotingInherentDataProvider {
    /// The current block number of the local block chain (the parachain)
    pub current_para_block: u32,
    /// The relay block in which this parachain appeared to start. This will be the relay block
    /// number in para block #P1
    pub relay_offset: u32,
    /// The number of relay blocks that elapses between each parablock. Probably set this to 1 or 2
    /// to simulate optimistic or realistic relay chain behavior.
    pub relay_blocks_per_para_block: u32,
    /// Orchestrator ParaId
    pub orchestrator_para_id: ParaId,
    /// Container ParaId,
    pub container_para_id: ParaId,
    /// Orchestrator ParaId
    pub authorities: Vec<NimbusId>,
}

#[async_trait::async_trait]
impl InherentDataProvider for MockAuthoritiesNotingInherentDataProvider {
    async fn provide_inherent_data(
        &self,
        inherent_data: &mut InherentData,
    ) -> Result<(), sp_inherents::Error> {
        let (sproof, orchestrator_chain_state) = self.build_sproof_builder();

        if let Ok(Some(validation_system_inherent_data)) =
            inherent_data.get_data::<ParachainInherentData>(&PARACHAIN_SYSTEM_INHERENT_IDENTIFIER)
        {
            let mut previous_validation_data = validation_system_inherent_data.clone();

            // We need to construct a new proof, based on previously inserted backend data
            let (root, proof) = sproof.from_existing_state(
                validation_system_inherent_data
                    .validation_data
                    .relay_parent_storage_root,
                validation_system_inherent_data.relay_chain_state,
            );

            // We push the new computed proof
            inherent_data.put_data(
                crate::INHERENT_IDENTIFIER,
                &ContainerChainAuthoritiesInherentData {
                    relay_chain_state: proof.clone(),
                    orchestrator_chain_state,
                },
            )?;

            // But we also need to override the previous one for parachain-system-validation-data
            previous_validation_data
                .validation_data
                .relay_parent_storage_root = root;
            previous_validation_data.relay_chain_state = proof;

            inherent_data.replace_data(
                PARACHAIN_SYSTEM_INHERENT_IDENTIFIER,
                &previous_validation_data,
            );
        } else {
            let (_root, proof) = sproof.into_state_root_and_proof();
            inherent_data.put_data(
                crate::INHERENT_IDENTIFIER,
                &ContainerChainAuthoritiesInherentData {
                    relay_chain_state: proof,
                    orchestrator_chain_state,
                },
            )?;
        }

        Ok(())
    }

    // Copied from the real implementation
    async fn try_handle_error(
        &self,
        _: &sp_inherents::InherentIdentifier,
        _: &[u8],
    ) -> Option<Result<(), sp_inherents::Error>> {
        None
    }
}

impl MockAuthoritiesNotingInherentDataProvider {
    pub fn get_key_values(&self) -> Vec<(Vec<u8>, Vec<u8>)> {
        let (sproof, _) = self.build_sproof_builder();

        sproof.key_values()
    }

    pub fn build_sproof_builder(&self) -> (ParaHeaderSproofBuilder, sp_trie::StorageProof) {
        let mut sproof_builder = ParaHeaderSproofBuilder::default();

        let container_chains =
            BTreeMap::from_iter([(self.container_para_id, self.authorities.clone())]);
        let assignment = AuthorityAssignmentSproofBuilder::<NimbusId> {
            authority_assignment: AssignedCollators {
                orchestrator_chain: vec![],
                container_chains,
            },
            session_index: 0,
        };

        let (orchestrator_chain_root, orchestrator_chain_state) =
            assignment.into_state_root_and_proof();

        // Use the "sproof" (spoof proof) builder to build valid mock state root and proof.
        let mut sproof_builder_item = ParaHeaderSproofBuilderItem {
            para_id: self.orchestrator_para_id,
            ..Default::default()
        };

        let header = HeaderAs::NonEncoded(tp_core::Header {
            parent_hash: Default::default(),
            number: Default::default(),
            state_root: orchestrator_chain_root,
            extrinsics_root: Default::default(),
            digest: sp_runtime::generic::Digest { logs: vec![] },
        });
        sproof_builder_item.author_id = header;

        sproof_builder.items.push(sproof_builder_item);

        (sproof_builder, orchestrator_chain_state)
    }
}
