use nimbus_primitives::CompatibleDigestItem as NimbusCompatibleDigestItem;
use nimbus_primitives::NimbusId;
use nimbus_primitives::NimbusSignature;
use sc_client_api::{AuxStore, UsageProvider};
use sc_consensus::BlockImportParams;
use sc_consensus_manual_seal::{ConsensusDataProvider, Error};
use sp_api::HeaderT;
use sp_api::{ProvideRuntimeApi, TransactionFor};
use sp_blockchain::{HeaderBackend, HeaderMetadata};
use sp_consensus_aura::{digests::CompatibleDigestItem, AuraApi, Slot, SlotDuration};
use sp_core::crypto::{ByteArray, Pair};
use sp_inherents::InherentData;
use sp_keystore::SyncCryptoStore;
use sp_keystore::SyncCryptoStorePtr;
use sp_runtime::{traits::Block as BlockT, Digest, DigestItem};
use sp_timestamp::TimestampInherentData;
use std::{marker::PhantomData, sync::Arc};

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

impl<B, C, P> ConsensusDataProvider<B> for OrchestratorManualSealAuraConsensusDataProvider<B, C, P>
where
    B: BlockT,
    C: AuxStore
        + HeaderBackend<B>
        + HeaderMetadata<B, Error = sp_blockchain::Error>
        + UsageProvider<B>
        + ProvideRuntimeApi<B>,
    C::Api: AuraApi<B, nimbus_primitives::NimbusId>,
    P: Pair + Send + Sync,
    P::Public: Into<NimbusId> + From<NimbusId>,
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
        let digest_item =
            <DigestItem as CompatibleDigestItem<NimbusSignature>>::aura_pre_digest(slot);

        let authorities: Vec<P::Public> = self
            .client
            .runtime_api()
            .authorities(parent.hash())
            .ok()
            .ok_or(sp_consensus::Error::InvalidAuthoritiesSet)?
            .iter()
            .map(|nimbus| nimbus.clone().into())
            .collect();

        let expected_author = crate::slot_author::<P>(slot, authorities.as_ref());
        let author = expected_author
            .and_then(|p| {
                if SyncCryptoStore::has_keys(
                    &*self.keystore,
                    &[(p.to_raw_vec(), sp_application_crypto::key_types::AURA)],
                ) {
                    Some(p.clone())
                } else {
                    None
                }
            })
            .ok_or(sp_consensus::Error::InvalidAuthoritiesSet)?;

        let nimbus_digest =
            <DigestItem as NimbusCompatibleDigestItem>::nimbus_pre_digest(author.clone().into());
        Ok(Digest {
            logs: vec![digest_item, nimbus_digest],
        })
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
