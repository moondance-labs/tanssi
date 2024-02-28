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
    cumulus_primitives_parachain_inherent::ParachainInherentData,
    dancebox_runtime::{
        AuthorInherent, BlockProductionCost, CollatorAssignmentCost, MaxBootNodeUrlLen,
        MaxBootNodes, MaxLengthTokenSymbol,
    },
    frame_support::{
        assert_ok,
        traits::{OnFinalize, OnInitialize},
    },
    nimbus_primitives::{NimbusId, NIMBUS_ENGINE_ID},
    pallet_collator_assignment_runtime_api::runtime_decl_for_collator_assignment_api::CollatorAssignmentApi,
    pallet_registrar_runtime_api::ContainerChainGenesisData,
    pallet_services_payment::{ProvideBlockProductionCost, ProvideCollatorAssignmentCost},
    parity_scale_codec::Encode,
    polkadot_parachain_primitives::primitives::HeadData,
    sp_consensus_aura::AURA_ENGINE_ID,
    sp_consensus_slots::Slot,
    sp_core::{Get, Pair},
    sp_runtime::{traits::Dispatchable, BoundedVec, BuildStorage, Digest, DigestItem},
    sp_std::collections::btree_map::BTreeMap,
    test_relay_sproof_builder::ParaHeaderSproofBuilder,
    tp_consensus::runtime_decl_for_tanssi_authority_assignment_api::TanssiAuthorityAssignmentApi,
};

mod xcm;

pub use dancebox_runtime::{
    AccountId, AssetRate, AuthorNoting, AuthorityAssignment, AuthorityMapping, Balance, Balances,
    CollatorAssignment, Configuration, DataPreservers, ForeignAssets, ForeignAssetsCreator,
    InflationRewards, Initializer, Invulnerables, MinimumSelfDelegation, ParachainInfo,
    PooledStaking, Proxy, ProxyType, Registrar, RewardsPortion, Runtime, RuntimeCall,
    ServicesPayment, Session, System,
};

pub fn session_to_block(n: u32) -> u32 {
    let block_number = dancebox_runtime::Period::get() * n;

    // Add 1 because the block that emits the NewSession event cannot contain any extrinsics,
    // so this is the first block of the new session that can actually be used
    block_number + 1
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RunSummary {
    pub author_id: AccountId,
    pub inflation: Balance,
}

pub fn run_to_session(n: u32) {
    run_to_block(session_to_block(n));
}

/// Utility function that advances the chain to the desired block number.
pub fn run_to_block(n: u32) -> BTreeMap<u32, RunSummary> {
    let mut summaries = BTreeMap::new();

    while System::block_number() < n {
        let summary = run_block();
        let block_number = System::block_number();
        summaries.insert(block_number, summary);
    }

    summaries
}

pub fn insert_authorities_and_slot_digests(slot: u64) {
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
}

pub fn run_block() -> RunSummary {
    let slot = current_slot() + 1;

    insert_authorities_and_slot_digests(slot);

    // Initialize the new block
    CollatorAssignment::on_initialize(System::block_number());
    Session::on_initialize(System::block_number());
    Initializer::on_initialize(System::block_number());
    AuthorInherent::on_initialize(System::block_number());

    // `Initializer::on_finalize` needs to run at least one to have
    // author mapping setup.
    let author_id = current_author();

    let current_issuance = Balances::total_issuance();
    InflationRewards::on_initialize(System::block_number());
    let new_issuance = Balances::total_issuance();

    frame_support::storage::unhashed::put(
        &frame_support::storage::storage_prefix(b"AsyncBacking", b"SlotInfo"),
        &(Slot::from(slot), 1),
    );

    pallet_author_inherent::Pallet::<Runtime>::kick_off_authorship_validation(None.into())
        .expect("author inherent to dispatch correctly");

    // Finalize the block
    CollatorAssignment::on_finalize(System::block_number());
    Session::on_finalize(System::block_number());
    Initializer::on_finalize(System::block_number());
    AuthorInherent::on_finalize(System::block_number());

    RunSummary {
        author_id,
        inflation: new_issuance - current_issuance,
    }
}

/// Mock the inherent that sets validation data in ParachainSystem, which
/// contains the `relay_chain_block_number`, which is used in `collator-assignment` as a
/// source of randomness.
pub fn set_parachain_inherent_data() {
    use cumulus_test_relay_sproof_builder::RelayStateSproofBuilder;

    let relay_sproof = RelayStateSproofBuilder {
        para_id: 100u32.into(),
        included_para_head: Some(HeadData(vec![1, 2, 3])),
        current_slot: (current_slot() * 2).into(),
        ..Default::default()
    };

    let (relay_parent_storage_root, relay_chain_state) = relay_sproof.into_state_root_and_proof();
    let vfp = PersistedValidationData {
        relay_parent_number: 1u32,
        relay_parent_storage_root,
        ..Default::default()
    };
    let parachain_inherent_data = ParachainInherentData {
        validation_data: vfp,
        relay_chain_state,
        downward_messages: Default::default(),
        horizontal_messages: Default::default(),
    };

    assert_ok!(RuntimeCall::ParachainSystem(
        cumulus_pallet_parachain_system::Call::<Runtime>::set_validation_data {
            data: parachain_inherent_data
        }
    )
    .dispatch(inherent_origin()));
}

pub fn set_parachain_inherent_data_random_seed(random_seed: [u8; 32]) {
    use {
        cumulus_primitives_core::relay_chain::well_known_keys,
        cumulus_test_relay_sproof_builder::RelayStateSproofBuilder,
    };

    let slot = current_slot() + 1;

    let (relay_parent_storage_root, relay_chain_state) = {
        let mut sproof = RelayStateSproofBuilder::default();
        sproof.additional_key_values.push((
            well_known_keys::CURRENT_BLOCK_RANDOMNESS.to_vec(),
            Some(random_seed).encode(),
        ));

        sproof.para_id = 100u32.into();
        sproof.included_para_head = Some(HeadData(vec![1, 2, 3]));
        sproof.current_slot = (slot * 2).into();

        sproof.into_state_root_and_proof()
    };
    let vfp = PersistedValidationData {
        // TODO: this is previous relay_parent_number + 1, but not sure where can I get that value
        relay_parent_number: 2u32,
        relay_parent_storage_root,
        ..Default::default()
    };
    let parachain_inherent_data = ParachainInherentData {
        validation_data: vfp,
        relay_chain_state,
        downward_messages: Default::default(),
        horizontal_messages: Default::default(),
    };
    // Delete existing flag to avoid error
    // 'ValidationData must be updated only once in a block'
    // TODO: this is a hack
    frame_support::storage::unhashed::kill(&frame_support::storage::storage_prefix(
        b"ParachainSystem",
        b"ValidationData",
    ));

    insert_authorities_and_slot_digests(slot);

    assert_ok!(RuntimeCall::ParachainSystem(
        cumulus_pallet_parachain_system::Call::<Runtime>::set_validation_data {
            data: parachain_inherent_data
        }
    )
    .dispatch(inherent_origin()));
}

#[derive(Default)]
pub struct ExtBuilder {
    // endowed accounts with balances
    balances: Vec<(AccountId, Balance)>,
    // [collator, amount]
    collators: Vec<(AccountId, Balance)>,
    // sudo key
    sudo: Option<AccountId>,
    // list of registered para ids: para_id, genesis_data, boot_nodes, block_credits, session_credits
    para_ids: Vec<ParaRegistrationParams>,
    // configuration to apply
    config: pallet_configuration::HostConfiguration,
    safe_xcm_version: Option<u32>,
    own_para_id: Option<ParaId>,
}

#[derive(Default, Clone)]
pub struct ParaRegistrationParams {
    para_id: u32,
    genesis_data: ContainerChainGenesisData<MaxLengthTokenSymbol>,
    bootnodes: Vec<Vec<u8>>,
    block_production_credits: u32,
    collator_assignment_credits: u32,
}

impl
    From<(
        u32,
        ContainerChainGenesisData<MaxLengthTokenSymbol>,
        Vec<Vec<u8>>,
        u32,
        u32,
    )> for ParaRegistrationParams
{
    fn from(
        value: (
            u32,
            ContainerChainGenesisData<MaxLengthTokenSymbol>,
            Vec<Vec<u8>>,
            u32,
            u32,
        ),
    ) -> Self {
        Self {
            para_id: value.0,
            genesis_data: value.1,
            bootnodes: value.2,
            block_production_credits: value.3,
            collator_assignment_credits: value.4,
        }
    }
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

    pub fn with_sudo(mut self, sudo: AccountId) -> Self {
        self.sudo = Some(sudo);
        self
    }

    pub fn with_para_ids(mut self, para_ids: Vec<ParaRegistrationParams>) -> Self {
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
                .iter()
                .cloned()
                .map(|registered_para| {
                    (registered_para.para_id.into(), registered_para.genesis_data)
                })
                .collect(),
        }
        .assimilate_storage(&mut t)
        .unwrap();

        pallet_services_payment::GenesisConfig::<Runtime> {
            para_id_credits: self
                .para_ids
                .clone()
                .into_iter()
                .map(|registered_para| {
                    (
                        registered_para.para_id.into(),
                        registered_para.block_production_credits,
                        registered_para.collator_assignment_credits,
                    )
                        .into()
                })
                .collect(),
        }
        .assimilate_storage(&mut t)
        .unwrap();

        pallet_data_preservers::GenesisConfig::<Runtime> {
            para_id_boot_nodes: self
                .para_ids
                .into_iter()
                .map(|registered_para| (registered_para.para_id.into(), registered_para.bootnodes))
                .collect(),
            _phantom: Default::default(),
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
        pallet_sudo::GenesisConfig::<Runtime> { key: self.sudo }
            .assimilate_storage(&mut t)
            .unwrap();

        t
    }

    pub fn build(self) -> sp_io::TestExternalities {
        let t = self.build_storage();
        let mut ext = sp_io::TestExternalities::new(t);

        ext.execute_with(|| {
            System::set_block_number(1);
            System::deposit_log(DigestItem::PreRuntime(
                AURA_ENGINE_ID,
                (current_slot()).encode(),
            ));
            set_parachain_inherent_data();
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
    let slot: u64 = current_slot();
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

    // But we also need to store the new proof submitted
    frame_support::storage::unhashed::put(
        &frame_support::storage::storage_prefix(b"ParachainSystem", b"RelayStateProof"),
        &relay_storage_proof,
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

pub fn dummy_boot_nodes() -> BoundedVec<BoundedVec<u8, MaxBootNodeUrlLen>, MaxBootNodes> {
    vec![BoundedVec::try_from(
        b"/ip4/127.0.0.1/tcp/33049/ws/p2p/12D3KooWHVMhQDHBpj9vQmssgyfspYecgV6e3hH1dQVDUkUbCYC9"
            .to_vec(),
    )
    .unwrap()]
    .try_into()
    .unwrap()
}

pub fn current_slot() -> u64 {
    u64::from(
        pallet_async_backing::SlotInfo::<Runtime>::get()
            .unwrap_or_default()
            .0,
    )
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

pub fn block_credits_to_required_balance(number_of_blocks: u32, para_id: ParaId) -> Balance {
    let block_cost = BlockProductionCost::block_cost(&para_id).0;
    (number_of_blocks as u128).saturating_mul(block_cost)
}

pub fn collator_assignment_credits_to_required_balance(
    number_of_sessions: u32,
    para_id: ParaId,
) -> Balance {
    let collator_assignment_cost = CollatorAssignmentCost::collator_assignment_cost(&para_id).0;
    (number_of_sessions as u128).saturating_mul(collator_assignment_cost)
}

pub const ALICE: [u8; 32] = [4u8; 32];
pub const BOB: [u8; 32] = [5u8; 32];
pub const CHARLIE: [u8; 32] = [6u8; 32];
pub const DAVE: [u8; 32] = [7u8; 32];
