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

//! The Manual Seal implementation for the OrchestratorAuraConsensus

use {
    cumulus_primitives_core::ParaId,
    dp_consensus::TanssiAuthorityAssignmentApi,
    nimbus_primitives::{
        CompatibleDigestItem as NimbusCompatibleDigestItem, NimbusId, NimbusPair, NimbusSignature,
    },
    sc_client_api::{AuxStore, UsageProvider},
    sc_consensus::BlockImportParams,
    sc_consensus_manual_seal::{ConsensusDataProvider, Error},
    sp_api::ProvideRuntimeApi,
    sp_blockchain::{HeaderBackend, HeaderMetadata},
    sp_consensus_aura::{digests::CompatibleDigestItem, AuraApi, Slot, SlotDuration},
    sp_core::Pair,
    sp_inherents::InherentData,
    sp_keystore::KeystorePtr,
    sp_runtime::{
        traits::{Block as BlockT, Header as HeaderT},
        Digest, DigestItem,
    },
    sp_timestamp::TimestampInherentData,
    std::{marker::PhantomData, sync::Arc},
};
/// Consensus data provider for Orchestrator Manual Seal Aura.
pub struct OrchestratorManualSealAuraConsensusDataProvider<B, C, P> {
    // slot duration
    slot_duration: SlotDuration,
    /// Shared reference to keystore
    pub keystore: KeystorePtr,

    /// Shared reference to the client
    pub client: Arc<C>,

    /// ParaId of the orchestrator
    pub orchestrator_para_id: ParaId,

    // phantom data for required generics
    _phantom: PhantomData<(B, C, P)>,
}

impl<B, C, P> OrchestratorManualSealAuraConsensusDataProvider<B, C, P>
where
    B: BlockT,
    C: AuxStore + ProvideRuntimeApi<B> + UsageProvider<B>,
    C::Api: AuraApi<B, nimbus_primitives::NimbusId>,
{
    /// Creates a new instance of the [`AuraConsensusDataProvider`], requires that `client`
    /// implements [`sp_consensus_aura::AuraApi`]
    pub fn new(client: Arc<C>, keystore: KeystorePtr, orchestrator_para_id: ParaId) -> Self {
        let slot_duration = sc_consensus_aura::slot_duration(&*client)
            .expect("slot_duration is always present; qed.");

        Self {
            slot_duration,
            keystore,
            client,
            orchestrator_para_id,
            _phantom: PhantomData,
        }
    }
}
impl<B, C, P> ConsensusDataProvider<B> for OrchestratorManualSealAuraConsensusDataProvider<B, C, P>
where
    B: BlockT,
    C: AuxStore
        + HeaderBackend<B>
        + HeaderMetadata<B, Error = sp_blockchain::Error>
        + UsageProvider<B>
        + ProvideRuntimeApi<B>,
    C::Api: TanssiAuthorityAssignmentApi<B, nimbus_primitives::NimbusId>,
    P: Send + Sync,
{
    type Proof = P;

    fn create_digest(&self, parent: &B::Header, inherents: &InherentData) -> Result<Digest, Error> {
        let timestamp = inherents
            .timestamp_inherent_data()?
            .expect("Timestamp is always present; qed");

        // we always calculate the new slot number based on the current time-stamp and the slot
        // duration.
        // TODO: we need to add the nimbus digest here
        let slot = Slot::from_timestamp(timestamp, self.slot_duration);
        let aura_digest_item =
            <DigestItem as CompatibleDigestItem<NimbusSignature>>::aura_pre_digest(slot);

        // Fetch the authorities for the orchestrator chain
        let authorities = self
            .client
            .runtime_api()
            .para_id_authorities(parent.hash(), self.orchestrator_para_id)
            .ok()
            .ok_or(sp_consensus::Error::InvalidAuthoritiesSet)?
            .unwrap_or_default();

        let expected_author = crate::slot_author::<NimbusPair>(slot, authorities.as_ref());

        // TODO: this should always be included, but breaks manual seal tests. We should modify
        // once configuration on how manual seal changes
        let digest = if let Some(author) = expected_author {
            let nimbus_digest =
                <DigestItem as NimbusCompatibleDigestItem>::nimbus_pre_digest(author.clone());
            Digest {
                logs: vec![aura_digest_item, nimbus_digest],
            }
        } else {
            Digest {
                logs: vec![aura_digest_item],
            }
        };
        Ok(digest)
    }

    fn append_block_import(
        &self,
        _parent: &B::Header,
        _params: &mut BlockImportParams<B>,
        _inherents: &InherentData,
        _proof: Self::Proof,
    ) -> Result<(), Error> {
        Ok(())
    }
}

/// Helper function to generate a crypto pair from seed
pub fn get_aura_id_from_seed(seed: &str) -> NimbusId {
    sp_core::sr25519::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
        .into()
}

/// Consensus data provider for Container Manual Seal Aura.
pub struct ContainerManualSealAuraConsensusDataProvider<B> {
    // slot duration
    slot_duration: SlotDuration,
    // Authorities from which the author should be calculated
    pub authorities: Vec<NimbusId>,
    // phantom data for required generics
    _phantom: PhantomData<B>,
}

impl<B> ContainerManualSealAuraConsensusDataProvider<B>
where
    B: BlockT,
{
    /// Creates a new instance of the [`AuraConsensusDataProvider`], requires that `client`
    /// implements [`sp_consensus_aura::AuraApi`]
    pub fn new(slot_duration: SlotDuration, authorities: Vec<NimbusId>) -> Self {
        Self {
            slot_duration,
            authorities,
            _phantom: PhantomData,
        }
    }
}
impl<B> ConsensusDataProvider<B> for ContainerManualSealAuraConsensusDataProvider<B>
where
    B: BlockT,
{
    type Proof = ();

    fn create_digest(
        &self,
        _parent: &B::Header,
        inherents: &InherentData,
    ) -> Result<Digest, Error> {
        let timestamp = inherents
            .timestamp_inherent_data()?
            .expect("Timestamp is always present; qed");

        // we always calculate the new slot number based on the current time-stamp and the slot
        // duration.
        // TODO: we need to add the nimbus digest here
        let slot = Slot::from_timestamp(timestamp, self.slot_duration);
        let aura_digest_item =
            <DigestItem as CompatibleDigestItem<NimbusSignature>>::aura_pre_digest(slot);

        let alice_id = get_aura_id_from_seed("alice");
        let expected_author: Option<nimbus_primitives::NimbusId> = Some(alice_id);

        // TODO: this should always be included, but breaks manual seal tests. We should modify
        // once configuration on how manual seal changes
        let digest = if let Some(author) = expected_author {
            let nimbus_digest =
                <DigestItem as NimbusCompatibleDigestItem>::nimbus_pre_digest(author);
            Digest {
                logs: vec![aura_digest_item, nimbus_digest],
            }
        } else {
            Digest {
                logs: vec![aura_digest_item],
            }
        };
        Ok(digest)
    }

    fn append_block_import(
        &self,
        _parent: &B::Header,
        _params: &mut BlockImportParams<B>,
        _inherents: &InherentData,
        _proof: Self::Proof,
    ) -> Result<(), Error> {
        Ok(())
    }
}

impl<B> fc_rpc::pending::ConsensusDataProvider<B>
    for ContainerManualSealAuraConsensusDataProvider<B>
where
    B: BlockT,
{
    fn create_digest(
        &self,
        _parent: &B::Header,
        inherents: &InherentData,
    ) -> Result<sp_runtime::Digest, sp_inherents::Error> {
        <Self as ConsensusDataProvider<B>>::create_digest(self, _parent, inherents)
            .map_err(|_| sp_inherents::Error::FatalErrorReported)
    }
}
