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

use {
    cumulus_primitives_core::PersistedValidationData,
    frame_support::{
        assert_ok,
        dispatch::Dispatchable,
        traits::{GenesisBuild, OnFinalize, OnInitialize},
    },
    nimbus_primitives::NimbusId,
    pallet_collator_assignment_runtime_api::runtime_decl_for_collator_assignment_api::CollatorAssignmentApi,
    pallet_registrar_runtime_api::ContainerChainGenesisData,
    parity_scale_codec::Encode,
    polkadot_parachain::primitives::HeadData,
    sp_consensus_aura::AURA_ENGINE_ID,
    sp_core::{Get, Pair},
    sp_runtime::{Digest, DigestItem},
    test_relay_sproof_builder::ParaHeaderSproofBuilder,
};

pub use orchestrator_runtime::{
    AccountId, Aura, Authorship, Balance, Balances, Initializer, ParachainInfo, Registrar, Runtime,
    RuntimeCall, RuntimeEvent, Session, System,
};

pub fn run_to_session(n: u32, add_author: bool) {
    let block_number = orchestrator_runtime::Period::get() * n;
    run_to_block(block_number + 1, add_author);
}

/// Utility function that advances the chain to the desired block number.
/// If add_author is true, the author information is injected to all the blocks in the meantime.
pub fn run_to_block(n: u32, add_author: bool) {
    /*
    // Finalize the first block
    AuthorInherent::on_finalize(System::block_number());
    */
    while System::block_number() < n {
        // Set the new block number and author
        if add_author {
            let slot = Aura::current_slot();
            let pre_digest = Digest {
                logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, (slot + 1).encode())],
            };
            System::reset_events();
            System::initialize(
                &(System::block_number() + 1),
                &System::parent_hash(),
                &pre_digest,
            );
        } else {
            System::set_block_number(System::block_number() + 1);
        }

        // Initialize the new block

        Session::on_initialize(System::block_number());
        Initializer::on_initialize(System::block_number());
        Aura::on_initialize(System::block_number());
        Authorship::on_initialize(System::block_number());

        // Finalize the block
        Session::on_finalize(System::block_number());
        Initializer::on_finalize(System::block_number());
        Aura::on_finalize(System::block_number());
        Authorship::on_finalize(System::block_number());
    }
}

#[derive(Default)]
pub struct ExtBuilder {
    // endowed accounts with balances
    balances: Vec<(AccountId, Balance)>,
    // [collator, amount]
    collators: Vec<(AccountId, Balance)>,
    // list of registered para ids
    para_ids: Vec<(u32, ContainerChainGenesisData)>,
    // configuration to apply
    config: pallet_configuration::HostConfiguration,
}

impl ExtBuilder {
    pub fn with_balances(mut self, balances: Vec<(AccountId, Balance)>) -> Self {
        self.balances = balances;
        self
    }

    pub fn with_collators(mut self, collators: Vec<(AccountId, Balance)>) -> Self {
        self.collators = collators;
        self
    }

    pub fn with_para_ids(mut self, para_ids: Vec<(u32, ContainerChainGenesisData)>) -> Self {
        self.para_ids = para_ids;
        self
    }

    pub fn with_config(mut self, config: pallet_configuration::HostConfiguration) -> Self {
        self.config = config;
        self
    }

    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Runtime>()
            .unwrap();

        pallet_balances::GenesisConfig::<Runtime> {
            balances: self.balances,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        // We need to initialize these pallets first. When initializing pallet-session,
        // these values will be taken into account for collator-assignment.
        <pallet_registrar::GenesisConfig as GenesisBuild<Runtime>>::assimilate_storage(
            &pallet_registrar::GenesisConfig {
                para_ids: self
                    .para_ids
                    .into_iter()
                    .map(|(para_id, genesis_data)| (para_id.into(), genesis_data))
                    .collect(),
            },
            &mut t,
        )
        .unwrap();

        <pallet_configuration::GenesisConfig as GenesisBuild<Runtime>>::assimilate_storage(
            &pallet_configuration::GenesisConfig {
                config: self.config,
            },
            &mut t,
        )
        .unwrap();

        if !self.collators.is_empty() {
            // We set invulnerables in pallet_collator_selection
            let invulnerables: Vec<AccountId> = self
                .collators
                .clone()
                .into_iter()
                .map(|(account, _balance)| account)
                .collect();

            pallet_collator_selection::GenesisConfig::<Runtime> {
                invulnerables: invulnerables.clone(),
                candidacy_bond: Default::default(),
                desired_candidates: invulnerables.len() as u32,
            }
            .assimilate_storage(&mut t)
            .unwrap();

            // But we also initialize their keys in the session pallet
            let keys: Vec<_> = self
                .collators
                .into_iter()
                .map(|(account, _balance)| {
                    let aura_id = get_aura_id_from_seed(&account.to_string());
                    (
                        account.clone(),
                        account,
                        orchestrator_runtime::SessionKeys { aura: aura_id },
                    )
                })
                .collect();
            <pallet_session::GenesisConfig<Runtime> as GenesisBuild<Runtime>>::assimilate_storage(
                &pallet_session::GenesisConfig { keys },
                &mut t,
            )
            .unwrap();
        }

        let mut ext = sp_io::TestExternalities::new(t);

        ext.execute_with(|| {
            System::set_block_number(1);
        });
        ext
    }
}

pub const ALICE: [u8; 32] = [4u8; 32];
pub const BOB: [u8; 32] = [5u8; 32];
pub const CHARLIE: [u8; 32] = [6u8; 32];
pub const DAVE: [u8; 32] = [7u8; 32];

pub fn root_origin() -> <Runtime as frame_system::Config>::RuntimeOrigin {
    <Runtime as frame_system::Config>::RuntimeOrigin::root()
}

pub fn origin_of(account_id: AccountId) -> <Runtime as frame_system::Config>::RuntimeOrigin {
    <Runtime as frame_system::Config>::RuntimeOrigin::signed(account_id)
}

pub fn inherent_origin() -> <Runtime as frame_system::Config>::RuntimeOrigin {
    <Runtime as frame_system::Config>::RuntimeOrigin::none()
}

/// Helper function to generate a crypto pair from seed
pub fn get_aura_id_from_seed(seed: &str) -> NimbusId {
    sp_core::sr25519::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
        .into()
}

pub fn get_orchestrator_current_author() -> Option<AccountId> {
    let slot: u64 = Aura::current_slot().into();
    let orchestrator_collators = Runtime::parachain_collators(ParachainInfo::get())?;
    let author_index = slot % orchestrator_collators.len() as u64;
    let account = orchestrator_collators.get(author_index as usize)?;
    Some(account.clone())
}
/// Mocks the author noting inherent to insert the data we
pub fn set_author_noting_inherent_data(builder: ParaHeaderSproofBuilder) {
    let (relay_storage_root, relay_storage_proof) = builder.into_state_root_and_proof();

    // For now we directly touch parachain_system storage to set the relay state root.
    // TODO: Properly set the parachain_system inherent, which require a sproof builder combining
    // what is required by parachain_system and author_noting.
    frame_support::storage::unhashed::put(
        &frame_support::storage::storage_prefix(b"ParachainSystem", b"ValidationData"),
        &PersistedValidationData {
            parent_head: HeadData(Default::default()),
            relay_parent_number: 0u32,
            relay_parent_storage_root: relay_storage_root,
            max_pov_size: 0u32,
        },
    );

    assert_ok!(RuntimeCall::AuthorNoting(
        pallet_author_noting::Call::<Runtime>::set_latest_author_data {
            data: tp_author_noting_inherent::OwnParachainInherentData {
                relay_storage_proof,
            }
        }
    )
    .dispatch(inherent_origin()));
}

pub fn empty_genesis_data() -> ContainerChainGenesisData {
    ContainerChainGenesisData {
        storage: Default::default(),
        name: Default::default(),
        id: Default::default(),
        fork_id: Default::default(),
        extensions: Default::default(),
        properties: Default::default(),
    }
}
