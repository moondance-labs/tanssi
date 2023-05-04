use sc_consensus_manual_seal::{ConsensusDataProvider, Error};
use sc_client_api::{AuxStore, UsageProvider};
use sc_consensus::BlockImportParams;
use sp_api::{ProvideRuntimeApi, TransactionFor};
use sp_blockchain::{HeaderBackend, HeaderMetadata};
use sp_consensus_aura::{
	digests::CompatibleDigestItem,
	sr25519::{AuthorityId, AuthoritySignature},
	AuraApi, Slot, SlotDuration,
};
use sp_inherents::InherentData;
use sp_runtime::{traits::Block as BlockT, Digest, DigestItem};
use sp_timestamp::TimestampInherentData;
use std::{marker::PhantomData, sync::Arc};


/// Consensus data provider for Aura.
pub struct TanssiManualSealAuraConsensusDataProvider<B, C, P> {
	// slot duration
	slot_duration: SlotDuration,
	// phantom data for required generics
	_phantom: PhantomData<(B, C, P)>,
}

impl<B, C, P> TanssiManualSealAuraConsensusDataProvider<B, C, P>
where
	B: BlockT,
	C: AuxStore + ProvideRuntimeApi<B> + UsageProvider<B>,
	C::Api: AuraApi<B, nimbus_primitives::NimbusId>,
{
	/// Creates a new instance of the [`AuraConsensusDataProvider`], requires that `client`
	/// implements [`sp_consensus_aura::AuraApi`]
	pub fn new(client: Arc<C>) -> Self {
		let slot_duration = sc_consensus_aura::slot_duration(&*client)
			.expect("slot_duration is always present; qed.");

		Self { slot_duration, _phantom: PhantomData }
	}
}

impl<B, C, P> ConsensusDataProvider<B> for TanssiManualSealAuraConsensusDataProvider<B, C, P>
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

	fn create_digest(
		&self,
		_parent: &B::Header,
		inherents: &InherentData,
	) -> Result<Digest, Error> {
		let timestamp =
			inherents.timestamp_inherent_data()?.expect("Timestamp is always present; qed");

		// we always calculate the new slot number based on the current time-stamp and the slot
		// duration.
		let digest_item = <DigestItem as CompatibleDigestItem<AuthoritySignature>>::aura_pre_digest(
			Slot::from_timestamp(timestamp, self.slot_duration),
		);

		Ok(Digest { logs: vec![digest_item] })
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