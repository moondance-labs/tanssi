//! The Manual Seal implementation for the OrchestratorAuraConsensus
use {
    nimbus_primitives::{
        CompatibleDigestItem as NimbusCompatibleDigestItem, NimbusPair, NimbusSignature,
    },
    sc_client_api::{AuxStore, UsageProvider},
    sc_consensus::BlockImportParams,
    sc_consensus_manual_seal::{ConsensusDataProvider, Error},
    sp_api::{HeaderT, ProvideRuntimeApi, TransactionFor},
    sp_blockchain::{HeaderBackend, HeaderMetadata},
    sp_consensus_aura::{digests::CompatibleDigestItem, AuraApi, Slot, SlotDuration},
    sp_core::crypto::ByteArray,
    sp_inherents::InherentData,
    sp_keystore::{SyncCryptoStore, SyncCryptoStorePtr},
    sp_runtime::{traits::Block as BlockT, Digest, DigestItem},
    sp_timestamp::TimestampInherentData,
    std::{marker::PhantomData, sync::Arc},
};
/// Consensus data provider for Orchestrator Manual Seal Aura.
pub struct OrchestratorManualSealAuraConsensusDataProvider<B, C, P> {
    // slot duration
    slot_duration: SlotDuration,
    /// Shared reference to keystore
    pub keystore: SyncCryptoStorePtr,

    /// Shared reference to the client
    pub client: Arc<C>,

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
    pub fn new(client: Arc<C>, keystore: SyncCryptoStorePtr) -> Self {
        let slot_duration = sc_consensus_aura::slot_duration(&*client)
            .expect("slot_duration is always present; qed.");

        Self {
            slot_duration,
            keystore,
            client,
            _phantom: PhantomData,
        }
    }
}
use nimbus_primitives::NIMBUS_KEY_ID;
impl<B, C, P> ConsensusDataProvider<B> for OrchestratorManualSealAuraConsensusDataProvider<B, C, P>
where
    B: BlockT,
    C: AuxStore
        + HeaderBackend<B>
        + HeaderMetadata<B, Error = sp_blockchain::Error>
        + UsageProvider<B>
        + ProvideRuntimeApi<B>,
    C::Api: AuraApi<B, nimbus_primitives::NimbusId>,
    P: Send + Sync,
{
    type Transaction = TransactionFor<C, B>;
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

        let authorities = self
            .client
            .runtime_api()
            .authorities(parent.hash())
            .ok()
            .ok_or(sp_consensus::Error::InvalidAuthoritiesSet)?;

        let expected_author = crate::slot_author::<NimbusPair>(slot, authorities.as_ref());

        // TODO: this should always be included, but breaks manual seal tests. We should modify
        // once configuration on how manual seal changes
        let digest = if let Some(author) = expected_author.and_then(|p| {
            if SyncCryptoStore::has_keys(&*self.keystore, &[(p.to_raw_vec(), NIMBUS_KEY_ID)]) {
                log::error!("key found");
                Some(p.clone())
            } else {
                None
            }
        }) {
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
        _params: &mut BlockImportParams<B, Self::Transaction>,
        _inherents: &InherentData,
        _proof: Self::Proof,
    ) -> Result<(), Error> {
        Ok(())
    }
}
