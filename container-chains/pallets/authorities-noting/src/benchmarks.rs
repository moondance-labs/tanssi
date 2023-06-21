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

#![cfg(feature = "runtime-benchmarks")]

//! Benchmarking
use {
    crate::{Call, Config, Pallet, ParaId},
    cumulus_pallet_parachain_system::RelaychainStateProvider,
    frame_benchmarking::{account, benchmarks},
    frame_system::RawOrigin,
    sp_std::vec,
};

mod test_sproof {
    use sp_trie::StorageProof;

    /// Mocked proof because we cannot build proofs in a no-std environment.
    /// Only stores the number of parachains, and reads a previously encoded proof for that number
    /// of items from `crate::mock_proof`.
    #[derive(Clone, Default)]
    pub struct ParaHeaderSproofBuilder;

    impl ParaHeaderSproofBuilder {
        pub fn into_state_root_and_proof(
            self,
        ) -> (cumulus_primitives_core::relay_chain::Hash, StorageProof) {
            let encoded = crate::mock_proof::ENCODED_PROOFS[0].1;
            let root = hex::decode(encoded.0).unwrap();
            let proof = StorageProof::new(encoded.1.iter().map(|s| hex::decode(s).unwrap()));

            (<[u8; 32]>::try_from(root).unwrap().into(), proof)
        }
    }

    /// Mocked proof because we cannot build proofs in a no-std environment.
    /// Reads a previously encoded proof from `crate::mock_proof`.
    #[derive(Clone, Default)]
    pub struct AuthorityAssignmentSproofBuilder;

    impl AuthorityAssignmentSproofBuilder {
        pub fn into_state_root_and_proof(
            self,
        ) -> (cumulus_primitives_core::relay_chain::Hash, StorageProof) {
            let encoded = crate::mock_proof::ENCODED_PROOFS[0].2;
            let root = hex::decode(encoded.0).unwrap();
            let proof = StorageProof::new(encoded.1.iter().map(|s| hex::decode(s).unwrap()));

            (<[u8; 32]>::try_from(root).unwrap().into(), proof)
        }
    }
}

benchmarks! {
    set_latest_authorities_data {
        // TODO: this could measure the proof size
        let sproof_builder_relay = test_sproof::ParaHeaderSproofBuilder::default();
        let sproof_builder_orchestrator = test_sproof::AuthorityAssignmentSproofBuilder::default();

        let (relay_root, relay_proof) = sproof_builder_relay.into_state_root_and_proof();
        let (orchestrator_root, orchestrator_proof) = sproof_builder_orchestrator.into_state_root_and_proof();

        let data = ccp_authorities_noting_inherent::ContainerChainAuthoritiesInherentData {
            relay_chain_state: relay_proof,
            orchestrator_chain_state: orchestrator_proof,
        };

        T::RelayChainStateProvider::set_current_relay_chain_state(cumulus_pallet_parachain_system::RelayChainState {
            state_root: relay_root,
            number: 0,
        });
    }: _(RawOrigin::None, data)

    set_authorities {
        // Depend on the number of authorities
        let x in 0..10;

        let mut authorities = vec![];

        for _ in 0..x {
            let author: T::AuthorityId = account::<T::AuthorityId>("account id", x, 0u32);
            authorities.push(author);
        }
    }: _(RawOrigin::Root, authorities)

    set_orchestrator_para_id {
    }: _(RawOrigin::Root, ParaId::new(2000))

    impl_benchmark_test_suite!(
        Pallet,
        crate::mock::new_test_ext(),
        crate::mock::Test
    );
}
