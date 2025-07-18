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

use {
    babe_primitives::AuthorityId as BabeId,
    cumulus_primitives_core::relay_chain::{
        AccountId, AssignmentId, AuthorityDiscoveryId, ValidatorId,
    },
    frame_support::traits::OnIdle,
    frame_system::pallet_prelude::BlockNumberFor,
    frame_system::Pallet as SystemPallet,
    parity_scale_codec::Encode,
    sc_consensus_grandpa::AuthorityId as GrandpaId,
    sp_consensus_aura::AURA_ENGINE_ID,
    sp_core::{crypto::get_public_from_string_or_panic, sr25519},
    sp_runtime::generic::DigestItem,
    sp_runtime::traits::Convert,
    sp_runtime::Digest,
    sp_std::marker::PhantomData,
    sp_weights::Weight,
    xcm_emulator::{HeaderT, Network, Parachain, RelayChain},
};

pub mod accounts;
pub mod impls;
pub mod snowbridge;
pub mod validators;

pub fn force_process_bridge<R, P>(weight: Weight)
where
    R: RelayChain,
    P: Parachain<Network = R::Network>,
    R::Runtime: pallet_message_queue::Config,
{
    // Process MessageQueue on relay chain to consume the message we want to send to eth
    R::execute_with(|| {
        <pallet_message_queue::Pallet<R::Runtime>>::on_idle(
            SystemPallet::<R::Runtime>::block_number(),
            weight,
        );
    });

    // Execute empty block in parachain to trigger bridge message
    P::execute_with(|| {});
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed_no_beefy(
    seed: &str,
) -> (
    AccountId,
    AccountId,
    BabeId,
    GrandpaId,
    ValidatorId,
    AssignmentId,
    AuthorityDiscoveryId,
) {
    (
        get_public_from_string_or_panic::<sr25519::Public>(&format!("{}//stash", seed)).into(),
        get_public_from_string_or_panic::<sr25519::Public>(seed).into(),
        get_public_from_string_or_panic::<BabeId>(seed),
        get_public_from_string_or_panic::<GrandpaId>(seed),
        get_public_from_string_or_panic::<ValidatorId>(seed),
        get_public_from_string_or_panic::<AssignmentId>(seed),
        get_public_from_string_or_panic::<AuthorityDiscoveryId>(seed),
    )
}

pub struct TestDigestProvider<R: frame_system::Config, N: Network>(PhantomData<(R, N)>);

impl<R: frame_system::Config, N: Network> Convert<BlockNumberFor<R>, Digest> for TestDigestProvider<R, N>
where u64: From<<<<R as frame_system::Config>::Block as cumulus_primitives_core::BlockT>::Header as HeaderT>::Number> {
    fn convert(_block_number: BlockNumberFor<R>) -> Digest {
        let relay_block = N::relay_block_number();
        let slot = u64::from(relay_block.into());

        let new_slot_digest: Digest = Digest {
            logs: vec![
                DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode()),
            ],
        };
        new_slot_digest
    }
}
