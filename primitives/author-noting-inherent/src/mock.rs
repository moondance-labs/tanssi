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
    crate::OwnParachainInherentData,
    cumulus_primitives_core::ParaId,
    cumulus_primitives_parachain_inherent::{
        ParachainInherentData, INHERENT_IDENTIFIER as PARACHAIN_SYSTEM_INHERENT_IDENTIFIER,
    },
    parity_scale_codec::Encode,
    sp_consensus_aura::{inherents::InherentType, AURA_ENGINE_ID},
    sp_inherents::{InherentData, InherentDataProvider},
    sp_runtime::{traits::BlakeTwo256, DigestItem},
    test_relay_sproof_builder::{HeaderAs, ParaHeaderSproofBuilder, ParaHeaderSproofBuilderItem},
};

pub struct MockAuthorNotingInherentDataProvider {
    /// The current block number of the local block chain (the parachain)
    pub current_para_block: u32,
    /// The relay block in which this parachain appeared to start. This will be the relay block
    /// number in para block #P1
    pub relay_offset: u32,
    /// The number of relay blocks that elapses between each parablock. Probably set this to 1 or 2
    /// to simulate optimistic or realistic relay chain behavior.
    pub relay_blocks_per_para_block: u32,
    /// List of para ids for which to include the header proof. They will all have the same slot number.
    pub para_ids: Vec<ParaId>,
    /// Number of parachain blocks per relay chain epoch
    /// Mock epoch is computed by dividing `current_para_block` by this value.
    pub slots_per_para_block: u32,
}

#[async_trait::async_trait]
impl InherentDataProvider for MockAuthorNotingInherentDataProvider {
    async fn provide_inherent_data(
        &self,
        inherent_data: &mut InherentData,
    ) -> Result<(), sp_inherents::Error> {
        let slot_number = InherentType::from(
            u64::from(self.slots_per_para_block) * u64::from(self.current_para_block),
        );

        let mut sproof_builder = ParaHeaderSproofBuilder::default();

        // Use the "sproof" (spoof proof) builder to build valid mock state root and proof.
        for para_id in self.para_ids.iter() {
            let mut sproof_builder_item = ParaHeaderSproofBuilderItem {
                para_id: *para_id,
                ..Default::default()
            };

            let header = HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                parent_hash: Default::default(),
                number: self.current_para_block,
                state_root: Default::default(),
                extrinsics_root: Default::default(),
                digest: sp_runtime::generic::Digest {
                    logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot_number.encode())],
                },
            });
            sproof_builder_item.author_id = header;

            sproof_builder.items.push(sproof_builder_item);
        }

        if let Ok(Some(validation_system_inherent_data)) =
            inherent_data.get_data::<ParachainInherentData>(&PARACHAIN_SYSTEM_INHERENT_IDENTIFIER)
        {
            let mut previous_validation_data = validation_system_inherent_data.clone();

            // We need to construct a new proof, based on previously inserted backend data
            let (root, proof) = sproof_builder.from_existing_state(
                validation_system_inherent_data
                    .validation_data
                    .relay_parent_storage_root,
                validation_system_inherent_data.relay_chain_state,
            );

            // We push the new computed proof
            inherent_data.put_data(
                crate::INHERENT_IDENTIFIER,
                &OwnParachainInherentData {
                    relay_storage_proof: proof.clone(),
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
            let (_root, proof) = sproof_builder.into_state_root_and_proof();
            inherent_data.put_data(
                crate::INHERENT_IDENTIFIER,
                &OwnParachainInherentData {
                    relay_storage_proof: proof,
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

impl MockAuthorNotingInherentDataProvider {
    pub fn get_key_values(&self) -> Vec<(Vec<u8>, Vec<u8>)> {
        let slot_number = InherentType::from(
            u64::from(self.slots_per_para_block) * u64::from(self.current_para_block),
        );

        let mut sproof_builder = ParaHeaderSproofBuilder::default();

        // Use the "sproof" (spoof proof) builder to build valid mock state root and proof.
        for para_id in self.para_ids.iter() {
            let mut sproof_builder_item = ParaHeaderSproofBuilderItem {
                para_id: *para_id,
                ..Default::default()
            };

            let header = HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                parent_hash: Default::default(),
                number: self.current_para_block,
                state_root: Default::default(),
                extrinsics_root: Default::default(),
                digest: sp_runtime::generic::Digest {
                    logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot_number.encode())],
                },
            });
            sproof_builder_item.author_id = header;

            sproof_builder.items.push(sproof_builder_item);
        }
        sproof_builder.key_values()
    }
}
