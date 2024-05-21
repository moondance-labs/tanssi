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

pub mod basic;
pub mod lookahead;

use {
    crate::{find_pre_digest, AuthorityId, OrchestratorAuraWorkerAuxData},
    cumulus_client_collator::service::ServiceInterface as CollatorServiceInterface,
    cumulus_client_consensus_common::ParachainCandidate,
    cumulus_client_consensus_proposer::ProposerInterface,
    cumulus_client_parachain_inherent::{ParachainInherentData, ParachainInherentDataProvider},
    cumulus_primitives_core::{
        relay_chain::Hash as PHash, DigestItem, ParachainBlockData, PersistedValidationData,
    },
    cumulus_relay_chain_interface::RelayChainInterface,
    futures::prelude::*,
    nimbus_primitives::{CompatibleDigestItem as NimbusCompatibleDigestItem, NIMBUS_KEY_ID},
    parity_scale_codec::{Codec, Encode},
    polkadot_node_primitives::{Collation, MaybeCompressedPoV},
    polkadot_primitives::Id as ParaId,
    sc_consensus::{BlockImport, BlockImportParams, ForkChoiceStrategy, StateAction},
    sp_application_crypto::{AppCrypto, AppPublic},
    sp_consensus::BlockOrigin,
    sp_consensus_aura::{digests::CompatibleDigestItem, Slot},
    sp_core::crypto::{ByteArray, Pair},
    sp_inherents::{CreateInherentDataProviders, InherentData, InherentDataProvider},
    sp_keystore::{Keystore, KeystorePtr},
    sp_runtime::{
        generic::Digest,
        traits::{Block as BlockT, HashingFor, Header as HeaderT, Member, Zero},
    },
    sp_state_machine::StorageChanges,
    sp_timestamp::Timestamp,
    std::{convert::TryFrom, error::Error, time::Duration},
};

/// Parameters for instantiating a [`Collator`].
pub struct Params<BI, CIDP, RClient, Proposer, CS> {
    /// A builder for inherent data builders.
    pub create_inherent_data_providers: CIDP,
    /// The block import handle.
    pub block_import: BI,
    /// An interface to the relay-chain client.
    pub relay_client: RClient,
    /// The keystore handle used for accessing parachain key material.
    pub keystore: KeystorePtr,
    /// The identifier of the parachain within the relay-chain.
    pub para_id: ParaId,
    /// The block proposer used for building blocks.
    pub proposer: Proposer,
    /// The collator service used for bundling proposals into collations and announcing
    /// to the network.
    pub collator_service: CS,
}

/// A utility struct for writing collation logic that makes use of
/// Tanssi Aura entirely or in part.
pub struct Collator<Block, P, BI, CIDP, RClient, Proposer, CS> {
    create_inherent_data_providers: CIDP,
    block_import: BI,
    relay_client: RClient,
    keystore: KeystorePtr,
    para_id: ParaId,
    proposer: Proposer,
    collator_service: CS,
    _marker: std::marker::PhantomData<(Block, Box<dyn Fn(P) + Send + Sync + 'static>)>,
}

impl<Block, P, BI, CIDP, RClient, Proposer, CS> Collator<Block, P, BI, CIDP, RClient, Proposer, CS>
where
    Block: BlockT,
    RClient: RelayChainInterface,
    CIDP: CreateInherentDataProviders<Block, (PHash, PersistedValidationData)> + 'static,
    BI: BlockImport<Block> + Send + Sync + 'static,
    Proposer: ProposerInterface<Block>,
    CS: CollatorServiceInterface<Block>,
    P: Pair + Send + Sync + 'static,
    P::Public: AppPublic + Member,
    P::Signature: TryFrom<Vec<u8>> + Member + Codec,
{
    /// Instantiate a new instance of the `Tanssi Aura` manager.
    pub fn new(params: Params<BI, CIDP, RClient, Proposer, CS>) -> Self {
        Collator {
            create_inherent_data_providers: params.create_inherent_data_providers,
            block_import: params.block_import,
            relay_client: params.relay_client,
            keystore: params.keystore,
            para_id: params.para_id,
            proposer: params.proposer,
            collator_service: params.collator_service,
            _marker: std::marker::PhantomData,
        }
    }

    /// Explicitly creates the inherent data for parachain block authoring.
    pub async fn create_inherent_data(
        &self,
        relay_parent: PHash,
        validation_data: &PersistedValidationData,
        parent_hash: Block::Hash,
        _timestamp: impl Into<Option<Timestamp>>,
    ) -> Result<(ParachainInherentData, InherentData), Box<dyn Error + Send + Sync + 'static>> {
        let paras_inherent_data = ParachainInherentDataProvider::create_at(
            relay_parent,
            &self.relay_client,
            validation_data,
            self.para_id,
        )
        .await;

        let paras_inherent_data = match paras_inherent_data {
            Some(p) => p,
            None => {
                return Err(
                    format!("Could not create paras inherent data at {:?}", relay_parent).into(),
                )
            }
        };

        let other_inherent_data = self
            .create_inherent_data_providers
            .create_inherent_data_providers(parent_hash, (relay_parent, validation_data.clone()))
            .map_err(|e| e as Box<dyn Error + Send + Sync + 'static>)
            .await?
            .create_inherent_data()
            .await
            .map_err(Box::new)?;

        Ok((paras_inherent_data, other_inherent_data))
    }

    /// Propose, seal, and import a block, packaging it into a collation.
    ///
    /// Provide the slot to build at as well as any other necessary pre-digest logs,
    /// the inherent data, and the proposal duration and PoV size limits.
    ///
    /// The Tanssi Aura pre-digest is set internally.
    ///
    /// This does not announce the collation to the parachain network or the relay chain.
    #[allow(clippy::cast_precision_loss)]
    pub async fn collate(
        &mut self,
        parent_header: &Block::Header,
        slot_claim: &mut SlotClaim<P::Public>,
        additional_pre_digest: impl Into<Option<Vec<DigestItem>>>,
        inherent_data: (ParachainInherentData, InherentData),
        proposal_duration: Duration,
        max_pov_size: usize,
    ) -> Result<
        Option<(Collation, ParachainBlockData<Block>, Block::Hash)>,
        Box<dyn Error + Send + 'static>,
    > {
        let mut digest = additional_pre_digest.into().unwrap_or_default();
        digest.append(&mut slot_claim.pre_digest);

        let maybe_proposal = self
            .proposer
            .propose(
                parent_header,
                &inherent_data.0,
                inherent_data.1,
                Digest { logs: digest },
                proposal_duration,
                Some(max_pov_size),
            )
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error + Send>)?;

        let proposal = match maybe_proposal {
            None => return Ok(None),
            Some(p) => p,
        };

        let sealed_importable = seal_tanssi::<_, P>(
            proposal.block,
            proposal.storage_changes,
            &slot_claim.author_pub,
            &self.keystore,
        )
        .map_err(|e| e as Box<dyn Error + Send>)?;

        let post_hash = sealed_importable.post_hash();
        let block = Block::new(
            sealed_importable.post_header(),
            sealed_importable
                .body
                .as_ref()
                .expect("body always created with this `propose` fn; qed")
                .clone(),
        );

        self.block_import
            .import_block(sealed_importable)
            .map_err(|e| Box::new(e) as Box<dyn Error + Send>)
            .await?;

        if let Some((collation, block_data)) = self.collator_service.build_collation(
            parent_header,
            post_hash,
            ParachainCandidate {
                block,
                proof: proposal.proof,
            },
        ) {
            tracing::info!(
                target: crate::LOG_TARGET,
                "PoV size {{ header: {}kb, extrinsics: {}kb, storage_proof: {}kb }}",
                block_data.header().encoded_size() as f64 / 1024f64,
                block_data.extrinsics().encoded_size() as f64 / 1024f64,
                block_data.storage_proof().encoded_size() as f64 / 1024f64,
            );

            if let MaybeCompressedPoV::Compressed(ref pov) = collation.proof_of_validity {
                tracing::info!(
                    target: crate::LOG_TARGET,
                    "Compressed PoV size: {}kb",
                    pov.block_data.0.len() as f64 / 1024f64,
                );
            }

            Ok(Some((collation, block_data, post_hash)))
        } else {
            Err(
                Box::<dyn Error + Send + Sync>::from("Unable to produce collation")
                    as Box<dyn Error + Send>,
            )
        }
    }

    /// Get the underlying collator service.
    pub fn collator_service(&self) -> &CS {
        &self.collator_service
    }
}

fn pre_digest_data<P: Pair>(slot: Slot, claim: P::Public) -> Vec<sp_runtime::DigestItem>
where
    P::Public: Codec,
    P::Signature: Codec,
{
    vec![
        <DigestItem as CompatibleDigestItem<P::Signature>>::aura_pre_digest(slot),
        // We inject the nimbus digest as well. Crutial to be able to verify signatures
        <DigestItem as NimbusCompatibleDigestItem>::nimbus_pre_digest(
            // TODO remove this unwrap through trait reqs
            nimbus_primitives::NimbusId::from_slice(claim.as_ref()).unwrap(),
        ),
    ]
}

#[derive(Debug)]
pub struct SlotClaim<Pub> {
    author_pub: Pub,
    pre_digest: Vec<DigestItem>,
    slot: Slot,
}

impl<Pub: Clone> SlotClaim<Pub> {
    pub fn unchecked<P>(author_pub: Pub, slot: Slot) -> Self
    where
        P: Pair<Public = Pub>,
        P::Public: Codec,
        P::Signature: Codec,
    {
        SlotClaim {
            author_pub: author_pub.clone(),
            pre_digest: pre_digest_data::<P>(slot, author_pub),
            slot,
        }
    }

    /// Get the author's public key.
    pub fn author_pub(&self) -> &Pub {
        &self.author_pub
    }

    /// Get the pre-digest.
    pub fn pre_digest(&self) -> &Vec<DigestItem> {
        &self.pre_digest
    }

    /// Get the slot assigned to this claim.
    pub fn slot(&self) -> Slot {
        self.slot
    }
}

/// Attempt to claim a slot locally.
pub fn tanssi_claim_slot<P, B>(
    aux_data: OrchestratorAuraWorkerAuxData<P>,
    chain_head: &B::Header,
    slot: Slot,
    force_authoring: bool,
    keystore: &KeystorePtr,
) -> Result<Option<SlotClaim<P::Public>>, Box<dyn Error>>
where
    P: Pair + Send + Sync + 'static,
    P::Public: Codec + std::fmt::Debug,
    P::Signature: Codec,
    B: BlockT,
{
    let author_pub = {
        let res = claim_slot_inner::<P>(slot, &aux_data.authorities, keystore, force_authoring);
        match res {
            Some(p) => p,
            None => return Ok(None),
        }
    };

    if is_parathread_and_should_skip_slot::<P, B>(&aux_data, chain_head, slot) {
        return Ok(None);
    }

    Ok(Some(SlotClaim::unchecked::<P>(author_pub, slot)))
}

/// Returns true if this container chain is a parathread and the collator should skip this slot and not produce a block
pub fn is_parathread_and_should_skip_slot<P, B>(
    aux_data: &OrchestratorAuraWorkerAuxData<P>,
    chain_head: &B::Header,
    slot: Slot,
) -> bool
where
    P: Pair + Send + Sync + 'static,
    P::Public: Codec + std::fmt::Debug,
    P::Signature: Codec,
    B: BlockT,
{
    if slot.is_zero() {
        // Always produce on slot 0 (for tests)
        return false;
    }
    if let Some(min_slot_freq) = aux_data.min_slot_freq {
        if let Ok(chain_head_slot) = find_pre_digest::<B, P::Signature>(chain_head) {
            let slot_diff = slot.saturating_sub(chain_head_slot);

            // TODO: this doesn't take into account force authoring.
            // So a node with `force_authoring = true` will not propose a block for a parathread until the
            // `min_slot_freq` has elapsed.
            slot_diff < min_slot_freq
        } else {
            // In case of error always propose
            false
        }
    } else {
        // Not a parathread: always propose
        false
    }
}

/// Attempt to claim a slot using a keystore.
pub fn claim_slot_inner<P>(
    slot: Slot,
    authorities: &Vec<AuthorityId<P>>,
    keystore: &KeystorePtr,
    force_authoring: bool,
) -> Option<P::Public>
where
    P: Pair,
    P::Public: Codec + std::fmt::Debug,
    P::Signature: Codec,
{
    let expected_author = crate::slot_author::<P>(slot, authorities.as_slice());
    // if not running with force-authoring, just do the usual slot check
    if !force_authoring {
        expected_author.and_then(|p| {
            if keystore.has_keys(&[(p.to_raw_vec(), NIMBUS_KEY_ID)]) {
                Some(p.clone())
            } else {
                None
            }
        })
    }
    // if running with force-authoring, as long as you are in the authority set,
    // propose
    else {
        authorities
            .iter()
            .find(|key| keystore.has_keys(&[(key.to_raw_vec(), NIMBUS_KEY_ID)]))
            .cloned()
    }
}

/// Seal a block with a signature in the header.
pub fn seal_tanssi<B: BlockT, P>(
    pre_sealed: B,
    storage_changes: StorageChanges<HashingFor<B>>,
    author_pub: &P::Public,
    keystore: &KeystorePtr,
) -> Result<BlockImportParams<B>, Box<dyn Error + Send + Sync + 'static>>
where
    P: Pair,
    P::Signature: Codec + TryFrom<Vec<u8>>,
    P::Public: AppPublic,
{
    let (pre_header, body) = pre_sealed.deconstruct();
    let pre_hash = pre_header.hash();
    let block_number = *pre_header.number();

    // sign the pre-sealed hash of the block and then
    // add it to a digest item.
    let signature = Keystore::sign_with(
        keystore,
        <AuthorityId<P> as AppCrypto>::ID,
        <AuthorityId<P> as AppCrypto>::CRYPTO_ID,
        author_pub.as_slice(),
        pre_hash.as_ref(),
    )
    .map_err(|e| sp_consensus::Error::CannotSign(format!("{}. Key: {:?}", e, author_pub)))?
    .ok_or_else(|| {
        sp_consensus::Error::CannotSign(format!(
            "Could not find key in keystore. Key: {:?}",
            author_pub
        ))
    })?;
    let signature = signature
        .clone()
        .try_into()
        .map_err(|_| sp_consensus::Error::InvalidSignature(signature, author_pub.to_raw_vec()))?;

    let signature_digest_item = <DigestItem as NimbusCompatibleDigestItem>::nimbus_seal(signature);

    // seal the block.
    let block_import_params = {
        let mut block_import_params = BlockImportParams::new(BlockOrigin::Own, pre_header);
        block_import_params.post_digests.push(signature_digest_item);
        block_import_params.body = Some(body.clone());
        block_import_params.state_action =
            StateAction::ApplyChanges(sc_consensus::StorageChanges::Changes(storage_changes));
        block_import_params.fork_choice = Some(ForkChoiceStrategy::LongestChain);
        block_import_params
    };
    let post_hash = block_import_params.post_hash();

    tracing::info!(
        target: crate::LOG_TARGET,
        "🔖 Pre-sealed block for proposal at {}. Hash now {:?}, previously {:?}.",
        block_number,
        post_hash,
        pre_hash,
    );

    Ok(block_import_params)
}
