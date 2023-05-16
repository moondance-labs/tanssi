// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! The AuRa consensus algorithm for parachains.    
//!
//! This extends the Substrate provided AuRa consensus implementation to make it compatible for
//! parachains. The main entry points for of this consensus algorithm are [`AuraConsensus::build`]
//! and [`fn@import_queue`].
//!
//! For more information about AuRa, the Substrate crate should be checked.
use {sp_consensus_slots::Slot, sp_core::crypto::Pair};

mod consensus_container;
mod consensus_orchestrator;
mod manual_seal;

pub use {
    consensus_container::*,
    consensus_orchestrator::{BuildOrchestratorAuraConsensusParams, OrchestratorAuraConsensus},
    sc_consensus_aura::CompatibilityMode,
};

pub use {
    manual_seal::OrchestratorManualSealAuraConsensusDataProvider,
    parity_scale_codec::{Decode, Encode},
    sc_consensus_aura::{slot_duration, AuraVerifier, BuildAuraWorkerParams, SlotProportion},
    sc_consensus_slots::InherentDataProviderExt,
    sp_api::{Core, ProvideRuntimeApi},
    sp_application_crypto::AppPublic,
    sp_consensus::Error as ConsensusError,
    sp_core::crypto::{ByteArray, Public},
    sp_keystore::{SyncCryptoStore, SyncCryptoStorePtr},
    sp_runtime::traits::{Block as BlockT, Header as HeaderT, Member, NumberFor},
    std::hash::Hash,
    tp_consensus::TanssiAuthorityAssignmentApi,
};
const LOG_TARGET: &str = "aura::tanssi";

type AuthorityId<P> = <P as Pair>::Public;

/// Get slot author for given block along with authorities.
pub(crate) fn slot_author<P: Pair>(
    slot: Slot,
    authorities: &[AuthorityId<P>],
) -> Option<&AuthorityId<P>> {
    if authorities.is_empty() {
        return None;
    }

    let idx = *slot % (authorities.len() as u64);
    assert!(
        idx <= usize::MAX as u64,
        "It is impossible to have a vector with length beyond the address space; qed",
    );

    let current_author = authorities.get(idx as usize).expect(
        "authorities not empty; index constrained to list length;this is a valid index; qed",
    );

    Some(current_author)
}

pub fn authorities<P, B, C>(
    client: &C,
    parent_hash: B::Hash,
    context_block_number: NumberFor<B>,
    compatibility_mode: &CompatibilityMode<NumberFor<B>>,
    keystore: SyncCryptoStorePtr,
) -> Result<Vec<AuthorityId<P>>, ConsensusError>
where
    P: Pair + Send + Sync,
    P::Public: AppPublic + Hash + Member + Encode + Decode,
    P::Signature: TryFrom<Vec<u8>> + Hash + Member + Encode + Decode,
    B: BlockT,
    C: ProvideRuntimeApi<B>,
    C::Api: TanssiAuthorityAssignmentApi<B, AuthorityId<P>>,
    AuthorityId<P>: From<<NimbusPair as sp_application_crypto::Pair>::Public>,
{
    let runtime_api = client.runtime_api();

    match compatibility_mode {
        CompatibilityMode::None => {}
        // Use `initialize_block` until we hit the block that should disable the mode.
        CompatibilityMode::UseInitializeBlock { until } => {
            if *until > context_block_number {
                runtime_api
                    .initialize_block(
                        parent_hash,
                        &B::Header::new(
                            context_block_number,
                            Default::default(),
                            Default::default(),
                            parent_hash,
                            Default::default(),
                        ),
                    )
                    .map_err(|_| sp_consensus::Error::InvalidAuthoritiesSet)?;
            }
        }
    }
    first_eligible_key::<B, C, P>(
        client.clone(),
        keystore.clone(),
        &parent_hash,
        &context_block_number,
    )
    .ok_or(sp_consensus::Error::InvalidAuthoritiesSet)
}

use nimbus_primitives::{NimbusId, NimbusPair, NIMBUS_KEY_ID};
/// Grab the first eligible nimbus key from the keystore
/// If multiple keys are eligible this function still only returns one
/// and makes no guarantees which one as that depends on the keystore's iterator behavior.
/// This is the standard way of determining which key to author with.
pub(crate) fn first_eligible_key<B: BlockT, C, P>(
    client: &C,
    keystore: SyncCryptoStorePtr,
    parent_hash: &B::Hash,
    parent_number: &NumberFor<B>,
) -> Option<Vec<AuthorityId<P>>>
where
    C: ProvideRuntimeApi<B>,
    C::Api: TanssiAuthorityAssignmentApi<B, AuthorityId<P>>,
    P: Pair + Send + Sync,
    P::Public: AppPublic + Hash + Member + Encode + Decode,
    P::Signature: TryFrom<Vec<u8>> + Hash + Member + Encode + Decode,
    AuthorityId<P>: From<<NimbusPair as sp_application_crypto::Pair>::Public>,
{
    // Get all the available keys
    let available_keys = SyncCryptoStore::keys(&*keystore, NIMBUS_KEY_ID).ok()?;

    // Print a more helpful message than "not eligible" when there are no keys at all.
    if available_keys.is_empty() {
        log::warn!(
            target: LOG_TARGET,
            "🔏 No Nimbus keys available. We will not be able to author."
        );
        return None;
    }

    let runtime_api = client.runtime_api();

    // Iterate keys until we find an eligible one, or run out of candidates.
    // If we are skipping prediction, then we author with the first key we find.
    // prediction skipping only really makes sense when there is a single key in the keystore.
    let maybe_authorities = available_keys.into_iter().find_map(|type_public_pair| {
        // Have to convert to a typed NimbusId to pass to the runtime API. Maybe this is a clue
        // That I should be passing Vec<u8> across the wasm boundary?
        if let Ok(nimbus_id) = NimbusId::from_slice(&type_public_pair.1) {
            // If we dont find any parachain that we are assigned to, return non

            if let Ok(Some(para_id)) = runtime_api.check_para_id_assignment(
                parent_hash.clone(),
                nimbus_id.clone().into(),
                parent_number,
            ) {
                log::info!("Para id found for assignment {:?}", para_id);
                let authorities = runtime_api
                    .para_id_authorities(parent_hash.clone(), para_id, parent_number)
                    .ok()?;
                log::info!(
                    "Authorities found for para {:?} are {:?}",
                    para_id,
                    authorities
                );
                authorities
            } else {
                log::info!("No Para id found for assignment {:?}", nimbus_id);

                None
            }
        } else {
            None
        }
    });

    maybe_authorities
}
