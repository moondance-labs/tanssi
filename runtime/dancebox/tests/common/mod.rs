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
    crate::UNIT,
    cumulus_primitives_core::{ParaId, PersistedValidationData},
    cumulus_primitives_parachain_inherent::ParachainInherentData,
    dancebox_runtime::{
        AuthorInherent, BlockProductionCost, CollatorAssignmentCost, MaxBootNodeUrlLen,
        MaxBootNodes, MaxLengthTokenSymbol,
    },
    dp_consensus::runtime_decl_for_tanssi_authority_assignment_api::TanssiAuthorityAssignmentApi,
    frame_support::{
        assert_ok,
        traits::{OnFinalize, OnInitialize},
    },
    nimbus_primitives::{NimbusId, NIMBUS_ENGINE_ID},
    pallet_collator_assignment_runtime_api::runtime_decl_for_collator_assignment_api::CollatorAssignmentApi,
    pallet_registrar_runtime_api::ContainerChainGenesisData,
    pallet_services_payment::{ProvideBlockProductionCost, ProvideCollatorAssignmentCost},
    parity_scale_codec::{Decode, Encode, MaxEncodedLen},
    polkadot_parachain_primitives::primitives::HeadData,
    sp_consensus_aura::AURA_ENGINE_ID,
    sp_consensus_slots::Slot,
    sp_core::{Get, Pair},
    sp_runtime::{traits::Dispatchable, BoundedVec, BuildStorage, Digest, DigestItem},
    sp_std::collections::btree_map::BTreeMap,
    test_relay_sproof_builder::ParaHeaderSproofBuilder,
};

pub use dancebox_runtime::{
    AccountId, AssetRate, AuthorNoting, AuthorityAssignment, AuthorityMapping, Balance, Balances,
    CollatorAssignment, Configuration, DataPreservers, ForeignAssets, ForeignAssetsCreator,
    InflationRewards, Initializer, Invulnerables, MinimumSelfDelegation, ParachainInfo,
    PooledStaking, Proxy, ProxyType, Registrar, RewardsPortion, Runtime, RuntimeCall,
    ServicesPayment, Session, System, TransactionPayment,
};

mod xcm;

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
///
/// After this function returns, the current block number will be `n`, and the block will be "open",
/// meaning that on_initialize has been executed, but on_finalize has not. To execute on_finalize as
/// well, for example to test a runtime api, manually call `end_block` after this, run the test, and
/// call `start_block` to ensure that this function keeps working as expected.
/// Extrinsics should always be executed before on_finalize.
pub fn run_to_block(n: u32) -> BTreeMap<u32, RunSummary> {
    let current_block_number = System::block_number();
    assert!(
        current_block_number < n,
        "run_to_block called with block {} when current block is {}",
        n,
        current_block_number
    );
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

// Used to create the next block inherent data
#[derive(Clone, Encode, Decode, Default, PartialEq, Debug, scale_info::TypeInfo, MaxEncodedLen)]
pub struct MockInherentData {
    pub random_seed: Option<[u8; 32]>,
}

fn take_new_inherent_data() -> Option<MockInherentData> {
    let data: Option<MockInherentData> =
        frame_support::storage::unhashed::take(b"__mock_new_inherent_data");

    data
}

fn set_new_inherent_data(data: MockInherentData) {
    frame_support::storage::unhashed::put(b"__mock_new_inherent_data", &Some(data));
}

#[derive(Clone, Encode, Decode, PartialEq, Debug, scale_info::TypeInfo, MaxEncodedLen)]
enum RunBlockState {
    Start(u32),
    End(u32),
}

impl RunBlockState {
    fn assert_can_advance(&self, new_state: &RunBlockState) {
        match self {
            RunBlockState::Start(n) => {
                assert_eq!(
                    new_state,
                    &RunBlockState::End(*n),
                    "expected a call to end_block({}), but user called {:?}",
                    *n,
                    new_state
                );
            }
            RunBlockState::End(n) => {
                assert_eq!(
                    new_state,
                    &RunBlockState::Start(*n + 1),
                    "expected a call to start_block({}), but user called {:?}",
                    *n + 1,
                    new_state
                )
            }
        }
    }
}

fn advance_block_state_machine(new_state: RunBlockState) {
    if frame_support::storage::unhashed::exists(b"__mock_is_xcm_test") {
        // Disable this check in XCM tests, because the XCM emulator runs on_initialize and
        // on_finalize automatically
        return;
    }
    let old_state: RunBlockState =
        frame_support::storage::unhashed::get(b"__mock_debug_block_state").unwrap_or(
            // Initial state is expecting a call to start() with block number 1, so old state should be
            // end of block 0
            RunBlockState::End(0),
        );
    old_state.assert_can_advance(&new_state);
    frame_support::storage::unhashed::put(b"__mock_debug_block_state", &new_state);
}

pub fn start_block() -> RunSummary {
    let block_number = System::block_number();
    advance_block_state_machine(RunBlockState::Start(block_number + 1));
    let mut slot = current_slot() + 1;
    if block_number == 0 {
        // Hack to avoid breaking all tests. When the current block is 1, the slot number should be
        // 1. But all of our tests assume it will be 0. So use slot number = block_number - 1.
        slot = 0;
    }

    let maybe_mock_inherent = take_new_inherent_data();

    if let Some(mock_inherent_data) = maybe_mock_inherent {
        set_parachain_inherent_data(mock_inherent_data);
    }

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

    RunSummary {
        author_id,
        inflation: new_issuance - current_issuance,
    }
}

pub fn end_block() {
    let block_number = System::block_number();
    advance_block_state_machine(RunBlockState::End(block_number));
    // Finalize the block
    CollatorAssignment::on_finalize(System::block_number());
    Session::on_finalize(System::block_number());
    Initializer::on_finalize(System::block_number());
    AuthorInherent::on_finalize(System::block_number());
    TransactionPayment::on_finalize(System::block_number());
}

pub fn run_block() -> RunSummary {
    end_block();
    let summary = start_block();

    summary
}

/// Mock the inherent that sets validation data in ParachainSystem, which
/// contains the `relay_chain_block_number`, which is used in `collator-assignment` as a
/// source of randomness.
pub fn set_parachain_inherent_data(mock_inherent_data: MockInherentData) {
    use {
        cumulus_primitives_core::relay_chain::well_known_keys,
        cumulus_test_relay_sproof_builder::RelayStateSproofBuilder,
    };

    let relay_sproof = RelayStateSproofBuilder {
        para_id: 100u32.into(),
        included_para_head: Some(HeadData(vec![1, 2, 3])),
        current_slot: (current_slot()).into(),
        additional_key_values: if mock_inherent_data.random_seed.is_some() {
            vec![(
                well_known_keys::CURRENT_BLOCK_RANDOMNESS.to_vec(),
                Some(mock_inherent_data.random_seed).encode(),
            )]
        } else {
            vec![]
        },
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

    // Delete existing flag to avoid error
    // 'ValidationData must be updated only once in a block'
    // TODO: this is a hack
    frame_support::storage::unhashed::kill(&frame_support::storage::storage_prefix(
        b"ParachainSystem",
        b"ValidationData",
    ));

    assert_ok!(RuntimeCall::ParachainSystem(
        cumulus_pallet_parachain_system::Call::<Runtime>::set_validation_data {
            data: parachain_inherent_data
        }
    )
    .dispatch(inherent_origin()));
}

pub fn set_parachain_inherent_data_random_seed(random_seed: [u8; 32]) {
    set_new_inherent_data(MockInherentData {
        random_seed: Some(random_seed),
    });
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

pub fn default_config() -> pallet_configuration::HostConfiguration {
    pallet_configuration::HostConfiguration {
        max_collators: 100,
        min_orchestrator_collators: 2,
        max_orchestrator_collators: 2,
        collators_per_container: 2,
        full_rotation_period: 24,
        ..Default::default()
    }
}

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

impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            balances: vec![
                // Alice gets 10k extra tokens for her mapping deposit
                (AccountId::from(ALICE), 210_000 * UNIT),
                (AccountId::from(BOB), 100_000 * UNIT),
            ],
            collators: vec![
                (AccountId::from(ALICE), 210 * UNIT),
                (AccountId::from(BOB), 100 * UNIT),
            ],
            sudo: Default::default(),
            para_ids: Default::default(),
            config: default_config(),
            safe_xcm_version: Default::default(),
            own_para_id: Default::default(),
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

        if self.safe_xcm_version.is_some() {
            // Disable run_block checks in XCM tests, because the XCM emulator runs on_initialize and
            // on_finalize automatically
            t.top.insert(b"__mock_is_xcm_test".to_vec(), b"1".to_vec());
        }

        t
    }

    pub fn build(self) -> sp_io::TestExternalities {
        let t = self.build_storage();
        let mut ext = sp_io::TestExternalities::new(t);

        ext.execute_with(|| {
            // Start block 1
            start_block();
            set_parachain_inherent_data(Default::default());
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
    u128::from(number_of_blocks).saturating_mul(block_cost)
}

pub fn collator_assignment_credits_to_required_balance(
    number_of_sessions: u32,
    para_id: ParaId,
) -> Balance {
    let collator_assignment_cost = CollatorAssignmentCost::collator_assignment_cost(&para_id).0;
    u128::from(number_of_sessions).saturating_mul(collator_assignment_cost)
}

pub const ALICE: [u8; 32] = [4u8; 32];
pub const BOB: [u8; 32] = [5u8; 32];
pub const CHARLIE: [u8; 32] = [6u8; 32];
pub const DAVE: [u8; 32] = [7u8; 32];
pub const EVE: [u8; 32] = [8u8; 32];
pub const FERDIE: [u8; 32] = [9u8; 32];
