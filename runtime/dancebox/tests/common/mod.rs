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
    cumulus_primitives_core::{ParaId, PersistedValidationData},
    dancebox_runtime::{AuthorInherent, AuthorityAssignment},
    frame_support::{
        assert_ok,
        traits::{OnFinalize, OnInitialize},
    },
    nimbus_primitives::{NimbusId, NIMBUS_ENGINE_ID},
    pallet_collator_assignment_runtime_api::runtime_decl_for_collator_assignment_api::CollatorAssignmentApi,
    pallet_registrar_runtime_api::ContainerChainGenesisData,
    parity_scale_codec::Encode,
    polkadot_parachain_primitives::primitives::HeadData,
    sp_consensus_aura::AURA_ENGINE_ID,
    sp_core::{Get, Pair},
    sp_runtime::{traits::Dispatchable, BuildStorage, Digest, DigestItem},
    test_relay_sproof_builder::ParaHeaderSproofBuilder,
    tp_consensus::runtime_decl_for_tanssi_authority_assignment_api::TanssiAuthorityAssignmentApi,
};

mod xcm;

use dancebox_runtime::MaxLengthTokenSymbol;
pub use dancebox_runtime::{
    AccountId, Balance, Balances, Initializer, ParachainInfo, Registrar, Runtime, RuntimeCall,
    RuntimeEvent, Session, System,
};

pub fn session_to_block(n: u32) -> u32 {
    let block_number = dancebox_runtime::Period::get() * n;

    // Add 1 because the block that emits the NewSession event cannot contain any extrinsics,
    // so this is the first block of the new session that can actually be used
    block_number + 1
}

pub fn run_to_session(n: u32) {
    run_to_block(session_to_block(n));
}

/// Utility function that advances the chain to the desired block number.
pub fn run_to_block(n: u32) {
    while System::block_number() < n {
        let slot = current_slot() + 1;

        let authorities =
            Runtime::para_id_authorities(ParachainInfo::get()).expect("authorities should be set");

        let authority: NimbusId = authorities[slot as usize % authorities.len()].clone();

        let pre_digest = Digest {
            logs: vec![
                DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode()),
                DigestItem::PreRuntime(NIMBUS_ENGINE_ID, authority.encode()),
            ],
        };

        System::reset_events();
        System::initialize(
            &(System::block_number() + 1),
            &System::parent_hash(),
            &pre_digest,
        );

        // Initialize the new block
        Session::on_initialize(System::block_number());
        Initializer::on_initialize(System::block_number());
        AuthorInherent::on_initialize(System::block_number());

        pallet_author_inherent::Pallet::<Runtime>::kick_off_authorship_validation(None.into())
            .expect("author inherent to dispatch correctly");

        // Finalize the block
        Session::on_finalize(System::block_number());
        Initializer::on_finalize(System::block_number());
        AuthorInherent::on_finalize(System::block_number());
    }
}

#[derive(Default)]
pub struct ExtBuilder {
    // endowed accounts with balances
    balances: Vec<(AccountId, Balance)>,
    // [collator, amount]
    collators: Vec<(AccountId, Balance)>,
    // list of registered para ids
    para_ids: Vec<(
        u32,
        ContainerChainGenesisData<MaxLengthTokenSymbol>,
        Vec<Vec<u8>>,
    )>,
    // configuration to apply
    config: pallet_configuration::HostConfiguration,
    safe_xcm_version: Option<u32>,
    own_para_id: Option<ParaId>,
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

    pub fn with_para_ids(
        mut self,
        para_ids: Vec<(
            u32,
            ContainerChainGenesisData<MaxLengthTokenSymbol>,
            Vec<Vec<u8>>,
        )>,
    ) -> Self {
        self.para_ids = para_ids;
        self
    }

    pub fn with_config(mut self, config: pallet_configuration::HostConfiguration) -> Self {
        self.config = config;
        self
    }

    pub fn with_safe_xcm_version(mut self, safe_xcm_version: u32) -> Self {
        self.safe_xcm_version = Some(safe_xcm_version);
        self
    }

    pub fn with_own_para_id(mut self, own_para_id: ParaId) -> Self {
        self.own_para_id = Some(own_para_id);
        self
    }

    pub fn build_storage(self) -> sp_core::storage::Storage {
        let mut t = frame_system::GenesisConfig::<Runtime>::default()
            .build_storage()
            .unwrap();

        pallet_balances::GenesisConfig::<Runtime> {
            balances: self.balances,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        // We need to initialize these pallets first. When initializing pallet-session,
        // these values will be taken into account for collator-assignment.

        pallet_registrar::GenesisConfig::<Runtime> {
            para_ids: self
                .para_ids
                .into_iter()
                .map(|(para_id, genesis_data, boot_nodes)| {
                    (para_id.into(), genesis_data, boot_nodes)
                })
                .collect(),
        }
        .assimilate_storage(&mut t)
        .unwrap();

        pallet_configuration::GenesisConfig::<Runtime> {
            config: self.config,
            ..Default::default()
        }
        .assimilate_storage(&mut t)
        .unwrap();

        pallet_xcm::GenesisConfig::<Runtime> {
            safe_xcm_version: self.safe_xcm_version,
            ..Default::default()
        }
        .assimilate_storage(&mut t)
        .unwrap();

        if let Some(own_para_id) = self.own_para_id {
            parachain_info::GenesisConfig::<Runtime> {
                parachain_id: own_para_id,
                ..Default::default()
            }
            .assimilate_storage(&mut t)
            .unwrap();
        }

        if !self.collators.is_empty() {
            // We set invulnerables in pallet_invulnerables
            let invulnerables: Vec<AccountId> = self
                .collators
                .clone()
                .into_iter()
                .map(|(account, _balance)| account)
                .collect();

            pallet_invulnerables::GenesisConfig::<Runtime> {
                invulnerables: invulnerables.clone(),
            }
            .assimilate_storage(&mut t)
            .unwrap();

            // But we also initialize their keys in the session pallet
            let keys: Vec<_> = self
                .collators
                .into_iter()
                .map(|(account, _balance)| {
                    let nimbus_id = get_aura_id_from_seed(&account.to_string());
                    (
                        account.clone(),
                        account,
                        dancebox_runtime::SessionKeys { nimbus: nimbus_id },
                    )
                })
                .collect();
            pallet_session::GenesisConfig::<Runtime> { keys }
                .assimilate_storage(&mut t)
                .unwrap();
        }
        t
    }

    pub fn build(self) -> sp_io::TestExternalities {
        let t = self.build_storage();
        let mut ext = sp_io::TestExternalities::new(t);

        ext.execute_with(|| {
            System::set_block_number(1);
        });
        ext
    }
}

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
    let slot: u64 = current_slot().into();
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

pub fn empty_genesis_data() -> ContainerChainGenesisData<MaxLengthTokenSymbol> {
    ContainerChainGenesisData {
        storage: Default::default(),
        name: Default::default(),
        id: Default::default(),
        fork_id: Default::default(),
        extensions: Default::default(),
        properties: Default::default(),
    }
}

pub fn current_slot() -> u64 {
    pallet_author_inherent::HighestSlotSeen::<Runtime>::get().into()
}

pub fn authorities() -> Vec<NimbusId> {
    let session_index = Session::current_index();

    AuthorityAssignment::collator_container_chain(session_index)
        .expect("authorities should be set")
        .orchestrator_chain
}

pub fn current_author() -> AccountId {
    let current_session = Session::current_index();
    let mapping =
        pallet_authority_mapping::Pallet::<Runtime>::authority_id_mapping(current_session)
            .expect("there is a mapping for the current session");

    let author = pallet_author_inherent::Author::<Runtime>::get()
        .expect("there should be a registered author");

    mapping
        .get(&author)
        .expect("there is a mapping for the current author")
        .clone()
}

pub const ALICE: [u8; 32] = [4u8; 32];
pub const BOB: [u8; 32] = [5u8; 32];
pub const CHARLIE: [u8; 32] = [6u8; 32];
pub const DAVE: [u8; 32] = [7u8; 32];
